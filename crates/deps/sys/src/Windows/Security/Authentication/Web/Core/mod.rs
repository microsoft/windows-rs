#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct FindAllAccountsResult(pub *mut ::core::ffi::c_void);
pub struct FindAllWebAccountsStatus(i32);
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
pub struct WebAuthenticationCoreManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebProviderError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebTokenRequest(pub *mut ::core::ffi::c_void);
pub struct WebTokenRequestPromptType(i32);
#[repr(transparent)]
pub struct WebTokenRequestResult(pub *mut ::core::ffi::c_void);
pub struct WebTokenRequestStatus(i32);
#[repr(transparent)]
pub struct WebTokenResponse(pub *mut ::core::ffi::c_void);
