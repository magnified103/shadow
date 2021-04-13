/* automatically generated by rust-bindgen */
use crate::host::descriptor::CompatDescriptor;

pub const CONFIG_PIPE_BUFFER_SIZE: u32 = 65536;
pub const SYSCALL_IO_BUFSIZE: u32 = 10485760;
pub type size_t = ::std::os::raw::c_ulong;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __pid_t = ::std::os::raw::c_int;
pub type pid_t = __pid_t;
pub type gint = ::std::os::raw::c_int;
pub type guint = ::std::os::raw::c_uint;
pub type gdouble = f64;
pub type gpointer = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GTimer {
    _unused: [u8; 0],
}
pub type GTimer = _GTimer;
pub use self::_LogLevel as LogLevel;
pub const _LogLevel_LOGLEVEL_UNSET: _LogLevel = 0;
pub const _LogLevel_LOGLEVEL_ERROR: _LogLevel = 1;
pub const _LogLevel_LOGLEVEL_CRITICAL: _LogLevel = 2;
pub const _LogLevel_LOGLEVEL_WARNING: _LogLevel = 3;
pub const _LogLevel_LOGLEVEL_MESSAGE: _LogLevel = 4;
pub const _LogLevel_LOGLEVEL_INFO: _LogLevel = 5;
pub const _LogLevel_LOGLEVEL_DEBUG: _LogLevel = 6;
pub const _LogLevel_LOGLEVEL_TRACE: _LogLevel = 7;
pub type _LogLevel = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ShadowLogger {
    _unused: [u8; 0],
}
pub type ShadowLogger = _ShadowLogger;
extern "C" {
    pub fn shadow_logger_getDefault() -> *mut ShadowLogger;
}
extern "C" {
    pub fn shadow_logger_shouldFilter(logger: *mut ShadowLogger, level: LogLevel) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Process {
    _unused: [u8; 0],
}
pub type Process = _Process;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Host {
    _unused: [u8; 0],
}
pub type Host = _Host;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Counter {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PosixFileArc {
    _unused: [u8; 0],
}
pub use self::_Status as Status;
pub const _Status_STATUS_NONE: _Status = 0;
pub const _Status_STATUS_DESCRIPTOR_ACTIVE: _Status = 1;
pub const _Status_STATUS_DESCRIPTOR_READABLE: _Status = 2;
pub const _Status_STATUS_DESCRIPTOR_WRITABLE: _Status = 4;
pub const _Status_STATUS_DESCRIPTOR_CLOSED: _Status = 8;
pub const _Status_STATUS_FUTEX_WAKEUP: _Status = 16;
pub type _Status = i32;
pub type LegacyDescriptor = [u64; 7usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _StatusListener {
    _unused: [u8; 0],
}
pub type StatusListener = _StatusListener;
extern "C" {
    pub fn statuslistener_ref(listener: *mut StatusListener);
}
extern "C" {
    pub fn statuslistener_unref(listener: *mut StatusListener);
}
extern "C" {
    pub fn statuslistener_onStatusChanged(
        listener: *mut StatusListener,
        currentStatus: Status,
        transitions: Status,
    );
}
pub type SysCallHandler = _SysCallHandler;
pub type PluginVirtualPtr = _PluginVirtualPtr;
pub type PluginPtr = _PluginVirtualPtr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PluginVirtualPtr {
    pub val: u64,
}
#[test]
fn bindgen_test_layout__PluginVirtualPtr() {
    assert_eq!(
        ::std::mem::size_of::<_PluginVirtualPtr>(),
        8usize,
        concat!("Size of: ", stringify!(_PluginVirtualPtr))
    );
    assert_eq!(
        ::std::mem::align_of::<_PluginVirtualPtr>(),
        8usize,
        concat!("Alignment of ", stringify!(_PluginVirtualPtr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PluginVirtualPtr>())).val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PluginVirtualPtr),
            "::",
            stringify!(val)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _SysCallReg {
    pub as_i64: i64,
    pub as_u64: u64,
    pub as_ptr: PluginPtr,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout__SysCallReg() {
    assert_eq!(
        ::std::mem::size_of::<_SysCallReg>(),
        8usize,
        concat!("Size of: ", stringify!(_SysCallReg))
    );
    assert_eq!(
        ::std::mem::align_of::<_SysCallReg>(),
        8usize,
        concat!("Alignment of ", stringify!(_SysCallReg))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallReg>())).as_i64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallReg),
            "::",
            stringify!(as_i64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallReg>())).as_u64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallReg),
            "::",
            stringify!(as_u64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallReg>())).as_ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallReg),
            "::",
            stringify!(as_ptr)
        )
    );
}
pub type SysCallReg = _SysCallReg;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SysCallArgs {
    pub number: ::std::os::raw::c_long,
    pub args: [SysCallReg; 6usize],
}
#[test]
fn bindgen_test_layout__SysCallArgs() {
    assert_eq!(
        ::std::mem::size_of::<_SysCallArgs>(),
        56usize,
        concat!("Size of: ", stringify!(_SysCallArgs))
    );
    assert_eq!(
        ::std::mem::align_of::<_SysCallArgs>(),
        8usize,
        concat!("Alignment of ", stringify!(_SysCallArgs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallArgs>())).number as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallArgs),
            "::",
            stringify!(number)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallArgs>())).args as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallArgs),
            "::",
            stringify!(args)
        )
    );
}
pub type SysCallArgs = _SysCallArgs;
pub const SysCallReturnState_SYSCALL_DONE: SysCallReturnState = 0;
pub const SysCallReturnState_SYSCALL_BLOCK: SysCallReturnState = 1;
pub const SysCallReturnState_SYSCALL_NATIVE: SysCallReturnState = 2;
pub type SysCallReturnState = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _SysCallCondition {
    _unused: [u8; 0],
}
pub type SysCallCondition = _SysCallCondition;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SysCallReturn {
    pub state: SysCallReturnState,
    pub retval: SysCallReg,
    pub cond: *mut SysCallCondition,
}
#[test]
fn bindgen_test_layout__SysCallReturn() {
    assert_eq!(
        ::std::mem::size_of::<_SysCallReturn>(),
        24usize,
        concat!("Size of: ", stringify!(_SysCallReturn))
    );
    assert_eq!(
        ::std::mem::align_of::<_SysCallReturn>(),
        8usize,
        concat!("Alignment of ", stringify!(_SysCallReturn))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallReturn>())).state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallReturn),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallReturn>())).retval as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallReturn),
            "::",
            stringify!(retval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallReturn>())).cond as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallReturn),
            "::",
            stringify!(cond)
        )
    );
}
pub type SysCallReturn = _SysCallReturn;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Thread {
    _unused: [u8; 0],
}
pub type Thread = _Thread;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ShMemBlock {
    pub p: *mut ::std::os::raw::c_void,
    pub nbytes: size_t,
}
#[test]
fn bindgen_test_layout__ShMemBlock() {
    assert_eq!(
        ::std::mem::size_of::<_ShMemBlock>(),
        16usize,
        concat!("Size of: ", stringify!(_ShMemBlock))
    );
    assert_eq!(
        ::std::mem::align_of::<_ShMemBlock>(),
        8usize,
        concat!("Alignment of ", stringify!(_ShMemBlock))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ShMemBlock>())).p as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_ShMemBlock),
            "::",
            stringify!(p)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ShMemBlock>())).nbytes as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_ShMemBlock),
            "::",
            stringify!(nbytes)
        )
    );
}
pub type ShMemBlock = _ShMemBlock;
extern "C" {
    pub fn thread_ref(thread: *mut Thread);
}
extern "C" {
    pub fn thread_unref(thread: *mut Thread);
}
extern "C" {
    pub fn thread_run(
        thread: *mut Thread,
        argv: *mut *mut ::std::os::raw::c_char,
        envv: *mut *mut ::std::os::raw::c_char,
        workingDir: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn thread_resume(thread: *mut Thread);
}
extern "C" {
    pub fn thread_handleProcessExit(thread: *mut Thread);
}
extern "C" {
    pub fn thread_getReturnCode(thread: *mut Thread) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn thread_readPtr(
        thread: *mut Thread,
        dst: *mut ::std::os::raw::c_void,
        src: PluginVirtualPtr,
        n: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn thread_readStringPtr(
        base: *mut Thread,
        dst: *mut ::std::os::raw::c_char,
        src: PluginVirtualPtr,
        n: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn thread_writePtr(
        thread: *mut Thread,
        dst: PluginVirtualPtr,
        src: *const ::std::os::raw::c_void,
        n: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn thread_getReadablePtr(
        thread: *mut Thread,
        plugin_src: PluginPtr,
        n: size_t,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn thread_getReadableString(
        thread: *mut Thread,
        plugin_src: PluginPtr,
        n: size_t,
        str_: *mut *const ::std::os::raw::c_char,
        strlen: *mut size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn thread_getWriteablePtr(
        thread: *mut Thread,
        plugin_src: PluginPtr,
        n: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn thread_getMutablePtr(
        thread: *mut Thread,
        plugin_src: PluginPtr,
        n: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn thread_flushPtrs(thread: *mut Thread);
}
extern "C" {
    pub fn thread_nativeSyscall(
        thread: *mut Thread,
        n: ::std::os::raw::c_long,
        ...
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn thread_mallocPluginPtr(thread: *mut Thread, size: size_t) -> PluginPtr;
}
extern "C" {
    pub fn thread_freePluginPtr(thread: *mut Thread, ptr: PluginPtr, size: size_t);
}
extern "C" {
    pub fn thread_isRunning(thread: *mut Thread) -> bool;
}
extern "C" {
    pub fn thread_getProcessId(thread: *mut Thread) -> u32;
}
extern "C" {
    pub fn thread_getHostId(thread: *mut Thread) -> u32;
}
extern "C" {
    pub fn thread_getNativePid(thread: *mut Thread) -> pid_t;
}
extern "C" {
    pub fn thread_getNativeTid(thread: *mut Thread) -> pid_t;
}
extern "C" {
    pub fn thread_getID(thread: *mut Thread) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn thread_clone(
        thread: *mut Thread,
        flags: ::std::os::raw::c_ulong,
        child_stack: PluginPtr,
        ptid: PluginPtr,
        ctid: PluginPtr,
        newtls: ::std::os::raw::c_ulong,
        child: *mut *mut Thread,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn thread_setTidAddress(thread: *mut Thread, addr: PluginVirtualPtr);
}
extern "C" {
    pub fn thread_getTidAddress(thread: *mut Thread) -> PluginVirtualPtr;
}
extern "C" {
    pub fn thread_isLeader(thread: *mut Thread) -> bool;
}
extern "C" {
    pub fn thread_getIPCBlock(thread: *mut Thread) -> *mut ShMemBlock;
}
extern "C" {
    pub fn thread_getShMBlock(thread: *mut Thread) -> *mut ShMemBlock;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Timer {
    _unused: [u8; 0],
}
pub type Timer = _Timer;
extern "C" {
    pub fn process_registerCompatDescriptor(
        proc_: *mut Process,
        compatDesc: *mut CompatDescriptor,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn process_deregisterCompatDescriptor(
        proc_: *mut Process,
        handle: ::std::os::raw::c_int,
    ) -> *mut CompatDescriptor;
}
extern "C" {
    pub fn process_getRegisteredCompatDescriptor(
        proc_: *mut Process,
        handle: ::std::os::raw::c_int,
    ) -> *mut CompatDescriptor;
}
extern "C" {
    pub fn process_readPtr(
        proc_: *mut Process,
        dst: *mut ::std::os::raw::c_void,
        src: PluginVirtualPtr,
        n: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn process_readStringPtr(
        proc_: *mut Process,
        dst: *mut ::std::os::raw::c_char,
        src: PluginVirtualPtr,
        n: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn process_writePtr(
        proc_: *mut Process,
        dst: PluginVirtualPtr,
        src: *const ::std::os::raw::c_void,
        n: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn process_getReadablePtr(
        proc_: *mut Process,
        plugin_src: PluginPtr,
        n: size_t,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn process_getWriteablePtr(
        proc_: *mut Process,
        plugin_src: PluginPtr,
        n: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn process_getMutablePtr(
        proc_: *mut Process,
        plugin_src: PluginPtr,
        n: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn process_parseArgStr(
        commandLine: *const ::std::os::raw::c_char,
        argc: *mut ::std::os::raw::c_int,
        argv: *mut *mut *mut ::std::os::raw::c_char,
        error: *mut *mut ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn process_parseArgStrFree(
        argv: *mut *mut ::std::os::raw::c_char,
        error: *mut ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn descriptor_unref(data: gpointer);
}
extern "C" {
    pub fn descriptor_setHandle(descriptor: *mut LegacyDescriptor, handle: gint);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Futex {
    _unused: [u8; 0],
}
pub type Futex = _Futex;
extern "C" {
    pub fn worker_getActiveProcess() -> *mut Process;
}
pub use self::_TriggerType as TriggerType;
pub const _TriggerType_TRIGGER_NONE: _TriggerType = 0;
pub const _TriggerType_TRIGGER_DESCRIPTOR: _TriggerType = 1;
pub const _TriggerType_TRIGGER_POSIX_FILE: _TriggerType = 2;
pub const _TriggerType_TRIGGER_FUTEX: _TriggerType = 3;
pub type _TriggerType = i32;
pub type TriggerObject = _TriggerObject;
#[repr(C)]
#[derive(Copy, Clone)]
pub union _TriggerObject {
    pub as_pointer: *mut ::std::os::raw::c_void,
    pub as_descriptor: *mut LegacyDescriptor,
    pub as_file: *const PosixFileArc,
    pub as_futex: *mut Futex,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout__TriggerObject() {
    assert_eq!(
        ::std::mem::size_of::<_TriggerObject>(),
        8usize,
        concat!("Size of: ", stringify!(_TriggerObject))
    );
    assert_eq!(
        ::std::mem::align_of::<_TriggerObject>(),
        8usize,
        concat!("Alignment of ", stringify!(_TriggerObject))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_TriggerObject>())).as_pointer as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_TriggerObject),
            "::",
            stringify!(as_pointer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_TriggerObject>())).as_descriptor as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_TriggerObject),
            "::",
            stringify!(as_descriptor)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_TriggerObject>())).as_file as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_TriggerObject),
            "::",
            stringify!(as_file)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_TriggerObject>())).as_futex as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_TriggerObject),
            "::",
            stringify!(as_futex)
        )
    );
}
pub type Trigger = _Trigger;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _Trigger {
    pub type_: TriggerType,
    pub object: TriggerObject,
    pub status: Status,
}
#[test]
fn bindgen_test_layout__Trigger() {
    assert_eq!(
        ::std::mem::size_of::<_Trigger>(),
        24usize,
        concat!("Size of: ", stringify!(_Trigger))
    );
    assert_eq!(
        ::std::mem::align_of::<_Trigger>(),
        8usize,
        concat!("Alignment of ", stringify!(_Trigger))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Trigger>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Trigger),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Trigger>())).object as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_Trigger),
            "::",
            stringify!(object)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Trigger>())).status as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_Trigger),
            "::",
            stringify!(status)
        )
    );
}
extern "C" {
    pub fn syscallcondition_new(trigger: Trigger, timeout: *mut Timer) -> *mut SysCallCondition;
}
extern "C" {
    pub fn syscallcondition_unref(cond: *mut SysCallCondition);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Epoll {
    _unused: [u8; 0],
}
pub type Epoll = _Epoll;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _SysCallHandler {
    pub host: *mut Host,
    pub process: *mut Process,
    pub thread: *mut Thread,
    pub timer: *mut Timer,
    pub epoll: *mut Epoll,
    pub blockedSyscallNR: ::std::os::raw::c_long,
    pub perfTimer: *mut GTimer,
    pub perfSecondsCurrent: gdouble,
    pub perfSecondsTotal: gdouble,
    pub numSyscalls: ::std::os::raw::c_long,
    pub syscall_counter: *mut Counter,
    pub referenceCount: ::std::os::raw::c_int,
    pub magic: guint,
}
#[test]
fn bindgen_test_layout__SysCallHandler() {
    assert_eq!(
        ::std::mem::size_of::<_SysCallHandler>(),
        96usize,
        concat!("Size of: ", stringify!(_SysCallHandler))
    );
    assert_eq!(
        ::std::mem::align_of::<_SysCallHandler>(),
        8usize,
        concat!("Alignment of ", stringify!(_SysCallHandler))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallHandler>())).host as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallHandler),
            "::",
            stringify!(host)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallHandler>())).process as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallHandler),
            "::",
            stringify!(process)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallHandler>())).thread as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallHandler),
            "::",
            stringify!(thread)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallHandler>())).timer as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallHandler),
            "::",
            stringify!(timer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallHandler>())).epoll as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallHandler),
            "::",
            stringify!(epoll)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_SysCallHandler>())).blockedSyscallNR as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallHandler),
            "::",
            stringify!(blockedSyscallNR)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallHandler>())).perfTimer as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallHandler),
            "::",
            stringify!(perfTimer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_SysCallHandler>())).perfSecondsCurrent as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallHandler),
            "::",
            stringify!(perfSecondsCurrent)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_SysCallHandler>())).perfSecondsTotal as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallHandler),
            "::",
            stringify!(perfSecondsTotal)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallHandler>())).numSyscalls as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallHandler),
            "::",
            stringify!(numSyscalls)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallHandler>())).syscall_counter as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallHandler),
            "::",
            stringify!(syscall_counter)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallHandler>())).referenceCount as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallHandler),
            "::",
            stringify!(referenceCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SysCallHandler>())).magic as *const _ as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(_SysCallHandler),
            "::",
            stringify!(magic)
        )
    );
}
extern "C" {
    pub fn syscallhandler_close(
        sys: *mut SysCallHandler,
        args: *const SysCallArgs,
    ) -> SysCallReturn;
}
extern "C" {
    pub fn syscallhandler_dup(sys: *mut SysCallHandler, args: *const SysCallArgs) -> SysCallReturn;
}
extern "C" {
    pub fn syscallhandler_exit_group(
        sys: *mut SysCallHandler,
        args: *const SysCallArgs,
    ) -> SysCallReturn;
}
extern "C" {
    pub fn syscallhandler_getpid(
        sys: *mut SysCallHandler,
        args: *const SysCallArgs,
    ) -> SysCallReturn;
}
extern "C" {
    pub fn syscallhandler_getppid(
        sys: *mut SysCallHandler,
        args: *const SysCallArgs,
    ) -> SysCallReturn;
}
extern "C" {
    pub fn syscallhandler_pipe(sys: *mut SysCallHandler, args: *const SysCallArgs)
        -> SysCallReturn;
}
extern "C" {
    pub fn syscallhandler_pipe2(
        sys: *mut SysCallHandler,
        args: *const SysCallArgs,
    ) -> SysCallReturn;
}
extern "C" {
    pub fn syscallhandler_pread64(
        sys: *mut SysCallHandler,
        args: *const SysCallArgs,
    ) -> SysCallReturn;
}
extern "C" {
    pub fn syscallhandler_pwrite64(
        sys: *mut SysCallHandler,
        args: *const SysCallArgs,
    ) -> SysCallReturn;
}
extern "C" {
    pub fn syscallhandler_read(sys: *mut SysCallHandler, args: *const SysCallArgs)
        -> SysCallReturn;
}
extern "C" {
    pub fn syscallhandler_set_tid_address(
        sys: *mut SysCallHandler,
        args: *const SysCallArgs,
    ) -> SysCallReturn;
}
extern "C" {
    pub fn syscallhandler_uname(
        sys: *mut SysCallHandler,
        args: *const SysCallArgs,
    ) -> SysCallReturn;
}
extern "C" {
    pub fn syscallhandler_write(
        sys: *mut SysCallHandler,
        args: *const SysCallArgs,
    ) -> SysCallReturn;
}
