#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Pdf`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPdfRendererNative(pub ::windows::runtime::IUnknown);
impl IPdfRendererNative {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
    #[doc = "*Required features: `Win32_System_WinRT_Pdf`, `Win32_Foundation`, `Win32_Graphics_Direct2D_Common`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn RenderPageToSurface<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGISurface>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::POINT>>(&self, pdfpage: Param0, psurface: Param1, offset: Param2, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pdfpage.into_param().abi(), psurface.into_param().abi(), offset.into_param().abi(), ::core::mem::transmute(prenderparams)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common"))]
    #[doc = "*Required features: `Win32_System_WinRT_Pdf`, `Win32_Foundation`, `Win32_Graphics_Direct2D`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn RenderPageToDeviceContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct2D::ID2D1DeviceContext>>(&self, pdfpage: Param0, pd2ddevicecontext: Param1, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pdfpage.into_param().abi(), pd2ddevicecontext.into_param().abi(), ::core::mem::transmute(prenderparams)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPdfRendererNative {
    type Vtable = IPdfRendererNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2107493777, 53879, 18759, [133, 39, 7, 160, 218, 237, 169, 74]);
}
impl ::core::convert::From<IPdfRendererNative> for ::windows::runtime::IUnknown {
    fn from(value: IPdfRendererNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPdfRendererNative> for ::windows::runtime::IUnknown {
    fn from(value: &IPdfRendererNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPdfRendererNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPdfRendererNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfRendererNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdfpage: ::windows::runtime::RawPtr, psurface: ::windows::runtime::RawPtr, offset: super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdfpage: ::windows::runtime::RawPtr, pd2ddevicecontext: ::windows::runtime::RawPtr, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common")))] usize,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
#[doc = "*Required features: `Win32_System_WinRT_Pdf`, `Win32_Foundation`, `Win32_Graphics_Direct2D_Common`*"]
pub struct PDF_RENDER_PARAMS {
    pub SourceRect: super::super::super::Graphics::Direct2D::Common::D2D_RECT_F,
    pub DestinationWidth: u32,
    pub DestinationHeight: u32,
    pub BackgroundColor: super::super::super::Graphics::Direct2D::Common::D2D_COLOR_F,
    pub IgnoreHighContrast: super::super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl PDF_RENDER_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::default::Default for PDF_RENDER_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::fmt::Debug for PDF_RENDER_PARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDF_RENDER_PARAMS").field("SourceRect", &self.SourceRect).field("DestinationWidth", &self.DestinationWidth).field("DestinationHeight", &self.DestinationHeight).field("BackgroundColor", &self.BackgroundColor).field("IgnoreHighContrast", &self.IgnoreHighContrast).finish()
    }
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
unsafe impl ::windows::runtime::Abi for PDF_RENDER_PARAMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT_Pdf`, `Win32_Graphics_Dxgi`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_PDF_CREATE_RENDERER = unsafe extern "system" fn(param0: ::windows::runtime::RawPtr, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_System_WinRT_Pdf`, `Win32_Graphics_Dxgi`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn PdfCreateRenderer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGIDevice>>(pdevice: Param0) -> ::windows::runtime::Result<IPdfRendererNative> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdfCreateRenderer(pdevice: ::windows::runtime::RawPtr, pprenderer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IPdfRendererNative as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        PdfCreateRenderer(pdevice.into_param().abi(), &mut result__).from_abi::<IPdfRendererNative>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
