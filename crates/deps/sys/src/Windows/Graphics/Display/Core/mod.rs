#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct HdmiDisplayColorSpace(i32);
#[repr(C)]
pub struct HdmiDisplayHdr2086Metadata(i32);
#[repr(C)]
pub struct HdmiDisplayHdrOption(i32);
#[repr(transparent)]
pub struct HdmiDisplayInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HdmiDisplayMode(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct HdmiDisplayPixelEncoding(i32);
#[repr(transparent)]
pub struct IHdmiDisplayInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHdmiDisplayInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHdmiDisplayMode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHdmiDisplayMode2(pub *mut ::core::ffi::c_void);
