#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub const DML_MINIMUM_BUFFER_TENSOR_ALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub const DML_PERSISTENT_BUFFER_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub const DML_TARGET_VERSION: u32 = 16384u32;
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub const DML_TEMPORARY_BUFFER_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub const DML_TENSOR_DIMENSION_COUNT_MAX: u32 = 5u32;
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub const DML_TENSOR_DIMENSION_COUNT_MAX1: u32 = 8u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub fn DMLCreateDevice(d3d12device: super::super::super::Graphics::Direct3D12::ID3D12Device, flags: DML_CREATE_DEVICE_FLAGS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub fn DMLCreateDevice1(d3d12device: super::super::super::Graphics::Direct3D12::ID3D12Device, flags: DML_CREATE_DEVICE_FLAGS, minimumfeaturelevel: DML_FEATURE_LEVEL, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
