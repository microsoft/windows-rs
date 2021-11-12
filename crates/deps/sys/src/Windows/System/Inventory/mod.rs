#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IInstalledDesktopApp(pub *mut ::core::ffi::c_void);
pub struct IInstalledDesktopAppStatics(pub *mut ::core::ffi::c_void);
pub struct InstalledDesktopApp(i32);
