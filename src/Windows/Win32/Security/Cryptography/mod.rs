#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Security_Cryptography_Certificates")]
pub mod Certificates;
#[cfg(feature = "Win32_Security_Cryptography_Core")]
pub mod Core;
