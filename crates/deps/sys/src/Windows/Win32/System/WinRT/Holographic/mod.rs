#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IHolographicCameraInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCameraInterop {}
impl ::core::clone::Clone for IHolographicCameraInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCameraRenderingParametersInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCameraRenderingParametersInterop {}
impl ::core::clone::Clone for IHolographicCameraRenderingParametersInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicQuadLayerInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicQuadLayerInterop {}
impl ::core::clone::Clone for IHolographicQuadLayerInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicQuadLayerUpdateParametersInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicQuadLayerUpdateParametersInterop {}
impl ::core::clone::Clone for IHolographicQuadLayerUpdateParametersInterop {
    fn clone(&self) -> Self {
        *self
    }
}
