pub trait IWebViewControlImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSource(&self, source: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanGoBack(&self) -> ::windows::core::Result<bool>;
    fn CanGoForward(&self) -> ::windows::core::Result<bool>;
    fn SetDefaultBackgroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn DefaultBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn ContainsFullScreenElement(&self) -> ::windows::core::Result<bool>;
    fn Settings(&self) -> ::windows::core::Result<WebViewControlSettings>;
    fn DeferredPermissionRequests(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WebViewControlDeferredPermissionRequest>>;
    fn GoForward(&self) -> ::windows::core::Result<()>;
    fn GoBack(&self) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Navigate(&self, source: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn NavigateToString(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NavigateToLocalStreamUri(&self, source: &::core::option::Option<super::super::Foundation::Uri>, streamresolver: &::core::option::Option<super::IUriToStreamResolver>) -> ::windows::core::Result<()>;
    fn NavigateWithHttpRequestMessage(&self, requestmessage: &::core::option::Option<super::Http::HttpRequestMessage>) -> ::windows::core::Result<()>;
    fn InvokeScriptAsync(&self, scriptname: &::windows::core::HSTRING, arguments: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn CapturePreviewToStreamAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CaptureSelectedContentToDataPackageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>>;
    fn BuildLocalStreamUri(&self, contentidentifier: &::windows::core::HSTRING, relativepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut ::core::option::Option<WebViewControlDeferredPermissionRequest>) -> ::windows::core::Result<()>;
    fn NavigationStarting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContentLoading(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentLoading(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DOMContentLoaded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDOMContentLoaded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameNavigationStarting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameContentLoading(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameContentLoading(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameDOMContentLoaded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameDOMContentLoaded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameNavigationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ScriptNotify(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScriptNotify(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LongRunningScriptDetected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLongRunningScriptDetected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UnsafeContentWarningDisplaying(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsafeContentWarningDisplaying(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UnviewableContentIdentified(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnviewableContentIdentified(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PermissionRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePermissionRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UnsupportedUriSchemeIdentified(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsupportedUriSchemeIdentified(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NewWindowRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNewWindowRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContainsFullScreenElementChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContainsFullScreenElementChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WebResourceRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveWebResourceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
pub trait IWebViewControl2Impl: Sized {
    fn AddInitializeScript(&self, script: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlContentLoadingEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlDOMContentLoadedEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlDeferredPermissionRequestImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn PermissionType(&self) -> ::windows::core::Result<WebViewControlPermissionType>;
    fn Allow(&self) -> ::windows::core::Result<()>;
    fn Deny(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlLongRunningScriptDetectedEventArgsImpl: Sized {
    fn ExecutionTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn StopPageScriptExecution(&self) -> ::windows::core::Result<bool>;
    fn SetStopPageScriptExecution(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlNavigationCompletedEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn IsSuccess(&self) -> ::windows::core::Result<bool>;
    fn WebErrorStatus(&self) -> ::windows::core::Result<super::WebErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlNavigationStartingEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlNewWindowRequestedEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Referrer(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlNewWindowRequestedEventArgs2Impl: Sized {
    fn NewWindow(&self) -> ::windows::core::Result<IWebViewControl>;
    fn SetNewWindow(&self, value: &::core::option::Option<IWebViewControl>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlPermissionRequestImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn PermissionType(&self) -> ::windows::core::Result<WebViewControlPermissionType>;
    fn State(&self) -> ::windows::core::Result<WebViewControlPermissionState>;
    fn Defer(&self) -> ::windows::core::Result<()>;
    fn Allow(&self) -> ::windows::core::Result<()>;
    fn Deny(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlPermissionRequestedEventArgsImpl: Sized {
    fn PermissionRequest(&self) -> ::windows::core::Result<WebViewControlPermissionRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlScriptNotifyEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlSettingsImpl: Sized {
    fn SetIsJavaScriptEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsJavaScriptEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsIndexedDBEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsIndexedDBEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsScriptNotifyAllowed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsScriptNotifyAllowed(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlUnsupportedUriSchemeIdentifiedEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlUnviewableContentIdentifiedEventArgsImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Referrer(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlWebResourceRequestedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
    fn Request(&self) -> ::windows::core::Result<super::Http::HttpRequestMessage>;
    fn SetResponse(&self, value: &::core::option::Option<super::Http::HttpResponseMessage>) -> ::windows::core::Result<()>;
    fn Response(&self) -> ::windows::core::Result<super::Http::HttpResponseMessage>;
}
