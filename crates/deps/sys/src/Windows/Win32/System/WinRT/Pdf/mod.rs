#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_WinRT_Pdf`, `Win32_Graphics_Dxgi`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn PdfCreateRenderer(pdevice: super::super::super::Graphics::Dxgi::IDXGIDevice, pprenderer: *mut IPdfRendererNative) -> ::windows_sys::core::HRESULT;
}
pub struct IPdfRendererNative(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct PDF_RENDER_PARAMS(i32);
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub struct PFN_PDF_CREATE_RENDERER(i32);
