#![cfg_attr(
    all(
        feature = "Win32_Security",
        feature = "Win32_System_Threading",
        feature = "Win32_UI_WindowsAndMessaging",
    ),
    doc = include_str!("../README.md")
)]

// fallback if not all features are enabled
#![cfg_attr(
    not(all(
        feature = "Win32_Security",
        feature = "Win32_System_Threading",
        feature = "Win32_UI_WindowsAndMessaging",
    )),
    doc = "Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>\n\n[Feature search](https://microsoft.github.io/windows-rs/features/#/0.58.0)",
)]

#![no_std]
#![doc(html_no_source)]
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, missing_docs, clippy::all)]
#![cfg_attr(not(feature = "docs"), doc(hidden))]

#[allow(unused_extern_crates)]
extern crate self as windows_sys;

pub mod core;

include!("Windows/mod.rs");
