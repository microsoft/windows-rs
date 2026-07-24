pub const CRM_PROTOCOL_DYNAMIC_MARSHAL_INFO: i32 = 2;
pub const CRM_PROTOCOL_EXPLICIT_MARSHAL_ONLY: i32 = 1;
pub type CRM_PROTOCOL_ID = windows_core::GUID;
pub const CRM_PROTOCOL_MAXIMUM_OPTION: i32 = 3;
pub const ENLISTMENT_MAXIMUM_OPTION: i32 = 1;
pub const ENLISTMENT_OBJECT_PATH: windows_core::PCWSTR = windows_core::w!("\\Enlistment\\");
pub const ENLISTMENT_SUPERIOR: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KCRM_MARSHAL_HEADER {
    pub VersionMajor: u32,
    pub VersionMinor: u32,
    pub NumProtocols: u32,
    pub Unused: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KCRM_PROTOCOL_BLOB {
    pub ProtocolId: CRM_PROTOCOL_ID,
    pub StaticInfoLength: u32,
    pub TransactionIdInfoLength: u32,
    pub Unused1: u32,
    pub Unused2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KCRM_TRANSACTION_BLOB {
    pub UOW: UOW,
    pub TmIdentity: windows_core::GUID,
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
pub const KTM_MARSHAL_BLOB_VERSION_MAJOR: i32 = 1;
pub const KTM_MARSHAL_BLOB_VERSION_MINOR: i32 = 1;
pub const MAX_RESOURCEMANAGER_DESCRIPTION_LENGTH: i32 = 64;
pub const MAX_TRANSACTION_DESCRIPTION_LENGTH: i32 = 64;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NOTIFICATION_MASK(pub u32);
pub type PCRM_PROTOCOL_ID = *mut windows_core::GUID;
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
pub type PUOW = *mut windows_core::GUID;
pub const RESOURCE_MANAGER_COMMUNICATION: i32 = 2;
pub const RESOURCE_MANAGER_MAXIMUM_OPTION: i32 = 3;
pub const RESOURCE_MANAGER_OBJECT_PATH: windows_core::PCWSTR = windows_core::w!("\\ResourceManager\\");
pub const RESOURCE_MANAGER_VOLATILE: i32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SAVEPOINT_ID(pub u32);
pub const TRANSACTIONMANAGER_OBJECT_PATH: windows_core::PCWSTR = windows_core::w!("\\TransactionManager\\");
pub const TRANSACTION_DO_NOT_PROMOTE: i32 = 1;
pub const TRANSACTION_MANAGER_COMMIT_DEFAULT: i32 = 0;
pub const TRANSACTION_MANAGER_COMMIT_LOWEST: i32 = 8;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_HIVES: i32 = 4;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_VOLUME: i32 = 2;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_PROGRESS: i32 = 32;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_RECOVERY: i32 = 16;
pub const TRANSACTION_MANAGER_MAXIMUM_OPTION: i32 = 63;
pub const TRANSACTION_MANAGER_VOLATILE: i32 = 1;
pub const TRANSACTION_MAXIMUM_OPTION: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    pub MarshalCookie: u32,
    pub UOW: windows_core::GUID,
}
pub type TRANSACTION_NOTIFICATION_PROMOTE_ARGUMENT = TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    pub PropagationCookie: u32,
    pub UOW: windows_core::GUID,
    pub TmIdentity: windows_core::GUID,
    pub BufferLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    pub EnlistmentId: windows_core::GUID,
    pub UOW: UOW,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    pub SavepointId: SAVEPOINT_ID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    pub TmIdentity: windows_core::GUID,
    pub Flags: u32,
}
pub const TRANSACTION_NOTIFICATION_TM_ONLINE_FLAG_IS_CLUSTERED: i32 = 1;
pub const TRANSACTION_NOTIFY_COMMIT: i32 = 4;
pub const TRANSACTION_NOTIFY_COMMIT_COMPLETE: i32 = 64;
pub const TRANSACTION_NOTIFY_COMMIT_FINALIZE: i32 = 1073741824;
pub const TRANSACTION_NOTIFY_COMMIT_REQUEST: i32 = 67108864;
pub const TRANSACTION_NOTIFY_DELEGATE_COMMIT: i32 = 1024;
pub const TRANSACTION_NOTIFY_ENLIST_MASK: i32 = 262144;
pub const TRANSACTION_NOTIFY_ENLIST_PREPREPARE: i32 = 4096;
pub const TRANSACTION_NOTIFY_INDOUBT: i32 = 16384;
pub const TRANSACTION_NOTIFY_LAST_RECOVER: i32 = 8192;
pub const TRANSACTION_NOTIFY_MARSHAL: i32 = 131072;
pub const TRANSACTION_NOTIFY_MASK: i32 = 1073741823;
pub const TRANSACTION_NOTIFY_PREPARE: i32 = 2;
pub const TRANSACTION_NOTIFY_PREPARE_COMPLETE: i32 = 32;
pub const TRANSACTION_NOTIFY_PREPREPARE: i32 = 1;
pub const TRANSACTION_NOTIFY_PREPREPARE_COMPLETE: i32 = 16;
pub const TRANSACTION_NOTIFY_PROMOTE: i32 = 134217728;
pub const TRANSACTION_NOTIFY_PROMOTE_NEW: i32 = 268435456;
pub const TRANSACTION_NOTIFY_PROPAGATE_PULL: i32 = 32768;
pub const TRANSACTION_NOTIFY_PROPAGATE_PUSH: i32 = 65536;
pub const TRANSACTION_NOTIFY_RECOVER: i32 = 256;
pub const TRANSACTION_NOTIFY_RECOVER_QUERY: i32 = 2048;
pub const TRANSACTION_NOTIFY_REQUEST_OUTCOME: i32 = 536870912;
pub const TRANSACTION_NOTIFY_RM_DISCONNECTED: i32 = 16777216;
pub const TRANSACTION_NOTIFY_ROLLBACK: i32 = 8;
pub const TRANSACTION_NOTIFY_ROLLBACK_COMPLETE: i32 = 128;
pub const TRANSACTION_NOTIFY_SINGLE_PHASE_COMMIT: i32 = 512;
pub const TRANSACTION_NOTIFY_TM_ONLINE: i32 = 33554432;
pub const TRANSACTION_OBJECT_PATH: windows_core::PCWSTR = windows_core::w!("\\Transaction\\");
pub type UOW = windows_core::GUID;
