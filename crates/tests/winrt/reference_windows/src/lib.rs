#![cfg(windows)]
#[expect(non_snake_case, non_camel_case_types, clippy::all)]
mod bindings;
pub use bindings::*;
