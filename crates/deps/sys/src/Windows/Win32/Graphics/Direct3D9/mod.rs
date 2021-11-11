#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_BeginEvent(col: u32, wszname: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`*"]
    pub fn D3DPERF_EndEvent() -> i32;
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`*"]
    pub fn D3DPERF_GetStatus() -> u32;
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_QueryRepeatFrame() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_SetMarker(col: u32, wszname: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`*"]
    pub fn D3DPERF_SetOptions(dwoptions: u32);
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_SetRegion(col: u32, wszname: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`*"]
    pub fn Direct3DCreate9(sdkversion: u32) -> ::core::option::Option<IDirect3D9>;
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`*"]
    pub fn Direct3DCreate9Ex(sdkversion: u32, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
}
