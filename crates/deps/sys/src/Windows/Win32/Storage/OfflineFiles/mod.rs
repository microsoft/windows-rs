#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::marker::Copy for IEnumOfflineFilesItems {}
impl ::core::clone::Clone for IEnumOfflineFilesItems {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumOfflineFilesSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumOfflineFilesSettings {}
impl ::core::clone::Clone for IEnumOfflineFilesSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesCache(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesCache {}
impl ::core::clone::Clone for IOfflineFilesCache {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesCache2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesCache2 {}
impl ::core::clone::Clone for IOfflineFilesCache2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesChangeInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesChangeInfo {}
impl ::core::clone::Clone for IOfflineFilesChangeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesConnectionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesConnectionInfo {}
impl ::core::clone::Clone for IOfflineFilesConnectionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesDirectoryItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesDirectoryItem {}
impl ::core::clone::Clone for IOfflineFilesDirectoryItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesDirtyInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesDirtyInfo {}
impl ::core::clone::Clone for IOfflineFilesDirtyInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesErrorInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesErrorInfo {}
impl ::core::clone::Clone for IOfflineFilesErrorInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesEvents {}
impl ::core::clone::Clone for IOfflineFilesEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesEvents2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesEvents2 {}
impl ::core::clone::Clone for IOfflineFilesEvents2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesEvents3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesEvents3 {}
impl ::core::clone::Clone for IOfflineFilesEvents3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesEvents4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesEvents4 {}
impl ::core::clone::Clone for IOfflineFilesEvents4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesEventsFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesEventsFilter {}
impl ::core::clone::Clone for IOfflineFilesEventsFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesFileItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesFileItem {}
impl ::core::clone::Clone for IOfflineFilesFileItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesFileSysInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesFileSysInfo {}
impl ::core::clone::Clone for IOfflineFilesFileSysInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesGhostInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesGhostInfo {}
impl ::core::clone::Clone for IOfflineFilesGhostInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesItem {}
impl ::core::clone::Clone for IOfflineFilesItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesItemContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesItemContainer {}
impl ::core::clone::Clone for IOfflineFilesItemContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesItemFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesItemFilter {}
impl ::core::clone::Clone for IOfflineFilesItemFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesPinInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesPinInfo {}
impl ::core::clone::Clone for IOfflineFilesPinInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesPinInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesPinInfo2 {}
impl ::core::clone::Clone for IOfflineFilesPinInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesProgress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesProgress {}
impl ::core::clone::Clone for IOfflineFilesProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesServerItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesServerItem {}
impl ::core::clone::Clone for IOfflineFilesServerItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesSetting(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesSetting {}
impl ::core::clone::Clone for IOfflineFilesSetting {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesShareInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesShareInfo {}
impl ::core::clone::Clone for IOfflineFilesShareInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesShareItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesShareItem {}
impl ::core::clone::Clone for IOfflineFilesShareItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesSimpleProgress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesSimpleProgress {}
impl ::core::clone::Clone for IOfflineFilesSimpleProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesSuspend(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesSuspend {}
impl ::core::clone::Clone for IOfflineFilesSuspend {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesSuspendInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesSuspendInfo {}
impl ::core::clone::Clone for IOfflineFilesSuspendInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesSyncConflictHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesSyncConflictHandler {}
impl ::core::clone::Clone for IOfflineFilesSyncConflictHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesSyncErrorInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesSyncErrorInfo {}
impl ::core::clone::Clone for IOfflineFilesSyncErrorInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesSyncErrorItemInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesSyncErrorItemInfo {}
impl ::core::clone::Clone for IOfflineFilesSyncErrorItemInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesSyncProgress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesSyncProgress {}
impl ::core::clone::Clone for IOfflineFilesSyncProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineFilesTransparentCacheInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineFilesTransparentCacheInfo {}
impl ::core::clone::Clone for IOfflineFilesTransparentCacheInfo {
    fn clone(&self) -> Self {
        *self
    }
}
pub const OFFLINEFILES_CACHING_MODE_NONE: i32 = 0i32;
pub const OFFLINEFILES_CACHING_MODE_NOCACHING: i32 = 1i32;
pub const OFFLINEFILES_CACHING_MODE_MANUAL: i32 = 2i32;
pub const OFFLINEFILES_CACHING_MODE_AUTO_DOC: i32 = 3i32;
pub const OFFLINEFILES_CACHING_MODE_AUTO_PROGANDDOC: i32 = 4i32;
pub const OFFLINEFILES_CHANGES_LOCAL_ATTRIBUTES: u32 = 2u32;
pub const OFFLINEFILES_CHANGES_LOCAL_SIZE: u32 = 1u32;
pub const OFFLINEFILES_CHANGES_LOCAL_TIME: u32 = 4u32;
pub const OFFLINEFILES_CHANGES_NONE: u32 = 0u32;
pub const OFFLINEFILES_CHANGES_REMOTE_ATTRIBUTES: u32 = 16u32;
pub const OFFLINEFILES_CHANGES_REMOTE_SIZE: u32 = 8u32;
pub const OFFLINEFILES_CHANGES_REMOTE_TIME: u32 = 32u32;
pub const OFFLINEFILES_COMPARE_EQ: i32 = 0i32;
pub const OFFLINEFILES_COMPARE_NEQ: i32 = 1i32;
pub const OFFLINEFILES_COMPARE_LT: i32 = 2i32;
pub const OFFLINEFILES_COMPARE_GT: i32 = 3i32;
pub const OFFLINEFILES_COMPARE_LTE: i32 = 4i32;
pub const OFFLINEFILES_COMPARE_GTE: i32 = 5i32;
pub const OFFLINEFILES_CONNECT_STATE_UNKNOWN: i32 = 0i32;
pub const OFFLINEFILES_CONNECT_STATE_OFFLINE: i32 = 1i32;
pub const OFFLINEFILES_CONNECT_STATE_ONLINE: i32 = 2i32;
pub const OFFLINEFILES_CONNECT_STATE_TRANSPARENTLY_CACHED: i32 = 3i32;
pub const OFFLINEFILES_CONNECT_STATE_PARTLY_TRANSPARENTLY_CACHED: i32 = 4i32;
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
pub const OFFLINEFILES_EVENT_CACHEMOVED: i32 = 0i32;
pub const OFFLINEFILES_EVENT_CACHEISFULL: i32 = 1i32;
pub const OFFLINEFILES_EVENT_CACHEISCORRUPTED: i32 = 2i32;
pub const OFFLINEFILES_EVENT_ENABLED: i32 = 3i32;
pub const OFFLINEFILES_EVENT_ENCRYPTIONCHANGED: i32 = 4i32;
pub const OFFLINEFILES_EVENT_SYNCBEGIN: i32 = 5i32;
pub const OFFLINEFILES_EVENT_SYNCFILERESULT: i32 = 6i32;
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECADDED: i32 = 7i32;
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECUPDATED: i32 = 8i32;
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECREMOVED: i32 = 9i32;
pub const OFFLINEFILES_EVENT_SYNCEND: i32 = 10i32;
pub const OFFLINEFILES_EVENT_BACKGROUNDSYNCBEGIN: i32 = 11i32;
pub const OFFLINEFILES_EVENT_BACKGROUNDSYNCEND: i32 = 12i32;
pub const OFFLINEFILES_EVENT_NETTRANSPORTARRIVED: i32 = 13i32;
pub const OFFLINEFILES_EVENT_NONETTRANSPORTS: i32 = 14i32;
pub const OFFLINEFILES_EVENT_ITEMDISCONNECTED: i32 = 15i32;
pub const OFFLINEFILES_EVENT_ITEMRECONNECTED: i32 = 16i32;
pub const OFFLINEFILES_EVENT_ITEMAVAILABLEOFFLINE: i32 = 17i32;
pub const OFFLINEFILES_EVENT_ITEMNOTAVAILABLEOFFLINE: i32 = 18i32;
pub const OFFLINEFILES_EVENT_ITEMPINNED: i32 = 19i32;
pub const OFFLINEFILES_EVENT_ITEMNOTPINNED: i32 = 20i32;
pub const OFFLINEFILES_EVENT_ITEMMODIFIED: i32 = 21i32;
pub const OFFLINEFILES_EVENT_ITEMADDEDTOCACHE: i32 = 22i32;
pub const OFFLINEFILES_EVENT_ITEMDELETEDFROMCACHE: i32 = 23i32;
pub const OFFLINEFILES_EVENT_ITEMRENAMED: i32 = 24i32;
pub const OFFLINEFILES_EVENT_DATALOST: i32 = 25i32;
pub const OFFLINEFILES_EVENT_PING: i32 = 26i32;
pub const OFFLINEFILES_EVENT_ITEMRECONNECTBEGIN: i32 = 27i32;
pub const OFFLINEFILES_EVENT_ITEMRECONNECTEND: i32 = 28i32;
pub const OFFLINEFILES_EVENT_CACHEEVICTBEGIN: i32 = 29i32;
pub const OFFLINEFILES_EVENT_CACHEEVICTEND: i32 = 30i32;
pub const OFFLINEFILES_EVENT_POLICYCHANGEDETECTED: i32 = 31i32;
pub const OFFLINEFILES_EVENT_PREFERENCECHANGEDETECTED: i32 = 32i32;
pub const OFFLINEFILES_EVENT_SETTINGSCHANGESAPPLIED: i32 = 33i32;
pub const OFFLINEFILES_EVENT_TRANSPARENTCACHEITEMNOTIFY: i32 = 34i32;
pub const OFFLINEFILES_EVENT_PREFETCHFILEBEGIN: i32 = 35i32;
pub const OFFLINEFILES_EVENT_PREFETCHFILEEND: i32 = 36i32;
pub const OFFLINEFILES_EVENT_PREFETCHCLOSEHANDLEBEGIN: i32 = 37i32;
pub const OFFLINEFILES_EVENT_PREFETCHCLOSEHANDLEEND: i32 = 38i32;
pub const OFFLINEFILES_NUM_EVENTS: i32 = 39i32;
pub const OFFLINEFILES_ITEM_COPY_LOCAL: i32 = 0i32;
pub const OFFLINEFILES_ITEM_COPY_REMOTE: i32 = 1i32;
pub const OFFLINEFILES_ITEM_COPY_ORIGINAL: i32 = 2i32;
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
pub const OFFLINEFILES_ITEM_TIME_CREATION: i32 = 0i32;
pub const OFFLINEFILES_ITEM_TIME_LASTACCESS: i32 = 1i32;
pub const OFFLINEFILES_ITEM_TIME_LASTWRITE: i32 = 2i32;
pub const OFFLINEFILES_ITEM_TYPE_FILE: i32 = 0i32;
pub const OFFLINEFILES_ITEM_TYPE_DIRECTORY: i32 = 1i32;
pub const OFFLINEFILES_ITEM_TYPE_SHARE: i32 = 2i32;
pub const OFFLINEFILES_ITEM_TYPE_SERVER: i32 = 3i32;
pub const OFFLINEFILES_OFFLINE_REASON_UNKNOWN: i32 = 0i32;
pub const OFFLINEFILES_OFFLINE_REASON_NOT_APPLICABLE: i32 = 1i32;
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_FORCED: i32 = 2i32;
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_SLOW: i32 = 3i32;
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_ERROR: i32 = 4i32;
pub const OFFLINEFILES_OFFLINE_REASON_ITEM_VERSION_CONFLICT: i32 = 5i32;
pub const OFFLINEFILES_OFFLINE_REASON_ITEM_SUSPENDED: i32 = 6i32;
pub const OFFLINEFILES_OP_CONTINUE: i32 = 0i32;
pub const OFFLINEFILES_OP_RETRY: i32 = 1i32;
pub const OFFLINEFILES_OP_ABORT: i32 = 2i32;
pub const OFFLINEFILES_PATHFILTER_SELF: i32 = 0i32;
pub const OFFLINEFILES_PATHFILTER_CHILD: i32 = 1i32;
pub const OFFLINEFILES_PATHFILTER_DESCENDENT: i32 = 2i32;
pub const OFFLINEFILES_PATHFILTER_SELFORCHILD: i32 = 3i32;
pub const OFFLINEFILES_PATHFILTER_SELFORDESCENDENT: i32 = 4i32;
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
pub const OFFLINEFILES_SETTING_VALUE_UI4: i32 = 0i32;
pub const OFFLINEFILES_SETTING_VALUE_BSTR: i32 = 1i32;
pub const OFFLINEFILES_SETTING_VALUE_BSTR_DBLNULTERM: i32 = 2i32;
pub const OFFLINEFILES_SETTING_VALUE_2DIM_ARRAY_BSTR_UI4: i32 = 3i32;
pub const OFFLINEFILES_SETTING_VALUE_2DIM_ARRAY_BSTR_BSTR: i32 = 4i32;
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_NONE: i32 = 0i32;
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPLOCAL: i32 = 1i32;
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPREMOTE: i32 = 2i32;
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPALLCHANGES: i32 = 3i32;
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPLATEST: i32 = 4i32;
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_LOG: i32 = 5i32;
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_SKIP: i32 = 6i32;
pub const OFFLINEFILES_SYNC_CONFLICT_ABORT: i32 = 7i32;
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_NUMCODES: i32 = 8i32;
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
pub const OFFLINEFILES_SYNC_OPERATION_CREATE_COPY_ON_SERVER: i32 = 0i32;
pub const OFFLINEFILES_SYNC_OPERATION_CREATE_COPY_ON_CLIENT: i32 = 1i32;
pub const OFFLINEFILES_SYNC_OPERATION_SYNC_TO_SERVER: i32 = 2i32;
pub const OFFLINEFILES_SYNC_OPERATION_SYNC_TO_CLIENT: i32 = 3i32;
pub const OFFLINEFILES_SYNC_OPERATION_DELETE_SERVER_COPY: i32 = 4i32;
pub const OFFLINEFILES_SYNC_OPERATION_DELETE_CLIENT_COPY: i32 = 5i32;
pub const OFFLINEFILES_SYNC_OPERATION_PIN: i32 = 6i32;
pub const OFFLINEFILES_SYNC_OPERATION_PREPARE: i32 = 7i32;
pub const OFFLINEFILES_SYNC_STATE_Stable: i32 = 0i32;
pub const OFFLINEFILES_SYNC_STATE_FileOnClient_DirOnServer: i32 = 1i32;
pub const OFFLINEFILES_SYNC_STATE_FileOnClient_NoServerCopy: i32 = 2i32;
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_FileOnServer: i32 = 3i32;
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_FileChangedOnServer: i32 = 4i32;
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_NoServerCopy: i32 = 5i32;
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_NoServerCopy: i32 = 6i32;
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_FileChangedOnServer: i32 = 7i32;
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DirChangedOnServer: i32 = 8i32;
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_FileOnServer: i32 = 9i32;
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DirOnServer: i32 = 10i32;
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DeletedOnServer: i32 = 11i32;
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_ChangedOnServer: i32 = 12i32;
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DirOnServer: i32 = 13i32;
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DirChangedOnServer: i32 = 14i32;
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DeletedOnServer: i32 = 15i32;
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_ChangedOnServer: i32 = 16i32;
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DeletedOnServer: i32 = 17i32;
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DirOnServer: i32 = 18i32;
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DirChangedOnServer: i32 = 19i32;
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_NoServerCopy: i32 = 20i32;
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DirOnServer: i32 = 21i32;
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_FileOnServer: i32 = 22i32;
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_FileChangedOnServer: i32 = 23i32;
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DirChangedOnServer: i32 = 24i32;
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DeletedOnServer: i32 = 25i32;
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_FileOnServer: i32 = 26i32;
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_FileChangedOnServer: i32 = 27i32;
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_ChangedOnServer: i32 = 28i32;
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_DeletedOnServer: i32 = 29i32;
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_FileOnServer: i32 = 30i32;
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_DirOnServer: i32 = 31i32;
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_FileChangedOnServer: i32 = 32i32;
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_DirChangedOnServer: i32 = 33i32;
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_FileOnServer: i32 = 34i32;
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_DirOnServer: i32 = 35i32;
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_FileChangedOnServer: i32 = 36i32;
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_DirChangedOnServer: i32 = 37i32;
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient: i32 = 38i32;
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient: i32 = 39i32;
pub const OFFLINEFILES_SYNC_STATE_FileRenamedOnClient: i32 = 40i32;
pub const OFFLINEFILES_SYNC_STATE_DirSparseOnClient: i32 = 41i32;
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient: i32 = 42i32;
pub const OFFLINEFILES_SYNC_STATE_DirRenamedOnClient: i32 = 43i32;
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnServer: i32 = 44i32;
pub const OFFLINEFILES_SYNC_STATE_FileRenamedOnServer: i32 = 45i32;
pub const OFFLINEFILES_SYNC_STATE_FileDeletedOnServer: i32 = 46i32;
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnServer: i32 = 47i32;
pub const OFFLINEFILES_SYNC_STATE_DirRenamedOnServer: i32 = 48i32;
pub const OFFLINEFILES_SYNC_STATE_DirDeletedOnServer: i32 = 49i32;
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_FileOnServer: i32 = 50i32;
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_FileChangedOnServer: i32 = 51i32;
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_DirOnServer: i32 = 52i32;
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_DirChangedOnServer: i32 = 53i32;
pub const OFFLINEFILES_SYNC_STATE_NUMSTATES: i32 = 54i32;
pub const OFFLINEFILES_SYNC_STATE_LOCAL_KNOWN: u32 = 1u32;
pub const OFFLINEFILES_SYNC_STATE_REMOTE_KNOWN: u32 = 2u32;
pub const OFFLINEFILES_TRANSITION_FLAG_CONSOLE: u32 = 2u32;
pub const OFFLINEFILES_TRANSITION_FLAG_INTERACTIVE: u32 = 1u32;
pub const OfflineFilesCache: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1220984444,
    data2: 14449,
    data3: 17356,
    data4: [180, 111, 20, 73, 161, 187, 47, 243],
};
pub const OfflineFilesSetting: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4248197609,
    data2: 43296,
    data3: 16675,
    data4: [173, 100, 127, 199, 108, 122, 172, 223],
};
