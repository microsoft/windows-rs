#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn DevCloseObjectQuery(hdevquery: *const HDEVQUERY__);
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQuery(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQueryEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQueryFromId(objecttype: DEV_OBJECT_TYPE, pszobjectid: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQueryFromIdEx(
        objecttype: DEV_OBJECT_TYPE,
        pszobjectid: super::super::Foundation::PWSTR,
        queryflags: u32,
        crequestedproperties: u32,
        prequestedproperties: *const super::Properties::DEVPROPCOMPKEY,
        cfilterexpressioncount: u32,
        pfilter: *const DEVPROP_FILTER_EXPRESSION,
        cextendedparametercount: u32,
        pextendedparameters: *const DEV_QUERY_PARAMETER,
        pcallback: PDEV_QUERY_RESULT_CALLBACK,
        pcontext: *const ::core::ffi::c_void,
        phdevquery: *mut *mut HDEVQUERY__,
    ) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQueryFromIds(objecttype: DEV_OBJECT_TYPE, pszzobjectids: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQueryFromIdsEx(
        objecttype: DEV_OBJECT_TYPE,
        pszzobjectids: super::super::Foundation::PWSTR,
        queryflags: u32,
        crequestedproperties: u32,
        prequestedproperties: *const super::Properties::DEVPROPCOMPKEY,
        cfilterexpressioncount: u32,
        pfilter: *const DEVPROP_FILTER_EXPRESSION,
        cextendedparametercount: u32,
        pextendedparameters: *const DEV_QUERY_PARAMETER,
        pcallback: PDEV_QUERY_RESULT_CALLBACK,
        pcontext: *const ::core::ffi::c_void,
        phdevquery: *mut *mut HDEVQUERY__,
    ) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevFindProperty(pkey: *const super::Properties::DEVPROPKEY, store: super::Properties::DEVPROPSTORE, pszlocalename: super::super::Foundation::PWSTR, cproperties: u32, pproperties: *const super::Properties::DEVPROPERTY) -> *mut super::Properties::DEVPROPERTY;
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevFreeObjectProperties(cpropertycount: u32, pproperties: *const super::Properties::DEVPROPERTY);
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevFreeObjects(cobjectcount: u32, pobjects: *const DEV_OBJECT);
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevGetObjectProperties(objecttype: DEV_OBJECT_TYPE, pszobjectid: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevGetObjectPropertiesEx(objecttype: DEV_OBJECT_TYPE, pszobjectid: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevGetObjects(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevGetObjectsEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub struct DEVPROP_FILTER_EXPRESSION {
    pub Operator: DEVPROP_OPERATOR,
    pub Property: super::Properties::DEVPROPERTY,
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::marker::Copy for DEVPROP_FILTER_EXPRESSION {}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::clone::Clone for DEVPROP_FILTER_EXPRESSION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DEVPROP_OPERATOR_MODIFIER_NOT: u32 = 65536u32;
pub const DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE: u32 = 131072u32;
pub const DEVPROP_OPERATOR_NONE: u32 = 0u32;
pub const DEVPROP_OPERATOR_EXISTS: u32 = 1u32;
pub const DEVPROP_OPERATOR_NOT_EXISTS: u32 = 65537u32;
pub const DEVPROP_OPERATOR_EQUALS: u32 = 2u32;
pub const DEVPROP_OPERATOR_NOT_EQUALS: u32 = 65538u32;
pub const DEVPROP_OPERATOR_GREATER_THAN: u32 = 3u32;
pub const DEVPROP_OPERATOR_LESS_THAN: u32 = 4u32;
pub const DEVPROP_OPERATOR_GREATER_THAN_EQUALS: u32 = 5u32;
pub const DEVPROP_OPERATOR_LESS_THAN_EQUALS: u32 = 6u32;
pub const DEVPROP_OPERATOR_EQUALS_IGNORE_CASE: u32 = 131074u32;
pub const DEVPROP_OPERATOR_NOT_EQUALS_IGNORE_CASE: u32 = 196610u32;
pub const DEVPROP_OPERATOR_BITWISE_AND: u32 = 7u32;
pub const DEVPROP_OPERATOR_BITWISE_OR: u32 = 8u32;
pub const DEVPROP_OPERATOR_BEGINS_WITH: u32 = 9u32;
pub const DEVPROP_OPERATOR_ENDS_WITH: u32 = 10u32;
pub const DEVPROP_OPERATOR_CONTAINS: u32 = 11u32;
pub const DEVPROP_OPERATOR_BEGINS_WITH_IGNORE_CASE: u32 = 131081u32;
pub const DEVPROP_OPERATOR_ENDS_WITH_IGNORE_CASE: u32 = 131082u32;
pub const DEVPROP_OPERATOR_CONTAINS_IGNORE_CASE: u32 = 131083u32;
pub const DEVPROP_OPERATOR_LIST_CONTAINS: u32 = 4096u32;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH: u32 = 8192u32;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH: u32 = 12288u32;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS: u32 = 16384u32;
pub const DEVPROP_OPERATOR_LIST_CONTAINS_IGNORE_CASE: u32 = 135168u32;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH_IGNORE_CASE: u32 = 139264u32;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH_IGNORE_CASE: u32 = 143360u32;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS_IGNORE_CASE: u32 = 147456u32;
pub const DEVPROP_OPERATOR_AND_OPEN: u32 = 1048576u32;
pub const DEVPROP_OPERATOR_AND_CLOSE: u32 = 2097152u32;
pub const DEVPROP_OPERATOR_OR_OPEN: u32 = 3145728u32;
pub const DEVPROP_OPERATOR_OR_CLOSE: u32 = 4194304u32;
pub const DEVPROP_OPERATOR_NOT_OPEN: u32 = 5242880u32;
pub const DEVPROP_OPERATOR_NOT_CLOSE: u32 = 6291456u32;
pub const DEVPROP_OPERATOR_ARRAY_CONTAINS: u32 = 268435456u32;
pub const DEVPROP_OPERATOR_MASK_EVAL: u32 = 4095u32;
pub const DEVPROP_OPERATOR_MASK_LIST: u32 = 61440u32;
pub const DEVPROP_OPERATOR_MASK_MODIFIER: u32 = 983040u32;
pub const DEVPROP_OPERATOR_MASK_NOT_LOGICAL: u32 = 4027580415u32;
pub const DEVPROP_OPERATOR_MASK_LOGICAL: u32 = 267386880u32;
pub const DEVPROP_OPERATOR_MASK_ARRAY: u32 = 4026531840u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub struct DEV_OBJECT {
    pub ObjectType: DEV_OBJECT_TYPE,
    pub pszObjectId: super::super::Foundation::PWSTR,
    pub cPropertyCount: u32,
    pub pProperties: *mut super::Properties::DEVPROPERTY,
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::marker::Copy for DEV_OBJECT {}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::clone::Clone for DEV_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DevObjectTypeUnknown: i32 = 0i32;
pub const DevObjectTypeDeviceInterface: i32 = 1i32;
pub const DevObjectTypeDeviceContainer: i32 = 2i32;
pub const DevObjectTypeDevice: i32 = 3i32;
pub const DevObjectTypeDeviceInterfaceClass: i32 = 4i32;
pub const DevObjectTypeAEP: i32 = 5i32;
pub const DevObjectTypeAEPContainer: i32 = 6i32;
pub const DevObjectTypeDeviceInstallerClass: i32 = 7i32;
pub const DevObjectTypeDeviceInterfaceDisplay: i32 = 8i32;
pub const DevObjectTypeDeviceContainerDisplay: i32 = 9i32;
pub const DevObjectTypeAEPService: i32 = 10i32;
pub const DevObjectTypeDevicePanel: i32 = 11i32;
pub const DevQueryFlagNone: i32 = 0i32;
pub const DevQueryFlagUpdateResults: i32 = 1i32;
pub const DevQueryFlagAllProperties: i32 = 2i32;
pub const DevQueryFlagLocalize: i32 = 4i32;
pub const DevQueryFlagAsyncClose: i32 = 8i32;
#[repr(C)]
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
pub const DevQueryResultStateChange: i32 = 0i32;
pub const DevQueryResultAdd: i32 = 1i32;
pub const DevQueryResultUpdate: i32 = 2i32;
pub const DevQueryResultRemove: i32 = 3i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub struct DEV_QUERY_RESULT_ACTION_DATA {
    pub Action: DEV_QUERY_RESULT_ACTION,
    pub Data: DEV_QUERY_RESULT_ACTION_DATA_0,
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::marker::Copy for DEV_QUERY_RESULT_ACTION_DATA {}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::clone::Clone for DEV_QUERY_RESULT_ACTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub union DEV_QUERY_RESULT_ACTION_DATA_0 {
    pub State: DEV_QUERY_STATE,
    pub DeviceObject: DEV_OBJECT,
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::marker::Copy for DEV_QUERY_RESULT_ACTION_DATA_0 {}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::clone::Clone for DEV_QUERY_RESULT_ACTION_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DevQueryStateInitialized: i32 = 0i32;
pub const DevQueryStateEnumCompleted: i32 = 1i32;
pub const DevQueryStateAborted: i32 = 2i32;
pub const DevQueryStateClosed: i32 = 3i32;
#[repr(C)]
pub struct HDEVQUERY__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDEVQUERY__ {}
impl ::core::clone::Clone for HDEVQUERY__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type PDEV_QUERY_RESULT_CALLBACK = unsafe extern "system" fn(hdevquery: *const HDEVQUERY__, pcontext: *const ::core::ffi::c_void, pactiondata: *const DEV_QUERY_RESULT_ACTION_DATA);
