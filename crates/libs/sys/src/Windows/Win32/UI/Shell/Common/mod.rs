#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COMDLG_FILTERSPEC {
    pub pszName: windows_sys::core::PCWSTR,
    pub pszSpec: windows_sys::core::PCWSTR,
}
impl Default for COMDLG_FILTERSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DEVICE_SCALE_FACTOR = i32;
pub const DEVICE_SCALE_FACTOR_INVALID: DEVICE_SCALE_FACTOR = 0;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ITEMIDLIST {
    pub mkid: SHITEMID,
}
pub type PERCEIVED = i32;
pub const PERCEIVEDFLAG_GDIPLUS: u32 = 16;
pub const PERCEIVEDFLAG_HARDCODED: u32 = 2;
pub const PERCEIVEDFLAG_NATIVESUPPORT: u32 = 4;
pub const PERCEIVEDFLAG_SOFTCODED: u32 = 1;
pub const PERCEIVEDFLAG_UNDEFINED: u32 = 0;
pub const PERCEIVEDFLAG_WMSDK: u32 = 32;
pub const PERCEIVEDFLAG_ZIPFOLDER: u32 = 64;
pub const PERCEIVED_TYPE_APPLICATION: PERCEIVED = 8;
pub const PERCEIVED_TYPE_AUDIO: PERCEIVED = 3;
pub const PERCEIVED_TYPE_COMPRESSED: PERCEIVED = 5;
pub const PERCEIVED_TYPE_CONTACTS: PERCEIVED = 10;
pub const PERCEIVED_TYPE_CUSTOM: PERCEIVED = -3;
pub const PERCEIVED_TYPE_DOCUMENT: PERCEIVED = 6;
pub const PERCEIVED_TYPE_FIRST: PERCEIVED = -3;
pub const PERCEIVED_TYPE_FOLDER: PERCEIVED = -1;
pub const PERCEIVED_TYPE_GAMEMEDIA: PERCEIVED = 9;
pub const PERCEIVED_TYPE_IMAGE: PERCEIVED = 2;
pub const PERCEIVED_TYPE_LAST: PERCEIVED = 10;
pub const PERCEIVED_TYPE_SYSTEM: PERCEIVED = 7;
pub const PERCEIVED_TYPE_TEXT: PERCEIVED = 1;
pub const PERCEIVED_TYPE_UNKNOWN: PERCEIVED = 0;
pub const PERCEIVED_TYPE_UNSPECIFIED: PERCEIVED = -2;
pub const PERCEIVED_TYPE_VIDEO: PERCEIVED = 4;
pub const SCALE_100_PERCENT: DEVICE_SCALE_FACTOR = 100;
pub const SCALE_120_PERCENT: DEVICE_SCALE_FACTOR = 120;
pub const SCALE_125_PERCENT: DEVICE_SCALE_FACTOR = 125;
pub const SCALE_140_PERCENT: DEVICE_SCALE_FACTOR = 140;
pub const SCALE_150_PERCENT: DEVICE_SCALE_FACTOR = 150;
pub const SCALE_160_PERCENT: DEVICE_SCALE_FACTOR = 160;
pub const SCALE_175_PERCENT: DEVICE_SCALE_FACTOR = 175;
pub const SCALE_180_PERCENT: DEVICE_SCALE_FACTOR = 180;
pub const SCALE_200_PERCENT: DEVICE_SCALE_FACTOR = 200;
pub const SCALE_225_PERCENT: DEVICE_SCALE_FACTOR = 225;
pub const SCALE_250_PERCENT: DEVICE_SCALE_FACTOR = 250;
pub const SCALE_300_PERCENT: DEVICE_SCALE_FACTOR = 300;
pub const SCALE_350_PERCENT: DEVICE_SCALE_FACTOR = 350;
pub const SCALE_400_PERCENT: DEVICE_SCALE_FACTOR = 400;
pub const SCALE_450_PERCENT: DEVICE_SCALE_FACTOR = 450;
pub const SCALE_500_PERCENT: DEVICE_SCALE_FACTOR = 500;
pub type SHCOLSTATE = i32;
pub const SHCOLSTATE_BATCHREAD: SHCOLSTATE = 131072;
pub const SHCOLSTATE_DEFAULT: SHCOLSTATE = 0;
pub const SHCOLSTATE_DISPLAYMASK: SHCOLSTATE = 61440;
pub const SHCOLSTATE_EXTENDED: SHCOLSTATE = 64;
pub const SHCOLSTATE_FIXED_RATIO: SHCOLSTATE = 16384;
pub const SHCOLSTATE_FIXED_WIDTH: SHCOLSTATE = 4096;
pub const SHCOLSTATE_HIDDEN: SHCOLSTATE = 256;
pub const SHCOLSTATE_NODPISCALE: SHCOLSTATE = 8192;
pub const SHCOLSTATE_NOSORTBYFOLDERNESS: SHCOLSTATE = 2048;
pub const SHCOLSTATE_NO_GROUPBY: SHCOLSTATE = 262144;
pub const SHCOLSTATE_ONBYDEFAULT: SHCOLSTATE = 16;
pub const SHCOLSTATE_PREFER_FMTCMP: SHCOLSTATE = 1024;
pub const SHCOLSTATE_PREFER_VARCMP: SHCOLSTATE = 512;
pub const SHCOLSTATE_SECONDARYUI: SHCOLSTATE = 128;
pub const SHCOLSTATE_SLOW: SHCOLSTATE = 32;
pub const SHCOLSTATE_TYPEMASK: SHCOLSTATE = 15;
pub const SHCOLSTATE_TYPE_DATE: SHCOLSTATE = 3;
pub const SHCOLSTATE_TYPE_INT: SHCOLSTATE = 2;
pub const SHCOLSTATE_TYPE_STR: SHCOLSTATE = 1;
pub const SHCOLSTATE_VIEWONLY: SHCOLSTATE = 65536;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SHELLDETAILS {
    pub fmt: i32,
    pub cxChar: i32,
    pub str: STRRET,
}
impl Default for SHELLDETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SHITEMID {
    pub cb: u16,
    pub abID: [u8; 1],
}
impl Default for SHITEMID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STRRET {
    pub uType: u32,
    pub Anonymous: STRRET_0,
}
impl Default for STRRET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STRRET_0 {
    pub pOleStr: windows_sys::core::PWSTR,
    pub uOffset: u32,
    pub cStr: [u8; 260],
}
impl Default for STRRET_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STRRET_CSTR: STRRET_TYPE = 2;
pub const STRRET_OFFSET: STRRET_TYPE = 1;
pub type STRRET_TYPE = i32;
pub const STRRET_WSTR: STRRET_TYPE = 0;
