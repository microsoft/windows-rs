#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COMDLG_FILTERSPEC {
    pub pszName: super::super::super::Foundation::PWSTR,
    pub pszSpec: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMDLG_FILTERSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMDLG_FILTERSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DEVICE_SCALE_FACTOR(pub i32);
pub const DEVICE_SCALE_FACTOR_INVALID: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(0i32);
pub const SCALE_100_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(100i32);
pub const SCALE_120_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(120i32);
pub const SCALE_125_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(125i32);
pub const SCALE_140_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(140i32);
pub const SCALE_150_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(150i32);
pub const SCALE_160_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(160i32);
pub const SCALE_175_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(175i32);
pub const SCALE_180_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(180i32);
pub const SCALE_200_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(200i32);
pub const SCALE_225_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(225i32);
pub const SCALE_250_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(250i32);
pub const SCALE_300_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(300i32);
pub const SCALE_350_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(350i32);
pub const SCALE_400_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(400i32);
pub const SCALE_450_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(450i32);
pub const SCALE_500_PERCENT: DEVICE_SCALE_FACTOR = DEVICE_SCALE_FACTOR(500i32);
impl ::core::marker::Copy for DEVICE_SCALE_FACTOR {}
impl ::core::clone::Clone for DEVICE_SCALE_FACTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectArray(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectArray {}
impl ::core::clone::Clone for IObjectArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectCollection {}
impl ::core::clone::Clone for IObjectCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ITEMIDLIST {
    pub mkid: SHITEMID,
}
impl ::core::marker::Copy for ITEMIDLIST {}
impl ::core::clone::Clone for ITEMIDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PERCEIVED(pub i32);
pub const PERCEIVED_TYPE_FIRST: PERCEIVED = PERCEIVED(-3i32);
pub const PERCEIVED_TYPE_CUSTOM: PERCEIVED = PERCEIVED(-3i32);
pub const PERCEIVED_TYPE_UNSPECIFIED: PERCEIVED = PERCEIVED(-2i32);
pub const PERCEIVED_TYPE_FOLDER: PERCEIVED = PERCEIVED(-1i32);
pub const PERCEIVED_TYPE_UNKNOWN: PERCEIVED = PERCEIVED(0i32);
pub const PERCEIVED_TYPE_TEXT: PERCEIVED = PERCEIVED(1i32);
pub const PERCEIVED_TYPE_IMAGE: PERCEIVED = PERCEIVED(2i32);
pub const PERCEIVED_TYPE_AUDIO: PERCEIVED = PERCEIVED(3i32);
pub const PERCEIVED_TYPE_VIDEO: PERCEIVED = PERCEIVED(4i32);
pub const PERCEIVED_TYPE_COMPRESSED: PERCEIVED = PERCEIVED(5i32);
pub const PERCEIVED_TYPE_DOCUMENT: PERCEIVED = PERCEIVED(6i32);
pub const PERCEIVED_TYPE_SYSTEM: PERCEIVED = PERCEIVED(7i32);
pub const PERCEIVED_TYPE_APPLICATION: PERCEIVED = PERCEIVED(8i32);
pub const PERCEIVED_TYPE_GAMEMEDIA: PERCEIVED = PERCEIVED(9i32);
pub const PERCEIVED_TYPE_CONTACTS: PERCEIVED = PERCEIVED(10i32);
pub const PERCEIVED_TYPE_LAST: PERCEIVED = PERCEIVED(10i32);
impl ::core::marker::Copy for PERCEIVED {}
impl ::core::clone::Clone for PERCEIVED {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PERCEIVEDFLAG_GDIPLUS: u32 = 16u32;
pub const PERCEIVEDFLAG_HARDCODED: u32 = 2u32;
pub const PERCEIVEDFLAG_NATIVESUPPORT: u32 = 4u32;
pub const PERCEIVEDFLAG_SOFTCODED: u32 = 1u32;
pub const PERCEIVEDFLAG_UNDEFINED: u32 = 0u32;
pub const PERCEIVEDFLAG_WMSDK: u32 = 32u32;
pub const PERCEIVEDFLAG_ZIPFOLDER: u32 = 64u32;
#[repr(transparent)]
pub struct SHCOLSTATE(pub i32);
pub const SHCOLSTATE_DEFAULT: SHCOLSTATE = SHCOLSTATE(0i32);
pub const SHCOLSTATE_TYPE_STR: SHCOLSTATE = SHCOLSTATE(1i32);
pub const SHCOLSTATE_TYPE_INT: SHCOLSTATE = SHCOLSTATE(2i32);
pub const SHCOLSTATE_TYPE_DATE: SHCOLSTATE = SHCOLSTATE(3i32);
pub const SHCOLSTATE_TYPEMASK: SHCOLSTATE = SHCOLSTATE(15i32);
pub const SHCOLSTATE_ONBYDEFAULT: SHCOLSTATE = SHCOLSTATE(16i32);
pub const SHCOLSTATE_SLOW: SHCOLSTATE = SHCOLSTATE(32i32);
pub const SHCOLSTATE_EXTENDED: SHCOLSTATE = SHCOLSTATE(64i32);
pub const SHCOLSTATE_SECONDARYUI: SHCOLSTATE = SHCOLSTATE(128i32);
pub const SHCOLSTATE_HIDDEN: SHCOLSTATE = SHCOLSTATE(256i32);
pub const SHCOLSTATE_PREFER_VARCMP: SHCOLSTATE = SHCOLSTATE(512i32);
pub const SHCOLSTATE_PREFER_FMTCMP: SHCOLSTATE = SHCOLSTATE(1024i32);
pub const SHCOLSTATE_NOSORTBYFOLDERNESS: SHCOLSTATE = SHCOLSTATE(2048i32);
pub const SHCOLSTATE_VIEWONLY: SHCOLSTATE = SHCOLSTATE(65536i32);
pub const SHCOLSTATE_BATCHREAD: SHCOLSTATE = SHCOLSTATE(131072i32);
pub const SHCOLSTATE_NO_GROUPBY: SHCOLSTATE = SHCOLSTATE(262144i32);
pub const SHCOLSTATE_FIXED_WIDTH: SHCOLSTATE = SHCOLSTATE(4096i32);
pub const SHCOLSTATE_NODPISCALE: SHCOLSTATE = SHCOLSTATE(8192i32);
pub const SHCOLSTATE_FIXED_RATIO: SHCOLSTATE = SHCOLSTATE(16384i32);
pub const SHCOLSTATE_DISPLAYMASK: SHCOLSTATE = SHCOLSTATE(61440i32);
impl ::core::marker::Copy for SHCOLSTATE {}
impl ::core::clone::Clone for SHCOLSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct SHELLDETAILS {
    pub fmt: i32,
    pub cxChar: i32,
    pub str: STRRET,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SHELLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SHELLDETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct SHITEMID {
    pub cb: u16,
    pub abID: [u8; 1],
}
impl ::core::marker::Copy for SHITEMID {}
impl ::core::clone::Clone for SHITEMID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STRRET {
    pub uType: u32,
    pub Anonymous: STRRET_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STRRET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STRRET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union STRRET_0 {
    pub pOleStr: super::super::super::Foundation::PWSTR,
    pub uOffset: u32,
    pub cStr: [u8; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STRRET_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STRRET_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct STRRET_TYPE(pub i32);
pub const STRRET_WSTR: STRRET_TYPE = STRRET_TYPE(0i32);
pub const STRRET_OFFSET: STRRET_TYPE = STRRET_TYPE(1i32);
pub const STRRET_CSTR: STRRET_TYPE = STRRET_TYPE(2i32);
impl ::core::marker::Copy for STRRET_TYPE {}
impl ::core::clone::Clone for STRRET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
