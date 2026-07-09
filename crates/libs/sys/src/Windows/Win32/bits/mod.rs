pub type BG_ERROR_CONTEXT = i32;
pub const BG_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER: BG_ERROR_CONTEXT = 2;
pub const BG_ERROR_CONTEXT_GENERAL_TRANSPORT: BG_ERROR_CONTEXT = 6;
pub const BG_ERROR_CONTEXT_LOCAL_FILE: BG_ERROR_CONTEXT = 4;
pub const BG_ERROR_CONTEXT_NONE: BG_ERROR_CONTEXT = 0;
pub const BG_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION: BG_ERROR_CONTEXT = 3;
pub const BG_ERROR_CONTEXT_REMOTE_APPLICATION: BG_ERROR_CONTEXT = 7;
pub const BG_ERROR_CONTEXT_REMOTE_FILE: BG_ERROR_CONTEXT = 5;
pub const BG_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK: BG_ERROR_CONTEXT = 8;
pub const BG_ERROR_CONTEXT_UNKNOWN: BG_ERROR_CONTEXT = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BG_FILE_INFO {
    pub RemoteName: windows_sys::core::PWSTR,
    pub LocalName: windows_sys::core::PWSTR,
}
impl Default for BG_FILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BG_FILE_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub Completed: windows_sys::core::BOOL,
}
pub const BG_JOB_ENUM_ALL_USERS: u32 = 1;
pub type BG_JOB_PRIORITY = i32;
pub const BG_JOB_PRIORITY_FOREGROUND: BG_JOB_PRIORITY = 0;
pub const BG_JOB_PRIORITY_HIGH: BG_JOB_PRIORITY = 1;
pub const BG_JOB_PRIORITY_LOW: BG_JOB_PRIORITY = 3;
pub const BG_JOB_PRIORITY_NORMAL: BG_JOB_PRIORITY = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BG_JOB_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub FilesTotal: u32,
    pub FilesTransferred: u32,
}
pub type BG_JOB_PROXY_USAGE = i32;
pub const BG_JOB_PROXY_USAGE_AUTODETECT: BG_JOB_PROXY_USAGE = 3;
pub const BG_JOB_PROXY_USAGE_NO_PROXY: BG_JOB_PROXY_USAGE = 1;
pub const BG_JOB_PROXY_USAGE_OVERRIDE: BG_JOB_PROXY_USAGE = 2;
pub const BG_JOB_PROXY_USAGE_PRECONFIG: BG_JOB_PROXY_USAGE = 0;
pub type BG_JOB_STATE = i32;
pub const BG_JOB_STATE_ACKNOWLEDGED: BG_JOB_STATE = 7;
pub const BG_JOB_STATE_CANCELLED: BG_JOB_STATE = 8;
pub const BG_JOB_STATE_CONNECTING: BG_JOB_STATE = 1;
pub const BG_JOB_STATE_ERROR: BG_JOB_STATE = 4;
pub const BG_JOB_STATE_QUEUED: BG_JOB_STATE = 0;
pub const BG_JOB_STATE_SUSPENDED: BG_JOB_STATE = 3;
pub const BG_JOB_STATE_TRANSFERRED: BG_JOB_STATE = 6;
pub const BG_JOB_STATE_TRANSFERRING: BG_JOB_STATE = 2;
pub const BG_JOB_STATE_TRANSIENT_ERROR: BG_JOB_STATE = 5;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Default)]
pub struct BG_JOB_TIMES {
    pub CreationTime: super::minwindef::FILETIME,
    pub ModificationTime: super::minwindef::FILETIME,
    pub TransferCompletionTime: super::minwindef::FILETIME,
}
pub type BG_JOB_TYPE = i32;
pub const BG_JOB_TYPE_DOWNLOAD: BG_JOB_TYPE = 0;
pub const BG_JOB_TYPE_UPLOAD: BG_JOB_TYPE = 1;
pub const BG_JOB_TYPE_UPLOAD_REPLY: BG_JOB_TYPE = 2;
pub const BG_NOTIFY_DISABLE: u32 = 4;
pub const BG_NOTIFY_FILE_RANGES_TRANSFERRED: u32 = 32;
pub const BG_NOTIFY_FILE_TRANSFERRED: u32 = 16;
pub const BG_NOTIFY_JOB_ERROR: u32 = 2;
pub const BG_NOTIFY_JOB_MODIFICATION: u32 = 8;
pub const BG_NOTIFY_JOB_TRANSFERRED: u32 = 1;
pub const BG_SIZE_UNKNOWN: i32 = -1;
pub const BackgroundCopyManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4991d34b_80a1_4291_83b6_3328366b9097);
