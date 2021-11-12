#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPowerManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPowerManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PowerSavingMode(pub i32);
impl PowerSavingMode {
    pub const Off: PowerSavingMode = PowerSavingMode(0i32);
    pub const On: PowerSavingMode = PowerSavingMode(1i32);
}
