#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::marker::Copy for HingeState {}
impl ::core::clone::Clone for HingeState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITwoPanelHingedDevicePosturePreview {}
impl ::core::clone::Clone for ITwoPanelHingedDevicePosturePreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreviewReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITwoPanelHingedDevicePosturePreviewReading {}
impl ::core::clone::Clone for ITwoPanelHingedDevicePosturePreviewReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {}
impl ::core::clone::Clone for ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITwoPanelHingedDevicePosturePreviewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITwoPanelHingedDevicePosturePreviewStatics {}
impl ::core::clone::Clone for ITwoPanelHingedDevicePosturePreviewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TwoPanelHingedDevicePosturePreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TwoPanelHingedDevicePosturePreview {}
impl ::core::clone::Clone for TwoPanelHingedDevicePosturePreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TwoPanelHingedDevicePosturePreviewReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TwoPanelHingedDevicePosturePreviewReading {}
impl ::core::clone::Clone for TwoPanelHingedDevicePosturePreviewReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {}
impl ::core::clone::Clone for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
