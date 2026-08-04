#![doc = include_str!("../readme.md")]

/// Windows Runtime metadata.
pub static WINRT: &[u8] = include_bytes!("../Windows.winmd");

/// Windows API metadata.
pub static WIN32: &[u8] = include_bytes!("../Windows.Win32.winmd");
