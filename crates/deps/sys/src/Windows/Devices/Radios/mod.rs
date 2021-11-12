#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IRadio(pub *mut ::core::ffi::c_void);
pub struct IRadioStatics(pub *mut ::core::ffi::c_void);
pub struct Radio(i32);
pub struct RadioAccessStatus(i32);
pub struct RadioKind(i32);
pub struct RadioState(i32);
