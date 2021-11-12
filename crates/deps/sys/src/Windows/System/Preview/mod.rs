#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct HingeState(i32);
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreviewReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreviewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TwoPanelHingedDevicePosturePreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TwoPanelHingedDevicePosturePreviewReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
