#[cfg(feature = "implement_exclusive")]
pub trait IHttpCacheDirectiveHeaderValueCollectionImpl: Sized {
    fn MaxAge(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetMaxAge(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn MaxStale(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetMaxStale(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn MinFresh(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetMinFresh(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn SharedMaxAge(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetSharedMaxAge(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpChallengeHeaderValueImpl: Sized {
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Scheme(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpChallengeHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpChallengeHeaderValueFactoryImpl: Sized {
    fn CreateFromScheme(&self, scheme: &::windows::core::HSTRING) -> ::windows::core::Result<HttpChallengeHeaderValue>;
    fn CreateFromSchemeWithToken(&self, scheme: &::windows::core::HSTRING, token: &::windows::core::HSTRING) -> ::windows::core::Result<HttpChallengeHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpChallengeHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpChallengeHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, challengeheadervalue: &mut ::core::option::Option<HttpChallengeHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpConnectionOptionHeaderValueImpl: Sized {
    fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpConnectionOptionHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpConnectionOptionHeaderValueFactoryImpl: Sized {
    fn Create(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<HttpConnectionOptionHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpConnectionOptionHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpConnectionOptionHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, connectionoptionheadervalue: &mut ::core::option::Option<HttpConnectionOptionHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingHeaderValueImpl: Sized {
    fn ContentCoding(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingHeaderValueFactoryImpl: Sized {
    fn Create(&self, contentcoding: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, contentcodingheadervalue: &mut ::core::option::Option<HttpContentCodingHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingWithQualityHeaderValueImpl: Sized {
    fn ContentCoding(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Quality(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingWithQualityHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingWithQualityHeaderValueFactoryImpl: Sized {
    fn CreateFromValue(&self, contentcoding: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue>;
    fn CreateFromValueWithQuality(&self, contentcoding: &::windows::core::HSTRING, quality: f64) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentCodingWithQualityHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, contentcodingwithqualityheadervalue: &mut ::core::option::Option<HttpContentCodingWithQualityHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentDispositionHeaderValueImpl: Sized {
    fn DispositionType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDispositionType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FileNameStar(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFileNameStar(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn SetSize(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u64>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentDispositionHeaderValueFactoryImpl: Sized {
    fn Create(&self, dispositiontype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentDispositionHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentDispositionHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentDispositionHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, contentdispositionheadervalue: &mut ::core::option::Option<HttpContentDispositionHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentHeaderCollectionImpl: Sized {
    fn ContentDisposition(&self) -> ::windows::core::Result<HttpContentDispositionHeaderValue>;
    fn SetContentDisposition(&self, value: &::core::option::Option<HttpContentDispositionHeaderValue>) -> ::windows::core::Result<()>;
    fn ContentEncoding(&self) -> ::windows::core::Result<HttpContentCodingHeaderValueCollection>;
    fn ContentLanguage(&self) -> ::windows::core::Result<HttpLanguageHeaderValueCollection>;
    fn ContentLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn SetContentLength(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u64>>) -> ::windows::core::Result<()>;
    fn ContentLocation(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetContentLocation(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentMD5(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetContentMD5(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ContentRange(&self) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
    fn SetContentRange(&self, value: &::core::option::Option<HttpContentRangeHeaderValue>) -> ::windows::core::Result<()>;
    fn ContentType(&self) -> ::windows::core::Result<HttpMediaTypeHeaderValue>;
    fn SetContentType(&self, value: &::core::option::Option<HttpMediaTypeHeaderValue>) -> ::windows::core::Result<()>;
    fn Expires(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetExpires(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn LastModified(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetLastModified(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Append(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryAppendWithoutValidation(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentRangeHeaderValueImpl: Sized {
    fn FirstBytePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn LastBytePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn Length(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn Unit(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUnit(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentRangeHeaderValueFactoryImpl: Sized {
    fn CreateFromLength(&self, length: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
    fn CreateFromRange(&self, from: u64, to: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
    fn CreateFromRangeWithLength(&self, from: u64, to: u64, length: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpContentRangeHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentRangeHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, contentrangeheadervalue: &mut ::core::option::Option<HttpContentRangeHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookiePairHeaderValueImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookiePairHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookiePairHeaderValueFactoryImpl: Sized {
    fn CreateFromName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookiePairHeaderValue>;
    fn CreateFromNameWithValue(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookiePairHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCookiePairHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookiePairHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, cookiepairheadervalue: &mut ::core::option::Option<HttpCookiePairHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCredentialsHeaderValueImpl: Sized {
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Scheme(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCredentialsHeaderValueFactoryImpl: Sized {
    fn CreateFromScheme(&self, scheme: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
    fn CreateFromSchemeWithToken(&self, scheme: &::windows::core::HSTRING, token: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpCredentialsHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, credentialsheadervalue: &mut ::core::option::Option<HttpCredentialsHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpDateOrDeltaHeaderValueImpl: Sized {
    fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn Delta(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpDateOrDeltaHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpDateOrDeltaHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, dateordeltaheadervalue: &mut ::core::option::Option<HttpDateOrDeltaHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpExpectationHeaderValueImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpExpectationHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpExpectationHeaderValueFactoryImpl: Sized {
    fn CreateFromName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<HttpExpectationHeaderValue>;
    fn CreateFromNameWithValue(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<HttpExpectationHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpExpectationHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpExpectationHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, expectationheadervalue: &mut ::core::option::Option<HttpExpectationHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageRangeWithQualityHeaderValueImpl: Sized {
    fn LanguageRange(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Quality(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageRangeWithQualityHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageRangeWithQualityHeaderValueFactoryImpl: Sized {
    fn CreateFromLanguageRange(&self, languagerange: &::windows::core::HSTRING) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue>;
    fn CreateFromLanguageRangeWithQuality(&self, languagerange: &::windows::core::HSTRING, quality: f64) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpLanguageRangeWithQualityHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, languagerangewithqualityheadervalue: &mut ::core::option::Option<HttpLanguageRangeWithQualityHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeHeaderValueImpl: Sized {
    fn CharSet(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCharSet(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMediaType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeHeaderValueFactoryImpl: Sized {
    fn Create(&self, mediatype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, mediatypeheadervalue: &mut ::core::option::Option<HttpMediaTypeHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeWithQualityHeaderValueImpl: Sized {
    fn CharSet(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCharSet(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMediaType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Quality(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn SetQuality(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeWithQualityHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeWithQualityHeaderValueFactoryImpl: Sized {
    fn CreateFromMediaType(&self, mediatype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue>;
    fn CreateFromMediaTypeWithQuality(&self, mediatype: &::windows::core::HSTRING, quality: f64) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMediaTypeWithQualityHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, mediatypewithqualityheadervalue: &mut ::core::option::Option<HttpMediaTypeWithQualityHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMethodHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpNameValueHeaderValueImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpNameValueHeaderValueFactoryImpl: Sized {
    fn CreateFromName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<HttpNameValueHeaderValue>;
    fn CreateFromNameWithValue(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<HttpNameValueHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpNameValueHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpNameValueHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, namevalueheadervalue: &mut ::core::option::Option<HttpNameValueHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductHeaderValueImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductHeaderValueFactoryImpl: Sized {
    fn CreateFromName(&self, productname: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductHeaderValue>;
    fn CreateFromNameWithVersion(&self, productname: &::windows::core::HSTRING, productversion: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, productheadervalue: &mut ::core::option::Option<HttpProductHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductInfoHeaderValueImpl: Sized {
    fn Product(&self) -> ::windows::core::Result<HttpProductHeaderValue>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductInfoHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductInfoHeaderValueFactoryImpl: Sized {
    fn CreateFromComment(&self, productcomment: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductInfoHeaderValue>;
    fn CreateFromNameWithVersion(&self, productname: &::windows::core::HSTRING, productversion: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductInfoHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpProductInfoHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductInfoHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, productinfoheadervalue: &mut ::core::option::Option<HttpProductInfoHeaderValue>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpRequestHeaderCollectionImpl: Sized {
    fn Accept(&self) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValueCollection>;
    fn AcceptEncoding(&self) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValueCollection>;
    fn AcceptLanguage(&self) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValueCollection>;
    fn Authorization(&self) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
    fn SetAuthorization(&self, value: &::core::option::Option<HttpCredentialsHeaderValue>) -> ::windows::core::Result<()>;
    fn CacheControl(&self) -> ::windows::core::Result<HttpCacheDirectiveHeaderValueCollection>;
    fn Connection(&self) -> ::windows::core::Result<HttpConnectionOptionHeaderValueCollection>;
    fn Cookie(&self) -> ::windows::core::Result<HttpCookiePairHeaderValueCollection>;
    fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetDate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Expect(&self) -> ::windows::core::Result<HttpExpectationHeaderValueCollection>;
    fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFrom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Host(&self) -> ::windows::core::Result<super::super::super::Networking::HostName>;
    fn SetHost(&self, value: &::core::option::Option<super::super::super::Networking::HostName>) -> ::windows::core::Result<()>;
    fn IfModifiedSince(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetIfModifiedSince(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn IfUnmodifiedSince(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetIfUnmodifiedSince(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn MaxForwards(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetMaxForwards(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn ProxyAuthorization(&self) -> ::windows::core::Result<HttpCredentialsHeaderValue>;
    fn SetProxyAuthorization(&self, value: &::core::option::Option<HttpCredentialsHeaderValue>) -> ::windows::core::Result<()>;
    fn Referer(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetReferer(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn TransferEncoding(&self) -> ::windows::core::Result<HttpTransferCodingHeaderValueCollection>;
    fn UserAgent(&self) -> ::windows::core::Result<HttpProductInfoHeaderValueCollection>;
    fn Append(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryAppendWithoutValidation(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpResponseHeaderCollectionImpl: Sized {
    fn Age(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetAge(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Allow(&self) -> ::windows::core::Result<HttpMethodHeaderValueCollection>;
    fn CacheControl(&self) -> ::windows::core::Result<HttpCacheDirectiveHeaderValueCollection>;
    fn Connection(&self) -> ::windows::core::Result<HttpConnectionOptionHeaderValueCollection>;
    fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetDate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetLocation(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ProxyAuthenticate(&self) -> ::windows::core::Result<HttpChallengeHeaderValueCollection>;
    fn RetryAfter(&self) -> ::windows::core::Result<HttpDateOrDeltaHeaderValue>;
    fn SetRetryAfter(&self, value: &::core::option::Option<HttpDateOrDeltaHeaderValue>) -> ::windows::core::Result<()>;
    fn TransferEncoding(&self) -> ::windows::core::Result<HttpTransferCodingHeaderValueCollection>;
    fn WwwAuthenticate(&self) -> ::windows::core::Result<HttpChallengeHeaderValueCollection>;
    fn Append(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryAppendWithoutValidation(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpTransferCodingHeaderValueImpl: Sized {
    fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpTransferCodingHeaderValueCollectionImpl: Sized {
    fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpTransferCodingHeaderValueFactoryImpl: Sized {
    fn Create(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpTransferCodingHeaderValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpTransferCodingHeaderValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpTransferCodingHeaderValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, transfercodingheadervalue: &mut ::core::option::Option<HttpTransferCodingHeaderValue>) -> ::windows::core::Result<bool>;
}
