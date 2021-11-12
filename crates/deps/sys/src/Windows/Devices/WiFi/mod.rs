#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IWiFiAdapter(pub *mut ::core::ffi::c_void);
pub struct IWiFiAdapter2(pub *mut ::core::ffi::c_void);
pub struct IWiFiAdapterStatics(pub *mut ::core::ffi::c_void);
pub struct IWiFiAvailableNetwork(pub *mut ::core::ffi::c_void);
pub struct IWiFiConnectionResult(pub *mut ::core::ffi::c_void);
pub struct IWiFiNetworkReport(pub *mut ::core::ffi::c_void);
pub struct IWiFiWpsConfigurationResult(pub *mut ::core::ffi::c_void);
pub struct WiFiAccessStatus(i32);
pub struct WiFiAdapter(i32);
pub struct WiFiAvailableNetwork(i32);
pub struct WiFiConnectionMethod(i32);
pub struct WiFiConnectionResult(i32);
pub struct WiFiConnectionStatus(i32);
pub struct WiFiNetworkKind(i32);
pub struct WiFiNetworkReport(i32);
pub struct WiFiPhyKind(i32);
pub struct WiFiReconnectionKind(i32);
pub struct WiFiWpsConfigurationResult(i32);
pub struct WiFiWpsConfigurationStatus(i32);
pub struct WiFiWpsKind(i32);
