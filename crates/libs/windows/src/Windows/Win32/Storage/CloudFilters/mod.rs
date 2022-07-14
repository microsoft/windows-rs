#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_System_CorrelationVector\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
pub type CF_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackinfo: *const CF_CALLBACK_INFO, callbackparameters: *const CF_CALLBACK_PARAMETERS)>;
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_CANCEL_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_CANCEL_FLAG_NONE: CF_CALLBACK_CANCEL_FLAGS = CF_CALLBACK_CANCEL_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_CANCEL_FLAG_IO_TIMEOUT: CF_CALLBACK_CANCEL_FLAGS = CF_CALLBACK_CANCEL_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_CANCEL_FLAG_IO_ABORTED: CF_CALLBACK_CANCEL_FLAGS = CF_CALLBACK_CANCEL_FLAGS(2u32);
impl ::core::marker::Copy for CF_CALLBACK_CANCEL_FLAGS {}
impl ::core::clone::Clone for CF_CALLBACK_CANCEL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_CANCEL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_CANCEL_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_CANCEL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_CANCEL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_CANCEL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_CANCEL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_CANCEL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_CANCEL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_CANCEL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_CLOSE_COMPLETION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_CLOSE_COMPLETION_FLAG_NONE: CF_CALLBACK_CLOSE_COMPLETION_FLAGS = CF_CALLBACK_CLOSE_COMPLETION_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_CLOSE_COMPLETION_FLAG_DELETED: CF_CALLBACK_CLOSE_COMPLETION_FLAGS = CF_CALLBACK_CLOSE_COMPLETION_FLAGS(1u32);
impl ::core::marker::Copy for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {}
impl ::core::clone::Clone for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_CLOSE_COMPLETION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_NONE: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_BACKGROUND: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_DEHYDRATED: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS(2u32);
impl ::core::marker::Copy for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {}
impl ::core::clone::Clone for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_DEHYDRATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DEHYDRATE_FLAG_NONE: CF_CALLBACK_DEHYDRATE_FLAGS = CF_CALLBACK_DEHYDRATE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DEHYDRATE_FLAG_BACKGROUND: CF_CALLBACK_DEHYDRATE_FLAGS = CF_CALLBACK_DEHYDRATE_FLAGS(1u32);
impl ::core::marker::Copy for CF_CALLBACK_DEHYDRATE_FLAGS {}
impl ::core::clone::Clone for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_DEHYDRATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_DEHYDRATION_REASON(pub i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DEHYDRATION_REASON_NONE: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(0i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DEHYDRATION_REASON_USER_MANUAL: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(1i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_LOW_SPACE: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(2i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_INACTIVITY: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(3i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_OS_UPGRADE: CF_CALLBACK_DEHYDRATION_REASON = CF_CALLBACK_DEHYDRATION_REASON(4i32);
impl ::core::marker::Copy for CF_CALLBACK_DEHYDRATION_REASON {}
impl ::core::clone::Clone for CF_CALLBACK_DEHYDRATION_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_DEHYDRATION_REASON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_DEHYDRATION_REASON {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_DEHYDRATION_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_DEHYDRATION_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_DELETE_COMPLETION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DELETE_COMPLETION_FLAG_NONE: CF_CALLBACK_DELETE_COMPLETION_FLAGS = CF_CALLBACK_DELETE_COMPLETION_FLAGS(0u32);
impl ::core::marker::Copy for CF_CALLBACK_DELETE_COMPLETION_FLAGS {}
impl ::core::clone::Clone for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_DELETE_COMPLETION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_DELETE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DELETE_FLAG_NONE: CF_CALLBACK_DELETE_FLAGS = CF_CALLBACK_DELETE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DELETE_FLAG_IS_DIRECTORY: CF_CALLBACK_DELETE_FLAGS = CF_CALLBACK_DELETE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_DELETE_FLAG_IS_UNDELETE: CF_CALLBACK_DELETE_FLAGS = CF_CALLBACK_DELETE_FLAGS(2u32);
impl ::core::marker::Copy for CF_CALLBACK_DELETE_FLAGS {}
impl ::core::clone::Clone for CF_CALLBACK_DELETE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_DELETE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_DELETE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_DELETE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_DELETE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_DELETE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_DELETE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_DELETE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_DELETE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_DELETE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_FETCH_DATA_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_FETCH_DATA_FLAG_NONE: CF_CALLBACK_FETCH_DATA_FLAGS = CF_CALLBACK_FETCH_DATA_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_FETCH_DATA_FLAG_RECOVERY: CF_CALLBACK_FETCH_DATA_FLAGS = CF_CALLBACK_FETCH_DATA_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_FETCH_DATA_FLAG_EXPLICIT_HYDRATION: CF_CALLBACK_FETCH_DATA_FLAGS = CF_CALLBACK_FETCH_DATA_FLAGS(2u32);
impl ::core::marker::Copy for CF_CALLBACK_FETCH_DATA_FLAGS {}
impl ::core::clone::Clone for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_FETCH_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_FETCH_PLACEHOLDERS_FLAG_NONE: CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS = CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS(0u32);
impl ::core::marker::Copy for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {}
impl ::core::clone::Clone for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_System_CorrelationVector\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
pub struct CF_CALLBACK_INFO {
    pub StructSize: u32,
    pub ConnectionKey: CF_CONNECTION_KEY,
    pub CallbackContext: *mut ::core::ffi::c_void,
    pub VolumeGuidName: ::windows::core::PCWSTR,
    pub VolumeDosName: ::windows::core::PCWSTR,
    pub VolumeSerialNumber: u32,
    pub SyncRootFileId: i64,
    pub SyncRootIdentity: *const ::core::ffi::c_void,
    pub SyncRootIdentityLength: u32,
    pub FileId: i64,
    pub FileSize: i64,
    pub FileIdentity: *const ::core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub NormalizedPath: ::windows::core::PCWSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::fmt::Debug for CF_CALLBACK_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_INFO")
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
unsafe impl ::windows::core::Abi for CF_CALLBACK_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::cmp::PartialEq for CF_CALLBACK_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::cmp::Eq for CF_CALLBACK_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::default::Default for CF_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_OPEN_COMPLETION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_NONE: CF_CALLBACK_OPEN_COMPLETION_FLAGS = CF_CALLBACK_OPEN_COMPLETION_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNKNOWN: CF_CALLBACK_OPEN_COMPLETION_FLAGS = CF_CALLBACK_OPEN_COMPLETION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNSUPPORTED: CF_CALLBACK_OPEN_COMPLETION_FLAGS = CF_CALLBACK_OPEN_COMPLETION_FLAGS(2u32);
impl ::core::marker::Copy for CF_CALLBACK_OPEN_COMPLETION_FLAGS {}
impl ::core::clone::Clone for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_OPEN_COMPLETION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS {
    pub ParamSize: u32,
    pub Anonymous: CF_CALLBACK_PARAMETERS_0,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
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
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS_0_0 {
    pub Flags: CF_CALLBACK_CANCEL_FLAGS,
    pub Anonymous: CF_CALLBACK_PARAMETERS_0_0_0,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_0 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_0 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub union CF_CALLBACK_PARAMETERS_0_0_0 {
    pub FetchData: CF_CALLBACK_PARAMETERS_0_0_0_0,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_0_0 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_0_0 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS_0_0_0_0 {
    pub FileOffset: i64,
    pub Length: i64,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_0_0_0 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_PARAMETERS_0_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_PARAMETERS_0_0_0_0").field("FileOffset", &self.FileOffset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_0_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_0_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_0_0_0 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS_0_1 {
    pub Flags: CF_CALLBACK_CLOSE_COMPLETION_FLAGS,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_1 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_PARAMETERS_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_PARAMETERS_0_1").field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_1 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS_0_2 {
    pub Flags: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS,
    pub Reason: CF_CALLBACK_DEHYDRATION_REASON,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_2 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_PARAMETERS_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_PARAMETERS_0_2").field("Flags", &self.Flags).field("Reason", &self.Reason).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_2 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS_0_3 {
    pub Flags: CF_CALLBACK_DEHYDRATE_FLAGS,
    pub Reason: CF_CALLBACK_DEHYDRATION_REASON,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_3 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_PARAMETERS_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_PARAMETERS_0_3").field("Flags", &self.Flags).field("Reason", &self.Reason).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_3>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_3 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS_0_4 {
    pub Flags: CF_CALLBACK_DELETE_COMPLETION_FLAGS,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_4 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_PARAMETERS_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_PARAMETERS_0_4").field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_4 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_4>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_4 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS_0_5 {
    pub Flags: CF_CALLBACK_DELETE_FLAGS,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_5 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_PARAMETERS_0_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_PARAMETERS_0_5").field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_5 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_5>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_5 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS_0_6 {
    pub Flags: CF_CALLBACK_FETCH_DATA_FLAGS,
    pub RequiredFileOffset: i64,
    pub RequiredLength: i64,
    pub OptionalFileOffset: i64,
    pub OptionalLength: i64,
    pub LastDehydrationTime: i64,
    pub LastDehydrationReason: CF_CALLBACK_DEHYDRATION_REASON,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_6 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_PARAMETERS_0_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_PARAMETERS_0_6").field("Flags", &self.Flags).field("RequiredFileOffset", &self.RequiredFileOffset).field("RequiredLength", &self.RequiredLength).field("OptionalFileOffset", &self.OptionalFileOffset).field("OptionalLength", &self.OptionalLength).field("LastDehydrationTime", &self.LastDehydrationTime).field("LastDehydrationReason", &self.LastDehydrationReason).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_6>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_6 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS_0_7 {
    pub Flags: CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS,
    pub Pattern: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_7 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_PARAMETERS_0_7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_PARAMETERS_0_7").field("Flags", &self.Flags).field("Pattern", &self.Pattern).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_7 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_7 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_7>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_7 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS_0_8 {
    pub Flags: CF_CALLBACK_OPEN_COMPLETION_FLAGS,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_8 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_8 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_PARAMETERS_0_8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_PARAMETERS_0_8").field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_8 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_8 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_8>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_8 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS_0_9 {
    pub Flags: CF_CALLBACK_RENAME_COMPLETION_FLAGS,
    pub SourcePath: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_9 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_9 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_PARAMETERS_0_9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_PARAMETERS_0_9").field("Flags", &self.Flags).field("SourcePath", &self.SourcePath).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_9 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_9 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_9>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_9 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS_0_10 {
    pub Flags: CF_CALLBACK_RENAME_FLAGS,
    pub TargetPath: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_10 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_10 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_PARAMETERS_0_10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_PARAMETERS_0_10").field("Flags", &self.Flags).field("TargetPath", &self.TargetPath).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_10 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_10 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_10>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_10 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_CALLBACK_PARAMETERS_0_11 {
    pub Flags: CF_CALLBACK_VALIDATE_DATA_FLAGS,
    pub RequiredFileOffset: i64,
    pub RequiredLength: i64,
}
impl ::core::marker::Copy for CF_CALLBACK_PARAMETERS_0_11 {}
impl ::core::clone::Clone for CF_CALLBACK_PARAMETERS_0_11 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_PARAMETERS_0_11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_PARAMETERS_0_11").field("Flags", &self.Flags).field("RequiredFileOffset", &self.RequiredFileOffset).field("RequiredLength", &self.RequiredLength).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_11 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_11 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_11>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_11 {}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_11 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_System_CorrelationVector\"`*"]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::fmt::Debug for CF_CALLBACK_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_REGISTRATION").field("Type", &self.Type).field("Callback", &self.Callback.map(|f| f as usize)).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
unsafe impl ::windows::core::Abi for CF_CALLBACK_REGISTRATION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::cmp::PartialEq for CF_CALLBACK_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_REGISTRATION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::cmp::Eq for CF_CALLBACK_REGISTRATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::default::Default for CF_CALLBACK_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_RENAME_COMPLETION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_RENAME_COMPLETION_FLAG_NONE: CF_CALLBACK_RENAME_COMPLETION_FLAGS = CF_CALLBACK_RENAME_COMPLETION_FLAGS(0u32);
impl ::core::marker::Copy for CF_CALLBACK_RENAME_COMPLETION_FLAGS {}
impl ::core::clone::Clone for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_RENAME_COMPLETION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_RENAME_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_RENAME_FLAG_NONE: CF_CALLBACK_RENAME_FLAGS = CF_CALLBACK_RENAME_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_RENAME_FLAG_IS_DIRECTORY: CF_CALLBACK_RENAME_FLAGS = CF_CALLBACK_RENAME_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_RENAME_FLAG_SOURCE_IN_SCOPE: CF_CALLBACK_RENAME_FLAGS = CF_CALLBACK_RENAME_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_RENAME_FLAG_TARGET_IN_SCOPE: CF_CALLBACK_RENAME_FLAGS = CF_CALLBACK_RENAME_FLAGS(4u32);
impl ::core::marker::Copy for CF_CALLBACK_RENAME_FLAGS {}
impl ::core::clone::Clone for CF_CALLBACK_RENAME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_RENAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_RENAME_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_RENAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_RENAME_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_RENAME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_RENAME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_RENAME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_RENAME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_RENAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_FETCH_DATA: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_VALIDATE_DATA: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_CANCEL_FETCH_DATA: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_FETCH_PLACEHOLDERS: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_CANCEL_FETCH_PLACEHOLDERS: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_NOTIFY_FILE_OPEN_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_NOTIFY_FILE_CLOSE_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_NOTIFY_DELETE: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_NOTIFY_DELETE_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_NOTIFY_RENAME: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_NOTIFY_RENAME_COMPLETION: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_TYPE_NONE: CF_CALLBACK_TYPE = CF_CALLBACK_TYPE(-1i32);
impl ::core::marker::Copy for CF_CALLBACK_TYPE {}
impl ::core::clone::Clone for CF_CALLBACK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CALLBACK_VALIDATE_DATA_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_VALIDATE_DATA_FLAG_NONE: CF_CALLBACK_VALIDATE_DATA_FLAGS = CF_CALLBACK_VALIDATE_DATA_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CALLBACK_VALIDATE_DATA_FLAG_EXPLICIT_HYDRATION: CF_CALLBACK_VALIDATE_DATA_FLAGS = CF_CALLBACK_VALIDATE_DATA_FLAGS(2u32);
impl ::core::marker::Copy for CF_CALLBACK_VALIDATE_DATA_FLAGS {}
impl ::core::clone::Clone for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_VALIDATE_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CONNECTION_KEY(pub isize);
impl CF_CONNECTION_KEY {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for CF_CONNECTION_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CF_CONNECTION_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CF_CONNECTION_KEY {}
impl ::core::fmt::Debug for CF_CONNECTION_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CONNECTION_KEY").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<CF_CONNECTION_KEY>> for CF_CONNECTION_KEY {
    fn from(optional: ::core::option::Option<CF_CONNECTION_KEY>) -> CF_CONNECTION_KEY {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for CF_CONNECTION_KEY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CONNECT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CONNECT_FLAG_NONE: CF_CONNECT_FLAGS = CF_CONNECT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CONNECT_FLAG_REQUIRE_PROCESS_INFO: CF_CONNECT_FLAGS = CF_CONNECT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CONNECT_FLAG_REQUIRE_FULL_FILE_PATH: CF_CONNECT_FLAGS = CF_CONNECT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CONNECT_FLAG_BLOCK_SELF_IMPLICIT_HYDRATION: CF_CONNECT_FLAGS = CF_CONNECT_FLAGS(8u32);
impl ::core::marker::Copy for CF_CONNECT_FLAGS {}
impl ::core::clone::Clone for CF_CONNECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CONNECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CONNECT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CONNECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CONNECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CONNECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CONNECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CONNECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CONNECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CONNECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CONVERT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CONVERT_FLAG_NONE: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CONVERT_FLAG_MARK_IN_SYNC: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CONVERT_FLAG_DEHYDRATE: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CONVERT_FLAG_ENABLE_ON_DEMAND_POPULATION: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CONVERT_FLAG_ALWAYS_FULL: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CONVERT_FLAG_FORCE_CONVERT_TO_CLOUD_FILE: CF_CONVERT_FLAGS = CF_CONVERT_FLAGS(16u32);
impl ::core::marker::Copy for CF_CONVERT_FLAGS {}
impl ::core::clone::Clone for CF_CONVERT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CONVERT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CONVERT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CONVERT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CONVERT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CONVERT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CONVERT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CONVERT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CONVERT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CONVERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_CREATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CREATE_FLAG_NONE: CF_CREATE_FLAGS = CF_CREATE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_CREATE_FLAG_STOP_ON_ERROR: CF_CREATE_FLAGS = CF_CREATE_FLAGS(1u32);
impl ::core::marker::Copy for CF_CREATE_FLAGS {}
impl ::core::clone::Clone for CF_CREATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_CREATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_CREATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_CREATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CREATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CREATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CREATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_DEHYDRATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_DEHYDRATE_FLAG_NONE: CF_DEHYDRATE_FLAGS = CF_DEHYDRATE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_DEHYDRATE_FLAG_BACKGROUND: CF_DEHYDRATE_FLAGS = CF_DEHYDRATE_FLAGS(1u32);
impl ::core::marker::Copy for CF_DEHYDRATE_FLAGS {}
impl ::core::clone::Clone for CF_DEHYDRATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_DEHYDRATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_DEHYDRATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_DEHYDRATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_DEHYDRATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_DEHYDRATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_DEHYDRATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_DEHYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
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
impl ::core::fmt::Debug for CF_FILE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_FILE_RANGE").field("StartingOffset", &self.StartingOffset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_FILE_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_FILE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_FILE_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_FILE_RANGE {}
impl ::core::default::Default for CF_FILE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Storage_FileSystem\"`*"]
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
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for CF_FS_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_FS_METADATA").field("BasicInfo", &self.BasicInfo).field("FileSize", &self.FileSize).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for CF_FS_METADATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for CF_FS_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_FS_METADATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for CF_FS_METADATA {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for CF_FS_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_HARDLINK_POLICY(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_HARDLINK_POLICY_NONE: CF_HARDLINK_POLICY = CF_HARDLINK_POLICY(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_HARDLINK_POLICY_ALLOWED: CF_HARDLINK_POLICY = CF_HARDLINK_POLICY(1u32);
impl ::core::marker::Copy for CF_HARDLINK_POLICY {}
impl ::core::clone::Clone for CF_HARDLINK_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_HARDLINK_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_HARDLINK_POLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_HARDLINK_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_HARDLINK_POLICY").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_HARDLINK_POLICY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_HARDLINK_POLICY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_HARDLINK_POLICY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_HARDLINK_POLICY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_HARDLINK_POLICY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_HYDRATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_HYDRATE_FLAG_NONE: CF_HYDRATE_FLAGS = CF_HYDRATE_FLAGS(0u32);
impl ::core::marker::Copy for CF_HYDRATE_FLAGS {}
impl ::core::clone::Clone for CF_HYDRATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_HYDRATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_HYDRATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_HYDRATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_HYDRATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_HYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_HYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_HYDRATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_HYDRATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_HYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
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
impl ::core::fmt::Debug for CF_HYDRATION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_HYDRATION_POLICY").field("Primary", &self.Primary).field("Modifier", &self.Modifier).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_HYDRATION_POLICY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_HYDRATION_POLICY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_HYDRATION_POLICY>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_HYDRATION_POLICY {}
impl ::core::default::Default for CF_HYDRATION_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_HYDRATION_POLICY_MODIFIER(pub u16);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_HYDRATION_POLICY_MODIFIER_NONE: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(0u16);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_HYDRATION_POLICY_MODIFIER_VALIDATION_REQUIRED: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(1u16);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_HYDRATION_POLICY_MODIFIER_STREAMING_ALLOWED: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(2u16);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_HYDRATION_POLICY_MODIFIER_AUTO_DEHYDRATION_ALLOWED: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(4u16);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_HYDRATION_POLICY_MODIFIER_ALLOW_FULL_RESTART_HYDRATION: CF_HYDRATION_POLICY_MODIFIER = CF_HYDRATION_POLICY_MODIFIER(8u16);
impl ::core::marker::Copy for CF_HYDRATION_POLICY_MODIFIER {}
impl ::core::clone::Clone for CF_HYDRATION_POLICY_MODIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_HYDRATION_POLICY_MODIFIER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_HYDRATION_POLICY_MODIFIER {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_HYDRATION_POLICY_MODIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_HYDRATION_POLICY_MODIFIER").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_HYDRATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_HYDRATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_HYDRATION_POLICY_MODIFIER {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_HYDRATION_POLICY_MODIFIER {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_HYDRATION_POLICY_MODIFIER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_HYDRATION_POLICY_MODIFIER_USHORT {
    pub us: u16,
}
impl ::core::marker::Copy for CF_HYDRATION_POLICY_MODIFIER_USHORT {}
impl ::core::clone::Clone for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_HYDRATION_POLICY_MODIFIER_USHORT").field("us", &self.us).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_HYDRATION_POLICY_MODIFIER_USHORT>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_HYDRATION_POLICY_MODIFIER_USHORT {}
impl ::core::default::Default for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_HYDRATION_POLICY_PRIMARY(pub u16);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_HYDRATION_POLICY_PARTIAL: CF_HYDRATION_POLICY_PRIMARY = CF_HYDRATION_POLICY_PRIMARY(0u16);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_HYDRATION_POLICY_PROGRESSIVE: CF_HYDRATION_POLICY_PRIMARY = CF_HYDRATION_POLICY_PRIMARY(1u16);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_HYDRATION_POLICY_FULL: CF_HYDRATION_POLICY_PRIMARY = CF_HYDRATION_POLICY_PRIMARY(2u16);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_HYDRATION_POLICY_ALWAYS_FULL: CF_HYDRATION_POLICY_PRIMARY = CF_HYDRATION_POLICY_PRIMARY(3u16);
impl ::core::marker::Copy for CF_HYDRATION_POLICY_PRIMARY {}
impl ::core::clone::Clone for CF_HYDRATION_POLICY_PRIMARY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_HYDRATION_POLICY_PRIMARY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_HYDRATION_POLICY_PRIMARY {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_HYDRATION_POLICY_PRIMARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_HYDRATION_POLICY_PRIMARY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_HYDRATION_POLICY_PRIMARY_USHORT {
    pub us: u16,
}
impl ::core::marker::Copy for CF_HYDRATION_POLICY_PRIMARY_USHORT {}
impl ::core::clone::Clone for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_HYDRATION_POLICY_PRIMARY_USHORT").field("us", &self.us).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_HYDRATION_POLICY_PRIMARY_USHORT>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_HYDRATION_POLICY_PRIMARY_USHORT {}
impl ::core::default::Default for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_INSYNC_POLICY(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_NONE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_TRACK_FILE_CREATION_TIME: CF_INSYNC_POLICY = CF_INSYNC_POLICY(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_TRACK_FILE_READONLY_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(2u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_TRACK_FILE_HIDDEN_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(4u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_TRACK_FILE_SYSTEM_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(8u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_CREATION_TIME: CF_INSYNC_POLICY = CF_INSYNC_POLICY(16u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_READONLY_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(32u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_HIDDEN_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(64u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_SYSTEM_ATTRIBUTE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(128u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_TRACK_FILE_LAST_WRITE_TIME: CF_INSYNC_POLICY = CF_INSYNC_POLICY(256u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_LAST_WRITE_TIME: CF_INSYNC_POLICY = CF_INSYNC_POLICY(512u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_TRACK_FILE_ALL: CF_INSYNC_POLICY = CF_INSYNC_POLICY(5592335u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_ALL: CF_INSYNC_POLICY = CF_INSYNC_POLICY(11184880u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_TRACK_ALL: CF_INSYNC_POLICY = CF_INSYNC_POLICY(16777215u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_INSYNC_POLICY_PRESERVE_INSYNC_FOR_SYNC_ENGINE: CF_INSYNC_POLICY = CF_INSYNC_POLICY(2147483648u32);
impl ::core::marker::Copy for CF_INSYNC_POLICY {}
impl ::core::clone::Clone for CF_INSYNC_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_INSYNC_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_INSYNC_POLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_INSYNC_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_INSYNC_POLICY").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_INSYNC_POLICY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_INSYNC_POLICY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_INSYNC_POLICY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_INSYNC_POLICY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_INSYNC_POLICY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_IN_SYNC_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_IN_SYNC_STATE_NOT_IN_SYNC: CF_IN_SYNC_STATE = CF_IN_SYNC_STATE(0i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_IN_SYNC_STATE_IN_SYNC: CF_IN_SYNC_STATE = CF_IN_SYNC_STATE(1i32);
impl ::core::marker::Copy for CF_IN_SYNC_STATE {}
impl ::core::clone::Clone for CF_IN_SYNC_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_IN_SYNC_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_IN_SYNC_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_IN_SYNC_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_IN_SYNC_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_MAX_PRIORITY_HINT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_MAX_PROVIDER_NAME_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_MAX_PROVIDER_VERSION_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_OPEN_FILE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPEN_FILE_FLAG_NONE: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPEN_FILE_FLAG_EXCLUSIVE: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPEN_FILE_FLAG_WRITE_ACCESS: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPEN_FILE_FLAG_DELETE_ACCESS: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPEN_FILE_FLAG_FOREGROUND: CF_OPEN_FILE_FLAGS = CF_OPEN_FILE_FLAGS(8u32);
impl ::core::marker::Copy for CF_OPEN_FILE_FLAGS {}
impl ::core::clone::Clone for CF_OPEN_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_OPEN_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_OPEN_FILE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_OPEN_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPEN_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPEN_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPEN_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPEN_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPEN_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPEN_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_OPERATION_ACK_DATA_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_ACK_DATA_FLAG_NONE: CF_OPERATION_ACK_DATA_FLAGS = CF_OPERATION_ACK_DATA_FLAGS(0u32);
impl ::core::marker::Copy for CF_OPERATION_ACK_DATA_FLAGS {}
impl ::core::clone::Clone for CF_OPERATION_ACK_DATA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_OPERATION_ACK_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_OPERATION_ACK_DATA_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_OPERATION_ACK_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_ACK_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_ACK_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_ACK_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_ACK_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_ACK_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_ACK_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_OPERATION_ACK_DEHYDRATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_ACK_DEHYDRATE_FLAG_NONE: CF_OPERATION_ACK_DEHYDRATE_FLAGS = CF_OPERATION_ACK_DEHYDRATE_FLAGS(0u32);
impl ::core::marker::Copy for CF_OPERATION_ACK_DEHYDRATE_FLAGS {}
impl ::core::clone::Clone for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_ACK_DEHYDRATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_OPERATION_ACK_DELETE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_ACK_DELETE_FLAG_NONE: CF_OPERATION_ACK_DELETE_FLAGS = CF_OPERATION_ACK_DELETE_FLAGS(0u32);
impl ::core::marker::Copy for CF_OPERATION_ACK_DELETE_FLAGS {}
impl ::core::clone::Clone for CF_OPERATION_ACK_DELETE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_OPERATION_ACK_DELETE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_OPERATION_ACK_DELETE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_OPERATION_ACK_DELETE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_ACK_DELETE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_ACK_DELETE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_ACK_DELETE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_ACK_DELETE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_ACK_DELETE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_ACK_DELETE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_OPERATION_ACK_RENAME_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_ACK_RENAME_FLAG_NONE: CF_OPERATION_ACK_RENAME_FLAGS = CF_OPERATION_ACK_RENAME_FLAGS(0u32);
impl ::core::marker::Copy for CF_OPERATION_ACK_RENAME_FLAGS {}
impl ::core::clone::Clone for CF_OPERATION_ACK_RENAME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_OPERATION_ACK_RENAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_OPERATION_ACK_RENAME_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_OPERATION_ACK_RENAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_ACK_RENAME_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_ACK_RENAME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_ACK_RENAME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_ACK_RENAME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_ACK_RENAME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_ACK_RENAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_System_CorrelationVector\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
pub struct CF_OPERATION_INFO {
    pub StructSize: u32,
    pub Type: CF_OPERATION_TYPE,
    pub ConnectionKey: CF_CONNECTION_KEY,
    pub TransferKey: i64,
    pub CorrelationVector: *const super::super::System::CorrelationVector::CORRELATION_VECTOR,
    pub SyncStatus: *const CF_SYNC_STATUS,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::fmt::Debug for CF_OPERATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_OPERATION_INFO").field("StructSize", &self.StructSize).field("Type", &self.Type).field("ConnectionKey", &self.ConnectionKey).field("TransferKey", &self.TransferKey).field("CorrelationVector", &self.CorrelationVector).field("SyncStatus", &self.SyncStatus).field("RequestKey", &self.RequestKey).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
unsafe impl ::windows::core::Abi for CF_OPERATION_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::cmp::PartialEq for CF_OPERATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_OPERATION_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::cmp::Eq for CF_OPERATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::default::Default for CF_OPERATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::core::Abi for CF_OPERATION_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::PartialEq for CF_OPERATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_OPERATION_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::Eq for CF_OPERATION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for CF_OPERATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::core::Abi for CF_OPERATION_PARAMETERS_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::PartialEq for CF_OPERATION_PARAMETERS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_OPERATION_PARAMETERS_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::Eq for CF_OPERATION_PARAMETERS_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for CF_OPERATION_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::fmt::Debug for CF_OPERATION_PARAMETERS_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_OPERATION_PARAMETERS_0_0").field("Flags", &self.Flags).field("CompletionStatus", &self.CompletionStatus).field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::core::Abi for CF_OPERATION_PARAMETERS_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_OPERATION_PARAMETERS_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::Eq for CF_OPERATION_PARAMETERS_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for CF_OPERATION_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_OPERATION_PARAMETERS_0_1 {
    pub Flags: CF_OPERATION_ACK_DEHYDRATE_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
    pub FileIdentity: *const ::core::ffi::c_void,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::fmt::Debug for CF_OPERATION_PARAMETERS_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_OPERATION_PARAMETERS_0_1").field("Flags", &self.Flags).field("CompletionStatus", &self.CompletionStatus).field("FileIdentity", &self.FileIdentity).field("FileIdentityLength", &self.FileIdentityLength).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::core::Abi for CF_OPERATION_PARAMETERS_0_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_OPERATION_PARAMETERS_0_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::Eq for CF_OPERATION_PARAMETERS_0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for CF_OPERATION_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::fmt::Debug for CF_OPERATION_PARAMETERS_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_OPERATION_PARAMETERS_0_2").field("Flags", &self.Flags).field("CompletionStatus", &self.CompletionStatus).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::core::Abi for CF_OPERATION_PARAMETERS_0_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_OPERATION_PARAMETERS_0_2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::Eq for CF_OPERATION_PARAMETERS_0_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for CF_OPERATION_PARAMETERS_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::fmt::Debug for CF_OPERATION_PARAMETERS_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_OPERATION_PARAMETERS_0_3").field("Flags", &self.Flags).field("CompletionStatus", &self.CompletionStatus).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::core::Abi for CF_OPERATION_PARAMETERS_0_3 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_OPERATION_PARAMETERS_0_3>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::Eq for CF_OPERATION_PARAMETERS_0_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for CF_OPERATION_PARAMETERS_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_OPERATION_PARAMETERS_0_4 {
    pub Flags: CF_OPERATION_RESTART_HYDRATION_FLAGS,
    pub FsMetadata: *const CF_FS_METADATA,
    pub FileIdentity: *const ::core::ffi::c_void,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::fmt::Debug for CF_OPERATION_PARAMETERS_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_OPERATION_PARAMETERS_0_4").field("Flags", &self.Flags).field("FsMetadata", &self.FsMetadata).field("FileIdentity", &self.FileIdentity).field("FileIdentityLength", &self.FileIdentityLength).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::core::Abi for CF_OPERATION_PARAMETERS_0_4 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_OPERATION_PARAMETERS_0_4>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::Eq for CF_OPERATION_PARAMETERS_0_4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for CF_OPERATION_PARAMETERS_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::fmt::Debug for CF_OPERATION_PARAMETERS_0_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_OPERATION_PARAMETERS_0_5").field("Flags", &self.Flags).field("Buffer", &self.Buffer).field("Offset", &self.Offset).field("Length", &self.Length).field("ReturnedLength", &self.ReturnedLength).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::core::Abi for CF_OPERATION_PARAMETERS_0_5 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_OPERATION_PARAMETERS_0_5>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::Eq for CF_OPERATION_PARAMETERS_0_5 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for CF_OPERATION_PARAMETERS_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_OPERATION_PARAMETERS_0_6 {
    pub Flags: CF_OPERATION_TRANSFER_DATA_FLAGS,
    pub CompletionStatus: super::super::Foundation::NTSTATUS,
    pub Buffer: *const ::core::ffi::c_void,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::fmt::Debug for CF_OPERATION_PARAMETERS_0_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_OPERATION_PARAMETERS_0_6").field("Flags", &self.Flags).field("CompletionStatus", &self.CompletionStatus).field("Buffer", &self.Buffer).field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::core::Abi for CF_OPERATION_PARAMETERS_0_6 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_OPERATION_PARAMETERS_0_6>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::Eq for CF_OPERATION_PARAMETERS_0_6 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for CF_OPERATION_PARAMETERS_0_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::fmt::Debug for CF_OPERATION_PARAMETERS_0_7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_OPERATION_PARAMETERS_0_7").field("Flags", &self.Flags).field("CompletionStatus", &self.CompletionStatus).field("PlaceholderTotalCount", &self.PlaceholderTotalCount).field("PlaceholderArray", &self.PlaceholderArray).field("PlaceholderCount", &self.PlaceholderCount).field("EntriesProcessed", &self.EntriesProcessed).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::core::Abi for CF_OPERATION_PARAMETERS_0_7 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::PartialEq for CF_OPERATION_PARAMETERS_0_7 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_OPERATION_PARAMETERS_0_7>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::Eq for CF_OPERATION_PARAMETERS_0_7 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for CF_OPERATION_PARAMETERS_0_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_OPERATION_RESTART_HYDRATION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_RESTART_HYDRATION_FLAG_NONE: CF_OPERATION_RESTART_HYDRATION_FLAGS = CF_OPERATION_RESTART_HYDRATION_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_RESTART_HYDRATION_FLAG_MARK_IN_SYNC: CF_OPERATION_RESTART_HYDRATION_FLAGS = CF_OPERATION_RESTART_HYDRATION_FLAGS(1u32);
impl ::core::marker::Copy for CF_OPERATION_RESTART_HYDRATION_FLAGS {}
impl ::core::clone::Clone for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_RESTART_HYDRATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_OPERATION_RETRIEVE_DATA_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_RETRIEVE_DATA_FLAG_NONE: CF_OPERATION_RETRIEVE_DATA_FLAGS = CF_OPERATION_RETRIEVE_DATA_FLAGS(0u32);
impl ::core::marker::Copy for CF_OPERATION_RETRIEVE_DATA_FLAGS {}
impl ::core::clone::Clone for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_RETRIEVE_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_OPERATION_TRANSFER_DATA_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_TRANSFER_DATA_FLAG_NONE: CF_OPERATION_TRANSFER_DATA_FLAGS = CF_OPERATION_TRANSFER_DATA_FLAGS(0u32);
impl ::core::marker::Copy for CF_OPERATION_TRANSFER_DATA_FLAGS {}
impl ::core::clone::Clone for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_TRANSFER_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_NONE: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_STOP_ON_ERROR: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS(2u32);
impl ::core::marker::Copy for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {}
impl ::core::clone::Clone for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_OPERATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_TYPE_TRANSFER_DATA: CF_OPERATION_TYPE = CF_OPERATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_TYPE_RETRIEVE_DATA: CF_OPERATION_TYPE = CF_OPERATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_TYPE_ACK_DATA: CF_OPERATION_TYPE = CF_OPERATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_TYPE_RESTART_HYDRATION: CF_OPERATION_TYPE = CF_OPERATION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_TYPE_TRANSFER_PLACEHOLDERS: CF_OPERATION_TYPE = CF_OPERATION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_TYPE_ACK_DEHYDRATE: CF_OPERATION_TYPE = CF_OPERATION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_TYPE_ACK_DELETE: CF_OPERATION_TYPE = CF_OPERATION_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_OPERATION_TYPE_ACK_RENAME: CF_OPERATION_TYPE = CF_OPERATION_TYPE(7i32);
impl ::core::marker::Copy for CF_OPERATION_TYPE {}
impl ::core::clone::Clone for CF_OPERATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_OPERATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_OPERATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_OPERATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_PIN_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PIN_STATE_UNSPECIFIED: CF_PIN_STATE = CF_PIN_STATE(0i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PIN_STATE_PINNED: CF_PIN_STATE = CF_PIN_STATE(1i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PIN_STATE_UNPINNED: CF_PIN_STATE = CF_PIN_STATE(2i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PIN_STATE_EXCLUDED: CF_PIN_STATE = CF_PIN_STATE(3i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PIN_STATE_INHERIT: CF_PIN_STATE = CF_PIN_STATE(4i32);
impl ::core::marker::Copy for CF_PIN_STATE {}
impl ::core::clone::Clone for CF_PIN_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_PIN_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_PIN_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_PIN_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_PIN_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
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
impl ::core::fmt::Debug for CF_PLACEHOLDER_BASIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_PLACEHOLDER_BASIC_INFO").field("PinState", &self.PinState).field("InSyncState", &self.InSyncState).field("FileId", &self.FileId).field("SyncRootFileId", &self.SyncRootFileId).field("FileIdentityLength", &self.FileIdentityLength).field("FileIdentity", &self.FileIdentity).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_PLACEHOLDER_BASIC_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_PLACEHOLDER_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_PLACEHOLDER_BASIC_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_PLACEHOLDER_BASIC_INFO {}
impl ::core::default::Default for CF_PLACEHOLDER_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_PLACEHOLDER_CREATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_CREATE_FLAG_NONE: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_CREATE_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_CREATE_FLAG_MARK_IN_SYNC: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_CREATE_FLAG_SUPERSEDE: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_CREATE_FLAG_ALWAYS_FULL: CF_PLACEHOLDER_CREATE_FLAGS = CF_PLACEHOLDER_CREATE_FLAGS(8u32);
impl ::core::marker::Copy for CF_PLACEHOLDER_CREATE_FLAGS {}
impl ::core::clone::Clone for CF_PLACEHOLDER_CREATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_PLACEHOLDER_CREATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_PLACEHOLDER_CREATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_PLACEHOLDER_CREATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_PLACEHOLDER_CREATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_PLACEHOLDER_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_PLACEHOLDER_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_PLACEHOLDER_CREATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_PLACEHOLDER_CREATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_PLACEHOLDER_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct CF_PLACEHOLDER_CREATE_INFO {
    pub RelativeFileName: ::windows::core::PCWSTR,
    pub FsMetadata: CF_FS_METADATA,
    pub FileIdentity: *const ::core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub Flags: CF_PLACEHOLDER_CREATE_FLAGS,
    pub Result: ::windows::core::HRESULT,
    pub CreateUsn: i64,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for CF_PLACEHOLDER_CREATE_INFO {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for CF_PLACEHOLDER_CREATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for CF_PLACEHOLDER_CREATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_PLACEHOLDER_CREATE_INFO").field("RelativeFileName", &self.RelativeFileName).field("FsMetadata", &self.FsMetadata).field("FileIdentity", &self.FileIdentity).field("FileIdentityLength", &self.FileIdentityLength).field("Flags", &self.Flags).field("Result", &self.Result).field("CreateUsn", &self.CreateUsn).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for CF_PLACEHOLDER_CREATE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for CF_PLACEHOLDER_CREATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_PLACEHOLDER_CREATE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for CF_PLACEHOLDER_CREATE_INFO {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for CF_PLACEHOLDER_CREATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_PLACEHOLDER_INFO_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_INFO_BASIC: CF_PLACEHOLDER_INFO_CLASS = CF_PLACEHOLDER_INFO_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_INFO_STANDARD: CF_PLACEHOLDER_INFO_CLASS = CF_PLACEHOLDER_INFO_CLASS(1i32);
impl ::core::marker::Copy for CF_PLACEHOLDER_INFO_CLASS {}
impl ::core::clone::Clone for CF_PLACEHOLDER_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_PLACEHOLDER_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_PLACEHOLDER_INFO_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_PLACEHOLDER_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_PLACEHOLDER_INFO_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_PLACEHOLDER_MANAGEMENT_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_DEFAULT: CF_PLACEHOLDER_MANAGEMENT_POLICY = CF_PLACEHOLDER_MANAGEMENT_POLICY(0i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_CREATE_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = CF_PLACEHOLDER_MANAGEMENT_POLICY(1i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_CONVERT_TO_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = CF_PLACEHOLDER_MANAGEMENT_POLICY(2i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_UPDATE_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = CF_PLACEHOLDER_MANAGEMENT_POLICY(4i32);
impl ::core::marker::Copy for CF_PLACEHOLDER_MANAGEMENT_POLICY {}
impl ::core::clone::Clone for CF_PLACEHOLDER_MANAGEMENT_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_PLACEHOLDER_MANAGEMENT_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_PLACEHOLDER_MANAGEMENT_POLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_PLACEHOLDER_MANAGEMENT_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_PLACEHOLDER_MANAGEMENT_POLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_MAX_FILE_IDENTITY_LENGTH: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_PLACEHOLDER_RANGE_INFO_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_RANGE_INFO_ONDISK: CF_PLACEHOLDER_RANGE_INFO_CLASS = CF_PLACEHOLDER_RANGE_INFO_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_RANGE_INFO_VALIDATED: CF_PLACEHOLDER_RANGE_INFO_CLASS = CF_PLACEHOLDER_RANGE_INFO_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_RANGE_INFO_MODIFIED: CF_PLACEHOLDER_RANGE_INFO_CLASS = CF_PLACEHOLDER_RANGE_INFO_CLASS(3i32);
impl ::core::marker::Copy for CF_PLACEHOLDER_RANGE_INFO_CLASS {}
impl ::core::clone::Clone for CF_PLACEHOLDER_RANGE_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_PLACEHOLDER_RANGE_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_PLACEHOLDER_RANGE_INFO_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_PLACEHOLDER_RANGE_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_PLACEHOLDER_RANGE_INFO_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
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
impl ::core::fmt::Debug for CF_PLACEHOLDER_STANDARD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_PLACEHOLDER_STANDARD_INFO")
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
unsafe impl ::windows::core::Abi for CF_PLACEHOLDER_STANDARD_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_PLACEHOLDER_STANDARD_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_PLACEHOLDER_STANDARD_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_PLACEHOLDER_STANDARD_INFO {}
impl ::core::default::Default for CF_PLACEHOLDER_STANDARD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_PLACEHOLDER_STATE(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_STATE_NO_STATES: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_STATE_PLACEHOLDER: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_STATE_SYNC_ROOT: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(2u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_STATE_ESSENTIAL_PROP_PRESENT: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(4u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_STATE_IN_SYNC: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(8u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_STATE_PARTIAL: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(16u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_STATE_PARTIALLY_ON_DISK: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(32u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PLACEHOLDER_STATE_INVALID: CF_PLACEHOLDER_STATE = CF_PLACEHOLDER_STATE(4294967295u32);
impl ::core::marker::Copy for CF_PLACEHOLDER_STATE {}
impl ::core::clone::Clone for CF_PLACEHOLDER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_PLACEHOLDER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_PLACEHOLDER_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_PLACEHOLDER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_PLACEHOLDER_STATE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_PLACEHOLDER_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_PLACEHOLDER_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_PLACEHOLDER_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_PLACEHOLDER_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_PLACEHOLDER_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
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
impl ::core::fmt::Debug for CF_PLATFORM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_PLATFORM_INFO").field("BuildNumber", &self.BuildNumber).field("RevisionNumber", &self.RevisionNumber).field("IntegrationNumber", &self.IntegrationNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_PLATFORM_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_PLATFORM_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_PLATFORM_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_PLATFORM_INFO {}
impl ::core::default::Default for CF_PLATFORM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
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
impl ::core::fmt::Debug for CF_POPULATION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_POPULATION_POLICY").field("Primary", &self.Primary).field("Modifier", &self.Modifier).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_POPULATION_POLICY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_POPULATION_POLICY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_POPULATION_POLICY>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_POPULATION_POLICY {}
impl ::core::default::Default for CF_POPULATION_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_POPULATION_POLICY_MODIFIER(pub u16);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_POPULATION_POLICY_MODIFIER_NONE: CF_POPULATION_POLICY_MODIFIER = CF_POPULATION_POLICY_MODIFIER(0u16);
impl ::core::marker::Copy for CF_POPULATION_POLICY_MODIFIER {}
impl ::core::clone::Clone for CF_POPULATION_POLICY_MODIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_POPULATION_POLICY_MODIFIER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_POPULATION_POLICY_MODIFIER {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_POPULATION_POLICY_MODIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_POPULATION_POLICY_MODIFIER").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_POPULATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_POPULATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_POPULATION_POLICY_MODIFIER {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_POPULATION_POLICY_MODIFIER {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_POPULATION_POLICY_MODIFIER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_POPULATION_POLICY_MODIFIER_USHORT {
    pub us: u16,
}
impl ::core::marker::Copy for CF_POPULATION_POLICY_MODIFIER_USHORT {}
impl ::core::clone::Clone for CF_POPULATION_POLICY_MODIFIER_USHORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_POPULATION_POLICY_MODIFIER_USHORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_POPULATION_POLICY_MODIFIER_USHORT").field("us", &self.us).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_POPULATION_POLICY_MODIFIER_USHORT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_POPULATION_POLICY_MODIFIER_USHORT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_POPULATION_POLICY_MODIFIER_USHORT>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_POPULATION_POLICY_MODIFIER_USHORT {}
impl ::core::default::Default for CF_POPULATION_POLICY_MODIFIER_USHORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_POPULATION_POLICY_PRIMARY(pub u16);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_POPULATION_POLICY_PARTIAL: CF_POPULATION_POLICY_PRIMARY = CF_POPULATION_POLICY_PRIMARY(0u16);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_POPULATION_POLICY_FULL: CF_POPULATION_POLICY_PRIMARY = CF_POPULATION_POLICY_PRIMARY(2u16);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_POPULATION_POLICY_ALWAYS_FULL: CF_POPULATION_POLICY_PRIMARY = CF_POPULATION_POLICY_PRIMARY(3u16);
impl ::core::marker::Copy for CF_POPULATION_POLICY_PRIMARY {}
impl ::core::clone::Clone for CF_POPULATION_POLICY_PRIMARY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_POPULATION_POLICY_PRIMARY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_POPULATION_POLICY_PRIMARY {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_POPULATION_POLICY_PRIMARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_POPULATION_POLICY_PRIMARY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_POPULATION_POLICY_PRIMARY_USHORT {
    pub us: u16,
}
impl ::core::marker::Copy for CF_POPULATION_POLICY_PRIMARY_USHORT {}
impl ::core::clone::Clone for CF_POPULATION_POLICY_PRIMARY_USHORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_POPULATION_POLICY_PRIMARY_USHORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_POPULATION_POLICY_PRIMARY_USHORT").field("us", &self.us).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_POPULATION_POLICY_PRIMARY_USHORT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_POPULATION_POLICY_PRIMARY_USHORT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_POPULATION_POLICY_PRIMARY_USHORT>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_POPULATION_POLICY_PRIMARY_USHORT {}
impl ::core::default::Default for CF_POPULATION_POLICY_PRIMARY_USHORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_PROCESS_INFO {
    pub StructSize: u32,
    pub ProcessId: u32,
    pub ImagePath: ::windows::core::PCWSTR,
    pub PackageName: ::windows::core::PCWSTR,
    pub ApplicationId: ::windows::core::PCWSTR,
    pub CommandLine: ::windows::core::PCWSTR,
    pub SessionId: u32,
}
impl ::core::marker::Copy for CF_PROCESS_INFO {}
impl ::core::clone::Clone for CF_PROCESS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_PROCESS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_PROCESS_INFO").field("StructSize", &self.StructSize).field("ProcessId", &self.ProcessId).field("ImagePath", &self.ImagePath).field("PackageName", &self.PackageName).field("ApplicationId", &self.ApplicationId).field("CommandLine", &self.CommandLine).field("SessionId", &self.SessionId).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_PROCESS_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_PROCESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_PROCESS_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_PROCESS_INFO {}
impl ::core::default::Default for CF_PROCESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_REGISTER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_REGISTER_FLAG_NONE: CF_REGISTER_FLAGS = CF_REGISTER_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_REGISTER_FLAG_UPDATE: CF_REGISTER_FLAGS = CF_REGISTER_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_REGISTER_FLAG_DISABLE_ON_DEMAND_POPULATION_ON_ROOT: CF_REGISTER_FLAGS = CF_REGISTER_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_REGISTER_FLAG_MARK_IN_SYNC_ON_ROOT: CF_REGISTER_FLAGS = CF_REGISTER_FLAGS(4u32);
impl ::core::marker::Copy for CF_REGISTER_FLAGS {}
impl ::core::clone::Clone for CF_REGISTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_REGISTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_REGISTER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_REGISTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_REGISTER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_REGISTER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_REGISTER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_REGISTER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_REGISTER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_REGISTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_REQUEST_KEY_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_REVERT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_REVERT_FLAG_NONE: CF_REVERT_FLAGS = CF_REVERT_FLAGS(0u32);
impl ::core::marker::Copy for CF_REVERT_FLAGS {}
impl ::core::clone::Clone for CF_REVERT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_REVERT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_REVERT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_REVERT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_REVERT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_REVERT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_REVERT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_REVERT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_REVERT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_REVERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_SET_IN_SYNC_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_SET_IN_SYNC_FLAG_NONE: CF_SET_IN_SYNC_FLAGS = CF_SET_IN_SYNC_FLAGS(0u32);
impl ::core::marker::Copy for CF_SET_IN_SYNC_FLAGS {}
impl ::core::clone::Clone for CF_SET_IN_SYNC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_SET_IN_SYNC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_SET_IN_SYNC_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_SET_IN_SYNC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_SET_IN_SYNC_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_SET_IN_SYNC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_SET_IN_SYNC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_SET_IN_SYNC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_SET_IN_SYNC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_SET_IN_SYNC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_SET_PIN_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_SET_PIN_FLAG_NONE: CF_SET_PIN_FLAGS = CF_SET_PIN_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_SET_PIN_FLAG_RECURSE: CF_SET_PIN_FLAGS = CF_SET_PIN_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_SET_PIN_FLAG_RECURSE_ONLY: CF_SET_PIN_FLAGS = CF_SET_PIN_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_SET_PIN_FLAG_RECURSE_STOP_ON_ERROR: CF_SET_PIN_FLAGS = CF_SET_PIN_FLAGS(4u32);
impl ::core::marker::Copy for CF_SET_PIN_FLAGS {}
impl ::core::clone::Clone for CF_SET_PIN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_SET_PIN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_SET_PIN_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_SET_PIN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_SET_PIN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_SET_PIN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_SET_PIN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_SET_PIN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_SET_PIN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_SET_PIN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
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
impl ::core::fmt::Debug for CF_SYNC_POLICIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_SYNC_POLICIES").field("StructSize", &self.StructSize).field("Hydration", &self.Hydration).field("Population", &self.Population).field("InSync", &self.InSync).field("HardLink", &self.HardLink).field("PlaceholderManagement", &self.PlaceholderManagement).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_SYNC_POLICIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_SYNC_POLICIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_SYNC_POLICIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_SYNC_POLICIES {}
impl ::core::default::Default for CF_SYNC_POLICIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_SYNC_PROVIDER_STATUS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PROVIDER_STATUS_DISCONNECTED: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PROVIDER_STATUS_IDLE: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PROVIDER_STATUS_POPULATE_NAMESPACE: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(2u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PROVIDER_STATUS_POPULATE_METADATA: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(4u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PROVIDER_STATUS_POPULATE_CONTENT: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(8u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PROVIDER_STATUS_SYNC_INCREMENTAL: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(16u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PROVIDER_STATUS_SYNC_FULL: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(32u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PROVIDER_STATUS_CONNECTIVITY_LOST: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(64u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PROVIDER_STATUS_CLEAR_FLAGS: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(2147483648u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PROVIDER_STATUS_TERMINATED: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(3221225473u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_PROVIDER_STATUS_ERROR: CF_SYNC_PROVIDER_STATUS = CF_SYNC_PROVIDER_STATUS(3221225474u32);
impl ::core::marker::Copy for CF_SYNC_PROVIDER_STATUS {}
impl ::core::clone::Clone for CF_SYNC_PROVIDER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_SYNC_PROVIDER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_SYNC_PROVIDER_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_SYNC_PROVIDER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_SYNC_PROVIDER_STATUS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_SYNC_PROVIDER_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_SYNC_PROVIDER_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_SYNC_PROVIDER_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_SYNC_PROVIDER_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_SYNC_PROVIDER_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_SYNC_REGISTRATION {
    pub StructSize: u32,
    pub ProviderName: ::windows::core::PCWSTR,
    pub ProviderVersion: ::windows::core::PCWSTR,
    pub SyncRootIdentity: *const ::core::ffi::c_void,
    pub SyncRootIdentityLength: u32,
    pub FileIdentity: *const ::core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub ProviderId: ::windows::core::GUID,
}
impl ::core::marker::Copy for CF_SYNC_REGISTRATION {}
impl ::core::clone::Clone for CF_SYNC_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_SYNC_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_SYNC_REGISTRATION").field("StructSize", &self.StructSize).field("ProviderName", &self.ProviderName).field("ProviderVersion", &self.ProviderVersion).field("SyncRootIdentity", &self.SyncRootIdentity).field("SyncRootIdentityLength", &self.SyncRootIdentityLength).field("FileIdentity", &self.FileIdentity).field("FileIdentityLength", &self.FileIdentityLength).field("ProviderId", &self.ProviderId).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_SYNC_REGISTRATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_SYNC_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_SYNC_REGISTRATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_SYNC_REGISTRATION {}
impl ::core::default::Default for CF_SYNC_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub struct CF_SYNC_ROOT_BASIC_INFO {
    pub SyncRootFileId: i64,
}
impl ::core::marker::Copy for CF_SYNC_ROOT_BASIC_INFO {}
impl ::core::clone::Clone for CF_SYNC_ROOT_BASIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CF_SYNC_ROOT_BASIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_SYNC_ROOT_BASIC_INFO").field("SyncRootFileId", &self.SyncRootFileId).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_SYNC_ROOT_BASIC_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_SYNC_ROOT_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_SYNC_ROOT_BASIC_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_SYNC_ROOT_BASIC_INFO {}
impl ::core::default::Default for CF_SYNC_ROOT_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_SYNC_ROOT_INFO_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_SYNC_ROOT_INFO_BASIC: CF_SYNC_ROOT_INFO_CLASS = CF_SYNC_ROOT_INFO_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_SYNC_ROOT_INFO_STANDARD: CF_SYNC_ROOT_INFO_CLASS = CF_SYNC_ROOT_INFO_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_SYNC_ROOT_INFO_PROVIDER: CF_SYNC_ROOT_INFO_CLASS = CF_SYNC_ROOT_INFO_CLASS(2i32);
impl ::core::marker::Copy for CF_SYNC_ROOT_INFO_CLASS {}
impl ::core::clone::Clone for CF_SYNC_ROOT_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_SYNC_ROOT_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_SYNC_ROOT_INFO_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_SYNC_ROOT_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_SYNC_ROOT_INFO_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
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
impl ::core::fmt::Debug for CF_SYNC_ROOT_PROVIDER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_SYNC_ROOT_PROVIDER_INFO").field("ProviderStatus", &self.ProviderStatus).field("ProviderName", &self.ProviderName).field("ProviderVersion", &self.ProviderVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_SYNC_ROOT_PROVIDER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_SYNC_ROOT_PROVIDER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_SYNC_ROOT_PROVIDER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_SYNC_ROOT_PROVIDER_INFO {}
impl ::core::default::Default for CF_SYNC_ROOT_PROVIDER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
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
impl ::core::fmt::Debug for CF_SYNC_ROOT_STANDARD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_SYNC_ROOT_STANDARD_INFO")
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
unsafe impl ::windows::core::Abi for CF_SYNC_ROOT_STANDARD_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_SYNC_ROOT_STANDARD_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_SYNC_ROOT_STANDARD_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_SYNC_ROOT_STANDARD_INFO {}
impl ::core::default::Default for CF_SYNC_ROOT_STANDARD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
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
impl ::core::fmt::Debug for CF_SYNC_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_SYNC_STATUS").field("StructSize", &self.StructSize).field("Code", &self.Code).field("DescriptionOffset", &self.DescriptionOffset).field("DescriptionLength", &self.DescriptionLength).field("DeviceIdOffset", &self.DeviceIdOffset).field("DeviceIdLength", &self.DeviceIdLength).finish()
    }
}
unsafe impl ::windows::core::Abi for CF_SYNC_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CF_SYNC_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_SYNC_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for CF_SYNC_STATUS {}
impl ::core::default::Default for CF_SYNC_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CF_UPDATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_UPDATE_FLAG_NONE: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_UPDATE_FLAG_VERIFY_IN_SYNC: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_UPDATE_FLAG_MARK_IN_SYNC: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_UPDATE_FLAG_DEHYDRATE: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_UPDATE_FLAG_ENABLE_ON_DEMAND_POPULATION: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_UPDATE_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_UPDATE_FLAG_REMOVE_FILE_IDENTITY: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_UPDATE_FLAG_CLEAR_IN_SYNC: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_UPDATE_FLAG_REMOVE_PROPERTY: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_UPDATE_FLAG_PASSTHROUGH_FS_METADATA: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_UPDATE_FLAG_ALWAYS_FULL: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
pub const CF_UPDATE_FLAG_ALLOW_PARTIAL: CF_UPDATE_FLAGS = CF_UPDATE_FLAGS(1024u32);
impl ::core::marker::Copy for CF_UPDATE_FLAGS {}
impl ::core::clone::Clone for CF_UPDATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CF_UPDATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CF_UPDATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CF_UPDATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_UPDATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_UPDATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_UPDATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_UPDATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_UPDATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_UPDATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfCloseHandle<'a, P0>(filehandle: P0)
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfCloseHandle(filehandle: super::super::Foundation::HANDLE);
    }
    CfCloseHandle(filehandle.into())
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_System_CorrelationVector\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
#[inline]
pub unsafe fn CfConnectSyncRoot<'a, P0>(syncrootpath: P0, callbacktable: *const CF_CALLBACK_REGISTRATION, callbackcontext: *const ::core::ffi::c_void, connectflags: CF_CONNECT_FLAGS) -> ::windows::core::Result<CF_CONNECTION_KEY>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfConnectSyncRoot(syncrootpath: ::windows::core::PCWSTR, callbacktable: *const CF_CALLBACK_REGISTRATION, callbackcontext: *const ::core::ffi::c_void, connectflags: CF_CONNECT_FLAGS, connectionkey: *mut CF_CONNECTION_KEY) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CfConnectSyncRoot(syncrootpath.into(), ::core::mem::transmute(callbacktable), ::core::mem::transmute(callbackcontext), connectflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CF_CONNECTION_KEY>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfConvertToPlaceholder<'a, P0>(filehandle: P0, fileidentity: *const ::core::ffi::c_void, fileidentitylength: u32, convertflags: CF_CONVERT_FLAGS, convertusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfConvertToPlaceholder(filehandle: super::super::Foundation::HANDLE, fileidentity: *const ::core::ffi::c_void, fileidentitylength: u32, convertflags: CF_CONVERT_FLAGS, convertusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT;
    }
    CfConvertToPlaceholder(filehandle.into(), ::core::mem::transmute(fileidentity), fileidentitylength, convertflags, ::core::mem::transmute(convertusn), ::core::mem::transmute(overlapped)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[inline]
pub unsafe fn CfCreatePlaceholders<'a, P0>(basedirectorypath: P0, placeholderarray: &mut [CF_PLACEHOLDER_CREATE_INFO], createflags: CF_CREATE_FLAGS, entriesprocessed: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfCreatePlaceholders(basedirectorypath: ::windows::core::PCWSTR, placeholderarray: *mut CF_PLACEHOLDER_CREATE_INFO, placeholdercount: u32, createflags: CF_CREATE_FLAGS, entriesprocessed: *mut u32) -> ::windows::core::HRESULT;
    }
    CfCreatePlaceholders(basedirectorypath.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(placeholderarray)), placeholderarray.len() as _, createflags, ::core::mem::transmute(entriesprocessed)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfDehydratePlaceholder<'a, P0>(filehandle: P0, startingoffset: i64, length: i64, dehydrateflags: CF_DEHYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfDehydratePlaceholder(filehandle: super::super::Foundation::HANDLE, startingoffset: i64, length: i64, dehydrateflags: CF_DEHYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT;
    }
    CfDehydratePlaceholder(filehandle.into(), startingoffset, length, dehydrateflags, ::core::mem::transmute(overlapped)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[inline]
pub unsafe fn CfDisconnectSyncRoot<'a, P0>(connectionkey: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<CF_CONNECTION_KEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfDisconnectSyncRoot(connectionkey: CF_CONNECTION_KEY) -> ::windows::core::HRESULT;
    }
    CfDisconnectSyncRoot(connectionkey.into()).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`, `\"Win32_System_CorrelationVector\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_CorrelationVector"))]
#[inline]
pub unsafe fn CfExecute(opinfo: *const CF_OPERATION_INFO, opparams: *mut CF_OPERATION_PARAMETERS) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfExecute(opinfo: *const CF_OPERATION_INFO, opparams: *mut CF_OPERATION_PARAMETERS) -> ::windows::core::HRESULT;
    }
    CfExecute(::core::mem::transmute(opinfo), ::core::mem::transmute(opparams)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_System_CorrelationVector\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
#[inline]
pub unsafe fn CfGetCorrelationVector<'a, P0>(filehandle: P0) -> ::windows::core::Result<super::super::System::CorrelationVector::CORRELATION_VECTOR>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfGetCorrelationVector(filehandle: super::super::Foundation::HANDLE, correlationvector: *mut super::super::System::CorrelationVector::CORRELATION_VECTOR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CfGetCorrelationVector(filehandle.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::CorrelationVector::CORRELATION_VECTOR>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetPlaceholderInfo<'a, P0>(filehandle: P0, infoclass: CF_PLACEHOLDER_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfGetPlaceholderInfo(filehandle: super::super::Foundation::HANDLE, infoclass: CF_PLACEHOLDER_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::HRESULT;
    }
    CfGetPlaceholderInfo(filehandle.into(), infoclass, ::core::mem::transmute(infobuffer), infobufferlength, ::core::mem::transmute(returnedlength)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetPlaceholderRangeInfo<'a, P0>(filehandle: P0, infoclass: CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset: i64, length: i64, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfGetPlaceholderRangeInfo(filehandle: super::super::Foundation::HANDLE, infoclass: CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset: i64, length: i64, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::HRESULT;
    }
    CfGetPlaceholderRangeInfo(filehandle.into(), infoclass, startingoffset, length, ::core::mem::transmute(infobuffer), infobufferlength, ::core::mem::transmute(returnedlength)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[inline]
pub unsafe fn CfGetPlaceholderStateFromAttributeTag(fileattributes: u32, reparsetag: u32) -> CF_PLACEHOLDER_STATE {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfGetPlaceholderStateFromAttributeTag(fileattributes: u32, reparsetag: u32) -> CF_PLACEHOLDER_STATE;
    }
    CfGetPlaceholderStateFromAttributeTag(fileattributes, reparsetag)
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[inline]
pub unsafe fn CfGetPlaceholderStateFromFileInfo(infobuffer: *const ::core::ffi::c_void, infoclass: super::FileSystem::FILE_INFO_BY_HANDLE_CLASS) -> CF_PLACEHOLDER_STATE {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfGetPlaceholderStateFromFileInfo(infobuffer: *const ::core::ffi::c_void, infoclass: super::FileSystem::FILE_INFO_BY_HANDLE_CLASS) -> CF_PLACEHOLDER_STATE;
    }
    CfGetPlaceholderStateFromFileInfo(::core::mem::transmute(infobuffer), infoclass)
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn CfGetPlaceholderStateFromFindData(finddata: *const super::FileSystem::WIN32_FIND_DATAA) -> CF_PLACEHOLDER_STATE {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfGetPlaceholderStateFromFindData(finddata: *const super::FileSystem::WIN32_FIND_DATAA) -> CF_PLACEHOLDER_STATE;
    }
    CfGetPlaceholderStateFromFindData(::core::mem::transmute(finddata))
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[inline]
pub unsafe fn CfGetPlatformInfo() -> ::windows::core::Result<CF_PLATFORM_INFO> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfGetPlatformInfo(platformversion: *mut CF_PLATFORM_INFO) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CfGetPlatformInfo(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CF_PLATFORM_INFO>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetSyncRootInfoByHandle<'a, P0>(filehandle: P0, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfGetSyncRootInfoByHandle(filehandle: super::super::Foundation::HANDLE, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::HRESULT;
    }
    CfGetSyncRootInfoByHandle(filehandle.into(), infoclass, ::core::mem::transmute(infobuffer), infobufferlength, ::core::mem::transmute(returnedlength)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[inline]
pub unsafe fn CfGetSyncRootInfoByPath<'a, P0>(filepath: P0, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfGetSyncRootInfoByPath(filepath: ::windows::core::PCWSTR, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::HRESULT;
    }
    CfGetSyncRootInfoByPath(filepath.into(), infoclass, ::core::mem::transmute(infobuffer), infobufferlength, ::core::mem::transmute(returnedlength)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetTransferKey<'a, P0>(filehandle: P0) -> ::windows::core::Result<i64>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfGetTransferKey(filehandle: super::super::Foundation::HANDLE, transferkey: *mut i64) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CfGetTransferKey(filehandle.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetWin32HandleFromProtectedHandle<'a, P0>(protectedhandle: P0) -> super::super::Foundation::HANDLE
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfGetWin32HandleFromProtectedHandle(protectedhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    }
    CfGetWin32HandleFromProtectedHandle(protectedhandle.into())
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfHydratePlaceholder<'a, P0>(filehandle: P0, startingoffset: i64, length: i64, hydrateflags: CF_HYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfHydratePlaceholder(filehandle: super::super::Foundation::HANDLE, startingoffset: i64, length: i64, hydrateflags: CF_HYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT;
    }
    CfHydratePlaceholder(filehandle.into(), startingoffset, length, hydrateflags, ::core::mem::transmute(overlapped)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfOpenFileWithOplock<'a, P0>(filepath: P0, flags: CF_OPEN_FILE_FLAGS) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfOpenFileWithOplock(filepath: ::windows::core::PCWSTR, flags: CF_OPEN_FILE_FLAGS, protectedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CfOpenFileWithOplock(filepath.into(), flags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HANDLE>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[inline]
pub unsafe fn CfQuerySyncProviderStatus<'a, P0>(connectionkey: P0) -> ::windows::core::Result<CF_SYNC_PROVIDER_STATUS>
where
    P0: ::std::convert::Into<CF_CONNECTION_KEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfQuerySyncProviderStatus(connectionkey: CF_CONNECTION_KEY, providerstatus: *mut CF_SYNC_PROVIDER_STATUS) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CfQuerySyncProviderStatus(connectionkey.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CF_SYNC_PROVIDER_STATUS>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfReferenceProtectedHandle<'a, P0>(protectedhandle: P0) -> super::super::Foundation::BOOLEAN
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfReferenceProtectedHandle(protectedhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOLEAN;
    }
    CfReferenceProtectedHandle(protectedhandle.into())
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[inline]
pub unsafe fn CfRegisterSyncRoot<'a, P0>(syncrootpath: P0, registration: *const CF_SYNC_REGISTRATION, policies: *const CF_SYNC_POLICIES, registerflags: CF_REGISTER_FLAGS) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfRegisterSyncRoot(syncrootpath: ::windows::core::PCWSTR, registration: *const CF_SYNC_REGISTRATION, policies: *const CF_SYNC_POLICIES, registerflags: CF_REGISTER_FLAGS) -> ::windows::core::HRESULT;
    }
    CfRegisterSyncRoot(syncrootpath.into(), ::core::mem::transmute(registration), ::core::mem::transmute(policies), registerflags).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfReleaseProtectedHandle<'a, P0>(protectedhandle: P0)
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfReleaseProtectedHandle(protectedhandle: super::super::Foundation::HANDLE);
    }
    CfReleaseProtectedHandle(protectedhandle.into())
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfReleaseTransferKey<'a, P0>(filehandle: P0, transferkey: *mut i64)
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfReleaseTransferKey(filehandle: super::super::Foundation::HANDLE, transferkey: *mut i64);
    }
    CfReleaseTransferKey(filehandle.into(), ::core::mem::transmute(transferkey))
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[inline]
pub unsafe fn CfReportProviderProgress<'a, P0>(connectionkey: P0, transferkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<CF_CONNECTION_KEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfReportProviderProgress(connectionkey: CF_CONNECTION_KEY, transferkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64) -> ::windows::core::HRESULT;
    }
    CfReportProviderProgress(connectionkey.into(), transferkey, providerprogresstotal, providerprogresscompleted).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[inline]
pub unsafe fn CfReportProviderProgress2<'a, P0>(connectionkey: P0, transferkey: i64, requestkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64, targetsessionid: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<CF_CONNECTION_KEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfReportProviderProgress2(connectionkey: CF_CONNECTION_KEY, transferkey: i64, requestkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64, targetsessionid: u32) -> ::windows::core::HRESULT;
    }
    CfReportProviderProgress2(connectionkey.into(), transferkey, requestkey, providerprogresstotal, providerprogresscompleted, targetsessionid).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[inline]
pub unsafe fn CfReportSyncStatus<'a, P0>(syncrootpath: P0, syncstatus: *const CF_SYNC_STATUS) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfReportSyncStatus(syncrootpath: ::windows::core::PCWSTR, syncstatus: *const CF_SYNC_STATUS) -> ::windows::core::HRESULT;
    }
    CfReportSyncStatus(syncrootpath.into(), ::core::mem::transmute(syncstatus)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfRevertPlaceholder<'a, P0>(filehandle: P0, revertflags: CF_REVERT_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfRevertPlaceholder(filehandle: super::super::Foundation::HANDLE, revertflags: CF_REVERT_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT;
    }
    CfRevertPlaceholder(filehandle.into(), revertflags, ::core::mem::transmute(overlapped)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_System_CorrelationVector\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
#[inline]
pub unsafe fn CfSetCorrelationVector<'a, P0>(filehandle: P0, correlationvector: *const super::super::System::CorrelationVector::CORRELATION_VECTOR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfSetCorrelationVector(filehandle: super::super::Foundation::HANDLE, correlationvector: *const super::super::System::CorrelationVector::CORRELATION_VECTOR) -> ::windows::core::HRESULT;
    }
    CfSetCorrelationVector(filehandle.into(), ::core::mem::transmute(correlationvector)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfSetInSyncState<'a, P0>(filehandle: P0, insyncstate: CF_IN_SYNC_STATE, insyncflags: CF_SET_IN_SYNC_FLAGS, insyncusn: *mut i64) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfSetInSyncState(filehandle: super::super::Foundation::HANDLE, insyncstate: CF_IN_SYNC_STATE, insyncflags: CF_SET_IN_SYNC_FLAGS, insyncusn: *mut i64) -> ::windows::core::HRESULT;
    }
    CfSetInSyncState(filehandle.into(), insyncstate, insyncflags, ::core::mem::transmute(insyncusn)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfSetPinState<'a, P0>(filehandle: P0, pinstate: CF_PIN_STATE, pinflags: CF_SET_PIN_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfSetPinState(filehandle: super::super::Foundation::HANDLE, pinstate: CF_PIN_STATE, pinflags: CF_SET_PIN_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT;
    }
    CfSetPinState(filehandle.into(), pinstate, pinflags, ::core::mem::transmute(overlapped)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[inline]
pub unsafe fn CfUnregisterSyncRoot<'a, P0>(syncrootpath: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfUnregisterSyncRoot(syncrootpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    CfUnregisterSyncRoot(syncrootpath.into()).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfUpdatePlaceholder<'a, P0>(filehandle: P0, fsmetadata: *const CF_FS_METADATA, fileidentity: *const ::core::ffi::c_void, fileidentitylength: u32, dehydraterangearray: &[CF_FILE_RANGE], updateflags: CF_UPDATE_FLAGS, updateusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfUpdatePlaceholder(filehandle: super::super::Foundation::HANDLE, fsmetadata: *const CF_FS_METADATA, fileidentity: *const ::core::ffi::c_void, fileidentitylength: u32, dehydraterangearray: *const CF_FILE_RANGE, dehydraterangecount: u32, updateflags: CF_UPDATE_FLAGS, updateusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT;
    }
    CfUpdatePlaceholder(filehandle.into(), ::core::mem::transmute(fsmetadata), ::core::mem::transmute(fileidentity), fileidentitylength, ::core::mem::transmute(::windows::core::as_ptr_or_null(dehydraterangearray)), dehydraterangearray.len() as _, updateflags, ::core::mem::transmute(updateusn), ::core::mem::transmute(overlapped)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_CloudFilters\"`*"]
#[inline]
pub unsafe fn CfUpdateSyncProviderStatus<'a, P0>(connectionkey: P0, providerstatus: CF_SYNC_PROVIDER_STATUS) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<CF_CONNECTION_KEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CfUpdateSyncProviderStatus(connectionkey: CF_CONNECTION_KEY, providerstatus: CF_SYNC_PROVIDER_STATUS) -> ::windows::core::HRESULT;
    }
    CfUpdateSyncProviderStatus(connectionkey.into(), providerstatus).ok()
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
