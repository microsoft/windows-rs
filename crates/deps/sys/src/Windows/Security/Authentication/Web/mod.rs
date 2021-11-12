#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct TokenBindingKeyType(pub i32);
impl TokenBindingKeyType {
    pub const Rsa2048: TokenBindingKeyType = TokenBindingKeyType(0i32);
    pub const EcdsaP256: TokenBindingKeyType = TokenBindingKeyType(1i32);
    pub const AnyExisting: TokenBindingKeyType = TokenBindingKeyType(2i32);
}
#[repr(transparent)]
pub struct WebAuthenticationOptions(pub u32);
impl WebAuthenticationOptions {
    pub const None: WebAuthenticationOptions = WebAuthenticationOptions(0u32);
    pub const SilentMode: WebAuthenticationOptions = WebAuthenticationOptions(1u32);
    pub const UseTitle: WebAuthenticationOptions = WebAuthenticationOptions(2u32);
    pub const UseHttpPost: WebAuthenticationOptions = WebAuthenticationOptions(4u32);
    pub const UseCorporateNetwork: WebAuthenticationOptions = WebAuthenticationOptions(8u32);
}
#[repr(transparent)]
pub struct WebAuthenticationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAuthenticationStatus(pub i32);
impl WebAuthenticationStatus {
    pub const Success: WebAuthenticationStatus = WebAuthenticationStatus(0i32);
    pub const UserCancel: WebAuthenticationStatus = WebAuthenticationStatus(1i32);
    pub const ErrorHttp: WebAuthenticationStatus = WebAuthenticationStatus(2i32);
}
