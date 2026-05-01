#![allow(non_snake_case)]
#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
include!("windows_main.rs");
