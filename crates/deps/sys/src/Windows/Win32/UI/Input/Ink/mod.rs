#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInkCommitRequestHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkD2DRenderer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkD2DRenderer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDesktopHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkHostWorkItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPresenterDesktop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INK_HIGH_CONTRAST_ADJUSTMENT(pub i32);
pub const USE_SYSTEM_COLORS_WHEN_NECESSARY: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(0i32);
pub const USE_SYSTEM_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(1i32);
pub const USE_ORIGINAL_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(2i32);
#[repr(C)]
pub struct InkD2DRenderer(i32);
#[repr(C)]
pub struct InkDesktopHost(i32);
