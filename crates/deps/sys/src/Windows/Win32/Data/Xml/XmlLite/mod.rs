#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CreateXmlReader();
    fn CreateXmlReaderInputWithEncodingCodePage();
    fn CreateXmlReaderInputWithEncodingName();
    fn CreateXmlWriter();
    fn CreateXmlWriterOutputWithEncodingCodePage();
    fn CreateXmlWriterOutputWithEncodingName();
}
