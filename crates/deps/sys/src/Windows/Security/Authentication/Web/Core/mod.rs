#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct FindAllAccountsResult(i32);
pub struct FindAllWebAccountsStatus(i32);
pub struct IFindAllAccountsResult(pub *mut ::core::ffi::c_void);
pub struct IWebAccountEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebAccountMonitor(pub *mut ::core::ffi::c_void);
pub struct IWebAccountMonitor2(pub *mut ::core::ffi::c_void);
pub struct IWebAuthenticationCoreManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IWebAuthenticationCoreManagerStatics2(pub *mut ::core::ffi::c_void);
pub struct IWebAuthenticationCoreManagerStatics3(pub *mut ::core::ffi::c_void);
pub struct IWebAuthenticationCoreManagerStatics4(pub *mut ::core::ffi::c_void);
pub struct IWebProviderError(pub *mut ::core::ffi::c_void);
pub struct IWebProviderErrorFactory(pub *mut ::core::ffi::c_void);
pub struct IWebTokenRequest(pub *mut ::core::ffi::c_void);
pub struct IWebTokenRequest2(pub *mut ::core::ffi::c_void);
pub struct IWebTokenRequest3(pub *mut ::core::ffi::c_void);
pub struct IWebTokenRequestFactory(pub *mut ::core::ffi::c_void);
pub struct IWebTokenRequestResult(pub *mut ::core::ffi::c_void);
pub struct IWebTokenResponse(pub *mut ::core::ffi::c_void);
pub struct IWebTokenResponseFactory(pub *mut ::core::ffi::c_void);
pub struct WebAccountEventArgs(i32);
pub struct WebAccountMonitor(i32);
pub struct WebAuthenticationCoreManager(i32);
pub struct WebProviderError(i32);
pub struct WebTokenRequest(i32);
pub struct WebTokenRequestPromptType(i32);
pub struct WebTokenRequestResult(i32);
pub struct WebTokenRequestStatus(i32);
pub struct WebTokenResponse(i32);
