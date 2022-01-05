#[cfg(feature = "implement_exclusive")]
pub trait IFindAllAccountsResultImpl: Sized {
    fn Accounts(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>;
    fn Status(&self) -> ::windows::core::Result<FindAllWebAccountsStatus>;
    fn ProviderError(&self) -> ::windows::core::Result<WebProviderError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountEventArgsImpl: Sized {
    fn Account(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountMonitorImpl: Sized {
    fn Updated(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DefaultSignInAccountChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDefaultSignInAccountChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountMonitor2Impl: Sized {
    fn AccountPictureUpdated(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountPictureUpdated(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationCoreManagerStaticsImpl: Sized {
    fn GetTokenSilentlyAsync(&self, request: &::core::option::Option<WebTokenRequest>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>;
    fn GetTokenSilentlyWithWebAccountAsync(&self, request: &::core::option::Option<WebTokenRequest>, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>;
    fn RequestTokenAsync(&self, request: &::core::option::Option<WebTokenRequest>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>;
    fn RequestTokenWithWebAccountAsync(&self, request: &::core::option::Option<WebTokenRequest>, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>;
    fn FindAccountAsync(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn FindAccountProviderAsync(&self, webaccountproviderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
    fn FindAccountProviderWithAuthorityAsync(&self, webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationCoreManagerStatics2Impl: Sized + IWebAuthenticationCoreManagerStaticsImpl {
    fn FindAccountProviderWithAuthorityForUserAsync(&self, webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING, user: &::core::option::Option<super::super::super::super::System::User>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationCoreManagerStatics3Impl: Sized + IWebAuthenticationCoreManagerStaticsImpl {
    fn CreateWebAccountMonitor(&self, webaccounts: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::super::super::Credentials::WebAccount>>) -> ::windows::core::Result<WebAccountMonitor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAuthenticationCoreManagerStatics4Impl: Sized + IWebAuthenticationCoreManagerStaticsImpl {
    fn FindAllAccountsAsync(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>;
    fn FindAllAccountsWithClientIdAsync(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, clientid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>;
    fn FindSystemAccountProviderAsync(&self, webaccountproviderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
    fn FindSystemAccountProviderWithAuthorityAsync(&self, webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
    fn FindSystemAccountProviderWithAuthorityForUserAsync(&self, webaccountproviderid: &::windows::core::HSTRING, authority: &::windows::core::HSTRING, user: &::core::option::Option<super::super::super::super::System::User>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderErrorImpl: Sized {
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
    fn ErrorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderErrorFactoryImpl: Sized {
    fn Create(&self, errorcode: u32, errormessage: &::windows::core::HSTRING) -> ::windows::core::Result<WebProviderError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenRequestImpl: Sized {
    fn WebAccountProvider(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccountProvider>;
    fn Scope(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ClientId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PromptType(&self) -> ::windows::core::Result<WebTokenRequestPromptType>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenRequest2Impl: Sized {
    fn AppProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenRequest3Impl: Sized {
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCorrelationId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenRequestFactoryImpl: Sized {
    fn Create(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, scope: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING) -> ::windows::core::Result<WebTokenRequest>;
    fn CreateWithPromptType(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, scope: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, prompttype: WebTokenRequestPromptType) -> ::windows::core::Result<WebTokenRequest>;
    fn CreateWithProvider(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>) -> ::windows::core::Result<WebTokenRequest>;
    fn CreateWithScope(&self, provider: &::core::option::Option<super::super::super::Credentials::WebAccountProvider>, scope: &::windows::core::HSTRING) -> ::windows::core::Result<WebTokenRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenRequestResultImpl: Sized {
    fn ResponseData(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<WebTokenResponse>>;
    fn ResponseStatus(&self) -> ::windows::core::Result<WebTokenRequestStatus>;
    fn ResponseError(&self) -> ::windows::core::Result<WebProviderError>;
    fn InvalidateCacheAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenResponseImpl: Sized {
    fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderError(&self) -> ::windows::core::Result<WebProviderError>;
    fn WebAccount(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebTokenResponseFactoryImpl: Sized {
    fn CreateWithToken(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<WebTokenResponse>;
    fn CreateWithTokenAndAccount(&self, token: &::windows::core::HSTRING, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<WebTokenResponse>;
    fn CreateWithTokenAccountAndError(&self, token: &::windows::core::HSTRING, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, error: &::core::option::Option<WebProviderError>) -> ::windows::core::Result<WebTokenResponse>;
}
