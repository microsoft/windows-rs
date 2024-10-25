pub const DEVPROP_OPERATOR_AND_CLOSE: DEVPROP_OPERATOR = 2097152u32;
pub const DEVPROP_OPERATOR_AND_OPEN: DEVPROP_OPERATOR = 1048576u32;
pub const DEVPROP_OPERATOR_ARRAY_CONTAINS: DEVPROP_OPERATOR = 268435456u32;
pub const DEVPROP_OPERATOR_BEGINS_WITH: DEVPROP_OPERATOR = 9u32;
pub const DEVPROP_OPERATOR_BEGINS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = 131081u32;
pub const DEVPROP_OPERATOR_BITWISE_AND: DEVPROP_OPERATOR = 7u32;
pub const DEVPROP_OPERATOR_BITWISE_OR: DEVPROP_OPERATOR = 8u32;
pub const DEVPROP_OPERATOR_CONTAINS: DEVPROP_OPERATOR = 11u32;
pub const DEVPROP_OPERATOR_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = 131083u32;
pub const DEVPROP_OPERATOR_ENDS_WITH: DEVPROP_OPERATOR = 10u32;
pub const DEVPROP_OPERATOR_ENDS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = 131082u32;
pub const DEVPROP_OPERATOR_EQUALS: DEVPROP_OPERATOR = 2u32;
pub const DEVPROP_OPERATOR_EQUALS_IGNORE_CASE: DEVPROP_OPERATOR = 131074u32;
pub const DEVPROP_OPERATOR_EXISTS: DEVPROP_OPERATOR = 1u32;
pub const DEVPROP_OPERATOR_GREATER_THAN: DEVPROP_OPERATOR = 3u32;
pub const DEVPROP_OPERATOR_GREATER_THAN_EQUALS: DEVPROP_OPERATOR = 5u32;
pub const DEVPROP_OPERATOR_LESS_THAN: DEVPROP_OPERATOR = 4u32;
pub const DEVPROP_OPERATOR_LESS_THAN_EQUALS: DEVPROP_OPERATOR = 6u32;
pub const DEVPROP_OPERATOR_LIST_CONTAINS: DEVPROP_OPERATOR = 4096u32;
pub const DEVPROP_OPERATOR_LIST_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = 135168u32;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH: DEVPROP_OPERATOR = 8192u32;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = 139264u32;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS: DEVPROP_OPERATOR = 16384u32;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = 147456u32;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH: DEVPROP_OPERATOR = 12288u32;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = 143360u32;
pub const DEVPROP_OPERATOR_MASK_ARRAY: DEVPROP_OPERATOR = 4026531840u32;
pub const DEVPROP_OPERATOR_MASK_EVAL: DEVPROP_OPERATOR = 4095u32;
pub const DEVPROP_OPERATOR_MASK_LIST: DEVPROP_OPERATOR = 61440u32;
pub const DEVPROP_OPERATOR_MASK_LOGICAL: DEVPROP_OPERATOR = 267386880u32;
pub const DEVPROP_OPERATOR_MASK_MODIFIER: DEVPROP_OPERATOR = 983040u32;
pub const DEVPROP_OPERATOR_MASK_NOT_LOGICAL: DEVPROP_OPERATOR = 4027580415u32;
pub const DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE: DEVPROP_OPERATOR = 131072u32;
pub const DEVPROP_OPERATOR_MODIFIER_NOT: DEVPROP_OPERATOR = 65536u32;
pub const DEVPROP_OPERATOR_NONE: DEVPROP_OPERATOR = 0u32;
pub const DEVPROP_OPERATOR_NOT_CLOSE: DEVPROP_OPERATOR = 6291456u32;
pub const DEVPROP_OPERATOR_NOT_EQUALS: DEVPROP_OPERATOR = 65538u32;
pub const DEVPROP_OPERATOR_NOT_EQUALS_IGNORE_CASE: DEVPROP_OPERATOR = 196610u32;
pub const DEVPROP_OPERATOR_NOT_EXISTS: DEVPROP_OPERATOR = 65537u32;
pub const DEVPROP_OPERATOR_NOT_OPEN: DEVPROP_OPERATOR = 5242880u32;
pub const DEVPROP_OPERATOR_OR_CLOSE: DEVPROP_OPERATOR = 4194304u32;
pub const DEVPROP_OPERATOR_OR_OPEN: DEVPROP_OPERATOR = 3145728u32;
pub const DevObjectTypeAEP: DEV_OBJECT_TYPE = 5i32;
pub const DevObjectTypeAEPContainer: DEV_OBJECT_TYPE = 6i32;
pub const DevObjectTypeAEPService: DEV_OBJECT_TYPE = 10i32;
pub const DevObjectTypeDevice: DEV_OBJECT_TYPE = 3i32;
pub const DevObjectTypeDeviceContainer: DEV_OBJECT_TYPE = 2i32;
pub const DevObjectTypeDeviceContainerDisplay: DEV_OBJECT_TYPE = 9i32;
pub const DevObjectTypeDeviceInstallerClass: DEV_OBJECT_TYPE = 7i32;
pub const DevObjectTypeDeviceInterface: DEV_OBJECT_TYPE = 1i32;
pub const DevObjectTypeDeviceInterfaceClass: DEV_OBJECT_TYPE = 4i32;
pub const DevObjectTypeDeviceInterfaceDisplay: DEV_OBJECT_TYPE = 8i32;
pub const DevObjectTypeDevicePanel: DEV_OBJECT_TYPE = 11i32;
pub const DevObjectTypeUnknown: DEV_OBJECT_TYPE = 0i32;
pub const DevQueryFlagAllProperties: DEV_QUERY_FLAGS = 2i32;
pub const DevQueryFlagAsyncClose: DEV_QUERY_FLAGS = 8i32;
pub const DevQueryFlagLocalize: DEV_QUERY_FLAGS = 4i32;
pub const DevQueryFlagNone: DEV_QUERY_FLAGS = 0i32;
pub const DevQueryFlagUpdateResults: DEV_QUERY_FLAGS = 1i32;
pub const DevQueryResultAdd: DEV_QUERY_RESULT_ACTION = 1i32;
pub const DevQueryResultRemove: DEV_QUERY_RESULT_ACTION = 3i32;
pub const DevQueryResultStateChange: DEV_QUERY_RESULT_ACTION = 0i32;
pub const DevQueryResultUpdate: DEV_QUERY_RESULT_ACTION = 2i32;
pub const DevQueryStateAborted: DEV_QUERY_STATE = 2i32;
pub const DevQueryStateClosed: DEV_QUERY_STATE = 3i32;
pub const DevQueryStateEnumCompleted: DEV_QUERY_STATE = 1i32;
pub const DevQueryStateInitialized: DEV_QUERY_STATE = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DEVPROP_OPERATOR(pub u32);
impl windows_core::TypeKind for DEVPROP_OPERATOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DEV_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for DEV_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DEV_QUERY_FLAGS(pub i32);
impl windows_core::TypeKind for DEV_QUERY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DEV_QUERY_RESULT_ACTION(pub i32);
impl windows_core::TypeKind for DEV_QUERY_RESULT_ACTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DEV_QUERY_STATE(pub i32);
impl windows_core::TypeKind for DEV_QUERY_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Devices_Properties")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVPROP_FILTER_EXPRESSION {
    pub Operator: DEVPROP_OPERATOR,
    pub Property: super::Properties::DEVPROPERTY,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl Default for DEVPROP_FILTER_EXPRESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
impl windows_core::TypeKind for DEVPROP_FILTER_EXPRESSION {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Devices_Properties")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEV_OBJECT {
    pub ObjectType: DEV_OBJECT_TYPE,
    pub pszObjectId: windows_core::PCWSTR,
    pub cPropertyCount: u32,
    pub pProperties: *const super::Properties::DEVPROPERTY,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl Default for DEV_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
impl windows_core::TypeKind for DEV_OBJECT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Devices_Properties")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEV_QUERY_PARAMETER {
    pub Key: super::Properties::DEVPROPKEY,
    pub Type: super::Properties::DEVPROPTYPE,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl Default for DEV_QUERY_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
impl windows_core::TypeKind for DEV_QUERY_PARAMETER {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Devices_Properties")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEV_QUERY_RESULT_ACTION_DATA {
    pub Action: DEV_QUERY_RESULT_ACTION,
    pub Data: DEV_QUERY_RESULT_ACTION_DATA_0,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl Default for DEV_QUERY_RESULT_ACTION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
impl windows_core::TypeKind for DEV_QUERY_RESULT_ACTION_DATA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Devices_Properties")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union DEV_QUERY_RESULT_ACTION_DATA_0 {
    pub State: DEV_QUERY_STATE,
    pub DeviceObject: DEV_OBJECT,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl Default for DEV_QUERY_RESULT_ACTION_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
impl windows_core::TypeKind for DEV_QUERY_RESULT_ACTION_DATA_0 {
    type TypeKind = windows_core::CloneType;
}
#[cfg(feature = "Win32_Devices_Properties")]
pub type PDEV_QUERY_RESULT_CALLBACK = Option<unsafe extern "system" fn(hdevquery: HDEVQUERY, pcontext: *const core::ffi::c_void, pactiondata: *const DEV_QUERY_RESULT_ACTION_DATA)>;
