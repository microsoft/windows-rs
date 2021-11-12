#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_WiFiDirect_Services")]
pub mod Services;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWiFiDirectAdvertisement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectAdvertisement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectAdvertisementPublisher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectConnectionListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectConnectionParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectConnectionParameters2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectConnectionParametersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectConnectionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectDeviceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectInformationElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectInformationElementStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWiFiDirectLegacySettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectAdvertisement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectAdvertisementListenStateDiscoverability(pub i32);
impl WiFiDirectAdvertisementListenStateDiscoverability {
    pub const None: Self = Self(0i32);
    pub const Normal: Self = Self(1i32);
    pub const Intensive: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectAdvertisementListenStateDiscoverability {}
impl ::core::clone::Clone for WiFiDirectAdvertisementListenStateDiscoverability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisherStatus(pub i32);
impl WiFiDirectAdvertisementPublisherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiDirectAdvertisementPublisherStatus {}
impl ::core::clone::Clone for WiFiDirectAdvertisementPublisherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisherStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectConfigurationMethod(pub i32);
impl WiFiDirectConfigurationMethod {
    pub const ProvidePin: Self = Self(0i32);
    pub const DisplayPin: Self = Self(1i32);
    pub const PushButton: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectConfigurationMethod {}
impl ::core::clone::Clone for WiFiDirectConfigurationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectConnectionListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectConnectionParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectConnectionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectConnectionStatus(pub i32);
impl WiFiDirectConnectionStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiDirectConnectionStatus {}
impl ::core::clone::Clone for WiFiDirectConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectDeviceSelectorType(pub i32);
impl WiFiDirectDeviceSelectorType {
    pub const DeviceInterface: Self = Self(0i32);
    pub const AssociationEndpoint: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiDirectDeviceSelectorType {}
impl ::core::clone::Clone for WiFiDirectDeviceSelectorType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectError(pub i32);
impl WiFiDirectError {
    pub const Success: Self = Self(0i32);
    pub const RadioNotAvailable: Self = Self(1i32);
    pub const ResourceInUse: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectError {}
impl ::core::clone::Clone for WiFiDirectError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectInformationElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectLegacySettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectPairingProcedure(pub i32);
impl WiFiDirectPairingProcedure {
    pub const GroupOwnerNegotiation: Self = Self(0i32);
    pub const Invitation: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiDirectPairingProcedure {}
impl ::core::clone::Clone for WiFiDirectPairingProcedure {
    fn clone(&self) -> Self {
        *self
    }
}
