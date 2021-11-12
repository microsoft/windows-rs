#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IWiFiDirectService(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectServiceAdvertiser(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectServiceAdvertiserFactory(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectServiceProvisioningInfo(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectServiceRemotePortAddedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectServiceSession(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectServiceSessionDeferredEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectServiceSessionRequest(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectServiceSessionRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWiFiDirectServiceStatics(pub *mut ::core::ffi::c_void);
pub struct WiFiDirectService(i32);
pub struct WiFiDirectServiceAdvertisementStatus(i32);
pub struct WiFiDirectServiceAdvertiser(i32);
pub struct WiFiDirectServiceAutoAcceptSessionConnectedEventArgs(i32);
pub struct WiFiDirectServiceConfigurationMethod(i32);
pub struct WiFiDirectServiceError(i32);
pub struct WiFiDirectServiceIPProtocol(i32);
pub struct WiFiDirectServiceProvisioningInfo(i32);
pub struct WiFiDirectServiceRemotePortAddedEventArgs(i32);
pub struct WiFiDirectServiceSession(i32);
pub struct WiFiDirectServiceSessionDeferredEventArgs(i32);
pub struct WiFiDirectServiceSessionErrorStatus(i32);
pub struct WiFiDirectServiceSessionRequest(i32);
pub struct WiFiDirectServiceSessionRequestedEventArgs(i32);
pub struct WiFiDirectServiceSessionStatus(i32);
pub struct WiFiDirectServiceStatus(i32);
