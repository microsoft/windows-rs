#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DeviceLockdownProfileInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DeviceLockdownProfileInformation {}
impl ::core::clone::Clone for DeviceLockdownProfileInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeviceLockdownProfileInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeviceLockdownProfileInformation {}
impl ::core::clone::Clone for IDeviceLockdownProfileInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeviceLockdownProfileStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeviceLockdownProfileStatics {}
impl ::core::clone::Clone for IDeviceLockdownProfileStatics {
    fn clone(&self) -> Self {
        *self
    }
}
