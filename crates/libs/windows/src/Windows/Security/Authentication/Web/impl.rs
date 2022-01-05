#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationBrokerStaticsImpl: Sized {
    fn AuthenticateWithCallbackUriAsync(&self, options: WebAuthenticationOptions, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>, callbackuri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>;
    fn AuthenticateWithoutCallbackUriAsync(&self, options: WebAuthenticationOptions, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>;
    fn GetCurrentApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationBrokerStatics2Impl: Sized {
    fn AuthenticateAndContinue(&self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AuthenticateWithCallbackUriAndContinue(&self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>, callbackuri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue(&self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>, callbackuri: &::core::option::Option<super::super::super::Foundation::Uri>, continuationdata: &::core::option::Option<super::super::super::Foundation::Collections::ValueSet>, options: WebAuthenticationOptions) -> ::windows::core::Result<()>;
    fn AuthenticateSilentlyAsync(&self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>;
    fn AuthenticateSilentlyWithOptionsAsync(&self, requesturi: &::core::option::Option<super::super::super::Foundation::Uri>, options: WebAuthenticationOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationResultImpl: Sized {
    fn ResponseData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ResponseStatus(&self) -> ::windows::core::Result<WebAuthenticationStatus>;
    fn ResponseErrorDetail(&self) -> ::windows::core::Result<u32>;
}
