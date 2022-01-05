#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverImpl: Sized {
    fn GetDefaultSettings(&self) -> ::windows::core::Result<MiracastReceiverSettings>;
    fn GetCurrentSettings(&self) -> ::windows::core::Result<MiracastReceiverSettings>;
    fn GetCurrentSettingsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSettings>>;
    fn DisconnectAllAndApplySettings(&self, settings: &::core::option::Option<MiracastReceiverSettings>) -> ::windows::core::Result<MiracastReceiverApplySettingsResult>;
    fn DisconnectAllAndApplySettingsAsync(&self, settings: &::core::option::Option<MiracastReceiverSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverApplySettingsResult>>;
    fn GetStatus(&self) -> ::windows::core::Result<MiracastReceiverStatus>;
    fn GetStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverStatus>>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiver, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateSession(&self, view: &::core::option::Option<super::super::ApplicationModel::Core::CoreApplicationView>) -> ::windows::core::Result<MiracastReceiverSession>;
    fn CreateSessionAsync(&self, view: &::core::option::Option<super::super::ApplicationModel::Core::CoreApplicationView>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSession>>;
    fn ClearKnownTransmitters(&self) -> ::windows::core::Result<()>;
    fn RemoveKnownTransmitter(&self, transmitter: &::core::option::Option<MiracastTransmitter>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverApplySettingsResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MiracastReceiverApplySettingsStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverConnectionImpl: Sized {
    fn Disconnect(&self, reason: MiracastReceiverDisconnectReason) -> ::windows::core::Result<()>;
    fn DisconnectWithMessage(&self, reason: MiracastReceiverDisconnectReason, message: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn PauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn ResumeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Transmitter(&self) -> ::windows::core::Result<MiracastTransmitter>;
    fn InputDevices(&self) -> ::windows::core::Result<MiracastReceiverInputDevices>;
    fn CursorImageChannel(&self) -> ::windows::core::Result<MiracastReceiverCursorImageChannel>;
    fn StreamControl(&self) -> ::windows::core::Result<MiracastReceiverStreamControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverConnectionCreatedEventArgsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<MiracastReceiverConnection>;
    fn Pin(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverCursorImageChannelImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn MaxImageSize(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn Position(&self) -> ::windows::core::Result<super::super::Graphics::PointInt32>;
    fn ImageStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn ImageStreamChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveImageStreamChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PositionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverCursorImageChannelSettingsImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxImageSize(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn SetMaxImageSize(&self, value: &super::super::Graphics::SizeInt32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverDisconnectedEventArgsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<MiracastReceiverConnection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverGameControllerDeviceImpl: Sized {
    fn TransmitInput(&self) -> ::windows::core::Result<bool>;
    fn SetTransmitInput(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRequestedByTransmitter(&self) -> ::windows::core::Result<bool>;
    fn IsTransmittingInput(&self) -> ::windows::core::Result<bool>;
    fn Mode(&self) -> ::windows::core::Result<MiracastReceiverGameControllerDeviceUsageMode>;
    fn SetMode(&self, value: MiracastReceiverGameControllerDeviceUsageMode) -> ::windows::core::Result<()>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverGameControllerDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverInputDevicesImpl: Sized {
    fn Keyboard(&self) -> ::windows::core::Result<MiracastReceiverKeyboardDevice>;
    fn GameController(&self) -> ::windows::core::Result<MiracastReceiverGameControllerDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverKeyboardDeviceImpl: Sized {
    fn TransmitInput(&self) -> ::windows::core::Result<bool>;
    fn SetTransmitInput(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRequestedByTransmitter(&self) -> ::windows::core::Result<bool>;
    fn IsTransmittingInput(&self) -> ::windows::core::Result<bool>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverKeyboardDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverMediaSourceCreatedEventArgsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<MiracastReceiverConnection>;
    fn MediaSource(&self) -> ::windows::core::Result<super::Core::MediaSource>;
    fn CursorImageChannelSettings(&self) -> ::windows::core::Result<MiracastReceiverCursorImageChannelSettings>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverSessionImpl: Sized {
    fn ConnectionCreated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverConnectionCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionCreated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaSourceCreated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverMediaSourceCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaSourceCreated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Disconnected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverDisconnectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisconnected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AllowConnectionTakeover(&self) -> ::windows::core::Result<bool>;
    fn SetAllowConnectionTakeover(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxSimultaneousConnections(&self) -> ::windows::core::Result<i32>;
    fn SetMaxSimultaneousConnections(&self, value: i32) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<MiracastReceiverSessionStartResult>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSessionStartResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverSessionStartResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MiracastReceiverSessionStartStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverSettingsImpl: Sized {
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetModelName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetModelNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AuthorizationMethod(&self) -> ::windows::core::Result<MiracastReceiverAuthorizationMethod>;
    fn SetAuthorizationMethod(&self, value: MiracastReceiverAuthorizationMethod) -> ::windows::core::Result<()>;
    fn RequireAuthorizationFromKnownTransmitters(&self) -> ::windows::core::Result<bool>;
    fn SetRequireAuthorizationFromKnownTransmitters(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverStatusImpl: Sized {
    fn ListeningStatus(&self) -> ::windows::core::Result<MiracastReceiverListeningStatus>;
    fn WiFiStatus(&self) -> ::windows::core::Result<MiracastReceiverWiFiStatus>;
    fn IsConnectionTakeoverSupported(&self) -> ::windows::core::Result<bool>;
    fn MaxSimultaneousConnections(&self) -> ::windows::core::Result<i32>;
    fn KnownTransmitters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MiracastTransmitter>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverStreamControlImpl: Sized {
    fn GetVideoStreamSettings(&self) -> ::windows::core::Result<MiracastReceiverVideoStreamSettings>;
    fn GetVideoStreamSettingsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverVideoStreamSettings>>;
    fn SuggestVideoStreamSettings(&self, settings: &::core::option::Option<MiracastReceiverVideoStreamSettings>) -> ::windows::core::Result<()>;
    fn SuggestVideoStreamSettingsAsync(&self, settings: &::core::option::Option<MiracastReceiverVideoStreamSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MuteAudio(&self) -> ::windows::core::Result<bool>;
    fn SetMuteAudio(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastReceiverVideoStreamSettingsImpl: Sized {
    fn Size(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn SetSize(&self, value: &super::super::Graphics::SizeInt32) -> ::windows::core::Result<()>;
    fn Bitrate(&self) -> ::windows::core::Result<i32>;
    fn SetBitrate(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMiracastTransmitterImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AuthorizationStatus(&self) -> ::windows::core::Result<MiracastTransmitterAuthorizationStatus>;
    fn SetAuthorizationStatus(&self, value: MiracastTransmitterAuthorizationStatus) -> ::windows::core::Result<()>;
    fn GetConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MiracastReceiverConnection>>;
    fn MacAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LastConnectionTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
