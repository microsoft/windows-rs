#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
pub const DEVICE_SCALE_FACTOR_INVALID: i32 = 0i32;
pub const SCALE_100_PERCENT: i32 = 100i32;
pub const SCALE_120_PERCENT: i32 = 120i32;
pub const SCALE_125_PERCENT: i32 = 125i32;
pub const SCALE_140_PERCENT: i32 = 140i32;
pub const SCALE_150_PERCENT: i32 = 150i32;
pub const SCALE_160_PERCENT: i32 = 160i32;
pub const SCALE_175_PERCENT: i32 = 175i32;
pub const SCALE_180_PERCENT: i32 = 180i32;
pub const SCALE_200_PERCENT: i32 = 200i32;
pub const SCALE_225_PERCENT: i32 = 225i32;
pub const SCALE_250_PERCENT: i32 = 250i32;
pub const SCALE_300_PERCENT: i32 = 300i32;
pub const SCALE_350_PERCENT: i32 = 350i32;
pub const SCALE_400_PERCENT: i32 = 400i32;
pub const SCALE_450_PERCENT: i32 = 450i32;
pub const SCALE_500_PERCENT: i32 = 500i32;
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
pub const PERCEIVED_TYPE_FIRST: i32 = -3i32;
pub const PERCEIVED_TYPE_CUSTOM: i32 = -3i32;
pub const PERCEIVED_TYPE_UNSPECIFIED: i32 = -2i32;
pub const PERCEIVED_TYPE_FOLDER: i32 = -1i32;
pub const PERCEIVED_TYPE_UNKNOWN: i32 = 0i32;
pub const PERCEIVED_TYPE_TEXT: i32 = 1i32;
pub const PERCEIVED_TYPE_IMAGE: i32 = 2i32;
pub const PERCEIVED_TYPE_AUDIO: i32 = 3i32;
pub const PERCEIVED_TYPE_VIDEO: i32 = 4i32;
pub const PERCEIVED_TYPE_COMPRESSED: i32 = 5i32;
pub const PERCEIVED_TYPE_DOCUMENT: i32 = 6i32;
pub const PERCEIVED_TYPE_SYSTEM: i32 = 7i32;
pub const PERCEIVED_TYPE_APPLICATION: i32 = 8i32;
pub const PERCEIVED_TYPE_GAMEMEDIA: i32 = 9i32;
pub const PERCEIVED_TYPE_CONTACTS: i32 = 10i32;
pub const PERCEIVED_TYPE_LAST: i32 = 10i32;
pub const PERCEIVEDFLAG_GDIPLUS: u32 = 16u32;
pub const PERCEIVEDFLAG_HARDCODED: u32 = 2u32;
pub const PERCEIVEDFLAG_NATIVESUPPORT: u32 = 4u32;
pub const PERCEIVEDFLAG_SOFTCODED: u32 = 1u32;
pub const PERCEIVEDFLAG_UNDEFINED: u32 = 0u32;
pub const PERCEIVEDFLAG_WMSDK: u32 = 32u32;
pub const PERCEIVEDFLAG_ZIPFOLDER: u32 = 64u32;
pub const SHCOLSTATE_DEFAULT: i32 = 0i32;
pub const SHCOLSTATE_TYPE_STR: i32 = 1i32;
pub const SHCOLSTATE_TYPE_INT: i32 = 2i32;
pub const SHCOLSTATE_TYPE_DATE: i32 = 3i32;
pub const SHCOLSTATE_TYPEMASK: i32 = 15i32;
pub const SHCOLSTATE_ONBYDEFAULT: i32 = 16i32;
pub const SHCOLSTATE_SLOW: i32 = 32i32;
pub const SHCOLSTATE_EXTENDED: i32 = 64i32;
pub const SHCOLSTATE_SECONDARYUI: i32 = 128i32;
pub const SHCOLSTATE_HIDDEN: i32 = 256i32;
pub const SHCOLSTATE_PREFER_VARCMP: i32 = 512i32;
pub const SHCOLSTATE_PREFER_FMTCMP: i32 = 1024i32;
pub const SHCOLSTATE_NOSORTBYFOLDERNESS: i32 = 2048i32;
pub const SHCOLSTATE_VIEWONLY: i32 = 65536i32;
pub const SHCOLSTATE_BATCHREAD: i32 = 131072i32;
pub const SHCOLSTATE_NO_GROUPBY: i32 = 262144i32;
pub const SHCOLSTATE_FIXED_WIDTH: i32 = 4096i32;
pub const SHCOLSTATE_NODPISCALE: i32 = 8192i32;
pub const SHCOLSTATE_FIXED_RATIO: i32 = 16384i32;
pub const SHCOLSTATE_DISPLAYMASK: i32 = 61440i32;
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
pub const STRRET_WSTR: i32 = 0i32;
pub const STRRET_OFFSET: i32 = 1i32;
pub const STRRET_CSTR: i32 = 2i32;
