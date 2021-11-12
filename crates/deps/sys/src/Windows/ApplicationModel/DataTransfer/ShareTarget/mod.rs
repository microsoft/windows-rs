#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IQuickLink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareOperation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareOperation3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct QuickLink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShareOperation(pub *mut ::core::ffi::c_void);
