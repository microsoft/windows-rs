#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_BeginEvent();
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`*"]
    pub fn D3DPERF_EndEvent();
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`*"]
    pub fn D3DPERF_GetStatus();
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_QueryRepeatFrame();
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_SetMarker();
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`*"]
    pub fn D3DPERF_SetOptions();
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_SetRegion();
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`*"]
    pub fn Direct3DCreate9();
    #[doc = "*Required features: `Win32_Graphics_Direct3D9`*"]
    pub fn Direct3DCreate9Ex();
}
