#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_AI_MachineLearning_DirectML")]
pub mod DirectML;
#[cfg(feature = "Win32_AI_MachineLearning_WinML")]
pub mod WinML;
