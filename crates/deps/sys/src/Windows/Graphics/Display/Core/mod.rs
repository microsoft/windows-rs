#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HdmiDisplayColorSpace(pub i32);
impl HdmiDisplayColorSpace {
    pub const RgbLimited: Self = Self(0i32);
    pub const RgbFull: Self = Self(1i32);
    pub const BT2020: Self = Self(2i32);
    pub const BT709: Self = Self(3i32);
}
#[repr(C)]
pub struct HdmiDisplayHdr2086Metadata(i32);
#[repr(transparent)]
pub struct HdmiDisplayHdrOption(pub i32);
impl HdmiDisplayHdrOption {
    pub const None: Self = Self(0i32);
    pub const EotfSdr: Self = Self(1i32);
    pub const Eotf2084: Self = Self(2i32);
    pub const DolbyVisionLowLatency: Self = Self(3i32);
}
#[repr(transparent)]
pub struct HdmiDisplayInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HdmiDisplayMode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HdmiDisplayPixelEncoding(pub i32);
impl HdmiDisplayPixelEncoding {
    pub const Rgb444: Self = Self(0i32);
    pub const Ycc444: Self = Self(1i32);
    pub const Ycc422: Self = Self(2i32);
    pub const Ycc420: Self = Self(3i32);
}
#[repr(transparent)]
pub struct IHdmiDisplayInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHdmiDisplayInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHdmiDisplayMode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHdmiDisplayMode2(pub *mut ::core::ffi::c_void);
