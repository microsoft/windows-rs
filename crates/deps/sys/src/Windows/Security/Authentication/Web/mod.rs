#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Security_Authentication_Web_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWebAuthenticationBrokerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAuthenticationBrokerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAuthenticationResult(pub *mut ::core::ffi::c_void);
pub struct TokenBindingKeyType(i32);
#[repr(transparent)]
pub struct WebAuthenticationBroker(pub *mut ::core::ffi::c_void);
pub struct WebAuthenticationOptions(i32);
#[repr(transparent)]
pub struct WebAuthenticationResult(pub *mut ::core::ffi::c_void);
pub struct WebAuthenticationStatus(i32);
