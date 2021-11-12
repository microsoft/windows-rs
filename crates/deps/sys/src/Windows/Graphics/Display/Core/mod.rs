#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct HdmiDisplayColorSpace(i32);
pub struct HdmiDisplayHdr2086Metadata(i32);
pub struct HdmiDisplayHdrOption(i32);
pub struct HdmiDisplayInformation(i32);
pub struct HdmiDisplayMode(i32);
pub struct HdmiDisplayPixelEncoding(i32);
pub struct IHdmiDisplayInformation(pub *mut ::core::ffi::c_void);
pub struct IHdmiDisplayInformationStatics(pub *mut ::core::ffi::c_void);
pub struct IHdmiDisplayMode(pub *mut ::core::ffi::c_void);
pub struct IHdmiDisplayMode2(pub *mut ::core::ffi::c_void);
