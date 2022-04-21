#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
    pub fn DevCloseObjectQuery(hdevquery: *const HDEVQUERY__);
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn DevCreateObjectQuery(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn DevCreateObjectQueryEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn DevCreateObjectQueryFromId(objecttype: DEV_OBJECT_TYPE, pszobjectid: ::windows_sys::core::PCWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn DevCreateObjectQueryFromIdEx(objecttype: DEV_OBJECT_TYPE, pszobjectid: ::windows_sys::core::PCWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn DevCreateObjectQueryFromIds(objecttype: DEV_OBJECT_TYPE, pszzobjectids: ::windows_sys::core::PCWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn DevCreateObjectQueryFromIdsEx(objecttype: DEV_OBJECT_TYPE, pszzobjectids: ::windows_sys::core::PCWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn DevFindProperty(pkey: *const super::Properties::DEVPROPKEY, store: super::Properties::DEVPROPSTORE, pszlocalename: ::windows_sys::core::PCWSTR, cproperties: u32, pproperties: *const super::Properties::DEVPROPERTY) -> *mut super::Properties::DEVPROPERTY;
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn DevFreeObjectProperties(cpropertycount: u32, pproperties: *const super::Properties::DEVPROPERTY);
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn DevFreeObjects(cobjectcount: u32, pobjects: *const DEV_OBJECT);
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn DevGetObjectProperties(objecttype: DEV_OBJECT_TYPE, pszobjectid: ::windows_sys::core::PCWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn DevGetObjectPropertiesEx(objecttype: DEV_OBJECT_TYPE, pszobjectid: ::windows_sys::core::PCWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn DevGetObjects(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn DevGetObjectsEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub struct DEVPROP_FILTER_EXPRESSION {
    pub Operator: DEVPROP_OPERATOR,
    pub Property: super::Properties::DEVPROPERTY,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::marker::Copy for DEVPROP_FILTER_EXPRESSION {}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::clone::Clone for DEVPROP_FILTER_EXPRESSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub type DEVPROP_OPERATOR = u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_MODIFIER_NOT: DEVPROP_OPERATOR = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE: DEVPROP_OPERATOR = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_NONE: DEVPROP_OPERATOR = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_EXISTS: DEVPROP_OPERATOR = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_NOT_EXISTS: DEVPROP_OPERATOR = 65537u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_EQUALS: DEVPROP_OPERATOR = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_NOT_EQUALS: DEVPROP_OPERATOR = 65538u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_GREATER_THAN: DEVPROP_OPERATOR = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_LESS_THAN: DEVPROP_OPERATOR = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_GREATER_THAN_EQUALS: DEVPROP_OPERATOR = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_LESS_THAN_EQUALS: DEVPROP_OPERATOR = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_EQUALS_IGNORE_CASE: DEVPROP_OPERATOR = 131074u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_NOT_EQUALS_IGNORE_CASE: DEVPROP_OPERATOR = 196610u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_BITWISE_AND: DEVPROP_OPERATOR = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_BITWISE_OR: DEVPROP_OPERATOR = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_BEGINS_WITH: DEVPROP_OPERATOR = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_ENDS_WITH: DEVPROP_OPERATOR = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_CONTAINS: DEVPROP_OPERATOR = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_BEGINS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = 131081u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_ENDS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = 131082u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = 131083u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_LIST_CONTAINS: DEVPROP_OPERATOR = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH: DEVPROP_OPERATOR = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH: DEVPROP_OPERATOR = 12288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS: DEVPROP_OPERATOR = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_LIST_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = 135168u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = 139264u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = 143360u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = 147456u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_AND_OPEN: DEVPROP_OPERATOR = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_AND_CLOSE: DEVPROP_OPERATOR = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_OR_OPEN: DEVPROP_OPERATOR = 3145728u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_OR_CLOSE: DEVPROP_OPERATOR = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_NOT_OPEN: DEVPROP_OPERATOR = 5242880u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_NOT_CLOSE: DEVPROP_OPERATOR = 6291456u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_ARRAY_CONTAINS: DEVPROP_OPERATOR = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_MASK_EVAL: DEVPROP_OPERATOR = 4095u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_MASK_LIST: DEVPROP_OPERATOR = 61440u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_MASK_MODIFIER: DEVPROP_OPERATOR = 983040u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_MASK_NOT_LOGICAL: DEVPROP_OPERATOR = 4027580415u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_MASK_LOGICAL: DEVPROP_OPERATOR = 267386880u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DEVPROP_OPERATOR_MASK_ARRAY: DEVPROP_OPERATOR = 4026531840u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub struct DEV_OBJECT {
    pub ObjectType: DEV_OBJECT_TYPE,
    pub pszObjectId: ::windows_sys::core::PCWSTR,
    pub cPropertyCount: u32,
    pub pProperties: *const super::Properties::DEVPROPERTY,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::marker::Copy for DEV_OBJECT {}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::clone::Clone for DEV_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub type DEV_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevObjectTypeUnknown: DEV_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevObjectTypeDeviceInterface: DEV_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevObjectTypeDeviceContainer: DEV_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevObjectTypeDevice: DEV_OBJECT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevObjectTypeDeviceInterfaceClass: DEV_OBJECT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevObjectTypeAEP: DEV_OBJECT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevObjectTypeAEPContainer: DEV_OBJECT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevObjectTypeDeviceInstallerClass: DEV_OBJECT_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevObjectTypeDeviceInterfaceDisplay: DEV_OBJECT_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevObjectTypeDeviceContainerDisplay: DEV_OBJECT_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevObjectTypeAEPService: DEV_OBJECT_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevObjectTypeDevicePanel: DEV_OBJECT_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub type DEV_QUERY_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevQueryFlagNone: DEV_QUERY_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevQueryFlagUpdateResults: DEV_QUERY_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevQueryFlagAllProperties: DEV_QUERY_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevQueryFlagLocalize: DEV_QUERY_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevQueryFlagAsyncClose: DEV_QUERY_FLAGS = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub struct DEV_QUERY_PARAMETER {
    pub Key: super::Properties::DEVPROPKEY,
    pub Type: u32,
    pub BufferSize: u32,
    pub Buffer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::marker::Copy for DEV_QUERY_PARAMETER {}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::clone::Clone for DEV_QUERY_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub type DEV_QUERY_RESULT_ACTION = i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevQueryResultStateChange: DEV_QUERY_RESULT_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevQueryResultAdd: DEV_QUERY_RESULT_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevQueryResultUpdate: DEV_QUERY_RESULT_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevQueryResultRemove: DEV_QUERY_RESULT_ACTION = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub struct DEV_QUERY_RESULT_ACTION_DATA {
    pub Action: DEV_QUERY_RESULT_ACTION,
    pub Data: DEV_QUERY_RESULT_ACTION_DATA_0,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::marker::Copy for DEV_QUERY_RESULT_ACTION_DATA {}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::clone::Clone for DEV_QUERY_RESULT_ACTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub union DEV_QUERY_RESULT_ACTION_DATA_0 {
    pub State: DEV_QUERY_STATE,
    pub DeviceObject: DEV_OBJECT,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::marker::Copy for DEV_QUERY_RESULT_ACTION_DATA_0 {}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::clone::Clone for DEV_QUERY_RESULT_ACTION_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub type DEV_QUERY_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevQueryStateInitialized: DEV_QUERY_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevQueryStateEnumCompleted: DEV_QUERY_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevQueryStateAborted: DEV_QUERY_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub const DevQueryStateClosed: DEV_QUERY_STATE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`*"]
pub struct HDEVQUERY__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDEVQUERY__ {}
impl ::core::clone::Clone for HDEVQUERY__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceQuery\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub type PDEV_QUERY_RESULT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hdevquery: *const HDEVQUERY__, pcontext: *const ::core::ffi::c_void, pactiondata: *const DEV_QUERY_RESULT_ACTION_DATA)>;
