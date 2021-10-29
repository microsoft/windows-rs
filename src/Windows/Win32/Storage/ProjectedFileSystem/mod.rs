#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_CALLBACKS {
    pub StartDirectoryEnumerationCallback:
        ::std::option::Option<PRJ_START_DIRECTORY_ENUMERATION_CB>,
    pub EndDirectoryEnumerationCallback: ::std::option::Option<PRJ_END_DIRECTORY_ENUMERATION_CB>,
    pub GetDirectoryEnumerationCallback: ::std::option::Option<PRJ_GET_DIRECTORY_ENUMERATION_CB>,
    pub GetPlaceholderInfoCallback: ::std::option::Option<PRJ_GET_PLACEHOLDER_INFO_CB>,
    pub GetFileDataCallback: ::std::option::Option<PRJ_GET_FILE_DATA_CB>,
    pub QueryFileNameCallback: ::std::option::Option<PRJ_QUERY_FILE_NAME_CB>,
    pub NotificationCallback: ::std::option::Option<PRJ_NOTIFICATION_CB>,
    pub CancelCommandCallback: ::std::option::Option<PRJ_CANCEL_COMMAND_CB>,
}
#[cfg(feature = "Win32_Foundation")]
impl PRJ_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PRJ_CALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PRJ_CALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRJ_CALLBACKS").finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PRJ_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.StartDirectoryEnumerationCallback.map(|f| f as usize)
            == other.StartDirectoryEnumerationCallback.map(|f| f as usize)
            && self.EndDirectoryEnumerationCallback.map(|f| f as usize)
                == other.EndDirectoryEnumerationCallback.map(|f| f as usize)
            && self.GetDirectoryEnumerationCallback.map(|f| f as usize)
                == other.GetDirectoryEnumerationCallback.map(|f| f as usize)
            && self.GetPlaceholderInfoCallback.map(|f| f as usize)
                == other.GetPlaceholderInfoCallback.map(|f| f as usize)
            && self.GetFileDataCallback.map(|f| f as usize)
                == other.GetFileDataCallback.map(|f| f as usize)
            && self.QueryFileNameCallback.map(|f| f as usize)
                == other.QueryFileNameCallback.map(|f| f as usize)
            && self.NotificationCallback.map(|f| f as usize)
                == other.NotificationCallback.map(|f| f as usize)
            && self.CancelCommandCallback.map(|f| f as usize)
                == other.CancelCommandCallback.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PRJ_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PRJ_CALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_CALLBACK_DATA {
    pub Size: u32,
    pub Flags: PRJ_CALLBACK_DATA_FLAGS,
    pub NamespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
    pub CommandId: i32,
    pub FileId: ::windows::runtime::GUID,
    pub DataStreamId: ::windows::runtime::GUID,
    pub FilePathName: super::super::Foundation::PWSTR,
    pub VersionInfo: *mut PRJ_PLACEHOLDER_VERSION_INFO,
    pub TriggeringProcessId: u32,
    pub TriggeringProcessImageFileName: super::super::Foundation::PWSTR,
    pub InstanceContext: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl PRJ_CALLBACK_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PRJ_CALLBACK_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PRJ_CALLBACK_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRJ_CALLBACK_DATA")
            .field("Size", &self.Size)
            .field("Flags", &self.Flags)
            .field(
                "NamespaceVirtualizationContext",
                &self.NamespaceVirtualizationContext,
            )
            .field("CommandId", &self.CommandId)
            .field("FileId", &self.FileId)
            .field("DataStreamId", &self.DataStreamId)
            .field("FilePathName", &self.FilePathName)
            .field("VersionInfo", &self.VersionInfo)
            .field("TriggeringProcessId", &self.TriggeringProcessId)
            .field(
                "TriggeringProcessImageFileName",
                &self.TriggeringProcessImageFileName,
            )
            .field("InstanceContext", &self.InstanceContext)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PRJ_CALLBACK_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Flags == other.Flags
            && self.NamespaceVirtualizationContext == other.NamespaceVirtualizationContext
            && self.CommandId == other.CommandId
            && self.FileId == other.FileId
            && self.DataStreamId == other.DataStreamId
            && self.FilePathName == other.FilePathName
            && self.VersionInfo == other.VersionInfo
            && self.TriggeringProcessId == other.TriggeringProcessId
            && self.TriggeringProcessImageFileName == other.TriggeringProcessImageFileName
            && self.InstanceContext == other.InstanceContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PRJ_CALLBACK_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PRJ_CALLBACK_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PRJ_CALLBACK_DATA_FLAGS(pub i32);
pub const PRJ_CB_DATA_FLAG_ENUM_RESTART_SCAN: PRJ_CALLBACK_DATA_FLAGS =
    PRJ_CALLBACK_DATA_FLAGS(1i32);
pub const PRJ_CB_DATA_FLAG_ENUM_RETURN_SINGLE_ENTRY: PRJ_CALLBACK_DATA_FLAGS =
    PRJ_CALLBACK_DATA_FLAGS(2i32);
impl ::std::convert::From<i32> for PRJ_CALLBACK_DATA_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRJ_CALLBACK_DATA_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_CANCEL_COMMAND_CB = unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    pub CommandType: PRJ_COMPLETE_COMMAND_TYPE,
    pub Anonymous: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0,
}
impl PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {}
impl ::std::default::Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    pub Notification: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1,
    pub Enumeration: PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0,
}
impl PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {}
impl ::std::default::Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {}
unsafe impl ::windows::runtime::Abi for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    pub DirEntryBufferHandle: PRJ_DIR_ENTRY_BUFFER_HANDLE,
}
impl PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {}
impl ::std::default::Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Enumeration_e__Struct")
            .field("DirEntryBufferHandle", &self.DirEntryBufferHandle)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.DirEntryBufferHandle == other.DirEntryBufferHandle
    }
}
impl ::std::cmp::Eq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {}
unsafe impl ::windows::runtime::Abi for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    pub NotificationMask: PRJ_NOTIFY_TYPES,
}
impl PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {}
impl ::std::default::Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Notification_e__Struct")
            .field("NotificationMask", &self.NotificationMask)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.NotificationMask == other.NotificationMask
    }
}
impl ::std::cmp::Eq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {}
unsafe impl ::windows::runtime::Abi for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PRJ_COMPLETE_COMMAND_TYPE(pub i32);
pub const PRJ_COMPLETE_COMMAND_TYPE_NOTIFICATION: PRJ_COMPLETE_COMMAND_TYPE =
    PRJ_COMPLETE_COMMAND_TYPE(1i32);
pub const PRJ_COMPLETE_COMMAND_TYPE_ENUMERATION: PRJ_COMPLETE_COMMAND_TYPE =
    PRJ_COMPLETE_COMMAND_TYPE(2i32);
impl ::std::convert::From<i32> for PRJ_COMPLETE_COMMAND_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRJ_COMPLETE_COMMAND_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct PRJ_DIR_ENTRY_BUFFER_HANDLE(pub isize);
impl ::std::default::Default for PRJ_DIR_ENTRY_BUFFER_HANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for PRJ_DIR_ENTRY_BUFFER_HANDLE {}
unsafe impl ::windows::runtime::Abi for PRJ_DIR_ENTRY_BUFFER_HANDLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_END_DIRECTORY_ENUMERATION_CB =
    unsafe extern "system" fn(
        callbackdata: *const PRJ_CALLBACK_DATA,
        enumerationid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_EXTENDED_INFO {
    pub InfoType: PRJ_EXT_INFO_TYPE,
    pub NextInfoOffset: u32,
    pub Anonymous: PRJ_EXTENDED_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl PRJ_EXTENDED_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PRJ_EXTENDED_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PRJ_EXTENDED_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PRJ_EXTENDED_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PRJ_EXTENDED_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PRJ_EXTENDED_INFO_0 {
    pub Symlink: PRJ_EXTENDED_INFO_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl PRJ_EXTENDED_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PRJ_EXTENDED_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PRJ_EXTENDED_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PRJ_EXTENDED_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PRJ_EXTENDED_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_EXTENDED_INFO_0_0 {
    pub TargetName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PRJ_EXTENDED_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PRJ_EXTENDED_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PRJ_EXTENDED_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Symlink_e__Struct")
            .field("TargetName", &self.TargetName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PRJ_EXTENDED_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.TargetName == other.TargetName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PRJ_EXTENDED_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PRJ_EXTENDED_INFO_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PRJ_EXT_INFO_TYPE(pub i32);
pub const PRJ_EXT_INFO_TYPE_SYMLINK: PRJ_EXT_INFO_TYPE = PRJ_EXT_INFO_TYPE(1i32);
impl ::std::convert::From<i32> for PRJ_EXT_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRJ_EXT_INFO_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl PRJ_FILE_BASIC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PRJ_FILE_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PRJ_FILE_BASIC_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRJ_FILE_BASIC_INFO")
            .field("IsDirectory", &self.IsDirectory)
            .field("FileSize", &self.FileSize)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("FileAttributes", &self.FileAttributes)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PRJ_FILE_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.IsDirectory == other.IsDirectory
            && self.FileSize == other.FileSize
            && self.CreationTime == other.CreationTime
            && self.LastAccessTime == other.LastAccessTime
            && self.LastWriteTime == other.LastWriteTime
            && self.ChangeTime == other.ChangeTime
            && self.FileAttributes == other.FileAttributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PRJ_FILE_BASIC_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PRJ_FILE_BASIC_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PRJ_FILE_STATE(pub u32);
pub const PRJ_FILE_STATE_PLACEHOLDER: PRJ_FILE_STATE = PRJ_FILE_STATE(1u32);
pub const PRJ_FILE_STATE_HYDRATED_PLACEHOLDER: PRJ_FILE_STATE = PRJ_FILE_STATE(2u32);
pub const PRJ_FILE_STATE_DIRTY_PLACEHOLDER: PRJ_FILE_STATE = PRJ_FILE_STATE(4u32);
pub const PRJ_FILE_STATE_FULL: PRJ_FILE_STATE = PRJ_FILE_STATE(8u32);
pub const PRJ_FILE_STATE_TOMBSTONE: PRJ_FILE_STATE = PRJ_FILE_STATE(16u32);
impl ::std::convert::From<u32> for PRJ_FILE_STATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRJ_FILE_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PRJ_FILE_STATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PRJ_FILE_STATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PRJ_FILE_STATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PRJ_FILE_STATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PRJ_FILE_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_GET_DIRECTORY_ENUMERATION_CB =
    unsafe extern "system" fn(
        callbackdata: *const PRJ_CALLBACK_DATA,
        enumerationid: *const ::windows::runtime::GUID,
        searchexpression: super::super::Foundation::PWSTR,
        direntrybufferhandle: PRJ_DIR_ENTRY_BUFFER_HANDLE,
    ) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_GET_FILE_DATA_CB = unsafe extern "system" fn(
    callbackdata: *const PRJ_CALLBACK_DATA,
    byteoffset: u64,
    length: u32,
) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_GET_PLACEHOLDER_INFO_CB = unsafe extern "system" fn(
    callbackdata: *const PRJ_CALLBACK_DATA,
) -> ::windows::runtime::HRESULT;
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT(pub isize);
impl ::std::default::Default for PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {}
unsafe impl ::windows::runtime::Abi for PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PRJ_NOTIFICATION(pub i32);
pub const PRJ_NOTIFICATION_FILE_OPENED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(2i32);
pub const PRJ_NOTIFICATION_NEW_FILE_CREATED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(4i32);
pub const PRJ_NOTIFICATION_FILE_OVERWRITTEN: PRJ_NOTIFICATION = PRJ_NOTIFICATION(8i32);
pub const PRJ_NOTIFICATION_PRE_DELETE: PRJ_NOTIFICATION = PRJ_NOTIFICATION(16i32);
pub const PRJ_NOTIFICATION_PRE_RENAME: PRJ_NOTIFICATION = PRJ_NOTIFICATION(32i32);
pub const PRJ_NOTIFICATION_PRE_SET_HARDLINK: PRJ_NOTIFICATION = PRJ_NOTIFICATION(64i32);
pub const PRJ_NOTIFICATION_FILE_RENAMED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(128i32);
pub const PRJ_NOTIFICATION_HARDLINK_CREATED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(256i32);
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_NO_MODIFICATION: PRJ_NOTIFICATION =
    PRJ_NOTIFICATION(512i32);
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_MODIFIED: PRJ_NOTIFICATION =
    PRJ_NOTIFICATION(1024i32);
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_DELETED: PRJ_NOTIFICATION =
    PRJ_NOTIFICATION(2048i32);
pub const PRJ_NOTIFICATION_FILE_PRE_CONVERT_TO_FULL: PRJ_NOTIFICATION = PRJ_NOTIFICATION(4096i32);
impl ::std::convert::From<i32> for PRJ_NOTIFICATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRJ_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_NOTIFICATION_CB = unsafe extern "system" fn(
    callbackdata: *const PRJ_CALLBACK_DATA,
    isdirectory: super::super::Foundation::BOOLEAN,
    notification: PRJ_NOTIFICATION,
    destinationfilename: super::super::Foundation::PWSTR,
    operationparameters: *mut PRJ_NOTIFICATION_PARAMETERS,
) -> ::windows::runtime::HRESULT;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_NOTIFICATION_MAPPING {
    pub NotificationBitMask: PRJ_NOTIFY_TYPES,
    pub NotificationRoot: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PRJ_NOTIFICATION_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PRJ_NOTIFICATION_MAPPING {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PRJ_NOTIFICATION_MAPPING {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRJ_NOTIFICATION_MAPPING")
            .field("NotificationBitMask", &self.NotificationBitMask)
            .field("NotificationRoot", &self.NotificationRoot)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PRJ_NOTIFICATION_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.NotificationBitMask == other.NotificationBitMask
            && self.NotificationRoot == other.NotificationRoot
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PRJ_NOTIFICATION_MAPPING {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PRJ_NOTIFICATION_MAPPING {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PRJ_NOTIFICATION_PARAMETERS {
    pub PostCreate: PRJ_NOTIFICATION_PARAMETERS_2,
    pub FileRenamed: PRJ_NOTIFICATION_PARAMETERS_1,
    pub FileDeletedOnHandleClose: PRJ_NOTIFICATION_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl PRJ_NOTIFICATION_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PRJ_NOTIFICATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PRJ_NOTIFICATION_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PRJ_NOTIFICATION_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PRJ_NOTIFICATION_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRJ_NOTIFICATION_PARAMETERS_0 {
    pub IsFileModified: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl PRJ_NOTIFICATION_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PRJ_NOTIFICATION_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PRJ_NOTIFICATION_PARAMETERS_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FileDeletedOnHandleClose_e__Struct")
            .field("IsFileModified", &self.IsFileModified)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PRJ_NOTIFICATION_PARAMETERS_0 {
    fn eq(&self, other: &Self) -> bool {
        self.IsFileModified == other.IsFileModified
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PRJ_NOTIFICATION_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PRJ_NOTIFICATION_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PRJ_NOTIFICATION_PARAMETERS_1 {
    pub NotificationMask: PRJ_NOTIFY_TYPES,
}
impl PRJ_NOTIFICATION_PARAMETERS_1 {}
impl ::std::default::Default for PRJ_NOTIFICATION_PARAMETERS_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PRJ_NOTIFICATION_PARAMETERS_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FileRenamed_e__Struct")
            .field("NotificationMask", &self.NotificationMask)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PRJ_NOTIFICATION_PARAMETERS_1 {
    fn eq(&self, other: &Self) -> bool {
        self.NotificationMask == other.NotificationMask
    }
}
impl ::std::cmp::Eq for PRJ_NOTIFICATION_PARAMETERS_1 {}
unsafe impl ::windows::runtime::Abi for PRJ_NOTIFICATION_PARAMETERS_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PRJ_NOTIFICATION_PARAMETERS_2 {
    pub NotificationMask: PRJ_NOTIFY_TYPES,
}
impl PRJ_NOTIFICATION_PARAMETERS_2 {}
impl ::std::default::Default for PRJ_NOTIFICATION_PARAMETERS_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PRJ_NOTIFICATION_PARAMETERS_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_PostCreate_e__Struct")
            .field("NotificationMask", &self.NotificationMask)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PRJ_NOTIFICATION_PARAMETERS_2 {
    fn eq(&self, other: &Self) -> bool {
        self.NotificationMask == other.NotificationMask
    }
}
impl ::std::cmp::Eq for PRJ_NOTIFICATION_PARAMETERS_2 {}
unsafe impl ::windows::runtime::Abi for PRJ_NOTIFICATION_PARAMETERS_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PRJ_NOTIFY_TYPES(pub u32);
pub const PRJ_NOTIFY_NONE: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(0u32);
pub const PRJ_NOTIFY_SUPPRESS_NOTIFICATIONS: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(1u32);
pub const PRJ_NOTIFY_FILE_OPENED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(2u32);
pub const PRJ_NOTIFY_NEW_FILE_CREATED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(4u32);
pub const PRJ_NOTIFY_FILE_OVERWRITTEN: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(8u32);
pub const PRJ_NOTIFY_PRE_DELETE: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(16u32);
pub const PRJ_NOTIFY_PRE_RENAME: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(32u32);
pub const PRJ_NOTIFY_PRE_SET_HARDLINK: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(64u32);
pub const PRJ_NOTIFY_FILE_RENAMED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(128u32);
pub const PRJ_NOTIFY_HARDLINK_CREATED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(256u32);
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_NO_MODIFICATION: PRJ_NOTIFY_TYPES =
    PRJ_NOTIFY_TYPES(512u32);
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_MODIFIED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(1024u32);
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_DELETED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(2048u32);
pub const PRJ_NOTIFY_FILE_PRE_CONVERT_TO_FULL: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(4096u32);
pub const PRJ_NOTIFY_USE_EXISTING_MASK: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(4294967295u32);
impl ::std::convert::From<u32> for PRJ_NOTIFY_TYPES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRJ_NOTIFY_TYPES {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PRJ_NOTIFY_TYPES {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PRJ_NOTIFY_TYPES {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PRJ_NOTIFY_TYPES {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PRJ_NOTIFY_TYPES {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PRJ_NOTIFY_TYPES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PRJ_PLACEHOLDER_ID(pub i32);
pub const PRJ_PLACEHOLDER_ID_LENGTH: PRJ_PLACEHOLDER_ID = PRJ_PLACEHOLDER_ID(128i32);
impl ::std::convert::From<i32> for PRJ_PLACEHOLDER_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRJ_PLACEHOLDER_ID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl PRJ_PLACEHOLDER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PRJ_PLACEHOLDER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PRJ_PLACEHOLDER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRJ_PLACEHOLDER_INFO")
            .field("FileBasicInfo", &self.FileBasicInfo)
            .field("EaInformation", &self.EaInformation)
            .field("SecurityInformation", &self.SecurityInformation)
            .field("StreamsInformation", &self.StreamsInformation)
            .field("VersionInfo", &self.VersionInfo)
            .field("VariableData", &self.VariableData)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PRJ_PLACEHOLDER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileBasicInfo == other.FileBasicInfo
            && self.EaInformation == other.EaInformation
            && self.SecurityInformation == other.SecurityInformation
            && self.StreamsInformation == other.StreamsInformation
            && self.VersionInfo == other.VersionInfo
            && self.VariableData == other.VariableData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PRJ_PLACEHOLDER_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PRJ_PLACEHOLDER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PRJ_PLACEHOLDER_INFO_0 {
    pub EaBufferSize: u32,
    pub OffsetToFirstEa: u32,
}
impl PRJ_PLACEHOLDER_INFO_0 {}
impl ::std::default::Default for PRJ_PLACEHOLDER_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PRJ_PLACEHOLDER_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_EaInformation_e__Struct")
            .field("EaBufferSize", &self.EaBufferSize)
            .field("OffsetToFirstEa", &self.OffsetToFirstEa)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PRJ_PLACEHOLDER_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.EaBufferSize == other.EaBufferSize && self.OffsetToFirstEa == other.OffsetToFirstEa
    }
}
impl ::std::cmp::Eq for PRJ_PLACEHOLDER_INFO_0 {}
unsafe impl ::windows::runtime::Abi for PRJ_PLACEHOLDER_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PRJ_PLACEHOLDER_INFO_1 {
    pub SecurityBufferSize: u32,
    pub OffsetToSecurityDescriptor: u32,
}
impl PRJ_PLACEHOLDER_INFO_1 {}
impl ::std::default::Default for PRJ_PLACEHOLDER_INFO_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PRJ_PLACEHOLDER_INFO_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_SecurityInformation_e__Struct")
            .field("SecurityBufferSize", &self.SecurityBufferSize)
            .field(
                "OffsetToSecurityDescriptor",
                &self.OffsetToSecurityDescriptor,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for PRJ_PLACEHOLDER_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityBufferSize == other.SecurityBufferSize
            && self.OffsetToSecurityDescriptor == other.OffsetToSecurityDescriptor
    }
}
impl ::std::cmp::Eq for PRJ_PLACEHOLDER_INFO_1 {}
unsafe impl ::windows::runtime::Abi for PRJ_PLACEHOLDER_INFO_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PRJ_PLACEHOLDER_INFO_2 {
    pub StreamsInfoBufferSize: u32,
    pub OffsetToFirstStreamInfo: u32,
}
impl PRJ_PLACEHOLDER_INFO_2 {}
impl ::std::default::Default for PRJ_PLACEHOLDER_INFO_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PRJ_PLACEHOLDER_INFO_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_StreamsInformation_e__Struct")
            .field("StreamsInfoBufferSize", &self.StreamsInfoBufferSize)
            .field("OffsetToFirstStreamInfo", &self.OffsetToFirstStreamInfo)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PRJ_PLACEHOLDER_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.StreamsInfoBufferSize == other.StreamsInfoBufferSize
            && self.OffsetToFirstStreamInfo == other.OffsetToFirstStreamInfo
    }
}
impl ::std::cmp::Eq for PRJ_PLACEHOLDER_INFO_2 {}
unsafe impl ::windows::runtime::Abi for PRJ_PLACEHOLDER_INFO_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PRJ_PLACEHOLDER_VERSION_INFO {
    pub ProviderID: [u8; 128],
    pub ContentID: [u8; 128],
}
impl PRJ_PLACEHOLDER_VERSION_INFO {}
impl ::std::default::Default for PRJ_PLACEHOLDER_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PRJ_PLACEHOLDER_VERSION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRJ_PLACEHOLDER_VERSION_INFO")
            .field("ProviderID", &self.ProviderID)
            .field("ContentID", &self.ContentID)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PRJ_PLACEHOLDER_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProviderID == other.ProviderID && self.ContentID == other.ContentID
    }
}
impl ::std::cmp::Eq for PRJ_PLACEHOLDER_VERSION_INFO {}
unsafe impl ::windows::runtime::Abi for PRJ_PLACEHOLDER_VERSION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_QUERY_FILE_NAME_CB = unsafe extern "system" fn(
    callbackdata: *const PRJ_CALLBACK_DATA,
) -> ::windows::runtime::HRESULT;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PRJ_STARTVIRTUALIZING_FLAGS(pub u32);
pub const PRJ_FLAG_NONE: PRJ_STARTVIRTUALIZING_FLAGS = PRJ_STARTVIRTUALIZING_FLAGS(0u32);
pub const PRJ_FLAG_USE_NEGATIVE_PATH_CACHE: PRJ_STARTVIRTUALIZING_FLAGS =
    PRJ_STARTVIRTUALIZING_FLAGS(1u32);
impl ::std::convert::From<u32> for PRJ_STARTVIRTUALIZING_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRJ_STARTVIRTUALIZING_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PRJ_STARTVIRTUALIZING_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PRJ_STARTVIRTUALIZING_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PRJ_STARTVIRTUALIZING_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PRJ_STARTVIRTUALIZING_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PRJ_STARTVIRTUALIZING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl PRJ_STARTVIRTUALIZING_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PRJ_STARTVIRTUALIZING_OPTIONS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PRJ_STARTVIRTUALIZING_OPTIONS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRJ_STARTVIRTUALIZING_OPTIONS")
            .field("Flags", &self.Flags)
            .field("PoolThreadCount", &self.PoolThreadCount)
            .field("ConcurrentThreadCount", &self.ConcurrentThreadCount)
            .field("NotificationMappings", &self.NotificationMappings)
            .field("NotificationMappingsCount", &self.NotificationMappingsCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PRJ_STARTVIRTUALIZING_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.PoolThreadCount == other.PoolThreadCount
            && self.ConcurrentThreadCount == other.ConcurrentThreadCount
            && self.NotificationMappings == other.NotificationMappings
            && self.NotificationMappingsCount == other.NotificationMappingsCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PRJ_STARTVIRTUALIZING_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PRJ_STARTVIRTUALIZING_OPTIONS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_START_DIRECTORY_ENUMERATION_CB =
    unsafe extern "system" fn(
        callbackdata: *const PRJ_CALLBACK_DATA,
        enumerationid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PRJ_UPDATE_FAILURE_CAUSES(pub u32);
pub const PRJ_UPDATE_FAILURE_CAUSE_NONE: PRJ_UPDATE_FAILURE_CAUSES =
    PRJ_UPDATE_FAILURE_CAUSES(0u32);
pub const PRJ_UPDATE_FAILURE_CAUSE_DIRTY_METADATA: PRJ_UPDATE_FAILURE_CAUSES =
    PRJ_UPDATE_FAILURE_CAUSES(1u32);
pub const PRJ_UPDATE_FAILURE_CAUSE_DIRTY_DATA: PRJ_UPDATE_FAILURE_CAUSES =
    PRJ_UPDATE_FAILURE_CAUSES(2u32);
pub const PRJ_UPDATE_FAILURE_CAUSE_TOMBSTONE: PRJ_UPDATE_FAILURE_CAUSES =
    PRJ_UPDATE_FAILURE_CAUSES(4u32);
pub const PRJ_UPDATE_FAILURE_CAUSE_READ_ONLY: PRJ_UPDATE_FAILURE_CAUSES =
    PRJ_UPDATE_FAILURE_CAUSES(8u32);
impl ::std::convert::From<u32> for PRJ_UPDATE_FAILURE_CAUSES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRJ_UPDATE_FAILURE_CAUSES {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PRJ_UPDATE_FAILURE_CAUSES {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PRJ_UPDATE_FAILURE_CAUSES {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PRJ_UPDATE_FAILURE_CAUSES {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PRJ_UPDATE_FAILURE_CAUSES {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PRJ_UPDATE_FAILURE_CAUSES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PRJ_UPDATE_TYPES(pub u32);
pub const PRJ_UPDATE_NONE: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(0u32);
pub const PRJ_UPDATE_ALLOW_DIRTY_METADATA: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(1u32);
pub const PRJ_UPDATE_ALLOW_DIRTY_DATA: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(2u32);
pub const PRJ_UPDATE_ALLOW_TOMBSTONE: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(4u32);
pub const PRJ_UPDATE_RESERVED1: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(8u32);
pub const PRJ_UPDATE_RESERVED2: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(16u32);
pub const PRJ_UPDATE_ALLOW_READ_ONLY: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(32u32);
pub const PRJ_UPDATE_MAX_VAL: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(64u32);
impl ::std::convert::From<u32> for PRJ_UPDATE_TYPES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRJ_UPDATE_TYPES {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PRJ_UPDATE_TYPES {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PRJ_UPDATE_TYPES {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PRJ_UPDATE_TYPES {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PRJ_UPDATE_TYPES {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PRJ_UPDATE_TYPES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PRJ_VIRTUALIZATION_INSTANCE_INFO {
    pub InstanceID: ::windows::runtime::GUID,
    pub WriteAlignment: u32,
}
impl PRJ_VIRTUALIZATION_INSTANCE_INFO {}
impl ::std::default::Default for PRJ_VIRTUALIZATION_INSTANCE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PRJ_VIRTUALIZATION_INSTANCE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRJ_VIRTUALIZATION_INSTANCE_INFO")
            .field("InstanceID", &self.InstanceID)
            .field("WriteAlignment", &self.WriteAlignment)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PRJ_VIRTUALIZATION_INSTANCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.InstanceID == other.InstanceID && self.WriteAlignment == other.WriteAlignment
    }
}
impl ::std::cmp::Eq for PRJ_VIRTUALIZATION_INSTANCE_INFO {}
unsafe impl ::windows::runtime::Abi for PRJ_VIRTUALIZATION_INSTANCE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn PrjAllocateAlignedBuffer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
>(
    namespacevirtualizationcontext: Param0,
    size: usize,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjAllocateAlignedBuffer(
                namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
                size: usize,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(PrjAllocateAlignedBuffer(
            namespacevirtualizationcontext.into_param().abi(),
            ::std::mem::transmute(size),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PrjClearNegativePathCache<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
>(
    namespacevirtualizationcontext: Param0,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjClearNegativePathCache(
                namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
                totalentrynumber: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PrjClearNegativePathCache(
            namespacevirtualizationcontext.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PrjCompleteCommand<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
>(
    namespacevirtualizationcontext: Param0,
    commandid: i32,
    completionresult: ::windows::runtime::HRESULT,
    extendedparameters: *const PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjCompleteCommand(
                namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
                commandid: i32,
                completionresult: ::windows::runtime::HRESULT,
                extendedparameters: *const PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS,
            ) -> ::windows::runtime::HRESULT;
        }
        PrjCompleteCommand(
            namespacevirtualizationcontext.into_param().abi(),
            ::std::mem::transmute(commandid),
            ::std::mem::transmute(completionresult),
            ::std::mem::transmute(extendedparameters),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjDeleteFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    namespacevirtualizationcontext: Param0,
    destinationfilename: Param1,
    updateflags: PRJ_UPDATE_TYPES,
) -> ::windows::runtime::Result<PRJ_UPDATE_FAILURE_CAUSES> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjDeleteFile(
                namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
                destinationfilename: super::super::Foundation::PWSTR,
                updateflags: PRJ_UPDATE_TYPES,
                failurereason: *mut PRJ_UPDATE_FAILURE_CAUSES,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <PRJ_UPDATE_FAILURE_CAUSES as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        PrjDeleteFile(
            namespacevirtualizationcontext.into_param().abi(),
            destinationfilename.into_param().abi(),
            ::std::mem::transmute(updateflags),
            &mut result__,
        )
        .from_abi::<PRJ_UPDATE_FAILURE_CAUSES>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjDoesNameContainWildCards<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    filename: Param0,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjDoesNameContainWildCards(
                filename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(PrjDoesNameContainWildCards(filename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjFileNameCompare<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    filename1: Param0,
    filename2: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjFileNameCompare(
                filename1: super::super::Foundation::PWSTR,
                filename2: super::super::Foundation::PWSTR,
            ) -> i32;
        }
        ::std::mem::transmute(PrjFileNameCompare(
            filename1.into_param().abi(),
            filename2.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjFileNameMatch<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    filenametocheck: Param0,
    pattern: Param1,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjFileNameMatch(
                filenametocheck: super::super::Foundation::PWSTR,
                pattern: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(PrjFileNameMatch(
            filenametocheck.into_param().abi(),
            pattern.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjFillDirEntryBuffer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, PRJ_DIR_ENTRY_BUFFER_HANDLE>,
>(
    filename: Param0,
    filebasicinfo: *const PRJ_FILE_BASIC_INFO,
    direntrybufferhandle: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjFillDirEntryBuffer(
                filename: super::super::Foundation::PWSTR,
                filebasicinfo: *const PRJ_FILE_BASIC_INFO,
                direntrybufferhandle: PRJ_DIR_ENTRY_BUFFER_HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        PrjFillDirEntryBuffer(
            filename.into_param().abi(),
            ::std::mem::transmute(filebasicinfo),
            direntrybufferhandle.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjFillDirEntryBuffer2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PRJ_DIR_ENTRY_BUFFER_HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    direntrybufferhandle: Param0,
    filename: Param1,
    filebasicinfo: *const PRJ_FILE_BASIC_INFO,
    extendedinfo: *const PRJ_EXTENDED_INFO,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjFillDirEntryBuffer2(
                direntrybufferhandle: PRJ_DIR_ENTRY_BUFFER_HANDLE,
                filename: super::super::Foundation::PWSTR,
                filebasicinfo: *const PRJ_FILE_BASIC_INFO,
                extendedinfo: *const PRJ_EXTENDED_INFO,
            ) -> ::windows::runtime::HRESULT;
        }
        PrjFillDirEntryBuffer2(
            direntrybufferhandle.into_param().abi(),
            filename.into_param().abi(),
            ::std::mem::transmute(filebasicinfo),
            ::std::mem::transmute(extendedinfo),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PrjFreeAlignedBuffer(buffer: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjFreeAlignedBuffer(buffer: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(PrjFreeAlignedBuffer(::std::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjGetOnDiskFileState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    destinationfilename: Param0,
) -> ::windows::runtime::Result<PRJ_FILE_STATE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjGetOnDiskFileState(
                destinationfilename: super::super::Foundation::PWSTR,
                filestate: *mut PRJ_FILE_STATE,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <PRJ_FILE_STATE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PrjGetOnDiskFileState(destinationfilename.into_param().abi(), &mut result__)
            .from_abi::<PRJ_FILE_STATE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PrjGetVirtualizationInstanceInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
>(
    namespacevirtualizationcontext: Param0,
) -> ::windows::runtime::Result<PRJ_VIRTUALIZATION_INSTANCE_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjGetVirtualizationInstanceInfo(
                namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
                virtualizationinstanceinfo: *mut PRJ_VIRTUALIZATION_INSTANCE_INFO,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <PRJ_VIRTUALIZATION_INSTANCE_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        PrjGetVirtualizationInstanceInfo(
            namespacevirtualizationcontext.into_param().abi(),
            &mut result__,
        )
        .from_abi::<PRJ_VIRTUALIZATION_INSTANCE_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjMarkDirectoryAsPlaceholder<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    rootpathname: Param0,
    targetpathname: Param1,
    versioninfo: *const PRJ_PLACEHOLDER_VERSION_INFO,
    virtualizationinstanceid: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjMarkDirectoryAsPlaceholder(
                rootpathname: super::super::Foundation::PWSTR,
                targetpathname: super::super::Foundation::PWSTR,
                versioninfo: *const PRJ_PLACEHOLDER_VERSION_INFO,
                virtualizationinstanceid: *const ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        PrjMarkDirectoryAsPlaceholder(
            rootpathname.into_param().abi(),
            targetpathname.into_param().abi(),
            ::std::mem::transmute(versioninfo),
            ::std::mem::transmute(virtualizationinstanceid),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjStartVirtualizing<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    virtualizationrootpath: Param0,
    callbacks: *const PRJ_CALLBACKS,
    instancecontext: *const ::std::ffi::c_void,
    options: *const PRJ_STARTVIRTUALIZING_OPTIONS,
) -> ::windows::runtime::Result<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjStartVirtualizing(
                virtualizationrootpath: super::super::Foundation::PWSTR,
                callbacks: *const ::std::mem::ManuallyDrop<PRJ_CALLBACKS>,
                instancecontext: *const ::std::ffi::c_void,
                options: *const PRJ_STARTVIRTUALIZING_OPTIONS,
                namespacevirtualizationcontext: *mut PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        PrjStartVirtualizing(
            virtualizationrootpath.into_param().abi(),
            ::std::mem::transmute(callbacks),
            ::std::mem::transmute(instancecontext),
            ::std::mem::transmute(options),
            &mut result__,
        )
        .from_abi::<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PrjStopVirtualizing<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
>(
    namespacevirtualizationcontext: Param0,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjStopVirtualizing(
                namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
            );
        }
        ::std::mem::transmute(PrjStopVirtualizing(
            namespacevirtualizationcontext.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjUpdateFileIfNeeded<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    namespacevirtualizationcontext: Param0,
    destinationfilename: Param1,
    placeholderinfo: *const PRJ_PLACEHOLDER_INFO,
    placeholderinfosize: u32,
    updateflags: PRJ_UPDATE_TYPES,
) -> ::windows::runtime::Result<PRJ_UPDATE_FAILURE_CAUSES> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjUpdateFileIfNeeded(
                namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
                destinationfilename: super::super::Foundation::PWSTR,
                placeholderinfo: *const PRJ_PLACEHOLDER_INFO,
                placeholderinfosize: u32,
                updateflags: PRJ_UPDATE_TYPES,
                failurereason: *mut PRJ_UPDATE_FAILURE_CAUSES,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <PRJ_UPDATE_FAILURE_CAUSES as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        PrjUpdateFileIfNeeded(
            namespacevirtualizationcontext.into_param().abi(),
            destinationfilename.into_param().abi(),
            ::std::mem::transmute(placeholderinfo),
            ::std::mem::transmute(placeholderinfosize),
            ::std::mem::transmute(updateflags),
            &mut result__,
        )
        .from_abi::<PRJ_UPDATE_FAILURE_CAUSES>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PrjWriteFileData<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
>(
    namespacevirtualizationcontext: Param0,
    datastreamid: *const ::windows::runtime::GUID,
    buffer: *const ::std::ffi::c_void,
    byteoffset: u64,
    length: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjWriteFileData(
                namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
                datastreamid: *const ::windows::runtime::GUID,
                buffer: *const ::std::ffi::c_void,
                byteoffset: u64,
                length: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        PrjWriteFileData(
            namespacevirtualizationcontext.into_param().abi(),
            ::std::mem::transmute(datastreamid),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(byteoffset),
            ::std::mem::transmute(length),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjWritePlaceholderInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    namespacevirtualizationcontext: Param0,
    destinationfilename: Param1,
    placeholderinfo: *const PRJ_PLACEHOLDER_INFO,
    placeholderinfosize: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjWritePlaceholderInfo(
                namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
                destinationfilename: super::super::Foundation::PWSTR,
                placeholderinfo: *const PRJ_PLACEHOLDER_INFO,
                placeholderinfosize: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        PrjWritePlaceholderInfo(
            namespacevirtualizationcontext.into_param().abi(),
            destinationfilename.into_param().abi(),
            ::std::mem::transmute(placeholderinfo),
            ::std::mem::transmute(placeholderinfosize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjWritePlaceholderInfo2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    namespacevirtualizationcontext: Param0,
    destinationfilename: Param1,
    placeholderinfo: *const PRJ_PLACEHOLDER_INFO,
    placeholderinfosize: u32,
    extendedinfo: *const PRJ_EXTENDED_INFO,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrjWritePlaceholderInfo2(
                namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
                destinationfilename: super::super::Foundation::PWSTR,
                placeholderinfo: *const PRJ_PLACEHOLDER_INFO,
                placeholderinfosize: u32,
                extendedinfo: *const PRJ_EXTENDED_INFO,
            ) -> ::windows::runtime::HRESULT;
        }
        PrjWritePlaceholderInfo2(
            namespacevirtualizationcontext.into_param().abi(),
            destinationfilename.into_param().abi(),
            ::std::mem::transmute(placeholderinfo),
            ::std::mem::transmute(placeholderinfosize),
            ::std::mem::transmute(extendedinfo),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
