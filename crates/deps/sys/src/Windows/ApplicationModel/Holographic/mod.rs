#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct HolographicKeyboard(i32);
pub struct IHolographicKeyboard(pub *mut ::core::ffi::c_void);
pub struct IHolographicKeyboardStatics(pub *mut ::core::ffi::c_void);
