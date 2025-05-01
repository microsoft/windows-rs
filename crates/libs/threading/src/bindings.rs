use super::*;
use windows_link::link;

// TODO: try again to get this generated with windows-bindgen

pub type PTP_POOL = isize;
pub type PTP_CLEANUP_GROUP = isize;
pub type BOOL = i32;

pub type TP_CALLBACK_PRIORITY = i32;
pub const TP_CALLBACK_PRIORITY_HIGH: TP_CALLBACK_PRIORITY = 0i32;
pub const TP_CALLBACK_PRIORITY_LOW: TP_CALLBACK_PRIORITY = 2i32;
pub const TP_CALLBACK_PRIORITY_NORMAL: TP_CALLBACK_PRIORITY = 1i32;

pub type PTP_SIMPLE_CALLBACK =
    unsafe extern "system" fn(instance: *const c_void, context: *const c_void);

pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK =
    Option<unsafe extern "system" fn(objectcontext: *mut c_void, cleanupcontext: *mut c_void)>;

#[repr(C)]
#[derive(Default)]
pub struct TP_CALLBACK_ENVIRON_V3 {
    pub Version: u32,
    pub Pool: PTP_POOL,
    pub CleanupGroup: PTP_CLEANUP_GROUP,
    pub CleanupGroupCancelCallback: PTP_CLEANUP_GROUP_CANCEL_CALLBACK,
    pub RaceDll: isize,
    pub ActivationContext: isize,
    pub FinalizationCallback: isize,
    pub u: TP_CALLBACK_ENVIRON_V3_0,
    pub CallbackPriority: TP_CALLBACK_PRIORITY,
    pub Size: u32,
}

#[repr(C)]
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
#[derive(Copy, Clone)]
pub struct TP_CALLBACK_ENVIRON_V3_0_0 {
    pub _bitfield: u32,
}

link!("kernel32.dll" "system" fn CreateThreadpool(reserved: *const c_void) -> PTP_POOL);
link!("kernel32.dll" "system" fn SetThreadpoolThreadMaximum(pool: PTP_POOL, max: u32));
link!("kernel32.dll" "system" fn SetThreadpoolThreadMinimum(pool: PTP_POOL, min: u32) -> BOOL);
link!("kernel32.dll" "system" fn TrySubmitThreadpoolCallback(callback: PTP_SIMPLE_CALLBACK, context: *const c_void, environment: *const TP_CALLBACK_ENVIRON_V3) -> BOOL);
link!("kernel32.dll" "system" fn CloseThreadpool(pool: PTP_POOL));
link!("kernel32.dll" "system" fn CreateThreadpoolCleanupGroup() -> PTP_CLEANUP_GROUP);
link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroup(group: PTP_CLEANUP_GROUP));
link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroupMembers(group: PTP_CLEANUP_GROUP, cancel_pending_callbacks: BOOL, context: *mut c_void));
