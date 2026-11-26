windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolWork(pwk : PTP_WORK));
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolWork(pfnwk : PTP_WORK_CALLBACK, pv : PVOID, pcbe : PTP_CALLBACK_ENVIRON) -> PTP_WORK);
windows_link::link!("kernel32.dll" "system" fn SubmitThreadpoolWork(pwk : PTP_WORK));
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolWorkCallbacks(pwk : PTP_WORK, fcancelpendingcallbacks : BOOL));
pub type BOOL = i32;
pub type DWORD = u32;
pub type PTP_CALLBACK_ENVIRON = *mut TP_CALLBACK_ENVIRON_V3;
pub type PTP_CALLBACK_INSTANCE = *mut _TP_CALLBACK_INSTANCE;
pub type PTP_CLEANUP_GROUP = *mut _TP_CLEANUP_GROUP;
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK =
    Option<unsafe extern "system" fn(objectcontext: PVOID, cleanupcontext: PVOID)>;
pub type PTP_POOL = *mut _TP_POOL;
pub type PTP_SIMPLE_CALLBACK =
    Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: PVOID)>;
pub type PTP_WORK = *mut _TP_WORK;
pub type PTP_WORK_CALLBACK = Option<
    unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: PVOID, work: PTP_WORK),
>;
pub type PVOID = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TP_CALLBACK_ENVIRON_V3 {
    pub Version: TP_VERSION,
    pub Pool: PTP_POOL,
    pub CleanupGroup: PTP_CLEANUP_GROUP,
    pub CleanupGroupCancelCallback: PTP_CLEANUP_GROUP_CANCEL_CALLBACK,
    pub RaceDll: PVOID,
    pub ActivationContext: *mut _ACTIVATION_CONTEXT,
    pub FinalizationCallback: PTP_SIMPLE_CALLBACK,
    pub u: TP_CALLBACK_ENVIRON_V3_0,
    pub CallbackPriority: TP_CALLBACK_PRIORITY,
    pub Size: DWORD,
}
impl Default for TP_CALLBACK_ENVIRON_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TP_CALLBACK_ENVIRON_V3_0 {
    pub Flags: DWORD,
    pub s: TP_CALLBACK_ENVIRON_V3_0_0,
}
impl Default for TP_CALLBACK_ENVIRON_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TP_CALLBACK_ENVIRON_V3_0_0 {
    pub _bitfield: u32,
}
pub type TP_CALLBACK_PRIORITY = i32;
pub type TP_VERSION = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _ACTIVATION_CONTEXT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _TP_CALLBACK_INSTANCE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _TP_CLEANUP_GROUP(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _TP_POOL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _TP_WORK(pub u8);
