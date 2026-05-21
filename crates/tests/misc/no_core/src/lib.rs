#![cfg(windows)]
// Tests that non-sys bindings can be generated without a depedency on the windows-core crate when feasible.

#[expect(non_snake_case, dead_code)]
mod bindings;
