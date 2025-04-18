use core::cell::Cell;

use linux_api::signal::{
    SigActionFlags, SigAltStackFlags, Signal, SignalHandler, defaultaction, sigaction, siginfo_t,
    sigset_t, stack_t,
};
use linux_api::ucontext::ucontext;
use log::{trace, warn};
use shadow_shim_helper_rs::shim_shmem;

use crate::tls::ShimTlsVar;
use crate::{ExecutionContext, global_host_shmem, tls_process_shmem, tls_thread_shmem};

/// Information passed through to the SIGUSR1 signal handler. Contains the info
/// needed to call a managed code signal handler.
struct Sigusr1Info {
    native_sigaltstack: Option<stack_t>,
    siginfo: siginfo_t,
    action: sigaction,
    // May be NULL, in the case that we didn't get here in the context of an
    // earlier signal handler (e.g. seccomp).

    // We don't copy by value in case additional fields are added to the stuct
    // definition, and because we currently accept a libc::ucontext_t in our C
    // API, which *does* have extra fields at the end.
    ctx: *mut ucontext,
}
static SIGUSR1_SIGINFO: ShimTlsVar<Cell<Option<Sigusr1Info>>> =
    ShimTlsVar::new(&crate::SHIM_TLS, || Cell::new(None));

extern "C" fn handle_sigusr1(_signo: i32, _info: *mut siginfo_t, ctx: *mut core::ffi::c_void) {
    debug_assert_eq!(
        ExecutionContext::current(),
        ExecutionContext::Shadow,
        "Native sigusr1 unexpectedly raised from non-shadow code"
    );

    let mut info = SIGUSR1_SIGINFO.get().take().unwrap();
    let signo = info.siginfo.signal().unwrap().as_i32();
    // SAFETY: Should have been initialized correctly in `process_signals`.
    let handler = unsafe { info.action.handler() };

    if let Some(stack) = &info.native_sigaltstack {
        // We temporarily switched the sigaltstack so that this handler would
        // run on the specified stack. Now switch back to the native sigaltstack (i.e.
        // the one Shadow originally configured for *it's* signal handling).
        unsafe { linux_api::signal::sigaltstack(Some(stack), None) }.unwrap();
    }

    // SAFETY: Not particularly. We're calling a handler provided by managed code, which
    // we don't attempt to analyze or sandbox. A "well behaved" handler should be safe to
    // call here, but it could do anything including things that are unsound in Rust.
    match handler {
        linux_api::signal::SignalHandler::Handler(handler_fn) => {
            let _prev = ExecutionContext::Application.enter();
            unsafe { handler_fn(signo) }
        }
        linux_api::signal::SignalHandler::Action(action_fn) => unsafe {
            // If there's an "earlier" context, we use it. This might be important e.g.
            // when handling a signal like SIGSEGV, where the handler might actually
            // inspect individual register values.
            //
            // Otherwise, use the the context that the kernel gave us for *this* signal
            // handler.  The register values won't make much sense to the handler, but
            // it should WAI with functionality like `swapcontext`, which might be done
            // in an implementation of user-space threads.
            let ctx: *mut ucontext = if info.ctx.is_null() {
                log::warn!("Passing a synthetic context to managed code signal handler");
                ctx.cast()
            } else {
                info.ctx
            };
            {
                let _prev = ExecutionContext::Application.enter();
                action_fn(signo, &mut info.siginfo, ctx.cast::<core::ffi::c_void>())
            }
        },
        linux_api::signal::SignalHandler::SigIgn | linux_api::signal::SignalHandler::SigDfl => {
            panic!("No handler")
        }
    }
}

fn die_with_fatal_signal(sig: Signal) -> ! {
    assert_eq!(ExecutionContext::current(), ExecutionContext::Shadow);
    if sig == Signal::SIGKILL {
        // No need to restore default action, and trying to do so would fail.
    } else {
        let action = sigaction::new_with_default_restorer(
            SignalHandler::SigDfl,
            SigActionFlags::empty(),
            sigset_t::EMPTY,
        );
        unsafe { linux_api::signal::rt_sigaction(sig, &action, None) }.unwrap();
    }
    let pid = rustix::process::getpid();
    rustix::process::kill_process(pid, rustix::process::Signal::from_raw(sig.into()).unwrap())
        .unwrap();
    unreachable!()
}

/// Handle pending unblocked signals, and return whether *all* corresponding
/// signal actions had the SA_RESTART flag set.
///
/// # Safety
///
/// Configured handlers for all pending unblocked signals must be safe to call. (Which
/// we basically can't ensure).
pub unsafe fn process_signals(mut ucontext: Option<&mut ucontext>) -> bool {
    debug_assert_eq!(ExecutionContext::current(), ExecutionContext::Shadow);

    let mut host = crate::global_host_shmem::get();
    let mut host_lock = host.protected().lock();

    let mut restartable = true;

    loop {
        let Some((sig, siginfo)) = tls_process_shmem::with(|process| {
            tls_thread_shmem::with(|thread| {
                shim_shmem::take_pending_unblocked_signal(&host_lock, process, thread)
            })
        }) else {
            break;
        };

        let action = tls_process_shmem::with(|process| unsafe {
            *process.protected.borrow(&host_lock.root).signal_action(sig)
        });

        if matches!(unsafe { action.handler() }, SignalHandler::SigIgn) {
            continue;
        }

        if matches!(unsafe { action.handler() }, SignalHandler::SigDfl) {
            match defaultaction(sig) {
                linux_api::signal::LinuxDefaultAction::IGN => continue,
                linux_api::signal::LinuxDefaultAction::CORE
                | linux_api::signal::LinuxDefaultAction::TERM => {
                    drop(host_lock);
                    die_with_fatal_signal(sig);
                }
                linux_api::signal::LinuxDefaultAction::STOP => unimplemented!(),
                linux_api::signal::LinuxDefaultAction::CONT => unimplemented!(),
            }
        }

        trace!("Handling emulated signal {sig:?}");

        let (sigaltstack_orig_emu, mask_orig_emu): (stack_t, sigset_t) =
            tls_thread_shmem::with(|thread| {
                let t = thread.protected.borrow(&host_lock.root);
                // SAFETY: Pointers in the sigaltstack are valid in the managed process.
                let stack = unsafe { t.sigaltstack() };
                (*stack, t.blocked_signals)
            });

        let mask_emu_during_handler = {
            let mut m = action.mask() | mask_orig_emu;
            if !action.flags_retain().contains(SigActionFlags::SA_NODEFER) {
                m.add(sig)
            }
            m
        };
        tls_thread_shmem::with(|thread| {
            thread.protected.borrow_mut(&host_lock.root).blocked_signals = mask_emu_during_handler
        });

        if action.flags_retain().contains(SigActionFlags::SA_RESETHAND) {
            tls_process_shmem::with(|process| {
                // SAFETY: The handler (`SigDfl`) is sound.
                unsafe {
                    *process
                        .protected
                        .borrow_mut(&host_lock.root)
                        .signal_action_mut(sig) = sigaction::new_with_default_restorer(
                        SignalHandler::SigDfl,
                        SigActionFlags::empty(),
                        sigset_t::EMPTY,
                    )
                };
            });
        }

        if !action.flags_retain().contains(SigActionFlags::SA_RESTART) {
            restartable = false;
        }

        let sigaltstack_orig_native = if action.flags_retain().contains(SigActionFlags::SA_ONSTACK)
            && !sigaltstack_orig_emu
                .flags_retain()
                .contains(SigAltStackFlags::SS_DISABLE)
        {
            // Call the handler on the configured stack.

            if sigaltstack_orig_emu
                .flags_retain()
                .contains(SigAltStackFlags::SS_ONSTACK)
            {
                // The specified stack is already in use.
                //
                // This *could* be ok, e.g. if the stack is in use by the
                // current thread, and never unwound back to the earlier use;
                // e.g. if the handler exits the process. golang appears to do
                // this in its default SIGTERM handling. (See
                // https://github.com/shadow/shadow/issues/3395).
                //
                // In other cases things could go horribly, but it'd be a bug in
                // the managed process rather than in shadow itself.
                log::debug!(
                    "Signal handler configured to switch to a stack that's already in use. This could go badly."
                )
            }

            // Update the signal-stack configuration while the handler is being run.
            let sigaltstack_emu_during_handler = if sigaltstack_orig_emu
                .flags_retain()
                .contains(SigAltStackFlags::SS_AUTODISARM)
            {
                stack_t::new(core::ptr::null_mut(), SigAltStackFlags::SS_DISABLE, 0)
            } else {
                stack_t::new(
                    sigaltstack_orig_emu.sp(),
                    sigaltstack_orig_emu.flags_retain() | SigAltStackFlags::SS_ONSTACK,
                    sigaltstack_orig_emu.size(),
                )
            };
            tls_thread_shmem::with(|thread| {
                // SAFETY: stack pointer in the assigned stack (if any) is valid in
                // the managed process.
                unsafe {
                    *thread
                        .protected
                        .borrow_mut(&host_lock.root)
                        .sigaltstack_mut() = sigaltstack_emu_during_handler
                };
            });

            let mut sigaltstack_orig_native =
                stack_t::new(core::ptr::null_mut(), SigAltStackFlags::empty(), 0);
            // Set the *native* sigaltstack to the *emulated* sigaltstack,
            // letting the kernel do the stack switch for us.
            unsafe {
                linux_api::signal::sigaltstack(
                    Some(&stack_t::new(
                        sigaltstack_orig_emu.sp(),
                        SigAltStackFlags::SS_AUTODISARM,
                        sigaltstack_orig_emu.size(),
                    )),
                    Some(&mut sigaltstack_orig_native),
                )
            }
            .unwrap();
            Some(sigaltstack_orig_native)
        } else {
            None
        };

        // Package up what our native signal handler will need to invoke the
        // managed code syscall handler for the emulated signal.
        let prev = SIGUSR1_SIGINFO.get().replace(Some(Sigusr1Info {
            native_sigaltstack: sigaltstack_orig_native,
            siginfo,
            action,
            ctx: ucontext
                .as_mut()
                .map(|c| core::ptr::from_mut(*c))
                .unwrap_or(core::ptr::null_mut()),
        }));
        assert!(prev.is_none());

        // Drop locks and references, since the handler could do ~anything,
        // including exit, recurse to here again, or `swapcontext` and never
        // return.
        drop(host_lock);
        drop(host);

        // We raise a signal natively to let the kernel create a ucontext for us
        // and switch stacks. We invoke the managed code's signal handler from our
        // signal handler.
        //
        // We could potentially skip this if the managed code signal handler isn't
        // configured to switch stacks and either doesn't need a context or we already
        // have one. But that'd mean another code path to maintain, and signal
        // handling shouldn't be on the hot path of performance for most
        // applications. (We could also consider implementing the stack switch
        // and/or creation of a ucontext ourselves, but again that would be more
        // complex code to maintain).

        // We install the signal handler every time, so that we can decide
        // whether to set `SA_ONSTACK` or not based on whether we actually need
        // to switch stacks.
        let flags = SigActionFlags::SA_SIGINFO
            | SigActionFlags::SA_NODEFER
            | SigActionFlags::SA_RESETHAND
            | if sigaltstack_orig_native.is_some() {
                SigActionFlags::SA_ONSTACK
            } else {
                SigActionFlags::empty()
            };
        // SAFETY: `handle_sigusr1` is sound, if the handler we're calling is.
        unsafe {
            linux_api::signal::rt_sigaction(
                Signal::SIGUSR1,
                &sigaction::new_with_default_restorer(
                    SignalHandler::Action(handle_sigusr1),
                    flags,
                    sigset_t::EMPTY,
                ),
                None,
            )
        }
        .unwrap();

        let pid = rustix::process::getpid();
        let tid = rustix::thread::gettid();
        linux_api::signal::tgkill(pid.into(), tid.into(), Some(Signal::SIGUSR1)).unwrap();

        // It's not unheard of for a signal handler to use setcontext to jump
        // out of the signal handler to the point where the signal was raised
        // instead of returning normally, e.g. for userspace scheduling.
        // In that case we'll still be in `ExecutionContext::Application`, since
        // the jump would have skipped over the execution context restorer's
        // drop impl above. This is done in our `test_signals.rs` test.
        //
        // Force back to `ExecutionContext::Shadow`.
        ExecutionContext::Shadow.enter_without_restorer();

        // Reacquire locks and references.
        host = crate::global_host_shmem::get();
        host_lock = host.protected().lock();

        // Restore mask and stack
        tls_thread_shmem::with(|thread| {
            let mut thread = thread.protected.borrow_mut(&host_lock.root);
            thread.blocked_signals = mask_orig_emu;
            // SAFETY: Pointers are valid in managed process.
            unsafe { *thread.sigaltstack_mut() = sigaltstack_orig_emu };
            if let Some(s) = sigaltstack_orig_native {
                // SAFETY: We're restoring the previous, presumably valid, stack.
                unsafe { linux_api::signal::sigaltstack(Some(&s), None) }.unwrap();
            }
        });
    }
    restartable
}

/// Handle a hardware error signal that was raised in `exe_ctx`.
///
/// # Safety
///
/// Configured handlers for all pending unblocked signals must be safe to call. (Which
/// we basically can't ensure).
unsafe fn handle_hardware_error_signal_inner(
    exe_ctx: ExecutionContext,
    signal: Signal,
    info: &mut siginfo_t,
    uctx: Option<&mut ucontext>,
) {
    if exe_ctx == ExecutionContext::Shadow {
        // Error was raised from shim code.
        die_with_fatal_signal(signal);
    }

    // Otherwise the error was raised from managed code, and could potentially
    // be handled by a signal handler that it installed.

    tls_thread_shmem::with(|thread| {
        let host = global_host_shmem::get();
        let host_lock = host.protected().lock();
        let pending_signals = thread.protected.borrow(&host_lock.root).pending_signals;
        if pending_signals.has(signal) {
            warn!("Received signal {signal:?} when it was already pending");
        } else {
            let mut thread_protected = thread.protected.borrow_mut(&host_lock.root);
            thread_protected.pending_signals |= signal.into();
            thread_protected.set_pending_standard_siginfo(signal, info);
        }
    });

    unsafe { process_signals(uctx) };
}

unsafe extern "C" fn handle_hardware_error_signal(
    signo: i32,
    info: *mut siginfo_t,
    uctx: *mut core::ffi::c_void,
) {
    let prev_ctx = ExecutionContext::Shadow.enter();
    let signal = Signal::try_from(signo).unwrap();
    // SAFETY: The kernel should have given us a valid `siginfo_t` here.
    let info = unsafe { info.as_mut().unwrap() };
    // SAFETY: The kernel should have given us a valid `ucontext` here.
    let uctx = unsafe { uctx.cast::<ucontext>().as_mut() };
    // SAFETY: We can only assume that the signal handlers are sound.
    unsafe { handle_hardware_error_signal_inner(prev_ctx.ctx(), signal, info, uctx) };
}

pub fn install_hardware_error_handlers() {
    // SA_NODEFER: Don't block the current signal in the handler.
    // Generating one of these signals while it is blocked is
    // undefined behavior; the handler itself detects recursion.
    // SA_SIGINFO: Required because we're specifying
    // sa_sigaction.
    // SA_ONSTACK: Use the alternate signal handling stack,
    // to avoid interfering with userspace thread stacks.
    let flags =
        SigActionFlags::SA_SIGINFO | SigActionFlags::SA_NODEFER | SigActionFlags::SA_ONSTACK;
    let handler = SignalHandler::Action(handle_hardware_error_signal);
    let action = sigaction::new_with_default_restorer(handler, flags, sigset_t::EMPTY);
    for signal in [
        Signal::SIGSEGV,
        Signal::SIGILL,
        Signal::SIGBUS,
        Signal::SIGFPE,
    ] {
        // SAFETY: We've set up a valid handler.
        unsafe { linux_api::signal::rt_sigaction(signal, &action, None) }.unwrap();
    }
}

mod export {
    use super::*;

    /// Handle pending unblocked signals, and return whether *all* corresponding
    /// signal actions had the SA_RESTART flag set.
    ///
    /// `ucontext` will be passed through to handlers if non-NULL. This should
    /// generally only be done if the caller has a `ucontext` that will be swapped to
    /// after this code returns; e.g. one that was passed to our own signal handler,
    /// which will be swapped to when that handler returns.
    ///
    /// If `ucontext` is NULL, one will be created at the point where we invoke
    /// the handler, and swapped back to when it returns.
    /// TODO: Creating `ucontext_t` is currently only implemented for handlers that
    /// execute on a sigaltstack.
    ///
    /// # Safety
    ///
    /// `ucontext` must be dereferenceable if not NULL.
    ///
    /// Configured handlers for all pending unblocked signals must be safe to call. (Which
    /// we basically can't ensure).
    #[unsafe(no_mangle)]
    pub unsafe extern "C-unwind" fn shim_process_signals(ucontext: *mut libc::ucontext_t) -> bool {
        // `libc::ucontext_t` appears to be safe to cast to a kernel `ucontext`; as
        // verified experimentally and by manual inspection of the definitions.
        //
        // The libc definition has some extra fields at the end, but we're
        // careful not to copy the ucontext so they shouldn't hurt anything.
        let ucontext: *mut ucontext = ucontext.cast();

        // SAFETY: ensured by caller.
        unsafe { process_signals(ucontext.as_mut()) }
    }

    /// Install signal handlers for signals that can be generated by hardware errors.
    /// e.g. SIGSEGV
    #[unsafe(no_mangle)]
    pub unsafe extern "C-unwind" fn shim_install_hardware_error_handlers() {
        install_hardware_error_handlers()
    }

    /// Handle a hardware error signal that was raised in `exe_ctx`.
    ///
    /// More-specialized error handlers (e.g. for rdtsc) can invoke this handler
    /// directly when unable to handle the current signal (e.g. when a SIGSEGV wasn't
    /// caused by an rdtsc instruction).
    ///
    /// # Safety
    ///
    /// `info` and `ctx` must be non-NULL and safely dereferenceable.
    #[unsafe(no_mangle)]
    pub unsafe extern "C-unwind" fn shim_handle_hardware_error_signal(
        exe_ctx: ExecutionContext,
        signo: i32,
        info: *mut siginfo_t,
        uctx: *mut linux_api::ucontext::linux_ucontext,
    ) {
        let signal = Signal::try_from(signo).unwrap();
        // SAFETY: Caller ensures.
        let info = unsafe { info.as_mut().unwrap() };
        // SAFETY: Caller ensures.
        let uctx = unsafe { uctx.as_mut() };
        // SAFETY: We can only assume that the signal handlers are sound.
        unsafe {
            handle_hardware_error_signal_inner(exe_ctx, signal, info, uctx);
        }
    }
}
