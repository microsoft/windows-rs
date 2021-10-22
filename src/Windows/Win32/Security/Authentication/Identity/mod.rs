#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Security_Authentication_Identity_Core")]
pub mod Core;
#[cfg(feature = "Win32_Security_Authentication_Identity_Provider")]
pub mod Provider;
