#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`, `Win32_Foundation`, `Win32_Graphics_Direct3D`, `Win32_Graphics_Dxgi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi"))]
    pub fn D3D11CreateDevice(padapter: ::windows::runtime::RawPtr, drivertype: super::Direct3D::D3D_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: D3D11_CREATE_DEVICE_FLAG, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, ppdevice: *mut ::windows::runtime::RawPtr, pfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL, ppimmediatecontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`, `Win32_Foundation`, `Win32_Graphics_Direct3D`, `Win32_Graphics_Dxgi`, `Win32_Graphics_Dxgi_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
    pub fn D3D11CreateDeviceAndSwapChain(
        padapter: ::windows::runtime::RawPtr,
        drivertype: super::Direct3D::D3D_DRIVER_TYPE,
        software: super::super::Foundation::HINSTANCE,
        flags: D3D11_CREATE_DEVICE_FLAG,
        pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL,
        featurelevels: u32,
        sdkversion: u32,
        pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut ::windows::runtime::RawPtr,
        ppdevice: *mut ::windows::runtime::RawPtr,
        pfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL,
        ppimmediatecontext: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3DDisassemble11Trace(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ptrace: ::windows::runtime::RawPtr, startstep: u32, numsteps: u32, flags: u32, ppdisassembly: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT(pdevicecontext: ::windows::runtime::RawPtr, pdesc: *const D3DX11_FFT_DESC, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT1DComplex(pdevicecontext: ::windows::runtime::RawPtr, x: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT1DReal(pdevicecontext: ::windows::runtime::RawPtr, x: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT2DComplex(pdevicecontext: ::windows::runtime::RawPtr, x: u32, y: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT2DReal(pdevicecontext: ::windows::runtime::RawPtr, x: u32, y: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT3DComplex(pdevicecontext: ::windows::runtime::RawPtr, x: u32, y: u32, z: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT3DReal(pdevicecontext: ::windows::runtime::RawPtr, x: u32, y: u32, z: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateScan(pdevicecontext: ::windows::runtime::RawPtr, maxelementscansize: u32, maxscancount: u32, ppscan: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateSegmentedScan(pdevicecontext: ::windows::runtime::RawPtr, maxelementscansize: u32, ppscan: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
}
