#![allow(non_snake_case, non_camel_case_types)]
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
pub struct WiFiDirectAdvertisementListenStateDiscoverability(i32);
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisher(pub *mut ::core::ffi::c_void);
pub struct WiFiDirectAdvertisementPublisherStatus(i32);
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisherStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct WiFiDirectConfigurationMethod(i32);
#[repr(transparent)]
pub struct WiFiDirectConnectionListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectConnectionParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectConnectionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct WiFiDirectConnectionStatus(i32);
#[repr(transparent)]
pub struct WiFiDirectDevice(pub *mut ::core::ffi::c_void);
pub struct WiFiDirectDeviceSelectorType(i32);
pub struct WiFiDirectError(i32);
#[repr(transparent)]
pub struct WiFiDirectInformationElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectLegacySettings(pub *mut ::core::ffi::c_void);
pub struct WiFiDirectPairingProcedure(i32);
