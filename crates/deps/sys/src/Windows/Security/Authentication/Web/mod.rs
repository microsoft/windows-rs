#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Security_Authentication_Web_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
pub struct IWebAuthenticationBrokerStatics(i32);
pub struct IWebAuthenticationBrokerStatics2(i32);
pub struct IWebAuthenticationResult(i32);
pub struct TokenBindingKeyType(i32);
pub struct WebAuthenticationBroker(i32);
pub struct WebAuthenticationOptions(i32);
pub struct WebAuthenticationResult(i32);
pub struct WebAuthenticationStatus(i32);
