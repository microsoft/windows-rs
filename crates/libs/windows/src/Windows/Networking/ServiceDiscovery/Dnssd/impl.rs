#[cfg(feature = "implement_exclusive")]
pub trait IDnssdRegistrationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<DnssdRegistrationStatus>;
    fn IPAddress(&self) -> ::windows::core::Result<super::super::HostName>;
    fn HasInstanceNameChanged(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDnssdServiceInstanceImpl: Sized {
    fn DnssdServiceInstanceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDnssdServiceInstanceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HostName(&self) -> ::windows::core::Result<super::super::HostName>;
    fn SetHostName(&self, value: &::core::option::Option<super::super::HostName>) -> ::windows::core::Result<()>;
    fn Port(&self) -> ::windows::core::Result<u16>;
    fn SetPort(&self, value: u16) -> ::windows::core::Result<()>;
    fn Priority(&self) -> ::windows::core::Result<u16>;
    fn SetPriority(&self, value: u16) -> ::windows::core::Result<()>;
    fn Weight(&self) -> ::windows::core::Result<u16>;
    fn SetWeight(&self, value: u16) -> ::windows::core::Result<()>;
    fn TextAttributes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn RegisterStreamSocketListenerAsync1(&self, socket: &::core::option::Option<super::super::Sockets::StreamSocketListener>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>;
    fn RegisterStreamSocketListenerAsync2(&self, socket: &::core::option::Option<super::super::Sockets::StreamSocketListener>, adapter: &::core::option::Option<super::super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>;
    fn RegisterDatagramSocketAsync1(&self, socket: &::core::option::Option<super::super::Sockets::DatagramSocket>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>;
    fn RegisterDatagramSocketAsync2(&self, socket: &::core::option::Option<super::super::Sockets::DatagramSocket>, adapter: &::core::option::Option<super::super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDnssdServiceInstanceFactoryImpl: Sized {
    fn Create(&self, dnssdserviceinstancename: &::windows::core::HSTRING, hostname: &::core::option::Option<super::super::HostName>, port: u16) -> ::windows::core::Result<DnssdServiceInstance>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDnssdServiceWatcherImpl: Sized {
    fn Added(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, DnssdServiceInstance>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<DnssdServiceWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
