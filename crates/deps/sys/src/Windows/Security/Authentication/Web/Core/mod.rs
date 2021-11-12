#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct FindAllAccountsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FindAllWebAccountsStatus(pub i32);
impl FindAllWebAccountsStatus {
    pub const Success: FindAllWebAccountsStatus = FindAllWebAccountsStatus(0i32);
    pub const NotAllowedByProvider: FindAllWebAccountsStatus = FindAllWebAccountsStatus(1i32);
    pub const NotSupportedByProvider: FindAllWebAccountsStatus = FindAllWebAccountsStatus(2i32);
    pub const ProviderError: FindAllWebAccountsStatus = FindAllWebAccountsStatus(3i32);
}
#[repr(transparent)]
pub struct IFindAllAccountsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountMonitor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebProviderError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebProviderErrorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebTokenRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebTokenRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebTokenRequest3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebTokenRequestFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebTokenRequestResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebTokenResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebTokenResponseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebProviderError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebTokenRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebTokenRequestPromptType(pub i32);
impl WebTokenRequestPromptType {
    pub const Default: WebTokenRequestPromptType = WebTokenRequestPromptType(0i32);
    pub const ForceAuthentication: WebTokenRequestPromptType = WebTokenRequestPromptType(1i32);
}
#[repr(transparent)]
pub struct WebTokenRequestResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebTokenRequestStatus(pub i32);
impl WebTokenRequestStatus {
    pub const Success: WebTokenRequestStatus = WebTokenRequestStatus(0i32);
    pub const UserCancel: WebTokenRequestStatus = WebTokenRequestStatus(1i32);
    pub const AccountSwitch: WebTokenRequestStatus = WebTokenRequestStatus(2i32);
    pub const UserInteractionRequired: WebTokenRequestStatus = WebTokenRequestStatus(3i32);
    pub const AccountProviderNotAvailable: WebTokenRequestStatus = WebTokenRequestStatus(4i32);
    pub const ProviderError: WebTokenRequestStatus = WebTokenRequestStatus(5i32);
}
#[repr(transparent)]
pub struct WebTokenResponse(pub *mut ::core::ffi::c_void);
