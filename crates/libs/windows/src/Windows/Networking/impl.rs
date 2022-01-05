#[cfg(feature = "implement_exclusive")]
pub trait IEndpointPairImpl: Sized {
    fn LocalHostName(&self) -> ::windows::core::Result<HostName>;
    fn SetLocalHostName(&self, value: &::core::option::Option<HostName>) -> ::windows::core::Result<()>;
    fn LocalServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocalServiceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoteHostName(&self) -> ::windows::core::Result<HostName>;
    fn SetRemoteHostName(&self, value: &::core::option::Option<HostName>) -> ::windows::core::Result<()>;
    fn RemoteServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteServiceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEndpointPairFactoryImpl: Sized {
    fn CreateEndpointPair(&self, localhostname: &::core::option::Option<HostName>, localservicename: &::windows::core::HSTRING, remotehostname: &::core::option::Option<HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<EndpointPair>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHostNameImpl: Sized {
    fn IPInformation(&self) -> ::windows::core::Result<Connectivity::IPInformation>;
    fn RawName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanonicalName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Type(&self) -> ::windows::core::Result<HostNameType>;
    fn IsEqual(&self, hostname: &::core::option::Option<HostName>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHostNameFactoryImpl: Sized {
    fn CreateHostName(&self, hostname: &::windows::core::HSTRING) -> ::windows::core::Result<HostName>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHostNameStaticsImpl: Sized {
    fn Compare(&self, value1: &::windows::core::HSTRING, value2: &::windows::core::HSTRING) -> ::windows::core::Result<i32>;
}
