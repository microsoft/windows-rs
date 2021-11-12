#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_ACTIVATION_LIMIT: u32 = 4294967294u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_CALL_LIMIT: u32 = 4294967293u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_LIFETIME_LIMIT: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_MEMORY_LIMIT: u32 = 4294967292u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_NO_REASON_SUPPLIED: u32 = 0u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_RECYCLED_FROM_UI: u32 = 4294967291u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const DATA_NOT_AVAILABLE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const GUID_STRING_SIZE: u32 = 40u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn CoCreateActivity(piunknown: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn CoEnterServiceDomain(pconfigobject: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CoGetDefaultContext(apttype: super::Com::APTTYPE, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn CoLeaveServiceDomain(punkstatus: ::windows_sys::core::IUnknown);
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn GetDispenserManager(param0: *mut IDispenserManager) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn GetManagedExtensions(dwexts: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn MTSCreateActivity(riid: *const ::windows_sys::core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn RecycleSurrogate(lreasoncode: i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn SafeRef(rid: *const ::windows_sys::core::GUID, punk: ::windows_sys::core::IUnknown) -> *mut ::core::ffi::c_void;
}
