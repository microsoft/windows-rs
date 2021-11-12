#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_WiFiDirect_Services")]
pub mod Services;
#[link(name = "windows")]
extern "system" {}
pub struct IWiFiDirectAdvertisement(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectAdvertisement2(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectAdvertisementPublisher(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectConnectionListener(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectConnectionParameters(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectConnectionParameters2(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectConnectionParametersStatics(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectConnectionRequest(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectDevice(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectDeviceStatics2(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectInformationElement(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectInformationElementStatics(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectLegacySettings(pub *mut ::core::ffi::c_void);
pub struct WiFiDirectAdvertisement(i32);
pub struct WiFiDirectAdvertisementListenStateDiscoverability(i32);
pub struct WiFiDirectAdvertisementPublisher(i32);
pub struct WiFiDirectAdvertisementPublisherStatus(i32);
pub struct WiFiDirectAdvertisementPublisherStatusChangedEventArgs(i32);
pub struct WiFiDirectConfigurationMethod(i32);
pub struct WiFiDirectConnectionListener(i32);
pub struct WiFiDirectConnectionParameters(i32);
pub struct WiFiDirectConnectionRequest(i32);
pub struct WiFiDirectConnectionRequestedEventArgs(i32);
pub struct WiFiDirectConnectionStatus(i32);
pub struct WiFiDirectDevice(i32);
pub struct WiFiDirectDeviceSelectorType(i32);
pub struct WiFiDirectError(i32);
pub struct WiFiDirectInformationElement(i32);
pub struct WiFiDirectLegacySettings(i32);
pub struct WiFiDirectPairingProcedure(i32);
