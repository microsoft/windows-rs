#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilterImpl: Sized {
    fn AllowAutoRedirect(&self) -> ::windows::core::Result<bool>;
    fn SetAllowAutoRedirect(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowUI(&self) -> ::windows::core::Result<bool>;
    fn SetAllowUI(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutomaticDecompression(&self) -> ::windows::core::Result<bool>;
    fn SetAutomaticDecompression(&self, value: bool) -> ::windows::core::Result<()>;
    fn CacheControl(&self) -> ::windows::core::Result<HttpCacheControl>;
    fn CookieManager(&self) -> ::windows::core::Result<super::HttpCookieManager>;
    fn ClientCertificate(&self) -> ::windows::core::Result<super::super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetClientCertificate(&self, value: &::core::option::Option<super::super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
    fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn MaxConnectionsPerServer(&self) -> ::windows::core::Result<u32>;
    fn SetMaxConnectionsPerServer(&self, value: u32) -> ::windows::core::Result<()>;
    fn ProxyCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&self, value: &::core::option::Option<super::super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn ServerCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&self, value: &::core::option::Option<super::super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn UseProxy(&self) -> ::windows::core::Result<bool>;
    fn SetUseProxy(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilter2Impl: Sized {
    fn MaxVersion(&self) -> ::windows::core::Result<super::HttpVersion>;
    fn SetMaxVersion(&self, value: super::HttpVersion) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilter3Impl: Sized {
    fn CookieUsageBehavior(&self) -> ::windows::core::Result<HttpCookieUsageBehavior>;
    fn SetCookieUsageBehavior(&self, value: HttpCookieUsageBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilter4Impl: Sized {
    fn ServerCustomValidationRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerCustomValidationRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ClearAuthenticationCache(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilter5Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpBaseProtocolFilterStaticsImpl: Sized {
    fn CreateForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<HttpBaseProtocolFilter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCacheControlImpl: Sized {
    fn ReadBehavior(&self) -> ::windows::core::Result<HttpCacheReadBehavior>;
    fn SetReadBehavior(&self, value: HttpCacheReadBehavior) -> ::windows::core::Result<()>;
    fn WriteBehavior(&self) -> ::windows::core::Result<HttpCacheWriteBehavior>;
    fn SetWriteBehavior(&self, value: HttpCacheWriteBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
pub trait IHttpFilterImpl: Sized + IClosableImpl {
    fn SendRequestAsync(&self, request: &::core::option::Option<super::HttpRequestMessage>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpServerCustomValidationRequestedEventArgsImpl: Sized {
    fn RequestMessage(&self) -> ::windows::core::Result<super::HttpRequestMessage>;
    fn ServerCertificate(&self) -> ::windows::core::Result<super::super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>>;
    fn Reject(&self) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
