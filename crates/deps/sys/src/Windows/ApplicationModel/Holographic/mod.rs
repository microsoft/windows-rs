#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HolographicKeyboard(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicKeyboard {}
impl ::core::clone::Clone for HolographicKeyboard {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicKeyboard(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicKeyboard {}
impl ::core::clone::Clone for IHolographicKeyboard {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicKeyboardStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicKeyboardStatics {}
impl ::core::clone::Clone for IHolographicKeyboardStatics {
    fn clone(&self) -> Self {
        *self
    }
}
