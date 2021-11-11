#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Media_Capture_Core")]
pub mod Core;
#[cfg(feature = "Media_Capture_Frames")]
pub mod Frames;
#[link(name = "windows")]
extern "system" {}
