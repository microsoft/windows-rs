#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWiFiDirectService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiDirectService {}
impl ::core::clone::Clone for IWiFiDirectService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiDirectServiceAdvertiser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiDirectServiceAdvertiser {}
impl ::core::clone::Clone for IWiFiDirectServiceAdvertiser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiDirectServiceAdvertiserFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiDirectServiceAdvertiserFactory {}
impl ::core::clone::Clone for IWiFiDirectServiceAdvertiserFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs {}
impl ::core::clone::Clone for IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiDirectServiceProvisioningInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiDirectServiceProvisioningInfo {}
impl ::core::clone::Clone for IWiFiDirectServiceProvisioningInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiDirectServiceRemotePortAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiDirectServiceRemotePortAddedEventArgs {}
impl ::core::clone::Clone for IWiFiDirectServiceRemotePortAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiDirectServiceSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiDirectServiceSession {}
impl ::core::clone::Clone for IWiFiDirectServiceSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiDirectServiceSessionDeferredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiDirectServiceSessionDeferredEventArgs {}
impl ::core::clone::Clone for IWiFiDirectServiceSessionDeferredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiDirectServiceSessionRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiDirectServiceSessionRequest {}
impl ::core::clone::Clone for IWiFiDirectServiceSessionRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiDirectServiceSessionRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiDirectServiceSessionRequestedEventArgs {}
impl ::core::clone::Clone for IWiFiDirectServiceSessionRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWiFiDirectServiceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWiFiDirectServiceStatics {}
impl ::core::clone::Clone for IWiFiDirectServiceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiDirectService {}
impl ::core::clone::Clone for WiFiDirectService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceAdvertisementStatus(pub i32);
impl WiFiDirectServiceAdvertisementStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiDirectServiceAdvertisementStatus {}
impl ::core::clone::Clone for WiFiDirectServiceAdvertisementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceAdvertiser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiDirectServiceAdvertiser {}
impl ::core::clone::Clone for WiFiDirectServiceAdvertiser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceAutoAcceptSessionConnectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {}
impl ::core::clone::Clone for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceConfigurationMethod(pub i32);
impl WiFiDirectServiceConfigurationMethod {
    pub const Default: Self = Self(0i32);
    pub const PinDisplay: Self = Self(1i32);
    pub const PinEntry: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectServiceConfigurationMethod {}
impl ::core::clone::Clone for WiFiDirectServiceConfigurationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceError(pub i32);
impl WiFiDirectServiceError {
    pub const Success: Self = Self(0i32);
    pub const RadioNotAvailable: Self = Self(1i32);
    pub const ResourceInUse: Self = Self(2i32);
    pub const UnsupportedHardware: Self = Self(3i32);
    pub const NoHardware: Self = Self(4i32);
}
impl ::core::marker::Copy for WiFiDirectServiceError {}
impl ::core::clone::Clone for WiFiDirectServiceError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceIPProtocol(pub i32);
impl WiFiDirectServiceIPProtocol {
    pub const Tcp: Self = Self(6i32);
    pub const Udp: Self = Self(17i32);
}
impl ::core::marker::Copy for WiFiDirectServiceIPProtocol {}
impl ::core::clone::Clone for WiFiDirectServiceIPProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceProvisioningInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiDirectServiceProvisioningInfo {}
impl ::core::clone::Clone for WiFiDirectServiceProvisioningInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceRemotePortAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiDirectServiceRemotePortAddedEventArgs {}
impl ::core::clone::Clone for WiFiDirectServiceRemotePortAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiDirectServiceSession {}
impl ::core::clone::Clone for WiFiDirectServiceSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceSessionDeferredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiDirectServiceSessionDeferredEventArgs {}
impl ::core::clone::Clone for WiFiDirectServiceSessionDeferredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceSessionErrorStatus(pub i32);
impl WiFiDirectServiceSessionErrorStatus {
    pub const Ok: Self = Self(0i32);
    pub const Disassociated: Self = Self(1i32);
    pub const LocalClose: Self = Self(2i32);
    pub const RemoteClose: Self = Self(3i32);
    pub const SystemFailure: Self = Self(4i32);
    pub const NoResponseFromRemote: Self = Self(5i32);
}
impl ::core::marker::Copy for WiFiDirectServiceSessionErrorStatus {}
impl ::core::clone::Clone for WiFiDirectServiceSessionErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceSessionRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiDirectServiceSessionRequest {}
impl ::core::clone::Clone for WiFiDirectServiceSessionRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceSessionRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WiFiDirectServiceSessionRequestedEventArgs {}
impl ::core::clone::Clone for WiFiDirectServiceSessionRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceSessionStatus(pub i32);
impl WiFiDirectServiceSessionStatus {
    pub const Closed: Self = Self(0i32);
    pub const Initiated: Self = Self(1i32);
    pub const Requested: Self = Self(2i32);
    pub const Open: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiDirectServiceSessionStatus {}
impl ::core::clone::Clone for WiFiDirectServiceSessionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WiFiDirectServiceStatus(pub i32);
impl WiFiDirectServiceStatus {
    pub const Available: Self = Self(0i32);
    pub const Busy: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectServiceStatus {}
impl ::core::clone::Clone for WiFiDirectServiceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
