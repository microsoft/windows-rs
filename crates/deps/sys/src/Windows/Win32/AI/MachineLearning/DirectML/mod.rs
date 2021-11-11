#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub fn DMLCreateDevice(d3d12device: ::windows::runtime::RawPtr, flags: DML_CREATE_DEVICE_FLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub fn DMLCreateDevice1(d3d12device: ::windows::runtime::RawPtr, flags: DML_CREATE_DEVICE_FLAGS, minimumfeaturelevel: DML_FEATURE_LEVEL, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
}
