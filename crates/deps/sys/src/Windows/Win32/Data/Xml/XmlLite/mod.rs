#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateXmlReader();
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateXmlReaderInputWithEncodingCodePage();
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateXmlReaderInputWithEncodingName();
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateXmlWriter();
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateXmlWriterOutputWithEncodingCodePage();
    #[doc = "*Required features: `Win32_Data_Xml_XmlLite`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateXmlWriterOutputWithEncodingName();
}
