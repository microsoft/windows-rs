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
pub const TF_CHAR_EMBEDDED: u32 = 65532;
pub const TF_CLIENTID_NULL: TfClientId = 0;
pub const TF_CLUIE_COUNT: u32 = 2;
pub const TF_CLUIE_CURRENTPAGE: u32 = 32;
pub const TF_CLUIE_DOCUMENTMGR: u32 = 1;
pub const TF_CLUIE_PAGEINDEX: u32 = 16;
pub const TF_CLUIE_SELECTION: u32 = 4;
pub const TF_CLUIE_STRING: u32 = 8;
pub const TF_CONVERSIONMODE_ALPHANUMERIC: u32 = 0;
pub const TF_CONVERSIONMODE_CHARCODE: u32 = 32;
pub const TF_CONVERSIONMODE_EUDC: u32 = 512;
pub const TF_CONVERSIONMODE_FIXED: u32 = 2048;
pub const TF_CONVERSIONMODE_FULLSHAPE: u32 = 8;
pub const TF_CONVERSIONMODE_KATAKANA: u32 = 2;
pub const TF_CONVERSIONMODE_NATIVE: u32 = 1;
pub const TF_CONVERSIONMODE_NOCONVERSION: u32 = 256;
pub const TF_CONVERSIONMODE_ROMAN: u32 = 16;
pub const TF_CONVERSIONMODE_SOFTKEYBOARD: u32 = 128;
pub const TF_CONVERSIONMODE_SYMBOL: u32 = 1024;
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
    pub cr: super::windef::COLORREF,
}
#[cfg(feature = "windef")]
impl Default for TF_DA_COLOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TF_DA_COLORTYPE = i32;
pub type TF_DA_LINESTYLE = i32;
pub const TF_DEFAULT_SELECTION: i32 = -1;
pub const TF_DISABLE_COMMANDING: u32 = 4;
pub const TF_DISABLE_DICTATION: u32 = 2;
pub const TF_DISABLE_SPEECH: u32 = 1;
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
pub const TF_ES_ASYNC: u32 = 8;
pub const TF_ES_ASYNCDONTCARE: u32 = 0;
pub const TF_ES_READ: u32 = 2;
pub const TF_ES_READWRITE: u32 = 6;
pub const TF_ES_SYNC: u32 = 1;
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
pub const TF_GTP_INCL_TEXT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TF_HALTCOND {
    pub pHaltRange: *mut core::ffi::c_void,
    pub aHaltPos: TfAnchor,
    pub dwFlags: u32,
}
impl Default for TF_HALTCOND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_HF_OBJECT: u32 = 1;
pub const TF_IAS_NOQUERY: u32 = 1;
pub const TF_IAS_NO_DEFAULT_COMPOSITION: u32 = 2147483648;
pub const TF_IAS_QUERYONLY: u32 = 2;
pub const TF_IE_CORRECTION: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct TF_INPUTPROCESSORPROFILE {
    pub dwProfileType: u32,
    pub langid: super::winnt::LANGID,
    pub clsid: windows_sys::core::GUID,
    pub guidProfile: windows_sys::core::GUID,
    pub catid: windows_sys::core::GUID,
    pub hklSubstitute: super::minwindef::HKL,
    pub dwCaps: u32,
    pub hkl: super::minwindef::HKL,
    pub dwFlags: u32,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for TF_INPUTPROCESSORPROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_INVALID_COOKIE: u32 = 4294967295;
pub const TF_INVALID_EDIT_COOKIE: u32 = 0;
pub const TF_INVALID_GUIDATOM: TfGuidAtom = 0;
pub const TF_INVALID_UIELEMENTID: i32 = -1;
pub const TF_IPPMF_DISABLEPROFILE: u32 = 2;
pub const TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE: u32 = 4;
pub const TF_IPPMF_ENABLEPROFILE: u32 = 1;
pub const TF_IPPMF_FORPROCESS: u32 = 268435456;
pub const TF_IPPMF_FORSESSION: u32 = 536870912;
pub const TF_IPPMF_FORSYSTEMALL: u32 = 1073741824;
pub const TF_IPP_CAPS_COMLESSSUPPORT: u32 = 8;
pub const TF_IPP_CAPS_DISABLEONTRANSITORY: u32 = 1;
pub const TF_IPP_CAPS_IMMERSIVESUPPORT: u32 = 65536;
pub const TF_IPP_CAPS_SECUREMODESUPPORT: u32 = 2;
pub const TF_IPP_CAPS_SYSTRAYSUPPORT: u32 = 131072;
pub const TF_IPP_CAPS_UIELEMENTENABLED: u32 = 4;
pub const TF_IPP_CAPS_WOW16SUPPORT: u32 = 16;
pub const TF_IPP_FLAG_ACTIVE: u32 = 1;
pub const TF_IPP_FLAG_ENABLED: u32 = 2;
pub const TF_IPP_FLAG_SUBSTITUTEDBYINPUTPROCESSOR: u32 = 4;
pub const TF_IPSINK_FLAG_ACTIVE: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct TF_LANGUAGEPROFILE {
    pub clsid: windows_sys::core::GUID,
    pub langid: super::winnt::LANGID,
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
pub const TF_MOD_ALT: u32 = 1;
pub const TF_MOD_CONTROL: u32 = 2;
pub const TF_MOD_IGNORE_ALL_MODIFIER: u32 = 1024;
pub const TF_MOD_LALT: u32 = 64;
pub const TF_MOD_LCONTROL: u32 = 128;
pub const TF_MOD_LSHIFT: u32 = 256;
pub const TF_MOD_ON_KEYUP: u32 = 512;
pub const TF_MOD_RALT: u32 = 8;
pub const TF_MOD_RCONTROL: u32 = 16;
pub const TF_MOD_RSHIFT: u32 = 32;
pub const TF_MOD_SHIFT: u32 = 4;
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
pub const TF_POPF_ALL: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TF_PRESERVEDKEY {
    pub uVKey: u32,
    pub uModifiers: u32,
}
pub const TF_PROFILETYPE_INPUTPROCESSOR: u32 = 1;
pub const TF_PROFILETYPE_KEYBOARDLAYOUT: u32 = 2;
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
    pub varValue: super::oaidl::VARIANT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for TF_PROPERTYVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_RCM_COMLESS: u32 = 1;
pub const TF_RCM_HINT_COLLISION: u32 = 8;
pub const TF_RCM_HINT_READING_LENGTH: u32 = 4;
pub const TF_RCM_VKEY: u32 = 2;
pub const TF_RIP_FLAG_FREEUNUSEDLIBRARIES: u32 = 1;
pub const TF_RIUIE_CONTEXT: u32 = 1;
pub const TF_RIUIE_ERRORINDEX: u32 = 8;
pub const TF_RIUIE_MAXREADINGSTRINGLENGTH: u32 = 4;
pub const TF_RIUIE_STRING: u32 = 2;
pub const TF_RIUIE_VERTICALORDER: u32 = 16;
pub const TF_RP_HIDDENINSETTINGUI: u32 = 2;
pub const TF_RP_LOCALPROCESS: u32 = 4;
pub const TF_RP_LOCALTHREAD: u32 = 8;
pub const TF_RP_SUBITEMINSETTINGUI: u32 = 16;
pub const TF_SD_BACKWARD: TfShiftDir = 0;
pub const TF_SD_FORWARD: TfShiftDir = 1;
pub const TF_SD_LOADING: u32 = 2;
pub const TF_SD_READONLY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TF_SELECTION {
    pub range: *mut core::ffi::c_void,
    pub style: TF_SELECTIONSTYLE,
}
impl Default for TF_SELECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TF_SELECTIONSTYLE {
    pub ase: TfActiveSelEnd,
    pub fInterimChar: windows_sys::core::BOOL,
}
pub const TF_SENTENCEMODE_AUTOMATIC: u32 = 4;
pub const TF_SENTENCEMODE_CONVERSATION: u32 = 16;
pub const TF_SENTENCEMODE_NONE: u32 = 0;
pub const TF_SENTENCEMODE_PHRASEPREDICT: u32 = 8;
pub const TF_SENTENCEMODE_PLAURALCLAUSE: u32 = 1;
pub const TF_SENTENCEMODE_SINGLECONVERT: u32 = 2;
pub const TF_SS_DISJOINTSEL: u32 = 1;
pub const TF_SS_REGIONS: u32 = 2;
pub const TF_SS_TKBAUTOCORRECTENABLE: u32 = 16;
pub const TF_SS_TKBPREDICTIONENABLE: u32 = 32;
pub const TF_SS_TRANSITORY: u32 = 4;
#[cfg(feature = "textstor")]
pub type TF_STATUS = super::textstor::TS_STATUS;
pub const TF_ST_CORRECTION: u32 = 1;
pub const TF_S_ASYNC: u32 = 262912;
pub const TF_TF_IGNOREEND: u32 = 2;
pub const TF_TF_MOVESTART: u32 = 1;
pub const TF_TMAE_COMLESS: u32 = 8;
pub const TF_TMAE_CONSOLE: u32 = 64;
pub const TF_TMAE_NOACTIVATEKEYBOARDLAYOUT: u32 = 32;
pub const TF_TMAE_NOACTIVATETIP: u32 = 1;
pub const TF_TMAE_SECUREMODE: u32 = 2;
pub const TF_TMAE_UIELEMENTENABLEDONLY: u32 = 4;
pub const TF_TMAE_WOW16: u32 = 16;
pub const TF_TMF_ACTIVATED: u32 = 2147483648;
pub const TF_TMF_COMLESS: u32 = 8;
pub const TF_TMF_CONSOLE: u32 = 64;
pub const TF_TMF_IMMERSIVEMODE: u32 = 1073741824;
pub const TF_TMF_NOACTIVATETIP: u32 = 1;
pub const TF_TMF_SECUREMODE: u32 = 2;
pub const TF_TMF_UIELEMENTENABLEDONLY: u32 = 4;
pub const TF_TMF_WOW16: u32 = 16;
pub const TF_TRANSITORYEXTENSION_ATSELECTION: u32 = 2;
pub const TF_TRANSITORYEXTENSION_FLOATING: u32 = 1;
pub const TF_TRANSITORYEXTENSION_NONE: u32 = 0;
pub const TF_TU_CORRECTION: u32 = 1;
pub const TF_URP_ALLPROFILES: u32 = 2;
pub const TF_URP_LOCALPROCESS: u32 = 4;
pub const TF_URP_LOCALTHREAD: u32 = 8;
pub const TF_US_HIDETIPUI: u32 = 1;
pub const TKB_ALTERNATES_AUTOCORRECTION_APPLIED: u32 = 4;
pub const TKB_ALTERNATES_FOR_AUTOCORRECTION: u32 = 2;
pub const TKB_ALTERNATES_FOR_PREDICTION: u32 = 3;
pub const TKB_ALTERNATES_STANDARD: u32 = 1;
pub type TfActiveSelEnd = i32;
pub type TfAnchor = i32;
pub type TfClientId = u32;
pub type TfEditCookie = u32;
pub type TfGravity = i32;
pub type TfGuidAtom = u32;
pub type TfLayoutCode = i32;
pub type TfShiftDir = i32;
