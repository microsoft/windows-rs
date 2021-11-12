#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IHolographicCameraInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicCameraRenderingParametersInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicQuadLayerInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicQuadLayerUpdateParametersInterop(pub *mut ::core::ffi::c_void);
