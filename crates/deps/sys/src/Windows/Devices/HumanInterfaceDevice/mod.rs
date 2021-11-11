#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn HidBooleanControl();
    fn HidBooleanControlDescription();
    fn HidCollection();
    fn HidCollectionType();
    fn HidDevice();
    fn HidFeatureReport();
    fn HidInputReport();
    fn HidInputReportReceivedEventArgs();
    fn HidNumericControl();
    fn HidNumericControlDescription();
    fn HidOutputReport();
    fn HidReportType();
    fn IHidBooleanControl();
    fn IHidBooleanControlDescription();
    fn IHidBooleanControlDescription2();
    fn IHidCollection();
    fn IHidDevice();
    fn IHidDeviceStatics();
    fn IHidFeatureReport();
    fn IHidInputReport();
    fn IHidInputReportReceivedEventArgs();
    fn IHidNumericControl();
    fn IHidNumericControlDescription();
    fn IHidOutputReport();
}
