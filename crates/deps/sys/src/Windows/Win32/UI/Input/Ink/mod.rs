#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct INK_HIGH_CONTRAST_ADJUSTMENT(i32);
#[repr(C)]
pub struct InkD2DRenderer(i32);
#[repr(C)]
pub struct InkDesktopHost(i32);
