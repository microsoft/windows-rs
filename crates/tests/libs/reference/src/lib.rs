#![cfg(windows)]
#![allow(
    missing_docs,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::missing_transmute_annotations
)]

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;
pub use bindings::*;
