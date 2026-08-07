pub const GUID_TS_SERVICE_ACCESSIBLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf9786200_a5bf_4a0f_8c24_fb16f5d1aabb);
pub const GUID_TS_SERVICE_ACTIVEX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xea937a50_c9a6_4b7d_894a_49d99b784834);
pub const GUID_TS_SERVICE_DATAOBJECT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6086fbb5_e225_46ce_a770_c1bbd3e05d7b);
pub const GXFPF_NEAREST: i32 = 2;
pub const GXFPF_ROUND_NEAREST: i32 = 1;
pub const TS_AE_END: TsActiveSelEnd = 2;
pub const TS_AE_NONE: TsActiveSelEnd = 0;
pub const TS_AE_START: TsActiveSelEnd = 1;
pub const TS_AS_ALL_SINKS: i32 = 31;
pub const TS_AS_ATTR_CHANGE: i32 = 8;
pub const TS_AS_LAYOUT_CHANGE: i32 = 4;
pub const TS_AS_SEL_CHANGE: i32 = 2;
pub const TS_AS_STATUS_CHANGE: i32 = 16;
pub const TS_AS_TEXT_CHANGE: i32 = 1;
pub type TS_ATTRID = windows_sys::core::GUID;
#[repr(C)]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct TS_ATTRVAL {
    pub idAttr: TS_ATTRID,
    pub dwOverlapId: u32,
    pub varValue: super::VARIANT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for TS_ATTRVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TS_ATTR_FIND_BACKWARDS: i32 = 1;
pub const TS_ATTR_FIND_HIDDEN: i32 = 32;
pub const TS_ATTR_FIND_UPDATESTART: i32 = 4;
pub const TS_ATTR_FIND_WANT_END: i32 = 16;
pub const TS_ATTR_FIND_WANT_OFFSET: i32 = 2;
pub const TS_ATTR_FIND_WANT_VALUE: i32 = 8;
pub const TS_CHAR_EMBEDDED: i32 = 65532;
pub const TS_CHAR_REGION: i32 = 0;
pub const TS_CHAR_REPLACEMENT: i32 = 65533;
pub const TS_CH_FOLLOWING_DEL: i32 = 2;
pub const TS_CH_PRECEDING_DEL: i32 = 1;
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
pub const TS_GEA_HIDDEN: i32 = 1;
pub const TS_GR_BACKWARD: TsGravity = 0;
pub const TS_GR_FORWARD: TsGravity = 1;
pub const TS_GTA_HIDDEN: i32 = 1;
pub const TS_IAS_NOQUERY: i32 = 1;
pub const TS_IAS_QUERYONLY: i32 = 2;
pub const TS_IE_COMPOSITION: i32 = 2;
pub const TS_IE_CORRECTION: i32 = 1;
pub const TS_LC_CHANGE: TsLayoutCode = 1;
pub const TS_LC_CREATE: TsLayoutCode = 0;
pub const TS_LC_DESTROY: TsLayoutCode = 2;
pub const TS_LF_READ: i32 = 2;
pub const TS_LF_READWRITE: i32 = 6;
pub const TS_LF_SYNC: i32 = 1;
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
pub const TS_SD_DISABLEWRITINGSUGGESTIONS: i32 = 512;
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_ENABLED: i32 = 128;
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_VISIBLE: i32 = 256;
pub const TS_SD_FORWARD: TsShiftDir = 1;
pub const TS_SD_INPUTPANEMANUALDISPLAYENABLE: i32 = 64;
pub const TS_SD_LOADING: i32 = 2;
pub const TS_SD_MASKALL: i32 = 3;
pub const TS_SD_READONLY: i32 = 1;
pub const TS_SD_RESERVED: i32 = 4;
pub const TS_SD_TKBAUTOCORRECTENABLE: i32 = 8;
pub const TS_SD_TKBPREDICTIONENABLE: i32 = 16;
pub const TS_SD_UIINTEGRATIONENABLE: i32 = 32;
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
#[derive(Clone, Copy, Default)]
pub struct TS_SELECTION_ANCHOR {
    pub paStart: *mut core::ffi::c_void,
    pub paEnd: *mut core::ffi::c_void,
    pub style: TS_SELECTIONSTYLE,
}
pub const TS_SHIFT_COUNT_HIDDEN: i32 = 1;
pub const TS_SHIFT_COUNT_ONLY: i32 = 8;
pub const TS_SHIFT_HALT_HIDDEN: i32 = 2;
pub const TS_SHIFT_HALT_VISIBLE: i32 = 4;
pub const TS_SS_DISJOINTSEL: i32 = 1;
pub const TS_SS_MULTILINE: i32 = 128;
pub const TS_SS_NOHIDDENTEXT: i32 = 8;
pub const TS_SS_REGIONS: i32 = 2;
pub const TS_SS_TKBAUTOCORRECTENABLE: i32 = 16;
pub const TS_SS_TKBPREDICTIONENABLE: i32 = 32;
pub const TS_SS_TRANSITORY: i32 = 4;
pub const TS_SS_UWPCONTROL: i32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TS_STATUS {
    pub dwDynamicFlags: u32,
    pub dwStaticFlags: u32,
}
pub const TS_ST_CORRECTION: i32 = 1;
pub const TS_S_ASYNC: i32 = 262912;
pub const TS_TC_CORRECTION: i32 = 1;
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
