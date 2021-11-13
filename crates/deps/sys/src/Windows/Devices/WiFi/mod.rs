#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWiFiAdapter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiAdapter {}
impl ::core::clone::Clone for IWiFiAdapter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiAdapter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiAdapter2 {}
impl ::core::clone::Clone for IWiFiAdapter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiAdapterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiAdapterStatics {}
impl ::core::clone::Clone for IWiFiAdapterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiAvailableNetwork(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiAvailableNetwork {}
impl ::core::clone::Clone for IWiFiAvailableNetwork {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiConnectionResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiConnectionResult {}
impl ::core::clone::Clone for IWiFiConnectionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiNetworkReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiNetworkReport {}
impl ::core::clone::Clone for IWiFiNetworkReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiWpsConfigurationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiWpsConfigurationResult {}
impl ::core::clone::Clone for IWiFiWpsConfigurationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiAccessStatus(pub i32);
impl WiFiAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiAccessStatus {}
impl ::core::clone::Clone for WiFiAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiAdapter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiAdapter {}
impl ::core::clone::Clone for WiFiAdapter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiAvailableNetwork(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiAvailableNetwork {}
impl ::core::clone::Clone for WiFiAvailableNetwork {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiConnectionMethod(pub i32);
impl WiFiConnectionMethod {
    pub const Default: Self = Self(0i32);
    pub const WpsPin: Self = Self(1i32);
    pub const WpsPushButton: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiConnectionMethod {}
impl ::core::clone::Clone for WiFiConnectionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiConnectionResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiConnectionResult {}
impl ::core::clone::Clone for WiFiConnectionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiConnectionStatus(pub i32);
impl WiFiConnectionStatus {
    pub const UnspecifiedFailure: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AccessRevoked: Self = Self(2i32);
    pub const InvalidCredential: Self = Self(3i32);
    pub const NetworkNotAvailable: Self = Self(4i32);
    pub const Timeout: Self = Self(5i32);
    pub const UnsupportedAuthenticationProtocol: Self = Self(6i32);
}
impl ::core::marker::Copy for WiFiConnectionStatus {}
impl ::core::clone::Clone for WiFiConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiNetworkKind(pub i32);
impl WiFiNetworkKind {
    pub const Any: Self = Self(0i32);
    pub const Infrastructure: Self = Self(1i32);
    pub const Adhoc: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiNetworkKind {}
impl ::core::clone::Clone for WiFiNetworkKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiNetworkReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiNetworkReport {}
impl ::core::clone::Clone for WiFiNetworkReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiPhyKind(pub i32);
impl WiFiPhyKind {
    pub const Unknown: Self = Self(0i32);
    pub const Fhss: Self = Self(1i32);
    pub const Dsss: Self = Self(2i32);
    pub const IRBaseband: Self = Self(3i32);
    pub const Ofdm: Self = Self(4i32);
    pub const Hrdsss: Self = Self(5i32);
    pub const Erp: Self = Self(6i32);
    pub const HT: Self = Self(7i32);
    pub const Vht: Self = Self(8i32);
    pub const Dmg: Self = Self(9i32);
    pub const HE: Self = Self(10i32);
}
impl ::core::marker::Copy for WiFiPhyKind {}
impl ::core::clone::Clone for WiFiPhyKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiReconnectionKind(pub i32);
impl WiFiReconnectionKind {
    pub const Automatic: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiReconnectionKind {}
impl ::core::clone::Clone for WiFiReconnectionKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiWpsConfigurationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiWpsConfigurationResult {}
impl ::core::clone::Clone for WiFiWpsConfigurationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiWpsConfigurationStatus(pub i32);
impl WiFiWpsConfigurationStatus {
    pub const UnspecifiedFailure: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const Timeout: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiWpsConfigurationStatus {}
impl ::core::clone::Clone for WiFiWpsConfigurationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiWpsKind(pub i32);
impl WiFiWpsKind {
    pub const Unknown: Self = Self(0i32);
    pub const Pin: Self = Self(1i32);
    pub const PushButton: Self = Self(2i32);
    pub const Nfc: Self = Self(3i32);
    pub const Ethernet: Self = Self(4i32);
    pub const Usb: Self = Self(5i32);
}
impl ::core::marker::Copy for WiFiWpsKind {}
impl ::core::clone::Clone for WiFiWpsKind {
    fn clone(&self) -> Self {
        *self
    }
}
