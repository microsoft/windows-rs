#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct HidBooleanControl(i32);
pub struct HidBooleanControlDescription(i32);
pub struct HidCollection(i32);
pub struct HidCollectionType(i32);
pub struct HidDevice(i32);
pub struct HidFeatureReport(i32);
pub struct HidInputReport(i32);
pub struct HidInputReportReceivedEventArgs(i32);
pub struct HidNumericControl(i32);
pub struct HidNumericControlDescription(i32);
pub struct HidOutputReport(i32);
pub struct HidReportType(i32);
pub struct IHidBooleanControl(pub *mut ::core::ffi::c_void);
pub struct IHidBooleanControlDescription(pub *mut ::core::ffi::c_void);
pub struct IHidBooleanControlDescription2(pub *mut ::core::ffi::c_void);
pub struct IHidCollection(pub *mut ::core::ffi::c_void);
pub struct IHidDevice(pub *mut ::core::ffi::c_void);
pub struct IHidDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct IHidFeatureReport(pub *mut ::core::ffi::c_void);
pub struct IHidInputReport(pub *mut ::core::ffi::c_void);
pub struct IHidInputReportReceivedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IHidNumericControl(pub *mut ::core::ffi::c_void);
pub struct IHidNumericControlDescription(pub *mut ::core::ffi::c_void);
pub struct IHidOutputReport(pub *mut ::core::ffi::c_void);
