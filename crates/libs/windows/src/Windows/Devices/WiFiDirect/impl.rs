#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectAdvertisementImpl: Sized {
    fn InformationElements(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>;
    fn SetInformationElements(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>) -> ::windows::core::Result<()>;
    fn ListenStateDiscoverability(&self) -> ::windows::core::Result<WiFiDirectAdvertisementListenStateDiscoverability>;
    fn SetListenStateDiscoverability(&self, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::core::Result<()>;
    fn IsAutonomousGroupOwnerEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsAutonomousGroupOwnerEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn LegacySettings(&self) -> ::windows::core::Result<WiFiDirectLegacySettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectAdvertisement2Impl: Sized {
    fn SupportedConfigurationMethods(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectAdvertisementPublisherImpl: Sized {
    fn Advertisement(&self) -> ::windows::core::Result<WiFiDirectAdvertisement>;
    fn Status(&self) -> ::windows::core::Result<WiFiDirectAdvertisementPublisherStatus>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WiFiDirectAdvertisementPublisher, WiFiDirectAdvertisementPublisherStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectAdvertisementPublisherStatusChangedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<WiFiDirectAdvertisementPublisherStatus>;
    fn Error(&self) -> ::windows::core::Result<WiFiDirectError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectConnectionListenerImpl: Sized {
    fn ConnectionRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WiFiDirectConnectionListener, WiFiDirectConnectionRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectConnectionParametersImpl: Sized {
    fn GroupOwnerIntent(&self) -> ::windows::core::Result<i16>;
    fn SetGroupOwnerIntent(&self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectConnectionParameters2Impl: Sized {
    fn PreferenceOrderedConfigurationMethods(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>;
    fn PreferredPairingProcedure(&self) -> ::windows::core::Result<WiFiDirectPairingProcedure>;
    fn SetPreferredPairingProcedure(&self, value: WiFiDirectPairingProcedure) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectConnectionParametersStaticsImpl: Sized {
    fn GetDevicePairingKinds(&self, configurationmethod: WiFiDirectConfigurationMethod) -> ::windows::core::Result<super::Enumeration::DevicePairingKinds>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWiFiDirectConnectionRequestImpl: Sized + IClosableImpl {
    fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectConnectionRequestedEventArgsImpl: Sized {
    fn GetConnectionRequest(&self) -> ::windows::core::Result<WiFiDirectConnectionRequest>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWiFiDirectDeviceImpl: Sized + IClosableImpl {
    fn ConnectionStatus(&self) -> ::windows::core::Result<WiFiDirectConnectionStatus>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WiFiDirectDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetConnectionEndpointPairs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::EndpointPair>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectDeviceStatics2Impl: Sized {
    fn GetDeviceSelector(&self, r#type: WiFiDirectDeviceSelectorType) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING, connectionparameters: &::core::option::Option<WiFiDirectConnectionParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectInformationElementImpl: Sized {
    fn Oui(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetOui(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn OuiType(&self) -> ::windows::core::Result<u8>;
    fn SetOuiType(&self, value: u8) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetValue(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectInformationElementStaticsImpl: Sized {
    fn CreateFromBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>;
    fn CreateFromDeviceInformation(&self, deviceinformation: &::core::option::Option<super::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectLegacySettingsImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Ssid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSsid(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Passphrase(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetPassphrase(&self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
}
