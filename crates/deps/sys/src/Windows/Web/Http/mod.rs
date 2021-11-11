#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Web_Http_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "Web_Http_Filters")]
pub mod Filters;
#[cfg(feature = "Web_Http_Headers")]
pub mod Headers;
#[link(name = "windows")]
extern "system" {
    fn HttpBufferContent();
    fn HttpClient();
    fn HttpCompletionOption();
    fn HttpCookie();
    fn HttpCookieCollection();
    fn HttpCookieManager();
    fn HttpFormUrlEncodedContent();
    fn HttpGetBufferResult();
    fn HttpGetInputStreamResult();
    fn HttpGetStringResult();
    fn HttpMethod();
    fn HttpMultipartContent();
    fn HttpMultipartFormDataContent();
    fn HttpProgress();
    fn HttpProgressStage();
    fn HttpRequestMessage();
    fn HttpRequestResult();
    fn HttpResponseMessage();
    fn HttpResponseMessageSource();
    fn HttpStatusCode();
    fn HttpStreamContent();
    fn HttpStringContent();
    fn HttpTransportInformation();
    fn HttpVersion();
    fn IHttpBufferContentFactory();
    fn IHttpClient();
    fn IHttpClient2();
    fn IHttpClientFactory();
    fn IHttpContent();
    fn IHttpCookie();
    fn IHttpCookieFactory();
    fn IHttpCookieManager();
    fn IHttpFormUrlEncodedContentFactory();
    fn IHttpGetBufferResult();
    fn IHttpGetInputStreamResult();
    fn IHttpGetStringResult();
    fn IHttpMethod();
    fn IHttpMethodFactory();
    fn IHttpMethodStatics();
    fn IHttpMultipartContent();
    fn IHttpMultipartContentFactory();
    fn IHttpMultipartFormDataContent();
    fn IHttpMultipartFormDataContentFactory();
    fn IHttpRequestMessage();
    fn IHttpRequestMessageFactory();
    fn IHttpRequestResult();
    fn IHttpResponseMessage();
    fn IHttpResponseMessageFactory();
    fn IHttpStreamContentFactory();
    fn IHttpStringContentFactory();
    fn IHttpTransportInformation();
}
