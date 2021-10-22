#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Storage_Packaging_Appx")]
pub mod Appx;
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub mod Opc;
