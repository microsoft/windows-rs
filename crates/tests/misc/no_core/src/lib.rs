#![cfg(windows)]
// Tests that non-sys bindings can be generated without a depedency on the windows-core crate when feasible.

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]
mod bindings;
