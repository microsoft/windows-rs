#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IInkCommitRequestHandler(pub *mut ::core::ffi::c_void);
pub struct IInkD2DRenderer(pub *mut ::core::ffi::c_void);
pub struct IInkD2DRenderer2(pub *mut ::core::ffi::c_void);
pub struct IInkDesktopHost(pub *mut ::core::ffi::c_void);
pub struct IInkHostWorkItem(pub *mut ::core::ffi::c_void);
pub struct IInkPresenterDesktop(pub *mut ::core::ffi::c_void);
pub struct INK_HIGH_CONTRAST_ADJUSTMENT(i32);
pub struct InkD2DRenderer(i32);
pub struct InkDesktopHost(i32);
