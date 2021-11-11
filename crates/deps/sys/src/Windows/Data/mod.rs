#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Data_Html")]
pub mod Html;
#[cfg(feature = "Data_Json")]
pub mod Json;
#[cfg(feature = "Data_Pdf")]
pub mod Pdf;
#[cfg(feature = "Data_Text")]
pub mod Text;
#[cfg(feature = "Data_Xml")]
pub mod Xml;
#[link(name = "windows")]
extern "system" {}
