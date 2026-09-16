windows_link::link!("kernel32.dll" "system" fn CloseThreadpool(ptpp : PTP_POOL));
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroup(ptpcg : PTP_CLEANUP_GROUP));
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroupMembers(ptpcg : PTP_CLEANUP_GROUP, fcancelpendingcallbacks : BOOL, pvcleanupcontext : *mut core::ffi::c_void));
windows_link::link!("kernel32.dll" "system" fn CreateThreadpool(reserved : *mut core::ffi::c_void) -> PTP_POOL);
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolCleanupGroup() -> PTP_CLEANUP_GROUP);
windows_link::link!("kernel32.dll" "system" fn GetCurrentThreadId() -> u32);
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolThreadMaximum(ptpp : PTP_POOL, cthrdmost : u32));
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolThreadMinimum(ptpp : PTP_POOL, cthrdmic : u32) -> BOOL);
windows_link::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds : u32));
windows_link::link!("kernel32.dll" "system" fn TrySubmitThreadpoolCallback(pfns : PTP_SIMPLE_CALLBACK, pv : *mut core::ffi::c_void, pcbe : PTP_CALLBACK_ENVIRON) -> BOOL);
pub type BOOL = i32;
pub type PTP_CALLBACK_ENVIRON = *mut TP_CALLBACK_ENVIRON_V3;
pub type PTP_CALLBACK_INSTANCE = *mut TP_CALLBACK_INSTANCE;
pub type PTP_CLEANUP_GROUP = *mut TP_CLEANUP_GROUP;
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK = Option<
    unsafe extern "system" fn(
        objectcontext: *mut core::ffi::c_void,
        cleanupcontext: *mut core::ffi::c_void,
    ),
>;
pub type PTP_POOL = *mut TP_POOL;
pub type PTP_SIMPLE_CALLBACK = Option<
    unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void),
>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TP_CALLBACK_ENVIRON_V3 {
    pub Version: TP_VERSION,
    pub Pool: PTP_POOL,
    pub CleanupGroup: PTP_CLEANUP_GROUP,
    pub CleanupGroupCancelCallback: PTP_CLEANUP_GROUP_CANCEL_CALLBACK,
    pub RaceDll: *mut core::ffi::c_void,
    pub ActivationContext: *mut _ACTIVATION_CONTEXT,
    pub FinalizationCallback: PTP_SIMPLE_CALLBACK,
    pub u: TP_CALLBACK_ENVIRON_V3_0,
    pub CallbackPriority: TP_CALLBACK_PRIORITY,
    pub Size: u32,
}
impl Default for TP_CALLBACK_ENVIRON_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TP_CALLBACK_ENVIRON_V3_0 {
    pub Flags: u32,
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TP_CALLBACK_INSTANCE(pub u8);
pub type TP_CALLBACK_PRIORITY = i32;
pub const TP_CALLBACK_PRIORITY_NORMAL: TP_CALLBACK_PRIORITY = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TP_CLEANUP_GROUP(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TP_POOL(pub u8);
pub type TP_VERSION = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _ACTIVATION_CONTEXT(pub u8);
