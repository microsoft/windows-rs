#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Graphics_Capture")]
pub mod Capture;
#[cfg(feature = "Graphics_DirectX")]
pub mod DirectX;
#[cfg(feature = "Graphics_Display")]
pub mod Display;
#[cfg(feature = "Graphics_Effects")]
pub mod Effects;
#[cfg(feature = "Graphics_Holographic")]
pub mod Holographic;
#[cfg(feature = "Graphics_Imaging")]
pub mod Imaging;
#[cfg(feature = "Graphics_Printing")]
pub mod Printing;
#[cfg(feature = "Graphics_Printing3D")]
pub mod Printing3D;
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct DisplayAdapterId(i32);
#[repr(C)]
pub struct DisplayId(i32);
#[repr(transparent)]
pub struct IGeometrySource2D(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PointInt32(i32);
#[repr(C)]
pub struct RectInt32(i32);
#[repr(C)]
pub struct SizeInt32(i32);
