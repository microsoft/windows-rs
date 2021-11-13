#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Battery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Battery {}
impl ::core::clone::Clone for Battery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBattery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBattery {}
impl ::core::clone::Clone for IBattery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBatteryStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBatteryStatics {}
impl ::core::clone::Clone for IBatteryStatics {
    fn clone(&self) -> Self {
        *self
    }
}
