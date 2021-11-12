#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn OfflineFilesEnable(benable: super::super::Foundation::BOOL, pbrebootrequired: *mut super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OfflineFilesQueryStatus(pbactive: *mut super::super::Foundation::BOOL, pbenabled: *mut super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OfflineFilesQueryStatusEx(pbactive: *mut super::super::Foundation::BOOL, pbenabled: *mut super::super::Foundation::BOOL, pbavailable: *mut super::super::Foundation::BOOL) -> u32;
    pub fn OfflineFilesStart() -> u32;
}
#[repr(transparent)]
pub struct IEnumOfflineFilesItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumOfflineFilesSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesCache(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesCache2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesChangeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesConnectionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesDirectoryItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesDirtyInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesErrorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesEvents2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesEvents3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesEvents4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesEventsFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesFileItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesFileSysInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesGhostInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesItemContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesItemFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesPinInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesPinInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesProgress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesServerItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesSetting(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesShareInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesShareItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesSimpleProgress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesSuspend(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesSuspendInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesSyncConflictHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesSyncErrorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesSyncErrorItemInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesSyncProgress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineFilesTransparentCacheInfo(pub *mut ::core::ffi::c_void);
pub struct OFFLINEFILES_CACHING_MODE(i32);
pub const OFFLINEFILES_CHANGES_LOCAL_ATTRIBUTES: u32 = 2u32;
pub const OFFLINEFILES_CHANGES_LOCAL_SIZE: u32 = 1u32;
pub const OFFLINEFILES_CHANGES_LOCAL_TIME: u32 = 4u32;
pub const OFFLINEFILES_CHANGES_NONE: u32 = 0u32;
pub const OFFLINEFILES_CHANGES_REMOTE_ATTRIBUTES: u32 = 16u32;
pub const OFFLINEFILES_CHANGES_REMOTE_SIZE: u32 = 8u32;
pub const OFFLINEFILES_CHANGES_REMOTE_TIME: u32 = 32u32;
pub struct OFFLINEFILES_COMPARE(i32);
pub struct OFFLINEFILES_CONNECT_STATE(i32);
pub const OFFLINEFILES_DELETE_FLAG_ADMIN: u32 = 2147483648u32;
pub const OFFLINEFILES_DELETE_FLAG_DELMODIFIED: u32 = 4u32;
pub const OFFLINEFILES_DELETE_FLAG_NOAUTOCACHED: u32 = 1u32;
pub const OFFLINEFILES_DELETE_FLAG_NOPINNED: u32 = 2u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
pub const OFFLINEFILES_ENUM_FLAT: u32 = 1u32;
pub const OFFLINEFILES_ENUM_FLAT_FILESONLY: u32 = 2u32;
pub struct OFFLINEFILES_EVENTS(i32);
pub struct OFFLINEFILES_ITEM_COPY(i32);
pub const OFFLINEFILES_ITEM_FILTER_FLAG_CREATED: u32 = 8u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DELETED: u32 = 16u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DIRECTORY: u32 = 256u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DIRTY: u32 = 32u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_FILE: u32 = 128u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GHOST: u32 = 8192u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_ANYACCESS: u32 = 33554432u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_READ: u32 = 16777216u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_WRITE: u32 = 8388608u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED: u32 = 4u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED_ATTRIBUTES: u32 = 2u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED_DATA: u32 = 1u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OFFLINE: u32 = 32768u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_ONLINE: u32 = 65536u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_ANYACCESS: u32 = 4194304u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_READ: u32 = 2097152u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_WRITE: u32 = 1048576u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED: u32 = 4096u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_COMPUTER: u32 = 2048u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_OTHERS: u32 = 1024u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_USER: u32 = 512u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_SPARSE: u32 = 64u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_SUSPENDED: u32 = 16384u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_ANYACCESS: u32 = 524288u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_READ: u32 = 262144u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_WRITE: u32 = 131072u32;
pub const OFFLINEFILES_ITEM_QUERY_ADMIN: u32 = 2147483648u32;
pub const OFFLINEFILES_ITEM_QUERY_ATTEMPT_TRANSITIONONLINE: u32 = 32u32;
pub const OFFLINEFILES_ITEM_QUERY_CONNECTIONSTATE: u32 = 2u32;
pub const OFFLINEFILES_ITEM_QUERY_INCLUDETRANSPARENTCACHE: u32 = 16u32;
pub const OFFLINEFILES_ITEM_QUERY_LOCALDIRTYBYTECOUNT: u32 = 4u32;
pub const OFFLINEFILES_ITEM_QUERY_REMOTEDIRTYBYTECOUNT: u32 = 8u32;
pub const OFFLINEFILES_ITEM_QUERY_REMOTEINFO: u32 = 1u32;
pub struct OFFLINEFILES_ITEM_TIME(i32);
pub struct OFFLINEFILES_ITEM_TYPE(i32);
pub struct OFFLINEFILES_OFFLINE_REASON(i32);
pub struct OFFLINEFILES_OP_RESPONSE(i32);
pub struct OFFLINEFILES_PATHFILTER_MATCH(i32);
pub const OFFLINEFILES_PINLINKTARGETS_ALWAYS: u32 = 2u32;
pub const OFFLINEFILES_PINLINKTARGETS_EXPLICIT: u32 = 1u32;
pub const OFFLINEFILES_PINLINKTARGETS_NEVER: u32 = 0u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FILL: u32 = 1u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORALL: u32 = 128u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORREDIR: u32 = 256u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORUSER: u32 = 32u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORUSER_POLICY: u32 = 64u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_PINLINKTARGETS: u32 = 16u32;
pub const OFFLINEFILES_SETTING_SCOPE_COMPUTER: u32 = 2u32;
pub const OFFLINEFILES_SETTING_SCOPE_USER: u32 = 1u32;
pub struct OFFLINEFILES_SETTING_VALUE_TYPE(i32);
pub struct OFFLINEFILES_SYNC_CONFLICT_RESOLVE(i32);
pub const OFFLINEFILES_SYNC_CONTROL_CR_DEFAULT: u32 = 0u32;
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPLATEST: u32 = 805306368u32;
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPLOCAL: u32 = 268435456u32;
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPREMOTE: u32 = 536870912u32;
pub const OFFLINEFILES_SYNC_CONTROL_CR_MASK: u32 = 4026531840u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_FILLSPARSE: u32 = 1u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_NONEWFILESOUT: u32 = 131072u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORALL: u32 = 128u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORREDIR: u32 = 256u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORUSER: u32 = 32u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORUSER_POLICY: u32 = 64u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINLINKTARGETS: u32 = 16u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINNEWFILES: u32 = 8u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SKIPSUSPENDEDDIRS: u32 = 8192u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SYNCIN: u32 = 2u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SYNCOUT: u32 = 4u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_ATTRIBUTES: u32 = 8u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_CHANGETIME: u32 = 1u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_FILESIZE: u32 = 4u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_NONE: u32 = 0u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_WRITETIME: u32 = 2u32;
pub struct OFFLINEFILES_SYNC_OPERATION(i32);
pub struct OFFLINEFILES_SYNC_STATE(i32);
pub const OFFLINEFILES_SYNC_STATE_LOCAL_KNOWN: u32 = 1u32;
pub const OFFLINEFILES_SYNC_STATE_REMOTE_KNOWN: u32 = 2u32;
pub const OFFLINEFILES_TRANSITION_FLAG_CONSOLE: u32 = 2u32;
pub const OFFLINEFILES_TRANSITION_FLAG_INTERACTIVE: u32 = 1u32;
pub struct OfflineFilesCache(i32);
pub struct OfflineFilesSetting(i32);
