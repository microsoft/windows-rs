#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct COMDLG_FILTERSPEC(i32);
#[repr(C)]
pub struct DEVICE_SCALE_FACTOR(i32);
#[repr(transparent)]
pub struct IObjectArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectCollection(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ITEMIDLIST(i32);
#[repr(C)]
pub struct PERCEIVED(i32);
pub const PERCEIVEDFLAG_GDIPLUS: u32 = 16u32;
pub const PERCEIVEDFLAG_HARDCODED: u32 = 2u32;
pub const PERCEIVEDFLAG_NATIVESUPPORT: u32 = 4u32;
pub const PERCEIVEDFLAG_SOFTCODED: u32 = 1u32;
pub const PERCEIVEDFLAG_UNDEFINED: u32 = 0u32;
pub const PERCEIVEDFLAG_WMSDK: u32 = 32u32;
pub const PERCEIVEDFLAG_ZIPFOLDER: u32 = 64u32;
#[repr(C)]
pub struct SHCOLSTATE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHELLDETAILS(i32);
#[repr(C)]
pub struct SHITEMID(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STRRET(i32);
#[repr(C)]
pub struct STRRET_TYPE(i32);
