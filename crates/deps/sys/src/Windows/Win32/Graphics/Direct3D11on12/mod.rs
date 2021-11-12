#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D`, `Win32_Graphics_Direct3D11`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
    pub fn D3D11On12CreateDevice(pdevice: ::windows_sys::core::IUnknown, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, ppcommandqueues: *const ::windows_sys::core::IUnknown, numqueues: u32, nodemask: u32, ppdevice: *mut super::Direct3D11::ID3D11Device, ppimmediatecontext: *mut super::Direct3D11::ID3D11DeviceContext, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows_sys::core::HRESULT;
}
