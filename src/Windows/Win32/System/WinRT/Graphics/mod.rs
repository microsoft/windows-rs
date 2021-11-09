#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_System_WinRT_Graphics_Capture")]
pub mod Capture;
#[cfg(feature = "Win32_System_WinRT_Graphics_Direct2D")]
pub mod Direct2D;
#[cfg(feature = "Win32_System_WinRT_Graphics_Imaging")]
pub mod Imaging;
