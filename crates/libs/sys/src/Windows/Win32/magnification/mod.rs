#[cfg(feature = "windef")]
windows_link::link!("magnification.dll" "system" fn MagGetColorEffect(hwnd : super::windef::HWND, peffect : *mut MAGCOLOREFFECT) -> windows_sys::core::BOOL);
windows_link::link!("magnification.dll" "system" fn MagGetFullscreenColorEffect(peffect : *mut MAGCOLOREFFECT) -> windows_sys::core::BOOL);
windows_link::link!("magnification.dll" "system" fn MagGetFullscreenTransform(pmaglevel : *mut f32, pxoffset : *mut i32, pyoffset : *mut i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "wincodec", feature = "windef"))]
windows_link::link!("magnification.dll" "system" fn MagGetImageScalingCallback(hwnd : super::windef::HWND) -> MagImageScalingCallback);
#[cfg(feature = "windef")]
windows_link::link!("magnification.dll" "system" fn MagGetInputTransform(pfenabled : *mut windows_sys::core::BOOL, prectsource : *mut super::windef::RECT, prectdest : *mut super::windef::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("magnification.dll" "system" fn MagGetWindowFilterList(hwnd : super::windef::HWND, pdwfiltermode : *mut u32, count : i32, phwnd : *mut super::windef::HWND) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("magnification.dll" "system" fn MagGetWindowSource(hwnd : super::windef::HWND, prect : *mut super::windef::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("magnification.dll" "system" fn MagGetWindowTransform(hwnd : super::windef::HWND, ptransform : *mut MAGTRANSFORM) -> windows_sys::core::BOOL);
windows_link::link!("magnification.dll" "system" fn MagInitialize() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("magnification.dll" "system" fn MagSetColorEffect(hwnd : super::windef::HWND, peffect : *mut MAGCOLOREFFECT) -> windows_sys::core::BOOL);
windows_link::link!("magnification.dll" "system" fn MagSetFullscreenColorEffect(peffect : *const MAGCOLOREFFECT) -> windows_sys::core::BOOL);
windows_link::link!("magnification.dll" "system" fn MagSetFullscreenTransform(maglevel : f32, xoffset : i32, yoffset : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "wincodec", feature = "windef"))]
windows_link::link!("magnification.dll" "system" fn MagSetImageScalingCallback(hwnd : super::windef::HWND, callback : MagImageScalingCallback) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("magnification.dll" "system" fn MagSetInputTransform(fenabled : windows_sys::core::BOOL, prectsource : *const super::windef::RECT, prectdest : *const super::windef::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("magnification.dll" "system" fn MagSetWindowFilterList(hwnd : super::windef::HWND, dwfiltermode : u32, count : i32, phwnd : *mut super::windef::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("magnification.dll" "system" fn MagSetWindowSource(hwnd : super::windef::HWND, rect : super::windef::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("magnification.dll" "system" fn MagSetWindowTransform(hwnd : super::windef::HWND, ptransform : *mut MAGTRANSFORM) -> windows_sys::core::BOOL);
windows_link::link!("magnification.dll" "system" fn MagShowSystemCursor(fshowcursor : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("magnification.dll" "system" fn MagUninitialize() -> windows_sys::core::BOOL);
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
pub struct MAGIMAGEHEADER {
    pub width: u32,
    pub height: u32,
    pub format: super::wincodec::WICPixelFormatGUID,
    pub stride: u32,
    pub offset: u32,
    pub cbSize: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
pub type MagImageScalingCallback = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, srcdata: *mut core::ffi::c_void, srcheader: MAGIMAGEHEADER, destdata: *mut core::ffi::c_void, destheader: MAGIMAGEHEADER, unclipped: super::windef::RECT, clipped: super::windef::RECT, dirty: super::minwindef::HRGN) -> windows_sys::core::BOOL>;
pub type PMAGCOLOREFFECT = *mut MAGCOLOREFFECT;
#[cfg(feature = "wincodec")]
pub type PMAGIMAGEHEADER = *mut MAGIMAGEHEADER;
pub type PMAGTRANSFORM = *mut MAGTRANSFORM;
pub const WC_MAGNIFIERA: windows_sys::core::PCSTR = windows_sys::core::s!("Magnifier");
pub const WC_MAGNIFIERW: windows_sys::core::PCWSTR = windows_sys::core::w!("Magnifier");
