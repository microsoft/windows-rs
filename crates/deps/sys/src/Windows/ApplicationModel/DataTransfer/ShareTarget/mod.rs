#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IQuickLink(pub *mut ::core::ffi::c_void);
pub struct IShareOperation(pub *mut ::core::ffi::c_void);
pub struct IShareOperation2(pub *mut ::core::ffi::c_void);
pub struct IShareOperation3(pub *mut ::core::ffi::c_void);
pub struct QuickLink(i32);
pub struct ShareOperation(i32);
