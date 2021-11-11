#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Graphics_Display_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
