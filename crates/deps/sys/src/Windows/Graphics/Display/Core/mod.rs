#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HdmiDisplayColorSpace(pub i32);
impl HdmiDisplayColorSpace {
    pub const RgbLimited: HdmiDisplayColorSpace = HdmiDisplayColorSpace(0i32);
    pub const RgbFull: HdmiDisplayColorSpace = HdmiDisplayColorSpace(1i32);
    pub const BT2020: HdmiDisplayColorSpace = HdmiDisplayColorSpace(2i32);
    pub const BT709: HdmiDisplayColorSpace = HdmiDisplayColorSpace(3i32);
}
#[repr(C)]
pub struct HdmiDisplayHdr2086Metadata(i32);
#[repr(transparent)]
pub struct HdmiDisplayHdrOption(pub i32);
impl HdmiDisplayHdrOption {
    pub const None: HdmiDisplayHdrOption = HdmiDisplayHdrOption(0i32);
    pub const EotfSdr: HdmiDisplayHdrOption = HdmiDisplayHdrOption(1i32);
    pub const Eotf2084: HdmiDisplayHdrOption = HdmiDisplayHdrOption(2i32);
    pub const DolbyVisionLowLatency: HdmiDisplayHdrOption = HdmiDisplayHdrOption(3i32);
}
#[repr(transparent)]
pub struct HdmiDisplayInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HdmiDisplayMode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HdmiDisplayPixelEncoding(pub i32);
impl HdmiDisplayPixelEncoding {
    pub const Rgb444: HdmiDisplayPixelEncoding = HdmiDisplayPixelEncoding(0i32);
    pub const Ycc444: HdmiDisplayPixelEncoding = HdmiDisplayPixelEncoding(1i32);
    pub const Ycc422: HdmiDisplayPixelEncoding = HdmiDisplayPixelEncoding(2i32);
    pub const Ycc420: HdmiDisplayPixelEncoding = HdmiDisplayPixelEncoding(3i32);
}
#[repr(transparent)]
pub struct IHdmiDisplayInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHdmiDisplayInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHdmiDisplayMode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHdmiDisplayMode2(pub *mut ::core::ffi::c_void);
