#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub struct MAGCOLOREFFECT {
    pub transform: [f32; 25],
}
impl ::core::marker::Copy for MAGCOLOREFFECT {}
impl ::core::clone::Clone for MAGCOLOREFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MAGCOLOREFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAGCOLOREFFECT").field("transform", &self.transform).finish()
    }
}
unsafe impl ::windows::core::Abi for MAGCOLOREFFECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MAGCOLOREFFECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MAGCOLOREFFECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MAGCOLOREFFECT {}
impl ::core::default::Default for MAGCOLOREFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub struct MAGIMAGEHEADER {
    pub width: u32,
    pub height: u32,
    pub format: ::windows::core::GUID,
    pub stride: u32,
    pub offset: u32,
    pub cbSize: usize,
}
impl ::core::marker::Copy for MAGIMAGEHEADER {}
impl ::core::clone::Clone for MAGIMAGEHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MAGIMAGEHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAGIMAGEHEADER").field("width", &self.width).field("height", &self.height).field("format", &self.format).field("stride", &self.stride).field("offset", &self.offset).field("cbSize", &self.cbSize).finish()
    }
}
unsafe impl ::windows::core::Abi for MAGIMAGEHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MAGIMAGEHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MAGIMAGEHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for MAGIMAGEHEADER {}
impl ::core::default::Default for MAGIMAGEHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub struct MAGTRANSFORM {
    pub v: [f32; 9],
}
impl ::core::marker::Copy for MAGTRANSFORM {}
impl ::core::clone::Clone for MAGTRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MAGTRANSFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAGTRANSFORM").field("v", &self.v).finish()
    }
}
unsafe impl ::windows::core::Abi for MAGTRANSFORM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MAGTRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MAGTRANSFORM>()) == 0 }
    }
}
impl ::core::cmp::Eq for MAGTRANSFORM {}
impl ::core::default::Default for MAGTRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const MS_CLIPAROUNDCURSOR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const MS_INVERTCOLORS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const MS_SHOWMAGNIFIEDCURSOR: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const MW_FILTERMODE_EXCLUDE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const MW_FILTERMODE_INCLUDE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetColorEffect<'a, P0>(hwnd: P0, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagGetColorEffect(hwnd: super::super::Foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
    }
    MagGetColorEffect(hwnd.into(), ::core::mem::transmute(peffect))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetFullscreenColorEffect(peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagGetFullscreenColorEffect(peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
    }
    MagGetFullscreenColorEffect(::core::mem::transmute(peffect))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetFullscreenTransform(pmaglevel: *mut f32, pxoffset: *mut i32, pyoffset: *mut i32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagGetFullscreenTransform(pmaglevel: *mut f32, pxoffset: *mut i32, pyoffset: *mut i32) -> super::super::Foundation::BOOL;
    }
    MagGetFullscreenTransform(::core::mem::transmute(pmaglevel), ::core::mem::transmute(pxoffset), ::core::mem::transmute(pyoffset))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn MagGetImageScalingCallback<'a, P0>(hwnd: P0) -> MagImageScalingCallback
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagGetImageScalingCallback(hwnd: super::super::Foundation::HWND) -> MagImageScalingCallback;
    }
    MagGetImageScalingCallback(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetInputTransform(pfenabled: *mut super::super::Foundation::BOOL, prectsource: *mut super::super::Foundation::RECT, prectdest: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagGetInputTransform(pfenabled: *mut super::super::Foundation::BOOL, prectsource: *mut super::super::Foundation::RECT, prectdest: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    MagGetInputTransform(::core::mem::transmute(pfenabled), ::core::mem::transmute(prectsource), ::core::mem::transmute(prectdest))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetWindowFilterList<'a, P0>(hwnd: P0, pdwfiltermode: *mut u32, count: i32, phwnd: *mut super::super::Foundation::HWND) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagGetWindowFilterList(hwnd: super::super::Foundation::HWND, pdwfiltermode: *mut u32, count: i32, phwnd: *mut super::super::Foundation::HWND) -> i32;
    }
    MagGetWindowFilterList(hwnd.into(), ::core::mem::transmute(pdwfiltermode), count, ::core::mem::transmute(phwnd))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetWindowSource<'a, P0>(hwnd: P0, prect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagGetWindowSource(hwnd: super::super::Foundation::HWND, prect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    MagGetWindowSource(hwnd.into(), ::core::mem::transmute(prect))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetWindowTransform<'a, P0>(hwnd: P0, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagGetWindowTransform(hwnd: super::super::Foundation::HWND, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL;
    }
    MagGetWindowTransform(hwnd.into(), ::core::mem::transmute(ptransform))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type MagImageScalingCallback = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, srcdata: *mut ::core::ffi::c_void, srcheader: MAGIMAGEHEADER, destdata: *mut ::core::ffi::c_void, destheader: MAGIMAGEHEADER, unclipped: super::super::Foundation::RECT, clipped: super::super::Foundation::RECT, dirty: super::super::Graphics::Gdi::HRGN) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagInitialize() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagInitialize() -> super::super::Foundation::BOOL;
    }
    MagInitialize()
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetColorEffect<'a, P0>(hwnd: P0, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagSetColorEffect(hwnd: super::super::Foundation::HWND, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
    }
    MagSetColorEffect(hwnd.into(), ::core::mem::transmute(peffect))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetFullscreenColorEffect(peffect: *const MAGCOLOREFFECT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagSetFullscreenColorEffect(peffect: *const MAGCOLOREFFECT) -> super::super::Foundation::BOOL;
    }
    MagSetFullscreenColorEffect(::core::mem::transmute(peffect))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetFullscreenTransform(maglevel: f32, xoffset: i32, yoffset: i32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagSetFullscreenTransform(maglevel: f32, xoffset: i32, yoffset: i32) -> super::super::Foundation::BOOL;
    }
    MagSetFullscreenTransform(maglevel, xoffset, yoffset)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn MagSetImageScalingCallback<'a, P0>(hwnd: P0, callback: MagImageScalingCallback) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagSetImageScalingCallback(hwnd: super::super::Foundation::HWND, callback: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    MagSetImageScalingCallback(hwnd.into(), ::core::mem::transmute(callback))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetInputTransform<'a, P0>(fenabled: P0, prectsource: *const super::super::Foundation::RECT, prectdest: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagSetInputTransform(fenabled: super::super::Foundation::BOOL, prectsource: *const super::super::Foundation::RECT, prectdest: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    MagSetInputTransform(fenabled.into(), ::core::mem::transmute(prectsource), ::core::mem::transmute(prectdest))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetWindowFilterList<'a, P0>(hwnd: P0, dwfiltermode: u32, count: i32, phwnd: *mut super::super::Foundation::HWND) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagSetWindowFilterList(hwnd: super::super::Foundation::HWND, dwfiltermode: u32, count: i32, phwnd: *mut super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    MagSetWindowFilterList(hwnd.into(), dwfiltermode, count, ::core::mem::transmute(phwnd))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetWindowSource<'a, P0>(hwnd: P0, rect: super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagSetWindowSource(hwnd: super::super::Foundation::HWND, rect: super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    }
    MagSetWindowSource(hwnd.into(), ::core::mem::transmute(rect))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetWindowTransform<'a, P0>(hwnd: P0, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagSetWindowTransform(hwnd: super::super::Foundation::HWND, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL;
    }
    MagSetWindowTransform(hwnd.into(), ::core::mem::transmute(ptransform))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagShowSystemCursor<'a, P0>(fshowcursor: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagShowSystemCursor(fshowcursor: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    MagShowSystemCursor(fshowcursor.into())
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagUninitialize() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MagUninitialize() -> super::super::Foundation::BOOL;
    }
    MagUninitialize()
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const WC_MAGNIFIER: &str = "Magnifier";
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const WC_MAGNIFIERA: &str = "Magnifier";
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const WC_MAGNIFIERW: &str = "Magnifier";
#[cfg(feature = "implement")]
::core::include!("impl.rs");
