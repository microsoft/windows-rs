#[cfg(feature = "implement_exclusive")]
pub trait IDialAppImpl: Sized {
    fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RequestLaunchAsync(&self, appargument: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DialAppLaunchResult>>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DialAppStopResult>>;
    fn GetAppStateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DialAppStateDetails>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialAppStateDetailsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<DialAppState>;
    fn FullXml(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialDeviceImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDialApp(&self, appname: &::windows::core::HSTRING) -> ::windows::core::Result<DialApp>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialDevice2Impl: Sized {
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialDevicePickerImpl: Sized {
    fn Filter(&self) -> ::windows::core::Result<DialDevicePickerFilter>;
    fn Appearance(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DevicePickerAppearance>;
    fn DialDeviceSelected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DialDevicePicker, DialDeviceSelectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDialDeviceSelected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DisconnectButtonClicked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DialDevicePicker, DialDisconnectButtonClickedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisconnectButtonClicked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DialDevicePickerDismissed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DialDevicePicker, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDialDevicePickerDismissed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Show(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn ShowWithPlacement(&self, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()>;
    fn PickSingleDialDeviceAsync(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DialDevice>>;
    fn PickSingleDialDeviceAsyncWithPlacement(&self, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DialDevice>>;
    fn Hide(&self) -> ::windows::core::Result<()>;
    fn SetDisplayStatus(&self, device: &::core::option::Option<DialDevice>, status: DialDeviceDisplayStatus) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialDevicePickerFilterImpl: Sized {
    fn SupportedAppNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialDeviceSelectedEventArgsImpl: Sized {
    fn SelectedDialDevice(&self) -> ::windows::core::Result<DialDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self, appname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DialDevice>>;
    fn DeviceInfoSupportsDialAsync(&self, device: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialDisconnectButtonClickedEventArgsImpl: Sized {
    fn Device(&self) -> ::windows::core::Result<DialDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialReceiverAppImpl: Sized {
    fn GetAdditionalDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>>;
    fn SetAdditionalDataAsync(&self, additionaldata: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialReceiverApp2Impl: Sized {
    fn GetUniqueDeviceNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialReceiverAppStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<DialReceiverApp>;
}
