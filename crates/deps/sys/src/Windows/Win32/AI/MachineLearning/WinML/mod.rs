#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub fn MLCreateOperatorRegistry(registry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_AI_MachineLearning_WinML`*"]
    pub fn WinMLCreateRuntime(runtime: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
}
