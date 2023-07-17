use linux_api::ucontext::{sigcontext, ucontext};
use static_assertions::const_assert_eq;

/// Used below to validate the offset of `field` from `base`.
/// TODO: replace with `core::ptr::offset_of` once stabilized.
/// https://github.com/rust-lang/rust/issues/106655
fn sigcontext_offset_of(base: &sigcontext, field: &u64) -> usize {
    let base = base as *const _ as usize;
    let field = field as *const _ as usize;
    field - base
}

/// Execute a native `clone` syscall. The newly created child thread will
/// resume execution from `ctx`, which should be the point where the managed
/// code originally made a `clone` syscall (but was intercepted by seccomp).
///
/// # Safety
///
/// * `ctx` must be dereferenceable, and must be safe for the newly spawned
/// child thread to restore.
/// * Other pointers, if non-null, must be safely dereferenceable.
/// * `child_stack` must be "sufficiently big" for the child thread to run on.
/// * `tls` if provided must point to correctly initialized thread local storage.
pub unsafe fn do_clone(
    ctx: &ucontext,
    flags: i32,
    child_stack: *mut u8,
    ptid: *mut i32,
    ctid: *mut i32,
    newtls: u64,
) -> i64 {
    assert!(
        !child_stack.is_null(),
        "clone without a new stack not implemented"
    );

    // x86-64 calling conventions require a 16-byte aligned stack
    assert_eq!(
        child_stack.align_offset(16),
        0,
        "clone with unaligned new stack not implemented"
    );

    // Copy ctx to top of the child stack.
    // SAFETY: Should still point within stack, assuming it fits.
    let child_stack = unsafe { child_stack.sub(core::mem::size_of::<sigcontext>()) };
    assert_eq!(
        child_stack.align_offset(16),
        0,
        "realignment not implemented"
    );
    assert_eq!(
        child_stack.align_offset(core::mem::align_of::<sigcontext>()),
        0,
        "realignment not implemented"
    );
    unsafe { core::ptr::write(child_stack.cast::<sigcontext>(), ctx.uc_mcontext) };

    let sigctx = &ctx.uc_mcontext;
    // These offsets are hard-coded into the asm format string below.
    // TODO: turn these into const parameters to the asm block when const
    // asm parameters are stabilized.
    // https://github.com/rust-lang/rust/issues/93332
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.r8), 0);
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.r9), 0x8);
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.r10), 0x10);
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.r11), 0x18);
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.r12), 0x20);
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.r13), 0x28);
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.r14), 0x30);
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.r15), 0x38);
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.rsi), 0x48);
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.rdi), 0x40);
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.rbx), 0x58);
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.rdx), 0x60);
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.rbp), 0x50);
    debug_assert_eq!(sigcontext_offset_of(sigctx, &sigctx.rip), 0x80);

    const_assert_eq!(core::mem::size_of::<sigcontext>(), 0x100);

    let rv: i64;
    // SAFETY:
    // * There's currently no way to tell Rust than an asm block "returns twice",
    //   so the child does not return from this asm block. It instead *jumps*
    //   to the point where the clone syscall war originally made.
    unsafe {
        core::arch::asm!(
            // Make the clone syscall
            "syscall",
            // If in the parent, exit the asm block (by jumping forward to the label
            // `2`). https://doc.rust-lang.org/rust-by-example/unsafe/asm.html#labels
            "cmp rax, 0",
            "jne 2f",
            // Initialize state for this thread
            "call {shim_ensure_init}",
            // Restore general purpose registers.
            // Offsets are validated in assertions above.
            "mov r8, [rsp+0x0]",
            "mov r9, [rsp+0x8]",
            "mov r10, [rsp+0x10]",
            "mov r11, [rsp+0x18]",
            "mov r12, [rsp+0x20]",
            "mov r13, [rsp+0x28]",
            "mov r14, [rsp+0x30]",
            "mov r15, [rsp+0x38]",
            "mov rsi, [rsp+0x48]",
            "mov rdi, [rsp+0x40]",
            "mov rbx, [rsp+0x58]",
            "mov rdx, [rsp+0x60]",
            "mov rbp, [rsp+0x50]",
            // Store `rip` in `rax`, to be pushed after
            // restoring `rsp`
            "mov rax, [rsp+0x80]",
            // "Free" ctx from the stack
            "add rsp, 0x100",
            // Push original `rip` onto the stack
            "push rax",
            // Not restored:
            // - RAX: stores the result of the syscall, which we set below.
            // - Floating point and other special registers: hopefully not needed.

            // Set return value of clone syscall
            "mov rax, 0",
            // Ret to original RIP
            "ret",
            "2:",
            // clone syscall number in, rv out
            inout("rax") libc::SYS_clone => rv,
            // clone syscall arg1
            in("rdi") flags,
            // clone syscall arg2
            in("rsi") child_stack,
            // clone syscall arg3
            in("rdx") ptid,
            // clone syscall arg4
            in("r10") ctid,
            // clone syscall arg5
            in("r8") newtls,
            shim_ensure_init = sym crate::bindings::shim_ensure_init,
        )
    }
    unsafe { crate::bindings::shim_newThreadFinish() };
    rv
}

pub mod export {
    use super::*;

    /// Execute a native `clone` syscall. The newly created child thread will
    /// resume execution from `ctx`, which should be the point where the managed
    /// code originally made a `clone` syscall (but was intercepted by seccomp).
    ///
    /// # Safety
    ///
    /// * `ctx` must be dereferenceable, and must be safe for the newly spawned
    /// child thread to restore.
    /// * Other pointers, if non-null, must be safely dereferenceable.
    /// * `child_stack` must be "sufficiently big" for the child thread to run on.
    /// * `tls` if provided must point to correctly initialized thread local storage.
    #[no_mangle]
    pub unsafe extern "C" fn shim_do_clone(
        ctx: *const libc::ucontext_t,
        flags: i32,
        child_stack: *mut u8,
        ptid: *mut i32,
        ctid: *mut i32,
        newtls: u64,
    ) -> i64 {
        assert!(
            !ctx.is_null(),
            "clone without signal ucontext unimplemented"
        );
        // Cast from libc to linux kernel context. These are compatible in practice.
        // TODO: Change calling code to use linux kernel context.
        let ctx = ctx.cast::<linux_api::ucontext::ucontext>();
        let ctx = unsafe { ctx.as_ref().unwrap() };

        unsafe { do_clone(ctx, flags, child_stack, ptid, ctid, newtls) }
    }
}
