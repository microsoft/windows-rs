#[cfg(feature = "implement_exclusive")]
pub trait IConnectionRequestedEventArgsImpl: Sized {
    fn PeerInformation(&self) -> ::windows::core::Result<PeerInformation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPeerFinderStaticsImpl: Sized {
    fn AllowBluetooth(&self) -> ::windows::core::Result<bool>;
    fn SetAllowBluetooth(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowInfrastructure(&self) -> ::windows::core::Result<bool>;
    fn SetAllowInfrastructure(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowWiFiDirect(&self) -> ::windows::core::Result<bool>;
    fn SetAllowWiFiDirect(&self, value: bool) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SupportedDiscoveryTypes(&self) -> ::windows::core::Result<PeerDiscoveryTypes>;
    fn AlternateIdentities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn StartWithMessage(&self, peermessage: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn TriggeredConnectionStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, TriggeredConnectionStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTriggeredConnectionStateChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectionRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ConnectionRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindAllPeersAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PeerInformation>>>;
    fn ConnectAsync(&self, peerinformation: &::core::option::Option<PeerInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Sockets::StreamSocket>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPeerFinderStatics2Impl: Sized {
    fn Role(&self) -> ::windows::core::Result<PeerRole>;
    fn SetRole(&self, value: PeerRole) -> ::windows::core::Result<()>;
    fn DiscoveryData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetDiscoveryData(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn CreateWatcher(&self) -> ::windows::core::Result<PeerWatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPeerInformationImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPeerInformation3Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DiscoveryData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPeerInformationWithHostAndServiceImpl: Sized {
    fn HostName(&self) -> ::windows::core::Result<super::HostName>;
    fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPeerWatcherImpl: Sized {
    fn Added(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<PeerWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximityDeviceImpl: Sized {
    fn SubscribeForMessage(&self, messagetype: &::windows::core::HSTRING, messagereceivedhandler: &::core::option::Option<MessageReceivedHandler>) -> ::windows::core::Result<i64>;
    fn PublishMessage(&self, messagetype: &::windows::core::HSTRING, message: &::windows::core::HSTRING) -> ::windows::core::Result<i64>;
    fn PublishMessageWithCallback(&self, messagetype: &::windows::core::HSTRING, message: &::windows::core::HSTRING, messagetransmittedhandler: &::core::option::Option<MessageTransmittedHandler>) -> ::windows::core::Result<i64>;
    fn PublishBinaryMessage(&self, messagetype: &::windows::core::HSTRING, message: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<i64>;
    fn PublishBinaryMessageWithCallback(&self, messagetype: &::windows::core::HSTRING, message: &::core::option::Option<super::super::Storage::Streams::IBuffer>, messagetransmittedhandler: &::core::option::Option<MessageTransmittedHandler>) -> ::windows::core::Result<i64>;
    fn PublishUriMessage(&self, message: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<i64>;
    fn PublishUriMessageWithCallback(&self, message: &::core::option::Option<super::super::Foundation::Uri>, messagetransmittedhandler: &::core::option::Option<MessageTransmittedHandler>) -> ::windows::core::Result<i64>;
    fn StopSubscribingForMessage(&self, subscriptionid: i64) -> ::windows::core::Result<()>;
    fn StopPublishingMessage(&self, messageid: i64) -> ::windows::core::Result<()>;
    fn DeviceArrived(&self, arrivedhandler: &::core::option::Option<DeviceArrivedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeviceArrived(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeviceDeparted(&self, departedhandler: &::core::option::Option<DeviceDepartedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeviceDeparted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MaxMessageBytes(&self) -> ::windows::core::Result<u32>;
    fn BitsPerSecond(&self) -> ::windows::core::Result<u64>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximityDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDefault(&self) -> ::windows::core::Result<ProximityDevice>;
    fn FromId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<ProximityDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximityMessageImpl: Sized {
    fn MessageType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubscriptionId(&self) -> ::windows::core::Result<i64>;
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn DataAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITriggeredConnectionStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<TriggeredConnectState>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Socket(&self) -> ::windows::core::Result<super::Sockets::StreamSocket>;
}
