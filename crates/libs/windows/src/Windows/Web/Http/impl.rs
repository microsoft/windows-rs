#[cfg(feature = "implement_exclusive")]
pub trait IHttpBufferContentFactoryImpl: Sized {
    fn CreateFromBuffer(&self, content: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<HttpBufferContent>;
    fn CreateFromBufferWithOffset(&self, content: &::core::option::Option<super::super::Storage::Streams::IBuffer>, offset: u32, count: u32) -> ::windows::core::Result<HttpBufferContent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpClientImpl: Sized {
    fn DeleteAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>;
    fn GetAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>;
    fn GetWithOptionAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, completionoption: HttpCompletionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>;
    fn GetBufferAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, HttpProgress>>;
    fn GetInputStreamAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, HttpProgress>>;
    fn GetStringAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, HttpProgress>>;
    fn PostAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, content: &::core::option::Option<IHttpContent>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>;
    fn PutAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, content: &::core::option::Option<IHttpContent>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>;
    fn SendRequestAsync(&self, request: &::core::option::Option<HttpRequestMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>;
    fn SendRequestWithOptionAsync(&self, request: &::core::option::Option<HttpRequestMessage>, completionoption: HttpCompletionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>;
    fn DefaultRequestHeaders(&self) -> ::windows::core::Result<Headers::HttpRequestHeaderCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpClient2Impl: Sized {
    fn TryDeleteAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>;
    fn TryGetAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>;
    fn TryGetAsync2(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, completionoption: HttpCompletionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>;
    fn TryGetBufferAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpGetBufferResult, HttpProgress>>;
    fn TryGetInputStreamAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpGetInputStreamResult, HttpProgress>>;
    fn TryGetStringAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpGetStringResult, HttpProgress>>;
    fn TryPostAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, content: &::core::option::Option<IHttpContent>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>;
    fn TryPutAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, content: &::core::option::Option<IHttpContent>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>;
    fn TrySendRequestAsync(&self, request: &::core::option::Option<HttpRequestMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>;
    fn TrySendRequestAsync2(&self, request: &::core::option::Option<HttpRequestMessage>, completionoption: HttpCompletionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpClientFactoryImpl: Sized {
    fn Create(&self, filter: &::core::option::Option<Filters::IHttpFilter>) -> ::windows::core::Result<HttpClient>;
}
#[cfg(feature = "Foundation")]
pub trait IHttpContentImpl: Sized + IClosableImpl {
    fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection>;
    fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
    fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>;
    fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>;
    fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>;
    fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool>;
    fn WriteToStreamAsync(&self, outputstream: &::core::option::Option<super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookieImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Expires(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetExpires(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn HttpOnly(&self) -> ::windows::core::Result<bool>;
    fn SetHttpOnly(&self, value: bool) -> ::windows::core::Result<()>;
    fn Secure(&self) -> ::windows::core::Result<bool>;
    fn SetSecure(&self, value: bool) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookieFactoryImpl: Sized {
    fn Create(&self, name: &::windows::core::HSTRING, domain: &::windows::core::HSTRING, path: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookie>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookieManagerImpl: Sized {
    fn SetCookie(&self, cookie: &::core::option::Option<HttpCookie>) -> ::windows::core::Result<bool>;
    fn SetCookieWithThirdParty(&self, cookie: &::core::option::Option<HttpCookie>, thirdparty: bool) -> ::windows::core::Result<bool>;
    fn DeleteCookie(&self, cookie: &::core::option::Option<HttpCookie>) -> ::windows::core::Result<()>;
    fn GetCookies(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<HttpCookieCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpFormUrlEncodedContentFactoryImpl: Sized {
    fn Create(&self, content: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<HttpFormUrlEncodedContent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpGetBufferResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage>;
    fn ResponseMessage(&self) -> ::windows::core::Result<HttpResponseMessage>;
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn Value(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpGetInputStreamResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage>;
    fn ResponseMessage(&self) -> ::windows::core::Result<HttpResponseMessage>;
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn Value(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpGetStringResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage>;
    fn ResponseMessage(&self) -> ::windows::core::Result<HttpResponseMessage>;
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMethodImpl: Sized {
    fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMethodFactoryImpl: Sized {
    fn Create(&self, method: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMethod>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMethodStaticsImpl: Sized {
    fn Delete(&self) -> ::windows::core::Result<HttpMethod>;
    fn Get(&self) -> ::windows::core::Result<HttpMethod>;
    fn Head(&self) -> ::windows::core::Result<HttpMethod>;
    fn Options(&self) -> ::windows::core::Result<HttpMethod>;
    fn Patch(&self) -> ::windows::core::Result<HttpMethod>;
    fn Post(&self) -> ::windows::core::Result<HttpMethod>;
    fn Put(&self) -> ::windows::core::Result<HttpMethod>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMultipartContentImpl: Sized {
    fn Add(&self, content: &::core::option::Option<IHttpContent>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMultipartContentFactoryImpl: Sized {
    fn CreateWithSubtype(&self, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMultipartContent>;
    fn CreateWithSubtypeAndBoundary(&self, subtype: &::windows::core::HSTRING, boundary: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMultipartContent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMultipartFormDataContentImpl: Sized {
    fn Add(&self, content: &::core::option::Option<IHttpContent>) -> ::windows::core::Result<()>;
    fn AddWithName(&self, content: &::core::option::Option<IHttpContent>, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddWithNameAndFileName(&self, content: &::core::option::Option<IHttpContent>, name: &::windows::core::HSTRING, filename: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMultipartFormDataContentFactoryImpl: Sized {
    fn CreateWithBoundary(&self, boundary: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMultipartFormDataContent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpRequestMessageImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<IHttpContent>;
    fn SetContent(&self, value: &::core::option::Option<IHttpContent>) -> ::windows::core::Result<()>;
    fn Headers(&self) -> ::windows::core::Result<Headers::HttpRequestHeaderCollection>;
    fn Method(&self) -> ::windows::core::Result<HttpMethod>;
    fn SetMethod(&self, value: &::core::option::Option<HttpMethod>) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn RequestUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetRequestUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn TransportInformation(&self) -> ::windows::core::Result<HttpTransportInformation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpRequestMessageFactoryImpl: Sized {
    fn Create(&self, method: &::core::option::Option<HttpMethod>, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<HttpRequestMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpRequestResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage>;
    fn ResponseMessage(&self) -> ::windows::core::Result<HttpResponseMessage>;
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpResponseMessageImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<IHttpContent>;
    fn SetContent(&self, value: &::core::option::Option<IHttpContent>) -> ::windows::core::Result<()>;
    fn Headers(&self) -> ::windows::core::Result<Headers::HttpResponseHeaderCollection>;
    fn IsSuccessStatusCode(&self) -> ::windows::core::Result<bool>;
    fn ReasonPhrase(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetReasonPhrase(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RequestMessage(&self) -> ::windows::core::Result<HttpRequestMessage>;
    fn SetRequestMessage(&self, value: &::core::option::Option<HttpRequestMessage>) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<HttpResponseMessageSource>;
    fn SetSource(&self, value: HttpResponseMessageSource) -> ::windows::core::Result<()>;
    fn StatusCode(&self) -> ::windows::core::Result<HttpStatusCode>;
    fn SetStatusCode(&self, value: HttpStatusCode) -> ::windows::core::Result<()>;
    fn Version(&self) -> ::windows::core::Result<HttpVersion>;
    fn SetVersion(&self, value: HttpVersion) -> ::windows::core::Result<()>;
    fn EnsureSuccessStatusCode(&self) -> ::windows::core::Result<HttpResponseMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpResponseMessageFactoryImpl: Sized {
    fn Create(&self, statuscode: HttpStatusCode) -> ::windows::core::Result<HttpResponseMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpStreamContentFactoryImpl: Sized {
    fn CreateFromInputStream(&self, content: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<HttpStreamContent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpStringContentFactoryImpl: Sized {
    fn CreateFromString(&self, content: &::windows::core::HSTRING) -> ::windows::core::Result<HttpStringContent>;
    fn CreateFromStringWithEncoding(&self, content: &::windows::core::HSTRING, encoding: super::super::Storage::Streams::UnicodeEncoding) -> ::windows::core::Result<HttpStringContent>;
    fn CreateFromStringWithEncodingAndMediaType(&self, content: &::windows::core::HSTRING, encoding: super::super::Storage::Streams::UnicodeEncoding, mediatype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpStringContent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpTransportInformationImpl: Sized {
    fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<super::super::Networking::Sockets::SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
}
