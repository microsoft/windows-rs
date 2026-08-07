pub const TF_AE_END: TfActiveSelEnd = 2;
pub const TF_AE_NONE: TfActiveSelEnd = 0;
pub const TF_AE_START: TfActiveSelEnd = 1;
pub const TF_ANCHOR_END: TfAnchor = 1;
pub const TF_ANCHOR_START: TfAnchor = 0;
pub const TF_ATTR_CONVERTED: TF_DA_ATTR_INFO = 2;
pub const TF_ATTR_FIXEDCONVERTED: TF_DA_ATTR_INFO = 5;
pub const TF_ATTR_INPUT: TF_DA_ATTR_INFO = 0;
pub const TF_ATTR_INPUT_ERROR: TF_DA_ATTR_INFO = 4;
pub const TF_ATTR_OTHER: TF_DA_ATTR_INFO = -1;
pub const TF_ATTR_TARGET_CONVERTED: TF_DA_ATTR_INFO = 1;
pub const TF_ATTR_TARGET_NOTCONVERTED: TF_DA_ATTR_INFO = 3;
pub const TF_CHAR_EMBEDDED: i32 = 65532;
pub const TF_CLIENTID_NULL: TfClientId = 0;
pub const TF_CLUIE_COUNT: i32 = 2;
pub const TF_CLUIE_CURRENTPAGE: i32 = 32;
pub const TF_CLUIE_DOCUMENTMGR: i32 = 1;
pub const TF_CLUIE_PAGEINDEX: i32 = 16;
pub const TF_CLUIE_SELECTION: i32 = 4;
pub const TF_CLUIE_STRING: i32 = 8;
pub const TF_CONVERSIONMODE_ALPHANUMERIC: i32 = 0;
pub const TF_CONVERSIONMODE_CHARCODE: i32 = 32;
pub const TF_CONVERSIONMODE_EUDC: i32 = 512;
pub const TF_CONVERSIONMODE_FIXED: i32 = 2048;
pub const TF_CONVERSIONMODE_FULLSHAPE: i32 = 8;
pub const TF_CONVERSIONMODE_KATAKANA: i32 = 2;
pub const TF_CONVERSIONMODE_NATIVE: i32 = 1;
pub const TF_CONVERSIONMODE_NOCONVERSION: i32 = 256;
pub const TF_CONVERSIONMODE_ROMAN: i32 = 16;
pub const TF_CONVERSIONMODE_SOFTKEYBOARD: i32 = 128;
pub const TF_CONVERSIONMODE_SYMBOL: i32 = 1024;
pub const TF_CT_COLORREF: TF_DA_COLORTYPE = 2;
pub const TF_CT_NONE: TF_DA_COLORTYPE = 0;
pub const TF_CT_SYSCOLOR: TF_DA_COLORTYPE = 1;
pub type TF_DA_ATTR_INFO = i32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct TF_DA_COLOR {
    pub r#type: TF_DA_COLORTYPE,
    pub Anonymous: TF_DA_COLOR_0,
}
#[cfg(feature = "windef")]
impl Default for TF_DA_COLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub union TF_DA_COLOR_0 {
    pub nIndex: i32,
    pub cr: super::COLORREF,
}
#[cfg(feature = "windef")]
impl Default for TF_DA_COLOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TF_DA_COLORTYPE = i32;
pub type TF_DA_LINESTYLE = i32;
pub const TF_DEFAULT_SELECTION: u32 = 4294967295;
pub const TF_DISABLE_COMMANDING: i32 = 4;
pub const TF_DISABLE_DICTATION: i32 = 2;
pub const TF_DISABLE_SPEECH: i32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct TF_DISPLAYATTRIBUTE {
    pub crText: TF_DA_COLOR,
    pub crBk: TF_DA_COLOR,
    pub lsStyle: TF_DA_LINESTYLE,
    pub fBoldLine: windows_sys::core::BOOL,
    pub crLine: TF_DA_COLOR,
    pub bAttr: TF_DA_ATTR_INFO,
}
#[cfg(feature = "windef")]
impl Default for TF_DISPLAYATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_ES_ASYNC: i32 = 8;
pub const TF_ES_ASYNCDONTCARE: i32 = 0;
pub const TF_ES_READ: i32 = 2;
pub const TF_ES_READWRITE: i32 = 6;
pub const TF_ES_SYNC: i32 = 1;
pub const TF_E_ALREADY_EXISTS: i32 = -2147220218;
pub const TF_E_COMPOSITION_REJECTED: i32 = -2147220216;
pub const TF_E_DISCONNECTED: i32 = -2147220220;
pub const TF_E_EMPTYCONTEXT: i32 = -2147220215;
pub const TF_E_FORMAT: i32 = -2147220982;
pub const TF_E_INVALIDPOINT: i32 = -2147220985;
pub const TF_E_INVALIDPOS: i32 = -2147220992;
pub const TF_E_INVALIDVIEW: i32 = -2147220219;
pub const TF_E_LOCKED: i32 = -2147220224;
pub const TF_E_NOINTERFACE: i32 = -2147220988;
pub const TF_E_NOLAYOUT: i32 = -2147220986;
pub const TF_E_NOLOCK: i32 = -2147220991;
pub const TF_E_NOOBJECT: i32 = -2147220990;
pub const TF_E_NOPROVIDER: i32 = -2147220221;
pub const TF_E_NOSELECTION: i32 = -2147220987;
pub const TF_E_NOSERVICE: i32 = -2147220989;
pub const TF_E_NOTOWNEDRANGE: i32 = -2147220222;
pub const TF_E_RANGE_NOT_COVERED: i32 = -2147220217;
pub const TF_E_READONLY: i32 = -2147220983;
pub const TF_E_STACKFULL: i32 = -2147220223;
pub const TF_E_SYNCHRONOUS: i32 = -2147220984;
pub const TF_GRAVITY_BACKWARD: TfGravity = 0;
pub const TF_GRAVITY_FORWARD: TfGravity = 1;
pub const TF_GTP_INCL_TEXT: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TF_HALTCOND {
    pub pHaltRange: *mut core::ffi::c_void,
    pub aHaltPos: TfAnchor,
    pub dwFlags: u32,
}
pub const TF_HF_OBJECT: i32 = 1;
pub const TF_IAS_NOQUERY: i32 = 1;
pub const TF_IAS_NO_DEFAULT_COMPOSITION: u32 = 2147483648;
pub const TF_IAS_QUERYONLY: i32 = 2;
pub const TF_IE_CORRECTION: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct TF_INPUTPROCESSORPROFILE {
    pub dwProfileType: u32,
    pub langid: super::LANGID,
    pub clsid: windows_sys::core::GUID,
    pub guidProfile: windows_sys::core::GUID,
    pub catid: windows_sys::core::GUID,
    pub hklSubstitute: super::HKL,
    pub dwCaps: u32,
    pub hkl: super::HKL,
    pub dwFlags: u32,
}
pub const TF_INVALID_COOKIE: u32 = 4294967295;
pub const TF_INVALID_EDIT_COOKIE: i32 = 0;
pub const TF_INVALID_GUIDATOM: TfGuidAtom = 0;
pub const TF_INVALID_UIELEMENTID: u32 = 4294967295;
pub const TF_IPPMF_DISABLEPROFILE: i32 = 2;
pub const TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE: i32 = 4;
pub const TF_IPPMF_ENABLEPROFILE: i32 = 1;
pub const TF_IPPMF_FORPROCESS: i32 = 268435456;
pub const TF_IPPMF_FORSESSION: i32 = 536870912;
pub const TF_IPPMF_FORSYSTEMALL: i32 = 1073741824;
pub const TF_IPP_CAPS_COMLESSSUPPORT: i32 = 8;
pub const TF_IPP_CAPS_DISABLEONTRANSITORY: i32 = 1;
pub const TF_IPP_CAPS_IMMERSIVESUPPORT: i32 = 65536;
pub const TF_IPP_CAPS_SECUREMODESUPPORT: i32 = 2;
pub const TF_IPP_CAPS_SYSTRAYSUPPORT: i32 = 131072;
pub const TF_IPP_CAPS_UIELEMENTENABLED: i32 = 4;
pub const TF_IPP_CAPS_WOW16SUPPORT: i32 = 16;
pub const TF_IPP_FLAG_ACTIVE: i32 = 1;
pub const TF_IPP_FLAG_ENABLED: i32 = 2;
pub const TF_IPP_FLAG_SUBSTITUTEDBYINPUTPROCESSOR: i32 = 4;
pub const TF_IPSINK_FLAG_ACTIVE: i32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct TF_LANGUAGEPROFILE {
    pub clsid: windows_sys::core::GUID,
    pub langid: super::LANGID,
    pub catid: windows_sys::core::GUID,
    pub fActive: windows_sys::core::BOOL,
    pub guidProfile: windows_sys::core::GUID,
}
pub const TF_LC_CHANGE: TfLayoutCode = 1;
pub const TF_LC_CREATE: TfLayoutCode = 0;
pub const TF_LC_DESTROY: TfLayoutCode = 2;
pub const TF_LS_DASH: TF_DA_LINESTYLE = 3;
pub const TF_LS_DOT: TF_DA_LINESTYLE = 2;
pub const TF_LS_NONE: TF_DA_LINESTYLE = 0;
pub const TF_LS_SOLID: TF_DA_LINESTYLE = 1;
pub const TF_LS_SQUIGGLE: TF_DA_LINESTYLE = 4;
pub const TF_MOD_ALT: i32 = 1;
pub const TF_MOD_CONTROL: i32 = 2;
pub const TF_MOD_IGNORE_ALL_MODIFIER: i32 = 1024;
pub const TF_MOD_LALT: i32 = 64;
pub const TF_MOD_LCONTROL: i32 = 128;
pub const TF_MOD_LSHIFT: i32 = 256;
pub const TF_MOD_ON_KEYUP: i32 = 512;
pub const TF_MOD_RALT: i32 = 8;
pub const TF_MOD_RCONTROL: i32 = 16;
pub const TF_MOD_RSHIFT: i32 = 32;
pub const TF_MOD_SHIFT: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TF_PERSISTENT_PROPERTY_HEADER_ACP {
    pub guidType: windows_sys::core::GUID,
    pub ichStart: i32,
    pub cch: i32,
    pub cb: u32,
    pub dwPrivate: u32,
    pub clsidTIP: windows_sys::core::GUID,
}
pub const TF_POPF_ALL: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TF_PRESERVEDKEY {
    pub uVKey: u32,
    pub uModifiers: u32,
}
pub const TF_PROFILETYPE_INPUTPROCESSOR: i32 = 1;
pub const TF_PROFILETYPE_KEYBOARDLAYOUT: i32 = 2;
pub const TF_PROFILE_ARRAY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd38eff65_aa46_4fd5_91a7_67845fb02f5b);
pub const TF_PROFILE_CANTONESE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0aec109c_7e96_11d4_b2ef_0080c882687e);
pub const TF_PROFILE_CHANGJIE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4bdf9f03_c7d3_11d4_b2ab_0080c882687e);
pub const TF_PROFILE_DAYI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x037b2c25_480c_4d7f_b027_d6ca6b69788a);
pub const TF_PROFILE_NEWCHANGJIE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf3ba907a_6c7e_11d4_97fa_0080c882687e);
pub const TF_PROFILE_NEWPHONETIC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb2f9c502_1742_11d4_9790_0080c882687e);
pub const TF_PROFILE_NEWQUICK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0b883ba0_c1c7_11d4_87f9_0080c882687e);
pub const TF_PROFILE_PHONETIC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x761309de_317a_11d4_9b5d_0080c882687e);
pub const TF_PROFILE_PINYIN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf3ba9077_6c7e_11d4_97fa_0080c882687e);
pub const TF_PROFILE_QUICK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6024b45f_5c54_11d4_b921_0080c882687e);
pub const TF_PROFILE_SIMPLEFAST: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfa550b04_5ad7_411f_a5ac_ca038ec515d7);
pub const TF_PROFILE_TIGRINYA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cab88b7_cc3e_46a6_9765_b772ad7761ff);
pub const TF_PROFILE_WUBI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x82590c13_f4dd_44f4_ba1d_8667246fdf8e);
pub const TF_PROFILE_YI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x409c8376_007b_4357_ae8e_26316ee3fb0d);
#[repr(C)]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct TF_PROPERTYVAL {
    pub guidId: windows_sys::core::GUID,
    pub varValue: super::VARIANT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for TF_PROPERTYVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_RCM_COMLESS: i32 = 1;
pub const TF_RCM_HINT_COLLISION: i32 = 8;
pub const TF_RCM_HINT_READING_LENGTH: i32 = 4;
pub const TF_RCM_VKEY: i32 = 2;
pub const TF_RIP_FLAG_FREEUNUSEDLIBRARIES: i32 = 1;
pub const TF_RIUIE_CONTEXT: i32 = 1;
pub const TF_RIUIE_ERRORINDEX: i32 = 8;
pub const TF_RIUIE_MAXREADINGSTRINGLENGTH: i32 = 4;
pub const TF_RIUIE_STRING: i32 = 2;
pub const TF_RIUIE_VERTICALORDER: i32 = 16;
pub const TF_RP_HIDDENINSETTINGUI: i32 = 2;
pub const TF_RP_LOCALPROCESS: i32 = 4;
pub const TF_RP_LOCALTHREAD: i32 = 8;
pub const TF_RP_SUBITEMINSETTINGUI: i32 = 16;
pub const TF_SD_BACKWARD: TfShiftDir = 0;
pub const TF_SD_FORWARD: TfShiftDir = 1;
pub const TF_SD_LOADING: i32 = 2;
pub const TF_SD_READONLY: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TF_SELECTION {
    pub range: *mut core::ffi::c_void,
    pub style: TF_SELECTIONSTYLE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TF_SELECTIONSTYLE {
    pub ase: TfActiveSelEnd,
    pub fInterimChar: windows_sys::core::BOOL,
}
pub const TF_SENTENCEMODE_AUTOMATIC: i32 = 4;
pub const TF_SENTENCEMODE_CONVERSATION: i32 = 16;
pub const TF_SENTENCEMODE_NONE: i32 = 0;
pub const TF_SENTENCEMODE_PHRASEPREDICT: i32 = 8;
pub const TF_SENTENCEMODE_PLAURALCLAUSE: i32 = 1;
pub const TF_SENTENCEMODE_SINGLECONVERT: i32 = 2;
pub const TF_SS_DISJOINTSEL: i32 = 1;
pub const TF_SS_REGIONS: i32 = 2;
pub const TF_SS_TKBAUTOCORRECTENABLE: i32 = 16;
pub const TF_SS_TKBPREDICTIONENABLE: i32 = 32;
pub const TF_SS_TRANSITORY: i32 = 4;
#[cfg(feature = "textstor")]
pub type TF_STATUS = super::TS_STATUS;
pub const TF_ST_CORRECTION: i32 = 1;
pub const TF_S_ASYNC: i32 = 262912;
pub const TF_TF_IGNOREEND: i32 = 2;
pub const TF_TF_MOVESTART: i32 = 1;
pub const TF_TMAE_COMLESS: i32 = 8;
pub const TF_TMAE_CONSOLE: i32 = 64;
pub const TF_TMAE_NOACTIVATEKEYBOARDLAYOUT: i32 = 32;
pub const TF_TMAE_NOACTIVATETIP: i32 = 1;
pub const TF_TMAE_SECUREMODE: i32 = 2;
pub const TF_TMAE_UIELEMENTENABLEDONLY: i32 = 4;
pub const TF_TMAE_WOW16: i32 = 16;
pub const TF_TMF_ACTIVATED: u32 = 2147483648;
pub const TF_TMF_COMLESS: i32 = 8;
pub const TF_TMF_CONSOLE: i32 = 64;
pub const TF_TMF_IMMERSIVEMODE: i32 = 1073741824;
pub const TF_TMF_NOACTIVATETIP: i32 = 1;
pub const TF_TMF_SECUREMODE: i32 = 2;
pub const TF_TMF_UIELEMENTENABLEDONLY: i32 = 4;
pub const TF_TMF_WOW16: i32 = 16;
pub const TF_TRANSITORYEXTENSION_ATSELECTION: i32 = 2;
pub const TF_TRANSITORYEXTENSION_FLOATING: i32 = 1;
pub const TF_TRANSITORYEXTENSION_NONE: i32 = 0;
pub const TF_TU_CORRECTION: i32 = 1;
pub const TF_URP_ALLPROFILES: i32 = 2;
pub const TF_URP_LOCALPROCESS: i32 = 4;
pub const TF_URP_LOCALTHREAD: i32 = 8;
pub const TF_US_HIDETIPUI: i32 = 1;
pub const TKB_ALTERNATES_AUTOCORRECTION_APPLIED: i32 = 4;
pub const TKB_ALTERNATES_FOR_AUTOCORRECTION: i32 = 2;
pub const TKB_ALTERNATES_FOR_PREDICTION: i32 = 3;
pub const TKB_ALTERNATES_STANDARD: i32 = 1;
pub type TfActiveSelEnd = i32;
pub type TfAnchor = i32;
pub type TfClientId = u32;
pub type TfEditCookie = u32;
pub type TfGravity = i32;
pub type TfGuidAtom = u32;
pub type TfLayoutCode = i32;
pub type TfShiftDir = i32;
