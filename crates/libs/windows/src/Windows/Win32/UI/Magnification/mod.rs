#[inline]
pub unsafe fn MagGetColorEffect(hwnd: super::super::Foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagGetColorEffect(hwnd : super::super::Foundation:: HWND, peffect : *mut MAGCOLOREFFECT) -> super::super::Foundation:: BOOL);
    MagGetColorEffect(hwnd, core::mem::transmute(peffect))
}
#[inline]
pub unsafe fn MagGetFullscreenColorEffect(peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagGetFullscreenColorEffect(peffect : *mut MAGCOLOREFFECT) -> super::super::Foundation:: BOOL);
    MagGetFullscreenColorEffect(core::mem::transmute(peffect))
}
#[inline]
pub unsafe fn MagGetFullscreenTransform(pmaglevel: *mut f32, pxoffset: *mut i32, pyoffset: *mut i32) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagGetFullscreenTransform(pmaglevel : *mut f32, pxoffset : *mut i32, pyoffset : *mut i32) -> super::super::Foundation:: BOOL);
    MagGetFullscreenTransform(core::mem::transmute(pmaglevel), core::mem::transmute(pxoffset), core::mem::transmute(pyoffset))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn MagGetImageScalingCallback(hwnd: super::super::Foundation::HWND) -> MagImageScalingCallback {
    windows_targets::link!("magnification.dll" "system" fn MagGetImageScalingCallback(hwnd : super::super::Foundation:: HWND) -> MagImageScalingCallback);
    MagGetImageScalingCallback(hwnd)
}
#[inline]
pub unsafe fn MagGetInputTransform(pfenabled: *mut super::super::Foundation::BOOL, prectsource: *mut super::super::Foundation::RECT, prectdest: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagGetInputTransform(pfenabled : *mut super::super::Foundation:: BOOL, prectsource : *mut super::super::Foundation:: RECT, prectdest : *mut super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    MagGetInputTransform(core::mem::transmute(pfenabled), core::mem::transmute(prectsource), core::mem::transmute(prectdest))
}
#[inline]
pub unsafe fn MagGetWindowFilterList(hwnd: super::super::Foundation::HWND, pdwfiltermode: *mut MW_FILTERMODE, count: i32, phwnd: *mut super::super::Foundation::HWND) -> i32 {
    windows_targets::link!("magnification.dll" "system" fn MagGetWindowFilterList(hwnd : super::super::Foundation:: HWND, pdwfiltermode : *mut MW_FILTERMODE, count : i32, phwnd : *mut super::super::Foundation:: HWND) -> i32);
    MagGetWindowFilterList(hwnd, core::mem::transmute(pdwfiltermode), count, core::mem::transmute(phwnd))
}
#[inline]
pub unsafe fn MagGetWindowSource(hwnd: super::super::Foundation::HWND, prect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagGetWindowSource(hwnd : super::super::Foundation:: HWND, prect : *mut super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    MagGetWindowSource(hwnd, core::mem::transmute(prect))
}
#[inline]
pub unsafe fn MagGetWindowTransform(hwnd: super::super::Foundation::HWND, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagGetWindowTransform(hwnd : super::super::Foundation:: HWND, ptransform : *mut MAGTRANSFORM) -> super::super::Foundation:: BOOL);
    MagGetWindowTransform(hwnd, core::mem::transmute(ptransform))
}
#[inline]
pub unsafe fn MagInitialize() -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagInitialize() -> super::super::Foundation:: BOOL);
    MagInitialize()
}
#[inline]
pub unsafe fn MagSetColorEffect(hwnd: super::super::Foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagSetColorEffect(hwnd : super::super::Foundation:: HWND, peffect : *mut MAGCOLOREFFECT) -> super::super::Foundation:: BOOL);
    MagSetColorEffect(hwnd, core::mem::transmute(peffect))
}
#[inline]
pub unsafe fn MagSetFullscreenColorEffect(peffect: *const MAGCOLOREFFECT) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagSetFullscreenColorEffect(peffect : *const MAGCOLOREFFECT) -> super::super::Foundation:: BOOL);
    MagSetFullscreenColorEffect(peffect)
}
#[inline]
pub unsafe fn MagSetFullscreenTransform(maglevel: f32, xoffset: i32, yoffset: i32) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagSetFullscreenTransform(maglevel : f32, xoffset : i32, yoffset : i32) -> super::super::Foundation:: BOOL);
    MagSetFullscreenTransform(maglevel, xoffset, yoffset)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn MagSetImageScalingCallback(hwnd: super::super::Foundation::HWND, callback: MagImageScalingCallback) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagSetImageScalingCallback(hwnd : super::super::Foundation:: HWND, callback : MagImageScalingCallback) -> super::super::Foundation:: BOOL);
    MagSetImageScalingCallback(hwnd, callback)
}
#[inline]
pub unsafe fn MagSetInputTransform(fenabled: bool, prectsource: *const super::super::Foundation::RECT, prectdest: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
    windows_targets::link!("magnification.dll" "system" fn MagSetInputTransform(fenabled : super::super::Foundation:: BOOL, prectsource : *const super::super::Foundation:: RECT, prectdest : *const super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    MagSetInputTransform(fenabled.into(), prectsource, prectdest).ok()
}
#[inline]
pub unsafe fn MagSetWindowFilterList(hwnd: super::super::Foundation::HWND, dwfiltermode: MW_FILTERMODE, count: i32, phwnd: *mut super::super::Foundation::HWND) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagSetWindowFilterList(hwnd : super::super::Foundation:: HWND, dwfiltermode : MW_FILTERMODE, count : i32, phwnd : *mut super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    MagSetWindowFilterList(hwnd, dwfiltermode, count, core::mem::transmute(phwnd))
}
#[inline]
pub unsafe fn MagSetWindowSource(hwnd: super::super::Foundation::HWND, rect: super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagSetWindowSource(hwnd : super::super::Foundation:: HWND, rect : super::super::Foundation:: RECT) -> super::super::Foundation:: BOOL);
    MagSetWindowSource(hwnd, core::mem::transmute(rect))
}
#[inline]
pub unsafe fn MagSetWindowTransform(hwnd: super::super::Foundation::HWND, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagSetWindowTransform(hwnd : super::super::Foundation:: HWND, ptransform : *mut MAGTRANSFORM) -> super::super::Foundation:: BOOL);
    MagSetWindowTransform(hwnd, core::mem::transmute(ptransform))
}
#[inline]
pub unsafe fn MagShowSystemCursor(fshowcursor: bool) -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagShowSystemCursor(fshowcursor : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    MagShowSystemCursor(fshowcursor.into())
}
#[inline]
pub unsafe fn MagUninitialize() -> super::super::Foundation::BOOL {
    windows_targets::link!("magnification.dll" "system" fn MagUninitialize() -> super::super::Foundation:: BOOL);
    MagUninitialize()
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MAGCOLOREFFECT {
    pub transform: [f32; 25],
}
impl Default for MAGCOLOREFFECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MAGIMAGEHEADER {
    pub width: u32,
    pub height: u32,
    pub format: windows_core::GUID,
    pub stride: u32,
    pub offset: u32,
    pub cbSize: usize,
}
impl Default for MAGIMAGEHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MAGTRANSFORM {
    pub v: [f32; 9],
}
impl Default for MAGTRANSFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MS_CLIPAROUNDCURSOR: i32 = 2i32;
pub const MS_INVERTCOLORS: i32 = 4i32;
pub const MS_SHOWMAGNIFIEDCURSOR: i32 = 1i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MW_FILTERMODE(pub u32);
pub const MW_FILTERMODE_EXCLUDE: MW_FILTERMODE = MW_FILTERMODE(0u32);
pub const MW_FILTERMODE_INCLUDE: MW_FILTERMODE = MW_FILTERMODE(1u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type MagImageScalingCallback = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, srcdata: *mut core::ffi::c_void, srcheader: MAGIMAGEHEADER, destdata: *mut core::ffi::c_void, destheader: MAGIMAGEHEADER, unclipped: super::super::Foundation::RECT, clipped: super::super::Foundation::RECT, dirty: super::super::Graphics::Gdi::HRGN) -> super::super::Foundation::BOOL>;
pub const WC_MAGNIFIER: windows_core::PCWSTR = windows_core::w!("Magnifier");
pub const WC_MAGNIFIERA: windows_core::PCSTR = windows_core::s!("Magnifier");
pub const WC_MAGNIFIERW: windows_core::PCWSTR = windows_core::w!("Magnifier");
