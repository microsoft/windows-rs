#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    pub fn PrjAllocateAlignedBuffer(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, size: usize) -> *mut ::core::ffi::c_void;
    pub fn PrjClearNegativePathCache(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, totalentrynumber: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn PrjCompleteCommand(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, commandid: i32, completionresult: ::windows_sys::core::HRESULT, extendedparameters: *const PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjDeleteFile(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename: super::super::Foundation::PWSTR, updateflags: PRJ_UPDATE_TYPES, failurereason: *mut PRJ_UPDATE_FAILURE_CAUSES) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjDoesNameContainWildCards(filename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjFileNameCompare(filename1: super::super::Foundation::PWSTR, filename2: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjFileNameMatch(filenametocheck: super::super::Foundation::PWSTR, pattern: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjFillDirEntryBuffer(filename: super::super::Foundation::PWSTR, filebasicinfo: *const PRJ_FILE_BASIC_INFO, direntrybufferhandle: PRJ_DIR_ENTRY_BUFFER_HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjFillDirEntryBuffer2(direntrybufferhandle: PRJ_DIR_ENTRY_BUFFER_HANDLE, filename: super::super::Foundation::PWSTR, filebasicinfo: *const PRJ_FILE_BASIC_INFO, extendedinfo: *const PRJ_EXTENDED_INFO) -> ::windows_sys::core::HRESULT;
    pub fn PrjFreeAlignedBuffer(buffer: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjGetOnDiskFileState(destinationfilename: super::super::Foundation::PWSTR, filestate: *mut PRJ_FILE_STATE) -> ::windows_sys::core::HRESULT;
    pub fn PrjGetVirtualizationInstanceInfo(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, virtualizationinstanceinfo: *mut PRJ_VIRTUALIZATION_INSTANCE_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjMarkDirectoryAsPlaceholder(rootpathname: super::super::Foundation::PWSTR, targetpathname: super::super::Foundation::PWSTR, versioninfo: *const PRJ_PLACEHOLDER_VERSION_INFO, virtualizationinstanceid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjStartVirtualizing(virtualizationrootpath: super::super::Foundation::PWSTR, callbacks: *const PRJ_CALLBACKS, instancecontext: *const ::core::ffi::c_void, options: *const PRJ_STARTVIRTUALIZING_OPTIONS, namespacevirtualizationcontext: *mut PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT) -> ::windows_sys::core::HRESULT;
    pub fn PrjStopVirtualizing(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjUpdateFileIfNeeded(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename: super::super::Foundation::PWSTR, placeholderinfo: *const PRJ_PLACEHOLDER_INFO, placeholderinfosize: u32, updateflags: PRJ_UPDATE_TYPES, failurereason: *mut PRJ_UPDATE_FAILURE_CAUSES) -> ::windows_sys::core::HRESULT;
    pub fn PrjWriteFileData(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, datastreamid: *const ::windows_sys::core::GUID, buffer: *const ::core::ffi::c_void, byteoffset: u64, length: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjWritePlaceholderInfo(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename: super::super::Foundation::PWSTR, placeholderinfo: *const PRJ_PLACEHOLDER_INFO, placeholderinfosize: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrjWritePlaceholderInfo2(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename: super::super::Foundation::PWSTR, placeholderinfo: *const PRJ_PLACEHOLDER_INFO, placeholderinfosize: u32, extendedinfo: *const PRJ_EXTENDED_INFO) -> ::windows_sys::core::HRESULT;
}
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_CALLBACKS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_CALLBACK_DATA(i32);
pub struct PRJ_CALLBACK_DATA_FLAGS(i32);
pub struct PRJ_CANCEL_COMMAND_CB(i32);
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS(i32);
pub struct PRJ_COMPLETE_COMMAND_TYPE(i32);
pub struct PRJ_DIR_ENTRY_BUFFER_HANDLE(i32);
pub struct PRJ_END_DIRECTORY_ENUMERATION_CB(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_EXTENDED_INFO(i32);
pub struct PRJ_EXT_INFO_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_FILE_BASIC_INFO(i32);
pub struct PRJ_FILE_STATE(i32);
pub struct PRJ_GET_DIRECTORY_ENUMERATION_CB(i32);
pub struct PRJ_GET_FILE_DATA_CB(i32);
pub struct PRJ_GET_PLACEHOLDER_INFO_CB(i32);
pub struct PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT(i32);
pub struct PRJ_NOTIFICATION(i32);
pub struct PRJ_NOTIFICATION_CB(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_NOTIFICATION_MAPPING(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_NOTIFICATION_PARAMETERS(i32);
pub struct PRJ_NOTIFY_TYPES(i32);
pub struct PRJ_PLACEHOLDER_ID(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_PLACEHOLDER_INFO(i32);
pub struct PRJ_PLACEHOLDER_VERSION_INFO(i32);
pub struct PRJ_QUERY_FILE_NAME_CB(i32);
pub struct PRJ_STARTVIRTUALIZING_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_STARTVIRTUALIZING_OPTIONS(i32);
pub struct PRJ_START_DIRECTORY_ENUMERATION_CB(i32);
pub struct PRJ_UPDATE_FAILURE_CAUSES(i32);
pub struct PRJ_UPDATE_TYPES(i32);
pub struct PRJ_VIRTUALIZATION_INSTANCE_INFO(i32);
