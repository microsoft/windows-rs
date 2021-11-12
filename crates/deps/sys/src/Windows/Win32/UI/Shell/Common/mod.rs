#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct COMDLG_FILTERSPEC(i32);
pub struct DEVICE_SCALE_FACTOR(i32);
pub struct IObjectArray(i32);
pub struct IObjectCollection(i32);
pub struct ITEMIDLIST(i32);
pub struct PERCEIVED(i32);
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_GDIPLUS: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_HARDCODED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_NATIVESUPPORT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_SOFTCODED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_WMSDK: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Shell_Common`*"]
pub const PERCEIVEDFLAG_ZIPFOLDER: u32 = 64u32;
pub struct SHCOLSTATE(i32);
pub struct SHELLDETAILS(i32);
pub struct SHITEMID(i32);
pub struct STRRET(i32);
pub struct STRRET_TYPE(i32);
