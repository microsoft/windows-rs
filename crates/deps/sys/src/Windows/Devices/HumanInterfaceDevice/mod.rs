#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HidBooleanControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HidBooleanControl {}
impl ::core::clone::Clone for HidBooleanControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HidBooleanControlDescription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HidBooleanControlDescription {}
impl ::core::clone::Clone for HidBooleanControlDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HidCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HidCollection {}
impl ::core::clone::Clone for HidCollection {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for HidCollectionType {}
impl ::core::clone::Clone for HidCollectionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HidDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HidDevice {}
impl ::core::clone::Clone for HidDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HidFeatureReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HidFeatureReport {}
impl ::core::clone::Clone for HidFeatureReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HidInputReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HidInputReport {}
impl ::core::clone::Clone for HidInputReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HidInputReportReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HidInputReportReceivedEventArgs {}
impl ::core::clone::Clone for HidInputReportReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HidNumericControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HidNumericControl {}
impl ::core::clone::Clone for HidNumericControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HidNumericControlDescription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HidNumericControlDescription {}
impl ::core::clone::Clone for HidNumericControlDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HidOutputReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HidOutputReport {}
impl ::core::clone::Clone for HidOutputReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HidReportType(pub i32);
impl HidReportType {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const Feature: Self = Self(2i32);
}
impl ::core::marker::Copy for HidReportType {}
impl ::core::clone::Clone for HidReportType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidBooleanControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidBooleanControl {}
impl ::core::clone::Clone for IHidBooleanControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidBooleanControlDescription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidBooleanControlDescription {}
impl ::core::clone::Clone for IHidBooleanControlDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidBooleanControlDescription2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidBooleanControlDescription2 {}
impl ::core::clone::Clone for IHidBooleanControlDescription2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidCollection {}
impl ::core::clone::Clone for IHidCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidDevice {}
impl ::core::clone::Clone for IHidDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidDeviceStatics {}
impl ::core::clone::Clone for IHidDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidFeatureReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidFeatureReport {}
impl ::core::clone::Clone for IHidFeatureReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidInputReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidInputReport {}
impl ::core::clone::Clone for IHidInputReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidInputReportReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidInputReportReceivedEventArgs {}
impl ::core::clone::Clone for IHidInputReportReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidNumericControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidNumericControl {}
impl ::core::clone::Clone for IHidNumericControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidNumericControlDescription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidNumericControlDescription {}
impl ::core::clone::Clone for IHidNumericControlDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHidOutputReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHidOutputReport {}
impl ::core::clone::Clone for IHidOutputReport {
    fn clone(&self) -> Self {
        *self
    }
}
