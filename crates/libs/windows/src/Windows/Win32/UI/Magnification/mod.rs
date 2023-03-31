#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetColorEffect<P0>(hwnd: P0, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "magnification.dll""system" fn MagGetColorEffect ( hwnd : super::super::Foundation:: HWND , peffect : *mut MAGCOLOREFFECT ) -> super::super::Foundation:: BOOL );
    MagGetColorEffect(hwnd.into_param().abi(), peffect)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetFullscreenColorEffect(peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "magnification.dll""system" fn MagGetFullscreenColorEffect ( peffect : *mut MAGCOLOREFFECT ) -> super::super::Foundation:: BOOL );
    MagGetFullscreenColorEffect(peffect)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetFullscreenTransform(pmaglevel: *mut f32, pxoffset: *mut i32, pyoffset: *mut i32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "magnification.dll""system" fn MagGetFullscreenTransform ( pmaglevel : *mut f32 , pxoffset : *mut i32 , pyoffset : *mut i32 ) -> super::super::Foundation:: BOOL );
    MagGetFullscreenTransform(pmaglevel, pxoffset, pyoffset)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn MagGetImageScalingCallback<P0>(hwnd: P0) -> MagImageScalingCallback
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "magnification.dll""system" fn MagGetImageScalingCallback ( hwnd : super::super::Foundation:: HWND ) -> MagImageScalingCallback );
    MagGetImageScalingCallback(hwnd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetInputTransform(pfenabled: *mut super::super::Foundation::BOOL, prectsource: *mut super::super::Foundation::RECT, prectdest: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "magnification.dll""system" fn MagGetInputTransform ( pfenabled : *mut super::super::Foundation:: BOOL , prectsource : *mut super::super::Foundation:: RECT , prectdest : *mut super::super::Foundation:: RECT ) -> super::super::Foundation:: BOOL );
    MagGetInputTransform(pfenabled, prectsource, prectdest)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetWindowFilterList<P0>(hwnd: P0, pdwfiltermode: *mut MW_FILTERMODE, count: i32, phwnd: *mut super::super::Foundation::HWND) -> i32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "magnification.dll""system" fn MagGetWindowFilterList ( hwnd : super::super::Foundation:: HWND , pdwfiltermode : *mut MW_FILTERMODE , count : i32 , phwnd : *mut super::super::Foundation:: HWND ) -> i32 );
    MagGetWindowFilterList(hwnd.into_param().abi(), pdwfiltermode, count, phwnd)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetWindowSource<P0>(hwnd: P0, prect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "magnification.dll""system" fn MagGetWindowSource ( hwnd : super::super::Foundation:: HWND , prect : *mut super::super::Foundation:: RECT ) -> super::super::Foundation:: BOOL );
    MagGetWindowSource(hwnd.into_param().abi(), prect)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagGetWindowTransform<P0>(hwnd: P0, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "magnification.dll""system" fn MagGetWindowTransform ( hwnd : super::super::Foundation:: HWND , ptransform : *mut MAGTRANSFORM ) -> super::super::Foundation:: BOOL );
    MagGetWindowTransform(hwnd.into_param().abi(), ptransform)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagInitialize() -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "magnification.dll""system" fn MagInitialize ( ) -> super::super::Foundation:: BOOL );
    MagInitialize()
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetColorEffect<P0>(hwnd: P0, peffect: *mut MAGCOLOREFFECT) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "magnification.dll""system" fn MagSetColorEffect ( hwnd : super::super::Foundation:: HWND , peffect : *mut MAGCOLOREFFECT ) -> super::super::Foundation:: BOOL );
    MagSetColorEffect(hwnd.into_param().abi(), peffect)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetFullscreenColorEffect(peffect: *const MAGCOLOREFFECT) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "magnification.dll""system" fn MagSetFullscreenColorEffect ( peffect : *const MAGCOLOREFFECT ) -> super::super::Foundation:: BOOL );
    MagSetFullscreenColorEffect(peffect)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetFullscreenTransform(maglevel: f32, xoffset: i32, yoffset: i32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "magnification.dll""system" fn MagSetFullscreenTransform ( maglevel : f32 , xoffset : i32 , yoffset : i32 ) -> super::super::Foundation:: BOOL );
    MagSetFullscreenTransform(maglevel, xoffset, yoffset)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn MagSetImageScalingCallback<P0>(hwnd: P0, callback: MagImageScalingCallback) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "magnification.dll""system" fn MagSetImageScalingCallback ( hwnd : super::super::Foundation:: HWND , callback : MagImageScalingCallback ) -> super::super::Foundation:: BOOL );
    MagSetImageScalingCallback(hwnd.into_param().abi(), callback)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetInputTransform<P0>(fenabled: P0, prectsource: *const super::super::Foundation::RECT, prectdest: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "magnification.dll""system" fn MagSetInputTransform ( fenabled : super::super::Foundation:: BOOL , prectsource : *const super::super::Foundation:: RECT , prectdest : *const super::super::Foundation:: RECT ) -> super::super::Foundation:: BOOL );
    MagSetInputTransform(fenabled.into_param().abi(), prectsource, prectdest)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetWindowFilterList<P0>(hwnd: P0, dwfiltermode: MW_FILTERMODE, count: i32, phwnd: *mut super::super::Foundation::HWND) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "magnification.dll""system" fn MagSetWindowFilterList ( hwnd : super::super::Foundation:: HWND , dwfiltermode : MW_FILTERMODE , count : i32 , phwnd : *mut super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    MagSetWindowFilterList(hwnd.into_param().abi(), dwfiltermode, count, phwnd)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetWindowSource<P0>(hwnd: P0, rect: super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "magnification.dll""system" fn MagSetWindowSource ( hwnd : super::super::Foundation:: HWND , rect : super::super::Foundation:: RECT ) -> super::super::Foundation:: BOOL );
    MagSetWindowSource(hwnd.into_param().abi(), ::core::mem::transmute(rect))
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagSetWindowTransform<P0>(hwnd: P0, ptransform: *mut MAGTRANSFORM) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "magnification.dll""system" fn MagSetWindowTransform ( hwnd : super::super::Foundation:: HWND , ptransform : *mut MAGTRANSFORM ) -> super::super::Foundation:: BOOL );
    MagSetWindowTransform(hwnd.into_param().abi(), ptransform)
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagShowSystemCursor<P0>(fshowcursor: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "magnification.dll""system" fn MagShowSystemCursor ( fshowcursor : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    MagShowSystemCursor(fshowcursor.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MagUninitialize() -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "magnification.dll""system" fn MagUninitialize ( ) -> super::super::Foundation:: BOOL );
    MagUninitialize()
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const MS_CLIPAROUNDCURSOR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const MS_INVERTCOLORS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const MS_SHOWMAGNIFIEDCURSOR: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const WC_MAGNIFIER: ::windows::core::PCWSTR = ::windows::core::w!("Magnifier");
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const WC_MAGNIFIERA: ::windows::core::PCSTR = ::windows::core::s!("Magnifier");
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const WC_MAGNIFIERW: ::windows::core::PCWSTR = ::windows::core::w!("Magnifier");
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MW_FILTERMODE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const MW_FILTERMODE_EXCLUDE: MW_FILTERMODE = MW_FILTERMODE(0u32);
#[doc = "*Required features: `\"Win32_UI_Magnification\"`*"]
pub const MW_FILTERMODE_INCLUDE: MW_FILTERMODE = MW_FILTERMODE(1u32);
impl ::core::marker::Copy for MW_FILTERMODE {}
impl ::core::clone::Clone for MW_FILTERMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MW_FILTERMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MW_FILTERMODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MW_FILTERMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MW_FILTERMODE").field(&self.0).finish()
    }
}
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
impl ::windows::core::TypeKind for MAGCOLOREFFECT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MAGCOLOREFFECT {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
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
impl ::windows::core::TypeKind for MAGIMAGEHEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MAGIMAGEHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.format == other.format && self.stride == other.stride && self.offset == other.offset && self.cbSize == other.cbSize
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
impl ::windows::core::TypeKind for MAGTRANSFORM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MAGTRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}
impl ::core::cmp::Eq for MAGTRANSFORM {}
impl ::core::default::Default for MAGTRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_Magnification\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type MagImageScalingCallback = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, srcdata: *mut ::core::ffi::c_void, srcheader: MAGIMAGEHEADER, destdata: *mut ::core::ffi::c_void, destheader: MAGIMAGEHEADER, unclipped: super::super::Foundation::RECT, clipped: super::super::Foundation::RECT, dirty: super::super::Graphics::Gdi::HRGN) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
