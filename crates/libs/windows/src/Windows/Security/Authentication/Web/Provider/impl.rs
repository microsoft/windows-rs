#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountClientViewImpl: Sized {
    fn ApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn Type(&self) -> ::windows::core::Result<WebAccountClientViewType>;
    fn AccountPairwiseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountClientViewFactoryImpl: Sized {
    fn Create(&self, viewtype: WebAccountClientViewType, applicationcallbackuri: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<WebAccountClientView>;
    fn CreateWithPairwiseId(&self, viewtype: WebAccountClientViewType, applicationcallbackuri: &::core::option::Option<super::super::super::super::Foundation::Uri>, accountpairwiseid: &::windows::core::HSTRING) -> ::windows::core::Result<WebAccountClientView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountManagerStaticsImpl: Sized {
    fn UpdateWebAccountPropertiesAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, webaccountusername: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn AddWebAccountAsync(&self, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn DeleteWebAccountAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn FindAllProviderWebAccountsAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>>;
    fn PushCookiesAsync(&self, uri: &::core::option::Option<super::super::super::super::Foundation::Uri>, cookies: &::core::option::Option<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Web::Http::HttpCookie>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn SetViewAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, view: &::core::option::Option<WebAccountClientView>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn ClearViewAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, applicationcallbackuri: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn GetViewsAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<WebAccountClientView>>>;
    fn SetWebAccountPictureAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, webaccountpicture: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn ClearWebAccountPictureAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountManagerStatics2Impl: Sized {
    fn PullCookiesAsync(&self, uristring: &::windows::core::HSTRING, callerpfn: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountManagerStatics3Impl: Sized {
    fn FindAllProviderWebAccountsForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>>;
    fn AddWebAccountForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn AddWebAccountWithScopeForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn AddWebAccountWithScopeAndMapForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, scope: WebAccountScope, peruserwebaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountManagerStatics4Impl: Sized {
    fn InvalidateAppCacheForAllAccountsAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn InvalidateAppCacheForAccountAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountMapManagerStaticsImpl: Sized {
    fn AddWebAccountWithScopeAndMapAsync(&self, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, scope: WebAccountScope, peruserwebaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn SetPerAppToPerUserAccountAsync(&self, perappaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, peruserwebaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn GetPerUserFromPerAppAccountAsync(&self, perappaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn ClearPerUserFromPerAppAccountAsync(&self, perappaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderAddAccountOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
}
pub trait IWebAccountProviderBaseReportOperationImpl: Sized {
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
    fn ReportError(&self, value: &::core::option::Option<super::Core::WebProviderError>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderDeleteAccountOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn WebAccount(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderManageAccountOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn WebAccount(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
}
pub trait IWebAccountProviderOperationImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<WebAccountProviderOperationKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderRetrieveCookiesOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn Context(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn Cookies(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::super::super::Web::Http::HttpCookie>>;
    fn SetUri(&self, uri: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Uri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn ApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderSignOutAccountOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn WebAccount(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount>;
    fn ApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn ClientId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IWebAccountProviderSilentReportOperationImpl: Sized + IWebAccountProviderBaseReportOperationImpl {
    fn ReportUserInteractionRequired(&self) -> ::windows::core::Result<()>;
    fn ReportUserInteractionRequiredWithError(&self, value: &::core::option::Option<super::Core::WebProviderError>) -> ::windows::core::Result<()>;
}
pub trait IWebAccountProviderTokenObjectsImpl: Sized {
    fn Operation(&self) -> ::windows::core::Result<IWebAccountProviderOperation>;
}
pub trait IWebAccountProviderTokenObjects2Impl: Sized + IWebAccountProviderTokenObjectsImpl {
    fn User(&self) -> ::windows::core::Result<super::super::super::super::System::User>;
}
pub trait IWebAccountProviderTokenOperationImpl: Sized + IWebAccountProviderOperationImpl {
    fn ProviderRequest(&self) -> ::windows::core::Result<WebProviderTokenRequest>;
    fn ProviderResponses(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>>;
    fn SetCacheExpirationTime(&self, value: &super::super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn CacheExpirationTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime>;
}
pub trait IWebAccountProviderUIReportOperationImpl: Sized + IWebAccountProviderBaseReportOperationImpl {
    fn ReportUserCanceled(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountScopeManagerStaticsImpl: Sized {
    fn AddWebAccountWithScopeAsync(&self, webaccountid: &::windows::core::HSTRING, webaccountusername: &::windows::core::HSTRING, props: &::core::option::Option<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>;
    fn SetScopeAsync(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn GetScope(&self, webaccount: &::core::option::Option<super::super::super::Credentials::WebAccount>) -> ::windows::core::Result<WebAccountScope>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderTokenRequestImpl: Sized {
    fn ClientRequest(&self) -> ::windows::core::Result<super::Core::WebTokenRequest>;
    fn WebAccounts(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>;
    fn WebAccountSelectionOptions(&self) -> ::windows::core::Result<WebAccountSelectionOptions>;
    fn ApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn GetApplicationTokenBindingKeyAsync(&self, keytype: super::TokenBindingKeyType, target: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Cryptography::Core::CryptographicKey>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderTokenRequest2Impl: Sized {
    fn GetApplicationTokenBindingKeyIdAsync(&self, keytype: super::TokenBindingKeyType, target: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderTokenRequest3Impl: Sized {
    fn ApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApplicationProcessName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CheckApplicationForCapabilityAsync(&self, capabilityname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderTokenResponseImpl: Sized {
    fn ClientResponse(&self) -> ::windows::core::Result<super::Core::WebTokenResponse>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebProviderTokenResponseFactoryImpl: Sized {
    fn Create(&self, webtokenresponse: &::core::option::Option<super::Core::WebTokenResponse>) -> ::windows::core::Result<WebProviderTokenResponse>;
}
