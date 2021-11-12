#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct WiFiAccessStatus(i32);
#[repr(transparent)]
pub struct WiFiAdapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiAvailableNetwork(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WiFiConnectionMethod(i32);
#[repr(transparent)]
pub struct WiFiConnectionResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WiFiConnectionStatus(i32);
#[repr(C)]
pub struct WiFiNetworkKind(i32);
#[repr(transparent)]
pub struct WiFiNetworkReport(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WiFiPhyKind(i32);
#[repr(C)]
pub struct WiFiReconnectionKind(i32);
#[repr(transparent)]
pub struct WiFiWpsConfigurationResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WiFiWpsConfigurationStatus(i32);
#[repr(C)]
pub struct WiFiWpsKind(i32);
