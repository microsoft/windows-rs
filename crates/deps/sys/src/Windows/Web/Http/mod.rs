#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Web_Http_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "Web_Http_Filters")]
pub mod Filters;
#[cfg(feature = "Web_Http_Headers")]
pub mod Headers;
#[link(name = "windows")]
extern "system" {}
pub struct HttpBufferContent(i32);
pub struct HttpClient(i32);
pub struct HttpCompletionOption(i32);
pub struct HttpCookie(i32);
pub struct HttpCookieCollection(i32);
pub struct HttpCookieManager(i32);
pub struct HttpFormUrlEncodedContent(i32);
pub struct HttpGetBufferResult(i32);
pub struct HttpGetInputStreamResult(i32);
pub struct HttpGetStringResult(i32);
pub struct HttpMethod(i32);
pub struct HttpMultipartContent(i32);
pub struct HttpMultipartFormDataContent(i32);
#[cfg(feature = "Foundation")]
pub struct HttpProgress(i32);
pub struct HttpProgressStage(i32);
pub struct HttpRequestMessage(i32);
pub struct HttpRequestResult(i32);
pub struct HttpResponseMessage(i32);
pub struct HttpResponseMessageSource(i32);
pub struct HttpStatusCode(i32);
pub struct HttpStreamContent(i32);
pub struct HttpStringContent(i32);
pub struct HttpTransportInformation(i32);
pub struct HttpVersion(i32);
pub struct IHttpBufferContentFactory(pub *mut ::core::ffi::c_void);
pub struct IHttpClient(pub *mut ::core::ffi::c_void);
pub struct IHttpClient2(pub *mut ::core::ffi::c_void);
pub struct IHttpClientFactory(pub *mut ::core::ffi::c_void);
pub struct IHttpContent(pub *mut ::core::ffi::c_void);
pub struct IHttpCookie(pub *mut ::core::ffi::c_void);
pub struct IHttpCookieFactory(pub *mut ::core::ffi::c_void);
pub struct IHttpCookieManager(pub *mut ::core::ffi::c_void);
pub struct IHttpFormUrlEncodedContentFactory(pub *mut ::core::ffi::c_void);
pub struct IHttpGetBufferResult(pub *mut ::core::ffi::c_void);
pub struct IHttpGetInputStreamResult(pub *mut ::core::ffi::c_void);
pub struct IHttpGetStringResult(pub *mut ::core::ffi::c_void);
pub struct IHttpMethod(pub *mut ::core::ffi::c_void);
pub struct IHttpMethodFactory(pub *mut ::core::ffi::c_void);
pub struct IHttpMethodStatics(pub *mut ::core::ffi::c_void);
pub struct IHttpMultipartContent(pub *mut ::core::ffi::c_void);
pub struct IHttpMultipartContentFactory(pub *mut ::core::ffi::c_void);
pub struct IHttpMultipartFormDataContent(pub *mut ::core::ffi::c_void);
pub struct IHttpMultipartFormDataContentFactory(pub *mut ::core::ffi::c_void);
pub struct IHttpRequestMessage(pub *mut ::core::ffi::c_void);
pub struct IHttpRequestMessageFactory(pub *mut ::core::ffi::c_void);
pub struct IHttpRequestResult(pub *mut ::core::ffi::c_void);
pub struct IHttpResponseMessage(pub *mut ::core::ffi::c_void);
pub struct IHttpResponseMessageFactory(pub *mut ::core::ffi::c_void);
pub struct IHttpStreamContentFactory(pub *mut ::core::ffi::c_void);
pub struct IHttpStringContentFactory(pub *mut ::core::ffi::c_void);
pub struct IHttpTransportInformation(pub *mut ::core::ffi::c_void);
