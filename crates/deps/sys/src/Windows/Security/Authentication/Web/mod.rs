#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Security_Authentication_Web_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {
    fn IWebAuthenticationBrokerStatics();
    fn IWebAuthenticationBrokerStatics2();
    fn IWebAuthenticationResult();
    fn TokenBindingKeyType();
    fn WebAuthenticationBroker();
    fn WebAuthenticationOptions();
    fn WebAuthenticationResult();
    fn WebAuthenticationStatus();
}
