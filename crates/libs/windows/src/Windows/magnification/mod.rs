#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MagGetColorEffect(hwnd: super::windef::HWND, peffect: *mut MAGCOLOREFFECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagGetColorEffect(hwnd : super::windef::HWND, peffect : *mut MAGCOLOREFFECT) -> windows_core::BOOL);
    unsafe { MagGetColorEffect(hwnd, peffect as _) }
}
#[inline]
pub unsafe fn MagGetFullscreenColorEffect(peffect: *mut MAGCOLOREFFECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagGetFullscreenColorEffect(peffect : *mut MAGCOLOREFFECT) -> windows_core::BOOL);
    unsafe { MagGetFullscreenColorEffect(peffect as _) }
}
#[inline]
pub unsafe fn MagGetFullscreenTransform(pmaglevel: *mut f32, pxoffset: *mut i32, pyoffset: *mut i32) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagGetFullscreenTransform(pmaglevel : *mut f32, pxoffset : *mut i32, pyoffset : *mut i32) -> windows_core::BOOL);
    unsafe { MagGetFullscreenTransform(pmaglevel as _, pxoffset as _, pyoffset as _) }
}
#[cfg(all(feature = "minwindef", feature = "wincodec", feature = "windef"))]
#[inline]
pub unsafe fn MagGetImageScalingCallback(hwnd: super::windef::HWND) -> MagImageScalingCallback {
    windows_core::link!("magnification.dll" "system" fn MagGetImageScalingCallback(hwnd : super::windef::HWND) -> MagImageScalingCallback);
    unsafe { MagGetImageScalingCallback(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MagGetInputTransform(pfenabled: *mut windows_core::BOOL, prectsource: *mut super::windef::RECT, prectdest: *mut super::windef::RECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagGetInputTransform(pfenabled : *mut windows_core::BOOL, prectsource : *mut super::windef::RECT, prectdest : *mut super::windef::RECT) -> windows_core::BOOL);
    unsafe { MagGetInputTransform(pfenabled as _, prectsource as _, prectdest as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MagGetWindowFilterList(hwnd: super::windef::HWND, pdwfiltermode: *mut u32, count: i32, phwnd: *mut super::windef::HWND) -> i32 {
    windows_core::link!("magnification.dll" "system" fn MagGetWindowFilterList(hwnd : super::windef::HWND, pdwfiltermode : *mut u32, count : i32, phwnd : *mut super::windef::HWND) -> i32);
    unsafe { MagGetWindowFilterList(hwnd, pdwfiltermode as _, count, phwnd as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MagGetWindowSource(hwnd: super::windef::HWND, prect: *mut super::windef::RECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagGetWindowSource(hwnd : super::windef::HWND, prect : *mut super::windef::RECT) -> windows_core::BOOL);
    unsafe { MagGetWindowSource(hwnd, prect as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MagGetWindowTransform(hwnd: super::windef::HWND, ptransform: *mut MAGTRANSFORM) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagGetWindowTransform(hwnd : super::windef::HWND, ptransform : *mut MAGTRANSFORM) -> windows_core::BOOL);
    unsafe { MagGetWindowTransform(hwnd, ptransform as _) }
}
#[inline]
pub unsafe fn MagInitialize() -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagInitialize() -> windows_core::BOOL);
    unsafe { MagInitialize() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MagSetColorEffect(hwnd: super::windef::HWND, peffect: *mut MAGCOLOREFFECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetColorEffect(hwnd : super::windef::HWND, peffect : *mut MAGCOLOREFFECT) -> windows_core::BOOL);
    unsafe { MagSetColorEffect(hwnd, peffect as _) }
}
#[inline]
pub unsafe fn MagSetFullscreenColorEffect(peffect: *const MAGCOLOREFFECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetFullscreenColorEffect(peffect : *const MAGCOLOREFFECT) -> windows_core::BOOL);
    unsafe { MagSetFullscreenColorEffect(peffect) }
}
#[inline]
pub unsafe fn MagSetFullscreenTransform(maglevel: f32, xoffset: i32, yoffset: i32) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetFullscreenTransform(maglevel : f32, xoffset : i32, yoffset : i32) -> windows_core::BOOL);
    unsafe { MagSetFullscreenTransform(maglevel, xoffset, yoffset) }
}
#[cfg(all(feature = "minwindef", feature = "wincodec", feature = "windef"))]
#[inline]
pub unsafe fn MagSetImageScalingCallback(hwnd: super::windef::HWND, callback: MagImageScalingCallback) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetImageScalingCallback(hwnd : super::windef::HWND, callback : MagImageScalingCallback) -> windows_core::BOOL);
    unsafe { MagSetImageScalingCallback(hwnd, callback) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MagSetInputTransform(fenabled: bool, prectsource: *const super::windef::RECT, prectdest: *const super::windef::RECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetInputTransform(fenabled : windows_core::BOOL, prectsource : *const super::windef::RECT, prectdest : *const super::windef::RECT) -> windows_core::BOOL);
    unsafe { MagSetInputTransform(fenabled.into(), prectsource, prectdest) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MagSetWindowFilterList(hwnd: super::windef::HWND, dwfiltermode: u32, count: i32, phwnd: *mut super::windef::HWND) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetWindowFilterList(hwnd : super::windef::HWND, dwfiltermode : u32, count : i32, phwnd : *mut super::windef::HWND) -> windows_core::BOOL);
    unsafe { MagSetWindowFilterList(hwnd, dwfiltermode, count, phwnd as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MagSetWindowSource(hwnd: super::windef::HWND, rect: super::windef::RECT) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetWindowSource(hwnd : super::windef::HWND, rect : super::windef::RECT) -> windows_core::BOOL);
    unsafe { MagSetWindowSource(hwnd, rect) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MagSetWindowTransform(hwnd: super::windef::HWND, ptransform: *mut MAGTRANSFORM) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagSetWindowTransform(hwnd : super::windef::HWND, ptransform : *mut MAGTRANSFORM) -> windows_core::BOOL);
    unsafe { MagSetWindowTransform(hwnd, ptransform as _) }
}
#[inline]
pub unsafe fn MagShowSystemCursor(fshowcursor: bool) -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagShowSystemCursor(fshowcursor : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { MagShowSystemCursor(fshowcursor.into()) }
}
#[inline]
pub unsafe fn MagUninitialize() -> windows_core::BOOL {
    windows_core::link!("magnification.dll" "system" fn MagUninitialize() -> windows_core::BOOL);
    unsafe { MagUninitialize() }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MAGCOLOREFFECT {
    pub transform: [[f32; 5]; 5],
}
impl Default for MAGCOLOREFFECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincodec")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MAGIMAGEHEADER {
    pub width: u32,
    pub height: u32,
    pub format: super::wincodec::WICPixelFormatGUID,
    pub stride: u32,
    pub offset: u32,
    pub cbSize: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MAGTRANSFORM {
    pub v: [[f32; 3]; 3],
}
impl Default for MAGTRANSFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MS_CLIPAROUNDCURSOR: u32 = 2;
pub const MS_INVERTCOLORS: u32 = 4;
pub const MS_SHOWMAGNIFIEDCURSOR: u32 = 1;
pub const MW_FILTERMODE_EXCLUDE: u32 = 0;
pub const MW_FILTERMODE_INCLUDE: u32 = 1;
#[cfg(all(feature = "minwindef", feature = "wincodec", feature = "windef"))]
pub type MagImageScalingCallback = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, srcdata: *mut core::ffi::c_void, srcheader: MAGIMAGEHEADER, destdata: *mut core::ffi::c_void, destheader: MAGIMAGEHEADER, unclipped: super::windef::RECT, clipped: super::windef::RECT, dirty: super::minwindef::HRGN) -> windows_core::BOOL>;
pub type PMAGCOLOREFFECT = *mut MAGCOLOREFFECT;
#[cfg(feature = "wincodec")]
pub type PMAGIMAGEHEADER = *mut MAGIMAGEHEADER;
pub type PMAGTRANSFORM = *mut MAGTRANSFORM;
pub const WC_MAGNIFIERA: windows_core::PCSTR = windows_core::s!("Magnifier");
pub const WC_MAGNIFIERW: windows_core::PCWSTR = windows_core::w!("Magnifier");
