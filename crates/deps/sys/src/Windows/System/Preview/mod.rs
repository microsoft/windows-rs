#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HingeState(pub i32);
impl HingeState {
    pub const Unknown: Self = Self(0i32);
    pub const Closed: Self = Self(1i32);
    pub const Concave: Self = Self(2i32);
    pub const Flat: Self = Self(3i32);
    pub const Convex: Self = Self(4i32);
    pub const Full: Self = Self(5i32);
}
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
