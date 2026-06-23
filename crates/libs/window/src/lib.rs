#![doc = include_str!("../readme.md")]

#[expect(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]
mod bindings;
mod window;

pub use window::{Window, WindowBuilder, pump, quit, run, run_with};
pub use windows_core::Result;
