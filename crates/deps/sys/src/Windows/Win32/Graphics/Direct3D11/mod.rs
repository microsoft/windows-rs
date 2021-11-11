#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`, `Win32_Foundation`, `Win32_Graphics_Direct3D`, `Win32_Graphics_Dxgi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi"))]
    pub fn D3D11CreateDevice();
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`, `Win32_Foundation`, `Win32_Graphics_Direct3D`, `Win32_Graphics_Dxgi`, `Win32_Graphics_Dxgi_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
    pub fn D3D11CreateDeviceAndSwapChain();
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3DDisassemble11Trace();
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT();
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT1DComplex();
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT1DReal();
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT2DComplex();
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT2DReal();
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT3DComplex();
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT3DReal();
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateScan();
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateSegmentedScan();
}
