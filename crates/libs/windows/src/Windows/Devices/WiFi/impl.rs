#[cfg(feature = "implement_exclusive")]
pub trait IWiFiAdapterImpl: Sized {
    fn NetworkAdapter(&self) -> ::windows::core::Result<super::super::Networking::Connectivity::NetworkAdapter>;
    fn ScanAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn NetworkReport(&self) -> ::windows::core::Result<WiFiNetworkReport>;
    fn AvailableNetworksChanged(&self, args: &::core::option::Option<super::super::Foundation::TypedEventHandler<WiFiAdapter, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailableNetworksChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectAsync(&self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>, reconnectionkind: WiFiReconnectionKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>;
    fn ConnectWithPasswordCredentialAsync(&self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>, reconnectionkind: WiFiReconnectionKind, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>;
    fn ConnectWithPasswordCredentialAndSsidAsync(&self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>, reconnectionkind: WiFiReconnectionKind, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>, ssid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiAdapter2Impl: Sized {
    fn GetWpsConfigurationAsync(&self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiWpsConfigurationResult>>;
    fn ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync(&self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>, reconnectionkind: WiFiReconnectionKind, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>, ssid: &::windows::core::HSTRING, connectionmethod: WiFiConnectionMethod) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiAdapterStaticsImpl: Sized {
    fn FindAllAdaptersAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WiFiAdapter>>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiAdapter>>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiAvailableNetworkImpl: Sized {
    fn Uptime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Ssid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Bssid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ChannelCenterFrequencyInKilohertz(&self) -> ::windows::core::Result<i32>;
    fn NetworkRssiInDecibelMilliwatts(&self) -> ::windows::core::Result<f64>;
    fn SignalBars(&self) -> ::windows::core::Result<u8>;
    fn NetworkKind(&self) -> ::windows::core::Result<WiFiNetworkKind>;
    fn PhyKind(&self) -> ::windows::core::Result<WiFiPhyKind>;
    fn SecuritySettings(&self) -> ::windows::core::Result<super::super::Networking::Connectivity::NetworkSecuritySettings>;
    fn BeaconInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IsWiFiDirect(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiConnectionResultImpl: Sized {
    fn ConnectionStatus(&self) -> ::windows::core::Result<WiFiConnectionStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiNetworkReportImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AvailableNetworks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WiFiAvailableNetwork>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiWpsConfigurationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<WiFiWpsConfigurationStatus>;
    fn SupportedWpsKinds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WiFiWpsKind>>;
}
