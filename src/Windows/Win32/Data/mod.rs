#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Data_HtmlHelp")]
pub mod HtmlHelp;
#[cfg(feature = "Win32_Data_RightsManagement")]
pub mod RightsManagement;
#[cfg(feature = "Win32_Data_Xml")]
pub mod Xml;
