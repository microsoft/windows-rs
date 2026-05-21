#![cfg(windows)]
// Tests that non-sys bindings can be generated with just a depedency on the windows-core crate when feasible.

#[expect(non_snake_case, non_camel_case_types, dead_code)]
mod bindings;
