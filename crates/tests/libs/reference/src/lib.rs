#![cfg(windows)]
#![expect(missing_docs)]

#[expect(non_snake_case, non_upper_case_globals, non_camel_case_types)]
mod bindings;
pub use bindings::*;
