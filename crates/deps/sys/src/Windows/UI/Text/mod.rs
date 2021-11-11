#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_Text_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
