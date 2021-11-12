#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct HingeState(i32);
pub struct ITwoPanelHingedDevicePosturePreview(pub *mut ::core::ffi::c_void);
pub struct ITwoPanelHingedDevicePosturePreviewReading(pub *mut ::core::ffi::c_void);
pub struct ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ITwoPanelHingedDevicePosturePreviewStatics(pub *mut ::core::ffi::c_void);
pub struct TwoPanelHingedDevicePosturePreview(i32);
pub struct TwoPanelHingedDevicePosturePreviewReading(i32);
pub struct TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(i32);
