#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_CALLBACKS {
    pub StartDirectoryEnumerationCallback: PRJ_START_DIRECTORY_ENUMERATION_CB,
    pub EndDirectoryEnumerationCallback: PRJ_END_DIRECTORY_ENUMERATION_CB,
    pub GetDirectoryEnumerationCallback: PRJ_GET_DIRECTORY_ENUMERATION_CB,
    pub GetPlaceholderInfoCallback: PRJ_GET_PLACEHOLDER_INFO_CB,
    pub GetFileDataCallback: PRJ_GET_FILE_DATA_CB,
    pub QueryFileNameCallback: PRJ_QUERY_FILE_NAME_CB,
    pub NotificationCallback: PRJ_NOTIFICATION_CB,
    pub CancelCommandCallback: PRJ_CANCEL_COMMAND_CB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_CALLBACK_DATA {
    pub Size: u32,
    pub Flags: PRJ_CALLBACK_DATA_FLAGS,
    pub NamespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
    pub CommandId: i32,
    pub FileId: ::windows_sys::core::GUID,
    pub DataStreamId: ::windows_sys::core::GUID,
    pub FilePathName: super::super::Foundation::PWSTR,
    pub VersionInfo: *mut PRJ_PLACEHOLDER_VERSION_INFO,
    pub TriggeringProcessId: u32,
    pub TriggeringProcessImageFileName: super::super::Foundation::PWSTR,
    pub InstanceContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_CALLBACK_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_CALLBACK_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PRJ_CALLBACK_DATA_FLAGS = i32;
pub const PRJ_CB_DATA_FLAG_ENUM_RESTART_SCAN: PRJ_CALLBACK_DATA_FLAGS = 1i32;
pub const PRJ_CB_DATA_FLAG_ENUM_RETURN_SINGLE_ENTRY: PRJ_CALLBACK_DATA_FLAGS = 2i32;
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_CANCEL_COMMAND_CB = unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA);
#[repr(C)]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    pub CommandType: PRJ_COMPLETE_COMMAND_TYPE,
    pub Anonymous: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0,
}
impl ::core::marker::Copy for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {}
impl ::core::clone::Clone for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    pub Notification: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1,
    pub Enumeration: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0,
}
impl ::core::marker::Copy for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {}
impl ::core::clone::Clone for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    pub DirEntryBufferHandle: PRJ_DIR_ENTRY_BUFFER_HANDLE,
}
impl ::core::marker::Copy for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {}
impl ::core::clone::Clone for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    pub NotificationMask: PRJ_NOTIFY_TYPES,
}
impl ::core::marker::Copy for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {}
impl ::core::clone::Clone for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PRJ_COMPLETE_COMMAND_TYPE = i32;
pub const PRJ_COMPLETE_COMMAND_TYPE_NOTIFICATION: PRJ_COMPLETE_COMMAND_TYPE = 1i32;
pub const PRJ_COMPLETE_COMMAND_TYPE_ENUMERATION: PRJ_COMPLETE_COMMAND_TYPE = 2i32;
pub type PRJ_DIR_ENTRY_BUFFER_HANDLE = isize;
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_END_DIRECTORY_ENUMERATION_CB = unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, enumerationid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_EXTENDED_INFO {
    pub InfoType: PRJ_EXT_INFO_TYPE,
    pub NextInfoOffset: u32,
    pub Anonymous: PRJ_EXTENDED_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_EXTENDED_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_EXTENDED_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PRJ_EXTENDED_INFO_0 {
    pub Symlink: PRJ_EXTENDED_INFO_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_EXTENDED_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_EXTENDED_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_EXTENDED_INFO_0_0 {
    pub TargetName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_EXTENDED_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_EXTENDED_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PRJ_EXT_INFO_TYPE = i32;
pub const PRJ_EXT_INFO_TYPE_SYMLINK: PRJ_EXT_INFO_TYPE = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_FILE_BASIC_INFO {
    pub IsDirectory: super::super::Foundation::BOOLEAN,
    pub FileSize: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_FILE_BASIC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_FILE_BASIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PRJ_FILE_STATE = u32;
pub const PRJ_FILE_STATE_PLACEHOLDER: PRJ_FILE_STATE = 1u32;
pub const PRJ_FILE_STATE_HYDRATED_PLACEHOLDER: PRJ_FILE_STATE = 2u32;
pub const PRJ_FILE_STATE_DIRTY_PLACEHOLDER: PRJ_FILE_STATE = 4u32;
pub const PRJ_FILE_STATE_FULL: PRJ_FILE_STATE = 8u32;
pub const PRJ_FILE_STATE_TOMBSTONE: PRJ_FILE_STATE = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_GET_DIRECTORY_ENUMERATION_CB = unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, enumerationid: *const ::windows_sys::core::GUID, searchexpression: super::super::Foundation::PWSTR, direntrybufferhandle: PRJ_DIR_ENTRY_BUFFER_HANDLE) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_GET_FILE_DATA_CB = unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, byteoffset: u64, length: u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_GET_PLACEHOLDER_INFO_CB = unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA) -> ::windows_sys::core::HRESULT;
pub type PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT = isize;
pub type PRJ_NOTIFICATION = i32;
pub const PRJ_NOTIFICATION_FILE_OPENED: PRJ_NOTIFICATION = 2i32;
pub const PRJ_NOTIFICATION_NEW_FILE_CREATED: PRJ_NOTIFICATION = 4i32;
pub const PRJ_NOTIFICATION_FILE_OVERWRITTEN: PRJ_NOTIFICATION = 8i32;
pub const PRJ_NOTIFICATION_PRE_DELETE: PRJ_NOTIFICATION = 16i32;
pub const PRJ_NOTIFICATION_PRE_RENAME: PRJ_NOTIFICATION = 32i32;
pub const PRJ_NOTIFICATION_PRE_SET_HARDLINK: PRJ_NOTIFICATION = 64i32;
pub const PRJ_NOTIFICATION_FILE_RENAMED: PRJ_NOTIFICATION = 128i32;
pub const PRJ_NOTIFICATION_HARDLINK_CREATED: PRJ_NOTIFICATION = 256i32;
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_NO_MODIFICATION: PRJ_NOTIFICATION = 512i32;
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_MODIFIED: PRJ_NOTIFICATION = 1024i32;
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_DELETED: PRJ_NOTIFICATION = 2048i32;
pub const PRJ_NOTIFICATION_FILE_PRE_CONVERT_TO_FULL: PRJ_NOTIFICATION = 4096i32;
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_NOTIFICATION_CB = unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, isdirectory: super::super::Foundation::BOOLEAN, notification: PRJ_NOTIFICATION, destinationfilename: super::super::Foundation::PWSTR, operationparameters: *mut PRJ_NOTIFICATION_PARAMETERS) -> ::windows_sys::core::HRESULT;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_NOTIFICATION_MAPPING {
    pub NotificationBitMask: PRJ_NOTIFY_TYPES,
    pub NotificationRoot: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_NOTIFICATION_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_NOTIFICATION_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PRJ_NOTIFICATION_PARAMETERS {
    pub PostCreate: PRJ_NOTIFICATION_PARAMETERS_2,
    pub FileRenamed: PRJ_NOTIFICATION_PARAMETERS_1,
    pub FileDeletedOnHandleClose: PRJ_NOTIFICATION_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_NOTIFICATION_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_NOTIFICATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_NOTIFICATION_PARAMETERS_0 {
    pub IsFileModified: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_NOTIFICATION_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_NOTIFICATION_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_NOTIFICATION_PARAMETERS_1 {
    pub NotificationMask: PRJ_NOTIFY_TYPES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_NOTIFICATION_PARAMETERS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_NOTIFICATION_PARAMETERS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_NOTIFICATION_PARAMETERS_2 {
    pub NotificationMask: PRJ_NOTIFY_TYPES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_NOTIFICATION_PARAMETERS_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_NOTIFICATION_PARAMETERS_2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PRJ_NOTIFY_TYPES = u32;
pub const PRJ_NOTIFY_NONE: PRJ_NOTIFY_TYPES = 0u32;
pub const PRJ_NOTIFY_SUPPRESS_NOTIFICATIONS: PRJ_NOTIFY_TYPES = 1u32;
pub const PRJ_NOTIFY_FILE_OPENED: PRJ_NOTIFY_TYPES = 2u32;
pub const PRJ_NOTIFY_NEW_FILE_CREATED: PRJ_NOTIFY_TYPES = 4u32;
pub const PRJ_NOTIFY_FILE_OVERWRITTEN: PRJ_NOTIFY_TYPES = 8u32;
pub const PRJ_NOTIFY_PRE_DELETE: PRJ_NOTIFY_TYPES = 16u32;
pub const PRJ_NOTIFY_PRE_RENAME: PRJ_NOTIFY_TYPES = 32u32;
pub const PRJ_NOTIFY_PRE_SET_HARDLINK: PRJ_NOTIFY_TYPES = 64u32;
pub const PRJ_NOTIFY_FILE_RENAMED: PRJ_NOTIFY_TYPES = 128u32;
pub const PRJ_NOTIFY_HARDLINK_CREATED: PRJ_NOTIFY_TYPES = 256u32;
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_NO_MODIFICATION: PRJ_NOTIFY_TYPES = 512u32;
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_MODIFIED: PRJ_NOTIFY_TYPES = 1024u32;
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_DELETED: PRJ_NOTIFY_TYPES = 2048u32;
pub const PRJ_NOTIFY_FILE_PRE_CONVERT_TO_FULL: PRJ_NOTIFY_TYPES = 4096u32;
pub const PRJ_NOTIFY_USE_EXISTING_MASK: PRJ_NOTIFY_TYPES = 4294967295u32;
pub type PRJ_PLACEHOLDER_ID = i32;
pub const PRJ_PLACEHOLDER_ID_LENGTH: PRJ_PLACEHOLDER_ID = 128i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_PLACEHOLDER_INFO {
    pub FileBasicInfo: PRJ_FILE_BASIC_INFO,
    pub EaInformation: PRJ_PLACEHOLDER_INFO_0,
    pub SecurityInformation: PRJ_PLACEHOLDER_INFO_1,
    pub StreamsInformation: PRJ_PLACEHOLDER_INFO_2,
    pub VersionInfo: PRJ_PLACEHOLDER_VERSION_INFO,
    pub VariableData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_PLACEHOLDER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_PLACEHOLDER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_PLACEHOLDER_INFO_0 {
    pub EaBufferSize: u32,
    pub OffsetToFirstEa: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_PLACEHOLDER_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_PLACEHOLDER_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_PLACEHOLDER_INFO_1 {
    pub SecurityBufferSize: u32,
    pub OffsetToSecurityDescriptor: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_PLACEHOLDER_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_PLACEHOLDER_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_PLACEHOLDER_INFO_2 {
    pub StreamsInfoBufferSize: u32,
    pub OffsetToFirstStreamInfo: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_PLACEHOLDER_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_PLACEHOLDER_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PRJ_PLACEHOLDER_VERSION_INFO {
    pub ProviderID: [u8; 128],
    pub ContentID: [u8; 128],
}
impl ::core::marker::Copy for PRJ_PLACEHOLDER_VERSION_INFO {}
impl ::core::clone::Clone for PRJ_PLACEHOLDER_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_QUERY_FILE_NAME_CB = unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA) -> ::windows_sys::core::HRESULT;
pub type PRJ_STARTVIRTUALIZING_FLAGS = u32;
pub const PRJ_FLAG_NONE: PRJ_STARTVIRTUALIZING_FLAGS = 0u32;
pub const PRJ_FLAG_USE_NEGATIVE_PATH_CACHE: PRJ_STARTVIRTUALIZING_FLAGS = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_STARTVIRTUALIZING_OPTIONS {
    pub Flags: PRJ_STARTVIRTUALIZING_FLAGS,
    pub PoolThreadCount: u32,
    pub ConcurrentThreadCount: u32,
    pub NotificationMappings: *mut PRJ_NOTIFICATION_MAPPING,
    pub NotificationMappingsCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRJ_STARTVIRTUALIZING_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRJ_STARTVIRTUALIZING_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_START_DIRECTORY_ENUMERATION_CB = unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, enumerationid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
pub type PRJ_UPDATE_FAILURE_CAUSES = u32;
pub const PRJ_UPDATE_FAILURE_CAUSE_NONE: PRJ_UPDATE_FAILURE_CAUSES = 0u32;
pub const PRJ_UPDATE_FAILURE_CAUSE_DIRTY_METADATA: PRJ_UPDATE_FAILURE_CAUSES = 1u32;
pub const PRJ_UPDATE_FAILURE_CAUSE_DIRTY_DATA: PRJ_UPDATE_FAILURE_CAUSES = 2u32;
pub const PRJ_UPDATE_FAILURE_CAUSE_TOMBSTONE: PRJ_UPDATE_FAILURE_CAUSES = 4u32;
pub const PRJ_UPDATE_FAILURE_CAUSE_READ_ONLY: PRJ_UPDATE_FAILURE_CAUSES = 8u32;
pub type PRJ_UPDATE_TYPES = u32;
pub const PRJ_UPDATE_NONE: PRJ_UPDATE_TYPES = 0u32;
pub const PRJ_UPDATE_ALLOW_DIRTY_METADATA: PRJ_UPDATE_TYPES = 1u32;
pub const PRJ_UPDATE_ALLOW_DIRTY_DATA: PRJ_UPDATE_TYPES = 2u32;
pub const PRJ_UPDATE_ALLOW_TOMBSTONE: PRJ_UPDATE_TYPES = 4u32;
pub const PRJ_UPDATE_RESERVED1: PRJ_UPDATE_TYPES = 8u32;
pub const PRJ_UPDATE_RESERVED2: PRJ_UPDATE_TYPES = 16u32;
pub const PRJ_UPDATE_ALLOW_READ_ONLY: PRJ_UPDATE_TYPES = 32u32;
pub const PRJ_UPDATE_MAX_VAL: PRJ_UPDATE_TYPES = 64u32;
#[repr(C)]
pub struct PRJ_VIRTUALIZATION_INSTANCE_INFO {
    pub InstanceID: ::windows_sys::core::GUID,
    pub WriteAlignment: u32,
}
impl ::core::marker::Copy for PRJ_VIRTUALIZATION_INSTANCE_INFO {}
impl ::core::clone::Clone for PRJ_VIRTUALIZATION_INSTANCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
