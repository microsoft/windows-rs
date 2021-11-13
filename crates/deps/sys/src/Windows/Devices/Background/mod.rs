#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DeviceServicingDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DeviceServicingDetails {}
impl ::core::clone::Clone for DeviceServicingDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeviceUseDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DeviceUseDetails {}
impl ::core::clone::Clone for DeviceUseDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeviceServicingDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeviceServicingDetails {}
impl ::core::clone::Clone for IDeviceServicingDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeviceUseDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeviceUseDetails {}
impl ::core::clone::Clone for IDeviceUseDetails {
    fn clone(&self) -> Self {
        *self
    }
}
