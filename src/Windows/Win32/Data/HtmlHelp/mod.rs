#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_IITCmdInt: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1180883618, 54163, 4560, [154, 86, 0, 192, 79, 182, 139, 247]);
pub const CLSID_IITDatabase: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1718039634, 35875, 4560, [168, 78, 0, 170, 0, 108, 125, 1]);
pub const CLSID_IITDatabaseLocal: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1180883625, 54163, 4560, [154, 86, 0, 192, 79, 182, 139, 247]);
pub const CLSID_IITGroupUpdate: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1180883620, 54163, 4560, [154, 86, 0, 192, 79, 182, 139, 247]);
pub const CLSID_IITIndexBuild: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2409682346, 57055, 4560, [154, 97, 0, 192, 79, 182, 139, 247]);
pub const CLSID_IITPropList: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1180883630, 54163, 4560, [154, 86, 0, 192, 79, 182, 139, 247]);
pub const CLSID_IITResultSet: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1180883623, 54163, 4560, [154, 86, 0, 192, 79, 182, 139, 247]);
pub const CLSID_IITSvMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1180883619, 54163, 4560, [154, 86, 0, 192, 79, 182, 139, 247]);
pub const CLSID_IITWWFilterBuild: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2409682347, 57055, 4560, [154, 97, 0, 192, 79, 182, 139, 247]);
pub const CLSID_IITWordWheel: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3610715586, 35858, 4560, [168, 78, 0, 170, 0, 108, 125, 1]);
pub const CLSID_IITWordWheelLocal: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1180883624, 54163, 4560, [154, 86, 0, 192, 79, 182, 139, 247]);
pub const CLSID_IITWordWheelUpdate: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1180883621, 54163, 4560, [154, 86, 0, 192, 79, 182, 139, 247]);
pub const CLSID_ITEngStemmer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2409682344, 57055, 4560, [154, 97, 0, 192, 79, 182, 139, 247]);
pub const CLSID_ITStdBreaker: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1180883631, 54163, 4560, [154, 86, 0, 192, 79, 182, 139, 247]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COLUMNSTATUS {
    pub cPropCount: i32,
    pub cPropsLoaded: i32,
}
impl COLUMNSTATUS {}
impl ::std::default::Default for COLUMNSTATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COLUMNSTATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COLUMNSTATUS").field("cPropCount", &self.cPropCount).field("cPropsLoaded", &self.cPropsLoaded).finish()
    }
}
impl ::std::cmp::PartialEq for COLUMNSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.cPropCount == other.cPropCount && self.cPropsLoaded == other.cPropsLoaded
    }
}
impl ::std::cmp::Eq for COLUMNSTATUS {}
unsafe impl ::windows::runtime::Abi for COLUMNSTATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CProperty {
    pub dwPropID: u32,
    pub cbData: u32,
    pub dwType: u32,
    pub Anonymous: CProperty_0,
    pub fPersist: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl CProperty {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CProperty {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CProperty {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CProperty {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CProperty {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union CProperty_0 {
    pub lpszwData: super::super::Foundation::PWSTR,
    pub lpvData: *mut ::std::ffi::c_void,
    pub dwValue: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CProperty_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CProperty_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CProperty_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CProperty_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CProperty_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const E_ALL_WILD: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479467i32 as _);
pub const E_ALREADYINIT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479421i32 as _);
pub const E_ALREADYOPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479533i32 as _);
pub const E_ASSERT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479546i32 as _);
pub const E_BADBREAKER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479469i32 as _);
pub const E_BADFILE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479549i32 as _);
pub const E_BADFILTERSIZE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479528i32 as _);
pub const E_BADFORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479548i32 as _);
pub const E_BADINDEXFLAGS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479456i32 as _);
pub const E_BADPARAM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479535i32 as _);
pub const E_BADRANGEOP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479459i32 as _);
pub const E_BADVALUE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479468i32 as _);
pub const E_BADVERSION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479550i32 as _);
pub const E_CANTFINDDLL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479538i32 as _);
pub const E_DISKFULL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479496i32 as _);
pub const E_DUPLICATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479551i32 as _);
pub const E_EXPECTEDTERM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479465i32 as _);
pub const E_FILECLOSE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479503i32 as _);
pub const E_FILECREATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479504i32 as _);
pub const E_FILEDELETE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479499i32 as _);
pub const E_FILEINVALID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479498i32 as _);
pub const E_FILENOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479497i32 as _);
pub const E_FILEREAD: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479502i32 as _);
pub const E_FILESEEK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479501i32 as _);
pub const E_FILEWRITE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479500i32 as _);
pub const E_GETLASTERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479536i32 as _);
pub const E_GROUPIDTOOBIG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479542i32 as _);
pub const E_INTERRUPT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479545i32 as _);
pub const E_INVALIDSTATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479534i32 as _);
pub const E_MISSINGPROP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479424i32 as _);
pub const E_MISSLPAREN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479464i32 as _);
pub const E_MISSQUOTE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479462i32 as _);
pub const E_MISSRPAREN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479463i32 as _);
pub const E_NAMETOOLONG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479520i32 as _);
pub const E_NOHANDLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479537i32 as _);
pub const E_NOKEYPROP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479417i32 as _);
pub const E_NOMERGEDDATA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479540i32 as _);
pub const E_NOPERMISSION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479547i32 as _);
pub const E_NOSTEMMER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479454i32 as _);
pub const E_NOTEXIST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479552i32 as _);
pub const E_NOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479539i32 as _);
pub const E_NOTINIT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479420i32 as _);
pub const E_NOTOPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479533i32 as _);
pub const E_NOTSUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479544i32 as _);
pub const E_NULLQUERY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479461i32 as _);
pub const E_OUTOFRANGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479543i32 as _);
pub const E_PROPLISTEMPTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479422i32 as _);
pub const E_PROPLISTNOTEMPTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479423i32 as _);
pub const E_RESULTSETEMPTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479419i32 as _);
pub const E_STOPWORD: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479460i32 as _);
pub const E_TOODEEP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479466i32 as _);
pub const E_TOOMANYCOLUMNS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479418i32 as _);
pub const E_TOOMANYDUPS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479471i32 as _);
pub const E_TOOMANYOBJECTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479527i32 as _);
pub const E_TOOMANYTITLES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479541i32 as _);
pub const E_TOOMANYTOPICS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479472i32 as _);
pub const E_TREETOOBIG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479470i32 as _);
pub const E_UNKNOWN_TRANSPORT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479530i32 as _);
pub const E_UNMATCHEDTYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479458i32 as _);
pub const E_UNSUPPORTED_TRANSPORT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479529i32 as _);
pub const E_WILD_IN_DTYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479455i32 as _);
pub const E_WORDTOOLONG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147479457i32 as _);
pub const HHACT_BACK: i32 = 7i32;
pub const HHACT_CONTRACT: i32 = 6i32;
pub const HHACT_CUSTOMIZE: i32 = 16i32;
pub const HHACT_EXPAND: i32 = 5i32;
pub const HHACT_FORWARD: i32 = 8i32;
pub const HHACT_HIGHLIGHT: i32 = 15i32;
pub const HHACT_HOME: i32 = 11i32;
pub const HHACT_JUMP1: i32 = 17i32;
pub const HHACT_JUMP2: i32 = 18i32;
pub const HHACT_LAST_ENUM: i32 = 23i32;
pub const HHACT_NOTES: i32 = 22i32;
pub const HHACT_OPTIONS: i32 = 13i32;
pub const HHACT_PRINT: i32 = 14i32;
pub const HHACT_REFRESH: i32 = 10i32;
pub const HHACT_STOP: i32 = 9i32;
pub const HHACT_SYNC: i32 = 12i32;
pub const HHACT_TAB_CONTENTS: i32 = 0i32;
pub const HHACT_TAB_FAVORITES: i32 = 4i32;
pub const HHACT_TAB_HISTORY: i32 = 3i32;
pub const HHACT_TAB_INDEX: i32 = 1i32;
pub const HHACT_TAB_SEARCH: i32 = 2i32;
pub const HHACT_TOC_NEXT: i32 = 20i32;
pub const HHACT_TOC_PREV: i32 = 21i32;
pub const HHACT_ZOOM: i32 = 19i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct HHNTRACK {
    pub hdr: super::super::UI::Controls::NMHDR,
    pub pszCurUrl: super::super::Foundation::PSTR,
    pub idAction: i32,
    pub phhWinType: *mut HH_WINTYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl HHNTRACK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::default::Default for HHNTRACK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::fmt::Debug for HHNTRACK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HHNTRACK").field("hdr", &self.hdr).field("pszCurUrl", &self.pszCurUrl).field("idAction", &self.idAction).field("phhWinType", &self.phhWinType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::cmp::PartialEq for HHNTRACK {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszCurUrl == other.pszCurUrl && self.idAction == other.idAction && self.phhWinType == other.phhWinType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::cmp::Eq for HHNTRACK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
unsafe impl ::windows::runtime::Abi for HHNTRACK {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct HHN_NOTIFY {
    pub hdr: super::super::UI::Controls::NMHDR,
    pub pszUrl: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl HHN_NOTIFY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::default::Default for HHN_NOTIFY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::fmt::Debug for HHN_NOTIFY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HHN_NOTIFY").field("hdr", &self.hdr).field("pszUrl", &self.pszUrl).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::cmp::PartialEq for HHN_NOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszUrl == other.pszUrl
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::cmp::Eq for HHN_NOTIFY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
unsafe impl ::windows::runtime::Abi for HHN_NOTIFY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HHWIN_BUTTON_BACK: u32 = 4u32;
pub const HHWIN_BUTTON_BROWSE_BCK: u32 = 256u32;
pub const HHWIN_BUTTON_BROWSE_FWD: u32 = 128u32;
pub const HHWIN_BUTTON_CONTENTS: u32 = 1024u32;
pub const HHWIN_BUTTON_EXPAND: u32 = 2u32;
pub const HHWIN_BUTTON_FAVORITES: u32 = 131072u32;
pub const HHWIN_BUTTON_FORWARD: u32 = 8u32;
pub const HHWIN_BUTTON_HISTORY: u32 = 65536u32;
pub const HHWIN_BUTTON_HOME: u32 = 64u32;
pub const HHWIN_BUTTON_INDEX: u32 = 16384u32;
pub const HHWIN_BUTTON_JUMP1: u32 = 262144u32;
pub const HHWIN_BUTTON_JUMP2: u32 = 524288u32;
pub const HHWIN_BUTTON_NOTES: u32 = 512u32;
pub const HHWIN_BUTTON_OPTIONS: u32 = 4096u32;
pub const HHWIN_BUTTON_PRINT: u32 = 8192u32;
pub const HHWIN_BUTTON_REFRESH: u32 = 32u32;
pub const HHWIN_BUTTON_SEARCH: u32 = 32768u32;
pub const HHWIN_BUTTON_STOP: u32 = 16u32;
pub const HHWIN_BUTTON_SYNC: u32 = 2048u32;
pub const HHWIN_BUTTON_TOC_NEXT: u32 = 2097152u32;
pub const HHWIN_BUTTON_TOC_PREV: u32 = 4194304u32;
pub const HHWIN_BUTTON_ZOOM: u32 = 1048576u32;
pub const HHWIN_NAVTAB_BOTTOM: i32 = 2i32;
pub const HHWIN_NAVTAB_LEFT: i32 = 1i32;
pub const HHWIN_NAVTAB_TOP: i32 = 0i32;
pub const HHWIN_NAVTYPE_AUTHOR: i32 = 5i32;
pub const HHWIN_NAVTYPE_CUSTOM_FIRST: i32 = 11i32;
pub const HHWIN_NAVTYPE_FAVORITES: i32 = 3i32;
pub const HHWIN_NAVTYPE_HISTORY: i32 = 4i32;
pub const HHWIN_NAVTYPE_INDEX: i32 = 1i32;
pub const HHWIN_NAVTYPE_SEARCH: i32 = 2i32;
pub const HHWIN_NAVTYPE_TOC: i32 = 0i32;
pub const HHWIN_PARAM_CUR_TAB: u32 = 8192u32;
pub const HHWIN_PARAM_EXPANSION: u32 = 512u32;
pub const HHWIN_PARAM_EXSTYLES: u32 = 8u32;
pub const HHWIN_PARAM_HISTORY_COUNT: u32 = 4096u32;
pub const HHWIN_PARAM_INFOTYPES: u32 = 128u32;
pub const HHWIN_PARAM_NAV_WIDTH: u32 = 32u32;
pub const HHWIN_PARAM_PROPERTIES: u32 = 2u32;
pub const HHWIN_PARAM_RECT: u32 = 16u32;
pub const HHWIN_PARAM_SHOWSTATE: u32 = 64u32;
pub const HHWIN_PARAM_STYLES: u32 = 4u32;
pub const HHWIN_PARAM_TABORDER: u32 = 2048u32;
pub const HHWIN_PARAM_TABPOS: u32 = 1024u32;
pub const HHWIN_PARAM_TB_FLAGS: u32 = 256u32;
pub const HHWIN_PROP_AUTO_SYNC: u32 = 256u32;
pub const HHWIN_PROP_CHANGE_TITLE: u32 = 8192u32;
pub const HHWIN_PROP_MENU: u32 = 65536u32;
pub const HHWIN_PROP_NAV_ONLY_WIN: u32 = 16384u32;
pub const HHWIN_PROP_NODEF_EXSTYLES: u32 = 16u32;
pub const HHWIN_PROP_NODEF_STYLES: u32 = 8u32;
pub const HHWIN_PROP_NOTB_TEXT: u32 = 64u32;
pub const HHWIN_PROP_NOTITLEBAR: u32 = 4u32;
pub const HHWIN_PROP_NO_TOOLBAR: u32 = 32768u32;
pub const HHWIN_PROP_ONTOP: u32 = 2u32;
pub const HHWIN_PROP_POST_QUIT: u32 = 128u32;
pub const HHWIN_PROP_TAB_ADVSEARCH: u32 = 131072u32;
pub const HHWIN_PROP_TAB_AUTOHIDESHOW: u32 = 1u32;
pub const HHWIN_PROP_TAB_CUSTOM1: u32 = 524288u32;
pub const HHWIN_PROP_TAB_CUSTOM2: u32 = 1048576u32;
pub const HHWIN_PROP_TAB_CUSTOM3: u32 = 2097152u32;
pub const HHWIN_PROP_TAB_CUSTOM4: u32 = 4194304u32;
pub const HHWIN_PROP_TAB_CUSTOM5: u32 = 8388608u32;
pub const HHWIN_PROP_TAB_CUSTOM6: u32 = 16777216u32;
pub const HHWIN_PROP_TAB_CUSTOM7: u32 = 33554432u32;
pub const HHWIN_PROP_TAB_CUSTOM8: u32 = 67108864u32;
pub const HHWIN_PROP_TAB_CUSTOM9: u32 = 134217728u32;
pub const HHWIN_PROP_TAB_FAVORITES: u32 = 4096u32;
pub const HHWIN_PROP_TAB_HISTORY: u32 = 2048u32;
pub const HHWIN_PROP_TAB_SEARCH: u32 = 1024u32;
pub const HHWIN_PROP_TRACKING: u32 = 512u32;
pub const HHWIN_PROP_TRI_PANE: u32 = 32u32;
pub const HHWIN_PROP_USER_POS: u32 = 262144u32;
pub const HHWIN_TB_MARGIN: u32 = 268435456u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_AKLINK {
    pub cbStruct: i32,
    pub fReserved: super::super::Foundation::BOOL,
    pub pszKeywords: *mut i8,
    pub pszUrl: *mut i8,
    pub pszMsgText: *mut i8,
    pub pszMsgTitle: *mut i8,
    pub pszWindow: *mut i8,
    pub fIndexOnFail: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_AKLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HH_AKLINK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HH_AKLINK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HH_AKLINK")
            .field("cbStruct", &self.cbStruct)
            .field("fReserved", &self.fReserved)
            .field("pszKeywords", &self.pszKeywords)
            .field("pszUrl", &self.pszUrl)
            .field("pszMsgText", &self.pszMsgText)
            .field("pszMsgTitle", &self.pszMsgTitle)
            .field("pszWindow", &self.pszWindow)
            .field("fIndexOnFail", &self.fIndexOnFail)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HH_AKLINK {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.fReserved == other.fReserved && self.pszKeywords == other.pszKeywords && self.pszUrl == other.pszUrl && self.pszMsgText == other.pszMsgText && self.pszMsgTitle == other.pszMsgTitle && self.pszWindow == other.pszWindow && self.fIndexOnFail == other.fIndexOnFail
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HH_AKLINK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HH_AKLINK {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HH_ALINK_LOOKUP: u32 = 19u32;
pub const HH_CLOSE_ALL: u32 = 18u32;
pub const HH_DISPLAY_INDEX: u32 = 2u32;
pub const HH_DISPLAY_SEARCH: u32 = 3u32;
pub const HH_DISPLAY_TEXT_POPUP: u32 = 14u32;
pub const HH_DISPLAY_TOC: u32 = 1u32;
pub const HH_DISPLAY_TOPIC: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_ENUM_CAT {
    pub cbStruct: i32,
    pub pszCatName: super::super::Foundation::PSTR,
    pub pszCatDescription: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_ENUM_CAT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HH_ENUM_CAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HH_ENUM_CAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HH_ENUM_CAT").field("cbStruct", &self.cbStruct).field("pszCatName", &self.pszCatName).field("pszCatDescription", &self.pszCatDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HH_ENUM_CAT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pszCatName == other.pszCatName && self.pszCatDescription == other.pszCatDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HH_ENUM_CAT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HH_ENUM_CAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HH_ENUM_CATEGORY: u32 = 21u32;
pub const HH_ENUM_CATEGORY_IT: u32 = 22u32;
pub const HH_ENUM_INFO_TYPE: u32 = 7u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_ENUM_IT {
    pub cbStruct: i32,
    pub iType: i32,
    pub pszCatName: super::super::Foundation::PSTR,
    pub pszITName: super::super::Foundation::PSTR,
    pub pszITDescription: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_ENUM_IT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HH_ENUM_IT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HH_ENUM_IT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HH_ENUM_IT").field("cbStruct", &self.cbStruct).field("iType", &self.iType).field("pszCatName", &self.pszCatName).field("pszITName", &self.pszITName).field("pszITDescription", &self.pszITDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HH_ENUM_IT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.iType == other.iType && self.pszCatName == other.pszCatName && self.pszITName == other.pszITName && self.pszITDescription == other.pszITDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HH_ENUM_IT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HH_ENUM_IT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HH_FTS_DEFAULT_PROXIMITY: i32 = -1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_FTS_QUERY {
    pub cbStruct: i32,
    pub fUniCodeStrings: super::super::Foundation::BOOL,
    pub pszSearchQuery: *mut i8,
    pub iProximity: i32,
    pub fStemmedSearch: super::super::Foundation::BOOL,
    pub fTitleOnly: super::super::Foundation::BOOL,
    pub fExecute: super::super::Foundation::BOOL,
    pub pszWindow: *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_FTS_QUERY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HH_FTS_QUERY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HH_FTS_QUERY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HH_FTS_QUERY")
            .field("cbStruct", &self.cbStruct)
            .field("fUniCodeStrings", &self.fUniCodeStrings)
            .field("pszSearchQuery", &self.pszSearchQuery)
            .field("iProximity", &self.iProximity)
            .field("fStemmedSearch", &self.fStemmedSearch)
            .field("fTitleOnly", &self.fTitleOnly)
            .field("fExecute", &self.fExecute)
            .field("pszWindow", &self.pszWindow)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HH_FTS_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.fUniCodeStrings == other.fUniCodeStrings && self.pszSearchQuery == other.pszSearchQuery && self.iProximity == other.iProximity && self.fStemmedSearch == other.fStemmedSearch && self.fTitleOnly == other.fTitleOnly && self.fExecute == other.fExecute && self.pszWindow == other.pszWindow
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HH_FTS_QUERY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HH_FTS_QUERY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HH_GET_LAST_ERROR: u32 = 20u32;
pub const HH_GET_WIN_HANDLE: u32 = 6u32;
pub const HH_GET_WIN_TYPE: u32 = 5u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl ::std::clone::Clone for HH_GLOBAL_PROPERTY {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
pub struct HH_GLOBAL_PROPERTY {
    pub id: HH_GPROPID,
    pub var: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl HH_GLOBAL_PROPERTY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for HH_GLOBAL_PROPERTY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for HH_GLOBAL_PROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for HH_GLOBAL_PROPERTY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for HH_GLOBAL_PROPERTY {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HH_GPROPID(pub i32);
pub const HH_GPROPID_SINGLETHREAD: HH_GPROPID = HH_GPROPID(1i32);
pub const HH_GPROPID_TOOLBAR_MARGIN: HH_GPROPID = HH_GPROPID(2i32);
pub const HH_GPROPID_UI_LANGUAGE: HH_GPROPID = HH_GPROPID(3i32);
pub const HH_GPROPID_CURRENT_SUBSET: HH_GPROPID = HH_GPROPID(4i32);
pub const HH_GPROPID_CONTENT_LANGUAGE: HH_GPROPID = HH_GPROPID(5i32);
impl ::std::convert::From<i32> for HH_GPROPID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HH_GPROPID {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HH_HELP_CONTEXT: u32 = 15u32;
pub const HH_HELP_FINDER: u32 = 0u32;
pub const HH_INITIALIZE: u32 = 28u32;
pub const HH_KEYWORD_LOOKUP: u32 = 13u32;
pub const HH_MAX_TABS: u32 = 19u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_POPUP {
    pub cbStruct: i32,
    pub hinst: super::super::Foundation::HINSTANCE,
    pub idString: u32,
    pub pszText: *mut i8,
    pub pt: super::super::Foundation::POINT,
    pub clrForeground: u32,
    pub clrBackground: u32,
    pub rcMargins: super::super::Foundation::RECT,
    pub pszFont: *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_POPUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HH_POPUP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HH_POPUP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HH_POPUP")
            .field("cbStruct", &self.cbStruct)
            .field("hinst", &self.hinst)
            .field("idString", &self.idString)
            .field("pszText", &self.pszText)
            .field("pt", &self.pt)
            .field("clrForeground", &self.clrForeground)
            .field("clrBackground", &self.clrBackground)
            .field("rcMargins", &self.rcMargins)
            .field("pszFont", &self.pszFont)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HH_POPUP {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.hinst == other.hinst && self.idString == other.idString && self.pszText == other.pszText && self.pt == other.pt && self.clrForeground == other.clrForeground && self.clrBackground == other.clrBackground && self.rcMargins == other.rcMargins && self.pszFont == other.pszFont
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HH_POPUP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HH_POPUP {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HH_PRETRANSLATEMESSAGE: u32 = 253u32;
pub const HH_RESERVED1: u32 = 10u32;
pub const HH_RESERVED2: u32 = 11u32;
pub const HH_RESERVED3: u32 = 12u32;
pub const HH_RESET_IT_FILTER: u32 = 23u32;
pub const HH_SAFE_DISPLAY_TOPIC: u32 = 32u32;
pub const HH_SET_EXCLUSIVE_FILTER: u32 = 25u32;
pub const HH_SET_GLOBAL_PROPERTY: u32 = 252u32;
pub const HH_SET_INCLUSIVE_FILTER: u32 = 24u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_SET_INFOTYPE {
    pub cbStruct: i32,
    pub pszCatName: super::super::Foundation::PSTR,
    pub pszInfoTypeName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_SET_INFOTYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HH_SET_INFOTYPE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HH_SET_INFOTYPE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HH_SET_INFOTYPE").field("cbStruct", &self.cbStruct).field("pszCatName", &self.pszCatName).field("pszInfoTypeName", &self.pszInfoTypeName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HH_SET_INFOTYPE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pszCatName == other.pszCatName && self.pszInfoTypeName == other.pszInfoTypeName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HH_SET_INFOTYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HH_SET_INFOTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HH_SET_INFO_TYPE: u32 = 8u32;
pub const HH_SET_QUERYSERVICE: u32 = 30u32;
pub const HH_SET_WIN_TYPE: u32 = 4u32;
pub const HH_SYNC: u32 = 9u32;
pub const HH_TAB_AUTHOR: i32 = 5i32;
pub const HH_TAB_CONTENTS: i32 = 0i32;
pub const HH_TAB_CUSTOM_FIRST: i32 = 11i32;
pub const HH_TAB_CUSTOM_LAST: i32 = 19i32;
pub const HH_TAB_FAVORITES: i32 = 3i32;
pub const HH_TAB_HISTORY: i32 = 4i32;
pub const HH_TAB_INDEX: i32 = 1i32;
pub const HH_TAB_SEARCH: i32 = 2i32;
pub const HH_TP_HELP_CONTEXTMENU: u32 = 16u32;
pub const HH_TP_HELP_WM_HELP: u32 = 17u32;
pub const HH_UNINITIALIZE: u32 = 29u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HH_WINTYPE {
    pub cbStruct: i32,
    pub fUniCodeStrings: super::super::Foundation::BOOL,
    pub pszType: *mut i8,
    pub fsValidMembers: u32,
    pub fsWinProperties: u32,
    pub pszCaption: *mut i8,
    pub dwStyles: u32,
    pub dwExStyles: u32,
    pub rcWindowPos: super::super::Foundation::RECT,
    pub nShowState: i32,
    pub hwndHelp: super::super::Foundation::HWND,
    pub hwndCaller: super::super::Foundation::HWND,
    pub paInfoTypes: *mut u32,
    pub hwndToolBar: super::super::Foundation::HWND,
    pub hwndNavigation: super::super::Foundation::HWND,
    pub hwndHTML: super::super::Foundation::HWND,
    pub iNavWidth: i32,
    pub rcHTML: super::super::Foundation::RECT,
    pub pszToc: *mut i8,
    pub pszIndex: *mut i8,
    pub pszFile: *mut i8,
    pub pszHome: *mut i8,
    pub fsToolBarFlags: u32,
    pub fNotExpanded: super::super::Foundation::BOOL,
    pub curNavType: i32,
    pub tabpos: i32,
    pub idNotify: i32,
    pub tabOrder: [u8; 20],
    pub cHistory: i32,
    pub pszJump1: *mut i8,
    pub pszJump2: *mut i8,
    pub pszUrlJump1: *mut i8,
    pub pszUrlJump2: *mut i8,
    pub rcMinSize: super::super::Foundation::RECT,
    pub cbInfoTypes: i32,
    pub pszCustomTabs: *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl HH_WINTYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HH_WINTYPE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HH_WINTYPE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HH_WINTYPE")
            .field("cbStruct", &self.cbStruct)
            .field("fUniCodeStrings", &self.fUniCodeStrings)
            .field("pszType", &self.pszType)
            .field("fsValidMembers", &self.fsValidMembers)
            .field("fsWinProperties", &self.fsWinProperties)
            .field("pszCaption", &self.pszCaption)
            .field("dwStyles", &self.dwStyles)
            .field("dwExStyles", &self.dwExStyles)
            .field("rcWindowPos", &self.rcWindowPos)
            .field("nShowState", &self.nShowState)
            .field("hwndHelp", &self.hwndHelp)
            .field("hwndCaller", &self.hwndCaller)
            .field("paInfoTypes", &self.paInfoTypes)
            .field("hwndToolBar", &self.hwndToolBar)
            .field("hwndNavigation", &self.hwndNavigation)
            .field("hwndHTML", &self.hwndHTML)
            .field("iNavWidth", &self.iNavWidth)
            .field("rcHTML", &self.rcHTML)
            .field("pszToc", &self.pszToc)
            .field("pszIndex", &self.pszIndex)
            .field("pszFile", &self.pszFile)
            .field("pszHome", &self.pszHome)
            .field("fsToolBarFlags", &self.fsToolBarFlags)
            .field("fNotExpanded", &self.fNotExpanded)
            .field("curNavType", &self.curNavType)
            .field("tabpos", &self.tabpos)
            .field("idNotify", &self.idNotify)
            .field("tabOrder", &self.tabOrder)
            .field("cHistory", &self.cHistory)
            .field("pszJump1", &self.pszJump1)
            .field("pszJump2", &self.pszJump2)
            .field("pszUrlJump1", &self.pszUrlJump1)
            .field("pszUrlJump2", &self.pszUrlJump2)
            .field("rcMinSize", &self.rcMinSize)
            .field("cbInfoTypes", &self.cbInfoTypes)
            .field("pszCustomTabs", &self.pszCustomTabs)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HH_WINTYPE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.fUniCodeStrings == other.fUniCodeStrings
            && self.pszType == other.pszType
            && self.fsValidMembers == other.fsValidMembers
            && self.fsWinProperties == other.fsWinProperties
            && self.pszCaption == other.pszCaption
            && self.dwStyles == other.dwStyles
            && self.dwExStyles == other.dwExStyles
            && self.rcWindowPos == other.rcWindowPos
            && self.nShowState == other.nShowState
            && self.hwndHelp == other.hwndHelp
            && self.hwndCaller == other.hwndCaller
            && self.paInfoTypes == other.paInfoTypes
            && self.hwndToolBar == other.hwndToolBar
            && self.hwndNavigation == other.hwndNavigation
            && self.hwndHTML == other.hwndHTML
            && self.iNavWidth == other.iNavWidth
            && self.rcHTML == other.rcHTML
            && self.pszToc == other.pszToc
            && self.pszIndex == other.pszIndex
            && self.pszFile == other.pszFile
            && self.pszHome == other.pszHome
            && self.fsToolBarFlags == other.fsToolBarFlags
            && self.fNotExpanded == other.fNotExpanded
            && self.curNavType == other.curNavType
            && self.tabpos == other.tabpos
            && self.idNotify == other.idNotify
            && self.tabOrder == other.tabOrder
            && self.cHistory == other.cHistory
            && self.pszJump1 == other.pszJump1
            && self.pszJump2 == other.pszJump2
            && self.pszUrlJump1 == other.pszUrlJump1
            && self.pszUrlJump2 == other.pszUrlJump2
            && self.rcMinSize == other.rcMinSize
            && self.cbInfoTypes == other.cbInfoTypes
            && self.pszCustomTabs == other.pszCustomTabs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HH_WINTYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HH_WINTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IDTB_BACK: u32 = 204u32;
pub const IDTB_BROWSE_BACK: u32 = 212u32;
pub const IDTB_BROWSE_FWD: u32 = 211u32;
pub const IDTB_CONTENTS: u32 = 213u32;
pub const IDTB_CONTRACT: u32 = 201u32;
pub const IDTB_CUSTOMIZE: u32 = 221u32;
pub const IDTB_EXPAND: u32 = 200u32;
pub const IDTB_FAVORITES: u32 = 217u32;
pub const IDTB_FORWARD: u32 = 209u32;
pub const IDTB_HISTORY: u32 = 216u32;
pub const IDTB_HOME: u32 = 205u32;
pub const IDTB_INDEX: u32 = 214u32;
pub const IDTB_JUMP1: u32 = 218u32;
pub const IDTB_JUMP2: u32 = 219u32;
pub const IDTB_NOTES: u32 = 210u32;
pub const IDTB_OPTIONS: u32 = 208u32;
pub const IDTB_PRINT: u32 = 207u32;
pub const IDTB_REFRESH: u32 = 203u32;
pub const IDTB_SEARCH: u32 = 215u32;
pub const IDTB_STOP: u32 = 202u32;
pub const IDTB_SYNC: u32 = 206u32;
pub const IDTB_TOC_NEXT: u32 = 223u32;
pub const IDTB_TOC_PREV: u32 = 224u32;
pub const IDTB_ZOOM: u32 = 222u32;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IITDatabase(::windows::runtime::IUnknown);
impl IITDatabase {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lpszhost: Param0, lpszmoniker: Param1, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), lpszhost.into_param().abi(), lpszmoniker.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CreateObject(&self, rclsid: *const ::windows::runtime::GUID, pdwobjinstance: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(rclsid), ::std::mem::transmute(pdwobjinstance)).ok()
    }
    pub unsafe fn GetObject(&self, dwobjinstance: u32, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwobjinstance), ::std::mem::transmute(riid), ::std::mem::transmute(ppvobj)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectPersistence<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, lpwszobject: Param0, dwobjinstance: u32, ppvpersistence: *mut *mut ::std::ffi::c_void, fstream: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), lpwszobject.into_param().abi(), ::std::mem::transmute(dwobjinstance), ::std::mem::transmute(ppvpersistence), fstream.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IITDatabase {
    type Vtable = IITDatabase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2409682338, 57055, 4560, [154, 97, 0, 192, 79, 182, 139, 247]);
}
impl ::std::convert::From<IITDatabase> for ::windows::runtime::IUnknown {
    fn from(value: IITDatabase) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IITDatabase> for ::windows::runtime::IUnknown {
    fn from(value: &IITDatabase) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IITDatabase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IITDatabase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IITDatabase_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpszhost: super::super::Foundation::PWSTR, lpszmoniker: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, pdwobjinstance: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwobjinstance: u32, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpwszobject: super::super::Foundation::PWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut ::std::ffi::c_void, fstream: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IITGroup(pub u8);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IITPropList(::windows::runtime::IUnknown);
impl IITPropList {
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstm: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn InitNew(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Set<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, propid: u32, lpszwstring: Param1, dwoperation: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(propid), lpszwstring.into_param().abi(), ::std::mem::transmute(dwoperation)).ok()
    }
    pub unsafe fn Set2(&self, propid: u32, lpvdata: *mut ::std::ffi::c_void, cbdata: u32, dwoperation: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(propid), ::std::mem::transmute(lpvdata), ::std::mem::transmute(cbdata), ::std::mem::transmute(dwoperation)).ok()
    }
    pub unsafe fn Set3(&self, propid: u32, dwdata: u32, dwoperation: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(propid), ::std::mem::transmute(dwdata), ::std::mem::transmute(dwoperation)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add(&self, prop: *mut CProperty) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(prop)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Get(&self, propid: u32, property: *mut CProperty) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(propid), ::std::mem::transmute(property)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersist<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fpersist: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), fpersist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersist2<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, propid: u32, fpersist: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(propid), fpersist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFirst(&self, property: *mut CProperty) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(property)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNext(&self, property: *mut CProperty) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(property)).ok()
    }
    pub unsafe fn GetPropCount(&self, cprop: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(cprop)).ok()
    }
    pub unsafe fn SaveHeader(&self, lpvdata: *mut ::std::ffi::c_void, dwhdrsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpvdata), ::std::mem::transmute(dwhdrsize)).ok()
    }
    pub unsafe fn SaveData(&self, lpvheader: *mut ::std::ffi::c_void, dwhdrsize: u32, lpvdata: *mut ::std::ffi::c_void, dwbufsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpvheader), ::std::mem::transmute(dwhdrsize), ::std::mem::transmute(lpvdata), ::std::mem::transmute(dwbufsize)).ok()
    }
    pub unsafe fn GetHeaderSize(&self, dwhdrsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwhdrsize)).ok()
    }
    pub unsafe fn GetDataSize(&self, lpvheader: *mut ::std::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpvheader), ::std::mem::transmute(dwhdrsize), ::std::mem::transmute(dwdatasize)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveDataToStream<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, lpvheader: *mut ::std::ffi::c_void, dwhdrsize: u32, pstream: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpvheader), ::std::mem::transmute(dwhdrsize), pstream.into_param().abi()).ok()
    }
    pub unsafe fn LoadFromMem(&self, lpvdata: *mut ::std::ffi::c_void, dwbufsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpvdata), ::std::mem::transmute(dwbufsize)).ok()
    }
    pub unsafe fn SaveToMem(&self, lpvdata: *mut ::std::ffi::c_void, dwbufsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpvdata), ::std::mem::transmute(dwbufsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IITPropList {
    type Vtable = IITPropList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(524303281, 39319, 4560, [168, 80, 0, 170, 0, 108, 125, 1]);
}
impl ::std::convert::From<IITPropList> for ::windows::runtime::IUnknown {
    fn from(value: IITPropList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IITPropList> for ::windows::runtime::IUnknown {
    fn from(value: &IITPropList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IITPropList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IITPropList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::std::convert::From<IITPropList> for super::super::System::Ole::IPersistStreamInit {
    fn from(value: IITPropList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::std::convert::From<&IITPropList> for super::super::System::Ole::IPersistStreamInit {
    fn from(value: &IITPropList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::IPersistStreamInit> for IITPropList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::IPersistStreamInit> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::IPersistStreamInit>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::IPersistStreamInit> for &IITPropList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::IPersistStreamInit> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::IPersistStreamInit>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IITPropList> for super::super::System::Com::IPersist {
    fn from(value: IITPropList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IITPropList> for super::super::System::Com::IPersist {
    fn from(value: &IITPropList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IPersist> for IITPropList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IPersist> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Com::IPersist>::into(self))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IPersist> for &IITPropList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IPersist> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Com::IPersist>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IITPropList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclassid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbsize: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propid: u32, lpszwstring: super::super::Foundation::PWSTR, dwoperation: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propid: u32, lpvdata: *mut ::std::ffi::c_void, cbdata: u32, dwoperation: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propid: u32, dwdata: u32, dwoperation: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prop: *mut CProperty) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propid: u32, property: *mut CProperty) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fpersist: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propid: u32, fpersist: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, property: *mut CProperty) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, property: *mut CProperty) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cprop: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpvdata: *mut ::std::ffi::c_void, dwhdrsize: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpvheader: *mut ::std::ffi::c_void, dwhdrsize: u32, lpvdata: *mut ::std::ffi::c_void, dwbufsize: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwhdrsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpvheader: *mut ::std::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpvheader: *mut ::std::ffi::c_void, dwhdrsize: u32, pstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpvdata: *mut ::std::ffi::c_void, dwbufsize: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpvdata: *mut ::std::ffi::c_void, dwbufsize: u32) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IITQuery(pub u8);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IITResultSet(::windows::runtime::IUnknown);
impl IITResultSet {
    pub unsafe fn SetColumnPriority(&self, lcolumnindex: i32, columnpriority: PRIORITY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(lcolumnindex), ::std::mem::transmute(columnpriority)).ok()
    }
    pub unsafe fn SetColumnHeap(&self, lcolumnindex: i32, lpvheap: *mut ::std::ffi::c_void, pfncolheapfree: ::std::option::Option<PFNCOLHEAPFREE>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(lcolumnindex), ::std::mem::transmute(lpvheap), ::std::mem::transmute(pfncolheapfree)).ok()
    }
    pub unsafe fn SetKeyProp(&self, propid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(propid)).ok()
    }
    pub unsafe fn Add(&self, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(propid), ::std::mem::transmute(dwdefaultdata), ::std::mem::transmute(priority)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add2<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, propid: u32, lpszwdefault: Param1, priority: PRIORITY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(propid), lpszwdefault.into_param().abi(), ::std::mem::transmute(priority)).ok()
    }
    pub unsafe fn Add3(&self, propid: u32, lpvdefaultdata: *mut ::std::ffi::c_void, cbdata: u32, priority: PRIORITY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(propid), ::std::mem::transmute(lpvdefaultdata), ::std::mem::transmute(cbdata), ::std::mem::transmute(priority)).ok()
    }
    pub unsafe fn Add4(&self, lpvhdr: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpvhdr)).ok()
    }
    pub unsafe fn Append(&self, lpvhdr: *mut ::std::ffi::c_void, lpvdata: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpvhdr), ::std::mem::transmute(lpvdata)).ok()
    }
    pub unsafe fn Set(&self, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut ::std::ffi::c_void, cbdata: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(lrowindex), ::std::mem::transmute(lcolumnindex), ::std::mem::transmute(lpvdata), ::std::mem::transmute(cbdata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Set2<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lrowindex: i32, lcolumnindex: i32, lpwstr: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(lrowindex), ::std::mem::transmute(lcolumnindex), lpwstr.into_param().abi()).ok()
    }
    pub unsafe fn Set3(&self, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(lrowindex), ::std::mem::transmute(lcolumnindex), ::std::mem::transmute(dwdata)).ok()
    }
    pub unsafe fn Set4(&self, lrowindex: i32, lpvhdr: *mut ::std::ffi::c_void, lpvdata: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(lrowindex), ::std::mem::transmute(lpvhdr), ::std::mem::transmute(lpvdata)).ok()
    }
    pub unsafe fn Copy<'a, Param0: ::windows::runtime::IntoParam<'a, IITResultSet>>(&self, prscopy: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), prscopy.into_param().abi()).ok()
    }
    pub unsafe fn AppendRows<'a, Param0: ::windows::runtime::IntoParam<'a, IITResultSet>>(&self, pressrc: Param0, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pressrc.into_param().abi(), ::std::mem::transmute(lrowsrcfirst), ::std::mem::transmute(csrcrows), ::std::mem::transmute(lrowfirstdest)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Get(&self, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(lrowindex), ::std::mem::transmute(lcolumnindex), ::std::mem::transmute(prop)).ok()
    }
    pub unsafe fn GetKeyProp(&self, keypropid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(keypropid)).ok()
    }
    pub unsafe fn GetColumnPriority(&self, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(lcolumnindex), ::std::mem::transmute(columnpriority)).ok()
    }
    pub unsafe fn GetRowCount(&self, lnumberofrows: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(lnumberofrows)).ok()
    }
    pub unsafe fn GetColumnCount(&self, lnumberofcolumns: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(lnumberofcolumns)).ok()
    }
    pub unsafe fn GetColumn(&self, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut ::std::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(lcolumnindex), ::std::mem::transmute(propid), ::std::mem::transmute(dwtype), ::std::mem::transmute(lpvdefaultvalue), ::std::mem::transmute(cbsize), ::std::mem::transmute(columnpriority)).ok()
    }
    pub unsafe fn GetColumn2(&self, lcolumnindex: i32, propid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(lcolumnindex), ::std::mem::transmute(propid)).ok()
    }
    pub unsafe fn GetColumnFromPropID(&self, propid: u32, lcolumnindex: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(propid), ::std::mem::transmute(lcolumnindex)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ClearRows(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Free(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsCompleted(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Pause<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fpause: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), fpause.into_param().abi()).ok()
    }
    pub unsafe fn GetRowStatus(&self, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(lrowfirst), ::std::mem::transmute(crows), ::std::mem::transmute(lprowstatus)).ok()
    }
    pub unsafe fn GetColumnStatus(&self, lpcolstatus: *mut COLUMNSTATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpcolstatus)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IITResultSet {
    type Vtable = IITResultSet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1001987393, 39307, 4560, [168, 80, 0, 170, 0, 108, 125, 1]);
}
impl ::std::convert::From<IITResultSet> for ::windows::runtime::IUnknown {
    fn from(value: IITResultSet) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IITResultSet> for ::windows::runtime::IUnknown {
    fn from(value: &IITResultSet) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IITResultSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IITResultSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IITResultSet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcolumnindex: i32, columnpriority: PRIORITY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcolumnindex: i32, lpvheap: *mut ::std::ffi::c_void, pfncolheapfree: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propid: u32, lpszwdefault: super::super::Foundation::PWSTR, priority: PRIORITY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propid: u32, lpvdefaultdata: *mut ::std::ffi::c_void, cbdata: u32, priority: PRIORITY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpvhdr: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpvhdr: *mut ::std::ffi::c_void, lpvdata: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut ::std::ffi::c_void, cbdata: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lrowindex: i32, lcolumnindex: i32, lpwstr: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lrowindex: i32, lpvhdr: *mut ::std::ffi::c_void, lpvdata: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prscopy: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pressrc: ::windows::runtime::RawPtr, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, keypropid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lnumberofrows: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lnumberofcolumns: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut ::std::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcolumnindex: i32, propid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propid: u32, lcolumnindex: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fpause: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpcolstatus: *mut COLUMNSTATUS) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IITStopWordList(pub u8);
pub const IITWBC_BREAK_ACCEPT_WILDCARDS: u32 = 1u32;
pub const IITWBC_BREAK_AND_STEM: u32 = 2u32;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IITWordWheel(::windows::runtime::IUnknown);
impl IITWordWheel {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, IITDatabase>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lpitdb: Param0, lpszmoniker: Param1, dwflags: WORD_WHEEL_OPEN_FLAGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), lpitdb.into_param().abi(), lpszmoniker.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwcodepageid), ::std::mem::transmute(plcid)).ok()
    }
    pub unsafe fn GetSorterInstance(&self, pdwobjinstance: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwobjinstance)).ok()
    }
    pub unsafe fn Count(&self, pcentries: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcentries)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Lookup<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, lpcvprefix: *const ::std::ffi::c_void, fexactmatch: Param1, plentry: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpcvprefix), fexactmatch.into_param().abi(), ::std::mem::transmute(plentry)).ok()
    }
    pub unsafe fn Lookup2<'a, Param1: ::windows::runtime::IntoParam<'a, IITResultSet>>(&self, lentry: i32, lpitresult: Param1, centries: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(lentry), lpitresult.into_param().abi(), ::std::mem::transmute(centries)).ok()
    }
    pub unsafe fn Lookup3(&self, lentry: i32, lpvkeybuf: *mut ::std::ffi::c_void, cbkeybuf: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(lentry), ::std::mem::transmute(lpvkeybuf), ::std::mem::transmute(cbkeybuf)).ok()
    }
    pub unsafe fn SetGroup(&self, piitgroup: *mut IITGroup) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(piitgroup)).ok()
    }
    pub unsafe fn GetGroup(&self, ppiitgroup: *mut *mut IITGroup) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppiitgroup)).ok()
    }
    pub unsafe fn GetDataCount(&self, lentry: i32, pdwcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(lentry), ::std::mem::transmute(pdwcount)).ok()
    }
    pub unsafe fn GetData<'a, Param1: ::windows::runtime::IntoParam<'a, IITResultSet>>(&self, lentry: i32, lpitresult: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(lentry), lpitresult.into_param().abi()).ok()
    }
    pub unsafe fn GetDataColumns<'a, Param0: ::windows::runtime::IntoParam<'a, IITResultSet>>(&self, prs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), prs.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IITWordWheel {
    type Vtable = IITWordWheel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2409682340, 57055, 4560, [154, 97, 0, 192, 79, 182, 139, 247]);
}
impl ::std::convert::From<IITWordWheel> for ::windows::runtime::IUnknown {
    fn from(value: IITWordWheel) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IITWordWheel> for ::windows::runtime::IUnknown {
    fn from(value: &IITWordWheel) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IITWordWheel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IITWordWheel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IITWordWheel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpitdb: ::windows::runtime::RawPtr, lpszmoniker: super::super::Foundation::PWSTR, dwflags: WORD_WHEEL_OPEN_FLAGS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwobjinstance: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcentries: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpcvprefix: *const ::std::ffi::c_void, fexactmatch: super::super::Foundation::BOOL, plentry: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lentry: i32, lpitresult: ::windows::runtime::RawPtr, centries: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lentry: i32, lpvkeybuf: *mut ::std::ffi::c_void, cbkeybuf: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piitgroup: *mut IITGroup) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiitgroup: *mut *mut IITGroup) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lentry: i32, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lentry: i32, lpitresult: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prs: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IStemSink(::windows::runtime::IUnknown);
impl IStemSink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PutAltWord<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwcinbuf: Param0, cwc: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwcinbuf.into_param().abi(), ::std::mem::transmute(cwc)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PutWord<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwcinbuf: Param0, cwc: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pwcinbuf.into_param().abi(), ::std::mem::transmute(cwc)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IStemSink {
    type Vtable = IStemSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4269261616, 32578, 4558, [190, 87, 0, 170, 0, 81, 254, 32]);
}
impl ::std::convert::From<IStemSink> for ::windows::runtime::IUnknown {
    fn from(value: IStemSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStemSink> for ::windows::runtime::IUnknown {
    fn from(value: &IStemSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStemSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IStemSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStemSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcinbuf: super::super::Foundation::PWSTR, cwc: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IStemmerConfig(::windows::runtime::IUnknown);
impl IStemmerConfig {
    pub unsafe fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcodepageid), ::std::mem::transmute(lcid)).ok()
    }
    pub unsafe fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwcodepageid), ::std::mem::transmute(plcid)).ok()
    }
    pub unsafe fn SetControlInfo(&self, grfstemflags: u32, dwreserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfstemflags), ::std::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn GetControlInfo(&self, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pgrfstemflags), ::std::mem::transmute(pdwreserved)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadExternalStemmerData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0, dwextdatatype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pstream.into_param().abi(), ::std::mem::transmute(dwextdatatype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IStemmerConfig {
    type Vtable = IStemmerConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2409682343, 57055, 4560, [154, 97, 0, 192, 79, 182, 139, 247]);
}
impl ::std::convert::From<IStemmerConfig> for ::windows::runtime::IUnknown {
    fn from(value: IStemmerConfig) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStemmerConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IStemmerConfig) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStemmerConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IStemmerConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStemmerConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcodepageid: u32, lcid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfstemflags: u32, dwreserved: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, dwextdatatype: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
pub const ITWW_CBKEY_MAX: u32 = 1024u32;
pub const ITWW_OPEN_NOCONNECT: u32 = 1u32;
pub const IT_EXCLUSIVE: i32 = 1i32;
pub const IT_HIDDEN: i32 = 2i32;
pub const IT_INCLUSIVE: i32 = 0i32;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWordBreakerConfig(::windows::runtime::IUnknown);
impl IWordBreakerConfig {
    pub unsafe fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcodepageid), ::std::mem::transmute(lcid)).ok()
    }
    pub unsafe fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwcodepageid), ::std::mem::transmute(plcid)).ok()
    }
    pub unsafe fn SetBreakWordType(&self, dwbreakwordtype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwbreakwordtype)).ok()
    }
    pub unsafe fn GetBreakWordType(&self, pdwbreakwordtype: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwbreakwordtype)).ok()
    }
    pub unsafe fn SetControlInfo(&self, grfbreakflags: u32, dwreserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfbreakflags), ::std::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn GetControlInfo(&self, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pgrfbreakflags), ::std::mem::transmute(pdwreserved)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadExternalBreakerData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0, dwextdatatype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pstream.into_param().abi(), ::std::mem::transmute(dwextdatatype)).ok()
    }
    #[cfg(feature = "Win32_System_Search")]
    pub unsafe fn SetWordStemmer<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Search::IStemmer>>(&self, rclsid: *const ::windows::runtime::GUID, pstemmer: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(rclsid), pstemmer.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Search")]
    pub unsafe fn GetWordStemmer(&self) -> ::windows::runtime::Result<super::super::System::Search::IStemmer> {
        let mut result__: <super::super::System::Search::IStemmer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Search::IStemmer>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWordBreakerConfig {
    type Vtable = IWordBreakerConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2409682342, 57055, 4560, [154, 97, 0, 192, 79, 182, 139, 247]);
}
impl ::std::convert::From<IWordBreakerConfig> for ::windows::runtime::IUnknown {
    fn from(value: IWordBreakerConfig) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWordBreakerConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IWordBreakerConfig) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWordBreakerConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWordBreakerConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordBreakerConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcodepageid: u32, lcid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwbreakwordtype: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwbreakwordtype: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfbreakflags: u32, dwreserved: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, dwextdatatype: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Search")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, pstemmer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))] usize,
    #[cfg(feature = "Win32_System_Search")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstemmer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))] usize,
);
pub const MAX_COLUMNS: u32 = 256u32;
pub type PFNCOLHEAPFREE = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void) -> i32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PRIORITY(pub i32);
pub const PRIORITY_LOW: PRIORITY = PRIORITY(0i32);
pub const PRIORITY_NORMAL: PRIORITY = PRIORITY(1i32);
pub const PRIORITY_HIGH: PRIORITY = PRIORITY(2i32);
impl ::std::convert::From<i32> for PRIORITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRIORITY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PROP_ADD: u32 = 0u32;
pub const PROP_DELETE: u32 = 1u32;
pub const PROP_UPDATE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ROWSTATUS {
    pub lRowFirst: i32,
    pub cRows: i32,
    pub cProperties: i32,
    pub cRowsTotal: i32,
}
impl ROWSTATUS {}
impl ::std::default::Default for ROWSTATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ROWSTATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ROWSTATUS").field("lRowFirst", &self.lRowFirst).field("cRows", &self.cRows).field("cProperties", &self.cProperties).field("cRowsTotal", &self.cRowsTotal).finish()
    }
}
impl ::std::cmp::PartialEq for ROWSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.lRowFirst == other.lRowFirst && self.cRows == other.cRows && self.cProperties == other.cProperties && self.cRowsTotal == other.cRowsTotal
    }
}
impl ::std::cmp::Eq for ROWSTATUS {}
unsafe impl ::windows::runtime::Abi for ROWSTATUS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STDPROP_DISPLAYKEY: u32 = 101u32;
pub const STDPROP_INDEX_BREAK: u32 = 204u32;
pub const STDPROP_INDEX_DTYPE: u32 = 202u32;
pub const STDPROP_INDEX_LENGTH: u32 = 203u32;
pub const STDPROP_INDEX_TERM: u32 = 210u32;
pub const STDPROP_INDEX_TERM_RAW_LENGTH: u32 = 211u32;
pub const STDPROP_INDEX_TEXT: u32 = 200u32;
pub const STDPROP_INDEX_VFLD: u32 = 201u32;
pub const STDPROP_KEY: u32 = 4u32;
pub const STDPROP_SORTKEY: u32 = 100u32;
pub const STDPROP_SORTORDINAL: u32 = 102u32;
pub const STDPROP_TITLE: u32 = 2u32;
pub const STDPROP_UID: u32 = 1u32;
pub const STDPROP_USERDATA: u32 = 3u32;
pub const STDPROP_USERPROP_BASE: u32 = 65536u32;
pub const STDPROP_USERPROP_MAX: u32 = 2147483647u32;
pub const TYPE_POINTER: u32 = 1u32;
pub const TYPE_STRING: u32 = 2u32;
pub const TYPE_VALUE: u32 = 0u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WORD_WHEEL_OPEN_FLAGS(pub u32);
pub const ITWW_OPEN_CONNECT: WORD_WHEEL_OPEN_FLAGS = WORD_WHEEL_OPEN_FLAGS(0u32);
impl ::std::convert::From<u32> for WORD_WHEEL_OPEN_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WORD_WHEEL_OPEN_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WORD_WHEEL_OPEN_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WORD_WHEEL_OPEN_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
