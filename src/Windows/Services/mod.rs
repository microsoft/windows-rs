#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Services_Cortana")]
pub mod Cortana;
#[cfg(feature = "Services_Maps")]
pub mod Maps;
#[cfg(feature = "Services_Store")]
pub mod Store;
#[cfg(feature = "Services_TargetedContent")]
pub mod TargetedContent;
