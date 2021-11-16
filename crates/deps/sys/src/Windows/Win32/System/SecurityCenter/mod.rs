#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::marker::Copy for IWSCDefaultProduct {}
impl ::core::clone::Clone for IWSCDefaultProduct {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWSCProductList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWSCProductList {}
impl ::core::clone::Clone for IWSCProductList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWscProduct(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWscProduct {}
impl ::core::clone::Clone for IWscProduct {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWscProduct2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWscProduct2 {}
impl ::core::clone::Clone for IWscProduct2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWscProduct3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWscProduct3 {}
impl ::core::clone::Clone for IWscProduct3 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SECURITY_PRODUCT_TYPE_ANTIVIRUS: i32 = 0i32;
pub const SECURITY_PRODUCT_TYPE_FIREWALL: i32 = 1i32;
pub const SECURITY_PRODUCT_TYPE_ANTISPYWARE: i32 = 2i32;
pub const WSCDefaultProduct: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 696361838, data2: 61997, data3: 4581, data4: [156, 233, 94, 85, 23, 80, 124, 102] };
pub const WSCProductList: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 386346875, data2: 39614, data3: 19060, data4: [162, 97, 30, 183, 107, 85, 16, 122] };
pub const WSC_SECURITY_PRODUCT_STATE_ON: i32 = 0i32;
pub const WSC_SECURITY_PRODUCT_STATE_OFF: i32 = 1i32;
pub const WSC_SECURITY_PRODUCT_STATE_SNOOZED: i32 = 2i32;
pub const WSC_SECURITY_PRODUCT_STATE_EXPIRED: i32 = 3i32;
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NOT_SET: i32 = 0i32;
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NO_ACTION: i32 = 1i32;
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_RECOMMENDED: i32 = 2i32;
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_NEEDED: i32 = 3i32;
pub const WSC_SECURITY_PROVIDER_FIREWALL: i32 = 1i32;
pub const WSC_SECURITY_PROVIDER_AUTOUPDATE_SETTINGS: i32 = 2i32;
pub const WSC_SECURITY_PROVIDER_ANTIVIRUS: i32 = 4i32;
pub const WSC_SECURITY_PROVIDER_ANTISPYWARE: i32 = 8i32;
pub const WSC_SECURITY_PROVIDER_INTERNET_SETTINGS: i32 = 16i32;
pub const WSC_SECURITY_PROVIDER_USER_ACCOUNT_CONTROL: i32 = 32i32;
pub const WSC_SECURITY_PROVIDER_SERVICE: i32 = 64i32;
pub const WSC_SECURITY_PROVIDER_NONE: i32 = 0i32;
pub const WSC_SECURITY_PROVIDER_ALL: i32 = 127i32;
pub const WSC_SECURITY_PROVIDER_HEALTH_GOOD: i32 = 0i32;
pub const WSC_SECURITY_PROVIDER_HEALTH_NOTMONITORED: i32 = 1i32;
pub const WSC_SECURITY_PROVIDER_HEALTH_POOR: i32 = 2i32;
pub const WSC_SECURITY_PROVIDER_HEALTH_SNOOZE: i32 = 3i32;
pub const WSC_SECURITY_PRODUCT_OUT_OF_DATE: i32 = 0i32;
pub const WSC_SECURITY_PRODUCT_UP_TO_DATE: i32 = 1i32;
