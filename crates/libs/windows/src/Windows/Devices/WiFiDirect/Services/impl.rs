#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceImpl: Sized {
    fn RemoteServiceInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SupportedConfigurationMethods(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<WiFiDirectServiceConfigurationMethod>>;
    fn PreferGroupOwnerMode(&self) -> ::windows::core::Result<bool>;
    fn SetPreferGroupOwnerMode(&self, value: bool) -> ::windows::core::Result<()>;
    fn SessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetSessionInfo(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ServiceError(&self) -> ::windows::core::Result<WiFiDirectServiceError>;
    fn SessionDeferred(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectService, WiFiDirectServiceSessionDeferredEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionDeferred(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetProvisioningInfoAsync(&self, selectedconfigurationmethod: WiFiDirectServiceConfigurationMethod) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceProvisioningInfo>>;
    fn ConnectAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>;
    fn ConnectAsyncWithPin(&self, pin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceAdvertiserImpl: Sized {
    fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceNamePrefixes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ServiceInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetServiceInfo(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn AutoAcceptSession(&self) -> ::windows::core::Result<bool>;
    fn SetAutoAcceptSession(&self, value: bool) -> ::windows::core::Result<()>;
    fn PreferGroupOwnerMode(&self) -> ::windows::core::Result<bool>;
    fn SetPreferGroupOwnerMode(&self, value: bool) -> ::windows::core::Result<()>;
    fn PreferredConfigurationMethods(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<WiFiDirectServiceConfigurationMethod>>;
    fn ServiceStatus(&self) -> ::windows::core::Result<WiFiDirectServiceStatus>;
    fn SetServiceStatus(&self, value: WiFiDirectServiceStatus) -> ::windows::core::Result<()>;
    fn CustomServiceStatusCode(&self) -> ::windows::core::Result<u32>;
    fn SetCustomServiceStatusCode(&self, value: u32) -> ::windows::core::Result<()>;
    fn DeferredSessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetDeferredSessionInfo(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn AdvertisementStatus(&self) -> ::windows::core::Result<WiFiDirectServiceAdvertisementStatus>;
    fn ServiceError(&self) -> ::windows::core::Result<WiFiDirectServiceError>;
    fn SessionRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceSessionRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutoAcceptSessionConnected(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceAutoAcceptSessionConnectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAutoAcceptSessionConnected(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AdvertisementStatusChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdvertisementStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectAsync(&self, deviceinfo: &::core::option::Option<super::super::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>;
    fn ConnectAsyncWithPin(&self, deviceinfo: &::core::option::Option<super::super::Enumeration::DeviceInformation>, pin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceAdvertiserFactoryImpl: Sized {
    fn CreateWiFiDirectServiceAdvertiser(&self, servicename: &::windows::core::HSTRING) -> ::windows::core::Result<WiFiDirectServiceAdvertiser>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceAutoAcceptSessionConnectedEventArgsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<WiFiDirectServiceSession>;
    fn SessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceProvisioningInfoImpl: Sized {
    fn SelectedConfigurationMethod(&self) -> ::windows::core::Result<WiFiDirectServiceConfigurationMethod>;
    fn IsGroupFormationNeeded(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceRemotePortAddedEventArgsImpl: Sized {
    fn EndpointPairs(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>>;
    fn Protocol(&self) -> ::windows::core::Result<WiFiDirectServiceIPProtocol>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWiFiDirectServiceSessionImpl: Sized + IClosableImpl {
    fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&self) -> ::windows::core::Result<WiFiDirectServiceSessionStatus>;
    fn ErrorStatus(&self) -> ::windows::core::Result<WiFiDirectServiceSessionErrorStatus>;
    fn SessionId(&self) -> ::windows::core::Result<u32>;
    fn AdvertisementId(&self) -> ::windows::core::Result<u32>;
    fn ServiceAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetConnectionEndpointPairs(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>>;
    fn SessionStatusChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AddStreamSocketListenerAsync(&self, value: &::core::option::Option<super::super::super::Networking::Sockets::StreamSocketListener>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn AddDatagramSocketAsync(&self, value: &::core::option::Option<super::super::super::Networking::Sockets::DatagramSocket>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn RemotePortAdded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, WiFiDirectServiceRemotePortAddedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemotePortAdded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceSessionDeferredEventArgsImpl: Sized {
    fn DeferredSessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWiFiDirectServiceSessionRequestImpl: Sized + IClosableImpl {
    fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Enumeration::DeviceInformation>;
    fn ProvisioningInfo(&self) -> ::windows::core::Result<WiFiDirectServiceProvisioningInfo>;
    fn SessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceSessionRequestedEventArgsImpl: Sized {
    fn GetSessionRequest(&self) -> ::windows::core::Result<WiFiDirectServiceSessionRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceStaticsImpl: Sized {
    fn GetSelector(&self, servicename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetSelectorWithFilter(&self, servicename: &::windows::core::HSTRING, serviceinfofilter: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectService>>;
}
