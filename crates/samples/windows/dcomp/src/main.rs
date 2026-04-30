#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
include!("windows_main.rs");
