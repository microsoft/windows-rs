#[cfg(feature = "implement_exclusive")]
pub trait IHttpDiagnosticProviderImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn RequestSent(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRequestSent(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResponseReceived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveResponseReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestResponseCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRequestResponseCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpDiagnosticProviderRequestResponseCompletedEventArgsImpl: Sized {
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Timestamps(&self) -> ::windows::core::Result<HttpDiagnosticProviderRequestResponseTimestamps>;
    fn RequestedUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ProcessId(&self) -> ::windows::core::Result<u32>;
    fn ThreadId(&self) -> ::windows::core::Result<u32>;
    fn Initiator(&self) -> ::windows::core::Result<HttpDiagnosticRequestInitiator>;
    fn SourceLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpDiagnosticProviderRequestResponseTimestampsImpl: Sized {
    fn CacheCheckedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ConnectionInitiatedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn NameResolvedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SslNegotiatedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ConnectionCompletedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn RequestSentTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn RequestCompletedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ResponseReceivedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ResponseCompletedTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpDiagnosticProviderRequestSentEventArgsImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Message(&self) -> ::windows::core::Result<super::HttpRequestMessage>;
    fn ProcessId(&self) -> ::windows::core::Result<u32>;
    fn ThreadId(&self) -> ::windows::core::Result<u32>;
    fn Initiator(&self) -> ::windows::core::Result<HttpDiagnosticRequestInitiator>;
    fn SourceLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpDiagnosticProviderResponseReceivedEventArgsImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Message(&self) -> ::windows::core::Result<super::HttpResponseMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpDiagnosticProviderStaticsImpl: Sized {
    fn CreateFromProcessDiagnosticInfo(&self, processdiagnosticinfo: &::core::option::Option<super::super::super::System::Diagnostics::ProcessDiagnosticInfo>) -> ::windows::core::Result<HttpDiagnosticProvider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpDiagnosticSourceLocationImpl: Sized {
    fn SourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn LineNumber(&self) -> ::windows::core::Result<u64>;
    fn ColumnNumber(&self) -> ::windows::core::Result<u64>;
}
