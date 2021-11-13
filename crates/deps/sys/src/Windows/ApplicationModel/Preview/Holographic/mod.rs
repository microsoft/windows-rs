#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HolographicKeyboardPlacementOverridePreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicKeyboardPlacementOverridePreview {}
impl ::core::clone::Clone for HolographicKeyboardPlacementOverridePreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicApplicationPreviewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicApplicationPreviewStatics {}
impl ::core::clone::Clone for IHolographicApplicationPreviewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicKeyboardPlacementOverridePreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicKeyboardPlacementOverridePreview {}
impl ::core::clone::Clone for IHolographicKeyboardPlacementOverridePreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicKeyboardPlacementOverridePreviewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicKeyboardPlacementOverridePreviewStatics {}
impl ::core::clone::Clone for IHolographicKeyboardPlacementOverridePreviewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
