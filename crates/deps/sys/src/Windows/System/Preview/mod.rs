#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn HingeState();
    fn ITwoPanelHingedDevicePosturePreview();
    fn ITwoPanelHingedDevicePosturePreviewReading();
    fn ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs();
    fn ITwoPanelHingedDevicePosturePreviewStatics();
    fn TwoPanelHingedDevicePosturePreview();
    fn TwoPanelHingedDevicePosturePreviewReading();
    fn TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs();
}
