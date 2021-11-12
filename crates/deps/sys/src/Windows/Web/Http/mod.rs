#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct HttpCompletionOption(i32);
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
#[repr(C)]
pub struct HttpProgressStage(i32);
#[repr(transparent)]
pub struct HttpRequestMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpRequestResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpResponseMessage(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct HttpResponseMessageSource(i32);
#[repr(C)]
pub struct HttpStatusCode(i32);
#[repr(transparent)]
pub struct HttpStreamContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpStringContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpTransportInformation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct HttpVersion(i32);
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
