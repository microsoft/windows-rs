#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Web_AtomPub")]
pub mod AtomPub;
#[cfg(feature = "Web_Http")]
pub mod Http;
#[cfg(feature = "Web_Syndication")]
pub mod Syndication;
#[cfg(feature = "Web_UI")]
pub mod UI;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUriToStreamResolver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebErrorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebErrorStatus(pub i32);
impl WebErrorStatus {
    pub const Unknown: WebErrorStatus = WebErrorStatus(0i32);
    pub const CertificateCommonNameIsIncorrect: WebErrorStatus = WebErrorStatus(1i32);
    pub const CertificateExpired: WebErrorStatus = WebErrorStatus(2i32);
    pub const CertificateContainsErrors: WebErrorStatus = WebErrorStatus(3i32);
    pub const CertificateRevoked: WebErrorStatus = WebErrorStatus(4i32);
    pub const CertificateIsInvalid: WebErrorStatus = WebErrorStatus(5i32);
    pub const ServerUnreachable: WebErrorStatus = WebErrorStatus(6i32);
    pub const Timeout: WebErrorStatus = WebErrorStatus(7i32);
    pub const ErrorHttpInvalidServerResponse: WebErrorStatus = WebErrorStatus(8i32);
    pub const ConnectionAborted: WebErrorStatus = WebErrorStatus(9i32);
    pub const ConnectionReset: WebErrorStatus = WebErrorStatus(10i32);
    pub const Disconnected: WebErrorStatus = WebErrorStatus(11i32);
    pub const HttpToHttpsOnRedirection: WebErrorStatus = WebErrorStatus(12i32);
    pub const HttpsToHttpOnRedirection: WebErrorStatus = WebErrorStatus(13i32);
    pub const CannotConnect: WebErrorStatus = WebErrorStatus(14i32);
    pub const HostNameNotResolved: WebErrorStatus = WebErrorStatus(15i32);
    pub const OperationCanceled: WebErrorStatus = WebErrorStatus(16i32);
    pub const RedirectFailed: WebErrorStatus = WebErrorStatus(17i32);
    pub const UnexpectedStatusCode: WebErrorStatus = WebErrorStatus(18i32);
    pub const UnexpectedRedirection: WebErrorStatus = WebErrorStatus(19i32);
    pub const UnexpectedClientError: WebErrorStatus = WebErrorStatus(20i32);
    pub const UnexpectedServerError: WebErrorStatus = WebErrorStatus(21i32);
    pub const InsufficientRangeSupport: WebErrorStatus = WebErrorStatus(22i32);
    pub const MissingContentLengthSupport: WebErrorStatus = WebErrorStatus(23i32);
    pub const MultipleChoices: WebErrorStatus = WebErrorStatus(300i32);
    pub const MovedPermanently: WebErrorStatus = WebErrorStatus(301i32);
    pub const Found: WebErrorStatus = WebErrorStatus(302i32);
    pub const SeeOther: WebErrorStatus = WebErrorStatus(303i32);
    pub const NotModified: WebErrorStatus = WebErrorStatus(304i32);
    pub const UseProxy: WebErrorStatus = WebErrorStatus(305i32);
    pub const TemporaryRedirect: WebErrorStatus = WebErrorStatus(307i32);
    pub const BadRequest: WebErrorStatus = WebErrorStatus(400i32);
    pub const Unauthorized: WebErrorStatus = WebErrorStatus(401i32);
    pub const PaymentRequired: WebErrorStatus = WebErrorStatus(402i32);
    pub const Forbidden: WebErrorStatus = WebErrorStatus(403i32);
    pub const NotFound: WebErrorStatus = WebErrorStatus(404i32);
    pub const MethodNotAllowed: WebErrorStatus = WebErrorStatus(405i32);
    pub const NotAcceptable: WebErrorStatus = WebErrorStatus(406i32);
    pub const ProxyAuthenticationRequired: WebErrorStatus = WebErrorStatus(407i32);
    pub const RequestTimeout: WebErrorStatus = WebErrorStatus(408i32);
    pub const Conflict: WebErrorStatus = WebErrorStatus(409i32);
    pub const Gone: WebErrorStatus = WebErrorStatus(410i32);
    pub const LengthRequired: WebErrorStatus = WebErrorStatus(411i32);
    pub const PreconditionFailed: WebErrorStatus = WebErrorStatus(412i32);
    pub const RequestEntityTooLarge: WebErrorStatus = WebErrorStatus(413i32);
    pub const RequestUriTooLong: WebErrorStatus = WebErrorStatus(414i32);
    pub const UnsupportedMediaType: WebErrorStatus = WebErrorStatus(415i32);
    pub const RequestedRangeNotSatisfiable: WebErrorStatus = WebErrorStatus(416i32);
    pub const ExpectationFailed: WebErrorStatus = WebErrorStatus(417i32);
    pub const InternalServerError: WebErrorStatus = WebErrorStatus(500i32);
    pub const NotImplemented: WebErrorStatus = WebErrorStatus(501i32);
    pub const BadGateway: WebErrorStatus = WebErrorStatus(502i32);
    pub const ServiceUnavailable: WebErrorStatus = WebErrorStatus(503i32);
    pub const GatewayTimeout: WebErrorStatus = WebErrorStatus(504i32);
    pub const HttpVersionNotSupported: WebErrorStatus = WebErrorStatus(505i32);
}
