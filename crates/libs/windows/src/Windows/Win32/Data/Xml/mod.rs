#[cfg(feature = "Win32_Data_Xml_MsXml")]
pub mod MsXml;
#[cfg(feature = "Win32_Data_Xml_XmlLite")]
pub mod XmlLite;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
