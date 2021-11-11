#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Printers_Extensions")]
pub mod Extensions;
#[link(name = "windows")]
extern "system" {
    fn IIppAttributeError();
    fn IIppAttributeValue();
    fn IIppAttributeValueStatics();
    fn IIppIntegerRange();
    fn IIppIntegerRangeFactory();
    fn IIppPrintDevice();
    fn IIppResolution();
    fn IIppResolutionFactory();
    fn IIppSetAttributesResult();
    fn IIppTextWithLanguage();
    fn IIppTextWithLanguageFactory();
    fn IPrint3DDevice();
    fn IPrint3DDeviceStatics();
    fn IPrintSchema();
    fn IppAttributeError();
    fn IppAttributeErrorReason();
    fn IppAttributeValue();
    fn IppAttributeValueKind();
    fn IppIntegerRange();
    fn IppPrintDevice();
    fn IppResolution();
    fn IppResolutionUnit();
    fn IppSetAttributesResult();
    fn IppTextWithLanguage();
    fn Print3DDevice();
    fn PrintSchema();
    fn PrintersContract();
}
