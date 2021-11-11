#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub fn DMLCreateDevice();
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub fn DMLCreateDevice1();
}
