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
    pub const None: WiFiDirectAdvertisementListenStateDiscoverability = WiFiDirectAdvertisementListenStateDiscoverability(0i32);
    pub const Normal: WiFiDirectAdvertisementListenStateDiscoverability = WiFiDirectAdvertisementListenStateDiscoverability(1i32);
    pub const Intensive: WiFiDirectAdvertisementListenStateDiscoverability = WiFiDirectAdvertisementListenStateDiscoverability(2i32);
}
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisherStatus(pub i32);
impl WiFiDirectAdvertisementPublisherStatus {
    pub const Created: WiFiDirectAdvertisementPublisherStatus = WiFiDirectAdvertisementPublisherStatus(0i32);
    pub const Started: WiFiDirectAdvertisementPublisherStatus = WiFiDirectAdvertisementPublisherStatus(1i32);
    pub const Stopped: WiFiDirectAdvertisementPublisherStatus = WiFiDirectAdvertisementPublisherStatus(2i32);
    pub const Aborted: WiFiDirectAdvertisementPublisherStatus = WiFiDirectAdvertisementPublisherStatus(3i32);
}
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisherStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectConfigurationMethod(pub i32);
impl WiFiDirectConfigurationMethod {
    pub const ProvidePin: WiFiDirectConfigurationMethod = WiFiDirectConfigurationMethod(0i32);
    pub const DisplayPin: WiFiDirectConfigurationMethod = WiFiDirectConfigurationMethod(1i32);
    pub const PushButton: WiFiDirectConfigurationMethod = WiFiDirectConfigurationMethod(2i32);
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
    pub const Disconnected: WiFiDirectConnectionStatus = WiFiDirectConnectionStatus(0i32);
    pub const Connected: WiFiDirectConnectionStatus = WiFiDirectConnectionStatus(1i32);
}
#[repr(transparent)]
pub struct WiFiDirectDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectDeviceSelectorType(pub i32);
impl WiFiDirectDeviceSelectorType {
    pub const DeviceInterface: WiFiDirectDeviceSelectorType = WiFiDirectDeviceSelectorType(0i32);
    pub const AssociationEndpoint: WiFiDirectDeviceSelectorType = WiFiDirectDeviceSelectorType(1i32);
}
#[repr(transparent)]
pub struct WiFiDirectError(pub i32);
impl WiFiDirectError {
    pub const Success: WiFiDirectError = WiFiDirectError(0i32);
    pub const RadioNotAvailable: WiFiDirectError = WiFiDirectError(1i32);
    pub const ResourceInUse: WiFiDirectError = WiFiDirectError(2i32);
}
#[repr(transparent)]
pub struct WiFiDirectInformationElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectLegacySettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectPairingProcedure(pub i32);
impl WiFiDirectPairingProcedure {
    pub const GroupOwnerNegotiation: WiFiDirectPairingProcedure = WiFiDirectPairingProcedure(0i32);
    pub const Invitation: WiFiDirectPairingProcedure = WiFiDirectPairingProcedure(1i32);
}
