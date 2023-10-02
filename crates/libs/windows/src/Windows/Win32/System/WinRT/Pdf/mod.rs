#[doc = "Required features: `\"Win32_Graphics_Dxgi\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn PdfCreateRenderer<P0>(pdevice: P0) -> ::windows_core::Result<IPdfRendererNative>
where
    P0: ::windows_core::IntoParam<super::super::super::Graphics::Dxgi::IDXGIDevice>,
{
    ::windows_targets::link!("windows.data.pdf.dll" "system" fn PdfCreateRenderer(pdevice : * mut::core::ffi::c_void, pprenderer : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PdfCreateRenderer(pdevice.into_param().abi(), &mut result__).from_abi(result__)
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPdfRendererNative(::windows_core::IUnknown);
impl IPdfRendererNative {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
    pub unsafe fn RenderPageToSurface<P0, P1>(&self, pdfpage: P0, psurface: P1, offset: super::super::super::Foundation::POINT, prenderparams: ::core::option::Option<*const PDF_RENDER_PARAMS>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Dxgi::IDXGISurface>,
    {
        (::windows_core::Interface::vtable(self).RenderPageToSurface)(::windows_core::Interface::as_raw(self), pdfpage.into_param().abi(), psurface.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(prenderparams.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn RenderPageToDeviceContext<P0, P1>(&self, pdfpage: P0, pd2ddevicecontext: P1, prenderparams: ::core::option::Option<*const PDF_RENDER_PARAMS>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Direct2D::ID2D1DeviceContext>,
    {
        (::windows_core::Interface::vtable(self).RenderPageToDeviceContext)(::windows_core::Interface::as_raw(self), pdfpage.into_param().abi(), pd2ddevicecontext.into_param().abi(), ::core::mem::transmute(prenderparams.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPdfRendererNative, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPdfRendererNative {
    type Vtable = IPdfRendererNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPdfRendererNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d9dcd91_d277_4947_8527_07a0daeda94a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfRendererNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
    pub RenderPageToSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, psurface: *mut ::core::ffi::c_void, offset: super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi")))]
    RenderPageToSurface: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub RenderPageToDeviceContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, pd2ddevicecontext: *mut ::core::ffi::c_void, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    RenderPageToDeviceContext: usize,
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
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
impl ::windows_core::TypeKind for PDF_RENDER_PARAMS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::cmp::PartialEq for PDF_RENDER_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.SourceRect == other.SourceRect && self.DestinationWidth == other.DestinationWidth && self.DestinationHeight == other.DestinationHeight && self.BackgroundColor == other.BackgroundColor && self.IgnoreHighContrast == other.IgnoreHighContrast
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
#[doc = "Required features: `\"Win32_Graphics_Dxgi\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_PDF_CREATE_RENDERER = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<super::super::super::Graphics::Dxgi::IDXGIDevice>, param1: *mut ::core::option::Option<IPdfRendererNative>) -> ::windows_core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
