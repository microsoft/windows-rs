#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfCloseHandle(filehandle: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_CorrelationVector`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
    pub fn CfConnectSyncRoot(syncrootpath: super::super::Foundation::PWSTR, callbacktable: *const ::core::mem::ManuallyDrop<CF_CALLBACK_REGISTRATION>, callbackcontext: *const ::core::ffi::c_void, connectflags: CF_CONNECT_FLAGS, connectionkey: *mut CF_CONNECTION_KEY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfConvertToPlaceholder(filehandle: super::super::Foundation::HANDLE, fileidentity: *const ::core::ffi::c_void, fileidentitylength: u32, convertflags: CF_CONVERT_FLAGS, convertusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_Storage_FileSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
    pub fn CfCreatePlaceholders(basedirectorypath: super::super::Foundation::PWSTR, placeholderarray: *mut CF_PLACEHOLDER_CREATE_INFO, placeholdercount: u32, createflags: CF_CREATE_FLAGS, entriesprocessed: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfDehydratePlaceholder(filehandle: super::super::Foundation::HANDLE, startingoffset: i64, length: i64, dehydrateflags: CF_DEHYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfDisconnectSyncRoot(connectionkey: CF_CONNECTION_KEY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_Storage_FileSystem`, `Win32_System_CorrelationVector`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_CorrelationVector"))]
    pub fn CfExecute(opinfo: *const CF_OPERATION_INFO, opparams: *mut CF_OPERATION_PARAMETERS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_CorrelationVector`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
    pub fn CfGetCorrelationVector(filehandle: super::super::Foundation::HANDLE, correlationvector: *mut super::super::System::CorrelationVector::CORRELATION_VECTOR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetPlaceholderInfo(filehandle: super::super::Foundation::HANDLE, infoclass: CF_PLACEHOLDER_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetPlaceholderRangeInfo(filehandle: super::super::Foundation::HANDLE, infoclass: CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset: i64, length: i64, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfGetPlaceholderStateFromAttributeTag(fileattributes: u32, reparsetag: u32) -> CF_PLACEHOLDER_STATE;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Storage_FileSystem`*"]
    #[cfg(feature = "Win32_Storage_FileSystem")]
    pub fn CfGetPlaceholderStateFromFileInfo(infobuffer: *const ::core::ffi::c_void, infoclass: super::FileSystem::FILE_INFO_BY_HANDLE_CLASS) -> CF_PLACEHOLDER_STATE;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_Storage_FileSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
    pub fn CfGetPlaceholderStateFromFindData(finddata: *const super::FileSystem::WIN32_FIND_DATAA) -> CF_PLACEHOLDER_STATE;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfGetPlatformInfo(platformversion: *mut CF_PLATFORM_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetSyncRootInfoByHandle(filehandle: super::super::Foundation::HANDLE, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetSyncRootInfoByPath(filepath: super::super::Foundation::PWSTR, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetTransferKey(filehandle: super::super::Foundation::HANDLE, transferkey: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfGetWin32HandleFromProtectedHandle(protectedhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfHydratePlaceholder(filehandle: super::super::Foundation::HANDLE, startingoffset: i64, length: i64, hydrateflags: CF_HYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfOpenFileWithOplock(filepath: super::super::Foundation::PWSTR, flags: CF_OPEN_FILE_FLAGS, protectedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfQuerySyncProviderStatus(connectionkey: CF_CONNECTION_KEY, providerstatus: *mut CF_SYNC_PROVIDER_STATUS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfReferenceProtectedHandle(protectedhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfRegisterSyncRoot(syncrootpath: super::super::Foundation::PWSTR, registration: *const CF_SYNC_REGISTRATION, policies: *const CF_SYNC_POLICIES, registerflags: CF_REGISTER_FLAGS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfReleaseProtectedHandle(protectedhandle: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfReleaseTransferKey(filehandle: super::super::Foundation::HANDLE, transferkey: *mut i64);
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfReportProviderProgress(connectionkey: CF_CONNECTION_KEY, transferkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfReportProviderProgress2(connectionkey: CF_CONNECTION_KEY, transferkey: i64, requestkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64, targetsessionid: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfReportSyncStatus(syncrootpath: super::super::Foundation::PWSTR, syncstatus: *const CF_SYNC_STATUS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfRevertPlaceholder(filehandle: super::super::Foundation::HANDLE, revertflags: CF_REVERT_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_CorrelationVector`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
    pub fn CfSetCorrelationVector(filehandle: super::super::Foundation::HANDLE, correlationvector: *const super::super::System::CorrelationVector::CORRELATION_VECTOR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfSetInSyncState(filehandle: super::super::Foundation::HANDLE, insyncstate: CF_IN_SYNC_STATE, insyncflags: CF_SET_IN_SYNC_FLAGS, insyncusn: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CfSetPinState(filehandle: super::super::Foundation::HANDLE, pinstate: CF_PIN_STATE, pinflags: CF_SET_PIN_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CfUnregisterSyncRoot(syncrootpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`, `Win32_Foundation`, `Win32_Storage_FileSystem`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO"))]
    pub fn CfUpdatePlaceholder(filehandle: super::super::Foundation::HANDLE, fsmetadata: *const CF_FS_METADATA, fileidentity: *const ::core::ffi::c_void, fileidentitylength: u32, dehydraterangearray: *const CF_FILE_RANGE, dehydraterangecount: u32, updateflags: CF_UPDATE_FLAGS, updateusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_CloudFilters`*"]
    pub fn CfUpdateSyncProviderStatus(connectionkey: CF_CONNECTION_KEY, providerstatus: CF_SYNC_PROVIDER_STATUS) -> ::windows::runtime::HRESULT;
}
