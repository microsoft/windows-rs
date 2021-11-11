#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D`, `Win32_Graphics_Direct3D11`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
    pub fn D3D11On12CreateDevice(pdevice: ::windows::runtime::RawPtr, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, ppcommandqueues: *const ::windows::runtime::RawPtr, numqueues: u32, nodemask: u32, ppdevice: *mut ::windows::runtime::RawPtr, ppimmediatecontext: *mut ::windows::runtime::RawPtr, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows::runtime::HRESULT;
}
