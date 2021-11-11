#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_AI_MachineLearning_DirectML")]
pub mod DirectML;
#[cfg(feature = "Win32_AI_MachineLearning_WinML")]
pub mod WinML;
#[link(name = "windows")]
extern "system" {}
