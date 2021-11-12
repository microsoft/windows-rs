#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn WscGetAntiMalwareUri(ppszuri: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn WscGetSecurityProviderHealth(providers: u32, phealth: *mut WSC_SECURITY_PROVIDER_HEALTH) -> ::windows_sys::core::HRESULT;
    pub fn WscQueryAntiMalwareUri() -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn WscRegisterForChanges(reserved: *mut ::core::ffi::c_void, phcallbackregistration: *mut super::super::Foundation::HANDLE, lpcallbackaddress: super::Threading::LPTHREAD_START_ROUTINE, pcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WscRegisterForUserNotifications() -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WscUnRegisterChanges(hregistrationhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct IWSCDefaultProduct(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSCProductList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWscProduct(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWscProduct2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWscProduct3(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SECURITY_PRODUCT_TYPE(i32);
#[repr(C)]
pub struct WSCDefaultProduct(i32);
#[repr(C)]
pub struct WSCProductList(i32);
#[repr(C)]
pub struct WSC_SECURITY_PRODUCT_STATE(i32);
#[repr(C)]
pub struct WSC_SECURITY_PRODUCT_SUBSTATUS(i32);
#[repr(C)]
pub struct WSC_SECURITY_PROVIDER(i32);
#[repr(C)]
pub struct WSC_SECURITY_PROVIDER_HEALTH(i32);
#[repr(C)]
pub struct WSC_SECURITY_SIGNATURE_STATUS(i32);
