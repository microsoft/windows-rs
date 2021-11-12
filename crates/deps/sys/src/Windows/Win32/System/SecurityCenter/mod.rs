#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_SecurityCenter`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WscGetAntiMalwareUri(ppszuri: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_SecurityCenter`*"]
    pub fn WscGetSecurityProviderHealth(providers: u32, phealth: *mut WSC_SECURITY_PROVIDER_HEALTH) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_SecurityCenter`*"]
    pub fn WscQueryAntiMalwareUri() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_SecurityCenter`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn WscRegisterForChanges(reserved: *mut ::core::ffi::c_void, phcallbackregistration: *mut super::super::Foundation::HANDLE, lpcallbackaddress: super::Threading::LPTHREAD_START_ROUTINE, pcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_SecurityCenter`*"]
    pub fn WscRegisterForUserNotifications() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_SecurityCenter`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WscUnRegisterChanges(hregistrationhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
}
