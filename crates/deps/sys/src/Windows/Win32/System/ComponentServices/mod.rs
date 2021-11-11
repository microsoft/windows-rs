#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn CoCreateActivity(piunknown: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn CoEnterServiceDomain(pconfigobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CoGetDefaultContext(apttype: super::Com::APTTYPE, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn CoLeaveServiceDomain(punkstatus: ::windows::runtime::RawPtr);
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn GetDispenserManager(param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn GetManagedExtensions(dwexts: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn MTSCreateActivity(riid: *const ::windows::runtime::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn RecycleSurrogate(lreasoncode: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn SafeRef(rid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr) -> *mut ::core::ffi::c_void;
}
