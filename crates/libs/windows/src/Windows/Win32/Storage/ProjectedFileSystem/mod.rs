#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PRJ_CALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_CALLBACKS")
            .field("StartDirectoryEnumerationCallback", &self.StartDirectoryEnumerationCallback.map(|f| f as usize))
            .field("EndDirectoryEnumerationCallback", &self.EndDirectoryEnumerationCallback.map(|f| f as usize))
            .field("GetDirectoryEnumerationCallback", &self.GetDirectoryEnumerationCallback.map(|f| f as usize))
            .field("GetPlaceholderInfoCallback", &self.GetPlaceholderInfoCallback.map(|f| f as usize))
            .field("GetFileDataCallback", &self.GetFileDataCallback.map(|f| f as usize))
            .field("QueryFileNameCallback", &self.QueryFileNameCallback.map(|f| f as usize))
            .field("NotificationCallback", &self.NotificationCallback.map(|f| f as usize))
            .field("CancelCommandCallback", &self.CancelCommandCallback.map(|f| f as usize))
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PRJ_CALLBACKS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRJ_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_CALLBACKS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRJ_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRJ_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub struct PRJ_CALLBACK_DATA {
    pub Size: u32,
    pub Flags: PRJ_CALLBACK_DATA_FLAGS,
    pub NamespaceVirtualizationContext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT,
    pub CommandId: i32,
    pub FileId: ::windows::core::GUID,
    pub DataStreamId: ::windows::core::GUID,
    pub FilePathName: ::windows::core::PCWSTR,
    pub VersionInfo: *mut PRJ_PLACEHOLDER_VERSION_INFO,
    pub TriggeringProcessId: u32,
    pub TriggeringProcessImageFileName: ::windows::core::PCWSTR,
    pub InstanceContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for PRJ_CALLBACK_DATA {}
impl ::core::clone::Clone for PRJ_CALLBACK_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PRJ_CALLBACK_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_CALLBACK_DATA")
            .field("Size", &self.Size)
            .field("Flags", &self.Flags)
            .field("NamespaceVirtualizationContext", &self.NamespaceVirtualizationContext)
            .field("CommandId", &self.CommandId)
            .field("FileId", &self.FileId)
            .field("DataStreamId", &self.DataStreamId)
            .field("FilePathName", &self.FilePathName)
            .field("VersionInfo", &self.VersionInfo)
            .field("TriggeringProcessId", &self.TriggeringProcessId)
            .field("TriggeringProcessImageFileName", &self.TriggeringProcessImageFileName)
            .field("InstanceContext", &self.InstanceContext)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for PRJ_CALLBACK_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PRJ_CALLBACK_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_CALLBACK_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for PRJ_CALLBACK_DATA {}
impl ::core::default::Default for PRJ_CALLBACK_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRJ_CALLBACK_DATA_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_CB_DATA_FLAG_ENUM_RESTART_SCAN: PRJ_CALLBACK_DATA_FLAGS = PRJ_CALLBACK_DATA_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_CB_DATA_FLAG_ENUM_RETURN_SINGLE_ENTRY: PRJ_CALLBACK_DATA_FLAGS = PRJ_CALLBACK_DATA_FLAGS(2i32);
impl ::core::marker::Copy for PRJ_CALLBACK_DATA_FLAGS {}
impl ::core::clone::Clone for PRJ_CALLBACK_DATA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRJ_CALLBACK_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PRJ_CALLBACK_DATA_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PRJ_CALLBACK_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRJ_CALLBACK_DATA_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub type PRJ_CANCEL_COMMAND_CB = ::core::option::Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
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
unsafe impl ::windows::core::Abi for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {}
impl ::core::default::Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
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
unsafe impl ::windows::core::Abi for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {}
impl ::core::default::Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    pub DirEntryBufferHandle: PRJ_DIR_ENTRY_BUFFER_HANDLE,
}
impl ::core::marker::Copy for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {}
impl ::core::clone::Clone for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0").field("DirEntryBufferHandle", &self.DirEntryBufferHandle).finish()
    }
}
unsafe impl ::windows::core::Abi for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {}
impl ::core::default::Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    pub NotificationMask: PRJ_NOTIFY_TYPES,
}
impl ::core::marker::Copy for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {}
impl ::core::clone::Clone for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1").field("NotificationMask", &self.NotificationMask).finish()
    }
}
unsafe impl ::windows::core::Abi for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {}
impl ::core::default::Default for PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRJ_COMPLETE_COMMAND_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_COMPLETE_COMMAND_TYPE_NOTIFICATION: PRJ_COMPLETE_COMMAND_TYPE = PRJ_COMPLETE_COMMAND_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_COMPLETE_COMMAND_TYPE_ENUMERATION: PRJ_COMPLETE_COMMAND_TYPE = PRJ_COMPLETE_COMMAND_TYPE(2i32);
impl ::core::marker::Copy for PRJ_COMPLETE_COMMAND_TYPE {}
impl ::core::clone::Clone for PRJ_COMPLETE_COMMAND_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRJ_COMPLETE_COMMAND_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PRJ_COMPLETE_COMMAND_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PRJ_COMPLETE_COMMAND_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRJ_COMPLETE_COMMAND_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRJ_DIR_ENTRY_BUFFER_HANDLE(pub isize);
impl PRJ_DIR_ENTRY_BUFFER_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for PRJ_DIR_ENTRY_BUFFER_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PRJ_DIR_ENTRY_BUFFER_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PRJ_DIR_ENTRY_BUFFER_HANDLE {}
impl ::core::fmt::Debug for PRJ_DIR_ENTRY_BUFFER_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRJ_DIR_ENTRY_BUFFER_HANDLE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<PRJ_DIR_ENTRY_BUFFER_HANDLE>> for PRJ_DIR_ENTRY_BUFFER_HANDLE {
    fn from(optional: ::core::option::Option<PRJ_DIR_ENTRY_BUFFER_HANDLE>) -> PRJ_DIR_ENTRY_BUFFER_HANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for PRJ_DIR_ENTRY_BUFFER_HANDLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub type PRJ_END_DIRECTORY_ENUMERATION_CB = ::core::option::Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, enumerationid: *const ::windows::core::GUID) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub struct PRJ_EXTENDED_INFO {
    pub InfoType: PRJ_EXT_INFO_TYPE,
    pub NextInfoOffset: u32,
    pub Anonymous: PRJ_EXTENDED_INFO_0,
}
impl ::core::marker::Copy for PRJ_EXTENDED_INFO {}
impl ::core::clone::Clone for PRJ_EXTENDED_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRJ_EXTENDED_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PRJ_EXTENDED_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_EXTENDED_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PRJ_EXTENDED_INFO {}
impl ::core::default::Default for PRJ_EXTENDED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub union PRJ_EXTENDED_INFO_0 {
    pub Symlink: PRJ_EXTENDED_INFO_0_0,
}
impl ::core::marker::Copy for PRJ_EXTENDED_INFO_0 {}
impl ::core::clone::Clone for PRJ_EXTENDED_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRJ_EXTENDED_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PRJ_EXTENDED_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_EXTENDED_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PRJ_EXTENDED_INFO_0 {}
impl ::core::default::Default for PRJ_EXTENDED_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub struct PRJ_EXTENDED_INFO_0_0 {
    pub TargetName: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for PRJ_EXTENDED_INFO_0_0 {}
impl ::core::clone::Clone for PRJ_EXTENDED_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PRJ_EXTENDED_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_EXTENDED_INFO_0_0").field("TargetName", &self.TargetName).finish()
    }
}
unsafe impl ::windows::core::Abi for PRJ_EXTENDED_INFO_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PRJ_EXTENDED_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_EXTENDED_INFO_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PRJ_EXTENDED_INFO_0_0 {}
impl ::core::default::Default for PRJ_EXTENDED_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRJ_EXT_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_EXT_INFO_TYPE_SYMLINK: PRJ_EXT_INFO_TYPE = PRJ_EXT_INFO_TYPE(1i32);
impl ::core::marker::Copy for PRJ_EXT_INFO_TYPE {}
impl ::core::clone::Clone for PRJ_EXT_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRJ_EXT_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PRJ_EXT_INFO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PRJ_EXT_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRJ_EXT_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PRJ_FILE_BASIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_FILE_BASIC_INFO").field("IsDirectory", &self.IsDirectory).field("FileSize", &self.FileSize).field("CreationTime", &self.CreationTime).field("LastAccessTime", &self.LastAccessTime).field("LastWriteTime", &self.LastWriteTime).field("ChangeTime", &self.ChangeTime).field("FileAttributes", &self.FileAttributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PRJ_FILE_BASIC_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRJ_FILE_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_FILE_BASIC_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRJ_FILE_BASIC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRJ_FILE_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRJ_FILE_STATE(pub u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_FILE_STATE_PLACEHOLDER: PRJ_FILE_STATE = PRJ_FILE_STATE(1u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_FILE_STATE_HYDRATED_PLACEHOLDER: PRJ_FILE_STATE = PRJ_FILE_STATE(2u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_FILE_STATE_DIRTY_PLACEHOLDER: PRJ_FILE_STATE = PRJ_FILE_STATE(4u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_FILE_STATE_FULL: PRJ_FILE_STATE = PRJ_FILE_STATE(8u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_FILE_STATE_TOMBSTONE: PRJ_FILE_STATE = PRJ_FILE_STATE(16u32);
impl ::core::marker::Copy for PRJ_FILE_STATE {}
impl ::core::clone::Clone for PRJ_FILE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRJ_FILE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PRJ_FILE_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PRJ_FILE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRJ_FILE_STATE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PRJ_FILE_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PRJ_FILE_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PRJ_FILE_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PRJ_FILE_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PRJ_FILE_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub type PRJ_GET_DIRECTORY_ENUMERATION_CB = ::core::option::Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, enumerationid: *const ::windows::core::GUID, searchexpression: ::windows::core::PCWSTR, direntrybufferhandle: PRJ_DIR_ENTRY_BUFFER_HANDLE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub type PRJ_GET_FILE_DATA_CB = ::core::option::Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, byteoffset: u64, length: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub type PRJ_GET_PLACEHOLDER_INFO_CB = ::core::option::Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA) -> ::windows::core::HRESULT>;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT(pub isize);
impl PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {}
impl ::core::fmt::Debug for PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>> for PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {
    fn from(optional: ::core::option::Option<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>) -> PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRJ_NOTIFICATION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFICATION_FILE_OPENED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(2i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFICATION_NEW_FILE_CREATED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(4i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFICATION_FILE_OVERWRITTEN: PRJ_NOTIFICATION = PRJ_NOTIFICATION(8i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFICATION_PRE_DELETE: PRJ_NOTIFICATION = PRJ_NOTIFICATION(16i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFICATION_PRE_RENAME: PRJ_NOTIFICATION = PRJ_NOTIFICATION(32i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFICATION_PRE_SET_HARDLINK: PRJ_NOTIFICATION = PRJ_NOTIFICATION(64i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFICATION_FILE_RENAMED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(128i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFICATION_HARDLINK_CREATED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(256i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_NO_MODIFICATION: PRJ_NOTIFICATION = PRJ_NOTIFICATION(512i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_MODIFIED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(1024i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_DELETED: PRJ_NOTIFICATION = PRJ_NOTIFICATION(2048i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFICATION_FILE_PRE_CONVERT_TO_FULL: PRJ_NOTIFICATION = PRJ_NOTIFICATION(4096i32);
impl ::core::marker::Copy for PRJ_NOTIFICATION {}
impl ::core::clone::Clone for PRJ_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRJ_NOTIFICATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PRJ_NOTIFICATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for PRJ_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRJ_NOTIFICATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRJ_NOTIFICATION_CB = ::core::option::Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, isdirectory: super::super::Foundation::BOOLEAN, notification: PRJ_NOTIFICATION, destinationfilename: ::windows::core::PCWSTR, operationparameters: *mut PRJ_NOTIFICATION_PARAMETERS) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub struct PRJ_NOTIFICATION_MAPPING {
    pub NotificationBitMask: PRJ_NOTIFY_TYPES,
    pub NotificationRoot: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for PRJ_NOTIFICATION_MAPPING {}
impl ::core::clone::Clone for PRJ_NOTIFICATION_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PRJ_NOTIFICATION_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_NOTIFICATION_MAPPING").field("NotificationBitMask", &self.NotificationBitMask).field("NotificationRoot", &self.NotificationRoot).finish()
    }
}
unsafe impl ::windows::core::Abi for PRJ_NOTIFICATION_MAPPING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PRJ_NOTIFICATION_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_NOTIFICATION_MAPPING>()) == 0 }
    }
}
impl ::core::cmp::Eq for PRJ_NOTIFICATION_MAPPING {}
impl ::core::default::Default for PRJ_NOTIFICATION_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PRJ_NOTIFICATION_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRJ_NOTIFICATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_NOTIFICATION_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRJ_NOTIFICATION_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRJ_NOTIFICATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PRJ_NOTIFICATION_PARAMETERS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_NOTIFICATION_PARAMETERS_0").field("IsFileModified", &self.IsFileModified).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PRJ_NOTIFICATION_PARAMETERS_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRJ_NOTIFICATION_PARAMETERS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_NOTIFICATION_PARAMETERS_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRJ_NOTIFICATION_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRJ_NOTIFICATION_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PRJ_NOTIFICATION_PARAMETERS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_NOTIFICATION_PARAMETERS_1").field("NotificationMask", &self.NotificationMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PRJ_NOTIFICATION_PARAMETERS_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRJ_NOTIFICATION_PARAMETERS_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_NOTIFICATION_PARAMETERS_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRJ_NOTIFICATION_PARAMETERS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRJ_NOTIFICATION_PARAMETERS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PRJ_NOTIFICATION_PARAMETERS_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_NOTIFICATION_PARAMETERS_2").field("NotificationMask", &self.NotificationMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PRJ_NOTIFICATION_PARAMETERS_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRJ_NOTIFICATION_PARAMETERS_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_NOTIFICATION_PARAMETERS_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRJ_NOTIFICATION_PARAMETERS_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRJ_NOTIFICATION_PARAMETERS_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRJ_NOTIFY_TYPES(pub u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_NONE: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(0u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_SUPPRESS_NOTIFICATIONS: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(1u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_FILE_OPENED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(2u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_NEW_FILE_CREATED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(4u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_FILE_OVERWRITTEN: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(8u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_PRE_DELETE: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(16u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_PRE_RENAME: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(32u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_PRE_SET_HARDLINK: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(64u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_FILE_RENAMED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(128u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_HARDLINK_CREATED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(256u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_NO_MODIFICATION: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(512u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_MODIFIED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(1024u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_DELETED: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(2048u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_FILE_PRE_CONVERT_TO_FULL: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(4096u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_NOTIFY_USE_EXISTING_MASK: PRJ_NOTIFY_TYPES = PRJ_NOTIFY_TYPES(4294967295u32);
impl ::core::marker::Copy for PRJ_NOTIFY_TYPES {}
impl ::core::clone::Clone for PRJ_NOTIFY_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRJ_NOTIFY_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PRJ_NOTIFY_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for PRJ_NOTIFY_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRJ_NOTIFY_TYPES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PRJ_NOTIFY_TYPES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PRJ_NOTIFY_TYPES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PRJ_NOTIFY_TYPES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PRJ_NOTIFY_TYPES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PRJ_NOTIFY_TYPES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRJ_PLACEHOLDER_ID(pub i32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_PLACEHOLDER_ID_LENGTH: PRJ_PLACEHOLDER_ID = PRJ_PLACEHOLDER_ID(128i32);
impl ::core::marker::Copy for PRJ_PLACEHOLDER_ID {}
impl ::core::clone::Clone for PRJ_PLACEHOLDER_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRJ_PLACEHOLDER_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PRJ_PLACEHOLDER_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for PRJ_PLACEHOLDER_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRJ_PLACEHOLDER_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PRJ_PLACEHOLDER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_PLACEHOLDER_INFO").field("FileBasicInfo", &self.FileBasicInfo).field("EaInformation", &self.EaInformation).field("SecurityInformation", &self.SecurityInformation).field("StreamsInformation", &self.StreamsInformation).field("VersionInfo", &self.VersionInfo).field("VariableData", &self.VariableData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PRJ_PLACEHOLDER_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRJ_PLACEHOLDER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_PLACEHOLDER_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRJ_PLACEHOLDER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRJ_PLACEHOLDER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PRJ_PLACEHOLDER_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_PLACEHOLDER_INFO_0").field("EaBufferSize", &self.EaBufferSize).field("OffsetToFirstEa", &self.OffsetToFirstEa).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PRJ_PLACEHOLDER_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRJ_PLACEHOLDER_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_PLACEHOLDER_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRJ_PLACEHOLDER_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRJ_PLACEHOLDER_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PRJ_PLACEHOLDER_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_PLACEHOLDER_INFO_1").field("SecurityBufferSize", &self.SecurityBufferSize).field("OffsetToSecurityDescriptor", &self.OffsetToSecurityDescriptor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PRJ_PLACEHOLDER_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRJ_PLACEHOLDER_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_PLACEHOLDER_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRJ_PLACEHOLDER_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRJ_PLACEHOLDER_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PRJ_PLACEHOLDER_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_PLACEHOLDER_INFO_2").field("StreamsInfoBufferSize", &self.StreamsInfoBufferSize).field("OffsetToFirstStreamInfo", &self.OffsetToFirstStreamInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PRJ_PLACEHOLDER_INFO_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRJ_PLACEHOLDER_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_PLACEHOLDER_INFO_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRJ_PLACEHOLDER_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRJ_PLACEHOLDER_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
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
impl ::core::fmt::Debug for PRJ_PLACEHOLDER_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_PLACEHOLDER_VERSION_INFO").field("ProviderID", &self.ProviderID).field("ContentID", &self.ContentID).finish()
    }
}
unsafe impl ::windows::core::Abi for PRJ_PLACEHOLDER_VERSION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PRJ_PLACEHOLDER_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_PLACEHOLDER_VERSION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PRJ_PLACEHOLDER_VERSION_INFO {}
impl ::core::default::Default for PRJ_PLACEHOLDER_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub type PRJ_QUERY_FILE_NAME_CB = ::core::option::Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRJ_STARTVIRTUALIZING_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_FLAG_NONE: PRJ_STARTVIRTUALIZING_FLAGS = PRJ_STARTVIRTUALIZING_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_FLAG_USE_NEGATIVE_PATH_CACHE: PRJ_STARTVIRTUALIZING_FLAGS = PRJ_STARTVIRTUALIZING_FLAGS(1u32);
impl ::core::marker::Copy for PRJ_STARTVIRTUALIZING_FLAGS {}
impl ::core::clone::Clone for PRJ_STARTVIRTUALIZING_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRJ_STARTVIRTUALIZING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PRJ_STARTVIRTUALIZING_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PRJ_STARTVIRTUALIZING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRJ_STARTVIRTUALIZING_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PRJ_STARTVIRTUALIZING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PRJ_STARTVIRTUALIZING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PRJ_STARTVIRTUALIZING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PRJ_STARTVIRTUALIZING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PRJ_STARTVIRTUALIZING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub struct PRJ_STARTVIRTUALIZING_OPTIONS {
    pub Flags: PRJ_STARTVIRTUALIZING_FLAGS,
    pub PoolThreadCount: u32,
    pub ConcurrentThreadCount: u32,
    pub NotificationMappings: *mut PRJ_NOTIFICATION_MAPPING,
    pub NotificationMappingsCount: u32,
}
impl ::core::marker::Copy for PRJ_STARTVIRTUALIZING_OPTIONS {}
impl ::core::clone::Clone for PRJ_STARTVIRTUALIZING_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PRJ_STARTVIRTUALIZING_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_STARTVIRTUALIZING_OPTIONS").field("Flags", &self.Flags).field("PoolThreadCount", &self.PoolThreadCount).field("ConcurrentThreadCount", &self.ConcurrentThreadCount).field("NotificationMappings", &self.NotificationMappings).field("NotificationMappingsCount", &self.NotificationMappingsCount).finish()
    }
}
unsafe impl ::windows::core::Abi for PRJ_STARTVIRTUALIZING_OPTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PRJ_STARTVIRTUALIZING_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_STARTVIRTUALIZING_OPTIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PRJ_STARTVIRTUALIZING_OPTIONS {}
impl ::core::default::Default for PRJ_STARTVIRTUALIZING_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub type PRJ_START_DIRECTORY_ENUMERATION_CB = ::core::option::Option<unsafe extern "system" fn(callbackdata: *const PRJ_CALLBACK_DATA, enumerationid: *const ::windows::core::GUID) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRJ_UPDATE_FAILURE_CAUSES(pub u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_UPDATE_FAILURE_CAUSE_NONE: PRJ_UPDATE_FAILURE_CAUSES = PRJ_UPDATE_FAILURE_CAUSES(0u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_UPDATE_FAILURE_CAUSE_DIRTY_METADATA: PRJ_UPDATE_FAILURE_CAUSES = PRJ_UPDATE_FAILURE_CAUSES(1u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_UPDATE_FAILURE_CAUSE_DIRTY_DATA: PRJ_UPDATE_FAILURE_CAUSES = PRJ_UPDATE_FAILURE_CAUSES(2u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_UPDATE_FAILURE_CAUSE_TOMBSTONE: PRJ_UPDATE_FAILURE_CAUSES = PRJ_UPDATE_FAILURE_CAUSES(4u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_UPDATE_FAILURE_CAUSE_READ_ONLY: PRJ_UPDATE_FAILURE_CAUSES = PRJ_UPDATE_FAILURE_CAUSES(8u32);
impl ::core::marker::Copy for PRJ_UPDATE_FAILURE_CAUSES {}
impl ::core::clone::Clone for PRJ_UPDATE_FAILURE_CAUSES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRJ_UPDATE_FAILURE_CAUSES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PRJ_UPDATE_FAILURE_CAUSES {
    type Abi = Self;
}
impl ::core::fmt::Debug for PRJ_UPDATE_FAILURE_CAUSES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRJ_UPDATE_FAILURE_CAUSES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PRJ_UPDATE_FAILURE_CAUSES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PRJ_UPDATE_FAILURE_CAUSES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PRJ_UPDATE_FAILURE_CAUSES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PRJ_UPDATE_FAILURE_CAUSES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PRJ_UPDATE_FAILURE_CAUSES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRJ_UPDATE_TYPES(pub u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_UPDATE_NONE: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(0u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_UPDATE_ALLOW_DIRTY_METADATA: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(1u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_UPDATE_ALLOW_DIRTY_DATA: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(2u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_UPDATE_ALLOW_TOMBSTONE: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(4u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_UPDATE_RESERVED1: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(8u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_UPDATE_RESERVED2: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(16u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_UPDATE_ALLOW_READ_ONLY: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(32u32);
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub const PRJ_UPDATE_MAX_VAL: PRJ_UPDATE_TYPES = PRJ_UPDATE_TYPES(64u32);
impl ::core::marker::Copy for PRJ_UPDATE_TYPES {}
impl ::core::clone::Clone for PRJ_UPDATE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRJ_UPDATE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PRJ_UPDATE_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for PRJ_UPDATE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRJ_UPDATE_TYPES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PRJ_UPDATE_TYPES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PRJ_UPDATE_TYPES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PRJ_UPDATE_TYPES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PRJ_UPDATE_TYPES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PRJ_UPDATE_TYPES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
pub struct PRJ_VIRTUALIZATION_INSTANCE_INFO {
    pub InstanceID: ::windows::core::GUID,
    pub WriteAlignment: u32,
}
impl ::core::marker::Copy for PRJ_VIRTUALIZATION_INSTANCE_INFO {}
impl ::core::clone::Clone for PRJ_VIRTUALIZATION_INSTANCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PRJ_VIRTUALIZATION_INSTANCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRJ_VIRTUALIZATION_INSTANCE_INFO").field("InstanceID", &self.InstanceID).field("WriteAlignment", &self.WriteAlignment).finish()
    }
}
unsafe impl ::windows::core::Abi for PRJ_VIRTUALIZATION_INSTANCE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PRJ_VIRTUALIZATION_INSTANCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRJ_VIRTUALIZATION_INSTANCE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PRJ_VIRTUALIZATION_INSTANCE_INFO {}
impl ::core::default::Default for PRJ_VIRTUALIZATION_INSTANCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[inline]
pub unsafe fn PrjAllocateAlignedBuffer<'a, P0>(namespacevirtualizationcontext: P0, size: usize) -> *mut ::core::ffi::c_void
where
    P0: ::std::convert::Into<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjAllocateAlignedBuffer(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, size: usize) -> *mut ::core::ffi::c_void;
    }
    PrjAllocateAlignedBuffer(namespacevirtualizationcontext.into(), size)
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[inline]
pub unsafe fn PrjClearNegativePathCache<'a, P0>(namespacevirtualizationcontext: P0) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjClearNegativePathCache(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, totalentrynumber: *mut u32) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PrjClearNegativePathCache(namespacevirtualizationcontext.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[inline]
pub unsafe fn PrjCompleteCommand<'a, P0>(namespacevirtualizationcontext: P0, commandid: i32, completionresult: ::windows::core::HRESULT, extendedparameters: *const PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjCompleteCommand(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, commandid: i32, completionresult: ::windows::core::HRESULT, extendedparameters: *const PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS) -> ::windows::core::HRESULT;
    }
    PrjCompleteCommand(namespacevirtualizationcontext.into(), commandid, completionresult, ::core::mem::transmute(extendedparameters)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[inline]
pub unsafe fn PrjDeleteFile<'a, P0, P1>(namespacevirtualizationcontext: P0, destinationfilename: P1, updateflags: PRJ_UPDATE_TYPES) -> ::windows::core::Result<PRJ_UPDATE_FAILURE_CAUSES>
where
    P0: ::std::convert::Into<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjDeleteFile(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename: ::windows::core::PCWSTR, updateflags: PRJ_UPDATE_TYPES, failurereason: *mut PRJ_UPDATE_FAILURE_CAUSES) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PrjDeleteFile(namespacevirtualizationcontext.into(), destinationfilename.into(), updateflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PRJ_UPDATE_FAILURE_CAUSES>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjDoesNameContainWildCards<'a, P0>(filename: P0) -> super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjDoesNameContainWildCards(filename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOLEAN;
    }
    PrjDoesNameContainWildCards(filename.into())
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[inline]
pub unsafe fn PrjFileNameCompare<'a, P0, P1>(filename1: P0, filename2: P1) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjFileNameCompare(filename1: ::windows::core::PCWSTR, filename2: ::windows::core::PCWSTR) -> i32;
    }
    PrjFileNameCompare(filename1.into(), filename2.into())
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjFileNameMatch<'a, P0, P1>(filenametocheck: P0, pattern: P1) -> super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjFileNameMatch(filenametocheck: ::windows::core::PCWSTR, pattern: ::windows::core::PCWSTR) -> super::super::Foundation::BOOLEAN;
    }
    PrjFileNameMatch(filenametocheck.into(), pattern.into())
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjFillDirEntryBuffer<'a, P0, P1>(filename: P0, filebasicinfo: *const PRJ_FILE_BASIC_INFO, direntrybufferhandle: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<PRJ_DIR_ENTRY_BUFFER_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjFillDirEntryBuffer(filename: ::windows::core::PCWSTR, filebasicinfo: *const PRJ_FILE_BASIC_INFO, direntrybufferhandle: PRJ_DIR_ENTRY_BUFFER_HANDLE) -> ::windows::core::HRESULT;
    }
    PrjFillDirEntryBuffer(filename.into(), ::core::mem::transmute(filebasicinfo), direntrybufferhandle.into()).ok()
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjFillDirEntryBuffer2<'a, P0, P1>(direntrybufferhandle: P0, filename: P1, filebasicinfo: *const PRJ_FILE_BASIC_INFO, extendedinfo: *const PRJ_EXTENDED_INFO) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<PRJ_DIR_ENTRY_BUFFER_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjFillDirEntryBuffer2(direntrybufferhandle: PRJ_DIR_ENTRY_BUFFER_HANDLE, filename: ::windows::core::PCWSTR, filebasicinfo: *const PRJ_FILE_BASIC_INFO, extendedinfo: *const PRJ_EXTENDED_INFO) -> ::windows::core::HRESULT;
    }
    PrjFillDirEntryBuffer2(direntrybufferhandle.into(), filename.into(), ::core::mem::transmute(filebasicinfo), ::core::mem::transmute(extendedinfo)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[inline]
pub unsafe fn PrjFreeAlignedBuffer(buffer: *const ::core::ffi::c_void) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjFreeAlignedBuffer(buffer: *const ::core::ffi::c_void);
    }
    PrjFreeAlignedBuffer(::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[inline]
pub unsafe fn PrjGetOnDiskFileState<'a, P0>(destinationfilename: P0) -> ::windows::core::Result<PRJ_FILE_STATE>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjGetOnDiskFileState(destinationfilename: ::windows::core::PCWSTR, filestate: *mut PRJ_FILE_STATE) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PrjGetOnDiskFileState(destinationfilename.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PRJ_FILE_STATE>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[inline]
pub unsafe fn PrjGetVirtualizationInstanceInfo<'a, P0>(namespacevirtualizationcontext: P0) -> ::windows::core::Result<PRJ_VIRTUALIZATION_INSTANCE_INFO>
where
    P0: ::std::convert::Into<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjGetVirtualizationInstanceInfo(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, virtualizationinstanceinfo: *mut PRJ_VIRTUALIZATION_INSTANCE_INFO) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PrjGetVirtualizationInstanceInfo(namespacevirtualizationcontext.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PRJ_VIRTUALIZATION_INSTANCE_INFO>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[inline]
pub unsafe fn PrjMarkDirectoryAsPlaceholder<'a, P0, P1>(rootpathname: P0, targetpathname: P1, versioninfo: *const PRJ_PLACEHOLDER_VERSION_INFO, virtualizationinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjMarkDirectoryAsPlaceholder(rootpathname: ::windows::core::PCWSTR, targetpathname: ::windows::core::PCWSTR, versioninfo: *const PRJ_PLACEHOLDER_VERSION_INFO, virtualizationinstanceid: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
    }
    PrjMarkDirectoryAsPlaceholder(rootpathname.into(), targetpathname.into(), ::core::mem::transmute(versioninfo), ::core::mem::transmute(virtualizationinstanceid)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjStartVirtualizing<'a, P0>(virtualizationrootpath: P0, callbacks: *const PRJ_CALLBACKS, instancecontext: *const ::core::ffi::c_void, options: *const PRJ_STARTVIRTUALIZING_OPTIONS) -> ::windows::core::Result<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjStartVirtualizing(virtualizationrootpath: ::windows::core::PCWSTR, callbacks: *const PRJ_CALLBACKS, instancecontext: *const ::core::ffi::c_void, options: *const PRJ_STARTVIRTUALIZING_OPTIONS, namespacevirtualizationcontext: *mut PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PrjStartVirtualizing(virtualizationrootpath.into(), ::core::mem::transmute(callbacks), ::core::mem::transmute(instancecontext), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[inline]
pub unsafe fn PrjStopVirtualizing<'a, P0>(namespacevirtualizationcontext: P0)
where
    P0: ::std::convert::Into<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjStopVirtualizing(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT);
    }
    PrjStopVirtualizing(namespacevirtualizationcontext.into())
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjUpdateFileIfNeeded<'a, P0, P1>(namespacevirtualizationcontext: P0, destinationfilename: P1, placeholderinfo: *const PRJ_PLACEHOLDER_INFO, placeholderinfosize: u32, updateflags: PRJ_UPDATE_TYPES) -> ::windows::core::Result<PRJ_UPDATE_FAILURE_CAUSES>
where
    P0: ::std::convert::Into<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjUpdateFileIfNeeded(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename: ::windows::core::PCWSTR, placeholderinfo: *const PRJ_PLACEHOLDER_INFO, placeholderinfosize: u32, updateflags: PRJ_UPDATE_TYPES, failurereason: *mut PRJ_UPDATE_FAILURE_CAUSES) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PrjUpdateFileIfNeeded(namespacevirtualizationcontext.into(), destinationfilename.into(), ::core::mem::transmute(placeholderinfo), placeholderinfosize, updateflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PRJ_UPDATE_FAILURE_CAUSES>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`*"]
#[inline]
pub unsafe fn PrjWriteFileData<'a, P0>(namespacevirtualizationcontext: P0, datastreamid: *const ::windows::core::GUID, buffer: *const ::core::ffi::c_void, byteoffset: u64, length: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjWriteFileData(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, datastreamid: *const ::windows::core::GUID, buffer: *const ::core::ffi::c_void, byteoffset: u64, length: u32) -> ::windows::core::HRESULT;
    }
    PrjWriteFileData(namespacevirtualizationcontext.into(), ::core::mem::transmute(datastreamid), ::core::mem::transmute(buffer), byteoffset, length).ok()
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjWritePlaceholderInfo<'a, P0, P1>(namespacevirtualizationcontext: P0, destinationfilename: P1, placeholderinfo: *const PRJ_PLACEHOLDER_INFO, placeholderinfosize: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjWritePlaceholderInfo(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename: ::windows::core::PCWSTR, placeholderinfo: *const PRJ_PLACEHOLDER_INFO, placeholderinfosize: u32) -> ::windows::core::HRESULT;
    }
    PrjWritePlaceholderInfo(namespacevirtualizationcontext.into(), destinationfilename.into(), ::core::mem::transmute(placeholderinfo), placeholderinfosize).ok()
}
#[doc = "*Required features: `\"Win32_Storage_ProjectedFileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrjWritePlaceholderInfo2<'a, P0, P1>(namespacevirtualizationcontext: P0, destinationfilename: P1, placeholderinfo: *const PRJ_PLACEHOLDER_INFO, placeholderinfosize: u32, extendedinfo: *const PRJ_EXTENDED_INFO) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PrjWritePlaceholderInfo2(namespacevirtualizationcontext: PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT, destinationfilename: ::windows::core::PCWSTR, placeholderinfo: *const PRJ_PLACEHOLDER_INFO, placeholderinfosize: u32, extendedinfo: *const PRJ_EXTENDED_INFO) -> ::windows::core::HRESULT;
    }
    PrjWritePlaceholderInfo2(namespacevirtualizationcontext.into(), destinationfilename.into(), ::core::mem::transmute(placeholderinfo), placeholderinfosize, ::core::mem::transmute(extendedinfo)).ok()
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
