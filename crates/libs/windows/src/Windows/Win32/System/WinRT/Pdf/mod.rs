#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_WinRT_Pdf\"`*"]
#[repr(transparent)]
pub struct IPdfRendererNative(::windows::core::IUnknown);
impl IPdfRendererNative {
    #[doc = "*Required features: `\"Win32_System_WinRT_Pdf\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
    pub unsafe fn RenderPageToSurface<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGISurface>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::POINT>>(&self, pdfpage: Param0, psurface: Param1, offset: Param2, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RenderPageToSurface)(::core::mem::transmute_copy(self), pdfpage.into_param().abi(), psurface.into_param().abi(), offset.into_param().abi(), ::core::mem::transmute(prenderparams)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_Pdf\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn RenderPageToDeviceContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct2D::ID2D1DeviceContext>>(&self, pdfpage: Param0, pd2ddevicecontext: Param1, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RenderPageToDeviceContext)(::core::mem::transmute_copy(self), pdfpage.into_param().abi(), pd2ddevicecontext.into_param().abi(), ::core::mem::transmute(prenderparams)).ok()
    }
}
impl ::core::convert::From<IPdfRendererNative> for ::windows::core::IUnknown {
    fn from(value: IPdfRendererNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPdfRendererNative> for ::windows::core::IUnknown {
    fn from(value: &IPdfRendererNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPdfRendererNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPdfRendererNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPdfRendererNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPdfRendererNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPdfRendererNative {}
impl ::core::fmt::Debug for IPdfRendererNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPdfRendererNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPdfRendererNative {
    type Vtable = IPdfRendererNative_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d9dcd91_d277_4947_8527_07a0daeda94a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfRendererNative_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
    pub RenderPageToSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, psurface: ::windows::core::RawPtr, offset: super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi")))]
    RenderPageToSurface: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub RenderPageToDeviceContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, pd2ddevicecontext: ::windows::core::RawPtr, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    RenderPageToDeviceContext: usize,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Pdf\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct PDF_RENDER_PARAMS {
    pub SourceRect: super::super::super::Graphics::Direct2D::Common::D2D_RECT_F,
    pub DestinationWidth: u32,
    pub DestinationHeight: u32,
    pub BackgroundColor: super::super::super::Graphics::Direct2D::Common::D2D_COLOR_F,
    pub IgnoreHighContrast: super::super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::marker::Copy for PDF_RENDER_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for PDF_RENDER_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::fmt::Debug for PDF_RENDER_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDF_RENDER_PARAMS").field("SourceRect", &self.SourceRect).field("DestinationWidth", &self.DestinationWidth).field("DestinationHeight", &self.DestinationHeight).field("BackgroundColor", &self.BackgroundColor).field("IgnoreHighContrast", &self.IgnoreHighContrast).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
unsafe impl ::windows::core::Abi for PDF_RENDER_PARAMS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::cmp::PartialEq for PDF_RENDER_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDF_RENDER_PARAMS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::cmp::Eq for PDF_RENDER_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::default::Default for PDF_RENDER_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Pdf\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_PDF_CREATE_RENDERER = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<super::super::super::Graphics::Dxgi::IDXGIDevice>, param1: *mut ::core::option::Option<IPdfRendererNative>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WinRT_Pdf\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn PdfCreateRenderer<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGIDevice>>(pdevice: Param0) -> ::windows::core::Result<IPdfRendererNative> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdfCreateRenderer(pdevice: ::windows::core::RawPtr, pprenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        PdfCreateRenderer(pdevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPdfRendererNative>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
