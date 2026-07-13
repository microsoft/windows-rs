pub const GUID_TS_SERVICE_ACCESSIBLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf9786200_a5bf_4a0f_8c24_fb16f5d1aabb);
pub const GUID_TS_SERVICE_ACTIVEX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xea937a50_c9a6_4b7d_894a_49d99b784834);
pub const GUID_TS_SERVICE_DATAOBJECT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6086fbb5_e225_46ce_a770_c1bbd3e05d7b);
pub const GXFPF_NEAREST: u32 = 2;
pub const GXFPF_ROUND_NEAREST: u32 = 1;
pub const TS_AE_END: TsActiveSelEnd = 2;
pub const TS_AE_NONE: TsActiveSelEnd = 0;
pub const TS_AE_START: TsActiveSelEnd = 1;
pub const TS_AS_ALL_SINKS: u32 = 31;
pub const TS_AS_ATTR_CHANGE: u32 = 8;
pub const TS_AS_LAYOUT_CHANGE: u32 = 4;
pub const TS_AS_SEL_CHANGE: u32 = 2;
pub const TS_AS_STATUS_CHANGE: u32 = 16;
pub const TS_AS_TEXT_CHANGE: u32 = 1;
pub type TS_ATTRID = windows_sys::core::GUID;
#[repr(C)]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct TS_ATTRVAL {
    pub idAttr: TS_ATTRID,
    pub dwOverlapId: u32,
    pub varValue: super::oaidl::VARIANT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for TS_ATTRVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TS_ATTR_FIND_BACKWARDS: u32 = 1;
pub const TS_ATTR_FIND_HIDDEN: u32 = 32;
pub const TS_ATTR_FIND_UPDATESTART: u32 = 4;
pub const TS_ATTR_FIND_WANT_END: u32 = 16;
pub const TS_ATTR_FIND_WANT_OFFSET: u32 = 2;
pub const TS_ATTR_FIND_WANT_VALUE: u32 = 8;
pub const TS_CHAR_EMBEDDED: u32 = 65532;
pub const TS_CHAR_REGION: u32 = 0;
pub const TS_CHAR_REPLACEMENT: u32 = 65533;
pub const TS_CH_FOLLOWING_DEL: u32 = 2;
pub const TS_CH_PRECEDING_DEL: u32 = 1;
pub const TS_DEFAULT_SELECTION: u32 = 4294967295;
pub const TS_E_FORMAT: i32 = -2147220982;
pub const TS_E_INVALIDPOINT: i32 = -2147220985;
pub const TS_E_INVALIDPOS: i32 = -2147220992;
pub const TS_E_NOINTERFACE: i32 = -2147220988;
pub const TS_E_NOLAYOUT: i32 = -2147220986;
pub const TS_E_NOLOCK: i32 = -2147220991;
pub const TS_E_NOOBJECT: i32 = -2147220990;
pub const TS_E_NOSELECTION: i32 = -2147220987;
pub const TS_E_NOSERVICE: i32 = -2147220989;
pub const TS_E_READONLY: i32 = -2147220983;
pub const TS_E_SYNCHRONOUS: i32 = -2147220984;
pub const TS_GEA_HIDDEN: u32 = 1;
pub const TS_GR_BACKWARD: TsGravity = 0;
pub const TS_GR_FORWARD: TsGravity = 1;
pub const TS_GTA_HIDDEN: u32 = 1;
pub const TS_IAS_NOQUERY: u32 = 1;
pub const TS_IAS_QUERYONLY: u32 = 2;
pub const TS_IE_COMPOSITION: u32 = 2;
pub const TS_IE_CORRECTION: u32 = 1;
pub const TS_LC_CHANGE: TsLayoutCode = 1;
pub const TS_LC_CREATE: TsLayoutCode = 0;
pub const TS_LC_DESTROY: TsLayoutCode = 2;
pub const TS_LF_READ: u32 = 2;
pub const TS_LF_READWRITE: u32 = 6;
pub const TS_LF_SYNC: u32 = 1;
pub const TS_RT_HIDDEN: TsRunType = 1;
pub const TS_RT_OPAQUE: TsRunType = 2;
pub const TS_RT_PLAIN: TsRunType = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TS_RUNINFO {
    pub uCount: u32,
    pub r#type: TsRunType,
}
pub const TS_SD_BACKWARD: TsShiftDir = 0;
pub const TS_SD_DISABLEWRITINGSUGGESTIONS: u32 = 512;
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_ENABLED: u32 = 128;
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_VISIBLE: u32 = 256;
pub const TS_SD_FORWARD: TsShiftDir = 1;
pub const TS_SD_INPUTPANEMANUALDISPLAYENABLE: u32 = 64;
pub const TS_SD_LOADING: u32 = 2;
pub const TS_SD_MASKALL: u32 = 3;
pub const TS_SD_READONLY: u32 = 1;
pub const TS_SD_RESERVED: u32 = 4;
pub const TS_SD_TKBAUTOCORRECTENABLE: u32 = 8;
pub const TS_SD_TKBPREDICTIONENABLE: u32 = 16;
pub const TS_SD_UIINTEGRATIONENABLE: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TS_SELECTIONSTYLE {
    pub ase: TsActiveSelEnd,
    pub fInterimChar: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TS_SELECTION_ACP {
    pub acpStart: i32,
    pub acpEnd: i32,
    pub style: TS_SELECTIONSTYLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TS_SELECTION_ANCHOR {
    pub paStart: *mut core::ffi::c_void,
    pub paEnd: *mut core::ffi::c_void,
    pub style: TS_SELECTIONSTYLE,
}
impl Default for TS_SELECTION_ANCHOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TS_SHIFT_COUNT_HIDDEN: u32 = 1;
pub const TS_SHIFT_COUNT_ONLY: u32 = 8;
pub const TS_SHIFT_HALT_HIDDEN: u32 = 2;
pub const TS_SHIFT_HALT_VISIBLE: u32 = 4;
pub const TS_SS_DISJOINTSEL: u32 = 1;
pub const TS_SS_MULTILINE: u32 = 128;
pub const TS_SS_NOHIDDENTEXT: u32 = 8;
pub const TS_SS_REGIONS: u32 = 2;
pub const TS_SS_TKBAUTOCORRECTENABLE: u32 = 16;
pub const TS_SS_TKBPREDICTIONENABLE: u32 = 32;
pub const TS_SS_TRANSITORY: u32 = 4;
pub const TS_SS_UWPCONTROL: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TS_STATUS {
    pub dwDynamicFlags: u32,
    pub dwStaticFlags: u32,
}
pub const TS_ST_CORRECTION: u32 = 1;
pub const TS_S_ASYNC: u32 = 262912;
pub const TS_TC_CORRECTION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TS_TEXTCHANGE {
    pub acpStart: i32,
    pub acpOldEnd: i32,
    pub acpNewEnd: i32,
}
pub const TS_VCOOKIE_NUL: u32 = 4294967295;
pub type TsActiveSelEnd = i32;
pub type TsGravity = i32;
pub type TsLayoutCode = i32;
pub type TsRunType = i32;
pub type TsShiftDir = i32;
pub type TsViewCookie = u32;
