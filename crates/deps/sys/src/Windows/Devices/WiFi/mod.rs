#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWiFiAdapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiAdapter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiAdapterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiAvailableNetwork(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiConnectionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiNetworkReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiWpsConfigurationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiAccessStatus(pub i32);
impl WiFiAccessStatus {
    pub const Unspecified: WiFiAccessStatus = WiFiAccessStatus(0i32);
    pub const Allowed: WiFiAccessStatus = WiFiAccessStatus(1i32);
    pub const DeniedByUser: WiFiAccessStatus = WiFiAccessStatus(2i32);
    pub const DeniedBySystem: WiFiAccessStatus = WiFiAccessStatus(3i32);
}
#[repr(transparent)]
pub struct WiFiAdapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiAvailableNetwork(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiConnectionMethod(pub i32);
impl WiFiConnectionMethod {
    pub const Default: WiFiConnectionMethod = WiFiConnectionMethod(0i32);
    pub const WpsPin: WiFiConnectionMethod = WiFiConnectionMethod(1i32);
    pub const WpsPushButton: WiFiConnectionMethod = WiFiConnectionMethod(2i32);
}
#[repr(transparent)]
pub struct WiFiConnectionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiConnectionStatus(pub i32);
impl WiFiConnectionStatus {
    pub const UnspecifiedFailure: WiFiConnectionStatus = WiFiConnectionStatus(0i32);
    pub const Success: WiFiConnectionStatus = WiFiConnectionStatus(1i32);
    pub const AccessRevoked: WiFiConnectionStatus = WiFiConnectionStatus(2i32);
    pub const InvalidCredential: WiFiConnectionStatus = WiFiConnectionStatus(3i32);
    pub const NetworkNotAvailable: WiFiConnectionStatus = WiFiConnectionStatus(4i32);
    pub const Timeout: WiFiConnectionStatus = WiFiConnectionStatus(5i32);
    pub const UnsupportedAuthenticationProtocol: WiFiConnectionStatus = WiFiConnectionStatus(6i32);
}
#[repr(transparent)]
pub struct WiFiNetworkKind(pub i32);
impl WiFiNetworkKind {
    pub const Any: WiFiNetworkKind = WiFiNetworkKind(0i32);
    pub const Infrastructure: WiFiNetworkKind = WiFiNetworkKind(1i32);
    pub const Adhoc: WiFiNetworkKind = WiFiNetworkKind(2i32);
}
#[repr(transparent)]
pub struct WiFiNetworkReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiPhyKind(pub i32);
impl WiFiPhyKind {
    pub const Unknown: WiFiPhyKind = WiFiPhyKind(0i32);
    pub const Fhss: WiFiPhyKind = WiFiPhyKind(1i32);
    pub const Dsss: WiFiPhyKind = WiFiPhyKind(2i32);
    pub const IRBaseband: WiFiPhyKind = WiFiPhyKind(3i32);
    pub const Ofdm: WiFiPhyKind = WiFiPhyKind(4i32);
    pub const Hrdsss: WiFiPhyKind = WiFiPhyKind(5i32);
    pub const Erp: WiFiPhyKind = WiFiPhyKind(6i32);
    pub const HT: WiFiPhyKind = WiFiPhyKind(7i32);
    pub const Vht: WiFiPhyKind = WiFiPhyKind(8i32);
    pub const Dmg: WiFiPhyKind = WiFiPhyKind(9i32);
    pub const HE: WiFiPhyKind = WiFiPhyKind(10i32);
}
#[repr(transparent)]
pub struct WiFiReconnectionKind(pub i32);
impl WiFiReconnectionKind {
    pub const Automatic: WiFiReconnectionKind = WiFiReconnectionKind(0i32);
    pub const Manual: WiFiReconnectionKind = WiFiReconnectionKind(1i32);
}
#[repr(transparent)]
pub struct WiFiWpsConfigurationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiWpsConfigurationStatus(pub i32);
impl WiFiWpsConfigurationStatus {
    pub const UnspecifiedFailure: WiFiWpsConfigurationStatus = WiFiWpsConfigurationStatus(0i32);
    pub const Success: WiFiWpsConfigurationStatus = WiFiWpsConfigurationStatus(1i32);
    pub const Timeout: WiFiWpsConfigurationStatus = WiFiWpsConfigurationStatus(2i32);
}
#[repr(transparent)]
pub struct WiFiWpsKind(pub i32);
impl WiFiWpsKind {
    pub const Unknown: WiFiWpsKind = WiFiWpsKind(0i32);
    pub const Pin: WiFiWpsKind = WiFiWpsKind(1i32);
    pub const PushButton: WiFiWpsKind = WiFiWpsKind(2i32);
    pub const Nfc: WiFiWpsKind = WiFiWpsKind(3i32);
    pub const Ethernet: WiFiWpsKind = WiFiWpsKind(4i32);
    pub const Usb: WiFiWpsKind = WiFiWpsKind(5i32);
}
