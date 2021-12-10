#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_System_CorrelationVector'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
pub type CF_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackinfo: *const CF_CALLBACK_INFO, callbackparameters: *const CF_CALLBACK_PARAMETERS)>;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_CANCEL_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_CANCEL_FLAG_NONE: CF_CALLBACK_CANCEL_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_CANCEL_FLAG_IO_TIMEOUT: CF_CALLBACK_CANCEL_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_CANCEL_FLAG_IO_ABORTED: CF_CALLBACK_CANCEL_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_CLOSE_COMPLETION_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_CLOSE_COMPLETION_FLAG_NONE: CF_CALLBACK_CLOSE_COMPLETION_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_CLOSE_COMPLETION_FLAG_DELETED: CF_CALLBACK_CLOSE_COMPLETION_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_NONE: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_BACKGROUND: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_DEHYDRATED: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_DEHYDRATE_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DEHYDRATE_FLAG_NONE: CF_CALLBACK_DEHYDRATE_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DEHYDRATE_FLAG_BACKGROUND: CF_CALLBACK_DEHYDRATE_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_DEHYDRATION_REASON = i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DEHYDRATION_REASON_NONE: CF_CALLBACK_DEHYDRATION_REASON = 0i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DEHYDRATION_REASON_USER_MANUAL: CF_CALLBACK_DEHYDRATION_REASON = 1i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_LOW_SPACE: CF_CALLBACK_DEHYDRATION_REASON = 2i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_INACTIVITY: CF_CALLBACK_DEHYDRATION_REASON = 3i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_OS_UPGRADE: CF_CALLBACK_DEHYDRATION_REASON = 4i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_DELETE_COMPLETION_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DELETE_COMPLETION_FLAG_NONE: CF_CALLBACK_DELETE_COMPLETION_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_DELETE_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DELETE_FLAG_NONE: CF_CALLBACK_DELETE_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DELETE_FLAG_IS_DIRECTORY: CF_CALLBACK_DELETE_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_DELETE_FLAG_IS_UNDELETE: CF_CALLBACK_DELETE_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_FETCH_DATA_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_FETCH_DATA_FLAG_NONE: CF_CALLBACK_FETCH_DATA_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_FETCH_DATA_FLAG_RECOVERY: CF_CALLBACK_FETCH_DATA_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_FETCH_DATA_FLAG_EXPLICIT_HYDRATION: CF_CALLBACK_FETCH_DATA_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_FETCH_PLACEHOLDERS_FLAG_NONE: CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_System_CorrelationVector'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_OPEN_COMPLETION_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_NONE: CF_CALLBACK_OPEN_COMPLETION_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNKNOWN: CF_CALLBACK_OPEN_COMPLETION_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNSUPPORTED: CF_CALLBACK_OPEN_COMPLETION_FLAGS = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_0_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_0_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_5 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_5>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_6 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_6>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_7 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_7 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_7>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_7 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_8 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_8 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_8>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_9 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_9 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_9>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_9 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_10 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_10 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_10>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_10 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_CALLBACK_PARAMETERS_0_11 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_CALLBACK_PARAMETERS_0_11 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_CALLBACK_PARAMETERS_0_11>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_CALLBACK_PARAMETERS_0_11 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_CALLBACK_PARAMETERS_0_11 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_System_CorrelationVector'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_RENAME_COMPLETION_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_RENAME_COMPLETION_FLAG_NONE: CF_CALLBACK_RENAME_COMPLETION_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_RENAME_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_RENAME_FLAG_NONE: CF_CALLBACK_RENAME_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_RENAME_FLAG_IS_DIRECTORY: CF_CALLBACK_RENAME_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_RENAME_FLAG_SOURCE_IN_SCOPE: CF_CALLBACK_RENAME_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_RENAME_FLAG_TARGET_IN_SCOPE: CF_CALLBACK_RENAME_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_FETCH_DATA: CF_CALLBACK_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_VALIDATE_DATA: CF_CALLBACK_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_CANCEL_FETCH_DATA: CF_CALLBACK_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_FETCH_PLACEHOLDERS: CF_CALLBACK_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_CANCEL_FETCH_PLACEHOLDERS: CF_CALLBACK_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_NOTIFY_FILE_OPEN_COMPLETION: CF_CALLBACK_TYPE = 5i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_NOTIFY_FILE_CLOSE_COMPLETION: CF_CALLBACK_TYPE = 6i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE: CF_CALLBACK_TYPE = 7i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE_COMPLETION: CF_CALLBACK_TYPE = 8i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_NOTIFY_DELETE: CF_CALLBACK_TYPE = 9i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_NOTIFY_DELETE_COMPLETION: CF_CALLBACK_TYPE = 10i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_NOTIFY_RENAME: CF_CALLBACK_TYPE = 11i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_NOTIFY_RENAME_COMPLETION: CF_CALLBACK_TYPE = 12i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_TYPE_NONE: CF_CALLBACK_TYPE = -1i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CALLBACK_VALIDATE_DATA_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_VALIDATE_DATA_FLAG_NONE: CF_CALLBACK_VALIDATE_DATA_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CALLBACK_VALIDATE_DATA_FLAG_EXPLICIT_HYDRATION: CF_CALLBACK_VALIDATE_DATA_FLAGS = 2u32;
pub type CF_CONNECTION_KEY = isize;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CONNECT_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CONNECT_FLAG_NONE: CF_CONNECT_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CONNECT_FLAG_REQUIRE_PROCESS_INFO: CF_CONNECT_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CONNECT_FLAG_REQUIRE_FULL_FILE_PATH: CF_CONNECT_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CONNECT_FLAG_BLOCK_SELF_IMPLICIT_HYDRATION: CF_CONNECT_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CONVERT_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CONVERT_FLAG_NONE: CF_CONVERT_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CONVERT_FLAG_MARK_IN_SYNC: CF_CONVERT_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CONVERT_FLAG_DEHYDRATE: CF_CONVERT_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CONVERT_FLAG_ENABLE_ON_DEMAND_POPULATION: CF_CONVERT_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CONVERT_FLAG_ALWAYS_FULL: CF_CONVERT_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CONVERT_FLAG_FORCE_CONVERT_TO_CLOUD_FILE: CF_CONVERT_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_CREATE_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CREATE_FLAG_NONE: CF_CREATE_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_CREATE_FLAG_STOP_ON_ERROR: CF_CREATE_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_DEHYDRATE_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_DEHYDRATE_FLAG_NONE: CF_DEHYDRATE_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_DEHYDRATE_FLAG_BACKGROUND: CF_DEHYDRATE_FLAGS = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Storage_FileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_HARDLINK_POLICY = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_HARDLINK_POLICY_NONE: CF_HARDLINK_POLICY = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_HARDLINK_POLICY_ALLOWED: CF_HARDLINK_POLICY = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_HYDRATE_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_HYDRATE_FLAG_NONE: CF_HYDRATE_FLAGS = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_HYDRATION_POLICY_MODIFIER = u16;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_HYDRATION_POLICY_MODIFIER_NONE: CF_HYDRATION_POLICY_MODIFIER = 0u16;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_HYDRATION_POLICY_MODIFIER_VALIDATION_REQUIRED: CF_HYDRATION_POLICY_MODIFIER = 1u16;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_HYDRATION_POLICY_MODIFIER_STREAMING_ALLOWED: CF_HYDRATION_POLICY_MODIFIER = 2u16;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_HYDRATION_POLICY_MODIFIER_AUTO_DEHYDRATION_ALLOWED: CF_HYDRATION_POLICY_MODIFIER = 4u16;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_HYDRATION_POLICY_MODIFIER_ALLOW_FULL_RESTART_HYDRATION: CF_HYDRATION_POLICY_MODIFIER = 8u16;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub struct CF_HYDRATION_POLICY_MODIFIER_USHORT {
    pub us: u16,
}
impl ::core::marker::Copy for CF_HYDRATION_POLICY_MODIFIER_USHORT {}
impl ::core::clone::Clone for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    fn clone(&self) -> Self {
        *self
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_HYDRATION_POLICY_PRIMARY = u16;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_HYDRATION_POLICY_PARTIAL: CF_HYDRATION_POLICY_PRIMARY = 0u16;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_HYDRATION_POLICY_PROGRESSIVE: CF_HYDRATION_POLICY_PRIMARY = 1u16;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_HYDRATION_POLICY_FULL: CF_HYDRATION_POLICY_PRIMARY = 2u16;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_HYDRATION_POLICY_ALWAYS_FULL: CF_HYDRATION_POLICY_PRIMARY = 3u16;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub struct CF_HYDRATION_POLICY_PRIMARY_USHORT {
    pub us: u16,
}
impl ::core::marker::Copy for CF_HYDRATION_POLICY_PRIMARY_USHORT {}
impl ::core::clone::Clone for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    fn clone(&self) -> Self {
        *self
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_INSYNC_POLICY = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_NONE: CF_INSYNC_POLICY = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_TRACK_FILE_CREATION_TIME: CF_INSYNC_POLICY = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_TRACK_FILE_READONLY_ATTRIBUTE: CF_INSYNC_POLICY = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_TRACK_FILE_HIDDEN_ATTRIBUTE: CF_INSYNC_POLICY = 4u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_TRACK_FILE_SYSTEM_ATTRIBUTE: CF_INSYNC_POLICY = 8u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_CREATION_TIME: CF_INSYNC_POLICY = 16u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_READONLY_ATTRIBUTE: CF_INSYNC_POLICY = 32u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_HIDDEN_ATTRIBUTE: CF_INSYNC_POLICY = 64u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_SYSTEM_ATTRIBUTE: CF_INSYNC_POLICY = 128u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_TRACK_FILE_LAST_WRITE_TIME: CF_INSYNC_POLICY = 256u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_LAST_WRITE_TIME: CF_INSYNC_POLICY = 512u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_TRACK_FILE_ALL: CF_INSYNC_POLICY = 5592335u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_ALL: CF_INSYNC_POLICY = 11184880u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_TRACK_ALL: CF_INSYNC_POLICY = 16777215u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_INSYNC_POLICY_PRESERVE_INSYNC_FOR_SYNC_ENGINE: CF_INSYNC_POLICY = 2147483648u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_IN_SYNC_STATE = i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_IN_SYNC_STATE_NOT_IN_SYNC: CF_IN_SYNC_STATE = 0i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_IN_SYNC_STATE_IN_SYNC: CF_IN_SYNC_STATE = 1i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_MAX_PRIORITY_HINT: u32 = 15u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_MAX_PROVIDER_NAME_LENGTH: u32 = 255u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_MAX_PROVIDER_VERSION_LENGTH: u32 = 255u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_OPEN_FILE_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPEN_FILE_FLAG_NONE: CF_OPEN_FILE_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPEN_FILE_FLAG_EXCLUSIVE: CF_OPEN_FILE_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPEN_FILE_FLAG_WRITE_ACCESS: CF_OPEN_FILE_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPEN_FILE_FLAG_DELETE_ACCESS: CF_OPEN_FILE_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPEN_FILE_FLAG_FOREGROUND: CF_OPEN_FILE_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_OPERATION_ACK_DATA_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_ACK_DATA_FLAG_NONE: CF_OPERATION_ACK_DATA_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_OPERATION_ACK_DEHYDRATE_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_ACK_DEHYDRATE_FLAG_NONE: CF_OPERATION_ACK_DEHYDRATE_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_OPERATION_ACK_DELETE_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_ACK_DELETE_FLAG_NONE: CF_OPERATION_ACK_DELETE_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_OPERATION_ACK_RENAME_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_ACK_RENAME_FLAG_NONE: CF_OPERATION_ACK_RENAME_FLAGS = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_System_CorrelationVector'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_OPERATION_RESTART_HYDRATION_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_RESTART_HYDRATION_FLAG_NONE: CF_OPERATION_RESTART_HYDRATION_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_RESTART_HYDRATION_FLAG_MARK_IN_SYNC: CF_OPERATION_RESTART_HYDRATION_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_OPERATION_RETRIEVE_DATA_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_RETRIEVE_DATA_FLAG_NONE: CF_OPERATION_RETRIEVE_DATA_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_OPERATION_TRANSFER_DATA_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_TRANSFER_DATA_FLAG_NONE: CF_OPERATION_TRANSFER_DATA_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_NONE: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_STOP_ON_ERROR: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_OPERATION_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_TYPE_TRANSFER_DATA: CF_OPERATION_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_TYPE_RETRIEVE_DATA: CF_OPERATION_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_TYPE_ACK_DATA: CF_OPERATION_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_TYPE_RESTART_HYDRATION: CF_OPERATION_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_TYPE_TRANSFER_PLACEHOLDERS: CF_OPERATION_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_TYPE_ACK_DEHYDRATE: CF_OPERATION_TYPE = 5i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_TYPE_ACK_DELETE: CF_OPERATION_TYPE = 6i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_OPERATION_TYPE_ACK_RENAME: CF_OPERATION_TYPE = 7i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_PIN_STATE = i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PIN_STATE_UNSPECIFIED: CF_PIN_STATE = 0i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PIN_STATE_PINNED: CF_PIN_STATE = 1i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PIN_STATE_UNPINNED: CF_PIN_STATE = 2i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PIN_STATE_EXCLUDED: CF_PIN_STATE = 3i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PIN_STATE_INHERIT: CF_PIN_STATE = 4i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_PLACEHOLDER_CREATE_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_CREATE_FLAG_NONE: CF_PLACEHOLDER_CREATE_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_CREATE_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_PLACEHOLDER_CREATE_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_CREATE_FLAG_MARK_IN_SYNC: CF_PLACEHOLDER_CREATE_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_CREATE_FLAG_SUPERSEDE: CF_PLACEHOLDER_CREATE_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_CREATE_FLAG_ALWAYS_FULL: CF_PLACEHOLDER_CREATE_FLAGS = 8u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct CF_PLACEHOLDER_CREATE_INFO {
    pub RelativeFileName: super::super::Foundation::PWSTR,
    pub FsMetadata: CF_FS_METADATA,
    pub FileIdentity: *mut ::core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub Flags: CF_PLACEHOLDER_CREATE_FLAGS,
    pub Result: ::windows::core::HRESULT,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::core::Abi for CF_PLACEHOLDER_CREATE_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::PartialEq for CF_PLACEHOLDER_CREATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_PLACEHOLDER_CREATE_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::Eq for CF_PLACEHOLDER_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for CF_PLACEHOLDER_CREATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_PLACEHOLDER_INFO_CLASS = i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_INFO_BASIC: CF_PLACEHOLDER_INFO_CLASS = 0i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_INFO_STANDARD: CF_PLACEHOLDER_INFO_CLASS = 1i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_PLACEHOLDER_MANAGEMENT_POLICY = i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_DEFAULT: CF_PLACEHOLDER_MANAGEMENT_POLICY = 0i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_CREATE_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = 1i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_CONVERT_TO_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = 2i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_UPDATE_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = 4i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_MAX_FILE_IDENTITY_LENGTH: u32 = 4096u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_PLACEHOLDER_RANGE_INFO_CLASS = i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_RANGE_INFO_ONDISK: CF_PLACEHOLDER_RANGE_INFO_CLASS = 1i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_RANGE_INFO_VALIDATED: CF_PLACEHOLDER_RANGE_INFO_CLASS = 2i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_RANGE_INFO_MODIFIED: CF_PLACEHOLDER_RANGE_INFO_CLASS = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_PLACEHOLDER_STATE = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_STATE_NO_STATES: CF_PLACEHOLDER_STATE = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_STATE_PLACEHOLDER: CF_PLACEHOLDER_STATE = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_STATE_SYNC_ROOT: CF_PLACEHOLDER_STATE = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_STATE_ESSENTIAL_PROP_PRESENT: CF_PLACEHOLDER_STATE = 4u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_STATE_IN_SYNC: CF_PLACEHOLDER_STATE = 8u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_STATE_PARTIAL: CF_PLACEHOLDER_STATE = 16u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_STATE_PARTIALLY_ON_DISK: CF_PLACEHOLDER_STATE = 32u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PLACEHOLDER_STATE_INVALID: CF_PLACEHOLDER_STATE = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_POPULATION_POLICY_MODIFIER = u16;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_POPULATION_POLICY_MODIFIER_NONE: CF_POPULATION_POLICY_MODIFIER = 0u16;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub struct CF_POPULATION_POLICY_MODIFIER_USHORT {
    pub us: u16,
}
impl ::core::marker::Copy for CF_POPULATION_POLICY_MODIFIER_USHORT {}
impl ::core::clone::Clone for CF_POPULATION_POLICY_MODIFIER_USHORT {
    fn clone(&self) -> Self {
        *self
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_POPULATION_POLICY_PRIMARY = u16;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_POPULATION_POLICY_PARTIAL: CF_POPULATION_POLICY_PRIMARY = 0u16;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_POPULATION_POLICY_FULL: CF_POPULATION_POLICY_PRIMARY = 2u16;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_POPULATION_POLICY_ALWAYS_FULL: CF_POPULATION_POLICY_PRIMARY = 3u16;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub struct CF_POPULATION_POLICY_PRIMARY_USHORT {
    pub us: u16,
}
impl ::core::marker::Copy for CF_POPULATION_POLICY_PRIMARY_USHORT {}
impl ::core::clone::Clone for CF_POPULATION_POLICY_PRIMARY_USHORT {
    fn clone(&self) -> Self {
        *self
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_PROCESS_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_PROCESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_PROCESS_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_PROCESS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_PROCESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_REGISTER_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_REGISTER_FLAG_NONE: CF_REGISTER_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_REGISTER_FLAG_UPDATE: CF_REGISTER_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_REGISTER_FLAG_DISABLE_ON_DEMAND_POPULATION_ON_ROOT: CF_REGISTER_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_REGISTER_FLAG_MARK_IN_SYNC_ON_ROOT: CF_REGISTER_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_REQUEST_KEY_DEFAULT: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_REVERT_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_REVERT_FLAG_NONE: CF_REVERT_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_SET_IN_SYNC_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_SET_IN_SYNC_FLAG_NONE: CF_SET_IN_SYNC_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_SET_PIN_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_SET_PIN_FLAG_NONE: CF_SET_PIN_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_SET_PIN_FLAG_RECURSE: CF_SET_PIN_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_SET_PIN_FLAG_RECURSE_ONLY: CF_SET_PIN_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_SET_PIN_FLAG_RECURSE_STOP_ON_ERROR: CF_SET_PIN_FLAGS = 4u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_SYNC_PROVIDER_STATUS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PROVIDER_STATUS_DISCONNECTED: CF_SYNC_PROVIDER_STATUS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PROVIDER_STATUS_IDLE: CF_SYNC_PROVIDER_STATUS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PROVIDER_STATUS_POPULATE_NAMESPACE: CF_SYNC_PROVIDER_STATUS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PROVIDER_STATUS_POPULATE_METADATA: CF_SYNC_PROVIDER_STATUS = 4u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PROVIDER_STATUS_POPULATE_CONTENT: CF_SYNC_PROVIDER_STATUS = 8u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PROVIDER_STATUS_SYNC_INCREMENTAL: CF_SYNC_PROVIDER_STATUS = 16u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PROVIDER_STATUS_SYNC_FULL: CF_SYNC_PROVIDER_STATUS = 32u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PROVIDER_STATUS_CONNECTIVITY_LOST: CF_SYNC_PROVIDER_STATUS = 64u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PROVIDER_STATUS_CLEAR_FLAGS: CF_SYNC_PROVIDER_STATUS = 2147483648u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PROVIDER_STATUS_TERMINATED: CF_SYNC_PROVIDER_STATUS = 3221225473u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_PROVIDER_STATUS_ERROR: CF_SYNC_PROVIDER_STATUS = 3221225474u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CF_SYNC_REGISTRATION {
    pub StructSize: u32,
    pub ProviderName: super::super::Foundation::PWSTR,
    pub ProviderVersion: super::super::Foundation::PWSTR,
    pub SyncRootIdentity: *mut ::core::ffi::c_void,
    pub SyncRootIdentityLength: u32,
    pub FileIdentity: *mut ::core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub ProviderId: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CF_SYNC_REGISTRATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CF_SYNC_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CF_SYNC_REGISTRATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CF_SYNC_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CF_SYNC_REGISTRATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CF_SYNC_REGISTRATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CF_SYNC_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub struct CF_SYNC_ROOT_BASIC_INFO {
    pub SyncRootFileId: i64,
}
impl ::core::marker::Copy for CF_SYNC_ROOT_BASIC_INFO {}
impl ::core::clone::Clone for CF_SYNC_ROOT_BASIC_INFO {
    fn clone(&self) -> Self {
        *self
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_SYNC_ROOT_INFO_CLASS = i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_SYNC_ROOT_INFO_BASIC: CF_SYNC_ROOT_INFO_CLASS = 0i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_SYNC_ROOT_INFO_STANDARD: CF_SYNC_ROOT_INFO_CLASS = 1i32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_SYNC_ROOT_INFO_PROVIDER: CF_SYNC_ROOT_INFO_CLASS = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
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
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub type CF_UPDATE_FLAGS = u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_UPDATE_FLAG_NONE: CF_UPDATE_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_UPDATE_FLAG_VERIFY_IN_SYNC: CF_UPDATE_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_UPDATE_FLAG_MARK_IN_SYNC: CF_UPDATE_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_UPDATE_FLAG_DEHYDRATE: CF_UPDATE_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_UPDATE_FLAG_ENABLE_ON_DEMAND_POPULATION: CF_UPDATE_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_UPDATE_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_UPDATE_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_UPDATE_FLAG_REMOVE_FILE_IDENTITY: CF_UPDATE_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_UPDATE_FLAG_CLEAR_IN_SYNC: CF_UPDATE_FLAGS = 64u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_UPDATE_FLAG_REMOVE_PROPERTY: CF_UPDATE_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_UPDATE_FLAG_PASSTHROUGH_FS_METADATA: CF_UPDATE_FLAGS = 256u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_UPDATE_FLAG_ALWAYS_FULL: CF_UPDATE_FLAGS = 512u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
pub const CF_UPDATE_FLAG_ALLOW_PARTIAL: CF_UPDATE_FLAGS = 1024u32;
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfCloseHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfCloseHandle(filehandle: super::super::Foundation::HANDLE);
        }
        CfCloseHandle(filehandle.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_System_CorrelationVector'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
#[inline]
pub unsafe fn CfConnectSyncRoot<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(syncrootpath: Param0, callbacktable: *const CF_CALLBACK_REGISTRATION, callbackcontext: *const ::core::ffi::c_void, connectflags: CF_CONNECT_FLAGS) -> ::windows::core::Result<CF_CONNECTION_KEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfConnectSyncRoot(syncrootpath: super::super::Foundation::PWSTR, callbacktable: *const CF_CALLBACK_REGISTRATION, callbackcontext: *const ::core::ffi::c_void, connectflags: CF_CONNECT_FLAGS, connectionkey: *mut CF_CONNECTION_KEY) -> ::windows::core::HRESULT;
        }
        let mut result__: CF_CONNECTION_KEY = ::core::mem::zeroed();
        CfConnectSyncRoot(syncrootpath.into_param().abi(), ::core::mem::transmute(callbacktable), ::core::mem::transmute(callbackcontext), ::core::mem::transmute(connectflags), ::core::mem::transmute(&mut result__)).from_abi::<CF_CONNECTION_KEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfConvertToPlaceholder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, fileidentity: *const ::core::ffi::c_void, fileidentitylength: u32, convertflags: CF_CONVERT_FLAGS, convertusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfConvertToPlaceholder(filehandle: super::super::Foundation::HANDLE, fileidentity: *const ::core::ffi::c_void, fileidentitylength: u32, convertflags: CF_CONVERT_FLAGS, convertusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT;
        }
        CfConvertToPlaceholder(filehandle.into_param().abi(), ::core::mem::transmute(fileidentity), ::core::mem::transmute(fileidentitylength), ::core::mem::transmute(convertflags), ::core::mem::transmute(convertusn), ::core::mem::transmute(overlapped)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn CfCreatePlaceholders<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(basedirectorypath: Param0, placeholderarray: *mut CF_PLACEHOLDER_CREATE_INFO, placeholdercount: u32, createflags: CF_CREATE_FLAGS, entriesprocessed: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfCreatePlaceholders(basedirectorypath: super::super::Foundation::PWSTR, placeholderarray: *mut CF_PLACEHOLDER_CREATE_INFO, placeholdercount: u32, createflags: CF_CREATE_FLAGS, entriesprocessed: *mut u32) -> ::windows::core::HRESULT;
        }
        CfCreatePlaceholders(basedirectorypath.into_param().abi(), ::core::mem::transmute(placeholderarray), ::core::mem::transmute(placeholdercount), ::core::mem::transmute(createflags), ::core::mem::transmute(entriesprocessed)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfDehydratePlaceholder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, startingoffset: i64, length: i64, dehydrateflags: CF_DEHYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfDehydratePlaceholder(filehandle: super::super::Foundation::HANDLE, startingoffset: i64, length: i64, dehydrateflags: CF_DEHYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT;
        }
        CfDehydratePlaceholder(filehandle.into_param().abi(), ::core::mem::transmute(startingoffset), ::core::mem::transmute(length), ::core::mem::transmute(dehydrateflags), ::core::mem::transmute(overlapped)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
#[inline]
pub unsafe fn CfDisconnectSyncRoot<'a, Param0: ::windows::core::IntoParam<'a, CF_CONNECTION_KEY>>(connectionkey: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfDisconnectSyncRoot(connectionkey: CF_CONNECTION_KEY) -> ::windows::core::HRESULT;
        }
        CfDisconnectSyncRoot(connectionkey.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem', 'Win32_System_CorrelationVector'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_CorrelationVector"))]
#[inline]
pub unsafe fn CfExecute(opinfo: *const CF_OPERATION_INFO, opparams: *mut CF_OPERATION_PARAMETERS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfExecute(opinfo: *const CF_OPERATION_INFO, opparams: *mut CF_OPERATION_PARAMETERS) -> ::windows::core::HRESULT;
        }
        CfExecute(::core::mem::transmute(opinfo), ::core::mem::transmute(opparams)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_System_CorrelationVector'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
#[inline]
pub unsafe fn CfGetCorrelationVector<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0) -> ::windows::core::Result<super::super::System::CorrelationVector::CORRELATION_VECTOR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetCorrelationVector(filehandle: super::super::Foundation::HANDLE, correlationvector: *mut super::super::System::CorrelationVector::CORRELATION_VECTOR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::System::CorrelationVector::CORRELATION_VECTOR = ::core::mem::zeroed();
        CfGetCorrelationVector(filehandle.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::CorrelationVector::CORRELATION_VECTOR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetPlaceholderInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, infoclass: CF_PLACEHOLDER_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetPlaceholderInfo(filehandle: super::super::Foundation::HANDLE, infoclass: CF_PLACEHOLDER_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::HRESULT;
        }
        CfGetPlaceholderInfo(filehandle.into_param().abi(), ::core::mem::transmute(infoclass), ::core::mem::transmute(infobuffer), ::core::mem::transmute(infobufferlength), ::core::mem::transmute(returnedlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetPlaceholderRangeInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, infoclass: CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset: i64, length: i64, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetPlaceholderRangeInfo(filehandle: super::super::Foundation::HANDLE, infoclass: CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset: i64, length: i64, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::HRESULT;
        }
        CfGetPlaceholderRangeInfo(filehandle.into_param().abi(), ::core::mem::transmute(infoclass), ::core::mem::transmute(startingoffset), ::core::mem::transmute(length), ::core::mem::transmute(infobuffer), ::core::mem::transmute(infobufferlength), ::core::mem::transmute(returnedlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
#[inline]
pub unsafe fn CfGetPlaceholderStateFromAttributeTag(fileattributes: u32, reparsetag: u32) -> CF_PLACEHOLDER_STATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetPlaceholderStateFromAttributeTag(fileattributes: u32, reparsetag: u32) -> CF_PLACEHOLDER_STATE;
        }
        ::core::mem::transmute(CfGetPlaceholderStateFromAttributeTag(::core::mem::transmute(fileattributes), ::core::mem::transmute(reparsetag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Storage_FileSystem'*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
#[inline]
pub unsafe fn CfGetPlaceholderStateFromFileInfo(infobuffer: *const ::core::ffi::c_void, infoclass: super::FileSystem::FILE_INFO_BY_HANDLE_CLASS) -> CF_PLACEHOLDER_STATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetPlaceholderStateFromFileInfo(infobuffer: *const ::core::ffi::c_void, infoclass: super::FileSystem::FILE_INFO_BY_HANDLE_CLASS) -> CF_PLACEHOLDER_STATE;
        }
        ::core::mem::transmute(CfGetPlaceholderStateFromFileInfo(::core::mem::transmute(infobuffer), ::core::mem::transmute(infoclass)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn CfGetPlaceholderStateFromFindData(finddata: *const super::FileSystem::WIN32_FIND_DATAA) -> CF_PLACEHOLDER_STATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetPlaceholderStateFromFindData(finddata: *const super::FileSystem::WIN32_FIND_DATAA) -> CF_PLACEHOLDER_STATE;
        }
        ::core::mem::transmute(CfGetPlaceholderStateFromFindData(::core::mem::transmute(finddata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
#[inline]
pub unsafe fn CfGetPlatformInfo() -> ::windows::core::Result<CF_PLATFORM_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetPlatformInfo(platformversion: *mut CF_PLATFORM_INFO) -> ::windows::core::HRESULT;
        }
        let mut result__: CF_PLATFORM_INFO = ::core::mem::zeroed();
        CfGetPlatformInfo(::core::mem::transmute(&mut result__)).from_abi::<CF_PLATFORM_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetSyncRootInfoByHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetSyncRootInfoByHandle(filehandle: super::super::Foundation::HANDLE, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::HRESULT;
        }
        CfGetSyncRootInfoByHandle(filehandle.into_param().abi(), ::core::mem::transmute(infoclass), ::core::mem::transmute(infobuffer), ::core::mem::transmute(infobufferlength), ::core::mem::transmute(returnedlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetSyncRootInfoByPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(filepath: Param0, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetSyncRootInfoByPath(filepath: super::super::Foundation::PWSTR, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut ::core::ffi::c_void, infobufferlength: u32, returnedlength: *mut u32) -> ::windows::core::HRESULT;
        }
        CfGetSyncRootInfoByPath(filepath.into_param().abi(), ::core::mem::transmute(infoclass), ::core::mem::transmute(infobuffer), ::core::mem::transmute(infobufferlength), ::core::mem::transmute(returnedlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetTransferKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0) -> ::windows::core::Result<i64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetTransferKey(filehandle: super::super::Foundation::HANDLE, transferkey: *mut i64) -> ::windows::core::HRESULT;
        }
        let mut result__: i64 = ::core::mem::zeroed();
        CfGetTransferKey(filehandle.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfGetWin32HandleFromProtectedHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(protectedhandle: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfGetWin32HandleFromProtectedHandle(protectedhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CfGetWin32HandleFromProtectedHandle(protectedhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfHydratePlaceholder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, startingoffset: i64, length: i64, hydrateflags: CF_HYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfHydratePlaceholder(filehandle: super::super::Foundation::HANDLE, startingoffset: i64, length: i64, hydrateflags: CF_HYDRATE_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT;
        }
        CfHydratePlaceholder(filehandle.into_param().abi(), ::core::mem::transmute(startingoffset), ::core::mem::transmute(length), ::core::mem::transmute(hydrateflags), ::core::mem::transmute(overlapped)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfOpenFileWithOplock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(filepath: Param0, flags: CF_OPEN_FILE_FLAGS) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfOpenFileWithOplock(filepath: super::super::Foundation::PWSTR, flags: CF_OPEN_FILE_FLAGS, protectedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        CfOpenFileWithOplock(filepath.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
#[inline]
pub unsafe fn CfQuerySyncProviderStatus<'a, Param0: ::windows::core::IntoParam<'a, CF_CONNECTION_KEY>>(connectionkey: Param0) -> ::windows::core::Result<CF_SYNC_PROVIDER_STATUS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfQuerySyncProviderStatus(connectionkey: CF_CONNECTION_KEY, providerstatus: *mut CF_SYNC_PROVIDER_STATUS) -> ::windows::core::HRESULT;
        }
        let mut result__: CF_SYNC_PROVIDER_STATUS = ::core::mem::zeroed();
        CfQuerySyncProviderStatus(connectionkey.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<CF_SYNC_PROVIDER_STATUS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfReferenceProtectedHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(protectedhandle: Param0) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfReferenceProtectedHandle(protectedhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(CfReferenceProtectedHandle(protectedhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfRegisterSyncRoot<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(syncrootpath: Param0, registration: *const CF_SYNC_REGISTRATION, policies: *const CF_SYNC_POLICIES, registerflags: CF_REGISTER_FLAGS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfRegisterSyncRoot(syncrootpath: super::super::Foundation::PWSTR, registration: *const CF_SYNC_REGISTRATION, policies: *const CF_SYNC_POLICIES, registerflags: CF_REGISTER_FLAGS) -> ::windows::core::HRESULT;
        }
        CfRegisterSyncRoot(syncrootpath.into_param().abi(), ::core::mem::transmute(registration), ::core::mem::transmute(policies), ::core::mem::transmute(registerflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfReleaseProtectedHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(protectedhandle: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfReleaseProtectedHandle(protectedhandle: super::super::Foundation::HANDLE);
        }
        CfReleaseProtectedHandle(protectedhandle.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfReleaseTransferKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, transferkey: *mut i64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfReleaseTransferKey(filehandle: super::super::Foundation::HANDLE, transferkey: *mut i64);
        }
        CfReleaseTransferKey(filehandle.into_param().abi(), ::core::mem::transmute(transferkey))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
#[inline]
pub unsafe fn CfReportProviderProgress<'a, Param0: ::windows::core::IntoParam<'a, CF_CONNECTION_KEY>>(connectionkey: Param0, transferkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfReportProviderProgress(connectionkey: CF_CONNECTION_KEY, transferkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64) -> ::windows::core::HRESULT;
        }
        CfReportProviderProgress(connectionkey.into_param().abi(), ::core::mem::transmute(transferkey), ::core::mem::transmute(providerprogresstotal), ::core::mem::transmute(providerprogresscompleted)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
#[inline]
pub unsafe fn CfReportProviderProgress2<'a, Param0: ::windows::core::IntoParam<'a, CF_CONNECTION_KEY>>(connectionkey: Param0, transferkey: i64, requestkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64, targetsessionid: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfReportProviderProgress2(connectionkey: CF_CONNECTION_KEY, transferkey: i64, requestkey: i64, providerprogresstotal: i64, providerprogresscompleted: i64, targetsessionid: u32) -> ::windows::core::HRESULT;
        }
        CfReportProviderProgress2(connectionkey.into_param().abi(), ::core::mem::transmute(transferkey), ::core::mem::transmute(requestkey), ::core::mem::transmute(providerprogresstotal), ::core::mem::transmute(providerprogresscompleted), ::core::mem::transmute(targetsessionid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfReportSyncStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(syncrootpath: Param0, syncstatus: *const CF_SYNC_STATUS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfReportSyncStatus(syncrootpath: super::super::Foundation::PWSTR, syncstatus: *const CF_SYNC_STATUS) -> ::windows::core::HRESULT;
        }
        CfReportSyncStatus(syncrootpath.into_param().abi(), ::core::mem::transmute(syncstatus)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfRevertPlaceholder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, revertflags: CF_REVERT_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfRevertPlaceholder(filehandle: super::super::Foundation::HANDLE, revertflags: CF_REVERT_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT;
        }
        CfRevertPlaceholder(filehandle.into_param().abi(), ::core::mem::transmute(revertflags), ::core::mem::transmute(overlapped)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_System_CorrelationVector'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
#[inline]
pub unsafe fn CfSetCorrelationVector<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, correlationvector: *const super::super::System::CorrelationVector::CORRELATION_VECTOR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfSetCorrelationVector(filehandle: super::super::Foundation::HANDLE, correlationvector: *const super::super::System::CorrelationVector::CORRELATION_VECTOR) -> ::windows::core::HRESULT;
        }
        CfSetCorrelationVector(filehandle.into_param().abi(), ::core::mem::transmute(correlationvector)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfSetInSyncState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, insyncstate: CF_IN_SYNC_STATE, insyncflags: CF_SET_IN_SYNC_FLAGS, insyncusn: *mut i64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfSetInSyncState(filehandle: super::super::Foundation::HANDLE, insyncstate: CF_IN_SYNC_STATE, insyncflags: CF_SET_IN_SYNC_FLAGS, insyncusn: *mut i64) -> ::windows::core::HRESULT;
        }
        CfSetInSyncState(filehandle.into_param().abi(), ::core::mem::transmute(insyncstate), ::core::mem::transmute(insyncflags), ::core::mem::transmute(insyncusn)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfSetPinState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, pinstate: CF_PIN_STATE, pinflags: CF_SET_PIN_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfSetPinState(filehandle: super::super::Foundation::HANDLE, pinstate: CF_PIN_STATE, pinflags: CF_SET_PIN_FLAGS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT;
        }
        CfSetPinState(filehandle.into_param().abi(), ::core::mem::transmute(pinstate), ::core::mem::transmute(pinflags), ::core::mem::transmute(overlapped)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CfUnregisterSyncRoot<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(syncrootpath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfUnregisterSyncRoot(syncrootpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        CfUnregisterSyncRoot(syncrootpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters', 'Win32_Foundation', 'Win32_Storage_FileSystem', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CfUpdatePlaceholder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, fsmetadata: *const CF_FS_METADATA, fileidentity: *const ::core::ffi::c_void, fileidentitylength: u32, dehydraterangearray: *const CF_FILE_RANGE, dehydraterangecount: u32, updateflags: CF_UPDATE_FLAGS, updateusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfUpdatePlaceholder(filehandle: super::super::Foundation::HANDLE, fsmetadata: *const CF_FS_METADATA, fileidentity: *const ::core::ffi::c_void, fileidentitylength: u32, dehydraterangearray: *const CF_FILE_RANGE, dehydraterangecount: u32, updateflags: CF_UPDATE_FLAGS, updateusn: *mut i64, overlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT;
        }
        CfUpdatePlaceholder(filehandle.into_param().abi(), ::core::mem::transmute(fsmetadata), ::core::mem::transmute(fileidentity), ::core::mem::transmute(fileidentitylength), ::core::mem::transmute(dehydraterangearray), ::core::mem::transmute(dehydraterangecount), ::core::mem::transmute(updateflags), ::core::mem::transmute(updateusn), ::core::mem::transmute(overlapped)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_CloudFilters'*"]
#[inline]
pub unsafe fn CfUpdateSyncProviderStatus<'a, Param0: ::windows::core::IntoParam<'a, CF_CONNECTION_KEY>>(connectionkey: Param0, providerstatus: CF_SYNC_PROVIDER_STATUS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CfUpdateSyncProviderStatus(connectionkey: CF_CONNECTION_KEY, providerstatus: CF_SYNC_PROVIDER_STATUS) -> ::windows::core::HRESULT;
        }
        CfUpdateSyncProviderStatus(connectionkey.into_param().abi(), ::core::mem::transmute(providerstatus)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
