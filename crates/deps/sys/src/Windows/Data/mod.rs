#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
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
