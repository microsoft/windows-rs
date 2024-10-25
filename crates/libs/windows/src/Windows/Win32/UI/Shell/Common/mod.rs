pub const DEVICE_SCALE_FACTOR_INVALID: DEVICE_SCALE_FACTOR = 0i32;
pub const PERCEIVEDFLAG_GDIPLUS: u32 = 16u32;
pub const PERCEIVEDFLAG_HARDCODED: u32 = 2u32;
pub const PERCEIVEDFLAG_NATIVESUPPORT: u32 = 4u32;
pub const PERCEIVEDFLAG_SOFTCODED: u32 = 1u32;
pub const PERCEIVEDFLAG_UNDEFINED: u32 = 0u32;
pub const PERCEIVEDFLAG_WMSDK: u32 = 32u32;
pub const PERCEIVEDFLAG_ZIPFOLDER: u32 = 64u32;
pub const PERCEIVED_TYPE_APPLICATION: PERCEIVED = 8i32;
pub const PERCEIVED_TYPE_AUDIO: PERCEIVED = 3i32;
pub const PERCEIVED_TYPE_COMPRESSED: PERCEIVED = 5i32;
pub const PERCEIVED_TYPE_CONTACTS: PERCEIVED = 10i32;
pub const PERCEIVED_TYPE_CUSTOM: PERCEIVED = -3i32;
pub const PERCEIVED_TYPE_DOCUMENT: PERCEIVED = 6i32;
pub const PERCEIVED_TYPE_FIRST: PERCEIVED = -3i32;
pub const PERCEIVED_TYPE_FOLDER: PERCEIVED = -1i32;
pub const PERCEIVED_TYPE_GAMEMEDIA: PERCEIVED = 9i32;
pub const PERCEIVED_TYPE_IMAGE: PERCEIVED = 2i32;
pub const PERCEIVED_TYPE_LAST: PERCEIVED = 10i32;
pub const PERCEIVED_TYPE_SYSTEM: PERCEIVED = 7i32;
pub const PERCEIVED_TYPE_TEXT: PERCEIVED = 1i32;
pub const PERCEIVED_TYPE_UNKNOWN: PERCEIVED = 0i32;
pub const PERCEIVED_TYPE_UNSPECIFIED: PERCEIVED = -2i32;
pub const PERCEIVED_TYPE_VIDEO: PERCEIVED = 4i32;
pub const SCALE_100_PERCENT: DEVICE_SCALE_FACTOR = 100i32;
pub const SCALE_120_PERCENT: DEVICE_SCALE_FACTOR = 120i32;
pub const SCALE_125_PERCENT: DEVICE_SCALE_FACTOR = 125i32;
pub const SCALE_140_PERCENT: DEVICE_SCALE_FACTOR = 140i32;
pub const SCALE_150_PERCENT: DEVICE_SCALE_FACTOR = 150i32;
pub const SCALE_160_PERCENT: DEVICE_SCALE_FACTOR = 160i32;
pub const SCALE_175_PERCENT: DEVICE_SCALE_FACTOR = 175i32;
pub const SCALE_180_PERCENT: DEVICE_SCALE_FACTOR = 180i32;
pub const SCALE_200_PERCENT: DEVICE_SCALE_FACTOR = 200i32;
pub const SCALE_225_PERCENT: DEVICE_SCALE_FACTOR = 225i32;
pub const SCALE_250_PERCENT: DEVICE_SCALE_FACTOR = 250i32;
pub const SCALE_300_PERCENT: DEVICE_SCALE_FACTOR = 300i32;
pub const SCALE_350_PERCENT: DEVICE_SCALE_FACTOR = 350i32;
pub const SCALE_400_PERCENT: DEVICE_SCALE_FACTOR = 400i32;
pub const SCALE_450_PERCENT: DEVICE_SCALE_FACTOR = 450i32;
pub const SCALE_500_PERCENT: DEVICE_SCALE_FACTOR = 500i32;
pub const SHCOLSTATE_BATCHREAD: SHCOLSTATE = 131072i32;
pub const SHCOLSTATE_DEFAULT: SHCOLSTATE = 0i32;
pub const SHCOLSTATE_DISPLAYMASK: SHCOLSTATE = 61440i32;
pub const SHCOLSTATE_EXTENDED: SHCOLSTATE = 64i32;
pub const SHCOLSTATE_FIXED_RATIO: SHCOLSTATE = 16384i32;
pub const SHCOLSTATE_FIXED_WIDTH: SHCOLSTATE = 4096i32;
pub const SHCOLSTATE_HIDDEN: SHCOLSTATE = 256i32;
pub const SHCOLSTATE_NODPISCALE: SHCOLSTATE = 8192i32;
pub const SHCOLSTATE_NOSORTBYFOLDERNESS: SHCOLSTATE = 2048i32;
pub const SHCOLSTATE_NO_GROUPBY: SHCOLSTATE = 262144i32;
pub const SHCOLSTATE_ONBYDEFAULT: SHCOLSTATE = 16i32;
pub const SHCOLSTATE_PREFER_FMTCMP: SHCOLSTATE = 1024i32;
pub const SHCOLSTATE_PREFER_VARCMP: SHCOLSTATE = 512i32;
pub const SHCOLSTATE_SECONDARYUI: SHCOLSTATE = 128i32;
pub const SHCOLSTATE_SLOW: SHCOLSTATE = 32i32;
pub const SHCOLSTATE_TYPEMASK: SHCOLSTATE = 15i32;
pub const SHCOLSTATE_TYPE_DATE: SHCOLSTATE = 3i32;
pub const SHCOLSTATE_TYPE_INT: SHCOLSTATE = 2i32;
pub const SHCOLSTATE_TYPE_STR: SHCOLSTATE = 1i32;
pub const SHCOLSTATE_VIEWONLY: SHCOLSTATE = 65536i32;
pub const STRRET_CSTR: STRRET_TYPE = 2i32;
pub const STRRET_OFFSET: STRRET_TYPE = 1i32;
pub const STRRET_WSTR: STRRET_TYPE = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DEVICE_SCALE_FACTOR(pub i32);
impl windows_core::TypeKind for DEVICE_SCALE_FACTOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PERCEIVED(pub i32);
impl windows_core::TypeKind for PERCEIVED {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SHCOLSTATE(pub i32);
impl windows_core::TypeKind for SHCOLSTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STRRET_TYPE(pub i32);
impl windows_core::TypeKind for STRRET_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COMDLG_FILTERSPEC {
    pub pszName: windows_core::PCWSTR,
    pub pszSpec: windows_core::PCWSTR,
}
impl Default for COMDLG_FILTERSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COMDLG_FILTERSPEC {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ITEMIDLIST {
    pub mkid: SHITEMID,
}
impl Default for ITEMIDLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ITEMIDLIST {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl windows_core::TypeKind for SHELLDETAILS {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHITEMID {
    pub cb: u16,
    pub abID: [u8; 1],
}
impl Default for SHITEMID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SHITEMID {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STRRET {
    pub uType: u32,
    pub Anonymous: STRRET_0,
}
impl Default for STRRET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for STRRET {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union STRRET_0 {
    pub pOleStr: windows_core::PWSTR,
    pub uOffset: u32,
    pub cStr: [u8; 260],
}
impl Default for STRRET_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for STRRET_0 {
    type TypeKind = windows_core::CopyType;
}
