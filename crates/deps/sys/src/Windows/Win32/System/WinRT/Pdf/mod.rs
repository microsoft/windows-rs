#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn PdfCreateRenderer(pdevice: super::super::super::Graphics::Dxgi::IDXGIDevice, pprenderer: *mut IPdfRendererNative) -> ::windows_sys::core::HRESULT;
}
pub type IPdfRendererNative = *mut ::core::ffi::c_void;
#[repr(C)]
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
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_PDF_CREATE_RENDERER = unsafe extern "system" fn(param0: super::super::super::Graphics::Dxgi::IDXGIDevice, param1: *mut IPdfRendererNative) -> ::windows_sys::core::HRESULT;
