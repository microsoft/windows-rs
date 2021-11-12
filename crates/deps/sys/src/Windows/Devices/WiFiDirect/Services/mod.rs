#![allow(non_snake_case, non_camel_case_types)]
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
pub struct WiFiDirectServiceAdvertisementStatus(i32);
#[repr(transparent)]
pub struct WiFiDirectServiceAdvertiser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceAutoAcceptSessionConnectedEventArgs(pub *mut ::core::ffi::c_void);
pub struct WiFiDirectServiceConfigurationMethod(i32);
pub struct WiFiDirectServiceError(i32);
pub struct WiFiDirectServiceIPProtocol(i32);
#[repr(transparent)]
pub struct WiFiDirectServiceProvisioningInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceRemotePortAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceSessionDeferredEventArgs(pub *mut ::core::ffi::c_void);
pub struct WiFiDirectServiceSessionErrorStatus(i32);
#[repr(transparent)]
pub struct WiFiDirectServiceSessionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WiFiDirectServiceSessionRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct WiFiDirectServiceSessionStatus(i32);
pub struct WiFiDirectServiceStatus(i32);
