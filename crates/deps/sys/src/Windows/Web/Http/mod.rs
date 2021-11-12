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
#[repr(transparent)]
pub struct HttpClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpCompletionOption(pub i32);
impl HttpCompletionOption {
    pub const ResponseContentRead: HttpCompletionOption = HttpCompletionOption(0i32);
    pub const ResponseHeadersRead: HttpCompletionOption = HttpCompletionOption(1i32);
}
#[repr(transparent)]
pub struct HttpCookie(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpCookieCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpCookieManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpFormUrlEncodedContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpGetBufferResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpGetInputStreamResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpGetStringResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpMethod(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpMultipartContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpMultipartFormDataContent(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct HttpProgress(i32);
#[repr(transparent)]
pub struct HttpProgressStage(pub i32);
impl HttpProgressStage {
    pub const None: HttpProgressStage = HttpProgressStage(0i32);
    pub const DetectingProxy: HttpProgressStage = HttpProgressStage(10i32);
    pub const ResolvingName: HttpProgressStage = HttpProgressStage(20i32);
    pub const ConnectingToServer: HttpProgressStage = HttpProgressStage(30i32);
    pub const NegotiatingSsl: HttpProgressStage = HttpProgressStage(40i32);
    pub const SendingHeaders: HttpProgressStage = HttpProgressStage(50i32);
    pub const SendingContent: HttpProgressStage = HttpProgressStage(60i32);
    pub const WaitingForResponse: HttpProgressStage = HttpProgressStage(70i32);
    pub const ReceivingHeaders: HttpProgressStage = HttpProgressStage(80i32);
    pub const ReceivingContent: HttpProgressStage = HttpProgressStage(90i32);
}
#[repr(transparent)]
pub struct HttpRequestMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpRequestResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpResponseMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpResponseMessageSource(pub i32);
impl HttpResponseMessageSource {
    pub const None: HttpResponseMessageSource = HttpResponseMessageSource(0i32);
    pub const Cache: HttpResponseMessageSource = HttpResponseMessageSource(1i32);
    pub const Network: HttpResponseMessageSource = HttpResponseMessageSource(2i32);
}
#[repr(transparent)]
pub struct HttpStatusCode(pub i32);
impl HttpStatusCode {
    pub const None: HttpStatusCode = HttpStatusCode(0i32);
    pub const Continue: HttpStatusCode = HttpStatusCode(100i32);
    pub const SwitchingProtocols: HttpStatusCode = HttpStatusCode(101i32);
    pub const Processing: HttpStatusCode = HttpStatusCode(102i32);
    pub const Ok: HttpStatusCode = HttpStatusCode(200i32);
    pub const Created: HttpStatusCode = HttpStatusCode(201i32);
    pub const Accepted: HttpStatusCode = HttpStatusCode(202i32);
    pub const NonAuthoritativeInformation: HttpStatusCode = HttpStatusCode(203i32);
    pub const NoContent: HttpStatusCode = HttpStatusCode(204i32);
    pub const ResetContent: HttpStatusCode = HttpStatusCode(205i32);
    pub const PartialContent: HttpStatusCode = HttpStatusCode(206i32);
    pub const MultiStatus: HttpStatusCode = HttpStatusCode(207i32);
    pub const AlreadyReported: HttpStatusCode = HttpStatusCode(208i32);
    pub const IMUsed: HttpStatusCode = HttpStatusCode(226i32);
    pub const MultipleChoices: HttpStatusCode = HttpStatusCode(300i32);
    pub const MovedPermanently: HttpStatusCode = HttpStatusCode(301i32);
    pub const Found: HttpStatusCode = HttpStatusCode(302i32);
    pub const SeeOther: HttpStatusCode = HttpStatusCode(303i32);
    pub const NotModified: HttpStatusCode = HttpStatusCode(304i32);
    pub const UseProxy: HttpStatusCode = HttpStatusCode(305i32);
    pub const TemporaryRedirect: HttpStatusCode = HttpStatusCode(307i32);
    pub const PermanentRedirect: HttpStatusCode = HttpStatusCode(308i32);
    pub const BadRequest: HttpStatusCode = HttpStatusCode(400i32);
    pub const Unauthorized: HttpStatusCode = HttpStatusCode(401i32);
    pub const PaymentRequired: HttpStatusCode = HttpStatusCode(402i32);
    pub const Forbidden: HttpStatusCode = HttpStatusCode(403i32);
    pub const NotFound: HttpStatusCode = HttpStatusCode(404i32);
    pub const MethodNotAllowed: HttpStatusCode = HttpStatusCode(405i32);
    pub const NotAcceptable: HttpStatusCode = HttpStatusCode(406i32);
    pub const ProxyAuthenticationRequired: HttpStatusCode = HttpStatusCode(407i32);
    pub const RequestTimeout: HttpStatusCode = HttpStatusCode(408i32);
    pub const Conflict: HttpStatusCode = HttpStatusCode(409i32);
    pub const Gone: HttpStatusCode = HttpStatusCode(410i32);
    pub const LengthRequired: HttpStatusCode = HttpStatusCode(411i32);
    pub const PreconditionFailed: HttpStatusCode = HttpStatusCode(412i32);
    pub const RequestEntityTooLarge: HttpStatusCode = HttpStatusCode(413i32);
    pub const RequestUriTooLong: HttpStatusCode = HttpStatusCode(414i32);
    pub const UnsupportedMediaType: HttpStatusCode = HttpStatusCode(415i32);
    pub const RequestedRangeNotSatisfiable: HttpStatusCode = HttpStatusCode(416i32);
    pub const ExpectationFailed: HttpStatusCode = HttpStatusCode(417i32);
    pub const UnprocessableEntity: HttpStatusCode = HttpStatusCode(422i32);
    pub const Locked: HttpStatusCode = HttpStatusCode(423i32);
    pub const FailedDependency: HttpStatusCode = HttpStatusCode(424i32);
    pub const UpgradeRequired: HttpStatusCode = HttpStatusCode(426i32);
    pub const PreconditionRequired: HttpStatusCode = HttpStatusCode(428i32);
    pub const TooManyRequests: HttpStatusCode = HttpStatusCode(429i32);
    pub const RequestHeaderFieldsTooLarge: HttpStatusCode = HttpStatusCode(431i32);
    pub const InternalServerError: HttpStatusCode = HttpStatusCode(500i32);
    pub const NotImplemented: HttpStatusCode = HttpStatusCode(501i32);
    pub const BadGateway: HttpStatusCode = HttpStatusCode(502i32);
    pub const ServiceUnavailable: HttpStatusCode = HttpStatusCode(503i32);
    pub const GatewayTimeout: HttpStatusCode = HttpStatusCode(504i32);
    pub const HttpVersionNotSupported: HttpStatusCode = HttpStatusCode(505i32);
    pub const VariantAlsoNegotiates: HttpStatusCode = HttpStatusCode(506i32);
    pub const InsufficientStorage: HttpStatusCode = HttpStatusCode(507i32);
    pub const LoopDetected: HttpStatusCode = HttpStatusCode(508i32);
    pub const NotExtended: HttpStatusCode = HttpStatusCode(510i32);
    pub const NetworkAuthenticationRequired: HttpStatusCode = HttpStatusCode(511i32);
}
#[repr(transparent)]
pub struct HttpStreamContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpStringContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpTransportInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpVersion(pub i32);
impl HttpVersion {
    pub const None: HttpVersion = HttpVersion(0i32);
    pub const Http10: HttpVersion = HttpVersion(1i32);
    pub const Http11: HttpVersion = HttpVersion(2i32);
    pub const Http20: HttpVersion = HttpVersion(3i32);
}
#[repr(transparent)]
pub struct IHttpBufferContentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpClient2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpClientFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpCookie(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpCookieFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpCookieManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpFormUrlEncodedContentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpGetBufferResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpGetInputStreamResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpGetStringResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpMethod(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpMethodFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpMethodStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpMultipartContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpMultipartContentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpMultipartFormDataContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpMultipartFormDataContentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpRequestMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpRequestMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpRequestResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpResponseMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpResponseMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpStreamContentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpStringContentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpTransportInformation(pub *mut ::core::ffi::c_void);
