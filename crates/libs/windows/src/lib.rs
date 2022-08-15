/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(html_no_source)]
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clashing_extern_declarations, unused_variables, dead_code, clippy::all)]

extern crate self as windows;
mod Windows;
pub mod core;
pub use Windows::*;
