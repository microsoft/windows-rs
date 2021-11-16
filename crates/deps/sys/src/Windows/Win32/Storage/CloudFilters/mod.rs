#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfCloseHandle(filehandle: super::super::Foundation::HANDLE);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
    pub fn CfConnectSyncRoot(syncrootpath: super::super::Foundation::PWSTR, callbacktable: *const CF_CALLBACK_REGISTRATION, callbackcontext: *const ::core::ffi::c_void, connectflags: CF_CONNECT_FLAGS, connectionkey: *mut CF_CONNECTION_KEY) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfConvertToPlaceholder(filehandle: super::super::Foundation::HANDLE, fileidentity: *const ::core::ffi::c_void, fileidentitylength: u32, convertflags: CF_CONVERT_FLAGS, convertusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
    pub fn CfCreatePlaceholders(basedirectorypath: super::super::Foundation::PWSTR, placeholderarray: *mut CF_PLACEHOLDER_CREATE_INFO, placeholdercount: u32, createflags: CF_CREATE_FLAGS, entriesprocessed: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfDehydratePlaceholder(filehandle: super::super::Foundation::HANDLE, startingoffset: i64, length: i64, dehydrateflags: CF_DEHYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT;
    pub fn CfDisconnectSyncRoot(connectionkey: CF_CONNECTION_KEY) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_CorrelationVector"))]
    pub fn CfExecute(opinfo: *const CF_OPERATION_INFO, opparams: *mut CF_OPERATION_PARAMETERS) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
    pub fn CfGetCorrelationVector(filehandle: super::super::Foundation::HANDLE, correlationvector: *mut super::super::System::CorrelationVector::CORRELATION_VECTOR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetPlaceholderInfo(filehandle: super::super::Foundation::HANDLE, infoclass: CF_PLACEHOLDER_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetPlaceholderRangeInfo(filehandle: super::super::Foundation::HANDLE, infoclass: CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset: i64, length: i64, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn CfGetPlaceholderStateFromAttributeTag(fileattributes: u32, reparsetag: u32) -> CF_PLACEHOLDER_STATE;
    #[cfg(feature = "Win32_Storage_FileSystem")]
    pub fn CfGetPlaceholderStateFromFileInfo(infobuffer: *const ::core::ffi::c_void, infoclass: super::FileSystem::FILE_INFO_BY_HANDLE_CLASS) -> CF_PLACEHOLDER_STATE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
    pub fn CfGetPlaceholderStateFromFindData(finddata: *const super::FileSystem::WIN32_FIND_DATAA) -> CF_PLACEHOLDER_STATE;
    pub fn CfGetPlatformInfo(platformversion: *mut CF_PLATFORM_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetSyncRootInfoByHandle(filehandle: super::super::Foundation::HANDLE, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetSyncRootInfoByPath(filepath: super::super::Foundation::PWSTR, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetTransferKey(filehandle: super::super::Foundation::HANDLE, transferkey: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetWin32HandleFromProtectedHandle(protectedhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfHydratePlaceholder(filehandle: super::super::Foundation::HANDLE, startingoffset: i64, length: i64, hydrateflags: CF_HYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfOpenFileWithOplock(filepath: super::super::Foundation::PWSTR, flags: CF_OPEN_FILE_FLAGS, protectedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn CfQuerySyncProviderStatus(connectionkey: CF_CONNECTION_KEY, providerstatus: *mut CF_SYNC_PROVIDER_STATUS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfReferenceProtectedHandle(protectedhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfRegisterSyncRoot(syncrootpath: super::super::Foundation::PWSTR, registration: *const CF_SYNC_REGISTRATION, policies: *const CF_SYNC_POLICIES, registerflags: CF_REGISTER_FLAGS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfReleaseProtectedHandle(protectedhandle: super::super::Foundation::HANDLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfReleaseTransferKey(filehandle: super::super::Foundation::HANDLE, transferkey: *mut i64);
    pub fn CfReportProviderProgress(connectionkey: CF_CONNECTION_KEY, transferkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64) -> ::windows_sys::core::HRESULT;
    pub fn CfReportProviderProgress2(connectionkey: CF_CONNECTION_KEY, transferkey: i64, requestkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64, targetsessionid: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfReportSyncStatus(syncrootpath: super::super::Foundation::PWSTR, syncstatus: *const CF_SYNC_STATUS) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfRevertPlaceholder(filehandle: super::super::Foundation::HANDLE, revertflags: CF_REVERT_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
    pub fn CfSetCorrelationVector(filehandle: super::super::Foundation::HANDLE, correlationvector: *const super::super::System::CorrelationVector::CORRELATION_VECTOR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfSetInSyncState(filehandle: super::super::Foundation::HANDLE, insyncstate: CF_IN_SYNC_STATE, insyncflags: CF_SET_IN_SYNC_FLAGS, insyncusn: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfSetPinState(filehandle: super::super::Foundation::HANDLE, pinstate: CF_PIN_STATE, pinflags: CF_SET_PIN_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfUnregisterSyncRoot(syncrootpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO"))]
    pub fn CfUpdatePlaceholder(filehandle: super::super::Foundation::HANDLE, fsmetadata: *const CF_FS_METADATA, fileidentity: *const ::core::ffi::c_void, fileidentitylength: u32, dehydraterangearray: *const CF_FILE_RANGE, dehydraterangecount: u32, updateflags: CF_UPDATE_FLAGS, updateusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT;
    pub fn CfUpdateSyncProviderStatus(connectionkey: CF_CONNECTION_KEY, providerstatus: CF_SYNC_PROVIDER_STATUS) -> ::windows_sys::core::HRESULT;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
pub type CF_CALLBACK = unsafe extern "system" fn(callbackinfo: *const CF_CALLBACK_INFO, callbackparameters: *const CF_CALLBACK_PARAMETERS);
pub const CF_CALLBACK_CANCEL_FLAG_NONE: u32 = 0u32;
pub const CF_CALLBACK_CANCEL_FLAG_IO_TIMEOUT: u32 = 1u32;
pub const CF_CALLBACK_CANCEL_FLAG_IO_ABORTED: u32 = 2u32;
pub const CF_CALLBACK_CLOSE_COMPLETION_FLAG_NONE: u32 = 0u32;
pub const CF_CALLBACK_CLOSE_COMPLETION_FLAG_DELETED: u32 = 1u32;
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_NONE: u32 = 0u32;
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_BACKGROUND: u32 = 1u32;
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_DEHYDRATED: u32 = 2u32;
pub const CF_CALLBACK_DEHYDRATE_FLAG_NONE: u32 = 0u32;
pub const CF_CALLBACK_DEHYDRATE_FLAG_BACKGROUND: u32 = 1u32;
pub const CF_CALLBACK_DEHYDRATION_REASON_NONE: i32 = 0i32;
pub const CF_CALLBACK_DEHYDRATION_REASON_USER_MANUAL: i32 = 1i32;
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_LOW_SPACE: i32 = 2i32;
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_INACTIVITY: i32 = 3i32;
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_OS_UPGRADE: i32 = 4i32;
pub const CF_CALLBACK_DELETE_COMPLETION_FLAG_NONE: u32 = 0u32;
pub const CF_CALLBACK_DELETE_FLAG_NONE: u32 = 0u32;
pub const CF_CALLBACK_DELETE_FLAG_IS_DIRECTORY: u32 = 1u32;
pub const CF_CALLBACK_DELETE_FLAG_IS_UNDELETE: u32 = 2u32;
pub const CF_CALLBACK_FETCH_DATA_FLAG_NONE: u32 = 0u32;
pub const CF_CALLBACK_FETCH_DATA_FLAG_RECOVERY: u32 = 1u32;
pub const CF_CALLBACK_FETCH_DATA_FLAG_EXPLICIT_HYDRATION: u32 = 2u32;
pub const CF_CALLBACK_FETCH_PLACEHOLDERS_FLAG_NONE: u32 = 0u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
pub struct CF_CALLBACK_INFO {
    pub StructSize: u32,
    pub ConnectionKey: CF_CONNECTION_KEY,
    pub CallbackContext: *mut ::core::ffi::c_void,
    pub VolumeGuidName: super::super::Foundation::PWSTR,
    pub VolumeDosName: super::super::Foundation::PWSTR,
    pub VolumeSerialNumber: u32,
    pub SyncRootFileId: i64,
    pub SyncRootIdentity: *mut ::core::ffi::c_void,
    pub SyncRootIdentityLength: u32,
    pub FileId: i64,
    pub FileSize: i64,
    pub FileIdentity: *mut ::core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub NormalizedPath: super::super::Foundation::PWSTR,
    pub TransferKey: i64,
    pub PriorityHint: u8,
    pub CorrelationVector: *mut super::super::System::CorrelationVector::CORRELATION_VECTOR,
    pub ProcessInfo: *mut CF_PROCESS_INFO,
    pub RequestKey: i64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::marker::Copy for CF_CALLBACK_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::clone::Clone for CF_CALLBACK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_NONE: u32 = 0u32;
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNKNOWN: u32 = 1u32;
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNSUPPORTED: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS {
    pub ParamSize: u32,
    pub Anonymous: CF_CALLBACK_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union CF_CALLBACK_PARAMETERS_0 {
    pub Cancel: CF_CALLBACK_PARAMETERS_0_0,
    pub FetchData: CF_CALLBACK_PARAMETERS_0_6,
    pub ValidateData: CF_CALLBACK_PARAMETERS_0_11,
    pub FetchPlaceholders: CF_CALLBACK_PARAMETERS_0_7,
    pub OpenCompletion: CF_CALLBACK_PARAMETERS_0_8,
    pub CloseCompletion: CF_CALLBACK_PARAMETERS_0_1,
    pub Dehydrate: CF_CALLBACK_PARAMETERS_0_3,
    pub DehydrateCompletion: CF_CALLBACK_PARAMETERS_0_2,
    pub Delete: CF_CALLBACK_PARAMETERS_0_5,
    pub DeleteCompletion: CF_CALLBACK_PARAMETERS_0_4,
    pub Rename: CF_CALLBACK_PARAMETERS_0_10,
    pub RenameCompletion: CF_CALLBACK_PARAMETERS_0_9,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_0 {
    pub Flags: CF_CALLBACK_CANCEL_FLAGS,
    pub Anonymous: CF_CALLBACK_PARAMETERS_0_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union CF_CALLBACK_PARAMETERS_0_0_0 {
    pub FetchData: CF_CALLBACK_PARAMETERS_0_0_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_0_0_0 {
    pub FileOffset: i64,
    pub Length: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_1 {
    pub Flags: CF_CALLBACK_CLOSE_COMPLETION_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_2 {
    pub Flags: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS,
    pub Reason: CF_CALLBACK_DEHYDRATION_REASON,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_3 {
    pub Flags: CF_CALLBACK_DEHYDRATE_FLAGS,
    pub Reason: CF_CALLBACK_DEHYDRATION_REASON,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_4 {
    pub Flags: CF_CALLBACK_DELETE_COMPLETION_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_5 {
    pub Flags: CF_CALLBACK_DELETE_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_6 {
    pub Flags: CF_CALLBACK_FETCH_DATA_FLAGS,
    pub RequiredFileOffset: i64,
    pub RequiredLength: i64,
    pub OptionalFileOffset: i64,
    pub OptionalLength: i64,
    pub LastDehydrationTime: i64,
    pub LastDehydrationReason: CF_CALLBACK_DEHYDRATION_REASON,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_7 {
    pub Flags: CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS,
    pub Pattern: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_7 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_8 {
    pub Flags: CF_CALLBACK_OPEN_COMPLETION_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_9 {
    pub Flags: CF_CALLBACK_RENAME_COMPLETION_FLAGS,
    pub SourcePath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_9 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_10 {
    pub Flags: CF_CALLBACK_RENAME_FLAGS,
    pub TargetPath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_10 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_11 {
    pub Flags: CF_CALLBACK_VALIDATE_DATA_FLAGS,
    pub RequiredFileOffset: i64,
    pub RequiredLength: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_11 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_11 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
pub struct CF_CALLBACK_REGISTRATION {
    pub Type: CF_CALLBACK_TYPE,
    pub Callback: CF_CALLBACK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::marker::Copy for CF_CALLBACK_REGISTRATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::clone::Clone for CF_CALLBACK_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_CALLBACK_RENAME_COMPLETION_FLAG_NONE: u32 = 0u32;
pub const CF_CALLBACK_RENAME_FLAG_NONE: u32 = 0u32;
pub const CF_CALLBACK_RENAME_FLAG_IS_DIRECTORY: u32 = 1u32;
pub const CF_CALLBACK_RENAME_FLAG_SOURCE_IN_SCOPE: u32 = 2u32;
pub const CF_CALLBACK_RENAME_FLAG_TARGET_IN_SCOPE: u32 = 4u32;
pub const CF_CALLBACK_TYPE_FETCH_DATA: i32 = 0i32;
pub const CF_CALLBACK_TYPE_VALIDATE_DATA: i32 = 1i32;
pub const CF_CALLBACK_TYPE_CANCEL_FETCH_DATA: i32 = 2i32;
pub const CF_CALLBACK_TYPE_FETCH_PLACEHOLDERS: i32 = 3i32;
pub const CF_CALLBACK_TYPE_CANCEL_FETCH_PLACEHOLDERS: i32 = 4i32;
pub const CF_CALLBACK_TYPE_NOTIFY_FILE_OPEN_COMPLETION: i32 = 5i32;
pub const CF_CALLBACK_TYPE_NOTIFY_FILE_CLOSE_COMPLETION: i32 = 6i32;
pub const CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE: i32 = 7i32;
pub const CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE_COMPLETION: i32 = 8i32;
pub const CF_CALLBACK_TYPE_NOTIFY_DELETE: i32 = 9i32;
pub const CF_CALLBACK_TYPE_NOTIFY_DELETE_COMPLETION: i32 = 10i32;
pub const CF_CALLBACK_TYPE_NOTIFY_RENAME: i32 = 11i32;
pub const CF_CALLBACK_TYPE_NOTIFY_RENAME_COMPLETION: i32 = 12i32;
pub const CF_CALLBACK_TYPE_NONE: i32 = -1i32;
pub const CF_CALLBACK_VALIDATE_DATA_FLAG_NONE: u32 = 0u32;
pub const CF_CALLBACK_VALIDATE_DATA_FLAG_EXPLICIT_HYDRATION: u32 = 2u32;
pub type CF_CONNECTION_KEY = isize;
pub const CF_CONNECT_FLAG_NONE: u32 = 0u32;
pub const CF_CONNECT_FLAG_REQUIRE_PROCESS_INFO: u32 = 2u32;
pub const CF_CONNECT_FLAG_REQUIRE_FULL_FILE_PATH: u32 = 4u32;
pub const CF_CONNECT_FLAG_BLOCK_SELF_IMPLICIT_HYDRATION: u32 = 8u32;
pub const CF_CONVERT_FLAG_NONE: u32 = 0u32;
pub const CF_CONVERT_FLAG_MARK_IN_SYNC: u32 = 1u32;
pub const CF_CONVERT_FLAG_DEHYDRATE: u32 = 2u32;
pub const CF_CONVERT_FLAG_ENABLE_ON_DEMAND_POPULATION: u32 = 4u32;
pub const CF_CONVERT_FLAG_ALWAYS_FULL: u32 = 8u32;
pub const CF_CONVERT_FLAG_FORCE_CONVERT_TO_CLOUD_FILE: u32 = 16u32;
pub const CF_CREATE_FLAG_NONE: u32 = 0u32;
pub const CF_CREATE_FLAG_STOP_ON_ERROR: u32 = 1u32;
pub const CF_DEHYDRATE_FLAG_NONE: u32 = 0u32;
pub const CF_DEHYDRATE_FLAG_BACKGROUND: u32 = 1u32;
#[repr(C)]
pub struct CF_FILE_RANGE {
    pub StartingOffset: i64,
    pub Length: i64,
}
impl ::core::marker::Copy for CF_FILE_RANGE {}
impl ::core::clone::Clone for CF_FILE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct CF_FS_METADATA {
    pub BasicInfo: super::FileSystem::FILE_BASIC_INFO,
    pub FileSize: i64,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for CF_FS_METADATA {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for CF_FS_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_HARDLINK_POLICY_NONE: u32 = 0u32;
pub const CF_HARDLINK_POLICY_ALLOWED: u32 = 1u32;
pub const CF_HYDRATE_FLAG_NONE: u32 = 0u32;
#[repr(C)]
pub struct CF_HYDRATION_POLICY {
    pub Primary: CF_HYDRATION_POLICY_PRIMARY_USHORT,
    pub Modifier: CF_HYDRATION_POLICY_MODIFIER_USHORT,
}
impl ::core::marker::Copy for CF_HYDRATION_POLICY {}
impl ::core::clone::Clone for CF_HYDRATION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_HYDRATION_POLICY_MODIFIER_NONE: u16 = 0u16;
pub const CF_HYDRATION_POLICY_MODIFIER_VALIDATION_REQUIRED: u16 = 1u16;
pub const CF_HYDRATION_POLICY_MODIFIER_STREAMING_ALLOWED: u16 = 2u16;
pub const CF_HYDRATION_POLICY_MODIFIER_AUTO_DEHYDRATION_ALLOWED: u16 = 4u16;
pub const CF_HYDRATION_POLICY_MODIFIER_ALLOW_FULL_RESTART_HYDRATION: u16 = 8u16;
#[repr(C)]
pub struct CF_HYDRATION_POLICY_MODIFIER_USHORT {
    pub us: u16,
}
impl ::core::marker::Copy for CF_HYDRATION_POLICY_MODIFIER_USHORT {}
impl ::core::clone::Clone for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_HYDRATION_POLICY_PARTIAL: u16 = 0u16;
pub const CF_HYDRATION_POLICY_PROGRESSIVE: u16 = 1u16;
pub const CF_HYDRATION_POLICY_FULL: u16 = 2u16;
pub const CF_HYDRATION_POLICY_ALWAYS_FULL: u16 = 3u16;
#[repr(C)]
pub struct CF_HYDRATION_POLICY_PRIMARY_USHORT {
    pub us: u16,
}
impl ::core::marker::Copy for CF_HYDRATION_POLICY_PRIMARY_USHORT {}
impl ::core::clone::Clone for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_INSYNC_POLICY_NONE: u32 = 0u32;
pub const CF_INSYNC_POLICY_TRACK_FILE_CREATION_TIME: u32 = 1u32;
pub const CF_INSYNC_POLICY_TRACK_FILE_READONLY_ATTRIBUTE: u32 = 2u32;
pub const CF_INSYNC_POLICY_TRACK_FILE_HIDDEN_ATTRIBUTE: u32 = 4u32;
pub const CF_INSYNC_POLICY_TRACK_FILE_SYSTEM_ATTRIBUTE: u32 = 8u32;
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_CREATION_TIME: u32 = 16u32;
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_READONLY_ATTRIBUTE: u32 = 32u32;
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_HIDDEN_ATTRIBUTE: u32 = 64u32;
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_SYSTEM_ATTRIBUTE: u32 = 128u32;
pub const CF_INSYNC_POLICY_TRACK_FILE_LAST_WRITE_TIME: u32 = 256u32;
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_LAST_WRITE_TIME: u32 = 512u32;
pub const CF_INSYNC_POLICY_TRACK_FILE_ALL: u32 = 5592335u32;
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_ALL: u32 = 11184880u32;
pub const CF_INSYNC_POLICY_TRACK_ALL: u32 = 16777215u32;
pub const CF_INSYNC_POLICY_PRESERVE_INSYNC_FOR_SYNC_ENGINE: u32 = 2147483648u32;
pub const CF_IN_SYNC_STATE_NOT_IN_SYNC: i32 = 0i32;
pub const CF_IN_SYNC_STATE_IN_SYNC: i32 = 1i32;
pub const CF_MAX_PRIORITY_HINT: u32 = 15u32;
pub const CF_MAX_PROVIDER_NAME_LENGTH: u32 = 255u32;
pub const CF_MAX_PROVIDER_VERSION_LENGTH: u32 = 255u32;
pub const CF_OPEN_FILE_FLAG_NONE: u32 = 0u32;
pub const CF_OPEN_FILE_FLAG_EXCLUSIVE: u32 = 1u32;
pub const CF_OPEN_FILE_FLAG_WRITE_ACCESS: u32 = 2u32;
pub const CF_OPEN_FILE_FLAG_DELETE_ACCESS: u32 = 4u32;
pub const CF_OPEN_FILE_FLAG_FOREGROUND: u32 = 8u32;
pub const CF_OPERATION_ACK_DATA_FLAG_NONE: u32 = 0u32;
pub const CF_OPERATION_ACK_DEHYDRATE_FLAG_NONE: u32 = 0u32;
pub const CF_OPERATION_ACK_DELETE_FLAG_NONE: u32 = 0u32;
pub const CF_OPERATION_ACK_RENAME_FLAG_NONE: u32 = 0u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
pub struct CF_OPERATION_INFO {
    pub StructSize: u32,
    pub Type: CF_OPERATION_TYPE,
    pub ConnectionKey: CF_CONNECTION_KEY,
    pub TransferKey: i64,
    pub CorrelationVector: *mut super::super::System::CorrelationVector::CORRELATION_VECTOR,
    pub SyncStatus: *mut CF_SYNC_STATUS,
    pub RequestKey: i64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::marker::Copy for CF_OPERATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::clone::Clone for CF_OPERATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_OPERATION_PARAMETERS {
    pub ParamSize: u32,
    pub Anonymous: CF_OPERATION_PARAMETERS_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::marker::Copy for CF_OPERATION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::clone::Clone for CF_OPERATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub union CF_OPERATION_PARAMETERS_0 {
    pub TransferData: CF_OPERATION_PARAMETERS_0_6,
    pub RetrieveData: CF_OPERATION_PARAMETERS_0_5,
    pub AckData: CF_OPERATION_PARAMETERS_0_0,
    pub RestartHydration: CF_OPERATION_PARAMETERS_0_4,
    pub TransferPlaceholders: CF_OPERATION_PARAMETERS_0_7,
    pub AckDehydrate: CF_OPERATION_PARAMETERS_0_1,
    pub AckRename: CF_OPERATION_PARAMETERS_0_3,
    pub AckDelete: CF_OPERATION_PARAMETERS_0_2,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::marker::Copy for CF_OPERATION_PARAMETERS_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::clone::Clone for CF_OPERATION_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_OPERATION_PARAMETERS_0_0 {
    pub Flags: CF_OPERATION_ACK_DATA_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
    pub Offset: i64,
    pub Length: i64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::marker::Copy for CF_OPERATION_PARAMETERS_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::clone::Clone for CF_OPERATION_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_OPERATION_PARAMETERS_0_1 {
    pub Flags: CF_OPERATION_ACK_DEHYDRATE_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
    pub FileIdentity: *mut ::core::ffi::c_void,
    pub FileIdentityLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::marker::Copy for CF_OPERATION_PARAMETERS_0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::clone::Clone for CF_OPERATION_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_OPERATION_PARAMETERS_0_2 {
    pub Flags: CF_OPERATION_ACK_DELETE_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::marker::Copy for CF_OPERATION_PARAMETERS_0_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::clone::Clone for CF_OPERATION_PARAMETERS_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_OPERATION_PARAMETERS_0_3 {
    pub Flags: CF_OPERATION_ACK_RENAME_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::marker::Copy for CF_OPERATION_PARAMETERS_0_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::clone::Clone for CF_OPERATION_PARAMETERS_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_OPERATION_PARAMETERS_0_4 {
    pub Flags: CF_OPERATION_RESTART_HYDRATION_FLAGS,
    pub FsMetadata: *mut CF_FS_METADATA,
    pub FileIdentity: *mut ::core::ffi::c_void,
    pub FileIdentityLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::marker::Copy for CF_OPERATION_PARAMETERS_0_4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::clone::Clone for CF_OPERATION_PARAMETERS_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_OPERATION_PARAMETERS_0_5 {
    pub Flags: CF_OPERATION_RETRIEVE_DATA_FLAGS,
    pub Buffer: *mut ::core::ffi::c_void,
    pub Offset: i64,
    pub Length: i64,
    pub ReturnedLength: i64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::marker::Copy for CF_OPERATION_PARAMETERS_0_5 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::clone::Clone for CF_OPERATION_PARAMETERS_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_OPERATION_PARAMETERS_0_6 {
    pub Flags: CF_OPERATION_TRANSFER_DATA_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
    pub Buffer: *mut ::core::ffi::c_void,
    pub Offset: i64,
    pub Length: i64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::marker::Copy for CF_OPERATION_PARAMETERS_0_6 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::clone::Clone for CF_OPERATION_PARAMETERS_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_OPERATION_PARAMETERS_0_7 {
    pub Flags: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
    pub PlaceholderTotalCount: i64,
    pub PlaceholderArray: *mut CF_PLACEHOLDER_CREATE_INFO,
    pub PlaceholderCount: u32,
    pub EntriesProcessed: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::marker::Copy for CF_OPERATION_PARAMETERS_0_7 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::clone::Clone for CF_OPERATION_PARAMETERS_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_OPERATION_RESTART_HYDRATION_FLAG_NONE: u32 = 0u32;
pub const CF_OPERATION_RESTART_HYDRATION_FLAG_MARK_IN_SYNC: u32 = 1u32;
pub const CF_OPERATION_RETRIEVE_DATA_FLAG_NONE: u32 = 0u32;
pub const CF_OPERATION_TRANSFER_DATA_FLAG_NONE: u32 = 0u32;
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_NONE: u32 = 0u32;
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_STOP_ON_ERROR: u32 = 1u32;
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_DISABLE_ON_DEMAND_POPULATION: u32 = 2u32;
pub const CF_OPERATION_TYPE_TRANSFER_DATA: i32 = 0i32;
pub const CF_OPERATION_TYPE_RETRIEVE_DATA: i32 = 1i32;
pub const CF_OPERATION_TYPE_ACK_DATA: i32 = 2i32;
pub const CF_OPERATION_TYPE_RESTART_HYDRATION: i32 = 3i32;
pub const CF_OPERATION_TYPE_TRANSFER_PLACEHOLDERS: i32 = 4i32;
pub const CF_OPERATION_TYPE_ACK_DEHYDRATE: i32 = 5i32;
pub const CF_OPERATION_TYPE_ACK_DELETE: i32 = 6i32;
pub const CF_OPERATION_TYPE_ACK_RENAME: i32 = 7i32;
pub const CF_PIN_STATE_UNSPECIFIED: i32 = 0i32;
pub const CF_PIN_STATE_PINNED: i32 = 1i32;
pub const CF_PIN_STATE_UNPINNED: i32 = 2i32;
pub const CF_PIN_STATE_EXCLUDED: i32 = 3i32;
pub const CF_PIN_STATE_INHERIT: i32 = 4i32;
#[repr(C)]
pub struct CF_PLACEHOLDER_BASIC_INFO {
    pub PinState: CF_PIN_STATE,
    pub InSyncState: CF_IN_SYNC_STATE,
    pub FileId: i64,
    pub SyncRootFileId: i64,
    pub FileIdentityLength: u32,
    pub FileIdentity: [u8; 1],
}
impl ::core::marker::Copy for CF_PLACEHOLDER_BASIC_INFO {}
impl ::core::clone::Clone for CF_PLACEHOLDER_BASIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_PLACEHOLDER_CREATE_FLAG_NONE: u32 = 0u32;
pub const CF_PLACEHOLDER_CREATE_FLAG_DISABLE_ON_DEMAND_POPULATION: u32 = 1u32;
pub const CF_PLACEHOLDER_CREATE_FLAG_MARK_IN_SYNC: u32 = 2u32;
pub const CF_PLACEHOLDER_CREATE_FLAG_SUPERSEDE: u32 = 4u32;
pub const CF_PLACEHOLDER_CREATE_FLAG_ALWAYS_FULL: u32 = 8u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_PLACEHOLDER_CREATE_INFO {
    pub RelativeFileName: super::super::Foundation::PWSTR,
    pub FsMetadata: CF_FS_METADATA,
    pub FileIdentity: *mut ::core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub Flags: CF_PLACEHOLDER_CREATE_FLAGS,
    pub Result: ::windows_sys::core::HRESULT,
    pub CreateUsn: i64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::marker::Copy for CF_PLACEHOLDER_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::clone::Clone for CF_PLACEHOLDER_CREATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_PLACEHOLDER_INFO_BASIC: i32 = 0i32;
pub const CF_PLACEHOLDER_INFO_STANDARD: i32 = 1i32;
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_DEFAULT: i32 = 0i32;
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_CREATE_UNRESTRICTED: i32 = 1i32;
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_CONVERT_TO_UNRESTRICTED: i32 = 2i32;
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_UPDATE_UNRESTRICTED: i32 = 4i32;
pub const CF_PLACEHOLDER_MAX_FILE_IDENTITY_LENGTH: u32 = 4096u32;
pub const CF_PLACEHOLDER_RANGE_INFO_ONDISK: i32 = 1i32;
pub const CF_PLACEHOLDER_RANGE_INFO_VALIDATED: i32 = 2i32;
pub const CF_PLACEHOLDER_RANGE_INFO_MODIFIED: i32 = 3i32;
#[repr(C)]
pub struct CF_PLACEHOLDER_STANDARD_INFO {
    pub OnDiskDataSize: i64,
    pub ValidatedDataSize: i64,
    pub ModifiedDataSize: i64,
    pub PropertiesSize: i64,
    pub PinState: CF_PIN_STATE,
    pub InSyncState: CF_IN_SYNC_STATE,
    pub FileId: i64,
    pub SyncRootFileId: i64,
    pub FileIdentityLength: u32,
    pub FileIdentity: [u8; 1],
}
impl ::core::marker::Copy for CF_PLACEHOLDER_STANDARD_INFO {}
impl ::core::clone::Clone for CF_PLACEHOLDER_STANDARD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_PLACEHOLDER_STATE_NO_STATES: u32 = 0u32;
pub const CF_PLACEHOLDER_STATE_PLACEHOLDER: u32 = 1u32;
pub const CF_PLACEHOLDER_STATE_SYNC_ROOT: u32 = 2u32;
pub const CF_PLACEHOLDER_STATE_ESSENTIAL_PROP_PRESENT: u32 = 4u32;
pub const CF_PLACEHOLDER_STATE_IN_SYNC: u32 = 8u32;
pub const CF_PLACEHOLDER_STATE_PARTIAL: u32 = 16u32;
pub const CF_PLACEHOLDER_STATE_PARTIALLY_ON_DISK: u32 = 32u32;
pub const CF_PLACEHOLDER_STATE_INVALID: u32 = 4294967295u32;
#[repr(C)]
pub struct CF_PLATFORM_INFO {
    pub BuildNumber: u32,
    pub RevisionNumber: u32,
    pub IntegrationNumber: u32,
}
impl ::core::marker::Copy for CF_PLATFORM_INFO {}
impl ::core::clone::Clone for CF_PLATFORM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CF_POPULATION_POLICY {
    pub Primary: CF_POPULATION_POLICY_PRIMARY_USHORT,
    pub Modifier: CF_POPULATION_POLICY_MODIFIER_USHORT,
}
impl ::core::marker::Copy for CF_POPULATION_POLICY {}
impl ::core::clone::Clone for CF_POPULATION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_POPULATION_POLICY_MODIFIER_NONE: u16 = 0u16;
#[repr(C)]
pub struct CF_POPULATION_POLICY_MODIFIER_USHORT {
    pub us: u16,
}
impl ::core::marker::Copy for CF_POPULATION_POLICY_MODIFIER_USHORT {}
impl ::core::clone::Clone for CF_POPULATION_POLICY_MODIFIER_USHORT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_POPULATION_POLICY_PARTIAL: u16 = 0u16;
pub const CF_POPULATION_POLICY_FULL: u16 = 2u16;
pub const CF_POPULATION_POLICY_ALWAYS_FULL: u16 = 3u16;
#[repr(C)]
pub struct CF_POPULATION_POLICY_PRIMARY_USHORT {
    pub us: u16,
}
impl ::core::marker::Copy for CF_POPULATION_POLICY_PRIMARY_USHORT {}
impl ::core::clone::Clone for CF_POPULATION_POLICY_PRIMARY_USHORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_PROCESS_INFO {
    pub StructSize: u32,
    pub ProcessId: u32,
    pub ImagePath: super::super::Foundation::PWSTR,
    pub PackageName: super::super::Foundation::PWSTR,
    pub ApplicationId: super::super::Foundation::PWSTR,
    pub CommandLine: super::super::Foundation::PWSTR,
    pub SessionId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_PROCESS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_PROCESS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_REGISTER_FLAG_NONE: u32 = 0u32;
pub const CF_REGISTER_FLAG_UPDATE: u32 = 1u32;
pub const CF_REGISTER_FLAG_DISABLE_ON_DEMAND_POPULATION_ON_ROOT: u32 = 2u32;
pub const CF_REGISTER_FLAG_MARK_IN_SYNC_ON_ROOT: u32 = 4u32;
pub const CF_REQUEST_KEY_DEFAULT: u32 = 0u32;
pub const CF_REVERT_FLAG_NONE: u32 = 0u32;
pub const CF_SET_IN_SYNC_FLAG_NONE: u32 = 0u32;
pub const CF_SET_PIN_FLAG_NONE: u32 = 0u32;
pub const CF_SET_PIN_FLAG_RECURSE: u32 = 1u32;
pub const CF_SET_PIN_FLAG_RECURSE_ONLY: u32 = 2u32;
pub const CF_SET_PIN_FLAG_RECURSE_STOP_ON_ERROR: u32 = 4u32;
#[repr(C)]
pub struct CF_SYNC_POLICIES {
    pub StructSize: u32,
    pub Hydration: CF_HYDRATION_POLICY,
    pub Population: CF_POPULATION_POLICY,
    pub InSync: CF_INSYNC_POLICY,
    pub HardLink: CF_HARDLINK_POLICY,
    pub PlaceholderManagement: CF_PLACEHOLDER_MANAGEMENT_POLICY,
}
impl ::core::marker::Copy for CF_SYNC_POLICIES {}
impl ::core::clone::Clone for CF_SYNC_POLICIES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_PROVIDER_STATUS_DISCONNECTED: u32 = 0u32;
pub const CF_PROVIDER_STATUS_IDLE: u32 = 1u32;
pub const CF_PROVIDER_STATUS_POPULATE_NAMESPACE: u32 = 2u32;
pub const CF_PROVIDER_STATUS_POPULATE_METADATA: u32 = 4u32;
pub const CF_PROVIDER_STATUS_POPULATE_CONTENT: u32 = 8u32;
pub const CF_PROVIDER_STATUS_SYNC_INCREMENTAL: u32 = 16u32;
pub const CF_PROVIDER_STATUS_SYNC_FULL: u32 = 32u32;
pub const CF_PROVIDER_STATUS_CONNECTIVITY_LOST: u32 = 64u32;
pub const CF_PROVIDER_STATUS_CLEAR_FLAGS: u32 = 2147483648u32;
pub const CF_PROVIDER_STATUS_TERMINATED: u32 = 3221225473u32;
pub const CF_PROVIDER_STATUS_ERROR: u32 = 3221225474u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_SYNC_REGISTRATION {
    pub StructSize: u32,
    pub ProviderName: super::super::Foundation::PWSTR,
    pub ProviderVersion: super::super::Foundation::PWSTR,
    pub SyncRootIdentity: *mut ::core::ffi::c_void,
    pub SyncRootIdentityLength: u32,
    pub FileIdentity: *mut ::core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub ProviderId: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_SYNC_REGISTRATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_SYNC_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CF_SYNC_ROOT_BASIC_INFO {
    pub SyncRootFileId: i64,
}
impl ::core::marker::Copy for CF_SYNC_ROOT_BASIC_INFO {}
impl ::core::clone::Clone for CF_SYNC_ROOT_BASIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_SYNC_ROOT_INFO_BASIC: i32 = 0i32;
pub const CF_SYNC_ROOT_INFO_STANDARD: i32 = 1i32;
pub const CF_SYNC_ROOT_INFO_PROVIDER: i32 = 2i32;
#[repr(C)]
pub struct CF_SYNC_ROOT_PROVIDER_INFO {
    pub ProviderStatus: CF_SYNC_PROVIDER_STATUS,
    pub ProviderName: [u16; 256],
    pub ProviderVersion: [u16; 256],
}
impl ::core::marker::Copy for CF_SYNC_ROOT_PROVIDER_INFO {}
impl ::core::clone::Clone for CF_SYNC_ROOT_PROVIDER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CF_SYNC_ROOT_STANDARD_INFO {
    pub SyncRootFileId: i64,
    pub HydrationPolicy: CF_HYDRATION_POLICY,
    pub PopulationPolicy: CF_POPULATION_POLICY,
    pub InSyncPolicy: CF_INSYNC_POLICY,
    pub HardLinkPolicy: CF_HARDLINK_POLICY,
    pub ProviderStatus: CF_SYNC_PROVIDER_STATUS,
    pub ProviderName: [u16; 256],
    pub ProviderVersion: [u16; 256],
    pub SyncRootIdentityLength: u32,
    pub SyncRootIdentity: [u8; 1],
}
impl ::core::marker::Copy for CF_SYNC_ROOT_STANDARD_INFO {}
impl ::core::clone::Clone for CF_SYNC_ROOT_STANDARD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CF_SYNC_STATUS {
    pub StructSize: u32,
    pub Code: u32,
    pub DescriptionOffset: u32,
    pub DescriptionLength: u32,
    pub DeviceIdOffset: u32,
    pub DeviceIdLength: u32,
}
impl ::core::marker::Copy for CF_SYNC_STATUS {}
impl ::core::clone::Clone for CF_SYNC_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_UPDATE_FLAG_NONE: u32 = 0u32;
pub const CF_UPDATE_FLAG_VERIFY_IN_SYNC: u32 = 1u32;
pub const CF_UPDATE_FLAG_MARK_IN_SYNC: u32 = 2u32;
pub const CF_UPDATE_FLAG_DEHYDRATE: u32 = 4u32;
pub const CF_UPDATE_FLAG_ENABLE_ON_DEMAND_POPULATION: u32 = 8u32;
pub const CF_UPDATE_FLAG_DISABLE_ON_DEMAND_POPULATION: u32 = 16u32;
pub const CF_UPDATE_FLAG_REMOVE_FILE_IDENTITY: u32 = 32u32;
pub const CF_UPDATE_FLAG_CLEAR_IN_SYNC: u32 = 64u32;
pub const CF_UPDATE_FLAG_REMOVE_PROPERTY: u32 = 128u32;
pub const CF_UPDATE_FLAG_PASSTHROUGH_FS_METADATA: u32 = 256u32;
pub const CF_UPDATE_FLAG_ALWAYS_FULL: u32 = 512u32;
pub const CF_UPDATE_FLAG_ALLOW_PARTIAL: u32 = 1024u32;
