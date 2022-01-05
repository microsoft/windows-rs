#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlAcceleratorKeyPressedEventArgsImpl: Sized {
    fn EventType(&self) -> ::windows::core::Result<super::super::super::UI::Core::CoreAcceleratorKeyEventType>;
    fn VirtualKey(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
    fn KeyStatus(&self) -> ::windows::core::Result<super::super::super::UI::Core::CorePhysicalKeyStatus>;
    fn RoutingStage(&self) -> ::windows::core::Result<WebViewControlAcceleratorKeyRoutingStage>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlMoveFocusRequestedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<WebViewControlMoveFocusReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlProcessImpl: Sized {
    fn ProcessId(&self) -> ::windows::core::Result<u32>;
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsPrivateNetworkClientServerCapabilityEnabled(&self) -> ::windows::core::Result<bool>;
    fn CreateWebViewControlAsync(&self, hostwindowhandle: i64, bounds: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebViewControl>>;
    fn GetWebViewControls(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<WebViewControl>>;
    fn Terminate(&self) -> ::windows::core::Result<()>;
    fn ProcessExited(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebViewControlProcess, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveProcessExited(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlProcessFactoryImpl: Sized {
    fn CreateWithOptions(&self, processoptions: &::core::option::Option<WebViewControlProcessOptions>) -> ::windows::core::Result<WebViewControlProcess>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlProcessOptionsImpl: Sized {
    fn SetEnterpriseId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPrivateNetworkClientServerCapability(&self, value: WebViewControlProcessCapabilityState) -> ::windows::core::Result<()>;
    fn PrivateNetworkClientServerCapability(&self) -> ::windows::core::Result<WebViewControlProcessCapabilityState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlSiteImpl: Sized {
    fn Process(&self) -> ::windows::core::Result<WebViewControlProcess>;
    fn SetScale(&self, value: f64) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<f64>;
    fn SetBounds(&self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn Bounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn MoveFocus(&self, reason: WebViewControlMoveFocusReason) -> ::windows::core::Result<()>;
    fn MoveFocusRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlMoveFocusRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMoveFocusRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AcceleratorKeyPressed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlAcceleratorKeyPressedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAcceleratorKeyPressed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlSite2Impl: Sized {
    fn GotFocus(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
