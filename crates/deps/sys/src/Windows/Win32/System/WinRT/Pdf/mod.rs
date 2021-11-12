#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn PdfCreateRenderer(pdevice: super::super::super::Graphics::Dxgi::IDXGIDevice, pprenderer: *mut IPdfRendererNative) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct IPdfRendererNative(pub *mut ::core::ffi::c_void);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
#[repr(C)]
pub struct PDF_RENDER_PARAMS(i32);
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_PDF_CREATE_RENDERER = unsafe extern "system" fn(param0: super::super::super::Graphics::Dxgi::IDXGIDevice, param1: *mut IPdfRendererNative) -> ::windows_sys::core::HRESULT;
