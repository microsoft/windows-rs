#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IRadio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadioStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Radio(pub *mut ::core::ffi::c_void);
pub struct RadioAccessStatus(i32);
pub struct RadioKind(i32);
pub struct RadioState(i32);
