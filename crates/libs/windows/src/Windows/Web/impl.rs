pub trait IUriToStreamResolverImpl: Sized {
    fn UriToStreamAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IInputStream>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebErrorStaticsImpl: Sized {
    fn GetStatus(&self, hresult: i32) -> ::windows::core::Result<WebErrorStatus>;
}
