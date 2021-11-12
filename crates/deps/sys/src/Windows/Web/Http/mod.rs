#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Web_Http_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "Web_Http_Filters")]
pub mod Filters;
#[cfg(feature = "Web_Http_Headers")]
pub mod Headers;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HttpBufferContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpBufferContent {}
impl ::core::clone::Clone for HttpBufferContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpClient {}
impl ::core::clone::Clone for HttpClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpCompletionOption(pub i32);
impl HttpCompletionOption {
    pub const ResponseContentRead: Self = Self(0i32);
    pub const ResponseHeadersRead: Self = Self(1i32);
}
impl ::core::marker::Copy for HttpCompletionOption {}
impl ::core::clone::Clone for HttpCompletionOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpCookie(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpCookie {}
impl ::core::clone::Clone for HttpCookie {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpCookieCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpCookieCollection {}
impl ::core::clone::Clone for HttpCookieCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpCookieManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpCookieManager {}
impl ::core::clone::Clone for HttpCookieManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpFormUrlEncodedContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpFormUrlEncodedContent {}
impl ::core::clone::Clone for HttpFormUrlEncodedContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpGetBufferResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpGetBufferResult {}
impl ::core::clone::Clone for HttpGetBufferResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpGetInputStreamResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpGetInputStreamResult {}
impl ::core::clone::Clone for HttpGetInputStreamResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpGetStringResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpGetStringResult {}
impl ::core::clone::Clone for HttpGetStringResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpMethod(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpMethod {}
impl ::core::clone::Clone for HttpMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpMultipartContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpMultipartContent {}
impl ::core::clone::Clone for HttpMultipartContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpMultipartFormDataContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpMultipartFormDataContent {}
impl ::core::clone::Clone for HttpMultipartFormDataContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct HttpProgress {
    pub Stage: HttpProgressStage,
    pub BytesSent: u64,
    pub TotalBytesToSend: super::super::Foundation::IReference<u64>,
    pub BytesReceived: u64,
    pub TotalBytesToReceive: super::super::Foundation::IReference<u64>,
    pub Retries: u32,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for HttpProgress {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for HttpProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpProgressStage(pub i32);
impl HttpProgressStage {
    pub const None: Self = Self(0i32);
    pub const DetectingProxy: Self = Self(10i32);
    pub const ResolvingName: Self = Self(20i32);
    pub const ConnectingToServer: Self = Self(30i32);
    pub const NegotiatingSsl: Self = Self(40i32);
    pub const SendingHeaders: Self = Self(50i32);
    pub const SendingContent: Self = Self(60i32);
    pub const WaitingForResponse: Self = Self(70i32);
    pub const ReceivingHeaders: Self = Self(80i32);
    pub const ReceivingContent: Self = Self(90i32);
}
impl ::core::marker::Copy for HttpProgressStage {}
impl ::core::clone::Clone for HttpProgressStage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpRequestMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpRequestMessage {}
impl ::core::clone::Clone for HttpRequestMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpRequestResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpRequestResult {}
impl ::core::clone::Clone for HttpRequestResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpResponseMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpResponseMessage {}
impl ::core::clone::Clone for HttpResponseMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpResponseMessageSource(pub i32);
impl HttpResponseMessageSource {
    pub const None: Self = Self(0i32);
    pub const Cache: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
}
impl ::core::marker::Copy for HttpResponseMessageSource {}
impl ::core::clone::Clone for HttpResponseMessageSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpStatusCode(pub i32);
impl HttpStatusCode {
    pub const None: Self = Self(0i32);
    pub const Continue: Self = Self(100i32);
    pub const SwitchingProtocols: Self = Self(101i32);
    pub const Processing: Self = Self(102i32);
    pub const Ok: Self = Self(200i32);
    pub const Created: Self = Self(201i32);
    pub const Accepted: Self = Self(202i32);
    pub const NonAuthoritativeInformation: Self = Self(203i32);
    pub const NoContent: Self = Self(204i32);
    pub const ResetContent: Self = Self(205i32);
    pub const PartialContent: Self = Self(206i32);
    pub const MultiStatus: Self = Self(207i32);
    pub const AlreadyReported: Self = Self(208i32);
    pub const IMUsed: Self = Self(226i32);
    pub const MultipleChoices: Self = Self(300i32);
    pub const MovedPermanently: Self = Self(301i32);
    pub const Found: Self = Self(302i32);
    pub const SeeOther: Self = Self(303i32);
    pub const NotModified: Self = Self(304i32);
    pub const UseProxy: Self = Self(305i32);
    pub const TemporaryRedirect: Self = Self(307i32);
    pub const PermanentRedirect: Self = Self(308i32);
    pub const BadRequest: Self = Self(400i32);
    pub const Unauthorized: Self = Self(401i32);
    pub const PaymentRequired: Self = Self(402i32);
    pub const Forbidden: Self = Self(403i32);
    pub const NotFound: Self = Self(404i32);
    pub const MethodNotAllowed: Self = Self(405i32);
    pub const NotAcceptable: Self = Self(406i32);
    pub const ProxyAuthenticationRequired: Self = Self(407i32);
    pub const RequestTimeout: Self = Self(408i32);
    pub const Conflict: Self = Self(409i32);
    pub const Gone: Self = Self(410i32);
    pub const LengthRequired: Self = Self(411i32);
    pub const PreconditionFailed: Self = Self(412i32);
    pub const RequestEntityTooLarge: Self = Self(413i32);
    pub const RequestUriTooLong: Self = Self(414i32);
    pub const UnsupportedMediaType: Self = Self(415i32);
    pub const RequestedRangeNotSatisfiable: Self = Self(416i32);
    pub const ExpectationFailed: Self = Self(417i32);
    pub const UnprocessableEntity: Self = Self(422i32);
    pub const Locked: Self = Self(423i32);
    pub const FailedDependency: Self = Self(424i32);
    pub const UpgradeRequired: Self = Self(426i32);
    pub const PreconditionRequired: Self = Self(428i32);
    pub const TooManyRequests: Self = Self(429i32);
    pub const RequestHeaderFieldsTooLarge: Self = Self(431i32);
    pub const InternalServerError: Self = Self(500i32);
    pub const NotImplemented: Self = Self(501i32);
    pub const BadGateway: Self = Self(502i32);
    pub const ServiceUnavailable: Self = Self(503i32);
    pub const GatewayTimeout: Self = Self(504i32);
    pub const HttpVersionNotSupported: Self = Self(505i32);
    pub const VariantAlsoNegotiates: Self = Self(506i32);
    pub const InsufficientStorage: Self = Self(507i32);
    pub const LoopDetected: Self = Self(508i32);
    pub const NotExtended: Self = Self(510i32);
    pub const NetworkAuthenticationRequired: Self = Self(511i32);
}
impl ::core::marker::Copy for HttpStatusCode {}
impl ::core::clone::Clone for HttpStatusCode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpStreamContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpStreamContent {}
impl ::core::clone::Clone for HttpStreamContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpStringContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpStringContent {}
impl ::core::clone::Clone for HttpStringContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpTransportInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpTransportInformation {}
impl ::core::clone::Clone for HttpTransportInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpVersion(pub i32);
impl HttpVersion {
    pub const None: Self = Self(0i32);
    pub const Http10: Self = Self(1i32);
    pub const Http11: Self = Self(2i32);
    pub const Http20: Self = Self(3i32);
}
impl ::core::marker::Copy for HttpVersion {}
impl ::core::clone::Clone for HttpVersion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpBufferContentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpBufferContentFactory {}
impl ::core::clone::Clone for IHttpBufferContentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpClient {}
impl ::core::clone::Clone for IHttpClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpClient2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpClient2 {}
impl ::core::clone::Clone for IHttpClient2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpClientFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpClientFactory {}
impl ::core::clone::Clone for IHttpClientFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpContent {}
impl ::core::clone::Clone for IHttpContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpCookie(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpCookie {}
impl ::core::clone::Clone for IHttpCookie {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpCookieFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpCookieFactory {}
impl ::core::clone::Clone for IHttpCookieFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpCookieManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpCookieManager {}
impl ::core::clone::Clone for IHttpCookieManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpFormUrlEncodedContentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpFormUrlEncodedContentFactory {}
impl ::core::clone::Clone for IHttpFormUrlEncodedContentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpGetBufferResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpGetBufferResult {}
impl ::core::clone::Clone for IHttpGetBufferResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpGetInputStreamResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpGetInputStreamResult {}
impl ::core::clone::Clone for IHttpGetInputStreamResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpGetStringResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpGetStringResult {}
impl ::core::clone::Clone for IHttpGetStringResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpMethod(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpMethod {}
impl ::core::clone::Clone for IHttpMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpMethodFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpMethodFactory {}
impl ::core::clone::Clone for IHttpMethodFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpMethodStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpMethodStatics {}
impl ::core::clone::Clone for IHttpMethodStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpMultipartContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpMultipartContent {}
impl ::core::clone::Clone for IHttpMultipartContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpMultipartContentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpMultipartContentFactory {}
impl ::core::clone::Clone for IHttpMultipartContentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpMultipartFormDataContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpMultipartFormDataContent {}
impl ::core::clone::Clone for IHttpMultipartFormDataContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpMultipartFormDataContentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpMultipartFormDataContentFactory {}
impl ::core::clone::Clone for IHttpMultipartFormDataContentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpRequestMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpRequestMessage {}
impl ::core::clone::Clone for IHttpRequestMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpRequestMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpRequestMessageFactory {}
impl ::core::clone::Clone for IHttpRequestMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpRequestResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpRequestResult {}
impl ::core::clone::Clone for IHttpRequestResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpResponseMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpResponseMessage {}
impl ::core::clone::Clone for IHttpResponseMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpResponseMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpResponseMessageFactory {}
impl ::core::clone::Clone for IHttpResponseMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpStreamContentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpStreamContentFactory {}
impl ::core::clone::Clone for IHttpStreamContentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpStringContentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpStringContentFactory {}
impl ::core::clone::Clone for IHttpStringContentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpTransportInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpTransportInformation {}
impl ::core::clone::Clone for IHttpTransportInformation {
    fn clone(&self) -> Self {
        *self
    }
}
