#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_WinRT_Pdf`, `Win32_Graphics_Dxgi`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn PdfCreateRenderer(pdevice: ::windows::runtime::RawPtr, pprenderer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
}
