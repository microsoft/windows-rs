#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn HdmiDisplayColorSpace();
    fn HdmiDisplayHdr2086Metadata();
    fn HdmiDisplayHdrOption();
    fn HdmiDisplayInformation();
    fn HdmiDisplayMode();
    fn HdmiDisplayPixelEncoding();
    fn IHdmiDisplayInformation();
    fn IHdmiDisplayInformationStatics();
    fn IHdmiDisplayMode();
    fn IHdmiDisplayMode2();
}
