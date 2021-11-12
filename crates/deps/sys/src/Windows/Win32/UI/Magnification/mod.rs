#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetColorEffect(hwnd: super::super::Foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetFullscreenColorEffect(peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetFullscreenTransform(pmaglevel: *mut f32, pxoffset: *mut i32, pyoffset: *mut i32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn MagGetImageScalingCallback(hwnd: super::super::Foundation::HWND) -> ::core::option::Option<MagImageScalingCallback>;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetInputTransform(pfenabled: *mut super::super::Foundation::BOOL, prectsource: *mut super::super::Foundation::RECT, prectdest: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetWindowFilterList(hwnd: super::super::Foundation::HWND, pdwfiltermode: *mut u32, count: i32, phwnd: *mut super::super::Foundation::HWND) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetWindowSource(hwnd: super::super::Foundation::HWND, prect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetWindowTransform(hwnd: super::super::Foundation::HWND, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagInitialize() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetColorEffect(hwnd: super::super::Foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetFullscreenColorEffect(peffect: *const MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetFullscreenTransform(maglevel: f32, xoffset: i32, yoffset: i32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn MagSetImageScalingCallback(hwnd: super::super::Foundation::HWND, callback: MagImageScalingCallback) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetInputTransform(fenabled: super::super::Foundation::BOOL, prectsource: *const super::super::Foundation::RECT, prectdest: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetWindowFilterList(hwnd: super::super::Foundation::HWND, dwfiltermode: u32, count: i32, phwnd: *mut super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetWindowSource(hwnd: super::super::Foundation::HWND, rect: super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetWindowTransform(hwnd: super::super::Foundation::HWND, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagShowSystemCursor(fshowcursor: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagUninitialize() -> super::super::Foundation::BOOL;
}
#[repr(C)]
pub struct MAGCOLOREFFECT(i32);
#[repr(C)]
pub struct MAGIMAGEHEADER(i32);
#[repr(C)]
pub struct MAGTRANSFORM(i32);
pub const MS_CLIPAROUNDCURSOR: i32 = 2i32;
pub const MS_INVERTCOLORS: i32 = 4i32;
pub const MS_SHOWMAGNIFIEDCURSOR: i32 = 1i32;
pub const MW_FILTERMODE_EXCLUDE: u32 = 0u32;
pub const MW_FILTERMODE_INCLUDE: u32 = 1u32;
#[repr(C)]
pub struct MagImageScalingCallback(i32);
