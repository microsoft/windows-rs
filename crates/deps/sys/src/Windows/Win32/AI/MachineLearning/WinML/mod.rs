#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
pub const WINML_TENSOR_DIMENSION_COUNT_MAX: u32 = 4u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub fn MLCreateOperatorRegistry(registry: *mut IMLOperatorRegistry) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub fn WinMLCreateRuntime(runtime: *mut IWinMLRuntime) -> ::windows_sys::core::HRESULT;
}
