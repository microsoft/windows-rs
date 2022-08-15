/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]
#![doc(html_no_source)]
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clashing_extern_declarations)]

extern crate self as windows_sys;
mod Windows;
pub mod core;
pub use Windows::*;
