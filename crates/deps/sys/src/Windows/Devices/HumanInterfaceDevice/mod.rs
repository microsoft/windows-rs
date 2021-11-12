#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HidBooleanControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HidBooleanControlDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HidCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HidCollectionType(pub i32);
impl HidCollectionType {
    pub const Physical: Self = Self(0i32);
    pub const Application: Self = Self(1i32);
    pub const Logical: Self = Self(2i32);
    pub const Report: Self = Self(3i32);
    pub const NamedArray: Self = Self(4i32);
    pub const UsageSwitch: Self = Self(5i32);
    pub const UsageModifier: Self = Self(6i32);
    pub const Other: Self = Self(7i32);
}
#[repr(transparent)]
pub struct HidDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HidFeatureReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HidInputReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HidInputReportReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HidNumericControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HidNumericControlDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HidOutputReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HidReportType(pub i32);
impl HidReportType {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const Feature: Self = Self(2i32);
}
#[repr(transparent)]
pub struct IHidBooleanControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHidBooleanControlDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHidBooleanControlDescription2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHidCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHidDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHidDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHidFeatureReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHidInputReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHidInputReportReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHidNumericControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHidNumericControlDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHidOutputReport(pub *mut ::core::ffi::c_void);
