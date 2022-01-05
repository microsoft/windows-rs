#[cfg(feature = "implement_exclusive")]
pub trait ISysStorageProviderEventReceivedEventArgsImpl: Sized {
    fn Json(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISysStorageProviderEventReceivedEventArgsFactoryImpl: Sized {
    fn CreateInstance(&self, json: &::windows::core::HSTRING) -> ::windows::core::Result<SysStorageProviderEventReceivedEventArgs>;
}
pub trait ISysStorageProviderEventSourceImpl: Sized {
    fn EventReceived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEventReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
pub trait ISysStorageProviderHandlerFactoryImpl: Sized {
    fn GetHttpRequestProvider(&self, syncrootid: &::windows::core::HSTRING) -> ::windows::core::Result<ISysStorageProviderHttpRequestProvider>;
    fn GetEventSource(&self, syncrootid: &::windows::core::HSTRING, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<ISysStorageProviderEventSource>;
}
pub trait ISysStorageProviderHttpRequestProviderImpl: Sized {
    fn SendRequestAsync(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>>;
}
