#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_WinRT_Direct3D11`, `Win32_Graphics_Dxgi`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice: ::windows::runtime::RawPtr, graphicsdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WinRT_Direct3D11`, `Win32_Graphics_Dxgi`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn CreateDirect3D11SurfaceFromDXGISurface(dgxisurface: ::windows::runtime::RawPtr, graphicssurface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
}
