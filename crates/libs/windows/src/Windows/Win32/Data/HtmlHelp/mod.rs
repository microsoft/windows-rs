#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CLSID_IITCmdInt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa2_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITDatabase: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66673452_8c23_11d0_a84e_00aa006c7d01);
pub const CLSID_IITDatabaseLocal: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa9_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITGroupUpdate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa4_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITIndexBuild: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5aa_dedf_11d0_9a61_00c04fb68bf7);
pub const CLSID_IITPropList: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daae_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITResultSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa7_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITSvMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa3_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITWWFilterBuild: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5ab_dedf_11d0_9a61_00c04fb68bf7);
pub const CLSID_IITWordWheel: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd73725c2_8c12_11d0_a84e_00aa006c7d01);
pub const CLSID_IITWordWheelLocal: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa8_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_IITWordWheelUpdate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daa5_d393_11d0_9a56_00c04fb68bf7);
pub const CLSID_ITEngStemmer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5a8_dedf_11d0_9a61_00c04fb68bf7);
pub const CLSID_ITStdBreaker: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4662daaf_d393_11d0_9a56_00c04fb68bf7);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub struct COLUMNSTATUS {
    pub cPropCount: i32,
    pub cPropsLoaded: i32,
}
impl ::core::marker::Copy for COLUMNSTATUS {}
impl ::core::clone::Clone for COLUMNSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COLUMNSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLUMNSTATUS").field("cPropCount", &self.cPropCount).field("cPropsLoaded", &self.cPropsLoaded).finish()
    }
}
unsafe impl ::windows::core::Abi for COLUMNSTATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COLUMNSTATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COLUMNSTATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for COLUMNSTATUS {}
impl ::core::default::Default for COLUMNSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CProperty {
    pub dwPropID: u32,
    pub cbData: u32,
    pub dwType: u32,
    pub Anonymous: CProperty_0,
    pub fPersist: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CProperty {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CProperty {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CProperty {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CProperty>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CProperty {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CProperty {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CProperty_0 {
    pub lpszwData: ::windows::core::PWSTR,
    pub lpvData: *mut ::core::ffi::c_void,
    pub dwValue: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CProperty_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CProperty_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CProperty_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CProperty_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CProperty_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CProperty_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CProperty_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_ALL_WILD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479467i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_ALREADYINIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479421i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_ALREADYOPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479533i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_ASSERT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479546i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_BADBREAKER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479469i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_BADFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479549i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_BADFILTERSIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479528i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_BADFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479548i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_BADINDEXFLAGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479456i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_BADPARAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479535i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_BADRANGEOP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479459i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_BADVALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479468i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_BADVERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479550i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_CANTFINDDLL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479538i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_DISKFULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479496i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_DUPLICATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479551i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_EXPECTEDTERM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479465i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_FILECLOSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479503i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_FILECREATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479504i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_FILEDELETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479499i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_FILEINVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479498i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_FILENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479497i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_FILEREAD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479502i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_FILESEEK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479501i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_FILEWRITE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479500i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_GETLASTERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479536i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_GROUPIDTOOBIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479542i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_INTERRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479545i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_INVALIDSTATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479534i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_MISSINGPROP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479424i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_MISSLPAREN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479464i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_MISSQUOTE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479462i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_MISSRPAREN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479463i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_NAMETOOLONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479520i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_NOHANDLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479537i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_NOKEYPROP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479417i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_NOMERGEDDATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479540i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_NOPERMISSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479547i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_NOSTEMMER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479454i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_NOTEXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479552i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_NOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479539i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_NOTINIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479420i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_NOTOPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479533i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_NOTSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479544i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_NULLQUERY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479461i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_OUTOFRANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479543i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_PROPLISTEMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479422i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_PROPLISTNOTEMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479423i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_RESULTSETEMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479419i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_STOPWORD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479460i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_TOODEEP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479466i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_TOOMANYCOLUMNS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479418i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_TOOMANYDUPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479471i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_TOOMANYOBJECTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479527i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_TOOMANYTITLES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479541i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_TOOMANYTOPICS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479472i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_TREETOOBIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479470i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_UNKNOWN_TRANSPORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479530i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_UNMATCHEDTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479458i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_UNSUPPORTED_TRANSPORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479529i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_WILD_IN_DTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479455i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const E_WORDTOOLONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147479457i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_BACK: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_CONTRACT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_CUSTOMIZE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_EXPAND: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_FORWARD: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_HIGHLIGHT: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_HOME: i32 = 11i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_JUMP1: i32 = 17i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_JUMP2: i32 = 18i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_LAST_ENUM: i32 = 23i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_NOTES: i32 = 22i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_OPTIONS: i32 = 13i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_PRINT: i32 = 14i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_REFRESH: i32 = 10i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_STOP: i32 = 9i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_SYNC: i32 = 12i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_TAB_CONTENTS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_TAB_FAVORITES: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_TAB_HISTORY: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_TAB_INDEX: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_TAB_SEARCH: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_TOC_NEXT: i32 = 20i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_TOC_PREV: i32 = 21i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHACT_ZOOM: i32 = 19i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct HHNTRACK {
    pub hdr: super::super::UI::Controls::NMHDR,
    pub pszCurUrl: ::windows::core::PCSTR,
    pub idAction: i32,
    pub phhWinType: *mut HH_WINTYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for HHNTRACK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for HHNTRACK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::fmt::Debug for HHNTRACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HHNTRACK").field("hdr", &self.hdr).field("pszCurUrl", &self.pszCurUrl).field("idAction", &self.idAction).field("phhWinType", &self.phhWinType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
unsafe impl ::windows::core::Abi for HHNTRACK {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::PartialEq for HHNTRACK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HHNTRACK>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::Eq for HHNTRACK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for HHNTRACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct HHN_NOTIFY {
    pub hdr: super::super::UI::Controls::NMHDR,
    pub pszUrl: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for HHN_NOTIFY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for HHN_NOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::fmt::Debug for HHN_NOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HHN_NOTIFY").field("hdr", &self.hdr).field("pszUrl", &self.pszUrl).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
unsafe impl ::windows::core::Abi for HHN_NOTIFY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::PartialEq for HHN_NOTIFY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HHN_NOTIFY>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::Eq for HHN_NOTIFY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for HHN_NOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_BACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_BROWSE_BCK: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_BROWSE_FWD: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_CONTENTS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_EXPAND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_FAVORITES: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_FORWARD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_HISTORY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_HOME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_INDEX: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_JUMP1: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_JUMP2: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_NOTES: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_OPTIONS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_PRINT: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_REFRESH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_SEARCH: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_STOP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_SYNC: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_TOC_NEXT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_TOC_PREV: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_BUTTON_ZOOM: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_NAVTAB_BOTTOM: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_NAVTAB_LEFT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_NAVTAB_TOP: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_NAVTYPE_AUTHOR: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_NAVTYPE_CUSTOM_FIRST: i32 = 11i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_NAVTYPE_FAVORITES: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_NAVTYPE_HISTORY: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_NAVTYPE_INDEX: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_NAVTYPE_SEARCH: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_NAVTYPE_TOC: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PARAM_CUR_TAB: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PARAM_EXPANSION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PARAM_EXSTYLES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PARAM_HISTORY_COUNT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PARAM_INFOTYPES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PARAM_NAV_WIDTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PARAM_PROPERTIES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PARAM_RECT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PARAM_SHOWSTATE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PARAM_STYLES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PARAM_TABORDER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PARAM_TABPOS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PARAM_TB_FLAGS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_AUTO_SYNC: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_CHANGE_TITLE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_MENU: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_NAV_ONLY_WIN: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_NODEF_EXSTYLES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_NODEF_STYLES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_NOTB_TEXT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_NOTITLEBAR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_NO_TOOLBAR: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_ONTOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_POST_QUIT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_ADVSEARCH: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_AUTOHIDESHOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_CUSTOM1: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_CUSTOM2: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_CUSTOM3: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_CUSTOM4: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_CUSTOM5: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_CUSTOM6: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_CUSTOM7: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_CUSTOM8: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_CUSTOM9: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_FAVORITES: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_HISTORY: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TAB_SEARCH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TRACKING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_TRI_PANE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_PROP_USER_POS: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HHWIN_TB_MARGIN: u32 = 268435456u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for HH_AKLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HH_AKLINK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_AKLINK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_AKLINK").field("cbStruct", &self.cbStruct).field("fReserved", &self.fReserved).field("pszKeywords", &self.pszKeywords).field("pszUrl", &self.pszUrl).field("pszMsgText", &self.pszMsgText).field("pszMsgTitle", &self.pszMsgTitle).field("pszWindow", &self.pszWindow).field("fIndexOnFail", &self.fIndexOnFail).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HH_AKLINK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_AKLINK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HH_AKLINK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_AKLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_AKLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_ALINK_LOOKUP: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_CLOSE_ALL: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_DISPLAY_INDEX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_DISPLAY_SEARCH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_DISPLAY_TEXT_POPUP: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_DISPLAY_TOC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_DISPLAY_TOPIC: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub struct HH_ENUM_CAT {
    pub cbStruct: i32,
    pub pszCatName: ::windows::core::PCSTR,
    pub pszCatDescription: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for HH_ENUM_CAT {}
impl ::core::clone::Clone for HH_ENUM_CAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HH_ENUM_CAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_ENUM_CAT").field("cbStruct", &self.cbStruct).field("pszCatName", &self.pszCatName).field("pszCatDescription", &self.pszCatDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for HH_ENUM_CAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HH_ENUM_CAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HH_ENUM_CAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for HH_ENUM_CAT {}
impl ::core::default::Default for HH_ENUM_CAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_ENUM_CATEGORY: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_ENUM_CATEGORY_IT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_ENUM_INFO_TYPE: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub struct HH_ENUM_IT {
    pub cbStruct: i32,
    pub iType: i32,
    pub pszCatName: ::windows::core::PCSTR,
    pub pszITName: ::windows::core::PCSTR,
    pub pszITDescription: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for HH_ENUM_IT {}
impl ::core::clone::Clone for HH_ENUM_IT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HH_ENUM_IT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_ENUM_IT").field("cbStruct", &self.cbStruct).field("iType", &self.iType).field("pszCatName", &self.pszCatName).field("pszITName", &self.pszITName).field("pszITDescription", &self.pszITDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for HH_ENUM_IT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HH_ENUM_IT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HH_ENUM_IT>()) == 0 }
    }
}
impl ::core::cmp::Eq for HH_ENUM_IT {}
impl ::core::default::Default for HH_ENUM_IT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_FTS_DEFAULT_PROXIMITY: i32 = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for HH_FTS_QUERY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HH_FTS_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_FTS_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_FTS_QUERY").field("cbStruct", &self.cbStruct).field("fUniCodeStrings", &self.fUniCodeStrings).field("pszSearchQuery", &self.pszSearchQuery).field("iProximity", &self.iProximity).field("fStemmedSearch", &self.fStemmedSearch).field("fTitleOnly", &self.fTitleOnly).field("fExecute", &self.fExecute).field("pszWindow", &self.pszWindow).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HH_FTS_QUERY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_FTS_QUERY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HH_FTS_QUERY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_FTS_QUERY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_FTS_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_GET_LAST_ERROR: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_GET_WIN_HANDLE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_GET_WIN_TYPE: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct HH_GLOBAL_PROPERTY {
    pub id: HH_GPROPID,
    pub var: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for HH_GLOBAL_PROPERTY {
    fn clone(&self) -> Self {
        Self { id: self.id, var: self.var.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for HH_GLOBAL_PROPERTY {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for HH_GLOBAL_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.var == other.var
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for HH_GLOBAL_PROPERTY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for HH_GLOBAL_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HH_GPROPID(pub i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_GPROPID_SINGLETHREAD: HH_GPROPID = HH_GPROPID(1i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_GPROPID_TOOLBAR_MARGIN: HH_GPROPID = HH_GPROPID(2i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_GPROPID_UI_LANGUAGE: HH_GPROPID = HH_GPROPID(3i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_GPROPID_CURRENT_SUBSET: HH_GPROPID = HH_GPROPID(4i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_GPROPID_CONTENT_LANGUAGE: HH_GPROPID = HH_GPROPID(5i32);
impl ::core::marker::Copy for HH_GPROPID {}
impl ::core::clone::Clone for HH_GPROPID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HH_GPROPID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HH_GPROPID {
    type Abi = Self;
}
impl ::core::fmt::Debug for HH_GPROPID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HH_GPROPID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_HELP_CONTEXT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_HELP_FINDER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_INITIALIZE: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_KEYWORD_LOOKUP: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_MAX_TABS: u32 = 19u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for HH_POPUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HH_POPUP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_POPUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_POPUP").field("cbStruct", &self.cbStruct).field("hinst", &self.hinst).field("idString", &self.idString).field("pszText", &self.pszText).field("pt", &self.pt).field("clrForeground", &self.clrForeground).field("clrBackground", &self.clrBackground).field("rcMargins", &self.rcMargins).field("pszFont", &self.pszFont).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HH_POPUP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_POPUP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HH_POPUP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_POPUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_POPUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_PRETRANSLATEMESSAGE: u32 = 253u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_RESERVED1: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_RESERVED2: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_RESERVED3: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_RESET_IT_FILTER: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_SAFE_DISPLAY_TOPIC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_SET_EXCLUSIVE_FILTER: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_SET_GLOBAL_PROPERTY: u32 = 252u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_SET_INCLUSIVE_FILTER: u32 = 24u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub struct HH_SET_INFOTYPE {
    pub cbStruct: i32,
    pub pszCatName: ::windows::core::PCSTR,
    pub pszInfoTypeName: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for HH_SET_INFOTYPE {}
impl ::core::clone::Clone for HH_SET_INFOTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HH_SET_INFOTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_SET_INFOTYPE").field("cbStruct", &self.cbStruct).field("pszCatName", &self.pszCatName).field("pszInfoTypeName", &self.pszInfoTypeName).finish()
    }
}
unsafe impl ::windows::core::Abi for HH_SET_INFOTYPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HH_SET_INFOTYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HH_SET_INFOTYPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for HH_SET_INFOTYPE {}
impl ::core::default::Default for HH_SET_INFOTYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_SET_INFO_TYPE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_SET_QUERYSERVICE: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_SET_WIN_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_SYNC: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_TAB_AUTHOR: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_TAB_CONTENTS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_TAB_CUSTOM_FIRST: i32 = 11i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_TAB_CUSTOM_LAST: i32 = 19i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_TAB_FAVORITES: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_TAB_HISTORY: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_TAB_INDEX: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_TAB_SEARCH: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_TP_HELP_CONTEXTMENU: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_TP_HELP_WM_HELP: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const HH_UNINITIALIZE: u32 = 29u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for HH_WINTYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HH_WINTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HH_WINTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HH_WINTYPE")
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
unsafe impl ::windows::core::Abi for HH_WINTYPE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HH_WINTYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HH_WINTYPE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HH_WINTYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HH_WINTYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_BACK: u32 = 204u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_BROWSE_BACK: u32 = 212u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_BROWSE_FWD: u32 = 211u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_CONTENTS: u32 = 213u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_CONTRACT: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_CUSTOMIZE: u32 = 221u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_EXPAND: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_FAVORITES: u32 = 217u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_FORWARD: u32 = 209u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_HISTORY: u32 = 216u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_HOME: u32 = 205u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_INDEX: u32 = 214u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_JUMP1: u32 = 218u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_JUMP2: u32 = 219u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_NOTES: u32 = 210u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_OPTIONS: u32 = 208u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_PRINT: u32 = 207u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_REFRESH: u32 = 203u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_SEARCH: u32 = 215u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_STOP: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_SYNC: u32 = 206u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_TOC_NEXT: u32 = 223u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_TOC_PREV: u32 = 224u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IDTB_ZOOM: u32 = 222u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
#[repr(transparent)]
pub struct IITDatabase(::windows::core::IUnknown);
impl IITDatabase {
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lpszhost: Param0, lpszmoniker: Param1, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::core::mem::transmute_copy(self), lpszhost.into_param().abi(), lpszmoniker.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn CreateObject(&self, rclsid: *const ::windows::core::GUID, pdwobjinstance: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(pdwobjinstance)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetObject(&self, dwobjinstance: u32, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwobjinstance), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectPersistence<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, lpwszobject: Param0, dwobjinstance: u32, ppvpersistence: *mut *mut ::core::ffi::c_void, fstream: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectPersistence)(::core::mem::transmute_copy(self), lpwszobject.into_param().abi(), ::core::mem::transmute(dwobjinstance), ::core::mem::transmute(ppvpersistence), fstream.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IITDatabase> for ::windows::core::IUnknown {
    fn from(value: IITDatabase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IITDatabase> for ::windows::core::IUnknown {
    fn from(value: &IITDatabase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IITDatabase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IITDatabase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IITDatabase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IITDatabase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IITDatabase {}
impl ::core::fmt::Debug for IITDatabase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IITDatabase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IITDatabase {
    type Vtable = IITDatabase_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5a2_dedf_11d0_9a61_00c04fb68bf7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IITDatabase_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszhost: ::windows::core::PCWSTR, lpszmoniker: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, pdwobjinstance: *mut u32) -> ::windows::core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwobjinstance: u32, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetObjectPersistence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpwszobject: ::windows::core::PCWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut ::core::ffi::c_void, fstream: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetObjectPersistence: usize,
}
#[repr(C)]
pub struct IITGroup(pub u8);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IITPropList(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IITPropList {
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetClassID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.IsDirty)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Load)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Save)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSizeMax)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.InitNew)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Set<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, propid: u32, lpszwstring: Param1, dwoperation: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Set)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), lpszwstring.into_param().abi(), ::core::mem::transmute(dwoperation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Set2(&self, propid: u32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32, dwoperation: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Set2)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), ::core::mem::transmute(lpvdata), ::core::mem::transmute(cbdata), ::core::mem::transmute(dwoperation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Set3(&self, propid: u32, dwdata: u32, dwoperation: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Set3)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), ::core::mem::transmute(dwdata), ::core::mem::transmute(dwoperation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add(&self, prop: *mut CProperty) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), ::core::mem::transmute(prop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Get(&self, propid: u32, property: *mut CProperty) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Get)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), ::core::mem::transmute(property)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersist<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fpersist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPersist)(::core::mem::transmute_copy(self), fpersist.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPersist2<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, propid: u32, fpersist: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPersist2)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), fpersist.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFirst(&self, property: *mut CProperty) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFirst)(::core::mem::transmute_copy(self), ::core::mem::transmute(property)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNext(&self, property: *mut CProperty) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNext)(::core::mem::transmute_copy(self), ::core::mem::transmute(property)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetPropCount(&self, cprop: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(cprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn SaveHeader(&self, lpvdata: *mut ::core::ffi::c_void, dwhdrsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveHeader)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvdata), ::core::mem::transmute(dwhdrsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn SaveData(&self, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveData)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvheader), ::core::mem::transmute(dwhdrsize), ::core::mem::transmute(lpvdata), ::core::mem::transmute(dwbufsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetHeaderSize(&self, dwhdrsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHeaderSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwhdrsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetDataSize(&self, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvheader), ::core::mem::transmute(dwhdrsize), ::core::mem::transmute(dwdatasize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveDataToStream<'a, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, pstream: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveDataToStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvheader), ::core::mem::transmute(dwhdrsize), pstream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn LoadFromMem(&self, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadFromMem)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvdata), ::core::mem::transmute(dwbufsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn SaveToMem(&self, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveToMem)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvdata), ::core::mem::transmute(dwbufsize)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IITPropList> for ::windows::core::IUnknown {
    fn from(value: IITPropList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IITPropList> for ::windows::core::IUnknown {
    fn from(value: &IITPropList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IITPropList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IITPropList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IITPropList> for super::super::System::Com::IPersist {
    fn from(value: IITPropList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IITPropList> for super::super::System::Com::IPersist {
    fn from(value: &IITPropList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersist> for IITPropList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersist> for &'a IITPropList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IITPropList> for super::super::System::Com::IPersistStreamInit {
    fn from(value: IITPropList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IITPropList> for super::super::System::Com::IPersistStreamInit {
    fn from(value: &IITPropList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersistStreamInit> for IITPropList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersistStreamInit> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersistStreamInit> for &'a IITPropList {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersistStreamInit> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IITPropList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IITPropList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IITPropList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IITPropList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IITPropList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IITPropList {
    type Vtable = IITPropList_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f403bb1_9997_11d0_a850_00aa006c7d01);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IITPropList_Vtbl {
    pub base: super::super::System::Com::IPersistStreamInit_Vtbl,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: u32, lpszwstring: ::windows::core::PCWSTR, dwoperation: u32) -> ::windows::core::HRESULT,
    pub Set2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: u32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32, dwoperation: u32) -> ::windows::core::HRESULT,
    pub Set3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: u32, dwdata: u32, dwoperation: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prop: *mut CProperty) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Add: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: u32, property: *mut CProperty) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Get: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPersist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fpersist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPersist: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPersist2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: u32, fpersist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPersist2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: *mut CProperty) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFirst: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: *mut CProperty) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNext: usize,
    pub GetPropCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cprop: *mut i32) -> ::windows::core::HRESULT,
    pub SaveHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwhdrsize: u32) -> ::windows::core::HRESULT,
    pub SaveData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT,
    pub GetHeaderSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwhdrsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetDataSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveDataToStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveDataToStream: usize,
    pub LoadFromMem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT,
    pub SaveToMem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows::core::HRESULT,
}
#[repr(C)]
pub struct IITQuery(pub u8);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
#[repr(transparent)]
pub struct IITResultSet(::windows::core::IUnknown);
impl IITResultSet {
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn SetColumnPriority(&self, lcolumnindex: i32, columnpriority: PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColumnPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(columnpriority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn SetColumnHeap(&self, lcolumnindex: i32, lpvheap: *mut ::core::ffi::c_void, pfncolheapfree: PFNCOLHEAPFREE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColumnHeap)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(lpvheap), ::core::mem::transmute(pfncolheapfree)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn SetKeyProp(&self, propid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetKeyProp)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Add(&self, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), ::core::mem::transmute(dwdefaultdata), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Add2<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, propid: u32, lpszwdefault: Param1, priority: PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add2)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), lpszwdefault.into_param().abi(), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Add3(&self, propid: u32, lpvdefaultdata: *mut ::core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add3)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), ::core::mem::transmute(lpvdefaultdata), ::core::mem::transmute(cbdata), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Add4(&self, lpvhdr: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvhdr)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Append(&self, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvhdr), ::core::mem::transmute(lpvdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Set(&self, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Set)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrowindex), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(lpvdata), ::core::mem::transmute(cbdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Set2<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lrowindex: i32, lcolumnindex: i32, lpwstr: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Set2)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrowindex), ::core::mem::transmute(lcolumnindex), lpwstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Set3(&self, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Set3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrowindex), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(dwdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Set4(&self, lrowindex: i32, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Set4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrowindex), ::core::mem::transmute(lpvhdr), ::core::mem::transmute(lpvdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Copy<'a, Param0: ::windows::core::IntoParam<'a, IITResultSet>>(&self, prscopy: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Copy)(::core::mem::transmute_copy(self), prscopy.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn AppendRows<'a, Param0: ::windows::core::IntoParam<'a, IITResultSet>>(&self, pressrc: Param0, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AppendRows)(::core::mem::transmute_copy(self), pressrc.into_param().abi(), ::core::mem::transmute(lrowsrcfirst), ::core::mem::transmute(csrcrows), ::core::mem::transmute(lrowfirstdest)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Get(&self, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Get)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrowindex), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(prop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetKeyProp(&self, keypropid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetKeyProp)(::core::mem::transmute_copy(self), ::core::mem::transmute(keypropid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetColumnPriority(&self, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColumnPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(columnpriority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetRowCount(&self, lnumberofrows: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRowCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnumberofrows)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetColumnCount(&self, lnumberofcolumns: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColumnCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnumberofcolumns)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetColumn(&self, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut ::core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColumn)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(propid), ::core::mem::transmute(dwtype), ::core::mem::transmute(lpvdefaultvalue), ::core::mem::transmute(cbsize), ::core::mem::transmute(columnpriority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetColumn2(&self, lcolumnindex: i32, propid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColumn2)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcolumnindex), ::core::mem::transmute(propid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetColumnFromPropID(&self, propid: u32, lcolumnindex: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColumnFromPropID)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), ::core::mem::transmute(lcolumnindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn ClearRows(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearRows)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Free(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Free)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn IsCompleted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsCompleted)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Pause<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fpause: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::core::mem::transmute_copy(self), fpause.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetRowStatus(&self, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRowStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(lrowfirst), ::core::mem::transmute(crows), ::core::mem::transmute(lprowstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetColumnStatus(&self, lpcolstatus: *mut COLUMNSTATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColumnStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpcolstatus)).ok()
    }
}
impl ::core::convert::From<IITResultSet> for ::windows::core::IUnknown {
    fn from(value: IITResultSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IITResultSet> for ::windows::core::IUnknown {
    fn from(value: &IITResultSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IITResultSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IITResultSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IITResultSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IITResultSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IITResultSet {}
impl ::core::fmt::Debug for IITResultSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IITResultSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IITResultSet {
    type Vtable = IITResultSet_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bb91d41_998b_11d0_a850_00aa006c7d01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IITResultSet_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetColumnPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcolumnindex: i32, columnpriority: PRIORITY) -> ::windows::core::HRESULT,
    pub SetColumnHeap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcolumnindex: i32, lpvheap: *mut ::core::ffi::c_void, pfncolheapfree: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetKeyProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: u32) -> ::windows::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> ::windows::core::HRESULT,
    pub Add2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: u32, lpszwdefault: ::windows::core::PCWSTR, priority: PRIORITY) -> ::windows::core::HRESULT,
    pub Add3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: u32, lpvdefaultdata: *mut ::core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> ::windows::core::HRESULT,
    pub Add4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpvhdr: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows::core::HRESULT,
    pub Set2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpwstr: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Set3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> ::windows::core::HRESULT,
    pub Set4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lrowindex: i32, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prscopy: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AppendRows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pressrc: ::windows::core::RawPtr, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Get: usize,
    pub GetKeyProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keypropid: *mut u32) -> ::windows::core::HRESULT,
    pub GetColumnPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> ::windows::core::HRESULT,
    pub GetRowCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnumberofrows: *mut i32) -> ::windows::core::HRESULT,
    pub GetColumnCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnumberofcolumns: *mut i32) -> ::windows::core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut ::core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> ::windows::core::HRESULT,
    pub GetColumn2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcolumnindex: i32, propid: *mut u32) -> ::windows::core::HRESULT,
    pub GetColumnFromPropID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: u32, lcolumnindex: *mut i32) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearRows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Free: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fpause: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Pause: usize,
    pub GetRowStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> ::windows::core::HRESULT,
    pub GetColumnStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcolstatus: *mut COLUMNSTATUS) -> ::windows::core::HRESULT,
}
#[repr(C)]
pub struct IITStopWordList(pub u8);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IITWBC_BREAK_ACCEPT_WILDCARDS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IITWBC_BREAK_AND_STEM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
#[repr(transparent)]
pub struct IITWordWheel(::windows::core::IUnknown);
impl IITWordWheel {
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, IITDatabase>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, lpitdb: Param0, lpszmoniker: Param1, dwflags: WORD_WHEEL_OPEN_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::core::mem::transmute_copy(self), lpitdb.into_param().abi(), lpszmoniker.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLocaleInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcodepageid), ::core::mem::transmute(plcid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetSorterInstance(&self, pdwobjinstance: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSorterInstance)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwobjinstance)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Count(&self, pcentries: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcentries)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Lookup<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, lpcvprefix: *const ::core::ffi::c_void, fexactmatch: Param1, plentry: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Lookup)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpcvprefix), fexactmatch.into_param().abi(), ::core::mem::transmute(plentry)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Lookup2<'a, Param1: ::windows::core::IntoParam<'a, IITResultSet>>(&self, lentry: i32, lpitresult: Param1, centries: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Lookup2)(::core::mem::transmute_copy(self), ::core::mem::transmute(lentry), lpitresult.into_param().abi(), ::core::mem::transmute(centries)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn Lookup3(&self, lentry: i32, lpvkeybuf: *mut ::core::ffi::c_void, cbkeybuf: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Lookup3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lentry), ::core::mem::transmute(lpvkeybuf), ::core::mem::transmute(cbkeybuf)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn SetGroup(&self, piitgroup: *mut IITGroup) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(piitgroup)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetGroup(&self, ppiitgroup: *mut *mut IITGroup) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppiitgroup)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetDataCount(&self, lentry: i32, pdwcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(lentry), ::core::mem::transmute(pdwcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetData<'a, Param1: ::windows::core::IntoParam<'a, IITResultSet>>(&self, lentry: i32, lpitresult: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetData)(::core::mem::transmute_copy(self), ::core::mem::transmute(lentry), lpitresult.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetDataColumns<'a, Param0: ::windows::core::IntoParam<'a, IITResultSet>>(&self, prs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataColumns)(::core::mem::transmute_copy(self), prs.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IITWordWheel> for ::windows::core::IUnknown {
    fn from(value: IITWordWheel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IITWordWheel> for ::windows::core::IUnknown {
    fn from(value: &IITWordWheel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IITWordWheel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IITWordWheel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IITWordWheel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IITWordWheel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IITWordWheel {}
impl ::core::fmt::Debug for IITWordWheel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IITWordWheel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IITWordWheel {
    type Vtable = IITWordWheel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5a4_dedf_11d0_9a61_00c04fb68bf7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IITWordWheel_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpitdb: ::windows::core::RawPtr, lpszmoniker: ::windows::core::PCWSTR, dwflags: WORD_WHEEL_OPEN_FLAGS) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLocaleInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT,
    pub GetSorterInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwobjinstance: *mut u32) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcentries: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Lookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcvprefix: *const ::core::ffi::c_void, fexactmatch: super::super::Foundation::BOOL, plentry: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Lookup: usize,
    pub Lookup2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lentry: i32, lpitresult: ::windows::core::RawPtr, centries: i32) -> ::windows::core::HRESULT,
    pub Lookup3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lentry: i32, lpvkeybuf: *mut ::core::ffi::c_void, cbkeybuf: u32) -> ::windows::core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piitgroup: *mut IITGroup) -> ::windows::core::HRESULT,
    pub GetGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiitgroup: *mut *mut IITGroup) -> ::windows::core::HRESULT,
    pub GetDataCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lentry: i32, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lentry: i32, lpitresult: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDataColumns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prs: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
#[repr(transparent)]
pub struct IStemSink(::windows::core::IUnknown);
impl IStemSink {
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn PutAltWord<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwcinbuf: Param0, cwc: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PutAltWord)(::core::mem::transmute_copy(self), pwcinbuf.into_param().abi(), ::core::mem::transmute(cwc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn PutWord<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwcinbuf: Param0, cwc: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PutWord)(::core::mem::transmute_copy(self), pwcinbuf.into_param().abi(), ::core::mem::transmute(cwc)).ok()
    }
}
impl ::core::convert::From<IStemSink> for ::windows::core::IUnknown {
    fn from(value: IStemSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStemSink> for ::windows::core::IUnknown {
    fn from(value: &IStemSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStemSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStemSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStemSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStemSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStemSink {}
impl ::core::fmt::Debug for IStemSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStemSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStemSink {
    type Vtable = IStemSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe77c330_7f42_11ce_be57_00aa0051fe20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStemSink_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub PutAltWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcinbuf: ::windows::core::PCWSTR, cwc: u32) -> ::windows::core::HRESULT,
    pub PutWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcinbuf: ::windows::core::PCWSTR, cwc: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
#[repr(transparent)]
pub struct IStemmerConfig(::windows::core::IUnknown);
impl IStemmerConfig {
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLocaleInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcodepageid), ::core::mem::transmute(lcid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLocaleInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcodepageid), ::core::mem::transmute(plcid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn SetControlInfo(&self, grfstemflags: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetControlInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfstemflags), ::core::mem::transmute(dwreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetControlInfo(&self, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetControlInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pgrfstemflags), ::core::mem::transmute(pdwreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadExternalStemmerData<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0, dwextdatatype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadExternalStemmerData)(::core::mem::transmute_copy(self), pstream.into_param().abi(), ::core::mem::transmute(dwextdatatype)).ok()
    }
}
impl ::core::convert::From<IStemmerConfig> for ::windows::core::IUnknown {
    fn from(value: IStemmerConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStemmerConfig> for ::windows::core::IUnknown {
    fn from(value: &IStemmerConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStemmerConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStemmerConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStemmerConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStemmerConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStemmerConfig {}
impl ::core::fmt::Debug for IStemmerConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStemmerConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStemmerConfig {
    type Vtable = IStemmerConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5a7_dedf_11d0_9a61_00c04fb68bf7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStemmerConfig_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetLocaleInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> ::windows::core::HRESULT,
    pub GetLocaleInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT,
    pub SetControlInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfstemflags: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub GetControlInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadExternalStemmerData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, dwextdatatype: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadExternalStemmerData: usize,
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const ITWW_CBKEY_MAX: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const ITWW_OPEN_NOCONNECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IT_EXCLUSIVE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IT_HIDDEN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const IT_INCLUSIVE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
#[repr(transparent)]
pub struct IWordBreakerConfig(::windows::core::IUnknown);
impl IWordBreakerConfig {
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn SetLocaleInfo(&self, dwcodepageid: u32, lcid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLocaleInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcodepageid), ::core::mem::transmute(lcid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetLocaleInfo(&self, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLocaleInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcodepageid), ::core::mem::transmute(plcid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn SetBreakWordType(&self, dwbreakwordtype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBreakWordType)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwbreakwordtype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetBreakWordType(&self, pdwbreakwordtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBreakWordType)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbreakwordtype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn SetControlInfo(&self, grfbreakflags: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetControlInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbreakflags), ::core::mem::transmute(dwreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
    pub unsafe fn GetControlInfo(&self, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetControlInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pgrfbreakflags), ::core::mem::transmute(pdwreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadExternalBreakerData<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0, dwextdatatype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadExternalBreakerData)(::core::mem::transmute_copy(self), pstream.into_param().abi(), ::core::mem::transmute(dwextdatatype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_System_Search\"`*"]
    #[cfg(feature = "Win32_System_Search")]
    pub unsafe fn SetWordStemmer<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Search::IStemmer>>(&self, rclsid: *const ::windows::core::GUID, pstemmer: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWordStemmer)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), pstemmer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`, `\"Win32_System_Search\"`*"]
    #[cfg(feature = "Win32_System_Search")]
    pub unsafe fn GetWordStemmer(&self) -> ::windows::core::Result<super::super::System::Search::IStemmer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetWordStemmer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Search::IStemmer>(result__)
    }
}
impl ::core::convert::From<IWordBreakerConfig> for ::windows::core::IUnknown {
    fn from(value: IWordBreakerConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWordBreakerConfig> for ::windows::core::IUnknown {
    fn from(value: &IWordBreakerConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWordBreakerConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWordBreakerConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWordBreakerConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWordBreakerConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWordBreakerConfig {}
impl ::core::fmt::Debug for IWordBreakerConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWordBreakerConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWordBreakerConfig {
    type Vtable = IWordBreakerConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fa0d5a6_dedf_11d0_9a61_00c04fb68bf7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordBreakerConfig_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetLocaleInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> ::windows::core::HRESULT,
    pub GetLocaleInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows::core::HRESULT,
    pub SetBreakWordType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbreakwordtype: u32) -> ::windows::core::HRESULT,
    pub GetBreakWordType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbreakwordtype: *mut u32) -> ::windows::core::HRESULT,
    pub SetControlInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbreakflags: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub GetControlInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadExternalBreakerData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, dwextdatatype: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadExternalBreakerData: usize,
    #[cfg(feature = "Win32_System_Search")]
    pub SetWordStemmer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, pstemmer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))]
    SetWordStemmer: usize,
    #[cfg(feature = "Win32_System_Search")]
    pub GetWordStemmer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstemmer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search"))]
    GetWordStemmer: usize,
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const MAX_COLUMNS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub type PFNCOLHEAPFREE = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const PRIORITY_LOW: PRIORITY = PRIORITY(0i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const PRIORITY_NORMAL: PRIORITY = PRIORITY(1i32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const PRIORITY_HIGH: PRIORITY = PRIORITY(2i32);
impl ::core::marker::Copy for PRIORITY {}
impl ::core::clone::Clone for PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRIORITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const PROP_ADD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const PROP_DELETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const PROP_UPDATE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub struct ROWSTATUS {
    pub lRowFirst: i32,
    pub cRows: i32,
    pub cProperties: i32,
    pub cRowsTotal: i32,
}
impl ::core::marker::Copy for ROWSTATUS {}
impl ::core::clone::Clone for ROWSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ROWSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ROWSTATUS").field("lRowFirst", &self.lRowFirst).field("cRows", &self.cRows).field("cProperties", &self.cProperties).field("cRowsTotal", &self.cRowsTotal).finish()
    }
}
unsafe impl ::windows::core::Abi for ROWSTATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ROWSTATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ROWSTATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for ROWSTATUS {}
impl ::core::default::Default for ROWSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_DISPLAYKEY: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_INDEX_BREAK: u32 = 204u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_INDEX_DTYPE: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_INDEX_LENGTH: u32 = 203u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_INDEX_TERM: u32 = 210u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_INDEX_TERM_RAW_LENGTH: u32 = 211u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_INDEX_TEXT: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_INDEX_VFLD: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_KEY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_SORTKEY: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_SORTORDINAL: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_TITLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_UID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_USERDATA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_USERPROP_BASE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const STDPROP_USERPROP_MAX: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const SZ_WWDEST_GLOBAL: &'static str = "GLOBAL";
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const SZ_WWDEST_KEY: &'static str = "KEY";
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const SZ_WWDEST_OCC: &'static str = "OCC";
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const TYPE_POINTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const TYPE_STRING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const TYPE_VALUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WORD_WHEEL_OPEN_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Data_HtmlHelp\"`*"]
pub const ITWW_OPEN_CONNECT: WORD_WHEEL_OPEN_FLAGS = WORD_WHEEL_OPEN_FLAGS(0u32);
impl ::core::marker::Copy for WORD_WHEEL_OPEN_FLAGS {}
impl ::core::clone::Clone for WORD_WHEEL_OPEN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WORD_WHEEL_OPEN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WORD_WHEEL_OPEN_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WORD_WHEEL_OPEN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WORD_WHEEL_OPEN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WORD_WHEEL_OPEN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WORD_WHEEL_OPEN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
