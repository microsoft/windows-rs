#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Media_DirectShow_Xml")]
pub mod Xml;
#[link(name = "windows")]
extern "system" {
    fn AMGetErrorTextA();
    fn AMGetErrorTextW();
}
