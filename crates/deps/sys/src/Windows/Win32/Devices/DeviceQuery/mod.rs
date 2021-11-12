#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`*"]
    pub fn DevCloseObjectQuery(hdevquery: *const HDEVQUERY__);
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQuery(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQueryEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQueryFromId(objecttype: DEV_OBJECT_TYPE, pszobjectid: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
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
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevCreateObjectQueryFromIds(objecttype: DEV_OBJECT_TYPE, pszzobjectids: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
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
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevFindProperty(pkey: *const super::Properties::DEVPROPKEY, store: super::Properties::DEVPROPSTORE, pszlocalename: super::super::Foundation::PWSTR, cproperties: u32, pproperties: *const super::Properties::DEVPROPERTY) -> *mut super::Properties::DEVPROPERTY;
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevFreeObjectProperties(cpropertycount: u32, pproperties: *const super::Properties::DEVPROPERTY);
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevFreeObjects(cobjectcount: u32, pobjects: *const DEV_OBJECT);
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevGetObjectProperties(objecttype: DEV_OBJECT_TYPE, pszobjectid: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevGetObjectPropertiesEx(objecttype: DEV_OBJECT_TYPE, pszobjectid: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevGetObjects(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_DeviceQuery`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn DevGetObjectsEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows_sys::core::HRESULT;
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub struct DEVPROP_FILTER_EXPRESSION(i32);
pub struct DEVPROP_OPERATOR(i32);
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub struct DEV_OBJECT(i32);
pub struct DEV_OBJECT_TYPE(i32);
pub struct DEV_QUERY_FLAGS(i32);
#[cfg(feature = "Win32_Devices_Properties")]
pub struct DEV_QUERY_PARAMETER(i32);
pub struct DEV_QUERY_RESULT_ACTION(i32);
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub struct DEV_QUERY_RESULT_ACTION_DATA(i32);
pub struct DEV_QUERY_STATE(i32);
pub struct HDEVQUERY__(i32);
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub struct PDEV_QUERY_RESULT_CALLBACK(i32);
