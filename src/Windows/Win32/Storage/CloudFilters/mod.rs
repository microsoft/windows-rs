#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub type CF_CALLBACK = unsafe extern "system" fn(callbackinfo: *const CF_CALLBACK_INFO, callbackparameters: *const CF_CALLBACK_PARAMETERS);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_CANCEL_FLAGS(pub u32);
pub const CF_CALLBACK_CANCEL_FLAG_NONE: CF_CALLBACK_CANCEL_FLAGS = CF_CALLBACK_CANCEL_FLAGS(0u32);
pub const CF_CALLBACK_CANCEL_FLAG_IO_TIMEOUT: CF_CALLBACK_CANCEL_FLAGS = CF_CALLBACK_CANCEL_FLAGS(1u32);
pub const CF_CALLBACK_CANCEL_FLAG_IO_ABORTED: CF_CALLBACK_CANCEL_FLAGS = CF_CALLBACK_CANCEL_FLAGS(2u32);
impl ::std::convert::From<u32> for CF_CALLBACK_CANCEL_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_CANCEL_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CALLBACK_CANCEL_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CALLBACK_CANCEL_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CALLBACK_CANCEL_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CALLBACK_CANCEL_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CALLBACK_CANCEL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_CLOSE_COMPLETION_FLAGS(pub u32);
pub const CF_CALLBACK_CLOSE_COMPLETION_FLAG_NONE: CF_CALLBACK_CLOSE_COMPLETION_FLAGS = CF_CALLBACK_CLOSE_COMPLETION_FLAGS(0u32);
pub const CF_CALLBACK_CLOSE_COMPLETION_FLAG_DELETED: CF_CALLBACK_CLOSE_COMPLETION_FLAGS = CF_CALLBACK_CLOSE_COMPLETION_FLAGS(1u32);
impl ::std::convert::From<u32> for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS(pub u32);
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_NONE: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS(0u32);
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_BACKGROUND: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS(1u32);
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_DEHYDRATED: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS(2u32);
impl ::std::convert::From<u32> for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_DEHYDRATE_FLAGS(pub u32);
pub const CF_CALLBACK_DEHYDRATE_FLAG_NONE: CF_CALLBACK_DEHYDRATE_FLAGS = CF_CALLBACK_DEHYDRATE_FLAGS(0u32);
pub const CF_CALLBACK_DEHYDRATE_FLAG_BACKGROUND: CF_CALLBACK_DEHYDRATE_FLAGS = CF_CALLBACK_DEHYDRATE_FLAGS(1u32);
impl ::std::convert::From<u32> for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_DEHYDRATION_REASON(pub i32);
pub const CF_CALLBACK_DEHYDRATION_REASON_NONE: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(0i32);
pub const CF_CALLBACK_DEHYDRATION_REASON_USER_MANUAL: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(1i32);
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_LOW_SPACE: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(2i32);
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_INACTIVITY: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(3i32);
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_OS_UPGRADE: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(4i32);
impl ::std::convert::From<i32> for CF_CALLBACK_DEHYDRATION_REASON {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_DEHYDRATION_REASON {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_DELETE_COMPLETION_FLAGS(pub u32);
pub const CF_CALLBACK_DELETE_COMPLETION_FLAG_NONE: CF_CALLBACK_DELETE_COMPLETION_FLAGS = CF_CALLBACK_DELETE_COMPLETION_FLAGS(0u32);
impl ::std::convert::From<u32> for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_DELETE_FLAGS(pub u32);
pub const CF_CALLBACK_DELETE_FLAG_NONE: CF_CALLBACK_DELETE_FLAGS = CF_CALLBACK_DELETE_FLAGS(0u32);
pub const CF_CALLBACK_DELETE_FLAG_IS_DIRECTORY: CF_CALLBACK_DELETE_FLAGS = CF_CALLBACK_DELETE_FLAGS(1u32);
pub const CF_CALLBACK_DELETE_FLAG_IS_UNDELETE: CF_CALLBACK_DELETE_FLAGS = CF_CALLBACK_DELETE_FLAGS(2u32);
impl ::std::convert::From<u32> for CF_CALLBACK_DELETE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_DELETE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CALLBACK_DELETE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CALLBACK_DELETE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CALLBACK_DELETE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CALLBACK_DELETE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CALLBACK_DELETE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_FETCH_DATA_FLAGS(pub u32);
pub const CF_CALLBACK_FETCH_DATA_FLAG_NONE: CF_CALLBACK_FETCH_DATA_FLAGS = CF_CALLBACK_FETCH_DATA_FLAGS(0u32);
pub const CF_CALLBACK_FETCH_DATA_FLAG_RECOVERY: CF_CALLBACK_FETCH_DATA_FLAGS = CF_CALLBACK_FETCH_DATA_FLAGS(1u32);
pub const CF_CALLBACK_FETCH_DATA_FLAG_EXPLICIT_HYDRATION: CF_CALLBACK_FETCH_DATA_FLAGS = CF_CALLBACK_FETCH_DATA_FLAGS(2u32);
impl ::std::convert::From<u32> for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS(pub u32);
pub const CF_CALLBACK_FETCH_PLACEHOLDERS_FLAG_NONE: CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS = CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS(0u32);
impl ::std::convert::From<u32> for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct CF_CALLBACK_INFO {
    pub StructSize: u32,
    pub ConnectionKey: CF_CONNECTION_KEY,
    pub CallbackContext: *mut ::std::ffi::c_void,
    pub VolumeGuidName: super::super::Foundation::PWSTR,
    pub VolumeDosName: super::super::Foundation::PWSTR,
    pub VolumeSerialNumber: u32,
    pub SyncRootFileId: i64,
    pub SyncRootIdentity: *mut ::std::ffi::c_void,
    pub SyncRootIdentityLength: u32,
    pub FileId: i64,
    pub FileSize: i64,
    pub FileIdentity: *mut ::std::ffi::c_void,
    pub FileIdentityLength: u32,
    pub NormalizedPath: super::super::Foundation::PWSTR,
    pub TransferKey: i64,
    pub PriorityHint: u8,
    pub CorrelationVector: *mut super::super::System::SystemServices::CORRELATION_VECTOR,
    pub ProcessInfo: *mut CF_PROCESS_INFO,
    pub RequestKey: i64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl CF_CALLBACK_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for CF_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for CF_CALLBACK_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_CALLBACK_INFO")
            .field("StructSize", &self.StructSize)
            .field("ConnectionKey", &self.ConnectionKey)
            .field("CallbackContext", &self.CallbackContext)
            .field("VolumeGuidName", &self.VolumeGuidName)
            .field("VolumeDosName", &self.VolumeDosName)
            .field("VolumeSerialNumber", &self.VolumeSerialNumber)
            .field("SyncRootFileId", &self.SyncRootFileId)
            .field("SyncRootIdentity", &self.SyncRootIdentity)
            .field("SyncRootIdentityLength", &self.SyncRootIdentityLength)
            .field("FileId", &self.FileId)
            .field("FileSize", &self.FileSize)
            .field("FileIdentity", &self.FileIdentity)
            .field("FileIdentityLength", &self.FileIdentityLength)
            .field("NormalizedPath", &self.NormalizedPath)
            .field("TransferKey", &self.TransferKey)
            .field("PriorityHint", &self.PriorityHint)
            .field("CorrelationVector", &self.CorrelationVector)
            .field("ProcessInfo", &self.ProcessInfo)
            .field("RequestKey", &self.RequestKey)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for CF_CALLBACK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StructSize == other.StructSize
            && self.ConnectionKey == other.ConnectionKey
            && self.CallbackContext == other.CallbackContext
            && self.VolumeGuidName == other.VolumeGuidName
            && self.VolumeDosName == other.VolumeDosName
            && self.VolumeSerialNumber == other.VolumeSerialNumber
            && self.SyncRootFileId == other.SyncRootFileId
            && self.SyncRootIdentity == other.SyncRootIdentity
            && self.SyncRootIdentityLength == other.SyncRootIdentityLength
            && self.FileId == other.FileId
            && self.FileSize == other.FileSize
            && self.FileIdentity == other.FileIdentity
            && self.FileIdentityLength == other.FileIdentityLength
            && self.NormalizedPath == other.NormalizedPath
            && self.TransferKey == other.TransferKey
            && self.PriorityHint == other.PriorityHint
            && self.CorrelationVector == other.CorrelationVector
            && self.ProcessInfo == other.ProcessInfo
            && self.RequestKey == other.RequestKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for CF_CALLBACK_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_OPEN_COMPLETION_FLAGS(pub u32);
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_NONE: CF_CALLBACK_OPEN_COMPLETION_FLAGS = CF_CALLBACK_OPEN_COMPLETION_FLAGS(0u32);
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNKNOWN: CF_CALLBACK_OPEN_COMPLETION_FLAGS = CF_CALLBACK_OPEN_COMPLETION_FLAGS(1u32);
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNSUPPORTED: CF_CALLBACK_OPEN_COMPLETION_FLAGS = CF_CALLBACK_OPEN_COMPLETION_FLAGS(2u32);
impl ::std::convert::From<u32> for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS {
    pub ParamSize: u32,
    pub Anonymous: CF_CALLBACK_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl CF_CALLBACK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CF_CALLBACK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl CF_CALLBACK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_CALLBACK_PARAMETERS_0_0 {
    pub Flags: CF_CALLBACK_CANCEL_FLAGS,
    pub Anonymous: CF_CALLBACK_PARAMETERS_0_0_0,
}
impl CF_CALLBACK_PARAMETERS_0_0 {}
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_0 {}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union CF_CALLBACK_PARAMETERS_0_0_0 {
    pub FetchData: CF_CALLBACK_PARAMETERS_0_0_0_0,
}
impl CF_CALLBACK_PARAMETERS_0_0_0 {}
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_0_0 {}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_CALLBACK_PARAMETERS_0_0_0_0 {
    pub FileOffset: i64,
    pub Length: i64,
}
impl CF_CALLBACK_PARAMETERS_0_0_0_0 {}
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_CALLBACK_PARAMETERS_0_0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FetchData_e__Struct").field("FileOffset", &self.FileOffset).field("Length", &self.Length).finish()
    }
}
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.FileOffset == other.FileOffset && self.Length == other.Length
    }
}
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_0_0_0 {}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_0_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_CALLBACK_PARAMETERS_0_1 {
    pub Flags: CF_CALLBACK_CLOSE_COMPLETION_FLAGS,
}
impl CF_CALLBACK_PARAMETERS_0_1 {}
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_CALLBACK_PARAMETERS_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_CloseCompletion_e__Struct").field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_1 {}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_CALLBACK_PARAMETERS_0_2 {
    pub Flags: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS,
    pub Reason: CF_CALLBACK_DEHYDRATION_REASON,
}
impl CF_CALLBACK_PARAMETERS_0_2 {}
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_CALLBACK_PARAMETERS_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_DehydrateCompletion_e__Struct").field("Flags", &self.Flags).field("Reason", &self.Reason).finish()
    }
}
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Reason == other.Reason
    }
}
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_2 {}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_CALLBACK_PARAMETERS_0_3 {
    pub Flags: CF_CALLBACK_DEHYDRATE_FLAGS,
    pub Reason: CF_CALLBACK_DEHYDRATION_REASON,
}
impl CF_CALLBACK_PARAMETERS_0_3 {}
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_CALLBACK_PARAMETERS_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Dehydrate_e__Struct").field("Flags", &self.Flags).field("Reason", &self.Reason).finish()
    }
}
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Reason == other.Reason
    }
}
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_3 {}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_CALLBACK_PARAMETERS_0_4 {
    pub Flags: CF_CALLBACK_DELETE_COMPLETION_FLAGS,
}
impl CF_CALLBACK_PARAMETERS_0_4 {}
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_CALLBACK_PARAMETERS_0_4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_DeleteCompletion_e__Struct").field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_4 {}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_4 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_CALLBACK_PARAMETERS_0_5 {
    pub Flags: CF_CALLBACK_DELETE_FLAGS,
}
impl CF_CALLBACK_PARAMETERS_0_5 {}
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_CALLBACK_PARAMETERS_0_5 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Delete_e__Struct").field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_5 {}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_5 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_CALLBACK_PARAMETERS_0_6 {
    pub Flags: CF_CALLBACK_FETCH_DATA_FLAGS,
    pub RequiredFileOffset: i64,
    pub RequiredLength: i64,
    pub OptionalFileOffset: i64,
    pub OptionalLength: i64,
    pub LastDehydrationTime: i64,
    pub LastDehydrationReason: CF_CALLBACK_DEHYDRATION_REASON,
}
impl CF_CALLBACK_PARAMETERS_0_6 {}
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_CALLBACK_PARAMETERS_0_6 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FetchData_e__Struct")
            .field("Flags", &self.Flags)
            .field("RequiredFileOffset", &self.RequiredFileOffset)
            .field("RequiredLength", &self.RequiredLength)
            .field("OptionalFileOffset", &self.OptionalFileOffset)
            .field("OptionalLength", &self.OptionalLength)
            .field("LastDehydrationTime", &self.LastDehydrationTime)
            .field("LastDehydrationReason", &self.LastDehydrationReason)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.RequiredFileOffset == other.RequiredFileOffset && self.RequiredLength == other.RequiredLength && self.OptionalFileOffset == other.OptionalFileOffset && self.OptionalLength == other.OptionalLength && self.LastDehydrationTime == other.LastDehydrationTime && self.LastDehydrationReason == other.LastDehydrationReason
    }
}
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_6 {}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_6 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_7 {
    pub Flags: CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS,
    pub Pattern: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CF_CALLBACK_PARAMETERS_0_7 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_7 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CF_CALLBACK_PARAMETERS_0_7 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FetchPlaceholders_e__Struct").field("Flags", &self.Flags).field("Pattern", &self.Pattern).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_7 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Pattern == other.Pattern
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_7 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_7 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_CALLBACK_PARAMETERS_0_8 {
    pub Flags: CF_CALLBACK_OPEN_COMPLETION_FLAGS,
}
impl CF_CALLBACK_PARAMETERS_0_8 {}
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_8 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_CALLBACK_PARAMETERS_0_8 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_OpenCompletion_e__Struct").field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_8 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_8 {}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_8 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_9 {
    pub Flags: CF_CALLBACK_RENAME_COMPLETION_FLAGS,
    pub SourcePath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CF_CALLBACK_PARAMETERS_0_9 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_9 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CF_CALLBACK_PARAMETERS_0_9 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_RenameCompletion_e__Struct").field("Flags", &self.Flags).field("SourcePath", &self.SourcePath).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_9 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.SourcePath == other.SourcePath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_9 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_9 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_CALLBACK_PARAMETERS_0_10 {
    pub Flags: CF_CALLBACK_RENAME_FLAGS,
    pub TargetPath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CF_CALLBACK_PARAMETERS_0_10 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_10 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CF_CALLBACK_PARAMETERS_0_10 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Rename_e__Struct").field("Flags", &self.Flags).field("TargetPath", &self.TargetPath).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_10 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.TargetPath == other.TargetPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_10 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_10 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_CALLBACK_PARAMETERS_0_11 {
    pub Flags: CF_CALLBACK_VALIDATE_DATA_FLAGS,
    pub RequiredFileOffset: i64,
    pub RequiredLength: i64,
}
impl CF_CALLBACK_PARAMETERS_0_11 {}
impl ::std::default::Default for CF_CALLBACK_PARAMETERS_0_11 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_CALLBACK_PARAMETERS_0_11 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ValidateData_e__Struct").field("Flags", &self.Flags).field("RequiredFileOffset", &self.RequiredFileOffset).field("RequiredLength", &self.RequiredLength).finish()
    }
}
impl ::std::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_11 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.RequiredFileOffset == other.RequiredFileOffset && self.RequiredLength == other.RequiredLength
    }
}
impl ::std::cmp::Eq for CF_CALLBACK_PARAMETERS_0_11 {}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_PARAMETERS_0_11 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct CF_CALLBACK_REGISTRATION {
    pub Type: CF_CALLBACK_TYPE,
    pub Callback: ::std::option::Option<CF_CALLBACK>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl CF_CALLBACK_REGISTRATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for CF_CALLBACK_REGISTRATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for CF_CALLBACK_REGISTRATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_CALLBACK_REGISTRATION").field("Type", &self.Type).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for CF_CALLBACK_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Callback.map(|f| f as usize) == other.Callback.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for CF_CALLBACK_REGISTRATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_REGISTRATION {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_RENAME_COMPLETION_FLAGS(pub u32);
pub const CF_CALLBACK_RENAME_COMPLETION_FLAG_NONE: CF_CALLBACK_RENAME_COMPLETION_FLAGS = CF_CALLBACK_RENAME_COMPLETION_FLAGS(0u32);
impl ::std::convert::From<u32> for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_RENAME_FLAGS(pub u32);
pub const CF_CALLBACK_RENAME_FLAG_NONE: CF_CALLBACK_RENAME_FLAGS = CF_CALLBACK_RENAME_FLAGS(0u32);
pub const CF_CALLBACK_RENAME_FLAG_IS_DIRECTORY: CF_CALLBACK_RENAME_FLAGS = CF_CALLBACK_RENAME_FLAGS(1u32);
pub const CF_CALLBACK_RENAME_FLAG_SOURCE_IN_SCOPE: CF_CALLBACK_RENAME_FLAGS = CF_CALLBACK_RENAME_FLAGS(2u32);
pub const CF_CALLBACK_RENAME_FLAG_TARGET_IN_SCOPE: CF_CALLBACK_RENAME_FLAGS = CF_CALLBACK_RENAME_FLAGS(4u32);
impl ::std::convert::From<u32> for CF_CALLBACK_RENAME_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_RENAME_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CALLBACK_RENAME_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CALLBACK_RENAME_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CALLBACK_RENAME_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CALLBACK_RENAME_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CALLBACK_RENAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_TYPE(pub i32);
pub const CF_CALLBACK_TYPE_FETCH_DATA: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(0i32);
pub const CF_CALLBACK_TYPE_VALIDATE_DATA: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(1i32);
pub const CF_CALLBACK_TYPE_CANCEL_FETCH_DATA: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(2i32);
pub const CF_CALLBACK_TYPE_FETCH_PLACEHOLDERS: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(3i32);
pub const CF_CALLBACK_TYPE_CANCEL_FETCH_PLACEHOLDERS: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(4i32);
pub const CF_CALLBACK_TYPE_NOTIFY_FILE_OPEN_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(5i32);
pub const CF_CALLBACK_TYPE_NOTIFY_FILE_CLOSE_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(6i32);
pub const CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(7i32);
pub const CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(8i32);
pub const CF_CALLBACK_TYPE_NOTIFY_DELETE: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(9i32);
pub const CF_CALLBACK_TYPE_NOTIFY_DELETE_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(10i32);
pub const CF_CALLBACK_TYPE_NOTIFY_RENAME: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(11i32);
pub const CF_CALLBACK_TYPE_NOTIFY_RENAME_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(12i32);
pub const CF_CALLBACK_TYPE_NONE: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(-1i32);
impl ::std::convert::From<i32> for CF_CALLBACK_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CALLBACK_VALIDATE_DATA_FLAGS(pub u32);
pub const CF_CALLBACK_VALIDATE_DATA_FLAG_NONE: CF_CALLBACK_VALIDATE_DATA_FLAGS = CF_CALLBACK_VALIDATE_DATA_FLAGS(0u32);
pub const CF_CALLBACK_VALIDATE_DATA_FLAG_EXPLICIT_HYDRATION: CF_CALLBACK_VALIDATE_DATA_FLAGS = CF_CALLBACK_VALIDATE_DATA_FLAGS(2u32);
impl ::std::convert::From<u32> for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct CF_CONNECTION_KEY(pub isize);
impl ::std::default::Default for CF_CONNECTION_KEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for CF_CONNECTION_KEY {}
unsafe impl ::windows::runtime::Abi for CF_CONNECTION_KEY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CONNECT_FLAGS(pub u32);
pub const CF_CONNECT_FLAG_NONE: CF_CONNECT_FLAGS = CF_CONNECT_FLAGS(0u32);
pub const CF_CONNECT_FLAG_REQUIRE_PROCESS_INFO: CF_CONNECT_FLAGS = CF_CONNECT_FLAGS(2u32);
pub const CF_CONNECT_FLAG_REQUIRE_FULL_FILE_PATH: CF_CONNECT_FLAGS = CF_CONNECT_FLAGS(4u32);
pub const CF_CONNECT_FLAG_BLOCK_SELF_IMPLICIT_HYDRATION: CF_CONNECT_FLAGS = CF_CONNECT_FLAGS(8u32);
impl ::std::convert::From<u32> for CF_CONNECT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CONNECT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CONNECT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CONNECT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CONNECT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CONNECT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CONNECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CONVERT_FLAGS(pub u32);
pub const CF_CONVERT_FLAG_NONE: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(0u32);
pub const CF_CONVERT_FLAG_MARK_IN_SYNC: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(1u32);
pub const CF_CONVERT_FLAG_DEHYDRATE: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(2u32);
pub const CF_CONVERT_FLAG_ENABLE_ON_DEMAND_POPULATION: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(4u32);
pub const CF_CONVERT_FLAG_ALWAYS_FULL: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(8u32);
pub const CF_CONVERT_FLAG_FORCE_CONVERT_TO_CLOUD_FILE: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(16u32);
impl ::std::convert::From<u32> for CF_CONVERT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CONVERT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CONVERT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CONVERT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CONVERT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CONVERT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CONVERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_CREATE_FLAGS(pub u32);
pub const CF_CREATE_FLAG_NONE: CF_CREATE_FLAGS = CF_CREATE_FLAGS(0u32);
pub const CF_CREATE_FLAG_STOP_ON_ERROR: CF_CREATE_FLAGS = CF_CREATE_FLAGS(1u32);
impl ::std::convert::From<u32> for CF_CREATE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_CREATE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_CREATE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_CREATE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_DEHYDRATE_FLAGS(pub u32);
pub const CF_DEHYDRATE_FLAG_NONE: CF_DEHYDRATE_FLAGS = CF_DEHYDRATE_FLAGS(0u32);
pub const CF_DEHYDRATE_FLAG_BACKGROUND: CF_DEHYDRATE_FLAGS = CF_DEHYDRATE_FLAGS(1u32);
impl ::std::convert::From<u32> for CF_DEHYDRATE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_DEHYDRATE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_DEHYDRATE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_DEHYDRATE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_DEHYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_FILE_RANGE {
    pub StartingOffset: i64,
    pub Length: i64,
}
impl CF_FILE_RANGE {}
impl ::std::default::Default for CF_FILE_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_FILE_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_FILE_RANGE").field("StartingOffset", &self.StartingOffset).field("Length", &self.Length).finish()
    }
}
impl ::std::cmp::PartialEq for CF_FILE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartingOffset == other.StartingOffset && self.Length == other.Length
    }
}
impl ::std::cmp::Eq for CF_FILE_RANGE {}
unsafe impl ::windows::runtime::Abi for CF_FILE_RANGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct CF_FS_METADATA {
    pub BasicInfo: super::FileSystem::FILE_BASIC_INFO,
    pub FileSize: i64,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl CF_FS_METADATA {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::std::default::Default for CF_FS_METADATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::std::fmt::Debug for CF_FS_METADATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_FS_METADATA").field("BasicInfo", &self.BasicInfo).field("FileSize", &self.FileSize).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::std::cmp::PartialEq for CF_FS_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.BasicInfo == other.BasicInfo && self.FileSize == other.FileSize
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::std::cmp::Eq for CF_FS_METADATA {}
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::runtime::Abi for CF_FS_METADATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_HARDLINK_POLICY(pub u32);
pub const CF_HARDLINK_POLICY_NONE: CF_HARDLINK_POLICY = CF_HARDLINK_POLICY(0u32);
pub const CF_HARDLINK_POLICY_ALLOWED: CF_HARDLINK_POLICY = CF_HARDLINK_POLICY(1u32);
impl ::std::convert::From<u32> for CF_HARDLINK_POLICY {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_HARDLINK_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_HARDLINK_POLICY {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_HARDLINK_POLICY {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_HARDLINK_POLICY {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_HARDLINK_POLICY {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_HARDLINK_POLICY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_HYDRATE_FLAGS(pub u32);
pub const CF_HYDRATE_FLAG_NONE: CF_HYDRATE_FLAGS = CF_HYDRATE_FLAGS(0u32);
impl ::std::convert::From<u32> for CF_HYDRATE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_HYDRATE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_HYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_HYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_HYDRATE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_HYDRATE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_HYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_HYDRATION_POLICY {
    pub Primary: CF_HYDRATION_POLICY_PRIMARY_USHORT,
    pub Modifier: CF_HYDRATION_POLICY_MODIFIER_USHORT,
}
impl CF_HYDRATION_POLICY {}
impl ::std::default::Default for CF_HYDRATION_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_HYDRATION_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_HYDRATION_POLICY").field("Primary", &self.Primary).field("Modifier", &self.Modifier).finish()
    }
}
impl ::std::cmp::PartialEq for CF_HYDRATION_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Primary == other.Primary && self.Modifier == other.Modifier
    }
}
impl ::std::cmp::Eq for CF_HYDRATION_POLICY {}
unsafe impl ::windows::runtime::Abi for CF_HYDRATION_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_HYDRATION_POLICY_MODIFIER(pub u32);
pub const CF_HYDRATION_POLICY_MODIFIER_NONE: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(0u32);
pub const CF_HYDRATION_POLICY_MODIFIER_VALIDATION_REQUIRED: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(1u32);
pub const CF_HYDRATION_POLICY_MODIFIER_STREAMING_ALLOWED: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(2u32);
pub const CF_HYDRATION_POLICY_MODIFIER_AUTO_DEHYDRATION_ALLOWED: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(4u32);
pub const CF_HYDRATION_POLICY_MODIFIER_ALLOW_FULL_RESTART_HYDRATION: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(8u32);
impl ::std::convert::From<u32> for CF_HYDRATION_POLICY_MODIFIER {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_HYDRATION_POLICY_MODIFIER {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_HYDRATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_HYDRATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_HYDRATION_POLICY_MODIFIER {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_HYDRATION_POLICY_MODIFIER {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_HYDRATION_POLICY_MODIFIER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_HYDRATION_POLICY_MODIFIER_USHORT {
    pub us: u16,
}
impl CF_HYDRATION_POLICY_MODIFIER_USHORT {}
impl ::std::default::Default for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_HYDRATION_POLICY_MODIFIER_USHORT").field("us", &self.us).finish()
    }
}
impl ::std::cmp::PartialEq for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    fn eq(&self, other: &Self) -> bool {
        self.us == other.us
    }
}
impl ::std::cmp::Eq for CF_HYDRATION_POLICY_MODIFIER_USHORT {}
unsafe impl ::windows::runtime::Abi for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_HYDRATION_POLICY_PRIMARY(pub i32);
pub const CF_HYDRATION_POLICY_PARTIAL: CF_HYDRATION_POLICY_PRIMARY = CF_HYDRATION_POLICY_PRIMARY(0i32);
pub const CF_HYDRATION_POLICY_PROGRESSIVE: CF_HYDRATION_POLICY_PRIMARY = CF_HYDRATION_POLICY_PRIMARY(1i32);
pub const CF_HYDRATION_POLICY_FULL: CF_HYDRATION_POLICY_PRIMARY = CF_HYDRATION_POLICY_PRIMARY(2i32);
pub const CF_HYDRATION_POLICY_ALWAYS_FULL: CF_HYDRATION_POLICY_PRIMARY = CF_HYDRATION_POLICY_PRIMARY(3i32);
impl ::std::convert::From<i32> for CF_HYDRATION_POLICY_PRIMARY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_HYDRATION_POLICY_PRIMARY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_HYDRATION_POLICY_PRIMARY_USHORT {
    pub us: u16,
}
impl CF_HYDRATION_POLICY_PRIMARY_USHORT {}
impl ::std::default::Default for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_HYDRATION_POLICY_PRIMARY_USHORT").field("us", &self.us).finish()
    }
}
impl ::std::cmp::PartialEq for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    fn eq(&self, other: &Self) -> bool {
        self.us == other.us
    }
}
impl ::std::cmp::Eq for CF_HYDRATION_POLICY_PRIMARY_USHORT {}
unsafe impl ::windows::runtime::Abi for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_INSYNC_POLICY(pub u32);
pub const CF_INSYNC_POLICY_NONE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(0u32);
pub const CF_INSYNC_POLICY_TRACK_FILE_CREATION_TIME: CF_INSYNC_POLICY = CF_INSYNC_POLICY(1u32);
pub const CF_INSYNC_POLICY_TRACK_FILE_READONLY_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(2u32);
pub const CF_INSYNC_POLICY_TRACK_FILE_HIDDEN_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(4u32);
pub const CF_INSYNC_POLICY_TRACK_FILE_SYSTEM_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(8u32);
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_CREATION_TIME: CF_INSYNC_POLICY = CF_INSYNC_POLICY(16u32);
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_READONLY_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(32u32);
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_HIDDEN_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(64u32);
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_SYSTEM_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(128u32);
pub const CF_INSYNC_POLICY_TRACK_FILE_LAST_WRITE_TIME: CF_INSYNC_POLICY = CF_INSYNC_POLICY(256u32);
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_LAST_WRITE_TIME: CF_INSYNC_POLICY = CF_INSYNC_POLICY(512u32);
pub const CF_INSYNC_POLICY_TRACK_FILE_ALL: CF_INSYNC_POLICY = CF_INSYNC_POLICY(5592335u32);
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_ALL: CF_INSYNC_POLICY = CF_INSYNC_POLICY(11184880u32);
pub const CF_INSYNC_POLICY_TRACK_ALL: CF_INSYNC_POLICY = CF_INSYNC_POLICY(16777215u32);
pub const CF_INSYNC_POLICY_PRESERVE_INSYNC_FOR_SYNC_ENGINE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(2147483648u32);
impl ::std::convert::From<u32> for CF_INSYNC_POLICY {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_INSYNC_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_INSYNC_POLICY {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_INSYNC_POLICY {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_INSYNC_POLICY {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_INSYNC_POLICY {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_INSYNC_POLICY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_IN_SYNC_STATE(pub i32);
pub const CF_IN_SYNC_STATE_NOT_IN_SYNC: CF_IN_SYNC_STATE = CF_IN_SYNC_STATE(0i32);
pub const CF_IN_SYNC_STATE_IN_SYNC: CF_IN_SYNC_STATE = CF_IN_SYNC_STATE(1i32);
impl ::std::convert::From<i32> for CF_IN_SYNC_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_IN_SYNC_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CF_MAX_PRIORITY_HINT: u32 = 15u32;
pub const CF_MAX_PROVIDER_NAME_LENGTH: u32 = 255u32;
pub const CF_MAX_PROVIDER_VERSION_LENGTH: u32 = 255u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_OPEN_FILE_FLAGS(pub u32);
pub const CF_OPEN_FILE_FLAG_NONE: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(0u32);
pub const CF_OPEN_FILE_FLAG_EXCLUSIVE: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(1u32);
pub const CF_OPEN_FILE_FLAG_WRITE_ACCESS: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(2u32);
pub const CF_OPEN_FILE_FLAG_DELETE_ACCESS: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(4u32);
pub const CF_OPEN_FILE_FLAG_FOREGROUND: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(8u32);
impl ::std::convert::From<u32> for CF_OPEN_FILE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_OPEN_FILE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_OPEN_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_OPEN_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_OPEN_FILE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_OPEN_FILE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_OPEN_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_OPERATION_ACK_DATA_FLAGS(pub u32);
pub const CF_OPERATION_ACK_DATA_FLAG_NONE: CF_OPERATION_ACK_DATA_FLAGS = CF_OPERATION_ACK_DATA_FLAGS(0u32);
impl ::std::convert::From<u32> for CF_OPERATION_ACK_DATA_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_OPERATION_ACK_DATA_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_OPERATION_ACK_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_OPERATION_ACK_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_OPERATION_ACK_DATA_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_OPERATION_ACK_DATA_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_OPERATION_ACK_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_OPERATION_ACK_DEHYDRATE_FLAGS(pub u32);
pub const CF_OPERATION_ACK_DEHYDRATE_FLAG_NONE: CF_OPERATION_ACK_DEHYDRATE_FLAGS = CF_OPERATION_ACK_DEHYDRATE_FLAGS(0u32);
impl ::std::convert::From<u32> for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_OPERATION_ACK_DELETE_FLAGS(pub u32);
pub const CF_OPERATION_ACK_DELETE_FLAG_NONE: CF_OPERATION_ACK_DELETE_FLAGS = CF_OPERATION_ACK_DELETE_FLAGS(0u32);
impl ::std::convert::From<u32> for CF_OPERATION_ACK_DELETE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_OPERATION_ACK_DELETE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_OPERATION_ACK_DELETE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_OPERATION_ACK_DELETE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_OPERATION_ACK_DELETE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_OPERATION_ACK_DELETE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_OPERATION_ACK_DELETE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_OPERATION_ACK_RENAME_FLAGS(pub u32);
pub const CF_OPERATION_ACK_RENAME_FLAG_NONE: CF_OPERATION_ACK_RENAME_FLAGS = CF_OPERATION_ACK_RENAME_FLAGS(0u32);
impl ::std::convert::From<u32> for CF_OPERATION_ACK_RENAME_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_OPERATION_ACK_RENAME_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_OPERATION_ACK_RENAME_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_OPERATION_ACK_RENAME_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_OPERATION_ACK_RENAME_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_OPERATION_ACK_RENAME_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_OPERATION_ACK_RENAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct CF_OPERATION_INFO {
    pub StructSize: u32,
    pub Type: CF_OPERATION_TYPE,
    pub ConnectionKey: CF_CONNECTION_KEY,
    pub TransferKey: i64,
    pub CorrelationVector: *mut super::super::System::SystemServices::CORRELATION_VECTOR,
    pub SyncStatus: *mut CF_SYNC_STATUS,
    pub RequestKey: i64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl CF_OPERATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for CF_OPERATION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for CF_OPERATION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_OPERATION_INFO")
            .field("StructSize", &self.StructSize)
            .field("Type", &self.Type)
            .field("ConnectionKey", &self.ConnectionKey)
            .field("TransferKey", &self.TransferKey)
            .field("CorrelationVector", &self.CorrelationVector)
            .field("SyncStatus", &self.SyncStatus)
            .field("RequestKey", &self.RequestKey)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for CF_OPERATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StructSize == other.StructSize && self.Type == other.Type && self.ConnectionKey == other.ConnectionKey && self.TransferKey == other.TransferKey && self.CorrelationVector == other.CorrelationVector && self.SyncStatus == other.SyncStatus && self.RequestKey == other.RequestKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for CF_OPERATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for CF_OPERATION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_OPERATION_PARAMETERS {
    pub ParamSize: u32,
    pub Anonymous: CF_OPERATION_PARAMETERS_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl CF_OPERATION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::default::Default for CF_OPERATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::cmp::PartialEq for CF_OPERATION_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::cmp::Eq for CF_OPERATION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::runtime::Abi for CF_OPERATION_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl CF_OPERATION_PARAMETERS_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::default::Default for CF_OPERATION_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::cmp::PartialEq for CF_OPERATION_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::cmp::Eq for CF_OPERATION_PARAMETERS_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::runtime::Abi for CF_OPERATION_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_OPERATION_PARAMETERS_0_0 {
    pub Flags: CF_OPERATION_ACK_DATA_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
    pub Offset: i64,
    pub Length: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl CF_OPERATION_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CF_OPERATION_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CF_OPERATION_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_AckData_e__Struct").field("Flags", &self.Flags).field("CompletionStatus", &self.CompletionStatus).field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.CompletionStatus == other.CompletionStatus && self.Offset == other.Offset && self.Length == other.Length
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CF_OPERATION_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CF_OPERATION_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_OPERATION_PARAMETERS_0_1 {
    pub Flags: CF_OPERATION_ACK_DEHYDRATE_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
    pub FileIdentity: *mut ::std::ffi::c_void,
    pub FileIdentityLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CF_OPERATION_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CF_OPERATION_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CF_OPERATION_PARAMETERS_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_AckDehydrate_e__Struct").field("Flags", &self.Flags).field("CompletionStatus", &self.CompletionStatus).field("FileIdentity", &self.FileIdentity).field("FileIdentityLength", &self.FileIdentityLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.CompletionStatus == other.CompletionStatus && self.FileIdentity == other.FileIdentity && self.FileIdentityLength == other.FileIdentityLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CF_OPERATION_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CF_OPERATION_PARAMETERS_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_OPERATION_PARAMETERS_0_2 {
    pub Flags: CF_OPERATION_ACK_DELETE_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl CF_OPERATION_PARAMETERS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CF_OPERATION_PARAMETERS_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CF_OPERATION_PARAMETERS_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_AckDelete_e__Struct").field("Flags", &self.Flags).field("CompletionStatus", &self.CompletionStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.CompletionStatus == other.CompletionStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CF_OPERATION_PARAMETERS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CF_OPERATION_PARAMETERS_0_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_OPERATION_PARAMETERS_0_3 {
    pub Flags: CF_OPERATION_ACK_RENAME_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl CF_OPERATION_PARAMETERS_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CF_OPERATION_PARAMETERS_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CF_OPERATION_PARAMETERS_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_AckRename_e__Struct").field("Flags", &self.Flags).field("CompletionStatus", &self.CompletionStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.CompletionStatus == other.CompletionStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CF_OPERATION_PARAMETERS_0_3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CF_OPERATION_PARAMETERS_0_3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct CF_OPERATION_PARAMETERS_0_4 {
    pub Flags: CF_OPERATION_RESTART_HYDRATION_FLAGS,
    pub FsMetadata: *mut CF_FS_METADATA,
    pub FileIdentity: *mut ::std::ffi::c_void,
    pub FileIdentityLength: u32,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl CF_OPERATION_PARAMETERS_0_4 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::std::default::Default for CF_OPERATION_PARAMETERS_0_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::std::fmt::Debug for CF_OPERATION_PARAMETERS_0_4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_RestartHydration_e__Struct").field("Flags", &self.Flags).field("FsMetadata", &self.FsMetadata).field("FileIdentity", &self.FileIdentity).field("FileIdentityLength", &self.FileIdentityLength).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::std::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.FsMetadata == other.FsMetadata && self.FileIdentity == other.FileIdentity && self.FileIdentityLength == other.FileIdentityLength
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::std::cmp::Eq for CF_OPERATION_PARAMETERS_0_4 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::runtime::Abi for CF_OPERATION_PARAMETERS_0_4 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_OPERATION_PARAMETERS_0_5 {
    pub Flags: CF_OPERATION_RETRIEVE_DATA_FLAGS,
    pub Buffer: *mut ::std::ffi::c_void,
    pub Offset: i64,
    pub Length: i64,
    pub ReturnedLength: i64,
}
impl CF_OPERATION_PARAMETERS_0_5 {}
impl ::std::default::Default for CF_OPERATION_PARAMETERS_0_5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_OPERATION_PARAMETERS_0_5 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_RetrieveData_e__Struct").field("Flags", &self.Flags).field("Buffer", &self.Buffer).field("Offset", &self.Offset).field("Length", &self.Length).field("ReturnedLength", &self.ReturnedLength).finish()
    }
}
impl ::std::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Buffer == other.Buffer && self.Offset == other.Offset && self.Length == other.Length && self.ReturnedLength == other.ReturnedLength
    }
}
impl ::std::cmp::Eq for CF_OPERATION_PARAMETERS_0_5 {}
unsafe impl ::windows::runtime::Abi for CF_OPERATION_PARAMETERS_0_5 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_OPERATION_PARAMETERS_0_6 {
    pub Flags: CF_OPERATION_TRANSFER_DATA_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
    pub Buffer: *mut ::std::ffi::c_void,
    pub Offset: i64,
    pub Length: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl CF_OPERATION_PARAMETERS_0_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CF_OPERATION_PARAMETERS_0_6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CF_OPERATION_PARAMETERS_0_6 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_TransferData_e__Struct").field("Flags", &self.Flags).field("CompletionStatus", &self.CompletionStatus).field("Buffer", &self.Buffer).field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.CompletionStatus == other.CompletionStatus && self.Buffer == other.Buffer && self.Offset == other.Offset && self.Length == other.Length
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CF_OPERATION_PARAMETERS_0_6 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CF_OPERATION_PARAMETERS_0_6 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl CF_OPERATION_PARAMETERS_0_7 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::default::Default for CF_OPERATION_PARAMETERS_0_7 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::fmt::Debug for CF_OPERATION_PARAMETERS_0_7 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_TransferPlaceholders_e__Struct")
            .field("Flags", &self.Flags)
            .field("CompletionStatus", &self.CompletionStatus)
            .field("PlaceholderTotalCount", &self.PlaceholderTotalCount)
            .field("PlaceholderArray", &self.PlaceholderArray)
            .field("PlaceholderCount", &self.PlaceholderCount)
            .field("EntriesProcessed", &self.EntriesProcessed)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_7 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.CompletionStatus == other.CompletionStatus && self.PlaceholderTotalCount == other.PlaceholderTotalCount && self.PlaceholderArray == other.PlaceholderArray && self.PlaceholderCount == other.PlaceholderCount && self.EntriesProcessed == other.EntriesProcessed
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::cmp::Eq for CF_OPERATION_PARAMETERS_0_7 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::runtime::Abi for CF_OPERATION_PARAMETERS_0_7 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_OPERATION_RESTART_HYDRATION_FLAGS(pub u32);
pub const CF_OPERATION_RESTART_HYDRATION_FLAG_NONE: CF_OPERATION_RESTART_HYDRATION_FLAGS = CF_OPERATION_RESTART_HYDRATION_FLAGS(0u32);
pub const CF_OPERATION_RESTART_HYDRATION_FLAG_MARK_IN_SYNC: CF_OPERATION_RESTART_HYDRATION_FLAGS = CF_OPERATION_RESTART_HYDRATION_FLAGS(1u32);
impl ::std::convert::From<u32> for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_OPERATION_RETRIEVE_DATA_FLAGS(pub u32);
pub const CF_OPERATION_RETRIEVE_DATA_FLAG_NONE: CF_OPERATION_RETRIEVE_DATA_FLAGS = CF_OPERATION_RETRIEVE_DATA_FLAGS(0u32);
impl ::std::convert::From<u32> for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_OPERATION_TRANSFER_DATA_FLAGS(pub u32);
pub const CF_OPERATION_TRANSFER_DATA_FLAG_NONE: CF_OPERATION_TRANSFER_DATA_FLAGS = CF_OPERATION_TRANSFER_DATA_FLAGS(0u32);
impl ::std::convert::From<u32> for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS(pub u32);
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_NONE: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS(0u32);
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_STOP_ON_ERROR: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS(1u32);
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS(2u32);
impl ::std::convert::From<u32> for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_OPERATION_TYPE(pub i32);
pub const CF_OPERATION_TYPE_TRANSFER_DATA: CF_OPERATION_TYPE = CF_OPERATION_TYPE(0i32);
pub const CF_OPERATION_TYPE_RETRIEVE_DATA: CF_OPERATION_TYPE = CF_OPERATION_TYPE(1i32);
pub const CF_OPERATION_TYPE_ACK_DATA: CF_OPERATION_TYPE = CF_OPERATION_TYPE(2i32);
pub const CF_OPERATION_TYPE_RESTART_HYDRATION: CF_OPERATION_TYPE = CF_OPERATION_TYPE(3i32);
pub const CF_OPERATION_TYPE_TRANSFER_PLACEHOLDERS: CF_OPERATION_TYPE = CF_OPERATION_TYPE(4i32);
pub const CF_OPERATION_TYPE_ACK_DEHYDRATE: CF_OPERATION_TYPE = CF_OPERATION_TYPE(5i32);
pub const CF_OPERATION_TYPE_ACK_DELETE: CF_OPERATION_TYPE = CF_OPERATION_TYPE(6i32);
pub const CF_OPERATION_TYPE_ACK_RENAME: CF_OPERATION_TYPE = CF_OPERATION_TYPE(7i32);
impl ::std::convert::From<i32> for CF_OPERATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_OPERATION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_PIN_STATE(pub i32);
pub const CF_PIN_STATE_UNSPECIFIED: CF_PIN_STATE = CF_PIN_STATE(0i32);
pub const CF_PIN_STATE_PINNED: CF_PIN_STATE = CF_PIN_STATE(1i32);
pub const CF_PIN_STATE_UNPINNED: CF_PIN_STATE = CF_PIN_STATE(2i32);
pub const CF_PIN_STATE_EXCLUDED: CF_PIN_STATE = CF_PIN_STATE(3i32);
pub const CF_PIN_STATE_INHERIT: CF_PIN_STATE = CF_PIN_STATE(4i32);
impl ::std::convert::From<i32> for CF_PIN_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_PIN_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_PLACEHOLDER_BASIC_INFO {
    pub PinState: CF_PIN_STATE,
    pub InSyncState: CF_IN_SYNC_STATE,
    pub FileId: i64,
    pub SyncRootFileId: i64,
    pub FileIdentityLength: u32,
    pub FileIdentity: [u8; 1],
}
impl CF_PLACEHOLDER_BASIC_INFO {}
impl ::std::default::Default for CF_PLACEHOLDER_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_PLACEHOLDER_BASIC_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_PLACEHOLDER_BASIC_INFO").field("PinState", &self.PinState).field("InSyncState", &self.InSyncState).field("FileId", &self.FileId).field("SyncRootFileId", &self.SyncRootFileId).field("FileIdentityLength", &self.FileIdentityLength).field("FileIdentity", &self.FileIdentity).finish()
    }
}
impl ::std::cmp::PartialEq for CF_PLACEHOLDER_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PinState == other.PinState && self.InSyncState == other.InSyncState && self.FileId == other.FileId && self.SyncRootFileId == other.SyncRootFileId && self.FileIdentityLength == other.FileIdentityLength && self.FileIdentity == other.FileIdentity
    }
}
impl ::std::cmp::Eq for CF_PLACEHOLDER_BASIC_INFO {}
unsafe impl ::windows::runtime::Abi for CF_PLACEHOLDER_BASIC_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_PLACEHOLDER_CREATE_FLAGS(pub u32);
pub const CF_PLACEHOLDER_CREATE_FLAG_NONE: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(0u32);
pub const CF_PLACEHOLDER_CREATE_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(1u32);
pub const CF_PLACEHOLDER_CREATE_FLAG_MARK_IN_SYNC: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(2u32);
pub const CF_PLACEHOLDER_CREATE_FLAG_SUPERSEDE: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(4u32);
pub const CF_PLACEHOLDER_CREATE_FLAG_ALWAYS_FULL: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(8u32);
impl ::std::convert::From<u32> for CF_PLACEHOLDER_CREATE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_PLACEHOLDER_CREATE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_PLACEHOLDER_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_PLACEHOLDER_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_PLACEHOLDER_CREATE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_PLACEHOLDER_CREATE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_PLACEHOLDER_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_PLACEHOLDER_CREATE_INFO {
    pub RelativeFileName: super::super::Foundation::PWSTR,
    pub FsMetadata: CF_FS_METADATA,
    pub FileIdentity: *mut ::std::ffi::c_void,
    pub FileIdentityLength: u32,
    pub Flags: CF_PLACEHOLDER_CREATE_FLAGS,
    pub Result: ::windows::runtime::HRESULT,
    pub CreateUsn: i64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl CF_PLACEHOLDER_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::default::Default for CF_PLACEHOLDER_CREATE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::fmt::Debug for CF_PLACEHOLDER_CREATE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_PLACEHOLDER_CREATE_INFO")
            .field("RelativeFileName", &self.RelativeFileName)
            .field("FsMetadata", &self.FsMetadata)
            .field("FileIdentity", &self.FileIdentity)
            .field("FileIdentityLength", &self.FileIdentityLength)
            .field("Flags", &self.Flags)
            .field("Result", &self.Result)
            .field("CreateUsn", &self.CreateUsn)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::cmp::PartialEq for CF_PLACEHOLDER_CREATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RelativeFileName == other.RelativeFileName && self.FsMetadata == other.FsMetadata && self.FileIdentity == other.FileIdentity && self.FileIdentityLength == other.FileIdentityLength && self.Flags == other.Flags && self.Result == other.Result && self.CreateUsn == other.CreateUsn
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::cmp::Eq for CF_PLACEHOLDER_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::runtime::Abi for CF_PLACEHOLDER_CREATE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_PLACEHOLDER_INFO_CLASS(pub i32);
pub const CF_PLACEHOLDER_INFO_BASIC: CF_PLACEHOLDER_INFO_CLASS = CF_PLACEHOLDER_INFO_CLASS(0i32);
pub const CF_PLACEHOLDER_INFO_STANDARD: CF_PLACEHOLDER_INFO_CLASS = CF_PLACEHOLDER_INFO_CLASS(1i32);
impl ::std::convert::From<i32> for CF_PLACEHOLDER_INFO_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_PLACEHOLDER_INFO_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_PLACEHOLDER_MANAGEMENT_POLICY(pub i32);
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_DEFAULT: CF_PLACEHOLDER_MANAGEMENT_POLICY = CF_PLACEHOLDER_MANAGEMENT_POLICY(0i32);
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_CREATE_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = CF_PLACEHOLDER_MANAGEMENT_POLICY(1i32);
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_CONVERT_TO_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = CF_PLACEHOLDER_MANAGEMENT_POLICY(2i32);
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_UPDATE_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = CF_PLACEHOLDER_MANAGEMENT_POLICY(4i32);
impl ::std::convert::From<i32> for CF_PLACEHOLDER_MANAGEMENT_POLICY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_PLACEHOLDER_MANAGEMENT_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CF_PLACEHOLDER_MAX_FILE_IDENTITY_LENGTH: u32 = 4096u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_PLACEHOLDER_RANGE_INFO_CLASS(pub i32);
pub const CF_PLACEHOLDER_RANGE_INFO_ONDISK: CF_PLACEHOLDER_RANGE_INFO_CLASS = CF_PLACEHOLDER_RANGE_INFO_CLASS(1i32);
pub const CF_PLACEHOLDER_RANGE_INFO_VALIDATED: CF_PLACEHOLDER_RANGE_INFO_CLASS = CF_PLACEHOLDER_RANGE_INFO_CLASS(2i32);
pub const CF_PLACEHOLDER_RANGE_INFO_MODIFIED: CF_PLACEHOLDER_RANGE_INFO_CLASS = CF_PLACEHOLDER_RANGE_INFO_CLASS(3i32);
impl ::std::convert::From<i32> for CF_PLACEHOLDER_RANGE_INFO_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_PLACEHOLDER_RANGE_INFO_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl CF_PLACEHOLDER_STANDARD_INFO {}
impl ::std::default::Default for CF_PLACEHOLDER_STANDARD_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_PLACEHOLDER_STANDARD_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_PLACEHOLDER_STANDARD_INFO")
            .field("OnDiskDataSize", &self.OnDiskDataSize)
            .field("ValidatedDataSize", &self.ValidatedDataSize)
            .field("ModifiedDataSize", &self.ModifiedDataSize)
            .field("PropertiesSize", &self.PropertiesSize)
            .field("PinState", &self.PinState)
            .field("InSyncState", &self.InSyncState)
            .field("FileId", &self.FileId)
            .field("SyncRootFileId", &self.SyncRootFileId)
            .field("FileIdentityLength", &self.FileIdentityLength)
            .field("FileIdentity", &self.FileIdentity)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CF_PLACEHOLDER_STANDARD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.OnDiskDataSize == other.OnDiskDataSize && self.ValidatedDataSize == other.ValidatedDataSize && self.ModifiedDataSize == other.ModifiedDataSize && self.PropertiesSize == other.PropertiesSize && self.PinState == other.PinState && self.InSyncState == other.InSyncState && self.FileId == other.FileId && self.SyncRootFileId == other.SyncRootFileId && self.FileIdentityLength == other.FileIdentityLength && self.FileIdentity == other.FileIdentity
    }
}
impl ::std::cmp::Eq for CF_PLACEHOLDER_STANDARD_INFO {}
unsafe impl ::windows::runtime::Abi for CF_PLACEHOLDER_STANDARD_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_PLACEHOLDER_STATE(pub u32);
pub const CF_PLACEHOLDER_STATE_NO_STATES: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(0u32);
pub const CF_PLACEHOLDER_STATE_PLACEHOLDER: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(1u32);
pub const CF_PLACEHOLDER_STATE_SYNC_ROOT: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(2u32);
pub const CF_PLACEHOLDER_STATE_ESSENTIAL_PROP_PRESENT: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(4u32);
pub const CF_PLACEHOLDER_STATE_IN_SYNC: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(8u32);
pub const CF_PLACEHOLDER_STATE_PARTIAL: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(16u32);
pub const CF_PLACEHOLDER_STATE_PARTIALLY_ON_DISK: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(32u32);
pub const CF_PLACEHOLDER_STATE_INVALID: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(4294967295u32);
impl ::std::convert::From<u32> for CF_PLACEHOLDER_STATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_PLACEHOLDER_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_PLACEHOLDER_STATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_PLACEHOLDER_STATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_PLACEHOLDER_STATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_PLACEHOLDER_STATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_PLACEHOLDER_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_PLATFORM_INFO {
    pub BuildNumber: u32,
    pub RevisionNumber: u32,
    pub IntegrationNumber: u32,
}
impl CF_PLATFORM_INFO {}
impl ::std::default::Default for CF_PLATFORM_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_PLATFORM_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_PLATFORM_INFO").field("BuildNumber", &self.BuildNumber).field("RevisionNumber", &self.RevisionNumber).field("IntegrationNumber", &self.IntegrationNumber).finish()
    }
}
impl ::std::cmp::PartialEq for CF_PLATFORM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.BuildNumber == other.BuildNumber && self.RevisionNumber == other.RevisionNumber && self.IntegrationNumber == other.IntegrationNumber
    }
}
impl ::std::cmp::Eq for CF_PLATFORM_INFO {}
unsafe impl ::windows::runtime::Abi for CF_PLATFORM_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_POPULATION_POLICY {
    pub Primary: CF_POPULATION_POLICY_PRIMARY_USHORT,
    pub Modifier: CF_POPULATION_POLICY_MODIFIER_USHORT,
}
impl CF_POPULATION_POLICY {}
impl ::std::default::Default for CF_POPULATION_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_POPULATION_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_POPULATION_POLICY").field("Primary", &self.Primary).field("Modifier", &self.Modifier).finish()
    }
}
impl ::std::cmp::PartialEq for CF_POPULATION_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Primary == other.Primary && self.Modifier == other.Modifier
    }
}
impl ::std::cmp::Eq for CF_POPULATION_POLICY {}
unsafe impl ::windows::runtime::Abi for CF_POPULATION_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_POPULATION_POLICY_MODIFIER(pub u32);
pub const CF_POPULATION_POLICY_MODIFIER_NONE: CF_POPULATION_POLICY_MODIFIER = CF_POPULATION_POLICY_MODIFIER(0u32);
impl ::std::convert::From<u32> for CF_POPULATION_POLICY_MODIFIER {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_POPULATION_POLICY_MODIFIER {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_POPULATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_POPULATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_POPULATION_POLICY_MODIFIER {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_POPULATION_POLICY_MODIFIER {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_POPULATION_POLICY_MODIFIER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_POPULATION_POLICY_MODIFIER_USHORT {
    pub us: u16,
}
impl CF_POPULATION_POLICY_MODIFIER_USHORT {}
impl ::std::default::Default for CF_POPULATION_POLICY_MODIFIER_USHORT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_POPULATION_POLICY_MODIFIER_USHORT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_POPULATION_POLICY_MODIFIER_USHORT").field("us", &self.us).finish()
    }
}
impl ::std::cmp::PartialEq for CF_POPULATION_POLICY_MODIFIER_USHORT {
    fn eq(&self, other: &Self) -> bool {
        self.us == other.us
    }
}
impl ::std::cmp::Eq for CF_POPULATION_POLICY_MODIFIER_USHORT {}
unsafe impl ::windows::runtime::Abi for CF_POPULATION_POLICY_MODIFIER_USHORT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_POPULATION_POLICY_PRIMARY(pub i32);
pub const CF_POPULATION_POLICY_PARTIAL: CF_POPULATION_POLICY_PRIMARY = CF_POPULATION_POLICY_PRIMARY(0i32);
pub const CF_POPULATION_POLICY_FULL: CF_POPULATION_POLICY_PRIMARY = CF_POPULATION_POLICY_PRIMARY(2i32);
pub const CF_POPULATION_POLICY_ALWAYS_FULL: CF_POPULATION_POLICY_PRIMARY = CF_POPULATION_POLICY_PRIMARY(3i32);
impl ::std::convert::From<i32> for CF_POPULATION_POLICY_PRIMARY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_POPULATION_POLICY_PRIMARY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_POPULATION_POLICY_PRIMARY_USHORT {
    pub us: u16,
}
impl CF_POPULATION_POLICY_PRIMARY_USHORT {}
impl ::std::default::Default for CF_POPULATION_POLICY_PRIMARY_USHORT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_POPULATION_POLICY_PRIMARY_USHORT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_POPULATION_POLICY_PRIMARY_USHORT").field("us", &self.us).finish()
    }
}
impl ::std::cmp::PartialEq for CF_POPULATION_POLICY_PRIMARY_USHORT {
    fn eq(&self, other: &Self) -> bool {
        self.us == other.us
    }
}
impl ::std::cmp::Eq for CF_POPULATION_POLICY_PRIMARY_USHORT {}
unsafe impl ::windows::runtime::Abi for CF_POPULATION_POLICY_PRIMARY_USHORT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl CF_PROCESS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CF_PROCESS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CF_PROCESS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_PROCESS_INFO")
            .field("StructSize", &self.StructSize)
            .field("ProcessId", &self.ProcessId)
            .field("ImagePath", &self.ImagePath)
            .field("PackageName", &self.PackageName)
            .field("ApplicationId", &self.ApplicationId)
            .field("CommandLine", &self.CommandLine)
            .field("SessionId", &self.SessionId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CF_PROCESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StructSize == other.StructSize && self.ProcessId == other.ProcessId && self.ImagePath == other.ImagePath && self.PackageName == other.PackageName && self.ApplicationId == other.ApplicationId && self.CommandLine == other.CommandLine && self.SessionId == other.SessionId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CF_PROCESS_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CF_PROCESS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_REGISTER_FLAGS(pub u32);
pub const CF_REGISTER_FLAG_NONE: CF_REGISTER_FLAGS = CF_REGISTER_FLAGS(0u32);
pub const CF_REGISTER_FLAG_UPDATE: CF_REGISTER_FLAGS = CF_REGISTER_FLAGS(1u32);
pub const CF_REGISTER_FLAG_DISABLE_ON_DEMAND_POPULATION_ON_ROOT: CF_REGISTER_FLAGS = CF_REGISTER_FLAGS(2u32);
pub const CF_REGISTER_FLAG_MARK_IN_SYNC_ON_ROOT: CF_REGISTER_FLAGS = CF_REGISTER_FLAGS(4u32);
impl ::std::convert::From<u32> for CF_REGISTER_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_REGISTER_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_REGISTER_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_REGISTER_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_REGISTER_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_REGISTER_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_REGISTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CF_REQUEST_KEY_DEFAULT: u32 = 0u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_REVERT_FLAGS(pub u32);
pub const CF_REVERT_FLAG_NONE: CF_REVERT_FLAGS = CF_REVERT_FLAGS(0u32);
impl ::std::convert::From<u32> for CF_REVERT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_REVERT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_REVERT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_REVERT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_REVERT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_REVERT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_REVERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_SET_IN_SYNC_FLAGS(pub u32);
pub const CF_SET_IN_SYNC_FLAG_NONE: CF_SET_IN_SYNC_FLAGS = CF_SET_IN_SYNC_FLAGS(0u32);
impl ::std::convert::From<u32> for CF_SET_IN_SYNC_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_SET_IN_SYNC_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_SET_IN_SYNC_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_SET_IN_SYNC_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_SET_IN_SYNC_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_SET_IN_SYNC_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_SET_IN_SYNC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_SET_PIN_FLAGS(pub u32);
pub const CF_SET_PIN_FLAG_NONE: CF_SET_PIN_FLAGS = CF_SET_PIN_FLAGS(0u32);
pub const CF_SET_PIN_FLAG_RECURSE: CF_SET_PIN_FLAGS = CF_SET_PIN_FLAGS(1u32);
pub const CF_SET_PIN_FLAG_RECURSE_ONLY: CF_SET_PIN_FLAGS = CF_SET_PIN_FLAGS(2u32);
pub const CF_SET_PIN_FLAG_RECURSE_STOP_ON_ERROR: CF_SET_PIN_FLAGS = CF_SET_PIN_FLAGS(4u32);
impl ::std::convert::From<u32> for CF_SET_PIN_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_SET_PIN_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_SET_PIN_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_SET_PIN_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_SET_PIN_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_SET_PIN_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_SET_PIN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_SYNC_POLICIES {
    pub StructSize: u32,
    pub Hydration: CF_HYDRATION_POLICY,
    pub Population: CF_POPULATION_POLICY,
    pub InSync: CF_INSYNC_POLICY,
    pub HardLink: CF_HARDLINK_POLICY,
    pub PlaceholderManagement: CF_PLACEHOLDER_MANAGEMENT_POLICY,
}
impl CF_SYNC_POLICIES {}
impl ::std::default::Default for CF_SYNC_POLICIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_SYNC_POLICIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_SYNC_POLICIES").field("StructSize", &self.StructSize).field("Hydration", &self.Hydration).field("Population", &self.Population).field("InSync", &self.InSync).field("HardLink", &self.HardLink).field("PlaceholderManagement", &self.PlaceholderManagement).finish()
    }
}
impl ::std::cmp::PartialEq for CF_SYNC_POLICIES {
    fn eq(&self, other: &Self) -> bool {
        self.StructSize == other.StructSize && self.Hydration == other.Hydration && self.Population == other.Population && self.InSync == other.InSync && self.HardLink == other.HardLink && self.PlaceholderManagement == other.PlaceholderManagement
    }
}
impl ::std::cmp::Eq for CF_SYNC_POLICIES {}
unsafe impl ::windows::runtime::Abi for CF_SYNC_POLICIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_SYNC_PROVIDER_STATUS(pub u32);
pub const CF_PROVIDER_STATUS_DISCONNECTED: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(0u32);
pub const CF_PROVIDER_STATUS_IDLE: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(1u32);
pub const CF_PROVIDER_STATUS_POPULATE_NAMESPACE: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(2u32);
pub const CF_PROVIDER_STATUS_POPULATE_METADATA: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(4u32);
pub const CF_PROVIDER_STATUS_POPULATE_CONTENT: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(8u32);
pub const CF_PROVIDER_STATUS_SYNC_INCREMENTAL: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(16u32);
pub const CF_PROVIDER_STATUS_SYNC_FULL: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(32u32);
pub const CF_PROVIDER_STATUS_CONNECTIVITY_LOST: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(64u32);
pub const CF_PROVIDER_STATUS_CLEAR_FLAGS: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(2147483648u32);
pub const CF_PROVIDER_STATUS_TERMINATED: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(3221225473u32);
pub const CF_PROVIDER_STATUS_ERROR: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(3221225474u32);
impl ::std::convert::From<u32> for CF_SYNC_PROVIDER_STATUS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_SYNC_PROVIDER_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_SYNC_PROVIDER_STATUS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_SYNC_PROVIDER_STATUS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_SYNC_PROVIDER_STATUS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_SYNC_PROVIDER_STATUS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_SYNC_PROVIDER_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_SYNC_REGISTRATION {
    pub StructSize: u32,
    pub ProviderName: super::super::Foundation::PWSTR,
    pub ProviderVersion: super::super::Foundation::PWSTR,
    pub SyncRootIdentity: *mut ::std::ffi::c_void,
    pub SyncRootIdentityLength: u32,
    pub FileIdentity: *mut ::std::ffi::c_void,
    pub FileIdentityLength: u32,
    pub ProviderId: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl CF_SYNC_REGISTRATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CF_SYNC_REGISTRATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CF_SYNC_REGISTRATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_SYNC_REGISTRATION")
            .field("StructSize", &self.StructSize)
            .field("ProviderName", &self.ProviderName)
            .field("ProviderVersion", &self.ProviderVersion)
            .field("SyncRootIdentity", &self.SyncRootIdentity)
            .field("SyncRootIdentityLength", &self.SyncRootIdentityLength)
            .field("FileIdentity", &self.FileIdentity)
            .field("FileIdentityLength", &self.FileIdentityLength)
            .field("ProviderId", &self.ProviderId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CF_SYNC_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.StructSize == other.StructSize && self.ProviderName == other.ProviderName && self.ProviderVersion == other.ProviderVersion && self.SyncRootIdentity == other.SyncRootIdentity && self.SyncRootIdentityLength == other.SyncRootIdentityLength && self.FileIdentity == other.FileIdentity && self.FileIdentityLength == other.FileIdentityLength && self.ProviderId == other.ProviderId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CF_SYNC_REGISTRATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CF_SYNC_REGISTRATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_SYNC_ROOT_BASIC_INFO {
    pub SyncRootFileId: i64,
}
impl CF_SYNC_ROOT_BASIC_INFO {}
impl ::std::default::Default for CF_SYNC_ROOT_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_SYNC_ROOT_BASIC_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_SYNC_ROOT_BASIC_INFO").field("SyncRootFileId", &self.SyncRootFileId).finish()
    }
}
impl ::std::cmp::PartialEq for CF_SYNC_ROOT_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SyncRootFileId == other.SyncRootFileId
    }
}
impl ::std::cmp::Eq for CF_SYNC_ROOT_BASIC_INFO {}
unsafe impl ::windows::runtime::Abi for CF_SYNC_ROOT_BASIC_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_SYNC_ROOT_INFO_CLASS(pub i32);
pub const CF_SYNC_ROOT_INFO_BASIC: CF_SYNC_ROOT_INFO_CLASS = CF_SYNC_ROOT_INFO_CLASS(0i32);
pub const CF_SYNC_ROOT_INFO_STANDARD: CF_SYNC_ROOT_INFO_CLASS = CF_SYNC_ROOT_INFO_CLASS(1i32);
pub const CF_SYNC_ROOT_INFO_PROVIDER: CF_SYNC_ROOT_INFO_CLASS = CF_SYNC_ROOT_INFO_CLASS(2i32);
impl ::std::convert::From<i32> for CF_SYNC_ROOT_INFO_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_SYNC_ROOT_INFO_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_SYNC_ROOT_PROVIDER_INFO {
    pub ProviderStatus: CF_SYNC_PROVIDER_STATUS,
    pub ProviderName: [u16; 256],
    pub ProviderVersion: [u16; 256],
}
impl CF_SYNC_ROOT_PROVIDER_INFO {}
impl ::std::default::Default for CF_SYNC_ROOT_PROVIDER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_SYNC_ROOT_PROVIDER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_SYNC_ROOT_PROVIDER_INFO").field("ProviderStatus", &self.ProviderStatus).field("ProviderName", &self.ProviderName).field("ProviderVersion", &self.ProviderVersion).finish()
    }
}
impl ::std::cmp::PartialEq for CF_SYNC_ROOT_PROVIDER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProviderStatus == other.ProviderStatus && self.ProviderName == other.ProviderName && self.ProviderVersion == other.ProviderVersion
    }
}
impl ::std::cmp::Eq for CF_SYNC_ROOT_PROVIDER_INFO {}
unsafe impl ::windows::runtime::Abi for CF_SYNC_ROOT_PROVIDER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl CF_SYNC_ROOT_STANDARD_INFO {}
impl ::std::default::Default for CF_SYNC_ROOT_STANDARD_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_SYNC_ROOT_STANDARD_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_SYNC_ROOT_STANDARD_INFO")
            .field("SyncRootFileId", &self.SyncRootFileId)
            .field("HydrationPolicy", &self.HydrationPolicy)
            .field("PopulationPolicy", &self.PopulationPolicy)
            .field("InSyncPolicy", &self.InSyncPolicy)
            .field("HardLinkPolicy", &self.HardLinkPolicy)
            .field("ProviderStatus", &self.ProviderStatus)
            .field("ProviderName", &self.ProviderName)
            .field("ProviderVersion", &self.ProviderVersion)
            .field("SyncRootIdentityLength", &self.SyncRootIdentityLength)
            .field("SyncRootIdentity", &self.SyncRootIdentity)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CF_SYNC_ROOT_STANDARD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SyncRootFileId == other.SyncRootFileId && self.HydrationPolicy == other.HydrationPolicy && self.PopulationPolicy == other.PopulationPolicy && self.InSyncPolicy == other.InSyncPolicy && self.HardLinkPolicy == other.HardLinkPolicy && self.ProviderStatus == other.ProviderStatus && self.ProviderName == other.ProviderName && self.ProviderVersion == other.ProviderVersion && self.SyncRootIdentityLength == other.SyncRootIdentityLength && self.SyncRootIdentity == other.SyncRootIdentity
    }
}
impl ::std::cmp::Eq for CF_SYNC_ROOT_STANDARD_INFO {}
unsafe impl ::windows::runtime::Abi for CF_SYNC_ROOT_STANDARD_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CF_SYNC_STATUS {
    pub StructSize: u32,
    pub Code: u32,
    pub DescriptionOffset: u32,
    pub DescriptionLength: u32,
    pub DeviceIdOffset: u32,
    pub DeviceIdLength: u32,
}
impl CF_SYNC_STATUS {}
impl ::std::default::Default for CF_SYNC_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CF_SYNC_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CF_SYNC_STATUS")
            .field("StructSize", &self.StructSize)
            .field("Code", &self.Code)
            .field("DescriptionOffset", &self.DescriptionOffset)
            .field("DescriptionLength", &self.DescriptionLength)
            .field("DeviceIdOffset", &self.DeviceIdOffset)
            .field("DeviceIdLength", &self.DeviceIdLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CF_SYNC_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.StructSize == other.StructSize && self.Code == other.Code && self.DescriptionOffset == other.DescriptionOffset && self.DescriptionLength == other.DescriptionLength && self.DeviceIdOffset == other.DeviceIdOffset && self.DeviceIdLength == other.DeviceIdLength
    }
}
impl ::std::cmp::Eq for CF_SYNC_STATUS {}
unsafe impl ::windows::runtime::Abi for CF_SYNC_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CF_UPDATE_FLAGS(pub u32);
pub const CF_UPDATE_FLAG_NONE: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(0u32);
pub const CF_UPDATE_FLAG_VERIFY_IN_SYNC: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(1u32);
pub const CF_UPDATE_FLAG_MARK_IN_SYNC: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(2u32);
pub const CF_UPDATE_FLAG_DEHYDRATE: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(4u32);
pub const CF_UPDATE_FLAG_ENABLE_ON_DEMAND_POPULATION: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(8u32);
pub const CF_UPDATE_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(16u32);
pub const CF_UPDATE_FLAG_REMOVE_FILE_IDENTITY: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(32u32);
pub const CF_UPDATE_FLAG_CLEAR_IN_SYNC: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(64u32);
pub const CF_UPDATE_FLAG_REMOVE_PROPERTY: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(128u32);
pub const CF_UPDATE_FLAG_PASSTHROUGH_FS_METADATA: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(256u32);
pub const CF_UPDATE_FLAG_ALWAYS_FULL: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(512u32);
pub const CF_UPDATE_FLAG_ALLOW_PARTIAL: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(1024u32);
impl ::std::convert::From<u32> for CF_UPDATE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CF_UPDATE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CF_UPDATE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CF_UPDATE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CF_UPDATE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CF_UPDATE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CF_UPDATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfCloseHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfCloseHandle(filehandle: super::super::Foundation::HANDLE);
        }
        ::std::mem::transmute(CfCloseHandle(filehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn CfConnectSyncRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(syncrootpath: Param0, callbacktable: *const CF_CALLBACK_REGISTRATION, callbackcontext: *const ::std::ffi::c_void, connectflags: CF_CONNECT_FLAGS) -> ::windows::runtime::Result<CF_CONNECTION_KEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfConnectSyncRoot(syncrootpath: super::super::Foundation::PWSTR, callbacktable: *const ::std::mem::ManuallyDrop<CF_CALLBACK_REGISTRATION>, callbackcontext: *const ::std::ffi::c_void, connectflags: CF_CONNECT_FLAGS, connectionkey: *mut CF_CONNECTION_KEY) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <CF_CONNECTION_KEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CfConnectSyncRoot(syncrootpath.into_param().abi(), ::std::mem::transmute(callbacktable), ::std::mem::transmute(callbackcontext), ::std::mem::transmute(connectflags), &mut result__).from_abi::<CF_CONNECTION_KEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfConvertToPlaceholder<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, fileidentity: *const ::std::ffi::c_void, fileidentitylength: u32, convertflags: CF_CONVERT_FLAGS, convertusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfConvertToPlaceholder(filehandle: super::super::Foundation::HANDLE, fileidentity: *const ::std::ffi::c_void, fileidentitylength: u32, convertflags: CF_CONVERT_FLAGS, convertusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
        }
        CfConvertToPlaceholder(filehandle.into_param().abi(), ::std::mem::transmute(fileidentity), ::std::mem::transmute(fileidentitylength), ::std::mem::transmute(convertflags), ::std::mem::transmute(convertusn), ::std::mem::transmute(overlapped)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn CfCreatePlaceholders<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(basedirectorypath: Param0, placeholderarray: *mut CF_PLACEHOLDER_CREATE_INFO, placeholdercount: u32, createflags: CF_CREATE_FLAGS, entriesprocessed: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfCreatePlaceholders(basedirectorypath: super::super::Foundation::PWSTR, placeholderarray: *mut CF_PLACEHOLDER_CREATE_INFO, placeholdercount: u32, createflags: CF_CREATE_FLAGS, entriesprocessed: *mut u32) -> ::windows::runtime::HRESULT;
        }
        CfCreatePlaceholders(basedirectorypath.into_param().abi(), ::std::mem::transmute(placeholderarray), ::std::mem::transmute(placeholdercount), ::std::mem::transmute(createflags), ::std::mem::transmute(entriesprocessed)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfDehydratePlaceholder<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, startingoffset: i64, length: i64, dehydrateflags: CF_DEHYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfDehydratePlaceholder(filehandle: super::super::Foundation::HANDLE, startingoffset: i64, length: i64, dehydrateflags: CF_DEHYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
        }
        CfDehydratePlaceholder(filehandle.into_param().abi(), ::std::mem::transmute(startingoffset), ::std::mem::transmute(length), ::std::mem::transmute(dehydrateflags), ::std::mem::transmute(overlapped)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CfDisconnectSyncRoot<'a, Param0: ::windows::runtime::IntoParam<'a, CF_CONNECTION_KEY>>(connectionkey: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfDisconnectSyncRoot(connectionkey: CF_CONNECTION_KEY) -> ::windows::runtime::HRESULT;
        }
        CfDisconnectSyncRoot(connectionkey.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn CfExecute(opinfo: *const CF_OPERATION_INFO, opparams: *mut CF_OPERATION_PARAMETERS) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfExecute(opinfo: *const CF_OPERATION_INFO, opparams: *mut CF_OPERATION_PARAMETERS) -> ::windows::runtime::HRESULT;
        }
        CfExecute(::std::mem::transmute(opinfo), ::std::mem::transmute(opparams)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn CfGetCorrelationVector<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0) -> ::windows::runtime::Result<super::super::System::SystemServices::CORRELATION_VECTOR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetCorrelationVector(filehandle: super::super::Foundation::HANDLE, correlationvector: *mut super::super::System::SystemServices::CORRELATION_VECTOR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::System::SystemServices::CORRELATION_VECTOR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CfGetCorrelationVector(filehandle.into_param().abi(), &mut result__).from_abi::<super::super::System::SystemServices::CORRELATION_VECTOR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetPlaceholderInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, infoclass: CF_PLACEHOLDER_INFO_CLASS, infobuffer: *mut ::std::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetPlaceholderInfo(filehandle: super::super::Foundation::HANDLE, infoclass: CF_PLACEHOLDER_INFO_CLASS, infobuffer: *mut ::std::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::runtime::HRESULT;
        }
        CfGetPlaceholderInfo(filehandle.into_param().abi(), ::std::mem::transmute(infoclass), ::std::mem::transmute(infobuffer), ::std::mem::transmute(infobufferlength), ::std::mem::transmute(returnedlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetPlaceholderRangeInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, infoclass: CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset: i64, length: i64, infobuffer: *mut ::std::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetPlaceholderRangeInfo(filehandle: super::super::Foundation::HANDLE, infoclass: CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset: i64, length: i64, infobuffer: *mut ::std::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::runtime::HRESULT;
        }
        CfGetPlaceholderRangeInfo(filehandle.into_param().abi(), ::std::mem::transmute(infoclass), ::std::mem::transmute(startingoffset), ::std::mem::transmute(length), ::std::mem::transmute(infobuffer), ::std::mem::transmute(infobufferlength), ::std::mem::transmute(returnedlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CfGetPlaceholderStateFromAttributeTag(fileattributes: u32, reparsetag: u32) -> CF_PLACEHOLDER_STATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetPlaceholderStateFromAttributeTag(fileattributes: u32, reparsetag: u32) -> CF_PLACEHOLDER_STATE;
        }
        ::std::mem::transmute(CfGetPlaceholderStateFromAttributeTag(::std::mem::transmute(fileattributes), ::std::mem::transmute(reparsetag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Storage_FileSystem")]
#[inline]
pub unsafe fn CfGetPlaceholderStateFromFileInfo(infobuffer: *const ::std::ffi::c_void, infoclass: super::FileSystem::FILE_INFO_BY_HANDLE_CLASS) -> CF_PLACEHOLDER_STATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetPlaceholderStateFromFileInfo(infobuffer: *const ::std::ffi::c_void, infoclass: super::FileSystem::FILE_INFO_BY_HANDLE_CLASS) -> CF_PLACEHOLDER_STATE;
        }
        ::std::mem::transmute(CfGetPlaceholderStateFromFileInfo(::std::mem::transmute(infobuffer), ::std::mem::transmute(infoclass)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn CfGetPlaceholderStateFromFindData(finddata: *const super::FileSystem::WIN32_FIND_DATAA) -> CF_PLACEHOLDER_STATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetPlaceholderStateFromFindData(finddata: *const super::FileSystem::WIN32_FIND_DATAA) -> CF_PLACEHOLDER_STATE;
        }
        ::std::mem::transmute(CfGetPlaceholderStateFromFindData(::std::mem::transmute(finddata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CfGetPlatformInfo() -> ::windows::runtime::Result<CF_PLATFORM_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetPlatformInfo(platformversion: *mut CF_PLATFORM_INFO) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <CF_PLATFORM_INFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CfGetPlatformInfo(&mut result__).from_abi::<CF_PLATFORM_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetSyncRootInfoByHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::std::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetSyncRootInfoByHandle(filehandle: super::super::Foundation::HANDLE, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::std::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::runtime::HRESULT;
        }
        CfGetSyncRootInfoByHandle(filehandle.into_param().abi(), ::std::mem::transmute(infoclass), ::std::mem::transmute(infobuffer), ::std::mem::transmute(infobufferlength), ::std::mem::transmute(returnedlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetSyncRootInfoByPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(filepath: Param0, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::std::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetSyncRootInfoByPath(filepath: super::super::Foundation::PWSTR, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::std::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::runtime::HRESULT;
        }
        CfGetSyncRootInfoByPath(filepath.into_param().abi(), ::std::mem::transmute(infoclass), ::std::mem::transmute(infobuffer), ::std::mem::transmute(infobufferlength), ::std::mem::transmute(returnedlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetTransferKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0) -> ::windows::runtime::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetTransferKey(filehandle: super::super::Foundation::HANDLE, transferkey: *mut i64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CfGetTransferKey(filehandle.into_param().abi(), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetWin32HandleFromProtectedHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(protectedhandle: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetWin32HandleFromProtectedHandle(protectedhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CfGetWin32HandleFromProtectedHandle(protectedhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfHydratePlaceholder<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, startingoffset: i64, length: i64, hydrateflags: CF_HYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfHydratePlaceholder(filehandle: super::super::Foundation::HANDLE, startingoffset: i64, length: i64, hydrateflags: CF_HYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
        }
        CfHydratePlaceholder(filehandle.into_param().abi(), ::std::mem::transmute(startingoffset), ::std::mem::transmute(length), ::std::mem::transmute(hydrateflags), ::std::mem::transmute(overlapped)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfOpenFileWithOplock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(filepath: Param0, flags: CF_OPEN_FILE_FLAGS) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfOpenFileWithOplock(filepath: super::super::Foundation::PWSTR, flags: CF_OPEN_FILE_FLAGS, protectedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CfOpenFileWithOplock(filepath.into_param().abi(), ::std::mem::transmute(flags), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CfQuerySyncProviderStatus<'a, Param0: ::windows::runtime::IntoParam<'a, CF_CONNECTION_KEY>>(connectionkey: Param0) -> ::windows::runtime::Result<CF_SYNC_PROVIDER_STATUS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfQuerySyncProviderStatus(connectionkey: CF_CONNECTION_KEY, providerstatus: *mut CF_SYNC_PROVIDER_STATUS) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <CF_SYNC_PROVIDER_STATUS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CfQuerySyncProviderStatus(connectionkey.into_param().abi(), &mut result__).from_abi::<CF_SYNC_PROVIDER_STATUS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfReferenceProtectedHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(protectedhandle: Param0) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfReferenceProtectedHandle(protectedhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(CfReferenceProtectedHandle(protectedhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfRegisterSyncRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(syncrootpath: Param0, registration: *const CF_SYNC_REGISTRATION, policies: *const CF_SYNC_POLICIES, registerflags: CF_REGISTER_FLAGS) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfRegisterSyncRoot(syncrootpath: super::super::Foundation::PWSTR, registration: *const CF_SYNC_REGISTRATION, policies: *const CF_SYNC_POLICIES, registerflags: CF_REGISTER_FLAGS) -> ::windows::runtime::HRESULT;
        }
        CfRegisterSyncRoot(syncrootpath.into_param().abi(), ::std::mem::transmute(registration), ::std::mem::transmute(policies), ::std::mem::transmute(registerflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfReleaseProtectedHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(protectedhandle: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfReleaseProtectedHandle(protectedhandle: super::super::Foundation::HANDLE);
        }
        ::std::mem::transmute(CfReleaseProtectedHandle(protectedhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfReleaseTransferKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, transferkey: *mut i64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfReleaseTransferKey(filehandle: super::super::Foundation::HANDLE, transferkey: *mut i64);
        }
        ::std::mem::transmute(CfReleaseTransferKey(filehandle.into_param().abi(), ::std::mem::transmute(transferkey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CfReportProviderProgress<'a, Param0: ::windows::runtime::IntoParam<'a, CF_CONNECTION_KEY>>(connectionkey: Param0, transferkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfReportProviderProgress(connectionkey: CF_CONNECTION_KEY, transferkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64) -> ::windows::runtime::HRESULT;
        }
        CfReportProviderProgress(connectionkey.into_param().abi(), ::std::mem::transmute(transferkey), ::std::mem::transmute(providerprogresstotal), ::std::mem::transmute(providerprogresscompleted)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CfReportProviderProgress2<'a, Param0: ::windows::runtime::IntoParam<'a, CF_CONNECTION_KEY>>(connectionkey: Param0, transferkey: i64, requestkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64, targetsessionid: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfReportProviderProgress2(connectionkey: CF_CONNECTION_KEY, transferkey: i64, requestkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64, targetsessionid: u32) -> ::windows::runtime::HRESULT;
        }
        CfReportProviderProgress2(connectionkey.into_param().abi(), ::std::mem::transmute(transferkey), ::std::mem::transmute(requestkey), ::std::mem::transmute(providerprogresstotal), ::std::mem::transmute(providerprogresscompleted), ::std::mem::transmute(targetsessionid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfReportSyncStatus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(syncrootpath: Param0, syncstatus: *const CF_SYNC_STATUS) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfReportSyncStatus(syncrootpath: super::super::Foundation::PWSTR, syncstatus: *const CF_SYNC_STATUS) -> ::windows::runtime::HRESULT;
        }
        CfReportSyncStatus(syncrootpath.into_param().abi(), ::std::mem::transmute(syncstatus)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfRevertPlaceholder<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, revertflags: CF_REVERT_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfRevertPlaceholder(filehandle: super::super::Foundation::HANDLE, revertflags: CF_REVERT_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
        }
        CfRevertPlaceholder(filehandle.into_param().abi(), ::std::mem::transmute(revertflags), ::std::mem::transmute(overlapped)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn CfSetCorrelationVector<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, correlationvector: *const super::super::System::SystemServices::CORRELATION_VECTOR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfSetCorrelationVector(filehandle: super::super::Foundation::HANDLE, correlationvector: *const super::super::System::SystemServices::CORRELATION_VECTOR) -> ::windows::runtime::HRESULT;
        }
        CfSetCorrelationVector(filehandle.into_param().abi(), ::std::mem::transmute(correlationvector)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfSetInSyncState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, insyncstate: CF_IN_SYNC_STATE, insyncflags: CF_SET_IN_SYNC_FLAGS, insyncusn: *mut i64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfSetInSyncState(filehandle: super::super::Foundation::HANDLE, insyncstate: CF_IN_SYNC_STATE, insyncflags: CF_SET_IN_SYNC_FLAGS, insyncusn: *mut i64) -> ::windows::runtime::HRESULT;
        }
        CfSetInSyncState(filehandle.into_param().abi(), ::std::mem::transmute(insyncstate), ::std::mem::transmute(insyncflags), ::std::mem::transmute(insyncusn)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfSetPinState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, pinstate: CF_PIN_STATE, pinflags: CF_SET_PIN_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfSetPinState(filehandle: super::super::Foundation::HANDLE, pinstate: CF_PIN_STATE, pinflags: CF_SET_PIN_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
        }
        CfSetPinState(filehandle.into_param().abi(), ::std::mem::transmute(pinstate), ::std::mem::transmute(pinflags), ::std::mem::transmute(overlapped)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfUnregisterSyncRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(syncrootpath: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfUnregisterSyncRoot(syncrootpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        CfUnregisterSyncRoot(syncrootpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfUpdatePlaceholder<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, fsmetadata: *const CF_FS_METADATA, fileidentity: *const ::std::ffi::c_void, fileidentitylength: u32, dehydraterangearray: *const CF_FILE_RANGE, dehydraterangecount: u32, updateflags: CF_UPDATE_FLAGS, updateusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfUpdatePlaceholder(filehandle: super::super::Foundation::HANDLE, fsmetadata: *const CF_FS_METADATA, fileidentity: *const ::std::ffi::c_void, fileidentitylength: u32, dehydraterangearray: *const CF_FILE_RANGE, dehydraterangecount: u32, updateflags: CF_UPDATE_FLAGS, updateusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
        }
        CfUpdatePlaceholder(
            filehandle.into_param().abi(),
            ::std::mem::transmute(fsmetadata),
            ::std::mem::transmute(fileidentity),
            ::std::mem::transmute(fileidentitylength),
            ::std::mem::transmute(dehydraterangearray),
            ::std::mem::transmute(dehydraterangecount),
            ::std::mem::transmute(updateflags),
            ::std::mem::transmute(updateusn),
            ::std::mem::transmute(overlapped),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CfUpdateSyncProviderStatus<'a, Param0: ::windows::runtime::IntoParam<'a, CF_CONNECTION_KEY>>(connectionkey: Param0, providerstatus: CF_SYNC_PROVIDER_STATUS) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfUpdateSyncProviderStatus(connectionkey: CF_CONNECTION_KEY, providerstatus: CF_SYNC_PROVIDER_STATUS) -> ::windows::runtime::HRESULT;
        }
        CfUpdateSyncProviderStatus(connectionkey.into_param().abi(), ::std::mem::transmute(providerstatus)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
