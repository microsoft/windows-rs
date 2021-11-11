#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Data_Xml_Dom")]
pub mod Dom;
#[cfg(feature = "Data_Xml_Xsl")]
pub mod Xsl;
#[link(name = "windows")]
extern "system" {}
