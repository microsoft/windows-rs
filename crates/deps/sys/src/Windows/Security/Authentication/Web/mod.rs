#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Security_Authentication_Web_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
pub struct IWebAuthenticationBrokerStatics(pub *mut ::core::ffi::c_void);
pub struct IWebAuthenticationBrokerStatics2(pub *mut ::core::ffi::c_void);
pub struct IWebAuthenticationResult(pub *mut ::core::ffi::c_void);
pub struct TokenBindingKeyType(i32);
pub struct WebAuthenticationBroker(i32);
pub struct WebAuthenticationOptions(i32);
pub struct WebAuthenticationResult(i32);
pub struct WebAuthenticationStatus(i32);
