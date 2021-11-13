#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DnssdRegistrationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DnssdRegistrationResult {}
impl ::core::clone::Clone for DnssdRegistrationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DnssdRegistrationStatus(pub i32);
impl DnssdRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const InvalidServiceName: Self = Self(1i32);
    pub const ServerError: Self = Self(2i32);
    pub const SecurityError: Self = Self(3i32);
}
impl ::core::marker::Copy for DnssdRegistrationStatus {}
impl ::core::clone::Clone for DnssdRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DnssdServiceInstance(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DnssdServiceInstance {}
impl ::core::clone::Clone for DnssdServiceInstance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DnssdServiceInstanceCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DnssdServiceInstanceCollection {}
impl ::core::clone::Clone for DnssdServiceInstanceCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DnssdServiceWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DnssdServiceWatcher {}
impl ::core::clone::Clone for DnssdServiceWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DnssdServiceWatcherStatus(pub i32);
impl DnssdServiceWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for DnssdServiceWatcherStatus {}
impl ::core::clone::Clone for DnssdServiceWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDnssdRegistrationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDnssdRegistrationResult {}
impl ::core::clone::Clone for IDnssdRegistrationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDnssdServiceInstance(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDnssdServiceInstance {}
impl ::core::clone::Clone for IDnssdServiceInstance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDnssdServiceInstanceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDnssdServiceInstanceFactory {}
impl ::core::clone::Clone for IDnssdServiceInstanceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDnssdServiceWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDnssdServiceWatcher {}
impl ::core::clone::Clone for IDnssdServiceWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
