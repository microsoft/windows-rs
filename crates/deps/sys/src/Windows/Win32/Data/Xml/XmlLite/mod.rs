#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CreateXmlReader();
    fn CreateXmlReaderInputWithEncodingCodePage();
    fn CreateXmlReaderInputWithEncodingName();
    fn CreateXmlWriter();
    fn CreateXmlWriterOutputWithEncodingCodePage();
    fn CreateXmlWriterOutputWithEncodingName();
    fn DtdProcessing();
    fn IXmlReader();
    fn IXmlResolver();
    fn IXmlWriter();
    fn IXmlWriterLite();
    fn XmlConformanceLevel();
    fn XmlError();
    fn XmlNodeType();
    fn XmlReadState();
    fn XmlReaderProperty();
    fn XmlStandalone();
    fn XmlWriterProperty();
    fn _IID_IXmlReader();
    fn _IID_IXmlResolver();
    fn _IID_IXmlWriter();
}
