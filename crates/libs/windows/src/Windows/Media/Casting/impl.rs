#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICastingConnectionImpl: Sized + IClosableImpl {
    fn State(&self) -> ::windows::core::Result<CastingConnectionState>;
    fn Device(&self) -> ::windows::core::Result<CastingDevice>;
    fn Source(&self) -> ::windows::core::Result<CastingSource>;
    fn SetSource(&self, value: &::core::option::Option<CastingSource>) -> ::windows::core::Result<()>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CastingConnection, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CastingConnection, CastingConnectionErrorOccurredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestStartCastingAsync(&self, value: &::core::option::Option<CastingSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>>;
    fn DisconnectAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICastingConnectionErrorOccurredEventArgsImpl: Sized {
    fn ErrorStatus(&self) -> ::windows::core::Result<CastingConnectionErrorStatus>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICastingDeviceImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Icon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn GetSupportedCastingPlaybackTypesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingPlaybackTypes>>;
    fn CreateCastingConnection(&self) -> ::windows::core::Result<CastingConnection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICastingDevicePickerImpl: Sized {
    fn Filter(&self) -> ::windows::core::Result<CastingDevicePickerFilter>;
    fn Appearance(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DevicePickerAppearance>;
    fn CastingDeviceSelected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CastingDevicePicker, CastingDeviceSelectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCastingDeviceSelected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CastingDevicePickerDismissed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CastingDevicePicker, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCastingDevicePickerDismissed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Show(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn ShowWithPlacement(&self, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()>;
    fn Hide(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICastingDevicePickerFilterImpl: Sized {
    fn SupportsAudio(&self) -> ::windows::core::Result<bool>;
    fn SetSupportsAudio(&self, value: bool) -> ::windows::core::Result<()>;
    fn SupportsVideo(&self) -> ::windows::core::Result<bool>;
    fn SetSupportsVideo(&self, value: bool) -> ::windows::core::Result<()>;
    fn SupportsPictures(&self) -> ::windows::core::Result<bool>;
    fn SetSupportsPictures(&self, value: bool) -> ::windows::core::Result<()>;
    fn SupportedCastingSources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<CastingSource>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICastingDeviceSelectedEventArgsImpl: Sized {
    fn SelectedCastingDevice(&self) -> ::windows::core::Result<CastingDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICastingDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self, r#type: CastingPlaybackTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromCastingSourceAsync(&self, castingsource: &::core::option::Option<CastingSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn FromIdAsync(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingDevice>>;
    fn DeviceInfoSupportsCastingAsync(&self, device: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICastingSourceImpl: Sized {
    fn PreferredSourceUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetPreferredSourceUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
