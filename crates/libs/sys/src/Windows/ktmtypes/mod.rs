pub const CRM_PROTOCOL_DYNAMIC_MARSHAL_INFO: u32 = 2;
pub const CRM_PROTOCOL_EXPLICIT_MARSHAL_ONLY: u32 = 1;
pub type CRM_PROTOCOL_ID = windows_sys::core::GUID;
pub const CRM_PROTOCOL_MAXIMUM_OPTION: u32 = 3;
pub const ENLISTMENT_MAXIMUM_OPTION: u32 = 1;
pub const ENLISTMENT_OBJECT_PATH: windows_sys::core::PCWSTR = windows_sys::core::w!("\\Enlistment\\");
pub const ENLISTMENT_SUPERIOR: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KCRM_MARSHAL_HEADER {
    pub VersionMajor: u32,
    pub VersionMinor: u32,
    pub NumProtocols: u32,
    pub Unused: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KCRM_PROTOCOL_BLOB {
    pub ProtocolId: CRM_PROTOCOL_ID,
    pub StaticInfoLength: u32,
    pub TransactionIdInfoLength: u32,
    pub Unused1: u32,
    pub Unused2: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KCRM_TRANSACTION_BLOB {
    pub UOW: UOW,
    pub TmIdentity: windows_sys::core::GUID,
    pub IsolationLevel: u32,
    pub IsolationFlags: u32,
    pub Timeout: u32,
    pub Description: [u16; 64],
}
impl Default for KCRM_TRANSACTION_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KTM_MARSHAL_BLOB_VERSION_MAJOR: u32 = 1;
pub const KTM_MARSHAL_BLOB_VERSION_MINOR: u32 = 1;
pub const MAX_RESOURCEMANAGER_DESCRIPTION_LENGTH: u32 = 64;
pub const MAX_TRANSACTION_DESCRIPTION_LENGTH: u32 = 64;
pub type NOTIFICATION_MASK = u32;
pub type PCRM_PROTOCOL_ID = *mut windows_sys::core::GUID;
pub type PKCRM_MARSHAL_HEADER = *mut KCRM_MARSHAL_HEADER;
pub type PKCRM_PROTOCOL_BLOB = *mut KCRM_PROTOCOL_BLOB;
pub type PKCRM_TRANSACTION_BLOB = *mut KCRM_TRANSACTION_BLOB;
pub type PRKCRM_MARSHAL_HEADER = *mut KCRM_MARSHAL_HEADER;
pub type PRKCRM_PROTOCOL_BLOB = *mut KCRM_PROTOCOL_BLOB;
pub type PRKCRM_TRANSACTION_BLOB = *mut KCRM_TRANSACTION_BLOB;
pub type PSAVEPOINT_ID = *mut u32;
pub type PTRANSACTION_NOTIFICATION = *mut TRANSACTION_NOTIFICATION;
pub type PTRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT = *mut TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT;
pub type PTRANSACTION_NOTIFICATION_PROMOTE_ARGUMENT = *mut TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT;
pub type PTRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT = *mut TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT;
pub type PTRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT = *mut TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT;
pub type PTRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT = *mut TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT;
pub type PTRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT = *mut TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT;
pub type PUOW = *mut windows_sys::core::GUID;
pub const RESOURCE_MANAGER_COMMUNICATION: u32 = 2;
pub const RESOURCE_MANAGER_MAXIMUM_OPTION: u32 = 3;
pub const RESOURCE_MANAGER_OBJECT_PATH: windows_sys::core::PCWSTR = windows_sys::core::w!("\\ResourceManager\\");
pub const RESOURCE_MANAGER_VOLATILE: u32 = 1;
pub type SAVEPOINT_ID = u32;
pub const TRANSACTIONMANAGER_OBJECT_PATH: windows_sys::core::PCWSTR = windows_sys::core::w!("\\TransactionManager\\");
pub const TRANSACTION_DO_NOT_PROMOTE: u32 = 1;
pub const TRANSACTION_MANAGER_COMMIT_DEFAULT: u32 = 0;
pub const TRANSACTION_MANAGER_COMMIT_LOWEST: u32 = 8;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_HIVES: u32 = 4;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_VOLUME: u32 = 2;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_PROGRESS: u32 = 32;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_RECOVERY: u32 = 16;
pub const TRANSACTION_MANAGER_MAXIMUM_OPTION: u32 = 63;
pub const TRANSACTION_MANAGER_VOLATILE: u32 = 1;
pub const TRANSACTION_MAXIMUM_OPTION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TRANSACTION_NOTIFICATION {
    pub TransactionKey: *mut core::ffi::c_void,
    pub TransactionNotification: u32,
    pub TmVirtualClock: i64,
    pub ArgumentLength: u32,
}
impl Default for TRANSACTION_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    pub MarshalCookie: u32,
    pub UOW: windows_sys::core::GUID,
}
pub type TRANSACTION_NOTIFICATION_PROMOTE_ARGUMENT = TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    pub PropagationCookie: u32,
    pub UOW: windows_sys::core::GUID,
    pub TmIdentity: windows_sys::core::GUID,
    pub BufferLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    pub EnlistmentId: windows_sys::core::GUID,
    pub UOW: UOW,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    pub SavepointId: SAVEPOINT_ID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    pub TmIdentity: windows_sys::core::GUID,
    pub Flags: u32,
}
pub const TRANSACTION_NOTIFICATION_TM_ONLINE_FLAG_IS_CLUSTERED: u32 = 1;
pub const TRANSACTION_NOTIFY_COMMIT: u32 = 4;
pub const TRANSACTION_NOTIFY_COMMIT_COMPLETE: u32 = 64;
pub const TRANSACTION_NOTIFY_COMMIT_FINALIZE: u32 = 1073741824;
pub const TRANSACTION_NOTIFY_COMMIT_REQUEST: u32 = 67108864;
pub const TRANSACTION_NOTIFY_DELEGATE_COMMIT: u32 = 1024;
pub const TRANSACTION_NOTIFY_ENLIST_MASK: u32 = 262144;
pub const TRANSACTION_NOTIFY_ENLIST_PREPREPARE: u32 = 4096;
pub const TRANSACTION_NOTIFY_INDOUBT: u32 = 16384;
pub const TRANSACTION_NOTIFY_LAST_RECOVER: u32 = 8192;
pub const TRANSACTION_NOTIFY_MARSHAL: u32 = 131072;
pub const TRANSACTION_NOTIFY_MASK: u32 = 1073741823;
pub const TRANSACTION_NOTIFY_PREPARE: u32 = 2;
pub const TRANSACTION_NOTIFY_PREPARE_COMPLETE: u32 = 32;
pub const TRANSACTION_NOTIFY_PREPREPARE: u32 = 1;
pub const TRANSACTION_NOTIFY_PREPREPARE_COMPLETE: u32 = 16;
pub const TRANSACTION_NOTIFY_PROMOTE: u32 = 134217728;
pub const TRANSACTION_NOTIFY_PROMOTE_NEW: u32 = 268435456;
pub const TRANSACTION_NOTIFY_PROPAGATE_PULL: u32 = 32768;
pub const TRANSACTION_NOTIFY_PROPAGATE_PUSH: u32 = 65536;
pub const TRANSACTION_NOTIFY_RECOVER: u32 = 256;
pub const TRANSACTION_NOTIFY_RECOVER_QUERY: u32 = 2048;
pub const TRANSACTION_NOTIFY_REQUEST_OUTCOME: u32 = 536870912;
pub const TRANSACTION_NOTIFY_RM_DISCONNECTED: u32 = 16777216;
pub const TRANSACTION_NOTIFY_ROLLBACK: u32 = 8;
pub const TRANSACTION_NOTIFY_ROLLBACK_COMPLETE: u32 = 128;
pub const TRANSACTION_NOTIFY_SINGLE_PHASE_COMMIT: u32 = 512;
pub const TRANSACTION_NOTIFY_TM_ONLINE: u32 = 33554432;
pub const TRANSACTION_OBJECT_PATH: windows_sys::core::PCWSTR = windows_sys::core::w!("\\Transaction\\");
pub type UOW = windows_sys::core::GUID;
