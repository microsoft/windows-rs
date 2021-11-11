#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn CreateDXGIFactory();
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn CreateDXGIFactory1();
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn CreateDXGIFactory2();
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn DXGIDeclareAdapterRemovalSupport();
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn DXGIGetDebugInterface1();
}
