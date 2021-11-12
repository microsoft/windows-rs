#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWiFiDirectService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceAdvertiser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceAdvertiserFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceProvisioningInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceRemotePortAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceSessionDeferredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceSessionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceSessionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectServiceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceAdvertisementStatus(pub i32);
impl WiFiDirectServiceAdvertisementStatus {
    pub const Created: WiFiDirectServiceAdvertisementStatus = WiFiDirectServiceAdvertisementStatus(0i32);
    pub const Started: WiFiDirectServiceAdvertisementStatus = WiFiDirectServiceAdvertisementStatus(1i32);
    pub const Stopped: WiFiDirectServiceAdvertisementStatus = WiFiDirectServiceAdvertisementStatus(2i32);
    pub const Aborted: WiFiDirectServiceAdvertisementStatus = WiFiDirectServiceAdvertisementStatus(3i32);
}
#[repr(transparent)]
pub struct WiFiDirectServiceAdvertiser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceAutoAcceptSessionConnectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceConfigurationMethod(pub i32);
impl WiFiDirectServiceConfigurationMethod {
    pub const Default: WiFiDirectServiceConfigurationMethod = WiFiDirectServiceConfigurationMethod(0i32);
    pub const PinDisplay: WiFiDirectServiceConfigurationMethod = WiFiDirectServiceConfigurationMethod(1i32);
    pub const PinEntry: WiFiDirectServiceConfigurationMethod = WiFiDirectServiceConfigurationMethod(2i32);
}
#[repr(transparent)]
pub struct WiFiDirectServiceError(pub i32);
impl WiFiDirectServiceError {
    pub const Success: WiFiDirectServiceError = WiFiDirectServiceError(0i32);
    pub const RadioNotAvailable: WiFiDirectServiceError = WiFiDirectServiceError(1i32);
    pub const ResourceInUse: WiFiDirectServiceError = WiFiDirectServiceError(2i32);
    pub const UnsupportedHardware: WiFiDirectServiceError = WiFiDirectServiceError(3i32);
    pub const NoHardware: WiFiDirectServiceError = WiFiDirectServiceError(4i32);
}
#[repr(transparent)]
pub struct WiFiDirectServiceIPProtocol(pub i32);
impl WiFiDirectServiceIPProtocol {
    pub const Tcp: WiFiDirectServiceIPProtocol = WiFiDirectServiceIPProtocol(6i32);
    pub const Udp: WiFiDirectServiceIPProtocol = WiFiDirectServiceIPProtocol(17i32);
}
#[repr(transparent)]
pub struct WiFiDirectServiceProvisioningInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceRemotePortAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceSessionDeferredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceSessionErrorStatus(pub i32);
impl WiFiDirectServiceSessionErrorStatus {
    pub const Ok: WiFiDirectServiceSessionErrorStatus = WiFiDirectServiceSessionErrorStatus(0i32);
    pub const Disassociated: WiFiDirectServiceSessionErrorStatus = WiFiDirectServiceSessionErrorStatus(1i32);
    pub const LocalClose: WiFiDirectServiceSessionErrorStatus = WiFiDirectServiceSessionErrorStatus(2i32);
    pub const RemoteClose: WiFiDirectServiceSessionErrorStatus = WiFiDirectServiceSessionErrorStatus(3i32);
    pub const SystemFailure: WiFiDirectServiceSessionErrorStatus = WiFiDirectServiceSessionErrorStatus(4i32);
    pub const NoResponseFromRemote: WiFiDirectServiceSessionErrorStatus = WiFiDirectServiceSessionErrorStatus(5i32);
}
#[repr(transparent)]
pub struct WiFiDirectServiceSessionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceSessionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceSessionStatus(pub i32);
impl WiFiDirectServiceSessionStatus {
    pub const Closed: WiFiDirectServiceSessionStatus = WiFiDirectServiceSessionStatus(0i32);
    pub const Initiated: WiFiDirectServiceSessionStatus = WiFiDirectServiceSessionStatus(1i32);
    pub const Requested: WiFiDirectServiceSessionStatus = WiFiDirectServiceSessionStatus(2i32);
    pub const Open: WiFiDirectServiceSessionStatus = WiFiDirectServiceSessionStatus(3i32);
}
#[repr(transparent)]
pub struct WiFiDirectServiceStatus(pub i32);
impl WiFiDirectServiceStatus {
    pub const Available: WiFiDirectServiceStatus = WiFiDirectServiceStatus(0i32);
    pub const Busy: WiFiDirectServiceStatus = WiFiDirectServiceStatus(1i32);
    pub const Custom: WiFiDirectServiceStatus = WiFiDirectServiceStatus(2i32);
}
