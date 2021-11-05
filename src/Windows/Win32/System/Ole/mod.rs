#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_System_Ole_Automation")]
pub mod Automation;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ACTIVATEFLAGS(pub i32);
pub const ACTIVATE_WINDOWLESS: ACTIVATEFLAGS = ACTIVATEFLAGS(1i32);
impl ::std::convert::From<i32> for ACTIVATEFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ACTIVATEFLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Ole`*"]
pub struct AspectInfo {
    pub cb: u32,
    pub dwFlags: u32,
}
impl AspectInfo {}
impl ::std::default::Default for AspectInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for AspectInfo {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AspectInfo").field("cb", &self.cb).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::std::cmp::PartialEq for AspectInfo {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.dwFlags == other.dwFlags
    }
}
impl ::std::cmp::Eq for AspectInfo {}
unsafe impl ::windows::runtime::Abi for AspectInfo {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AspectInfoFlag(pub i32);
pub const DVASPECTINFOFLAG_CANOPTIMIZE: AspectInfoFlag = AspectInfoFlag(1i32);
impl ::std::convert::From<i32> for AspectInfoFlag {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AspectInfoFlag {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BINDSPEED(pub i32);
pub const BINDSPEED_INDEFINITE: BINDSPEED = BINDSPEED(1i32);
pub const BINDSPEED_MODERATE: BINDSPEED = BINDSPEED(2i32);
pub const BINDSPEED_IMMEDIATE: BINDSPEED = BINDSPEED(3i32);
impl ::std::convert::From<i32> for BINDSPEED {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BINDSPEED {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const BZ_DISABLECANCELBUTTON: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const BZ_DISABLERETRYBUTTON: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const BZ_DISABLESWITCHTOBUTTON: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const BZ_NOTRESPONDINGDIALOG: i32 = 8i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Ole`*"]
pub struct CADWORD {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl CADWORD {}
impl ::std::default::Default for CADWORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CADWORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CADWORD").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CADWORD {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CADWORD {}
unsafe impl ::windows::runtime::Abi for CADWORD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
pub struct CALPOLESTR {
    pub cElems: u32,
    pub pElems: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CALPOLESTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CALPOLESTR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CALPOLESTR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CALPOLESTR").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CALPOLESTR {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CALPOLESTR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CALPOLESTR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Ole`*"]
pub struct CAUUID {
    pub cElems: u32,
    pub pElems: *mut ::windows::runtime::GUID,
}
impl CAUUID {}
impl ::std::default::Default for CAUUID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CAUUID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAUUID").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CAUUID {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CAUUID {}
unsafe impl ::windows::runtime::Abi for CAUUID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CF_CONVERTONLY: i32 = 256i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CF_DISABLEACTIVATEAS: i32 = 64i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CF_DISABLEDISPLAYASICON: i32 = 32i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CF_HIDECHANGEICON: i32 = 128i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CF_SELECTACTIVATEAS: i32 = 16i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CF_SELECTCONVERTTO: i32 = 8i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CF_SETACTIVATEDEFAULT: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CF_SETCONVERTDEFAULT: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CF_SHOWHELPBUTTON: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CIF_SELECTCURRENT: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CIF_SELECTDEFAULT: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CIF_SELECTFROMFILE: i32 = 8i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CIF_SHOWHELP: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CIF_USEICONEXE: i32 = 16i32;
pub const CLSID_CColorPropPage: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(199447041, 36753, 4558, [157, 227, 0, 170, 0, 75, 184, 81]);
pub const CLSID_CFontPropPage: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(199447040, 36753, 4558, [157, 227, 0, 170, 0, 75, 184, 81]);
pub const CLSID_CPicturePropPage: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(199447042, 36753, 4558, [157, 227, 0, 170, 0, 75, 184, 81]);
pub const CLSID_ConvertVBX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4220454946, 356, 4123, [132, 237, 8, 0, 43, 46, 199, 19]);
pub const CLSID_PersistPropset: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4220454945, 356, 4123, [132, 237, 8, 0, 43, 46, 199, 19]);
pub const CLSID_StdFont: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(199447043, 36753, 4558, [157, 227, 0, 170, 0, 75, 184, 81]);
pub const CLSID_StdPicture: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(199447044, 36753, 4558, [157, 227, 0, 170, 0, 75, 184, 81]);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CONNECT_E_ADVISELIMIT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220991i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CONNECT_E_CANNOTCONNECT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220990i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CONNECT_E_FIRST: i32 = -2147220992i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CONNECT_E_LAST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220977i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CONNECT_E_NOCONNECTION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220992i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CONNECT_E_OVERRIDDEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220989i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CONNECT_S_FIRST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(262656i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CONNECT_S_LAST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(262671i32 as _);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_UI_WindowsAndMessaging`*"]
pub struct CONTROLINFO {
    pub cb: u32,
    pub hAccel: super::super::UI::WindowsAndMessaging::HACCEL,
    pub cAccel: u16,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl CONTROLINFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::std::default::Default for CONTROLINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::std::fmt::Debug for CONTROLINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONTROLINFO").field("cb", &self.cb).field("hAccel", &self.hAccel).field("cAccel", &self.cAccel).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::std::cmp::PartialEq for CONTROLINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.hAccel == other.hAccel && self.cAccel == other.cAccel && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::std::cmp::Eq for CONTROLINFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::runtime::Abi for CONTROLINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CSF_EXPLORER: i32 = 8i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CSF_ONLYGETSOURCE: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CSF_SHOWHELP: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CSF_VALIDSOURCE: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const CTL_E_ILLEGALFUNCTIONCALL: i32 = -2146828283i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CTRLINFO(pub i32);
pub const CTRLINFO_EATS_RETURN: CTRLINFO = CTRLINFO(1i32);
pub const CTRLINFO_EATS_ESCAPE: CTRLINFO = CTRLINFO(2i32);
impl ::std::convert::From<i32> for CTRLINFO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CTRLINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[inline]
pub unsafe fn CreateOleAdviseHolder() -> ::windows::runtime::Result<IOleAdviseHolder> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateOleAdviseHolder(ppoaholder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IOleAdviseHolder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateOleAdviseHolder(&mut result__).from_abi::<IOleAdviseHolder>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DD_DEFDRAGDELAY: u32 = 200u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DD_DEFDRAGMINDIST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DD_DEFSCROLLDELAY: u32 = 50u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DD_DEFSCROLLINSET: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DD_DEFSCROLLINTERVAL: u32 = 50u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISCARDCACHE(pub i32);
pub const DISCARDCACHE_SAVEIFDIRTY: DISCARDCACHE = DISCARDCACHE(0i32);
pub const DISCARDCACHE_NOSAVE: DISCARDCACHE = DISCARDCACHE(1i32);
impl ::std::convert::From<i32> for DISCARDCACHE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISCARDCACHE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_ABOUTBOX: i32 = -552i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_ACCELERATOR: i32 = -543i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_ADDITEM: i32 = -553i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_APPEARANCE: i32 = -716i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_AUTOCLIP: i32 = -715i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_BACKCOLOR: i32 = -701i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_CHARSET: i32 = -727i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_CODEPAGE: i32 = -725i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_DISPLAYASDEFAULT: i32 = -713i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_DISPLAYNAME: i32 = -702i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_FONT: i32 = -703i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_FORECOLOR: i32 = -704i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_LOCALEID: i32 = -705i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_MESSAGEREFLECT: i32 = -706i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_PALETTE: i32 = -726i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_RIGHTTOLEFT: i32 = -732i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_SCALEUNITS: i32 = -707i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_SHOWGRABHANDLES: i32 = -711i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_SHOWHATCHING: i32 = -712i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_SUPPORTSMNEMONICS: i32 = -714i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_TEXTALIGN: i32 = -708i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_TOPTOBOTTOM: i32 = -733i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_TRANSFERPRIORITY: i32 = -728i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_UIDEAD: i32 = -710i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AMBIENT_USERMODE: i32 = -709i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_APPEARANCE: i32 = -520i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_AUTOSIZE: i32 = -500i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_BACKCOLOR: i32 = -501i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_BACKSTYLE: i32 = -502i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_BORDERCOLOR: i32 = -503i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_BORDERSTYLE: i32 = -504i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_BORDERVISIBLE: i32 = -519i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_BORDERWIDTH: i32 = -505i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_CAPTION: i32 = -518i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_CLEAR: i32 = -554i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_CLICK: i32 = -600i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_CLICK_VALUE: i32 = -610i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_COLUMN: i32 = -529i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_DBLCLICK: i32 = -601i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_DISPLAYSTYLE: i32 = -540i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_DOCLICK: i32 = -551i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_DRAWMODE: i32 = -507i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_DRAWSTYLE: i32 = -508i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_DRAWWIDTH: i32 = -509i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_Delete: i32 = -801i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_ENABLED: i32 = -514i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_ENTERKEYBEHAVIOR: i32 = -544i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_ERROREVENT: i32 = -608i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_FILLCOLOR: i32 = -510i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_FILLSTYLE: i32 = -511i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_FONT: i32 = -512i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_FONT_BOLD: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_FONT_CHANGED: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_FONT_CHARSET: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_FONT_ITALIC: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_FONT_NAME: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_FONT_SIZE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_FONT_STRIKE: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_FONT_UNDER: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_FONT_WEIGHT: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_FORECOLOR: i32 = -513i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_GROUPNAME: i32 = -541i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_HWND: i32 = -515i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_IMEMODE: i32 = -542i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_KEYDOWN: i32 = -602i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_KEYPRESS: i32 = -603i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_KEYUP: i32 = -604i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_LIST: i32 = -528i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_LISTCOUNT: i32 = -531i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_LISTINDEX: i32 = -526i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_MAXLENGTH: i32 = -533i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_MOUSEDOWN: i32 = -605i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_MOUSEICON: i32 = -522i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_MOUSEMOVE: i32 = -606i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_MOUSEPOINTER: i32 = -521i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_MOUSEUP: i32 = -607i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_MULTILINE: i32 = -537i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_MULTISELECT: i32 = -532i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_NUMBEROFCOLUMNS: i32 = -539i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_NUMBEROFROWS: i32 = -538i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_Name: i32 = -800i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_Object: i32 = -802i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_PASSWORDCHAR: i32 = -534i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_PICTURE: i32 = -523i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_PICT_HANDLE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_PICT_HEIGHT: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_PICT_HPAL: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_PICT_RENDER: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_PICT_TYPE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_PICT_WIDTH: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_Parent: i32 = -803i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_READYSTATE: i32 = -525i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_READYSTATECHANGE: i32 = -609i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_REFRESH: i32 = -550i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_REMOVEITEM: i32 = -555i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_RIGHTTOLEFT: i32 = -611i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_SCROLLBARS: i32 = -535i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_SELECTED: i32 = -527i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_SELLENGTH: i32 = -548i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_SELSTART: i32 = -547i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_SELTEXT: i32 = -546i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_TABKEYBEHAVIOR: i32 = -545i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_TABSTOP: i32 = -516i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_TEXT: i32 = -517i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_TOPTOBOTTOM: i32 = -612i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_VALID: i32 = -524i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DISPID_WORDWRAP: i32 = -536i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DOCMISC(pub i32);
pub const DOCMISC_CANCREATEMULTIPLEVIEWS: DOCMISC = DOCMISC(1i32);
pub const DOCMISC_SUPPORTCOMPLEXRECTANGLES: DOCMISC = DOCMISC(2i32);
pub const DOCMISC_CANTOPENEDIT: DOCMISC = DOCMISC(4i32);
pub const DOCMISC_NOFILESUPPORT: DOCMISC = DOCMISC(8i32);
impl ::std::convert::From<i32> for DOCMISC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DOCMISC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DROPEFFECT_COPY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DROPEFFECT_LINK: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DROPEFFECT_MOVE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DROPEFFECT_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const DROPEFFECT_SCROLL: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DVASPECT2(pub i32);
pub const DVASPECT_OPAQUE: DVASPECT2 = DVASPECT2(16i32);
pub const DVASPECT_TRANSPARENT: DVASPECT2 = DVASPECT2(32i32);
impl ::std::convert::From<i32> for DVASPECT2 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DVASPECT2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DoDragDrop<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>, Param1: ::windows::runtime::IntoParam<'a, IDropSource>>(pdataobj: Param0, pdropsource: Param1, dwokeffects: u32, pdweffect: *mut u32) -> ::windows::runtime::HRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DoDragDrop(pdataobj: ::windows::runtime::RawPtr, pdropsource: ::windows::runtime::RawPtr, dwokeffects: u32, pdweffect: *mut u32) -> ::windows::runtime::HRESULT;
        }
        ::std::mem::transmute(DoDragDrop(pdataobj.into_param().abi(), pdropsource.into_param().abi(), ::std::mem::transmute(dwokeffects), ::std::mem::transmute(pdweffect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const ELF_DISABLECANCELLINK: i32 = 16i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const ELF_DISABLECHANGESOURCE: i32 = 8i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const ELF_DISABLEOPENSOURCE: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const ELF_DISABLEUPDATENOW: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const ELF_SHOWHELP: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const EMBDHLP_CREATENOW: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const EMBDHLP_DELAYCREATE: i32 = 65536i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const EMBDHLP_INPROC_HANDLER: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const EMBDHLP_INPROC_SERVER: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ENUM_CONTROLS_WHICH_FLAGS(pub u32);
pub const GCW_WCH_SIBLING: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(1u32);
pub const GC_WCH_CONTAINER: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(2u32);
pub const GC_WCH_CONTAINED: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(3u32);
pub const GC_WCH_ALL: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(4u32);
pub const GC_WCH_FREVERSEDIR: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(134217728u32);
pub const GC_WCH_FONLYAFTER: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(268435456u32);
pub const GC_WCH_FONLYBEFORE: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(536870912u32);
pub const GC_WCH_FSELECTED: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(1073741824u32);
impl ::std::convert::From<u32> for ENUM_CONTROLS_WHICH_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ENUM_CONTROLS_WHICH_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for ENUM_CONTROLS_WHICH_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ENUM_CONTROLS_WHICH_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ENUM_CONTROLS_WHICH_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ENUM_CONTROLS_WHICH_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ENUM_CONTROLS_WHICH_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
pub struct ExtentInfo {
    pub cb: u32,
    pub dwExtentMode: u32,
    pub sizelProposed: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ExtentInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ExtentInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ExtentInfo {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ExtentInfo").field("cb", &self.cb).field("dwExtentMode", &self.dwExtentMode).field("sizelProposed", &self.sizelProposed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ExtentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.dwExtentMode == other.dwExtentMode && self.sizelProposed == other.sizelProposed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ExtentInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ExtentInfo {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtentMode(pub i32);
pub const DVEXTENT_CONTENT: ExtentMode = ExtentMode(0i32);
pub const DVEXTENT_INTEGRAL: ExtentMode = ExtentMode(1i32);
impl ::std::convert::From<i32> for ExtentMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ExtentMode {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct FONTDESC {
    pub cbSizeofstruct: u32,
    pub lpstrName: super::super::Foundation::PWSTR,
    pub cySize: super::Com::CY,
    pub sWeight: i16,
    pub sCharset: i16,
    pub fItalic: super::super::Foundation::BOOL,
    pub fUnderline: super::super::Foundation::BOOL,
    pub fStrikethrough: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl FONTDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for FONTDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for FONTDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for FONTDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for FONTDESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const GC_WCH_SIBLING: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GUIDKIND(pub i32);
pub const GUIDKIND_DEFAULT_SOURCE_DISP_IID: GUIDKIND = GUIDKIND(1i32);
impl ::std::convert::From<i32> for GUIDKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GUIDKIND {
    type Abi = Self;
}
pub const GUID_CHECKVALUEEXCLUSIVE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536076, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_COLOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536065, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_FONTBOLD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536079, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_FONTITALIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536080, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_FONTNAME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536077, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_FONTSIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536078, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_FONTSTRIKETHROUGH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536082, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_FONTUNDERSCORE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536081, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_HANDLE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536083, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_HIMETRIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536064, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_OPTIONVALUEEXCLUSIVE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536075, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_TRISTATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536074, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_XPOS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536070, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_XPOSPIXEL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536066, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_XSIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536072, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_XSIZEPIXEL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536068, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_YPOS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536071, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_YPOSPIXEL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536067, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_YSIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536073, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
pub const GUID_YSIZEPIXEL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716536069, 48655, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HITRESULT(pub i32);
pub const HITRESULT_OUTSIDE: HITRESULT = HITRESULT(0i32);
pub const HITRESULT_TRANSPARENT: HITRESULT = HITRESULT(1i32);
pub const HITRESULT_CLOSE: HITRESULT = HITRESULT(2i32);
pub const HITRESULT_HIT: HITRESULT = HITRESULT(3i32);
impl ::std::convert::From<i32> for HITRESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HITRESULT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserFree(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HRGN_UserFree(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN);
        }
        ::std::mem::transmute(HRGN_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserFree64(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HRGN_UserFree64(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN);
        }
        ::std::mem::transmute(HRGN_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HRGN_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8;
        }
        ::std::mem::transmute(HRGN_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HRGN_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8;
        }
        ::std::mem::transmute(HRGN_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserSize(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HRGN_UserSize(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32;
        }
        ::std::mem::transmute(HRGN_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HRGN_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32;
        }
        ::std::mem::transmute(HRGN_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HRGN_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8;
        }
        ::std::mem::transmute(HRGN_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HRGN_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8;
        }
        ::std::mem::transmute(HRGN_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAdviseSinkEx(pub ::windows::runtime::IUnknown);
impl IAdviseSinkEx {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn OnDataChange(&self, pformatetc: *const super::Com::FORMATETC, pstgmed: *const super::Com::STGMEDIUM) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetc), ::std::mem::transmute(pstgmed)))
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwaspect), ::std::mem::transmute(lindex)))
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn OnRename<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IMoniker>>(&self, pmk: Param0) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnSave(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnClose(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnViewStatusChange(&self, dwviewstatus: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwviewstatus)))
    }
}
unsafe impl ::windows::runtime::Interface for IAdviseSinkEx {
    type Vtable = IAdviseSinkEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(988955280, 3222, 4558, [160, 207, 0, 170, 0, 96, 10, 184]);
}
impl ::std::convert::From<IAdviseSinkEx> for ::windows::runtime::IUnknown {
    fn from(value: IAdviseSinkEx) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAdviseSinkEx> for ::windows::runtime::IUnknown {
    fn from(value: &IAdviseSinkEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAdviseSinkEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAdviseSinkEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IAdviseSinkEx> for super::Com::IAdviseSink {
    fn from(value: IAdviseSinkEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IAdviseSinkEx> for super::Com::IAdviseSink {
    fn from(value: &IAdviseSinkEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IAdviseSink> for IAdviseSinkEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IAdviseSink> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IAdviseSink> for &IAdviseSinkEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IAdviseSink> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSinkEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const super::Com::FORMATETC, pstgmed: *const ::std::mem::ManuallyDrop<super::Com::STGMEDIUM>),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, lindex: i32),
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr),
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwviewstatus: u32),
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IClassFactory2(pub ::windows::runtime::IUnknown);
impl IClassFactory2 {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, T: ::windows::runtime::Interface>(&self, punkouter: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), punkouter.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn LockServer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), flock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetLicInfo(&self, plicinfo: *mut LICINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(plicinfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn RequestLicKey(&self, dwreserved: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwreserved), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn CreateInstanceLic<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, punkouter: Param0, punkreserved: Param1, riid: *const ::windows::runtime::GUID, bstrkey: Param3, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), punkouter.into_param().abi(), punkreserved.into_param().abi(), ::std::mem::transmute(riid), bstrkey.into_param().abi(), ::std::mem::transmute(ppvobj)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IClassFactory2 {
    type Vtable = IClassFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443343, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::std::convert::From<IClassFactory2> for ::windows::runtime::IUnknown {
    fn from(value: IClassFactory2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IClassFactory2> for ::windows::runtime::IUnknown {
    fn from(value: &IClassFactory2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IClassFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IClassFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IClassFactory2> for super::Com::IClassFactory {
    fn from(value: IClassFactory2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IClassFactory2> for super::Com::IClassFactory {
    fn from(value: &IClassFactory2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IClassFactory> for IClassFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IClassFactory> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IClassFactory> for &IClassFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IClassFactory> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flock: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plicinfo: *mut LICINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwreserved: u32, pbstrkey: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr, punkreserved: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, bstrkey: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IContinue(pub ::windows::runtime::IUnknown);
impl IContinue {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn FContinue(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContinue {
    type Vtable = IContinue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(298, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IContinue> for ::windows::runtime::IUnknown {
    fn from(value: IContinue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IContinue> for ::windows::runtime::IUnknown {
    fn from(value: &IContinue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContinue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContinue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IContinueCallback(pub ::windows::runtime::IUnknown);
impl IContinueCallback {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn FContinue(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn FContinuePrinting<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ncntprinted: i32, ncurpage: i32, pwszprintstatus: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ncntprinted), ::std::mem::transmute(ncurpage), pwszprintstatus.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContinueCallback {
    type Vtable = IContinueCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3072507082, 20072, 4123, [162, 188, 0, 170, 0, 64, 71, 112]);
}
impl ::std::convert::From<IContinueCallback> for ::windows::runtime::IUnknown {
    fn from(value: IContinueCallback) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IContinueCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IContinueCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContinueCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContinueCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinueCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ncntprinted: i32, ncurpage: i32, pwszprintstatus: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_BZ_ICON: u32 = 601u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_BZ_MESSAGE1: u32 = 602u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_BZ_RETRY: u32 = 600u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_BZ_SWITCHTO: u32 = 604u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CI_BROWSE: u32 = 130u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CI_CURRENT: u32 = 121u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CI_CURRENTICON: u32 = 122u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CI_DEFAULT: u32 = 123u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CI_DEFAULTICON: u32 = 124u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CI_FROMFILE: u32 = 125u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CI_FROMFILEEDIT: u32 = 126u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CI_GROUP: u32 = 120u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CI_ICONDISPLAY: u32 = 131u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CI_ICONLIST: u32 = 127u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CI_LABEL: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CI_LABELEDIT: u32 = 129u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CV_ACTIVATEAS: u32 = 156u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CV_ACTIVATELIST: u32 = 154u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CV_CHANGEICON: u32 = 153u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CV_CONVERTLIST: u32 = 158u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CV_CONVERTTO: u32 = 155u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CV_DISPLAYASICON: u32 = 152u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CV_ICONDISPLAY: u32 = 165u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CV_OBJECTTYPE: u32 = 150u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_CV_RESULTTEXT: u32 = 157u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_EL_AUTOMATIC: u32 = 202u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_EL_CANCELLINK: u32 = 209u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_EL_CHANGESOURCE: u32 = 201u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_EL_COL1: u32 = 220u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_EL_COL2: u32 = 221u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_EL_COL3: u32 = 222u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_EL_LINKSLISTBOX: u32 = 206u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_EL_LINKSOURCE: u32 = 216u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_EL_LINKTYPE: u32 = 217u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_EL_MANUAL: u32 = 212u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_EL_OPENSOURCE: u32 = 211u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_EL_UPDATENOW: u32 = 210u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_GP_CONVERT: u32 = 1013u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_GP_OBJECTICON: u32 = 1014u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_GP_OBJECTLOCATION: u32 = 1022u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_GP_OBJECTNAME: u32 = 1009u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_GP_OBJECTSIZE: u32 = 1011u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_GP_OBJECTTYPE: u32 = 1010u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_ADDCONTROL: u32 = 2115u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_CHANGEICON: u32 = 2105u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_CONTROLTYPELIST: u32 = 2116u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_CREATEFROMFILE: u32 = 2101u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_CREATENEW: u32 = 2100u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_DISPLAYASICON: u32 = 2104u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_FILE: u32 = 2106u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_FILEDISPLAY: u32 = 2107u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_FILETEXT: u32 = 2112u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_FILETYPE: u32 = 2113u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_ICONDISPLAY: u32 = 2110u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_INSERTCONTROL: u32 = 2114u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_LINKFILE: u32 = 2102u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_OBJECTTYPELIST: u32 = 2103u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_OBJECTTYPETEXT: u32 = 2111u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_RESULTIMAGE: u32 = 2108u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_IO_RESULTTEXT: u32 = 2109u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_LP_AUTOMATIC: u32 = 1016u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_LP_BREAKLINK: u32 = 1008u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_LP_CHANGESOURCE: u32 = 1015u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_LP_DATE: u32 = 1018u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_LP_LINKSOURCE: u32 = 1012u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_LP_MANUAL: u32 = 1017u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_LP_OPENSOURCE: u32 = 1006u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_LP_TIME: u32 = 1019u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_LP_UPDATENOW: u32 = 1007u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_OLEUIHELP: u32 = 99u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PS_CHANGEICON: u32 = 508u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PS_DISPLAYASICON: u32 = 506u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PS_DISPLAYLIST: u32 = 505u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PS_ICONDISPLAY: u32 = 507u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PS_PASTE: u32 = 500u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PS_PASTELINK: u32 = 501u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PS_PASTELINKLIST: u32 = 504u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PS_PASTELIST: u32 = 503u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PS_RESULTIMAGE: u32 = 509u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PS_RESULTTEXT: u32 = 510u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PS_SOURCETEXT: u32 = 502u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PU_CONVERT: u32 = 902u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PU_ICON: u32 = 908u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PU_LINKS: u32 = 900u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_PU_TEXT: u32 = 901u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_UL_METER: u32 = 1029u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_UL_PERCENT: u32 = 1031u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_UL_PROGRESS: u32 = 1032u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_UL_STOP: u32 = 1030u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_VP_ASICON: u32 = 1003u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_VP_CHANGEICON: u32 = 1001u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_VP_EDITABLE: u32 = 1002u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_VP_ICONDISPLAY: u32 = 1021u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_VP_PERCENT: u32 = 1000u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_VP_RELATIVE: u32 = 1005u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_VP_RESULTIMAGE: u32 = 1033u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_VP_SCALETXT: u32 = 1034u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDC_VP_SPIN: u32 = 1006u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_BUSY: u32 = 1006u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_CANNOTUPDATELINK: u32 = 1008u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_CHANGEICON: u32 = 1001u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_CHANGEICONBROWSE: u32 = 1011u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_CHANGESOURCE: u32 = 1009u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_CHANGESOURCE4: u32 = 1013u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_CONVERT: u32 = 1002u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_CONVERT4: u32 = 1103u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_CONVERTONLY: u32 = 1012u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_CONVERTONLY4: u32 = 1104u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_EDITLINKS: u32 = 1004u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_EDITLINKS4: u32 = 1105u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_GNRLPROPS: u32 = 1100u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_GNRLPROPS4: u32 = 1106u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_INSERTFILEBROWSE: u32 = 1010u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_INSERTOBJECT: u32 = 1000u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_LINKPROPS: u32 = 1102u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_LINKPROPS4: u32 = 1107u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_LINKSOURCEUNAVAILABLE: u32 = 1020u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_LINKTYPECHANGED: u32 = 1022u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_LINKTYPECHANGEDA: u32 = 1026u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_LINKTYPECHANGEDW: u32 = 1022u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_OUTOFMEMORY: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_PASTESPECIAL: u32 = 1003u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_PASTESPECIAL4: u32 = 1108u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_SERVERNOTFOUND: u32 = 1023u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_SERVERNOTREG: u32 = 1021u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_SERVERNOTREGA: u32 = 1025u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_SERVERNOTREGW: u32 = 1021u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_UPDATELINKS: u32 = 1007u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IDD_VIEWPROPS: u32 = 1101u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const ID_BROWSE_ADDCONTROL: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const ID_BROWSE_CHANGEICON: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const ID_BROWSE_CHANGESOURCE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const ID_BROWSE_INSERTFILE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDropSource(pub ::windows::runtime::IUnknown);
impl IDropSource {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn QueryContinueDrag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fescapepressed: Param0, grfkeystate: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fescapepressed.into_param().abi(), ::std::mem::transmute(grfkeystate)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GiveFeedback(&self, dweffect: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dweffect)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDropSource {
    type Vtable = IDropSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(289, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IDropSource> for ::windows::runtime::IUnknown {
    fn from(value: IDropSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDropSource> for ::windows::runtime::IUnknown {
    fn from(value: &IDropSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDropSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDropSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fescapepressed: super::super::Foundation::BOOL, grfkeystate: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dweffect: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDropSourceNotify(pub ::windows::runtime::IUnknown);
impl IDropSourceNotify {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn DragEnterTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndtarget: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hwndtarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn DragLeaveTarget(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDropSourceNotify {
    type Vtable = IDropSourceNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(299, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IDropSourceNotify> for ::windows::runtime::IUnknown {
    fn from(value: IDropSourceNotify) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDropSourceNotify> for ::windows::runtime::IUnknown {
    fn from(value: &IDropSourceNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDropSourceNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDropSourceNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropSourceNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndtarget: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDropTarget(pub ::windows::runtime::IUnknown);
impl IDropTarget {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn DragEnter<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINTL>>(&self, pdataobj: Param0, grfkeystate: u32, pt: Param2, pdweffect: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pdataobj.into_param().abi(), ::std::mem::transmute(grfkeystate), pt.into_param().abi(), ::std::mem::transmute(pdweffect)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn DragOver<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINTL>>(&self, grfkeystate: u32, pt: Param1, pdweffect: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfkeystate), pt.into_param().abi(), ::std::mem::transmute(pdweffect)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn DragLeave(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Drop<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINTL>>(&self, pdataobj: Param0, grfkeystate: u32, pt: Param2, pdweffect: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdataobj.into_param().abi(), ::std::mem::transmute(grfkeystate), pt.into_param().abi(), ::std::mem::transmute(pdweffect)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDropTarget {
    type Vtable = IDropTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(290, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IDropTarget> for ::windows::runtime::IUnknown {
    fn from(value: IDropTarget) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDropTarget> for ::windows::runtime::IUnknown {
    fn from(value: &IDropTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDropTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDropTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataobj: ::windows::runtime::RawPtr, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataobj: ::windows::runtime::RawPtr, grfkeystate: u32, pt: super::super::Foundation::POINTL, pdweffect: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnterpriseDropTarget(pub ::windows::runtime::IUnknown);
impl IEnterpriseDropTarget {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetDropSourceEnterpriseId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, identity: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), identity.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn IsEvaluatingEdpPolicy(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnterpriseDropTarget {
    type Vtable = IEnterpriseDropTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(957233272, 64853, 19992, [129, 157, 70, 130, 8, 28, 12, 253]);
}
impl ::std::convert::From<IEnterpriseDropTarget> for ::windows::runtime::IUnknown {
    fn from(value: IEnterpriseDropTarget) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnterpriseDropTarget> for ::windows::runtime::IUnknown {
    fn from(value: &IEnterpriseDropTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnterpriseDropTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnterpriseDropTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseDropTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identity: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumOLEVERB(pub ::windows::runtime::IUnknown);
impl IEnumOLEVERB {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumOLEVERB> {
        let mut result__: <IEnumOLEVERB as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumOLEVERB>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumOLEVERB {
    type Vtable = IEnumOLEVERB_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(260, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumOLEVERB> for ::windows::runtime::IUnknown {
    fn from(value: IEnumOLEVERB) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumOLEVERB> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumOLEVERB) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumOLEVERB {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumOLEVERB {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOLEVERB_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumOleDocumentViews(pub ::windows::runtime::IUnknown);
impl IEnumOleDocumentViews {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Next(&self, cviews: u32, rgpview: *mut ::std::option::Option<IOleDocumentView>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cviews), ::std::mem::transmute(rgpview), ::std::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Skip(&self, cviews: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cviews)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumOleDocumentViews> {
        let mut result__: <IEnumOleDocumentViews as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumOleDocumentViews>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumOleDocumentViews {
    type Vtable = IEnumOleDocumentViews_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3072507080, 20072, 4123, [162, 188, 0, 170, 0, 64, 71, 112]);
}
impl ::std::convert::From<IEnumOleDocumentViews> for ::windows::runtime::IUnknown {
    fn from(value: IEnumOleDocumentViews) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumOleDocumentViews> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumOleDocumentViews) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumOleDocumentViews {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumOleDocumentViews {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOleDocumentViews_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cviews: u32, rgpview: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cviews: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumOleUndoUnits(pub ::windows::runtime::IUnknown);
impl IEnumOleUndoUnits {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::std::option::Option<IOleUndoUnit>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumOleUndoUnits> {
        let mut result__: <IEnumOleUndoUnits as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumOleUndoUnits>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumOleUndoUnits {
    type Vtable = IEnumOleUndoUnits_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3018310464, 61335, 4558, [155, 201, 0, 170, 0, 96, 142, 1]);
}
impl ::std::convert::From<IEnumOleUndoUnits> for ::windows::runtime::IUnknown {
    fn from(value: IEnumOleUndoUnits) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumOleUndoUnits> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumOleUndoUnits) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumOleUndoUnits {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumOleUndoUnits {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOleUndoUnits_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFont(pub ::windows::runtime::IUnknown);
impl IFont {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn Size(&self) -> ::windows::runtime::Result<super::Com::CY> {
        let mut result__: <super::Com::CY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::CY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn SetSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::CY>>(&self, size: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), size.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Bold(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetBold<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bold: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bold.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Italic(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetItalic<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, italic: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), italic.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Underline(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetUnderline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, underline: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), underline.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Strikethrough(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetStrikethrough<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, strikethrough: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), strikethrough.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Weight(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetWeight(&self, weight: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(weight)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Charset(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetCharset(&self, charset: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(charset)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn hFont(&self) -> ::windows::runtime::Result<super::super::Graphics::Gdi::HFONT> {
        let mut result__: <super::super::Graphics::Gdi::HFONT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Graphics::Gdi::HFONT>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IFont> {
        let mut result__: <IFont as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFont>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, IFont>>(&self, pfontother: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), pfontother.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetRatio(&self, cylogical: i32, cyhimetric: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(cylogical), ::std::mem::transmute(cyhimetric)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn QueryTextMetrics(&self) -> ::windows::runtime::Result<super::super::Graphics::Gdi::TEXTMETRICW> {
        let mut result__: <super::super::Graphics::Gdi::TEXTMETRICW as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Graphics::Gdi::TEXTMETRICW>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn AddRefHfont<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HFONT>>(&self, hfont: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), hfont.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn ReleaseHfont<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HFONT>>(&self, hfont: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), hfont.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn SetHdc<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(&self, hdc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), hdc.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFont {
    type Vtable = IFont_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3203850242, 43124, 4122, [139, 186, 0, 170, 0, 48, 12, 171]);
}
impl ::std::convert::From<IFont> for ::windows::runtime::IUnknown {
    fn from(value: IFont) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFont> for ::windows::runtime::IUnknown {
    fn from(value: &IFont) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFont {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFont {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFont_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psize: *mut super::Com::CY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, size: super::Com::CY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbold: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bold: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitalic: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, italic: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punderline: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, underline: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstrikethrough: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strikethrough: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pweight: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, weight: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcharset: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, charset: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phfont: *mut super::super::Graphics::Gdi::HFONT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfont: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfontother: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cylogical: i32, cyhimetric: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFontDisp(pub ::windows::runtime::IUnknown);
impl IFontDisp {}
unsafe impl ::windows::runtime::Interface for IFontDisp {
    type Vtable = IFontDisp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3203850243, 43124, 4122, [139, 186, 0, 170, 0, 48, 12, 171]);
}
impl ::std::convert::From<IFontDisp> for ::windows::runtime::IUnknown {
    fn from(value: IFontDisp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFontDisp> for ::windows::runtime::IUnknown {
    fn from(value: &IFontDisp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFontDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFontDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFontDisp> for Automation::IDispatch {
    fn from(value: IFontDisp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFontDisp> for Automation::IDispatch {
    fn from(value: &IFontDisp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, Automation::IDispatch> for IFontDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, Automation::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, Automation::IDispatch> for &IFontDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, Automation::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontDisp_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFontEventsDisp(pub ::windows::runtime::IUnknown);
impl IFontEventsDisp {}
unsafe impl ::windows::runtime::Interface for IFontEventsDisp {
    type Vtable = IFontEventsDisp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1324748810, 44936, 4560, [152, 70, 0, 192, 79, 194, 153, 147]);
}
impl ::std::convert::From<IFontEventsDisp> for ::windows::runtime::IUnknown {
    fn from(value: IFontEventsDisp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFontEventsDisp> for ::windows::runtime::IUnknown {
    fn from(value: &IFontEventsDisp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFontEventsDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFontEventsDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFontEventsDisp> for Automation::IDispatch {
    fn from(value: IFontEventsDisp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFontEventsDisp> for Automation::IDispatch {
    fn from(value: &IFontEventsDisp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, Automation::IDispatch> for IFontEventsDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, Automation::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, Automation::IDispatch> for &IFontEventsDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, Automation::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontEventsDisp_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IGNOREMIME(pub i32);
pub const IGNOREMIME_PROMPT: IGNOREMIME = IGNOREMIME(1i32);
pub const IGNOREMIME_TEXT: IGNOREMIME = IGNOREMIME(2i32);
impl ::std::convert::From<i32> for IGNOREMIME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IGNOREMIME {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IGetOleObject(pub ::windows::runtime::IUnknown);
impl IGetOleObject {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetOleObject(&self, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppvobj)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGetOleObject {
    type Vtable = IGetOleObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2322603424, 20459, 4123, [168, 46, 8, 0, 43, 43, 35, 55]);
}
impl ::std::convert::From<IGetOleObject> for ::windows::runtime::IUnknown {
    fn from(value: IGetOleObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGetOleObject> for ::windows::runtime::IUnknown {
    fn from(value: &IGetOleObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGetOleObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGetOleObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetOleObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IGetVBAObject(pub ::windows::runtime::IUnknown);
impl IGetVBAObject {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetObject(&self, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void, dwreserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppvobj), ::std::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGetVBAObject {
    type Vtable = IGetVBAObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2440247904, 16204, 4123, [163, 246, 0, 170, 0, 52, 228, 233]);
}
impl ::std::convert::From<IGetVBAObject> for ::windows::runtime::IUnknown {
    fn from(value: IGetVBAObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGetVBAObject> for ::windows::runtime::IUnknown {
    fn from(value: &IGetVBAObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGetVBAObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGetVBAObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetVBAObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void, dwreserved: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const INSTALL_SCOPE_INVALID: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const INSTALL_SCOPE_MACHINE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const INSTALL_SCOPE_USER: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_CHECKDISPLAYASICON: i32 = 16i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_CHECKLINK: i32 = 8i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_CREATEFILEOBJECT: i32 = 64i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_CREATELINKOBJECT: i32 = 128i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_CREATENEWOBJECT: i32 = 32i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_DISABLEDISPLAYASICON: i32 = 1024i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_DISABLELINK: i32 = 256i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_HIDECHANGEICON: i32 = 2048i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_SELECTCREATECONTROL: i32 = 8192i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_SELECTCREATEFROMFILE: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_SELECTCREATENEW: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_SHOWHELP: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_SHOWINSERTCONTROL: i32 = 4096i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const IOF_VERIFYSERVERSEXIST: i32 = 512i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IObjectWithSite(pub ::windows::runtime::IUnknown);
impl IObjectWithSite {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetSite<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punksite: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), punksite.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetSite<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IObjectWithSite {
    type Vtable = IObjectWithSite_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4232577443, 11177, 4559, [162, 41, 0, 170, 0, 61, 115, 82]);
}
impl ::std::convert::From<IObjectWithSite> for ::windows::runtime::IUnknown {
    fn from(value: IObjectWithSite) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IObjectWithSite> for ::windows::runtime::IUnknown {
    fn from(value: &IObjectWithSite) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IObjectWithSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IObjectWithSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectWithSite_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punksite: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvsite: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleAdviseHolder(pub ::windows::runtime::IUnknown);
impl IOleAdviseHolder {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IAdviseSink>>(&self, padvise: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), padvise.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwconnection)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn EnumAdvise(&self) -> ::windows::runtime::Result<super::Com::IEnumSTATDATA> {
        let mut result__: <super::Com::IEnumSTATDATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IEnumSTATDATA>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn SendOnRename<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IMoniker>>(&self, pmk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pmk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SendOnSave(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SendOnClose(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleAdviseHolder {
    type Vtable = IOleAdviseHolder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(273, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleAdviseHolder> for ::windows::runtime::IUnknown {
    fn from(value: IOleAdviseHolder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleAdviseHolder> for ::windows::runtime::IUnknown {
    fn from(value: &IOleAdviseHolder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleAdviseHolder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleAdviseHolder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleAdviseHolder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, padvise: ::windows::runtime::RawPtr, pdwconnection: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwconnection: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumadvise: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleCache(pub ::windows::runtime::IUnknown);
impl IOleCache {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn Cache(&self, pformatetc: *const super::Com::FORMATETC, advf: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetc), ::std::mem::transmute(advf), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Uncache(&self, dwconnection: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwconnection)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn EnumCache(&self) -> ::windows::runtime::Result<super::Com::IEnumSTATDATA> {
        let mut result__: <super::Com::IEnumSTATDATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IEnumSTATDATA>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn InitCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>>(&self, pdataobject: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdataobject.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetData<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetc), ::std::mem::transmute(pmedium), frelease.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleCache {
    type Vtable = IOleCache_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(286, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleCache> for ::windows::runtime::IUnknown {
    fn from(value: IOleCache) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleCache> for ::windows::runtime::IUnknown {
    fn from(value: &IOleCache) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleCache {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleCache {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleCache_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const super::Com::FORMATETC, advf: u32, pdwconnection: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwconnection: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstatdata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const super::Com::FORMATETC, pmedium: *const ::std::mem::ManuallyDrop<super::Com::STGMEDIUM>, frelease: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleCache2(pub ::windows::runtime::IUnknown);
impl IOleCache2 {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn Cache(&self, pformatetc: *const super::Com::FORMATETC, advf: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetc), ::std::mem::transmute(advf), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Uncache(&self, dwconnection: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwconnection)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn EnumCache(&self) -> ::windows::runtime::Result<super::Com::IEnumSTATDATA> {
        let mut result__: <super::Com::IEnumSTATDATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IEnumSTATDATA>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn InitCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>>(&self, pdataobject: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdataobject.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetData<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetc), ::std::mem::transmute(pmedium), frelease.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn UpdateCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>>(&self, pdataobject: Param0, grfupdf: UPDFCACHE_FLAGS, preserved: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pdataobject.into_param().abi(), ::std::mem::transmute(grfupdf), ::std::mem::transmute(preserved)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn DiscardCache(&self, dwdiscardoptions: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdiscardoptions)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleCache2 {
    type Vtable = IOleCache2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(296, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleCache2> for ::windows::runtime::IUnknown {
    fn from(value: IOleCache2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleCache2> for ::windows::runtime::IUnknown {
    fn from(value: &IOleCache2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleCache2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleCache2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleCache2> for IOleCache {
    fn from(value: IOleCache2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleCache2> for IOleCache {
    fn from(value: &IOleCache2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleCache> for IOleCache2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleCache> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleCache> for &IOleCache2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleCache> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleCache2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const super::Com::FORMATETC, advf: u32, pdwconnection: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwconnection: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstatdata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const super::Com::FORMATETC, pmedium: *const ::std::mem::ManuallyDrop<super::Com::STGMEDIUM>, frelease: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataobject: ::windows::runtime::RawPtr, grfupdf: UPDFCACHE_FLAGS, preserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdiscardoptions: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleCacheControl(pub ::windows::runtime::IUnknown);
impl IOleCacheControl {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn OnRun<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>>(&self, pdataobject: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pdataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnStop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleCacheControl {
    type Vtable = IOleCacheControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(297, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleCacheControl> for ::windows::runtime::IUnknown {
    fn from(value: IOleCacheControl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleCacheControl> for ::windows::runtime::IUnknown {
    fn from(value: &IOleCacheControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleCacheControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleCacheControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleCacheControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleClientSite(pub ::windows::runtime::IUnknown);
impl IOleClientSite {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SaveObject(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn GetMoniker(&self, dwassign: u32, dwwhichmoniker: u32) -> ::windows::runtime::Result<super::Com::IMoniker> {
        let mut result__: <super::Com::IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwassign), ::std::mem::transmute(dwwhichmoniker), &mut result__).from_abi::<super::Com::IMoniker>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetContainer(&self) -> ::windows::runtime::Result<IOleContainer> {
        let mut result__: <IOleContainer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOleContainer>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn ShowObject(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnShowWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn RequestNewObjectLayout(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleClientSite {
    type Vtable = IOleClientSite_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(280, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleClientSite> for ::windows::runtime::IUnknown {
    fn from(value: IOleClientSite) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleClientSite> for ::windows::runtime::IUnknown {
    fn from(value: &IOleClientSite) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleClientSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleClientSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleClientSite_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontainer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleCommandTarget(pub ::windows::runtime::IUnknown);
impl IOleCommandTarget {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn QueryStatus(&self, pguidcmdgroup: *const ::windows::runtime::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidcmdgroup), ::std::mem::transmute(ccmds), ::std::mem::transmute(prgcmds), ::std::mem::transmute(pcmdtext)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Exec(&self, pguidcmdgroup: *const ::windows::runtime::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::Com::VARIANT, pvaout: *mut super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidcmdgroup), ::std::mem::transmute(ncmdid), ::std::mem::transmute(ncmdexecopt), ::std::mem::transmute(pvain), ::std::mem::transmute(pvaout)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleCommandTarget {
    type Vtable = IOleCommandTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3072507083, 20072, 4123, [162, 188, 0, 170, 0, 64, 71, 112]);
}
impl ::std::convert::From<IOleCommandTarget> for ::windows::runtime::IUnknown {
    fn from(value: IOleCommandTarget) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleCommandTarget> for ::windows::runtime::IUnknown {
    fn from(value: &IOleCommandTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleCommandTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleCommandTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleCommandTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidcmdgroup: *const ::windows::runtime::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidcmdgroup: *const ::windows::runtime::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pvaout: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleContainer(pub ::windows::runtime::IUnknown);
impl IOleContainer {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn ParseDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pbc: Param0, pszdisplayname: Param1, pcheaten: *mut u32, ppmkout: *mut ::std::option::Option<super::Com::IMoniker>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pbc.into_param().abi(), pszdisplayname.into_param().abi(), ::std::mem::transmute(pcheaten), ::std::mem::transmute(ppmkout)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn EnumObjects(&self, grfflags: u32) -> ::windows::runtime::Result<super::Com::IEnumUnknown> {
        let mut result__: <super::Com::IEnumUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfflags), &mut result__).from_abi::<super::Com::IEnumUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn LockContainer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), flock.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleContainer {
    type Vtable = IOleContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(283, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleContainer> for ::windows::runtime::IUnknown {
    fn from(value: IOleContainer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleContainer> for ::windows::runtime::IUnknown {
    fn from(value: &IOleContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleContainer> for IParseDisplayName {
    fn from(value: IOleContainer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleContainer> for IParseDisplayName {
    fn from(value: &IOleContainer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IParseDisplayName> for IOleContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IParseDisplayName> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IParseDisplayName> for &IOleContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IParseDisplayName> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pszdisplayname: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfflags: u32, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flock: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleControl(pub ::windows::runtime::IUnknown);
impl IOleControl {
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn GetControlInfo(&self, pci: *mut CONTROLINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pci)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn OnMnemonic(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnAmbientPropertyChange(&self, dispid: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn FreezeEvents<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bfreeze: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), bfreeze.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleControl {
    type Vtable = IOleControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443336, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::std::convert::From<IOleControl> for ::windows::runtime::IUnknown {
    fn from(value: IOleControl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleControl> for ::windows::runtime::IUnknown {
    fn from(value: &IOleControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pci: *mut CONTROLINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispid: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bfreeze: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleControlSite(pub ::windows::runtime::IUnknown);
impl IOleControlSite {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnControlInfoChanged(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn LockInPlaceActive<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), flock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetExtendedControl(&self) -> ::windows::runtime::Result<Automation::IDispatch> {
        let mut result__: <Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn TransformCoords(&self, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: XFORMCOORDS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pptlhimetric), ::std::mem::transmute(pptfcontainer), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg), ::std::mem::transmute(grfmodifiers)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgotfocus: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), fgotfocus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn ShowPropertyFrame(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleControlSite {
    type Vtable = IOleControlSite_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443337, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::std::convert::From<IOleControlSite> for ::windows::runtime::IUnknown {
    fn from(value: IOleControlSite) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleControlSite> for ::windows::runtime::IUnknown {
    fn from(value: &IOleControlSite) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleControlSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleControlSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleControlSite_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flock: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: XFORMCOORDS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fgotfocus: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleDocument(pub ::windows::runtime::IUnknown);
impl IOleDocument {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn CreateView<'a, Param0: ::windows::runtime::IntoParam<'a, IOleInPlaceSite>, Param1: ::windows::runtime::IntoParam<'a, super::Com::IStream>>(&self, pipsite: Param0, pstm: Param1, dwreserved: u32) -> ::windows::runtime::Result<IOleDocumentView> {
        let mut result__: <IOleDocumentView as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pipsite.into_param().abi(), pstm.into_param().abi(), ::std::mem::transmute(dwreserved), &mut result__).from_abi::<IOleDocumentView>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetDocMiscStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn EnumViews(&self, ppenum: *mut ::std::option::Option<IEnumOleDocumentViews>, ppview: *mut ::std::option::Option<IOleDocumentView>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppenum), ::std::mem::transmute(ppview)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleDocument {
    type Vtable = IOleDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3072507077, 20072, 4123, [162, 188, 0, 170, 0, 64, 71, 112]);
}
impl ::std::convert::From<IOleDocument> for ::windows::runtime::IUnknown {
    fn from(value: IOleDocument) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleDocument> for ::windows::runtime::IUnknown {
    fn from(value: &IOleDocument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleDocument_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pipsite: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, dwreserved: u32, ppview: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr, ppview: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleDocumentSite(pub ::windows::runtime::IUnknown);
impl IOleDocumentSite {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn ActivateMe<'a, Param0: ::windows::runtime::IntoParam<'a, IOleDocumentView>>(&self, pviewtoactivate: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pviewtoactivate.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleDocumentSite {
    type Vtable = IOleDocumentSite_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3072507079, 20072, 4123, [162, 188, 0, 170, 0, 64, 71, 112]);
}
impl ::std::convert::From<IOleDocumentSite> for ::windows::runtime::IUnknown {
    fn from(value: IOleDocumentSite) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleDocumentSite> for ::windows::runtime::IUnknown {
    fn from(value: &IOleDocumentSite) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleDocumentSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleDocumentSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleDocumentSite_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pviewtoactivate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleDocumentView(pub ::windows::runtime::IUnknown);
impl IOleDocumentView {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetInPlaceSite<'a, Param0: ::windows::runtime::IntoParam<'a, IOleInPlaceSite>>(&self, pipsite: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pipsite.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetInPlaceSite(&self) -> ::windows::runtime::Result<IOleInPlaceSite> {
        let mut result__: <IOleInPlaceSite as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOleInPlaceSite>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetDocument(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetRect(&self, prcview: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(prcview)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetRect(&self) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetRectComplex(&self, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(prcview), ::std::mem::transmute(prchscroll), ::std::mem::transmute(prcvscroll), ::std::mem::transmute(prcsizebox)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn UIActivate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuiactivate: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), fuiactivate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Open(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn CloseView(&self, dwreserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn SaveViewState<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>>(&self, pstm: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn ApplyViewState<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>>(&self, pstm: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Clone<'a, Param0: ::windows::runtime::IntoParam<'a, IOleInPlaceSite>>(&self, pipsitenew: Param0) -> ::windows::runtime::Result<IOleDocumentView> {
        let mut result__: <IOleDocumentView as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pipsitenew.into_param().abi(), &mut result__).from_abi::<IOleDocumentView>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOleDocumentView {
    type Vtable = IOleDocumentView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3072507078, 20072, 4123, [162, 188, 0, 170, 0, 64, 71, 112]);
}
impl ::std::convert::From<IOleDocumentView> for ::windows::runtime::IUnknown {
    fn from(value: IOleDocumentView) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleDocumentView> for ::windows::runtime::IUnknown {
    fn from(value: &IOleDocumentView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleDocumentView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleDocumentView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleDocumentView_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pipsite: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppipsite: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prcview: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prcview: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuiactivate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwreserved: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pipsitenew: ::windows::runtime::RawPtr, ppviewnew: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleInPlaceActiveObject(pub ::windows::runtime::IUnknown);
impl IOleInPlaceActiveObject {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn ContextSensitiveHelp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fentermode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fentermode.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TranslateAccelerator(&self, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpmsg)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnFrameWindowActivate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, factivate: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), factivate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnDocWindowActivate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, factivate: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), factivate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn ResizeBorder<'a, Param1: ::windows::runtime::IntoParam<'a, IOleInPlaceUIWindow>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prcborder: *const super::super::Foundation::RECT, puiwindow: Param1, fframewindow: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(prcborder), puiwindow.into_param().abi(), fframewindow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn EnableModeless<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleInPlaceActiveObject {
    type Vtable = IOleInPlaceActiveObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(279, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleInPlaceActiveObject> for ::windows::runtime::IUnknown {
    fn from(value: IOleInPlaceActiveObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleInPlaceActiveObject> for ::windows::runtime::IUnknown {
    fn from(value: &IOleInPlaceActiveObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleInPlaceActiveObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleInPlaceActiveObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleInPlaceActiveObject> for IOleWindow {
    fn from(value: IOleInPlaceActiveObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleInPlaceActiveObject> for IOleWindow {
    fn from(value: &IOleInPlaceActiveObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for IOleInPlaceActiveObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for &IOleInPlaceActiveObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceActiveObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fentermode: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, factivate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, factivate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prcborder: *const super::super::Foundation::RECT, puiwindow: ::windows::runtime::RawPtr, fframewindow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleInPlaceFrame(pub ::windows::runtime::IUnknown);
impl IOleInPlaceFrame {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn ContextSensitiveHelp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fentermode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fentermode.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetBorder(&self) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn RequestBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pborderwidths)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pborderwidths)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetActiveObject<'a, Param0: ::windows::runtime::IntoParam<'a, IOleInPlaceActiveObject>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pactiveobject: Param0, pszobjname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pactiveobject.into_param().abi(), pszobjname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn InsertMenus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::WindowsAndMessaging::HMENU>>(&self, hmenushared: Param0, lpmenuwidths: *mut OleMenuGroupWidths) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), hmenushared.into_param().abi(), ::std::mem::transmute(lpmenuwidths)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn SetMenu<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::WindowsAndMessaging::HMENU>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hmenushared: Param0, holemenu: isize, hwndactiveobject: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), hmenushared.into_param().abi(), ::std::mem::transmute(holemenu), hwndactiveobject.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn RemoveMenus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::WindowsAndMessaging::HMENU>>(&self, hmenushared: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), hmenushared.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetStatusText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstatustext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszstatustext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn EnableModeless<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TranslateAccelerator(&self, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpmsg), ::std::mem::transmute(wid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleInPlaceFrame {
    type Vtable = IOleInPlaceFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(278, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleInPlaceFrame> for ::windows::runtime::IUnknown {
    fn from(value: IOleInPlaceFrame) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleInPlaceFrame> for ::windows::runtime::IUnknown {
    fn from(value: &IOleInPlaceFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleInPlaceFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleInPlaceFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleInPlaceFrame> for IOleInPlaceUIWindow {
    fn from(value: IOleInPlaceFrame) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleInPlaceFrame> for IOleInPlaceUIWindow {
    fn from(value: &IOleInPlaceFrame) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleInPlaceUIWindow> for IOleInPlaceFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleInPlaceUIWindow> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleInPlaceUIWindow> for &IOleInPlaceFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleInPlaceUIWindow> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IOleInPlaceFrame> for IOleWindow {
    fn from(value: IOleInPlaceFrame) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleInPlaceFrame> for IOleWindow {
    fn from(value: &IOleInPlaceFrame) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for IOleInPlaceFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for &IOleInPlaceFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fentermode: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprectborder: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pactiveobject: ::windows::runtime::RawPtr, pszobjname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OleMenuGroupWidths) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleInPlaceObject(pub ::windows::runtime::IUnknown);
impl IOleInPlaceObject {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn ContextSensitiveHelp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fentermode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fentermode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn InPlaceDeactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn UIDeactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetObjectRects(&self, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(lprcposrect), ::std::mem::transmute(lprccliprect)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn ReactivateAndUndo(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleInPlaceObject {
    type Vtable = IOleInPlaceObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(275, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleInPlaceObject> for ::windows::runtime::IUnknown {
    fn from(value: IOleInPlaceObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleInPlaceObject> for ::windows::runtime::IUnknown {
    fn from(value: &IOleInPlaceObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleInPlaceObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleInPlaceObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleInPlaceObject> for IOleWindow {
    fn from(value: IOleInPlaceObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleInPlaceObject> for IOleWindow {
    fn from(value: &IOleInPlaceObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for IOleInPlaceObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for &IOleInPlaceObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fentermode: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleInPlaceObjectWindowless(pub ::windows::runtime::IUnknown);
impl IOleInPlaceObjectWindowless {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn ContextSensitiveHelp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fentermode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fentermode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn InPlaceDeactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn UIDeactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetObjectRects(&self, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(lprcposrect), ::std::mem::transmute(lprccliprect)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn ReactivateAndUndo(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnWindowMessage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, msg: u32, wparam: Param1, lparam: Param2) -> ::windows::runtime::Result<super::super::Foundation::LRESULT> {
        let mut result__: <super::super::Foundation::LRESULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::LRESULT>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetDropTarget(&self) -> ::windows::runtime::Result<IDropTarget> {
        let mut result__: <IDropTarget as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDropTarget>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOleInPlaceObjectWindowless {
    type Vtable = IOleInPlaceObjectWindowless_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(471881420, 24308, 4123, [139, 200, 0, 170, 0, 62, 59, 41]);
}
impl ::std::convert::From<IOleInPlaceObjectWindowless> for ::windows::runtime::IUnknown {
    fn from(value: IOleInPlaceObjectWindowless) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleInPlaceObjectWindowless> for ::windows::runtime::IUnknown {
    fn from(value: &IOleInPlaceObjectWindowless) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleInPlaceObjectWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleInPlaceObjectWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleInPlaceObjectWindowless> for IOleInPlaceObject {
    fn from(value: IOleInPlaceObjectWindowless) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleInPlaceObjectWindowless> for IOleInPlaceObject {
    fn from(value: &IOleInPlaceObjectWindowless) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleInPlaceObject> for IOleInPlaceObjectWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleInPlaceObject> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleInPlaceObject> for &IOleInPlaceObjectWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleInPlaceObject> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IOleInPlaceObjectWindowless> for IOleWindow {
    fn from(value: IOleInPlaceObjectWindowless) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleInPlaceObjectWindowless> for IOleWindow {
    fn from(value: &IOleInPlaceObjectWindowless) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for IOleInPlaceObjectWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for &IOleInPlaceObjectWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceObjectWindowless_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fentermode: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdroptarget: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleInPlaceSite(pub ::windows::runtime::IUnknown);
impl IOleInPlaceSite {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn ContextSensitiveHelp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fentermode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fentermode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn CanInPlaceActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnInPlaceActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnUIActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn GetWindowContext(&self, ppframe: *mut ::std::option::Option<IOleInPlaceFrame>, ppdoc: *mut ::std::option::Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OIFI) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppframe), ::std::mem::transmute(ppdoc), ::std::mem::transmute(lprcposrect), ::std::mem::transmute(lprccliprect), ::std::mem::transmute(lpframeinfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Scroll<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::SIZE>>(&self, scrollextant: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), scrollextant.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnUIDeactivate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fundoable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), fundoable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnInPlaceDeactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn DiscardUndoState(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn DeactivateAndUndo(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnPosRectChange(&self, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(lprcposrect)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleInPlaceSite {
    type Vtable = IOleInPlaceSite_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(281, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleInPlaceSite> for ::windows::runtime::IUnknown {
    fn from(value: IOleInPlaceSite) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleInPlaceSite> for ::windows::runtime::IUnknown {
    fn from(value: &IOleInPlaceSite) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleInPlaceSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleInPlaceSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleInPlaceSite> for IOleWindow {
    fn from(value: IOleInPlaceSite) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleInPlaceSite> for IOleWindow {
    fn from(value: &IOleInPlaceSite) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for IOleInPlaceSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for &IOleInPlaceSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceSite_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fentermode: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppframe: *mut ::windows::runtime::RawPtr, ppdoc: *mut ::windows::runtime::RawPtr, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OIFI) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scrollextant: super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fundoable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleInPlaceSiteEx(pub ::windows::runtime::IUnknown);
impl IOleInPlaceSiteEx {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn ContextSensitiveHelp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fentermode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fentermode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn CanInPlaceActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnInPlaceActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnUIActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn GetWindowContext(&self, ppframe: *mut ::std::option::Option<IOleInPlaceFrame>, ppdoc: *mut ::std::option::Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OIFI) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppframe), ::std::mem::transmute(ppdoc), ::std::mem::transmute(lprcposrect), ::std::mem::transmute(lprccliprect), ::std::mem::transmute(lpframeinfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Scroll<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::SIZE>>(&self, scrollextant: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), scrollextant.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnUIDeactivate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fundoable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), fundoable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnInPlaceDeactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn DiscardUndoState(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn DeactivateAndUndo(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnPosRectChange(&self, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(lprcposrect)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnInPlaceActivateEx(&self, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfnoredraw), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnInPlaceDeactivateEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fnoredraw: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), fnoredraw.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn RequestUIActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleInPlaceSiteEx {
    type Vtable = IOleInPlaceSiteEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2620173696, 13348, 4559, [182, 112, 0, 170, 0, 76, 214, 216]);
}
impl ::std::convert::From<IOleInPlaceSiteEx> for ::windows::runtime::IUnknown {
    fn from(value: IOleInPlaceSiteEx) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleInPlaceSiteEx> for ::windows::runtime::IUnknown {
    fn from(value: &IOleInPlaceSiteEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleInPlaceSiteEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleInPlaceSiteEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleInPlaceSiteEx> for IOleInPlaceSite {
    fn from(value: IOleInPlaceSiteEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleInPlaceSiteEx> for IOleInPlaceSite {
    fn from(value: &IOleInPlaceSiteEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleInPlaceSite> for IOleInPlaceSiteEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleInPlaceSite> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleInPlaceSite> for &IOleInPlaceSiteEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleInPlaceSite> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IOleInPlaceSiteEx> for IOleWindow {
    fn from(value: IOleInPlaceSiteEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleInPlaceSiteEx> for IOleWindow {
    fn from(value: &IOleInPlaceSiteEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for IOleInPlaceSiteEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for &IOleInPlaceSiteEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceSiteEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fentermode: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppframe: *mut ::windows::runtime::RawPtr, ppdoc: *mut ::windows::runtime::RawPtr, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OIFI) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scrollextant: super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fundoable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fnoredraw: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleInPlaceSiteWindowless(pub ::windows::runtime::IUnknown);
impl IOleInPlaceSiteWindowless {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn ContextSensitiveHelp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fentermode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fentermode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn CanInPlaceActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnInPlaceActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnUIActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn GetWindowContext(&self, ppframe: *mut ::std::option::Option<IOleInPlaceFrame>, ppdoc: *mut ::std::option::Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OIFI) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppframe), ::std::mem::transmute(ppdoc), ::std::mem::transmute(lprcposrect), ::std::mem::transmute(lprccliprect), ::std::mem::transmute(lpframeinfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Scroll<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::SIZE>>(&self, scrollextant: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), scrollextant.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnUIDeactivate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fundoable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), fundoable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnInPlaceDeactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn DiscardUndoState(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn DeactivateAndUndo(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnPosRectChange(&self, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(lprcposrect)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnInPlaceActivateEx(&self, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfnoredraw), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnInPlaceDeactivateEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fnoredraw: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), fnoredraw.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn RequestUIActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn CanWindowlessActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetCapture(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetCapture<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fcapture: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), fcapture.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetFocus(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ffocus: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ffocus.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetDC(&self, prect: *const super::super::Foundation::RECT, grfflags: u32) -> ::windows::runtime::Result<super::super::Graphics::Gdi::HDC> {
        let mut result__: <super::super::Graphics::Gdi::HDC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(prect), ::std::mem::transmute(grfflags), &mut result__).from_abi::<super::super::Graphics::Gdi::HDC>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn ReleaseDC<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(&self, hdc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), hdc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn InvalidateRect<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prect: *const super::super::Foundation::RECT, ferase: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(prect), ferase.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn InvalidateRgn<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HRGN>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hrgn: Param0, ferase: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), hrgn.into_param().abi(), ferase.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn ScrollRect(&self, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(dx), ::std::mem::transmute(dy), ::std::mem::transmute(prectscroll), ::std::mem::transmute(prectclip)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn AdjustRect(&self, prc: *mut super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(prc)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnDefWindowMessage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, msg: u32, wparam: Param1, lparam: Param2) -> ::windows::runtime::Result<super::super::Foundation::LRESULT> {
        let mut result__: <super::super::Foundation::LRESULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::LRESULT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOleInPlaceSiteWindowless {
    type Vtable = IOleInPlaceSiteWindowless_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2452532640, 13348, 4559, [182, 112, 0, 170, 0, 76, 214, 216]);
}
impl ::std::convert::From<IOleInPlaceSiteWindowless> for ::windows::runtime::IUnknown {
    fn from(value: IOleInPlaceSiteWindowless) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleInPlaceSiteWindowless> for ::windows::runtime::IUnknown {
    fn from(value: &IOleInPlaceSiteWindowless) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleInPlaceSiteWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleInPlaceSiteWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleInPlaceSiteWindowless> for IOleInPlaceSiteEx {
    fn from(value: IOleInPlaceSiteWindowless) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleInPlaceSiteWindowless> for IOleInPlaceSiteEx {
    fn from(value: &IOleInPlaceSiteWindowless) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleInPlaceSiteEx> for IOleInPlaceSiteWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleInPlaceSiteEx> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleInPlaceSiteEx> for &IOleInPlaceSiteWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleInPlaceSiteEx> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IOleInPlaceSiteWindowless> for IOleInPlaceSite {
    fn from(value: IOleInPlaceSiteWindowless) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleInPlaceSiteWindowless> for IOleInPlaceSite {
    fn from(value: &IOleInPlaceSiteWindowless) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleInPlaceSite> for IOleInPlaceSiteWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleInPlaceSite> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleInPlaceSite> for &IOleInPlaceSiteWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleInPlaceSite> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IOleInPlaceSiteWindowless> for IOleWindow {
    fn from(value: IOleInPlaceSiteWindowless) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleInPlaceSiteWindowless> for IOleWindow {
    fn from(value: &IOleInPlaceSiteWindowless) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for IOleInPlaceSiteWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for &IOleInPlaceSiteWindowless {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceSiteWindowless_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fentermode: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppframe: *mut ::windows::runtime::RawPtr, ppdoc: *mut ::windows::runtime::RawPtr, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OIFI) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scrollextant: super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fundoable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fnoredraw: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcapture: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ffocus: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prect: *const super::super::Foundation::RECT, grfflags: u32, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prc: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleInPlaceUIWindow(pub ::windows::runtime::IUnknown);
impl IOleInPlaceUIWindow {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn ContextSensitiveHelp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fentermode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fentermode.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetBorder(&self) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn RequestBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pborderwidths)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pborderwidths)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetActiveObject<'a, Param0: ::windows::runtime::IntoParam<'a, IOleInPlaceActiveObject>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pactiveobject: Param0, pszobjname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pactiveobject.into_param().abi(), pszobjname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleInPlaceUIWindow {
    type Vtable = IOleInPlaceUIWindow_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(277, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleInPlaceUIWindow> for ::windows::runtime::IUnknown {
    fn from(value: IOleInPlaceUIWindow) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleInPlaceUIWindow> for ::windows::runtime::IUnknown {
    fn from(value: &IOleInPlaceUIWindow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleInPlaceUIWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleInPlaceUIWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleInPlaceUIWindow> for IOleWindow {
    fn from(value: IOleInPlaceUIWindow) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleInPlaceUIWindow> for IOleWindow {
    fn from(value: &IOleInPlaceUIWindow) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for IOleInPlaceUIWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleWindow> for &IOleInPlaceUIWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleWindow> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceUIWindow_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fentermode: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprectborder: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pactiveobject: ::windows::runtime::RawPtr, pszobjname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleItemContainer(pub ::windows::runtime::IUnknown);
impl IOleItemContainer {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn ParseDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pbc: Param0, pszdisplayname: Param1, pcheaten: *mut u32, ppmkout: *mut ::std::option::Option<super::Com::IMoniker>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pbc.into_param().abi(), pszdisplayname.into_param().abi(), ::std::mem::transmute(pcheaten), ::std::mem::transmute(ppmkout)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn EnumObjects(&self, grfflags: u32) -> ::windows::runtime::Result<super::Com::IEnumUnknown> {
        let mut result__: <super::Com::IEnumUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfflags), &mut result__).from_abi::<super::Com::IEnumUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn LockContainer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), flock.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Com::IBindCtx>, T: ::windows::runtime::Interface>(&self, pszitem: Param0, dwspeedneeded: u32, pbc: Param2) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pszitem.into_param().abi(), ::std::mem::transmute(dwspeedneeded), pbc.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetObjectStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::IBindCtx>, T: ::windows::runtime::Interface>(&self, pszitem: Param0, pbc: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pszitem.into_param().abi(), pbc.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn IsRunning<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszitem: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pszitem.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleItemContainer {
    type Vtable = IOleItemContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(284, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleItemContainer> for ::windows::runtime::IUnknown {
    fn from(value: IOleItemContainer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleItemContainer> for ::windows::runtime::IUnknown {
    fn from(value: &IOleItemContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleItemContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleItemContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleItemContainer> for IOleContainer {
    fn from(value: IOleItemContainer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleItemContainer> for IOleContainer {
    fn from(value: &IOleItemContainer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleContainer> for IOleItemContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleContainer> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleContainer> for &IOleItemContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleContainer> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IOleItemContainer> for IParseDisplayName {
    fn from(value: IOleItemContainer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleItemContainer> for IParseDisplayName {
    fn from(value: &IOleItemContainer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IParseDisplayName> for IOleItemContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IParseDisplayName> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IParseDisplayName> for &IOleItemContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IParseDisplayName> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleItemContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pszdisplayname: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfflags: u32, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flock: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszitem: super::super::Foundation::PWSTR, dwspeedneeded: u32, pbc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszitem: super::super::Foundation::PWSTR, pbc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvstorage: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszitem: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleLink(pub ::windows::runtime::IUnknown);
impl IOleLink {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetUpdateOptions(&self, dwupdateopt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwupdateopt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetUpdateOptions(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn SetSourceMoniker<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IMoniker>>(&self, pmk: Param0, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pmk.into_param().abi(), ::std::mem::transmute(rclsid)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn GetSourceMoniker(&self) -> ::windows::runtime::Result<super::Com::IMoniker> {
        let mut result__: <super::Com::IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IMoniker>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetSourceDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstatustext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pszstatustext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetSourceDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn BindToSource<'a, Param1: ::windows::runtime::IntoParam<'a, super::Com::IBindCtx>>(&self, bindflags: u32, pbc: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(bindflags), pbc.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn BindIfRunning(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetBoundSource(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn UnbindSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn Update<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IBindCtx>>(&self, pbc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pbc.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleLink {
    type Vtable = IOleLink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(285, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleLink> for ::windows::runtime::IUnknown {
    fn from(value: IOleLink) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleLink> for ::windows::runtime::IUnknown {
    fn from(value: &IOleLink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleLink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwupdateopt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwupdateopt: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstatustext: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bindflags: u32, pbc: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleObject(pub ::windows::runtime::IUnknown);
impl IOleObject {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetClientSite<'a, Param0: ::windows::runtime::IntoParam<'a, IOleClientSite>>(&self, pclientsite: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pclientsite.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetClientSite(&self) -> ::windows::runtime::Result<IOleClientSite> {
        let mut result__: <IOleClientSite as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOleClientSite>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetHostNames<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, szcontainerapp: Param0, szcontainerobj: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), szcontainerapp.into_param().abi(), szcontainerobj.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Close(&self, dwsaveoption: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsaveoption)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn SetMoniker<'a, Param1: ::windows::runtime::IntoParam<'a, super::Com::IMoniker>>(&self, dwwhichmoniker: u32, pmk: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwwhichmoniker), pmk.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn GetMoniker(&self, dwassign: u32, dwwhichmoniker: u32) -> ::windows::runtime::Result<super::Com::IMoniker> {
        let mut result__: <super::Com::IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwassign), ::std::mem::transmute(dwwhichmoniker), &mut result__).from_abi::<super::Com::IMoniker>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn InitFromData<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pdataobject: Param0, fcreation: Param1, dwreserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pdataobject.into_param().abi(), fcreation.into_param().abi(), ::std::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn GetClipboardData(&self, dwreserved: u32) -> ::windows::runtime::Result<super::Com::IDataObject> {
        let mut result__: <super::Com::IDataObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwreserved), &mut result__).from_abi::<super::Com::IDataObject>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn DoVerb<'a, Param2: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: Param2, lindex: i32, hwndparent: Param4, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(iverb), ::std::mem::transmute(lpmsg), pactivesite.into_param().abi(), ::std::mem::transmute(lindex), hwndparent.into_param().abi(), ::std::mem::transmute(lprcposrect)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn EnumVerbs(&self) -> ::windows::runtime::Result<IEnumOLEVERB> {
        let mut result__: <IEnumOLEVERB as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumOLEVERB>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Update(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn IsUpToDate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetUserClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetUserType(&self, dwformoftype: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwformoftype), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetExtent(&self, dwdrawaspect: u32, psizel: *const super::super::Foundation::SIZE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdrawaspect), ::std::mem::transmute(psizel)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetExtent(&self, dwdrawaspect: u32) -> ::windows::runtime::Result<super::super::Foundation::SIZE> {
        let mut result__: <super::super::Foundation::SIZE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdrawaspect), &mut result__).from_abi::<super::super::Foundation::SIZE>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IAdviseSink>>(&self, padvsink: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), padvsink.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwconnection)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn EnumAdvise(&self) -> ::windows::runtime::Result<super::Com::IEnumSTATDATA> {
        let mut result__: <super::Com::IEnumSTATDATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IEnumSTATDATA>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetMiscStatus(&self, dwaspect: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwaspect), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn SetColorScheme(&self, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(plogpal)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleObject {
    type Vtable = IOleObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(274, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleObject> for ::windows::runtime::IUnknown {
    fn from(value: IOleObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleObject> for ::windows::runtime::IUnknown {
    fn from(value: &IOleObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclientsite: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclientsite: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, szcontainerapp: super::super::Foundation::PWSTR, szcontainerobj: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsaveoption: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwwhichmoniker: u32, pmk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataobject: ::windows::runtime::RawPtr, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwreserved: u32, ppdataobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: ::windows::runtime::RawPtr, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumoleverb: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwformoftype: u32, pszusertype: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: u32, psizel: *const super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: u32, psizel: *mut super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, padvsink: ::windows::runtime::RawPtr, pdwconnection: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwconnection: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumadvise: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleParentUndoUnit(pub ::windows::runtime::IUnknown);
impl IOleParentUndoUnit {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Do<'a, Param0: ::windows::runtime::IntoParam<'a, IOleUndoManager>>(&self, pundomanager: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pundomanager.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetUnitType(&self, pclsid: *mut ::windows::runtime::GUID, plid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pclsid), ::std::mem::transmute(plid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnNextAdd(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, IOleParentUndoUnit>>(&self, ppuu: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ppuu.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Close<'a, Param0: ::windows::runtime::IntoParam<'a, IOleParentUndoUnit>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ppuu: Param0, fcommit: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ppuu.into_param().abi(), fcommit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, IOleUndoUnit>>(&self, puu: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), puu.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn FindUnit<'a, Param0: ::windows::runtime::IntoParam<'a, IOleUndoUnit>>(&self, puu: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), puu.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetParentState(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOleParentUndoUnit {
    type Vtable = IOleParentUndoUnit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2717578032, 61335, 4558, [155, 201, 0, 170, 0, 96, 142, 1]);
}
impl ::std::convert::From<IOleParentUndoUnit> for ::windows::runtime::IUnknown {
    fn from(value: IOleParentUndoUnit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleParentUndoUnit> for ::windows::runtime::IUnknown {
    fn from(value: &IOleParentUndoUnit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleParentUndoUnit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleParentUndoUnit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleParentUndoUnit> for IOleUndoUnit {
    fn from(value: IOleParentUndoUnit) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleParentUndoUnit> for IOleUndoUnit {
    fn from(value: &IOleParentUndoUnit) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleUndoUnit> for IOleParentUndoUnit {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleUndoUnit> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleUndoUnit> for &IOleParentUndoUnit {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleUndoUnit> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleParentUndoUnit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pundomanager: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID, plid: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppuu: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppuu: ::windows::runtime::RawPtr, fcommit: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puu: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puu: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstate: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleUILinkContainerA(pub ::windows::runtime::IUnknown);
impl IOleUILinkContainerA {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetNextLink(&self, dwlink: u32) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink)))
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), ::std::mem::transmute(dwupdateopt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetLinkUpdateOptions(&self, dwlink: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetLinkSource<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwlink: u32, lpszdisplayname: Param1, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), lpszdisplayname.into_param().abi(), ::std::mem::transmute(lenfilename), ::std::mem::transmute(pcheaten), fvalidatesource.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PSTR, lplpszshortlinktype: *mut super::super::Foundation::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwlink),
            ::std::mem::transmute(lplpszdisplayname),
            ::std::mem::transmute(lplenfilename),
            ::std::mem::transmute(lplpszfulllinktype),
            ::std::mem::transmute(lplpszshortlinktype),
            ::std::mem::transmute(lpfsourceavailable),
            ::std::mem::transmute(lpfisselected),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OpenLinkSource(&self, dwlink: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn UpdateLink<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwlink: u32, ferrormessage: Param1, freserved: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), ferrormessage.into_param().abi(), freserved.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn CancelLink(&self, dwlink: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleUILinkContainerA {
    type Vtable = IOleUILinkContainerA_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IOleUILinkContainerA> for ::windows::runtime::IUnknown {
    fn from(value: IOleUILinkContainerA) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleUILinkContainerA> for ::windows::runtime::IUnknown {
    fn from(value: &IOleUILinkContainerA) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleUILinkContainerA {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleUILinkContainerA {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUILinkContainerA_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, dwupdateopt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lpszdisplayname: super::super::Foundation::PSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PSTR, lplpszshortlinktype: *mut super::super::Foundation::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleUILinkContainerW(pub ::windows::runtime::IUnknown);
impl IOleUILinkContainerW {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetNextLink(&self, dwlink: u32) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink)))
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), ::std::mem::transmute(dwupdateopt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetLinkUpdateOptions(&self, dwlink: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetLinkSource<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwlink: u32, lpszdisplayname: Param1, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), lpszdisplayname.into_param().abi(), ::std::mem::transmute(lenfilename), ::std::mem::transmute(pcheaten), fvalidatesource.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PWSTR, lplpszshortlinktype: *mut super::super::Foundation::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwlink),
            ::std::mem::transmute(lplpszdisplayname),
            ::std::mem::transmute(lplenfilename),
            ::std::mem::transmute(lplpszfulllinktype),
            ::std::mem::transmute(lplpszshortlinktype),
            ::std::mem::transmute(lpfsourceavailable),
            ::std::mem::transmute(lpfisselected),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OpenLinkSource(&self, dwlink: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn UpdateLink<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwlink: u32, ferrormessage: Param1, freserved: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), ferrormessage.into_param().abi(), freserved.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn CancelLink(&self, dwlink: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleUILinkContainerW {
    type Vtable = IOleUILinkContainerW_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IOleUILinkContainerW> for ::windows::runtime::IUnknown {
    fn from(value: IOleUILinkContainerW) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleUILinkContainerW> for ::windows::runtime::IUnknown {
    fn from(value: &IOleUILinkContainerW) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleUILinkContainerW {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleUILinkContainerW {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUILinkContainerW_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, dwupdateopt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lpszdisplayname: super::super::Foundation::PWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PWSTR, lplpszshortlinktype: *mut super::super::Foundation::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleUILinkInfoA(pub ::windows::runtime::IUnknown);
impl IOleUILinkInfoA {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetNextLink(&self, dwlink: u32) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink)))
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), ::std::mem::transmute(dwupdateopt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetLinkUpdateOptions(&self, dwlink: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetLinkSource<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwlink: u32, lpszdisplayname: Param1, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), lpszdisplayname.into_param().abi(), ::std::mem::transmute(lenfilename), ::std::mem::transmute(pcheaten), fvalidatesource.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PSTR, lplpszshortlinktype: *mut super::super::Foundation::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwlink),
            ::std::mem::transmute(lplpszdisplayname),
            ::std::mem::transmute(lplenfilename),
            ::std::mem::transmute(lplpszfulllinktype),
            ::std::mem::transmute(lplpszshortlinktype),
            ::std::mem::transmute(lpfsourceavailable),
            ::std::mem::transmute(lpfisselected),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OpenLinkSource(&self, dwlink: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn UpdateLink<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwlink: u32, ferrormessage: Param1, freserved: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), ferrormessage.into_param().abi(), freserved.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn CancelLink(&self, dwlink: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetLastUpdate(&self, dwlink: u32) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOleUILinkInfoA {
    type Vtable = IOleUILinkInfoA_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IOleUILinkInfoA> for ::windows::runtime::IUnknown {
    fn from(value: IOleUILinkInfoA) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleUILinkInfoA> for ::windows::runtime::IUnknown {
    fn from(value: &IOleUILinkInfoA) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleUILinkInfoA {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleUILinkInfoA {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleUILinkInfoA> for IOleUILinkContainerA {
    fn from(value: IOleUILinkInfoA) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleUILinkInfoA> for IOleUILinkContainerA {
    fn from(value: &IOleUILinkInfoA) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleUILinkContainerA> for IOleUILinkInfoA {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleUILinkContainerA> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleUILinkContainerA> for &IOleUILinkInfoA {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleUILinkContainerA> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUILinkInfoA_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, dwupdateopt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lpszdisplayname: super::super::Foundation::PSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PSTR, lplpszshortlinktype: *mut super::super::Foundation::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleUILinkInfoW(pub ::windows::runtime::IUnknown);
impl IOleUILinkInfoW {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetNextLink(&self, dwlink: u32) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink)))
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), ::std::mem::transmute(dwupdateopt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetLinkUpdateOptions(&self, dwlink: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetLinkSource<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwlink: u32, lpszdisplayname: Param1, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), lpszdisplayname.into_param().abi(), ::std::mem::transmute(lenfilename), ::std::mem::transmute(pcheaten), fvalidatesource.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PWSTR, lplpszshortlinktype: *mut super::super::Foundation::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwlink),
            ::std::mem::transmute(lplpszdisplayname),
            ::std::mem::transmute(lplenfilename),
            ::std::mem::transmute(lplpszfulllinktype),
            ::std::mem::transmute(lplpszshortlinktype),
            ::std::mem::transmute(lpfsourceavailable),
            ::std::mem::transmute(lpfisselected),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OpenLinkSource(&self, dwlink: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn UpdateLink<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwlink: u32, ferrormessage: Param1, freserved: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), ferrormessage.into_param().abi(), freserved.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn CancelLink(&self, dwlink: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetLastUpdate(&self, dwlink: u32) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlink), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOleUILinkInfoW {
    type Vtable = IOleUILinkInfoW_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IOleUILinkInfoW> for ::windows::runtime::IUnknown {
    fn from(value: IOleUILinkInfoW) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleUILinkInfoW> for ::windows::runtime::IUnknown {
    fn from(value: &IOleUILinkInfoW) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleUILinkInfoW {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleUILinkInfoW {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOleUILinkInfoW> for IOleUILinkContainerW {
    fn from(value: IOleUILinkInfoW) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOleUILinkInfoW> for IOleUILinkContainerW {
    fn from(value: &IOleUILinkInfoW) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleUILinkContainerW> for IOleUILinkInfoW {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleUILinkContainerW> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOleUILinkContainerW> for &IOleUILinkInfoW {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOleUILinkContainerW> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUILinkInfoW_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, dwupdateopt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lpszdisplayname: super::super::Foundation::PWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lplpszdisplayname: *mut super::super::Foundation::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut super::super::Foundation::PWSTR, lplpszshortlinktype: *mut super::super::Foundation::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleUIObjInfoA(pub ::windows::runtime::IUnknown);
impl IOleUIObjInfoA {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetObjectInfo(&self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PSTR, lplpsztype: *mut super::super::Foundation::PSTR, lplpszshorttype: *mut super::super::Foundation::PSTR, lplpszlocation: *mut super::super::Foundation::PSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwobject), ::std::mem::transmute(lpdwobjsize), ::std::mem::transmute(lplpszlabel), ::std::mem::transmute(lplpsztype), ::std::mem::transmute(lplpszshorttype), ::std::mem::transmute(lplpszlocation)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetConvertInfo(&self, dwobject: u32, lpclassid: *mut ::windows::runtime::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::runtime::GUID, lplpclsidexclude: *mut *mut ::windows::runtime::GUID, lpcclsidexclude: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwobject), ::std::mem::transmute(lpclassid), ::std::mem::transmute(lpwformat), ::std::mem::transmute(lpconvertdefaultclassid), ::std::mem::transmute(lplpclsidexclude), ::std::mem::transmute(lpcclsidexclude)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn ConvertObject(&self, dwobject: u32, clsidnew: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwobject), ::std::mem::transmute(clsidnew)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetViewInfo(&self, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwobject), ::std::mem::transmute(phmetapict), ::std::mem::transmute(pdvaspect), ::std::mem::transmute(pncurrentscale)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetViewInfo<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwobject), ::std::mem::transmute(hmetapict), ::std::mem::transmute(dvaspect), ::std::mem::transmute(ncurrentscale), brelativetoorig.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleUIObjInfoA {
    type Vtable = IOleUIObjInfoA_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IOleUIObjInfoA> for ::windows::runtime::IUnknown {
    fn from(value: IOleUIObjInfoA) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleUIObjInfoA> for ::windows::runtime::IUnknown {
    fn from(value: &IOleUIObjInfoA) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleUIObjInfoA {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleUIObjInfoA {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUIObjInfoA_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PSTR, lplpsztype: *mut super::super::Foundation::PSTR, lplpszshorttype: *mut super::super::Foundation::PSTR, lplpszlocation: *mut super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwobject: u32, lpclassid: *mut ::windows::runtime::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::runtime::GUID, lplpclsidexclude: *mut *mut ::windows::runtime::GUID, lpcclsidexclude: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwobject: u32, clsidnew: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleUIObjInfoW(pub ::windows::runtime::IUnknown);
impl IOleUIObjInfoW {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetObjectInfo(&self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PWSTR, lplpsztype: *mut super::super::Foundation::PWSTR, lplpszshorttype: *mut super::super::Foundation::PWSTR, lplpszlocation: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwobject), ::std::mem::transmute(lpdwobjsize), ::std::mem::transmute(lplpszlabel), ::std::mem::transmute(lplpsztype), ::std::mem::transmute(lplpszshorttype), ::std::mem::transmute(lplpszlocation)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetConvertInfo(&self, dwobject: u32, lpclassid: *mut ::windows::runtime::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::runtime::GUID, lplpclsidexclude: *mut *mut ::windows::runtime::GUID, lpcclsidexclude: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwobject), ::std::mem::transmute(lpclassid), ::std::mem::transmute(lpwformat), ::std::mem::transmute(lpconvertdefaultclassid), ::std::mem::transmute(lplpclsidexclude), ::std::mem::transmute(lpcclsidexclude)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn ConvertObject(&self, dwobject: u32, clsidnew: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwobject), ::std::mem::transmute(clsidnew)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetViewInfo(&self, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwobject), ::std::mem::transmute(phmetapict), ::std::mem::transmute(pdvaspect), ::std::mem::transmute(pncurrentscale)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetViewInfo<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwobject), ::std::mem::transmute(hmetapict), ::std::mem::transmute(dvaspect), ::std::mem::transmute(ncurrentscale), brelativetoorig.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleUIObjInfoW {
    type Vtable = IOleUIObjInfoW_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IOleUIObjInfoW> for ::windows::runtime::IUnknown {
    fn from(value: IOleUIObjInfoW) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleUIObjInfoW> for ::windows::runtime::IUnknown {
    fn from(value: &IOleUIObjInfoW) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleUIObjInfoW {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleUIObjInfoW {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUIObjInfoW_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut super::super::Foundation::PWSTR, lplpsztype: *mut super::super::Foundation::PWSTR, lplpszshorttype: *mut super::super::Foundation::PWSTR, lplpszlocation: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwobject: u32, lpclassid: *mut ::windows::runtime::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::runtime::GUID, lplpclsidexclude: *mut *mut ::windows::runtime::GUID, lpcclsidexclude: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwobject: u32, clsidnew: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleUndoManager(pub ::windows::runtime::IUnknown);
impl IOleUndoManager {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, IOleParentUndoUnit>>(&self, ppuu: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ppuu.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Close<'a, Param0: ::windows::runtime::IntoParam<'a, IOleParentUndoUnit>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ppuu: Param0, fcommit: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ppuu.into_param().abi(), fcommit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, IOleUndoUnit>>(&self, puu: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), puu.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetOpenParentState(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn DiscardFrom<'a, Param0: ::windows::runtime::IntoParam<'a, IOleUndoUnit>>(&self, puu: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), puu.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn UndoTo<'a, Param0: ::windows::runtime::IntoParam<'a, IOleUndoUnit>>(&self, puu: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), puu.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn RedoTo<'a, Param0: ::windows::runtime::IntoParam<'a, IOleUndoUnit>>(&self, puu: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), puu.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn EnumUndoable(&self) -> ::windows::runtime::Result<IEnumOleUndoUnits> {
        let mut result__: <IEnumOleUndoUnits as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumOleUndoUnits>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn EnumRedoable(&self) -> ::windows::runtime::Result<IEnumOleUndoUnits> {
        let mut result__: <IEnumOleUndoUnits as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumOleUndoUnits>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetLastUndoDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetLastRedoDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Enable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleUndoManager {
    type Vtable = IOleUndoManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3489788416, 61335, 4558, [155, 201, 0, 170, 0, 96, 142, 1]);
}
impl ::std::convert::From<IOleUndoManager> for ::windows::runtime::IUnknown {
    fn from(value: IOleUndoManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleUndoManager> for ::windows::runtime::IUnknown {
    fn from(value: &IOleUndoManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleUndoManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleUndoManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUndoManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppuu: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppuu: ::windows::runtime::RawPtr, fcommit: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puu: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstate: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puu: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puu: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puu: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleUndoUnit(pub ::windows::runtime::IUnknown);
impl IOleUndoUnit {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Do<'a, Param0: ::windows::runtime::IntoParam<'a, IOleUndoManager>>(&self, pundomanager: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pundomanager.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetUnitType(&self, pclsid: *mut ::windows::runtime::GUID, plid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pclsid), ::std::mem::transmute(plid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnNextAdd(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleUndoUnit {
    type Vtable = IOleUndoUnit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2303382448, 61335, 4558, [155, 201, 0, 170, 0, 96, 142, 1]);
}
impl ::std::convert::From<IOleUndoUnit> for ::windows::runtime::IUnknown {
    fn from(value: IOleUndoUnit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleUndoUnit> for ::windows::runtime::IUnknown {
    fn from(value: &IOleUndoUnit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleUndoUnit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleUndoUnit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUndoUnit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pundomanager: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID, plid: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOleWindow(pub ::windows::runtime::IUnknown);
impl IOleWindow {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn ContextSensitiveHelp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fentermode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fentermode.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOleWindow {
    type Vtable = IOleWindow_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(276, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IOleWindow> for ::windows::runtime::IUnknown {
    fn from(value: IOleWindow) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOleWindow> for ::windows::runtime::IUnknown {
    fn from(value: &IOleWindow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOleWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOleWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleWindow_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fentermode: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IParseDisplayName(pub ::windows::runtime::IUnknown);
impl IParseDisplayName {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn ParseDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pbc: Param0, pszdisplayname: Param1, pcheaten: *mut u32, ppmkout: *mut ::std::option::Option<super::Com::IMoniker>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pbc.into_param().abi(), pszdisplayname.into_param().abi(), ::std::mem::transmute(pcheaten), ::std::mem::transmute(ppmkout)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IParseDisplayName {
    type Vtable = IParseDisplayName_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(282, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IParseDisplayName> for ::windows::runtime::IUnknown {
    fn from(value: IParseDisplayName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IParseDisplayName> for ::windows::runtime::IUnknown {
    fn from(value: &IParseDisplayName) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IParseDisplayName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IParseDisplayName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IParseDisplayName_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pszdisplayname: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPerPropertyBrowsing(pub ::windows::runtime::IUnknown);
impl IPerPropertyBrowsing {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayString(&self, dispid: i32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispid), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn MapPropertyToPage(&self, dispid: i32) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispid), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetPredefinedStrings(&self, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispid), ::std::mem::transmute(pcastringsout), ::std::mem::transmute(pcacookiesout)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetPredefinedValue(&self, dispid: i32, dwcookie: u32) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispid), ::std::mem::transmute(dwcookie), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPerPropertyBrowsing {
    type Vtable = IPerPropertyBrowsing_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(929813418, 14405, 4123, [132, 237, 8, 0, 43, 46, 199, 19]);
}
impl ::std::convert::From<IPerPropertyBrowsing> for ::windows::runtime::IUnknown {
    fn from(value: IPerPropertyBrowsing) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPerPropertyBrowsing> for ::windows::runtime::IUnknown {
    fn from(value: &IPerPropertyBrowsing) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPerPropertyBrowsing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPerPropertyBrowsing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerPropertyBrowsing_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispid: i32, pbstr: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispid: i32, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispid: i32, dwcookie: u32, pvarout: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPersistMemory(pub ::windows::runtime::IUnknown);
impl IPersistMemory {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Load(&self, pmem: *const ::std::ffi::c_void, cbsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmem), ::std::mem::transmute(cbsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pmem: *mut ::std::ffi::c_void, fcleardirty: Param1, cbsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmem), fcleardirty.into_param().abi(), ::std::mem::transmute(cbsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn InitNew(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPersistMemory {
    type Vtable = IPersistMemory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3172656608, 42670, 4558, [189, 55, 80, 66, 0, 193, 0, 0]);
}
impl ::std::convert::From<IPersistMemory> for ::windows::runtime::IUnknown {
    fn from(value: IPersistMemory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPersistMemory> for ::windows::runtime::IUnknown {
    fn from(value: &IPersistMemory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPersistMemory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPersistMemory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IPersistMemory> for super::Com::IPersist {
    fn from(value: IPersistMemory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IPersistMemory> for super::Com::IPersist {
    fn from(value: &IPersistMemory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IPersist> for IPersistMemory {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IPersist> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IPersist> for &IPersistMemory {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IPersist> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistMemory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclassid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmem: *const ::std::ffi::c_void, cbsize: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmem: *mut ::std::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, cbsize: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPersistPropertyBag(pub ::windows::runtime::IUnknown);
impl IPersistPropertyBag {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn InitNew(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, Automation::IErrorLog>>(&self, ppropbag: Param0, perrorlog: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ppropbag.into_param().abi(), perrorlog.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, Automation::IPropertyBag>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ppropbag: Param0, fcleardirty: Param1, fsaveallproperties: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ppropbag.into_param().abi(), fcleardirty.into_param().abi(), fsaveallproperties.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPersistPropertyBag {
    type Vtable = IPersistPropertyBag_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(936922976, 17099, 4558, [129, 53, 0, 170, 0, 75, 184, 81]);
}
impl ::std::convert::From<IPersistPropertyBag> for ::windows::runtime::IUnknown {
    fn from(value: IPersistPropertyBag) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPersistPropertyBag> for ::windows::runtime::IUnknown {
    fn from(value: &IPersistPropertyBag) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPersistPropertyBag {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPersistPropertyBag {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IPersistPropertyBag> for super::Com::IPersist {
    fn from(value: IPersistPropertyBag) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IPersistPropertyBag> for super::Com::IPersist {
    fn from(value: &IPersistPropertyBag) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IPersist> for IPersistPropertyBag {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IPersist> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IPersist> for &IPersistPropertyBag {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IPersist> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistPropertyBag_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclassid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropbag: ::windows::runtime::RawPtr, perrorlog: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropbag: ::windows::runtime::RawPtr, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPersistPropertyBag2(pub ::windows::runtime::IUnknown);
impl IPersistPropertyBag2 {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn InitNew(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, IPropertyBag2>, Param1: ::windows::runtime::IntoParam<'a, Automation::IErrorLog>>(&self, ppropbag: Param0, perrlog: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ppropbag.into_param().abi(), perrlog.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, IPropertyBag2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ppropbag: Param0, fcleardirty: Param1, fsaveallproperties: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ppropbag.into_param().abi(), fcleardirty.into_param().abi(), fsaveallproperties.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPersistPropertyBag2 {
    type Vtable = IPersistPropertyBag2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(586504321, 10251, 4560, [168, 169, 0, 160, 201, 12, 32, 4]);
}
impl ::std::convert::From<IPersistPropertyBag2> for ::windows::runtime::IUnknown {
    fn from(value: IPersistPropertyBag2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPersistPropertyBag2> for ::windows::runtime::IUnknown {
    fn from(value: &IPersistPropertyBag2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPersistPropertyBag2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPersistPropertyBag2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IPersistPropertyBag2> for super::Com::IPersist {
    fn from(value: IPersistPropertyBag2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IPersistPropertyBag2> for super::Com::IPersist {
    fn from(value: &IPersistPropertyBag2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IPersist> for IPersistPropertyBag2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IPersist> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IPersist> for &IPersistPropertyBag2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IPersist> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistPropertyBag2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclassid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropbag: ::windows::runtime::RawPtr, perrlog: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropbag: ::windows::runtime::RawPtr, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPersistStreamInit(pub ::windows::runtime::IUnknown);
impl IPersistStreamInit {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>>(&self, pstm: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn InitNew(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPersistStreamInit {
    type Vtable = IPersistStreamInit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2144674688, 19975, 4123, [174, 45, 8, 0, 43, 46, 199, 19]);
}
impl ::std::convert::From<IPersistStreamInit> for ::windows::runtime::IUnknown {
    fn from(value: IPersistStreamInit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPersistStreamInit> for ::windows::runtime::IUnknown {
    fn from(value: &IPersistStreamInit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPersistStreamInit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPersistStreamInit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IPersistStreamInit> for super::Com::IPersist {
    fn from(value: IPersistStreamInit) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IPersistStreamInit> for super::Com::IPersist {
    fn from(value: &IPersistStreamInit) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IPersist> for IPersistStreamInit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IPersist> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Com::IPersist> for &IPersistStreamInit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Com::IPersist> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStreamInit_abi(
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
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPicture(pub ::windows::runtime::IUnknown);
impl IPicture {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Handle(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn hPal(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Width(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Height(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn Render<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(&self, hdc: Param0, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            hdc.into_param().abi(),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
            ::std::mem::transmute(cx),
            ::std::mem::transmute(cy),
            ::std::mem::transmute(xsrc),
            ::std::mem::transmute(ysrc),
            ::std::mem::transmute(cxsrc),
            ::std::mem::transmute(cysrc),
            ::std::mem::transmute(prcwbounds),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn set_hPal(&self, hpal: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(hpal)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn CurDC(&self) -> ::windows::runtime::Result<super::super::Graphics::Gdi::HDC> {
        let mut result__: <super::super::Graphics::Gdi::HDC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Graphics::Gdi::HDC>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn SelectPicture<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(&self, hdcin: Param0, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), hdcin.into_param().abi(), ::std::mem::transmute(phdcout), ::std::mem::transmute(phbmpout)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn KeepOriginalFormat(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetKeepOriginalFormat<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, keep: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), keep.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn PictureChanged(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn SaveAsFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstream: Param0, fsavememcopy: Param1) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pstream.into_param().abi(), fsavememcopy.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Attributes(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPicture {
    type Vtable = IPicture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2079852928, 48946, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
}
impl ::std::convert::From<IPicture> for ::windows::runtime::IUnknown {
    fn from(value: IPicture) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPicture> for ::windows::runtime::IUnknown {
    fn from(value: &IPicture) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPicture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPicture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPicture_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phandle: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phpal: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwidth: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pheight: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hpal: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkeep: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, keep: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattr: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPicture2(pub ::windows::runtime::IUnknown);
impl IPicture2 {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Handle(&self) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<usize>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn hPal(&self) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<usize>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Width(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Height(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn Render<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(&self, hdc: Param0, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            hdc.into_param().abi(),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
            ::std::mem::transmute(cx),
            ::std::mem::transmute(cy),
            ::std::mem::transmute(xsrc),
            ::std::mem::transmute(ysrc),
            ::std::mem::transmute(cxsrc),
            ::std::mem::transmute(cysrc),
            ::std::mem::transmute(prcwbounds),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn set_hPal(&self, hpal: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(hpal)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn CurDC(&self) -> ::windows::runtime::Result<super::super::Graphics::Gdi::HDC> {
        let mut result__: <super::super::Graphics::Gdi::HDC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Graphics::Gdi::HDC>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn SelectPicture<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(&self, hdcin: Param0, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), hdcin.into_param().abi(), ::std::mem::transmute(phdcout), ::std::mem::transmute(phbmpout)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn KeepOriginalFormat(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetKeepOriginalFormat<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, keep: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), keep.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn PictureChanged(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn SaveAsFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstream: Param0, fsavememcopy: Param1) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pstream.into_param().abi(), fsavememcopy.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Attributes(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPicture2 {
    type Vtable = IPicture2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4112014808, 8210, 19211, [170, 217, 240, 82, 198, 189, 72, 43]);
}
impl ::std::convert::From<IPicture2> for ::windows::runtime::IUnknown {
    fn from(value: IPicture2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPicture2> for ::windows::runtime::IUnknown {
    fn from(value: &IPicture2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPicture2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPicture2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPicture2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phandle: *mut usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phpal: *mut usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwidth: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pheight: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hpal: usize) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pkeep: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, keep: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattr: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPictureDisp(pub ::windows::runtime::IUnknown);
impl IPictureDisp {}
unsafe impl ::windows::runtime::Interface for IPictureDisp {
    type Vtable = IPictureDisp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2079852929, 48946, 4122, [139, 187, 0, 170, 0, 48, 12, 171]);
}
impl ::std::convert::From<IPictureDisp> for ::windows::runtime::IUnknown {
    fn from(value: IPictureDisp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPictureDisp> for ::windows::runtime::IUnknown {
    fn from(value: &IPictureDisp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPictureDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPictureDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IPictureDisp> for Automation::IDispatch {
    fn from(value: IPictureDisp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IPictureDisp> for Automation::IDispatch {
    fn from(value: &IPictureDisp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, Automation::IDispatch> for IPictureDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, Automation::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, Automation::IDispatch> for &IPictureDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, Automation::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPictureDisp_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPointerInactive(pub ::windows::runtime::IUnknown);
impl IPointerInactive {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetActivationPolicy(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnInactiveMouseMove(&self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(prectbounds), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(grfkeystate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn OnInactiveSetCursor<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(prectbounds), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(dwmousemsg), fsetalways.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPointerInactive {
    type Vtable = IPointerInactive_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1436027808, 13738, 4559, [182, 113, 0, 170, 0, 76, 214, 216]);
}
impl ::std::convert::From<IPointerInactive> for ::windows::runtime::IUnknown {
    fn from(value: IPointerInactive) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPointerInactive> for ::windows::runtime::IUnknown {
    fn from(value: &IPointerInactive) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPointerInactive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPointerInactive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerInactive_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpolicy: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPrint(pub ::windows::runtime::IUnknown);
impl IPrint {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetInitialPageNum(&self, nfirstpage: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(nfirstpage)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetPageInfo(&self, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pnfirstpage), ::std::mem::transmute(pcpages)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Print<'a, Param4: ::windows::runtime::IntoParam<'a, IContinueCallback>>(&self, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: Param4, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(grfflags),
            ::std::mem::transmute(pptd),
            ::std::mem::transmute(pppageset),
            ::std::mem::transmute(pstgmoptions),
            pcallback.into_param().abi(),
            ::std::mem::transmute(nfirstpage),
            ::std::mem::transmute(pcpagesprinted),
            ::std::mem::transmute(pnlastpage),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPrint {
    type Vtable = IPrint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3072507081, 20072, 4123, [162, 188, 0, 170, 0, 64, 71, 112]);
}
impl ::std::convert::From<IPrint> for ::windows::runtime::IUnknown {
    fn from(value: IPrint) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPrint> for ::windows::runtime::IUnknown {
    fn from(value: &IPrint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nfirstpage: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut ::std::mem::ManuallyDrop<super::Com::STGMEDIUM>, pcallback: ::windows::runtime::RawPtr, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyBag2(pub ::windows::runtime::IUnknown);
impl IPropertyBag2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Read<'a, Param2: ::windows::runtime::IntoParam<'a, Automation::IErrorLog>>(&self, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: Param2, pvarvalue: *mut super::Com::VARIANT, phrerror: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cproperties), ::std::mem::transmute(ppropbag), perrlog.into_param().abi(), ::std::mem::transmute(pvarvalue), ::std::mem::transmute(phrerror)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Write(&self, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cproperties), ::std::mem::transmute(ppropbag), ::std::mem::transmute(pvarvalue)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn CountProperties(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyInfo(&self, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(iproperty), ::std::mem::transmute(cproperties), ::std::mem::transmute(ppropbag), ::std::mem::transmute(pcproperties)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn LoadObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, Automation::IErrorLog>>(&self, pstrname: Param0, dwhint: u32, punkobject: Param2, perrlog: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pstrname.into_param().abi(), ::std::mem::transmute(dwhint), punkobject.into_param().abi(), perrlog.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyBag2 {
    type Vtable = IPropertyBag2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(586504322, 10251, 4560, [168, 169, 0, 160, 201, 12, 32, 4]);
}
impl ::std::convert::From<IPropertyBag2> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyBag2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPropertyBag2> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyBag2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyBag2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPropertyBag2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyBag2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: ::windows::runtime::RawPtr, pvarvalue: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, phrerror: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcproperties: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstrname: super::super::Foundation::PWSTR, dwhint: u32, punkobject: ::windows::runtime::RawPtr, perrlog: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyNotifySink(pub ::windows::runtime::IUnknown);
impl IPropertyNotifySink {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnChanged(&self, dispid: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnRequestEdit(&self, dispid: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyNotifySink {
    type Vtable = IPropertyNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2616966146, 61425, 4122, [132, 237, 0, 170, 0, 52, 29, 7]);
}
impl ::std::convert::From<IPropertyNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyNotifySink) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPropertyNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPropertyNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispid: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispid: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyPage(pub ::windows::runtime::IUnknown);
impl IPropertyPage {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetPageSite<'a, Param0: ::windows::runtime::IntoParam<'a, IPropertyPageSite>>(&self, ppagesite: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ppagesite.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hwndparent: Param0, prect: *const super::super::Foundation::RECT, bmodal: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(prect), bmodal.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Deactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetPageInfo(&self) -> ::windows::runtime::Result<PROPPAGEINFO> {
        let mut result__: <PROPPAGEINFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPPAGEINFO>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetObjects(&self, cobjects: u32, ppunk: *const ::std::option::Option<::windows::runtime::IUnknown>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(cobjects), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Show(&self, ncmdshow: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ncmdshow)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Move(&self, prect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(prect)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn IsPageDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Apply(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Help<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszhelpdir: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszhelpdir.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyPage {
    type Vtable = IPropertyPage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443341, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::std::convert::From<IPropertyPage> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyPage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPropertyPage> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyPage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyPage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPropertyPage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppagesite: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppageinfo: *mut PROPPAGEINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cobjects: u32, ppunk: *const ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ncmdshow: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszhelpdir: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyPage2(pub ::windows::runtime::IUnknown);
impl IPropertyPage2 {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetPageSite<'a, Param0: ::windows::runtime::IntoParam<'a, IPropertyPageSite>>(&self, ppagesite: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ppagesite.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hwndparent: Param0, prect: *const super::super::Foundation::RECT, bmodal: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(prect), bmodal.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Deactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetPageInfo(&self) -> ::windows::runtime::Result<PROPPAGEINFO> {
        let mut result__: <PROPPAGEINFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PROPPAGEINFO>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn SetObjects(&self, cobjects: u32, ppunk: *const ::std::option::Option<::windows::runtime::IUnknown>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(cobjects), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Show(&self, ncmdshow: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ncmdshow)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Move(&self, prect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(prect)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn IsPageDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Apply(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn Help<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszhelpdir: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszhelpdir.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn EditProperty(&self, dispid: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyPage2 {
    type Vtable = IPropertyPage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(31737445, 9388, 4123, [132, 237, 8, 0, 43, 46, 199, 19]);
}
impl ::std::convert::From<IPropertyPage2> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyPage2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPropertyPage2> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyPage2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyPage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPropertyPage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IPropertyPage2> for IPropertyPage {
    fn from(value: IPropertyPage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyPage2> for IPropertyPage {
    fn from(value: &IPropertyPage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyPage> for IPropertyPage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyPage> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPropertyPage> for &IPropertyPage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyPage> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPage2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppagesite: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppageinfo: *mut PROPPAGEINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cobjects: u32, ppunk: *const ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ncmdshow: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszhelpdir: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispid: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyPageSite(pub ::windows::runtime::IUnknown);
impl IPropertyPageSite {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnStatusChange(&self, dwflags: PROPPAGESTATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetLocaleID(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetPageContainer(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyPageSite {
    type Vtable = IPropertyPageSite_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443340, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::std::convert::From<IPropertyPageSite> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyPageSite) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPropertyPageSite> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyPageSite) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyPageSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPropertyPageSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPageSite_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: PROPPAGESTATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plocaleid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IProtectFocus(pub ::windows::runtime::IUnknown);
impl IProtectFocus {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn AllowFocusChange(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IProtectFocus {
    type Vtable = IProtectFocus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3625947299, 33110, 17655, [173, 40, 90, 187, 135, 0, 50, 116]);
}
impl ::std::convert::From<IProtectFocus> for ::windows::runtime::IUnknown {
    fn from(value: IProtectFocus) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IProtectFocus> for ::windows::runtime::IUnknown {
    fn from(value: &IProtectFocus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProtectFocus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProtectFocus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectFocus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfallow: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IProtectedModeMenuServices(pub ::windows::runtime::IUnknown);
impl IProtectedModeMenuServices {
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn CreateMenu(&self) -> ::windows::runtime::Result<super::super::UI::WindowsAndMessaging::HMENU> {
        let mut result__: <super::super::UI::WindowsAndMessaging::HMENU as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::UI::WindowsAndMessaging::HMENU>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn LoadMenu<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszmodulename: Param0, pszmenuname: Param1) -> ::windows::runtime::Result<super::super::UI::WindowsAndMessaging::HMENU> {
        let mut result__: <super::super::UI::WindowsAndMessaging::HMENU as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszmodulename.into_param().abi(), pszmenuname.into_param().abi(), &mut result__).from_abi::<super::super::UI::WindowsAndMessaging::HMENU>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn LoadMenuID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszmodulename: Param0, wresourceid: u16) -> ::windows::runtime::Result<super::super::UI::WindowsAndMessaging::HMENU> {
        let mut result__: <super::super::UI::WindowsAndMessaging::HMENU as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszmodulename.into_param().abi(), ::std::mem::transmute(wresourceid), &mut result__).from_abi::<super::super::UI::WindowsAndMessaging::HMENU>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IProtectedModeMenuServices {
    type Vtable = IProtectedModeMenuServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1942029806, 40447, 18951, [184, 60, 126, 255, 41, 12, 38, 110]);
}
impl ::std::convert::From<IProtectedModeMenuServices> for ::windows::runtime::IUnknown {
    fn from(value: IProtectedModeMenuServices) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IProtectedModeMenuServices> for ::windows::runtime::IUnknown {
    fn from(value: &IProtectedModeMenuServices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProtectedModeMenuServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProtectedModeMenuServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedModeMenuServices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszmodulename: super::super::Foundation::PWSTR, pszmenuname: super::super::Foundation::PWSTR, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszmodulename: super::super::Foundation::PWSTR, wresourceid: u16, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IProvideClassInfo(pub ::windows::runtime::IUnknown);
impl IProvideClassInfo {
    #[cfg(feature = "Win32_System_Ole_Automation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetClassInfo(&self) -> ::windows::runtime::Result<Automation::ITypeInfo> {
        let mut result__: <Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<Automation::ITypeInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IProvideClassInfo {
    type Vtable = IProvideClassInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443331, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::std::convert::From<IProvideClassInfo> for ::windows::runtime::IUnknown {
    fn from(value: IProvideClassInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IProvideClassInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IProvideClassInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProvideClassInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProvideClassInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideClassInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppti: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IProvideClassInfo2(pub ::windows::runtime::IUnknown);
impl IProvideClassInfo2 {
    #[cfg(feature = "Win32_System_Ole_Automation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetClassInfo(&self) -> ::windows::runtime::Result<Automation::ITypeInfo> {
        let mut result__: <Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<Automation::ITypeInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetGUID(&self, dwguidkind: u32) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwguidkind), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IProvideClassInfo2 {
    type Vtable = IProvideClassInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2797353664, 56234, 4558, [157, 227, 0, 170, 0, 75, 184, 81]);
}
impl ::std::convert::From<IProvideClassInfo2> for ::windows::runtime::IUnknown {
    fn from(value: IProvideClassInfo2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IProvideClassInfo2> for ::windows::runtime::IUnknown {
    fn from(value: &IProvideClassInfo2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProvideClassInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProvideClassInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IProvideClassInfo2> for IProvideClassInfo {
    fn from(value: IProvideClassInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProvideClassInfo2> for IProvideClassInfo {
    fn from(value: &IProvideClassInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProvideClassInfo> for IProvideClassInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProvideClassInfo> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProvideClassInfo> for &IProvideClassInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProvideClassInfo> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideClassInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppti: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwguidkind: u32, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IProvideMultipleClassInfo(pub ::windows::runtime::IUnknown);
impl IProvideMultipleClassInfo {
    #[cfg(feature = "Win32_System_Ole_Automation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetClassInfo(&self) -> ::windows::runtime::Result<Automation::ITypeInfo> {
        let mut result__: <Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<Automation::ITypeInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetGUID(&self, dwguidkind: u32) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwguidkind), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetMultiTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetInfoOfIndex(&self, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut ::std::option::Option<Automation::ITypeInfo>, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows::runtime::GUID, piidsource: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(iti), ::std::mem::transmute(dwflags), ::std::mem::transmute(ppticoclass), ::std::mem::transmute(pdwtiflags), ::std::mem::transmute(pcdispidreserved), ::std::mem::transmute(piidprimary), ::std::mem::transmute(piidsource)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProvideMultipleClassInfo {
    type Vtable = IProvideMultipleClassInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2813045185, 35203, 4559, [143, 32, 0, 128, 95, 44, 208, 100]);
}
impl ::std::convert::From<IProvideMultipleClassInfo> for ::windows::runtime::IUnknown {
    fn from(value: IProvideMultipleClassInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IProvideMultipleClassInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IProvideMultipleClassInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProvideMultipleClassInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProvideMultipleClassInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IProvideMultipleClassInfo> for IProvideClassInfo2 {
    fn from(value: IProvideMultipleClassInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProvideMultipleClassInfo> for IProvideClassInfo2 {
    fn from(value: &IProvideMultipleClassInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProvideClassInfo2> for IProvideMultipleClassInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProvideClassInfo2> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProvideClassInfo2> for &IProvideMultipleClassInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProvideClassInfo2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IProvideMultipleClassInfo> for IProvideClassInfo {
    fn from(value: IProvideMultipleClassInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProvideMultipleClassInfo> for IProvideClassInfo {
    fn from(value: &IProvideMultipleClassInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProvideClassInfo> for IProvideMultipleClassInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProvideClassInfo> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProvideClassInfo> for &IProvideMultipleClassInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProvideClassInfo> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideMultipleClassInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppti: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwguidkind: u32, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcti: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut ::windows::runtime::RawPtr, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows::runtime::GUID, piidsource: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IQuickActivate(pub ::windows::runtime::IUnknown);
impl IQuickActivate {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn QuickActivate(&self, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pqacontainer), ::std::mem::transmute(pqacontrol)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn SetContentExtent(&self, psizel: *const super::super::Foundation::SIZE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(psizel)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetContentExtent(&self) -> ::windows::runtime::Result<super::super::Foundation::SIZE> {
        let mut result__: <super::super::Foundation::SIZE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::SIZE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IQuickActivate {
    type Vtable = IQuickActivate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3478252816, 25342, 4559, [191, 134, 0, 160, 201, 3, 72, 54]);
}
impl ::std::convert::From<IQuickActivate> for ::windows::runtime::IUnknown {
    fn from(value: IQuickActivate) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IQuickActivate> for ::windows::runtime::IUnknown {
    fn from(value: &IQuickActivate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IQuickActivate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IQuickActivate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuickActivate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pqacontainer: *const ::std::mem::ManuallyDrop<QACONTAINER>, pqacontrol: *mut QACONTROL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psizel: *const super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psizel: *mut super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISimpleFrameSite(pub ::windows::runtime::IUnknown);
impl ISimpleFrameSite {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn PreMessageFilter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, hwnd: Param0, msg: u32, wp: Param2, lp: Param3, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hwnd.into_param().abi(), ::std::mem::transmute(msg), wp.into_param().abi(), lp.into_param().abi(), ::std::mem::transmute(plresult), ::std::mem::transmute(pdwcookie)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn PostMessageFilter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, hwnd: Param0, msg: u32, wp: Param2, lp: Param3, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), hwnd.into_param().abi(), ::std::mem::transmute(msg), wp.into_param().abi(), lp.into_param().abi(), ::std::mem::transmute(plresult), ::std::mem::transmute(dwcookie)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISimpleFrameSite {
    type Vtable = ISimpleFrameSite_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1948978689, 5350, 4123, [145, 78, 0, 170, 0, 48, 12, 171]);
}
impl ::std::convert::From<ISimpleFrameSite> for ::windows::runtime::IUnknown {
    fn from(value: ISimpleFrameSite) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISimpleFrameSite> for ::windows::runtime::IUnknown {
    fn from(value: &ISimpleFrameSite) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISimpleFrameSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISimpleFrameSite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleFrameSite_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISpecifyPropertyPages(pub ::windows::runtime::IUnknown);
impl ISpecifyPropertyPages {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetPages(&self) -> ::windows::runtime::Result<CAUUID> {
        let mut result__: <CAUUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<CAUUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISpecifyPropertyPages {
    type Vtable = ISpecifyPropertyPages_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443339, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::std::convert::From<ISpecifyPropertyPages> for ::windows::runtime::IUnknown {
    fn from(value: ISpecifyPropertyPages) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISpecifyPropertyPages> for ::windows::runtime::IUnknown {
    fn from(value: &ISpecifyPropertyPages) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpecifyPropertyPages {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpecifyPropertyPages {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpecifyPropertyPages_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppages: *mut CAUUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVBFormat(pub ::windows::runtime::IUnknown);
impl IVBFormat {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Format<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, vdata: *mut super::Com::VARIANT, bstrformat: Param1, lpbuffer: *mut ::std::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(vdata), bstrformat.into_param().abi(), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(cb), ::std::mem::transmute(lcid), ::std::mem::transmute(sfirstdayofweek), ::std::mem::transmute(sfirstweekofyear), ::std::mem::transmute(rcb)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVBFormat {
    type Vtable = IVBFormat_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2554985824, 14184, 4123, [141, 114, 174, 97, 100, 255, 227, 207]);
}
impl ::std::convert::From<IVBFormat> for ::windows::runtime::IUnknown {
    fn from(value: IVBFormat) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IVBFormat> for ::windows::runtime::IUnknown {
    fn from(value: &IVBFormat) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVBFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVBFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVBFormat_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vdata: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, bstrformat: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpbuffer: *mut ::std::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVBGetControl(pub ::windows::runtime::IUnknown);
impl IVBGetControl {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn EnumControls(&self, dwolecontf: OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS) -> ::windows::runtime::Result<super::Com::IEnumUnknown> {
        let mut result__: <super::Com::IEnumUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwolecontf), ::std::mem::transmute(dwwhich), &mut result__).from_abi::<super::Com::IEnumUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVBGetControl {
    type Vtable = IVBGetControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1084248224, 15409, 4123, [168, 46, 8, 0, 43, 43, 35, 55]);
}
impl ::std::convert::From<IVBGetControl> for ::windows::runtime::IUnknown {
    fn from(value: IVBGetControl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IVBGetControl> for ::windows::runtime::IUnknown {
    fn from(value: &IVBGetControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVBGetControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVBGetControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVBGetControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwolecontf: OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS, ppenumunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IViewObject(pub ::windows::runtime::IUnknown);
impl IViewObject {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn Draw<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param5: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(
        &self,
        dwdrawaspect: u32,
        lindex: i32,
        pvaspect: *mut ::std::ffi::c_void,
        ptd: *const super::Com::DVTARGETDEVICE,
        hdctargetdev: Param4,
        hdcdraw: Param5,
        lprcbounds: *const super::super::Foundation::RECTL,
        lprcwbounds: *const super::super::Foundation::RECTL,
        pfncontinue: isize,
        dwcontinue: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwdrawaspect),
            ::std::mem::transmute(lindex),
            ::std::mem::transmute(pvaspect),
            ::std::mem::transmute(ptd),
            hdctargetdev.into_param().abi(),
            hdcdraw.into_param().abi(),
            ::std::mem::transmute(lprcbounds),
            ::std::mem::transmute(lprcwbounds),
            ::std::mem::transmute(pfncontinue),
            ::std::mem::transmute(dwcontinue),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn GetColorSet<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: Param4, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdrawaspect), ::std::mem::transmute(lindex), ::std::mem::transmute(pvaspect), ::std::mem::transmute(ptd), hictargetdev.into_param().abi(), ::std::mem::transmute(ppcolorset)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Freeze(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdrawaspect), ::std::mem::transmute(lindex), ::std::mem::transmute(pvaspect), ::std::mem::transmute(pdwfreeze)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Unfreeze(&self, dwfreeze: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwfreeze)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn SetAdvise<'a, Param2: ::windows::runtime::IntoParam<'a, super::Com::IAdviseSink>>(&self, aspects: u32, advf: u32, padvsink: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(aspects), ::std::mem::transmute(advf), padvsink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn GetAdvise(&self, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut ::std::option::Option<super::Com::IAdviseSink>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(paspects), ::std::mem::transmute(padvf), ::std::mem::transmute(ppadvsink)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IViewObject {
    type Vtable = IViewObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(269, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IViewObject> for ::windows::runtime::IUnknown {
    fn from(value: IViewObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IViewObject> for ::windows::runtime::IUnknown {
    fn from(value: &IViewObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IViewObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IViewObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwfreeze: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aspects: u32, advf: u32, padvsink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IViewObject2(pub ::windows::runtime::IUnknown);
impl IViewObject2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn Draw<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param5: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(
        &self,
        dwdrawaspect: u32,
        lindex: i32,
        pvaspect: *mut ::std::ffi::c_void,
        ptd: *const super::Com::DVTARGETDEVICE,
        hdctargetdev: Param4,
        hdcdraw: Param5,
        lprcbounds: *const super::super::Foundation::RECTL,
        lprcwbounds: *const super::super::Foundation::RECTL,
        pfncontinue: isize,
        dwcontinue: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwdrawaspect),
            ::std::mem::transmute(lindex),
            ::std::mem::transmute(pvaspect),
            ::std::mem::transmute(ptd),
            hdctargetdev.into_param().abi(),
            hdcdraw.into_param().abi(),
            ::std::mem::transmute(lprcbounds),
            ::std::mem::transmute(lprcwbounds),
            ::std::mem::transmute(pfncontinue),
            ::std::mem::transmute(dwcontinue),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn GetColorSet<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: Param4, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdrawaspect), ::std::mem::transmute(lindex), ::std::mem::transmute(pvaspect), ::std::mem::transmute(ptd), hictargetdev.into_param().abi(), ::std::mem::transmute(ppcolorset)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Freeze(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdrawaspect), ::std::mem::transmute(lindex), ::std::mem::transmute(pvaspect), ::std::mem::transmute(pdwfreeze)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Unfreeze(&self, dwfreeze: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwfreeze)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn SetAdvise<'a, Param2: ::windows::runtime::IntoParam<'a, super::Com::IAdviseSink>>(&self, aspects: u32, advf: u32, padvsink: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(aspects), ::std::mem::transmute(advf), padvsink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn GetAdvise(&self, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut ::std::option::Option<super::Com::IAdviseSink>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(paspects), ::std::mem::transmute(padvf), ::std::mem::transmute(ppadvsink)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetExtent(&self, dwdrawaspect: u32, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE) -> ::windows::runtime::Result<super::super::Foundation::SIZE> {
        let mut result__: <super::super::Foundation::SIZE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdrawaspect), ::std::mem::transmute(lindex), ::std::mem::transmute(ptd), &mut result__).from_abi::<super::super::Foundation::SIZE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IViewObject2 {
    type Vtable = IViewObject2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(295, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IViewObject2> for ::windows::runtime::IUnknown {
    fn from(value: IViewObject2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IViewObject2> for ::windows::runtime::IUnknown {
    fn from(value: &IViewObject2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IViewObject2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IViewObject2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IViewObject2> for IViewObject {
    fn from(value: IViewObject2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IViewObject2> for IViewObject {
    fn from(value: &IViewObject2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewObject> for IViewObject2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewObject> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewObject> for &IViewObject2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewObject> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObject2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwfreeze: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aspects: u32, advf: u32, padvsink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: u32, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, lpsizel: *mut super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IViewObjectEx(pub ::windows::runtime::IUnknown);
impl IViewObjectEx {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn Draw<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param5: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(
        &self,
        dwdrawaspect: u32,
        lindex: i32,
        pvaspect: *mut ::std::ffi::c_void,
        ptd: *const super::Com::DVTARGETDEVICE,
        hdctargetdev: Param4,
        hdcdraw: Param5,
        lprcbounds: *const super::super::Foundation::RECTL,
        lprcwbounds: *const super::super::Foundation::RECTL,
        pfncontinue: isize,
        dwcontinue: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwdrawaspect),
            ::std::mem::transmute(lindex),
            ::std::mem::transmute(pvaspect),
            ::std::mem::transmute(ptd),
            hdctargetdev.into_param().abi(),
            hdcdraw.into_param().abi(),
            ::std::mem::transmute(lprcbounds),
            ::std::mem::transmute(lprcwbounds),
            ::std::mem::transmute(pfncontinue),
            ::std::mem::transmute(dwcontinue),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn GetColorSet<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: Param4, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdrawaspect), ::std::mem::transmute(lindex), ::std::mem::transmute(pvaspect), ::std::mem::transmute(ptd), hictargetdev.into_param().abi(), ::std::mem::transmute(ppcolorset)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Freeze(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdrawaspect), ::std::mem::transmute(lindex), ::std::mem::transmute(pvaspect), ::std::mem::transmute(pdwfreeze)).ok()
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn Unfreeze(&self, dwfreeze: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwfreeze)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn SetAdvise<'a, Param2: ::windows::runtime::IntoParam<'a, super::Com::IAdviseSink>>(&self, aspects: u32, advf: u32, padvsink: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(aspects), ::std::mem::transmute(advf), padvsink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    pub unsafe fn GetAdvise(&self, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut ::std::option::Option<super::Com::IAdviseSink>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(paspects), ::std::mem::transmute(padvf), ::std::mem::transmute(ppadvsink)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetExtent(&self, dwdrawaspect: u32, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE) -> ::windows::runtime::Result<super::super::Foundation::SIZE> {
        let mut result__: <super::super::Foundation::SIZE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdrawaspect), ::std::mem::transmute(lindex), ::std::mem::transmute(ptd), &mut result__).from_abi::<super::super::Foundation::SIZE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn GetRect(&self, dwaspect: u32) -> ::windows::runtime::Result<super::super::Foundation::RECTL> {
        let mut result__: <super::super::Foundation::RECTL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwaspect), &mut result__).from_abi::<super::super::Foundation::RECTL>(result__)
    }
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn GetViewStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn QueryHitPoint<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINT>>(&self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: Param2, lclosehint: i32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwaspect), ::std::mem::transmute(prectbounds), ptlloc.into_param().abi(), ::std::mem::transmute(lclosehint), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    pub unsafe fn QueryHitRect(&self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwaspect), ::std::mem::transmute(prectbounds), ::std::mem::transmute(prectloc), ::std::mem::transmute(lclosehint), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn GetNaturalExtent<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(&self, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: Param3, pextentinfo: *const ExtentInfo) -> ::windows::runtime::Result<super::super::Foundation::SIZE> {
        let mut result__: <super::super::Foundation::SIZE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwaspect), ::std::mem::transmute(lindex), ::std::mem::transmute(ptd), hictargetdev.into_param().abi(), ::std::mem::transmute(pextentinfo), &mut result__).from_abi::<super::super::Foundation::SIZE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IViewObjectEx {
    type Vtable = IViewObjectEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(988955282, 3222, 4558, [160, 207, 0, 170, 0, 96, 10, 184]);
}
impl ::std::convert::From<IViewObjectEx> for ::windows::runtime::IUnknown {
    fn from(value: IViewObjectEx) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IViewObjectEx> for ::windows::runtime::IUnknown {
    fn from(value: &IViewObjectEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IViewObjectEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IViewObjectEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IViewObjectEx> for IViewObject2 {
    fn from(value: IViewObjectEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IViewObjectEx> for IViewObject2 {
    fn from(value: &IViewObjectEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewObject2> for IViewObjectEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewObject2> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewObject2> for &IViewObjectEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewObject2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IViewObjectEx> for IViewObject {
    fn from(value: IViewObjectEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IViewObjectEx> for IViewObject {
    fn from(value: &IViewObjectEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewObject> for IViewObjectEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewObject> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewObject> for &IViewObjectEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewObject> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObjectEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: u32, lindex: i32, pvaspect: *mut ::std::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwfreeze: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aspects: u32, advf: u32, padvsink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: u32, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, lpsizel: *mut super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, prect: *mut super::super::Foundation::RECTL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: super::super::Foundation::POINT, lclosehint: i32, phitresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32, phitresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const ExtentInfo, psizel: *mut super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IZoomEvents(pub ::windows::runtime::IUnknown);
impl IZoomEvents {
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub unsafe fn OnZoomPercentChanged(&self, ulzoompercent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulzoompercent)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IZoomEvents {
    type Vtable = IZoomEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1102479696, 36940, 19991, [160, 186, 164, 56, 24, 46, 53, 157]);
}
impl ::std::convert::From<IZoomEvents> for ::windows::runtime::IUnknown {
    fn from(value: IZoomEvents) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IZoomEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IZoomEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IZoomEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IZoomEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoomEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulzoompercent: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn IsAccelerator<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::WindowsAndMessaging::HACCEL>>(haccel: Param0, caccelentries: i32, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG, lpwcmd: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsAccelerator(haccel: super::super::UI::WindowsAndMessaging::HACCEL, caccelentries: i32, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG, lpwcmd: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsAccelerator(haccel.into_param().abi(), ::std::mem::transmute(caccelentries), ::std::mem::transmute(lpmsg), ::std::mem::transmute(lpwcmd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
pub struct LICINFO {
    pub cbLicInfo: i32,
    pub fRuntimeKeyAvail: super::super::Foundation::BOOL,
    pub fLicVerified: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl LICINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LICINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LICINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LICINFO").field("cbLicInfo", &self.cbLicInfo).field("fRuntimeKeyAvail", &self.fRuntimeKeyAvail).field("fLicVerified", &self.fLicVerified).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LICINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbLicInfo == other.cbLicInfo && self.fRuntimeKeyAvail == other.fRuntimeKeyAvail && self.fLicVerified == other.fLicVerified
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LICINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LICINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNOLEUIHOOK = unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::WPARAM, param3: super::super::Foundation::LPARAM) -> u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const LP_COLOR: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const LP_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const LP_MONOCHROME: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const LP_VGACOLOR: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MEDIAPLAYBACK_STATE(pub i32);
pub const MEDIAPLAYBACK_RESUME: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(0i32);
pub const MEDIAPLAYBACK_PAUSE: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(1i32);
pub const MEDIAPLAYBACK_PAUSE_AND_SUSPEND: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(2i32);
pub const MEDIAPLAYBACK_RESUME_FROM_SUSPEND: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(3i32);
impl ::std::convert::From<i32> for MEDIAPLAYBACK_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MEDIAPLAYBACK_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const MK_ALT: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const MSOCMDERR_E_CANCELED: i32 = -2147221245i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const MSOCMDERR_E_DISABLED: i32 = -2147221247i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const MSOCMDERR_E_FIRST: i32 = -2147221248i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const MSOCMDERR_E_NOHELP: i32 = -2147221246i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const MSOCMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const MSOCMDERR_E_UNKNOWNGROUP: i32 = -2147221244i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MULTICLASSINFO_FLAGS(pub u32);
pub const MULTICLASSINFO_GETTYPEINFO: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(1u32);
pub const MULTICLASSINFO_GETNUMRESERVEDDISPIDS: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(2u32);
pub const MULTICLASSINFO_GETIIDPRIMARY: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(4u32);
pub const MULTICLASSINFO_GETIIDSOURCE: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(8u32);
impl ::std::convert::From<u32> for MULTICLASSINFO_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MULTICLASSINFO_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for MULTICLASSINFO_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MULTICLASSINFO_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MULTICLASSINFO_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MULTICLASSINFO_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MULTICLASSINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
pub struct OBJECTDESCRIPTOR {
    pub cbSize: u32,
    pub clsid: ::windows::runtime::GUID,
    pub dwDrawAspect: u32,
    pub sizel: super::super::Foundation::SIZE,
    pub pointl: super::super::Foundation::POINTL,
    pub dwStatus: u32,
    pub dwFullUserTypeName: u32,
    pub dwSrcOfCopy: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl OBJECTDESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OBJECTDESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OBJECTDESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OBJECTDESCRIPTOR")
            .field("cbSize", &self.cbSize)
            .field("clsid", &self.clsid)
            .field("dwDrawAspect", &self.dwDrawAspect)
            .field("sizel", &self.sizel)
            .field("pointl", &self.pointl)
            .field("dwStatus", &self.dwStatus)
            .field("dwFullUserTypeName", &self.dwFullUserTypeName)
            .field("dwSrcOfCopy", &self.dwSrcOfCopy)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OBJECTDESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.clsid == other.clsid && self.dwDrawAspect == other.dwDrawAspect && self.sizel == other.sizel && self.pointl == other.pointl && self.dwStatus == other.dwStatus && self.dwFullUserTypeName == other.dwFullUserTypeName && self.dwSrcOfCopy == other.dwSrcOfCopy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OBJECTDESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OBJECTDESCRIPTOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OCM__BASE: u32 = 8192u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
pub struct OCPFIPARAMS {
    pub cbStructSize: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub x: i32,
    pub y: i32,
    pub lpszCaption: super::super::Foundation::PWSTR,
    pub cObjects: u32,
    pub lplpUnk: *mut ::std::option::Option<::windows::runtime::IUnknown>,
    pub cPages: u32,
    pub lpPages: *mut ::windows::runtime::GUID,
    pub lcid: u32,
    pub dispidInitialProperty: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl OCPFIPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OCPFIPARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OCPFIPARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OCPFIPARAMS")
            .field("cbStructSize", &self.cbStructSize)
            .field("hWndOwner", &self.hWndOwner)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("lpszCaption", &self.lpszCaption)
            .field("cObjects", &self.cObjects)
            .field("lplpUnk", &self.lplpUnk)
            .field("cPages", &self.cPages)
            .field("lpPages", &self.lpPages)
            .field("lcid", &self.lcid)
            .field("dispidInitialProperty", &self.dispidInitialProperty)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OCPFIPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructSize == other.cbStructSize && self.hWndOwner == other.hWndOwner && self.x == other.x && self.y == other.y && self.lpszCaption == other.lpszCaption && self.cObjects == other.cObjects && self.lplpUnk == other.lplpUnk && self.cPages == other.cPages && self.lpPages == other.lpPages && self.lcid == other.lcid && self.dispidInitialProperty == other.dispidInitialProperty
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OCPFIPARAMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OCPFIPARAMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OF_GET: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OF_HANDLER: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OF_SET: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub struct OIFI {
    pub cb: u32,
    pub fMDIApp: super::super::Foundation::BOOL,
    pub hwndFrame: super::super::Foundation::HWND,
    pub haccel: super::super::UI::WindowsAndMessaging::HACCEL,
    pub cAccelEntries: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl OIFI {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for OIFI {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for OIFI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OIFI").field("cb", &self.cb).field("fMDIApp", &self.fMDIApp).field("hwndFrame", &self.hwndFrame).field("haccel", &self.haccel).field("cAccelEntries", &self.cAccelEntries).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for OIFI {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.fMDIApp == other.fMDIApp && self.hwndFrame == other.hwndFrame && self.haccel == other.haccel && self.cAccelEntries == other.cAccelEntries
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for OIFI {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for OIFI {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLECLOSE(pub i32);
pub const OLECLOSE_SAVEIFDIRTY: OLECLOSE = OLECLOSE(0i32);
pub const OLECLOSE_NOSAVE: OLECLOSE = OLECLOSE(1i32);
pub const OLECLOSE_PROMPTSAVE: OLECLOSE = OLECLOSE(2i32);
impl ::std::convert::From<i32> for OLECLOSE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLECLOSE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Ole`*"]
pub struct OLECMD {
    pub cmdID: u32,
    pub cmdf: u32,
}
impl OLECMD {}
impl ::std::default::Default for OLECMD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OLECMD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLECMD").field("cmdID", &self.cmdID).field("cmdf", &self.cmdf).finish()
    }
}
impl ::std::cmp::PartialEq for OLECMD {
    fn eq(&self, other: &Self) -> bool {
        self.cmdID == other.cmdID && self.cmdf == other.cmdf
    }
}
impl ::std::cmp::Eq for OLECMD {}
unsafe impl ::windows::runtime::Abi for OLECMD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_CLSID: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_DISPLAYNAME: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_INSTALLSCOPE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_PUBLISHER: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_SOURCEURL: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_HWND: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_X: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_Y: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDERR_E_CANCELED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147221245i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDERR_E_DISABLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147221247i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDERR_E_FIRST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147221248i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDERR_E_NOHELP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147221246i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMDERR_E_UNKNOWNGROUP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147221244i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLECMDEXECOPT(pub i32);
pub const OLECMDEXECOPT_DODEFAULT: OLECMDEXECOPT = OLECMDEXECOPT(0i32);
pub const OLECMDEXECOPT_PROMPTUSER: OLECMDEXECOPT = OLECMDEXECOPT(1i32);
pub const OLECMDEXECOPT_DONTPROMPTUSER: OLECMDEXECOPT = OLECMDEXECOPT(2i32);
pub const OLECMDEXECOPT_SHOWHELP: OLECMDEXECOPT = OLECMDEXECOPT(3i32);
impl ::std::convert::From<i32> for OLECMDEXECOPT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLECMDEXECOPT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLECMDF(pub i32);
pub const OLECMDF_SUPPORTED: OLECMDF = OLECMDF(1i32);
pub const OLECMDF_ENABLED: OLECMDF = OLECMDF(2i32);
pub const OLECMDF_LATCHED: OLECMDF = OLECMDF(4i32);
pub const OLECMDF_NINCHED: OLECMDF = OLECMDF(8i32);
pub const OLECMDF_INVISIBLE: OLECMDF = OLECMDF(16i32);
pub const OLECMDF_DEFHIDEONCTXTMENU: OLECMDF = OLECMDF(32i32);
impl ::std::convert::From<i32> for OLECMDF {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLECMDF {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLECMDID(pub i32);
pub const OLECMDID_OPEN: OLECMDID = OLECMDID(1i32);
pub const OLECMDID_NEW: OLECMDID = OLECMDID(2i32);
pub const OLECMDID_SAVE: OLECMDID = OLECMDID(3i32);
pub const OLECMDID_SAVEAS: OLECMDID = OLECMDID(4i32);
pub const OLECMDID_SAVECOPYAS: OLECMDID = OLECMDID(5i32);
pub const OLECMDID_PRINT: OLECMDID = OLECMDID(6i32);
pub const OLECMDID_PRINTPREVIEW: OLECMDID = OLECMDID(7i32);
pub const OLECMDID_PAGESETUP: OLECMDID = OLECMDID(8i32);
pub const OLECMDID_SPELL: OLECMDID = OLECMDID(9i32);
pub const OLECMDID_PROPERTIES: OLECMDID = OLECMDID(10i32);
pub const OLECMDID_CUT: OLECMDID = OLECMDID(11i32);
pub const OLECMDID_COPY: OLECMDID = OLECMDID(12i32);
pub const OLECMDID_PASTE: OLECMDID = OLECMDID(13i32);
pub const OLECMDID_PASTESPECIAL: OLECMDID = OLECMDID(14i32);
pub const OLECMDID_UNDO: OLECMDID = OLECMDID(15i32);
pub const OLECMDID_REDO: OLECMDID = OLECMDID(16i32);
pub const OLECMDID_SELECTALL: OLECMDID = OLECMDID(17i32);
pub const OLECMDID_CLEARSELECTION: OLECMDID = OLECMDID(18i32);
pub const OLECMDID_ZOOM: OLECMDID = OLECMDID(19i32);
pub const OLECMDID_GETZOOMRANGE: OLECMDID = OLECMDID(20i32);
pub const OLECMDID_UPDATECOMMANDS: OLECMDID = OLECMDID(21i32);
pub const OLECMDID_REFRESH: OLECMDID = OLECMDID(22i32);
pub const OLECMDID_STOP: OLECMDID = OLECMDID(23i32);
pub const OLECMDID_HIDETOOLBARS: OLECMDID = OLECMDID(24i32);
pub const OLECMDID_SETPROGRESSMAX: OLECMDID = OLECMDID(25i32);
pub const OLECMDID_SETPROGRESSPOS: OLECMDID = OLECMDID(26i32);
pub const OLECMDID_SETPROGRESSTEXT: OLECMDID = OLECMDID(27i32);
pub const OLECMDID_SETTITLE: OLECMDID = OLECMDID(28i32);
pub const OLECMDID_SETDOWNLOADSTATE: OLECMDID = OLECMDID(29i32);
pub const OLECMDID_STOPDOWNLOAD: OLECMDID = OLECMDID(30i32);
pub const OLECMDID_ONTOOLBARACTIVATED: OLECMDID = OLECMDID(31i32);
pub const OLECMDID_FIND: OLECMDID = OLECMDID(32i32);
pub const OLECMDID_DELETE: OLECMDID = OLECMDID(33i32);
pub const OLECMDID_HTTPEQUIV: OLECMDID = OLECMDID(34i32);
pub const OLECMDID_HTTPEQUIV_DONE: OLECMDID = OLECMDID(35i32);
pub const OLECMDID_ENABLE_INTERACTION: OLECMDID = OLECMDID(36i32);
pub const OLECMDID_ONUNLOAD: OLECMDID = OLECMDID(37i32);
pub const OLECMDID_PROPERTYBAG2: OLECMDID = OLECMDID(38i32);
pub const OLECMDID_PREREFRESH: OLECMDID = OLECMDID(39i32);
pub const OLECMDID_SHOWSCRIPTERROR: OLECMDID = OLECMDID(40i32);
pub const OLECMDID_SHOWMESSAGE: OLECMDID = OLECMDID(41i32);
pub const OLECMDID_SHOWFIND: OLECMDID = OLECMDID(42i32);
pub const OLECMDID_SHOWPAGESETUP: OLECMDID = OLECMDID(43i32);
pub const OLECMDID_SHOWPRINT: OLECMDID = OLECMDID(44i32);
pub const OLECMDID_CLOSE: OLECMDID = OLECMDID(45i32);
pub const OLECMDID_ALLOWUILESSSAVEAS: OLECMDID = OLECMDID(46i32);
pub const OLECMDID_DONTDOWNLOADCSS: OLECMDID = OLECMDID(47i32);
pub const OLECMDID_UPDATEPAGESTATUS: OLECMDID = OLECMDID(48i32);
pub const OLECMDID_PRINT2: OLECMDID = OLECMDID(49i32);
pub const OLECMDID_PRINTPREVIEW2: OLECMDID = OLECMDID(50i32);
pub const OLECMDID_SETPRINTTEMPLATE: OLECMDID = OLECMDID(51i32);
pub const OLECMDID_GETPRINTTEMPLATE: OLECMDID = OLECMDID(52i32);
pub const OLECMDID_PAGEACTIONBLOCKED: OLECMDID = OLECMDID(55i32);
pub const OLECMDID_PAGEACTIONUIQUERY: OLECMDID = OLECMDID(56i32);
pub const OLECMDID_FOCUSVIEWCONTROLS: OLECMDID = OLECMDID(57i32);
pub const OLECMDID_FOCUSVIEWCONTROLSQUERY: OLECMDID = OLECMDID(58i32);
pub const OLECMDID_SHOWPAGEACTIONMENU: OLECMDID = OLECMDID(59i32);
pub const OLECMDID_ADDTRAVELENTRY: OLECMDID = OLECMDID(60i32);
pub const OLECMDID_UPDATETRAVELENTRY: OLECMDID = OLECMDID(61i32);
pub const OLECMDID_UPDATEBACKFORWARDSTATE: OLECMDID = OLECMDID(62i32);
pub const OLECMDID_OPTICAL_ZOOM: OLECMDID = OLECMDID(63i32);
pub const OLECMDID_OPTICAL_GETZOOMRANGE: OLECMDID = OLECMDID(64i32);
pub const OLECMDID_WINDOWSTATECHANGED: OLECMDID = OLECMDID(65i32);
pub const OLECMDID_ACTIVEXINSTALLSCOPE: OLECMDID = OLECMDID(66i32);
pub const OLECMDID_UPDATETRAVELENTRY_DATARECOVERY: OLECMDID = OLECMDID(67i32);
pub const OLECMDID_SHOWTASKDLG: OLECMDID = OLECMDID(68i32);
pub const OLECMDID_POPSTATEEVENT: OLECMDID = OLECMDID(69i32);
pub const OLECMDID_VIEWPORT_MODE: OLECMDID = OLECMDID(70i32);
pub const OLECMDID_LAYOUT_VIEWPORT_WIDTH: OLECMDID = OLECMDID(71i32);
pub const OLECMDID_VISUAL_VIEWPORT_EXCLUDE_BOTTOM: OLECMDID = OLECMDID(72i32);
pub const OLECMDID_USER_OPTICAL_ZOOM: OLECMDID = OLECMDID(73i32);
pub const OLECMDID_PAGEAVAILABLE: OLECMDID = OLECMDID(74i32);
pub const OLECMDID_GETUSERSCALABLE: OLECMDID = OLECMDID(75i32);
pub const OLECMDID_UPDATE_CARET: OLECMDID = OLECMDID(76i32);
pub const OLECMDID_ENABLE_VISIBILITY: OLECMDID = OLECMDID(77i32);
pub const OLECMDID_MEDIA_PLAYBACK: OLECMDID = OLECMDID(78i32);
pub const OLECMDID_SETFAVICON: OLECMDID = OLECMDID(79i32);
pub const OLECMDID_SET_HOST_FULLSCREENMODE: OLECMDID = OLECMDID(80i32);
pub const OLECMDID_EXITFULLSCREEN: OLECMDID = OLECMDID(81i32);
pub const OLECMDID_SCROLLCOMPLETE: OLECMDID = OLECMDID(82i32);
pub const OLECMDID_ONBEFOREUNLOAD: OLECMDID = OLECMDID(83i32);
pub const OLECMDID_SHOWMESSAGE_BLOCKABLE: OLECMDID = OLECMDID(84i32);
pub const OLECMDID_SHOWTASKDLG_BLOCKABLE: OLECMDID = OLECMDID(85i32);
impl ::std::convert::From<i32> for OLECMDID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLECMDID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLECMDID_BROWSERSTATEFLAG(pub i32);
pub const OLECMDIDF_BROWSERSTATE_EXTENSIONSOFF: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(1i32);
pub const OLECMDIDF_BROWSERSTATE_IESECURITY: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(2i32);
pub const OLECMDIDF_BROWSERSTATE_PROTECTEDMODE_OFF: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(4i32);
pub const OLECMDIDF_BROWSERSTATE_RESET: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(8i32);
pub const OLECMDIDF_BROWSERSTATE_REQUIRESACTIVEX: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(16i32);
pub const OLECMDIDF_BROWSERSTATE_DESKTOPHTMLDIALOG: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(32i32);
pub const OLECMDIDF_BROWSERSTATE_BLOCKEDVERSION: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(64i32);
impl ::std::convert::From<i32> for OLECMDID_BROWSERSTATEFLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLECMDID_BROWSERSTATEFLAG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLECMDID_OPTICAL_ZOOMFLAG(pub i32);
pub const OLECMDIDF_OPTICAL_ZOOM_NOPERSIST: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(1i32);
pub const OLECMDIDF_OPTICAL_ZOOM_NOLAYOUT: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(16i32);
pub const OLECMDIDF_OPTICAL_ZOOM_NOTRANSIENT: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(32i32);
pub const OLECMDIDF_OPTICAL_ZOOM_RELOADFORNEWTAB: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(64i32);
impl ::std::convert::From<i32> for OLECMDID_OPTICAL_ZOOMFLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLECMDID_OPTICAL_ZOOMFLAG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLECMDID_PAGEACTIONFLAG(pub i32);
pub const OLECMDIDF_PAGEACTION_FILEDOWNLOAD: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(2i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXTRUSTFAIL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(4i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERDISABLE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(8i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXDISALLOW: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(16i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXUNSAFE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(32i32);
pub const OLECMDIDF_PAGEACTION_POPUPWINDOW: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(64i32);
pub const OLECMDIDF_PAGEACTION_LOCALMACHINE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(128i32);
pub const OLECMDIDF_PAGEACTION_MIMETEXTPLAIN: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(256i32);
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(512i32);
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(512i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1024i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(2048i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(4096i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(8192i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(16384i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNDENY: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(32768i32);
pub const OLECMDIDF_PAGEACTION_POPUPALLOWED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(65536i32);
pub const OLECMDIDF_PAGEACTION_SCRIPTPROMPT: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(131072i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(262144i32);
pub const OLECMDIDF_PAGEACTION_MIXEDCONTENT: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(524288i32);
pub const OLECMDIDF_PAGEACTION_INVALID_CERT: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1048576i32);
pub const OLECMDIDF_PAGEACTION_INTRANETZONEREQUEST: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(2097152i32);
pub const OLECMDIDF_PAGEACTION_XSSFILTERED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(4194304i32);
pub const OLECMDIDF_PAGEACTION_SPOOFABLEIDNHOST: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(8388608i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEX_EPM_INCOMPATIBLE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(16777216i32);
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(33554432i32);
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(67108864i32);
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED_ACTIVEX: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(134217728i32);
pub const OLECMDIDF_PAGEACTION_EXTENSION_COMPAT_BLOCKED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(268435456i32);
pub const OLECMDIDF_PAGEACTION_NORESETACTIVEX: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(536870912i32);
pub const OLECMDIDF_PAGEACTION_GENERIC_STATE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1073741824i32);
pub const OLECMDIDF_PAGEACTION_RESET: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(-2147483648i32);
impl ::std::convert::From<i32> for OLECMDID_PAGEACTIONFLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLECMDID_PAGEACTIONFLAG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLECMDID_REFRESHFLAG(pub i32);
pub const OLECMDIDF_REFRESH_NORMAL: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(0i32);
pub const OLECMDIDF_REFRESH_IFEXPIRED: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(1i32);
pub const OLECMDIDF_REFRESH_CONTINUE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(2i32);
pub const OLECMDIDF_REFRESH_COMPLETELY: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(3i32);
pub const OLECMDIDF_REFRESH_NO_CACHE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(4i32);
pub const OLECMDIDF_REFRESH_RELOAD: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(5i32);
pub const OLECMDIDF_REFRESH_LEVELMASK: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(255i32);
pub const OLECMDIDF_REFRESH_CLEARUSERINPUT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(4096i32);
pub const OLECMDIDF_REFRESH_PROMPTIFOFFLINE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(8192i32);
pub const OLECMDIDF_REFRESH_THROUGHSCRIPT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(16384i32);
pub const OLECMDIDF_REFRESH_SKIPBEFOREUNLOADEVENT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(32768i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_ACTIVEXINSTALL: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(65536i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_FILEDOWNLOAD: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(131072i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_LOCALMACHINE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(262144i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_POPUPWINDOW: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(524288i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(1048576i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(2097152i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(4194304i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(8388608i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(16777216i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_MIXEDCONTENT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(33554432i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_INVALID_CERT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(67108864i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_ALLOW_VERSION: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(134217728i32);
impl ::std::convert::From<i32> for OLECMDID_REFRESHFLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLECMDID_REFRESHFLAG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLECMDID_VIEWPORT_MODE_FLAG(pub i32);
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(1i32);
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(2i32);
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH_VALID: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(65536i32);
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM_VALID: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(131072i32);
impl ::std::convert::From<i32> for OLECMDID_VIEWPORT_MODE_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLECMDID_VIEWPORT_MODE_FLAG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLECMDID_WINDOWSTATE_FLAG(pub i32);
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(1i32);
pub const OLECMDIDF_WINDOWSTATE_ENABLED: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(2i32);
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE_VALID: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(65536i32);
pub const OLECMDIDF_WINDOWSTATE_ENABLED_VALID: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(131072i32);
impl ::std::convert::From<i32> for OLECMDID_WINDOWSTATE_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLECMDID_WINDOWSTATE_FLAG {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Ole`*"]
pub struct OLECMDTEXT {
    pub cmdtextf: u32,
    pub cwActual: u32,
    pub cwBuf: u32,
    pub rgwz: [u16; 1],
}
impl OLECMDTEXT {}
impl ::std::default::Default for OLECMDTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OLECMDTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLECMDTEXT").field("cmdtextf", &self.cmdtextf).field("cwActual", &self.cwActual).field("cwBuf", &self.cwBuf).field("rgwz", &self.rgwz).finish()
    }
}
impl ::std::cmp::PartialEq for OLECMDTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cmdtextf == other.cmdtextf && self.cwActual == other.cwActual && self.cwBuf == other.cwBuf && self.rgwz == other.rgwz
    }
}
impl ::std::cmp::Eq for OLECMDTEXT {}
unsafe impl ::windows::runtime::Abi for OLECMDTEXT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLECMDTEXTF(pub i32);
pub const OLECMDTEXTF_NONE: OLECMDTEXTF = OLECMDTEXTF(0i32);
pub const OLECMDTEXTF_NAME: OLECMDTEXTF = OLECMDTEXTF(1i32);
pub const OLECMDTEXTF_STATUS: OLECMDTEXTF = OLECMDTEXTF(2i32);
impl ::std::convert::From<i32> for OLECMDTEXTF {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLECMDTEXTF {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECMD_TASKDLGID_ONBEFOREUNLOAD: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLECONTF(pub i32);
pub const OLECONTF_EMBEDDINGS: OLECONTF = OLECONTF(1i32);
pub const OLECONTF_LINKS: OLECONTF = OLECONTF(2i32);
pub const OLECONTF_OTHERS: OLECONTF = OLECONTF(4i32);
pub const OLECONTF_ONLYUSER: OLECONTF = OLECONTF(8i32);
pub const OLECONTF_ONLYIFRUNNING: OLECONTF = OLECONTF(16i32);
impl ::std::convert::From<i32> for OLECONTF {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLECONTF {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLECREATE_LEAVERUNNING: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLEDCFLAGS(pub i32);
pub const OLEDC_NODRAW: OLEDCFLAGS = OLEDCFLAGS(1i32);
pub const OLEDC_PAINTBKGND: OLEDCFLAGS = OLEDCFLAGS(2i32);
pub const OLEDC_OFFSCREEN: OLEDCFLAGS = OLEDCFLAGS(4i32);
impl ::std::convert::From<i32> for OLEDCFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLEDCFLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLEGETMONIKER(pub i32);
pub const OLEGETMONIKER_ONLYIFTHERE: OLEGETMONIKER = OLEGETMONIKER(1i32);
pub const OLEGETMONIKER_FORCEASSIGN: OLEGETMONIKER = OLEGETMONIKER(2i32);
pub const OLEGETMONIKER_UNASSIGN: OLEGETMONIKER = OLEGETMONIKER(3i32);
pub const OLEGETMONIKER_TEMPFORUSER: OLEGETMONIKER = OLEGETMONIKER(4i32);
impl ::std::convert::From<i32> for OLEGETMONIKER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLEGETMONIKER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEIVERB_DISCARDUNDOSTATE: i32 = -6i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEIVERB_HIDE: i32 = -3i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEIVERB_INPLACEACTIVATE: i32 = -5i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEIVERB_OPEN: i32 = -2i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEIVERB_PRIMARY: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEIVERB_PROPERTIES: i32 = -7i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEIVERB_SHOW: i32 = -1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEIVERB_UIACTIVATE: i32 = -4i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLELINKBIND(pub i32);
pub const OLELINKBIND_EVENIFCLASSDIFF: OLELINKBIND = OLELINKBIND(1i32);
impl ::std::convert::From<i32> for OLELINKBIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLELINKBIND {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLEMISC(pub i32);
pub const OLEMISC_RECOMPOSEONRESIZE: OLEMISC = OLEMISC(1i32);
pub const OLEMISC_ONLYICONIC: OLEMISC = OLEMISC(2i32);
pub const OLEMISC_INSERTNOTREPLACE: OLEMISC = OLEMISC(4i32);
pub const OLEMISC_STATIC: OLEMISC = OLEMISC(8i32);
pub const OLEMISC_CANTLINKINSIDE: OLEMISC = OLEMISC(16i32);
pub const OLEMISC_CANLINKBYOLE1: OLEMISC = OLEMISC(32i32);
pub const OLEMISC_ISLINKOBJECT: OLEMISC = OLEMISC(64i32);
pub const OLEMISC_INSIDEOUT: OLEMISC = OLEMISC(128i32);
pub const OLEMISC_ACTIVATEWHENVISIBLE: OLEMISC = OLEMISC(256i32);
pub const OLEMISC_RENDERINGISDEVICEINDEPENDENT: OLEMISC = OLEMISC(512i32);
pub const OLEMISC_INVISIBLEATRUNTIME: OLEMISC = OLEMISC(1024i32);
pub const OLEMISC_ALWAYSRUN: OLEMISC = OLEMISC(2048i32);
pub const OLEMISC_ACTSLIKEBUTTON: OLEMISC = OLEMISC(4096i32);
pub const OLEMISC_ACTSLIKELABEL: OLEMISC = OLEMISC(8192i32);
pub const OLEMISC_NOUIACTIVATE: OLEMISC = OLEMISC(16384i32);
pub const OLEMISC_ALIGNABLE: OLEMISC = OLEMISC(32768i32);
pub const OLEMISC_SIMPLEFRAME: OLEMISC = OLEMISC(65536i32);
pub const OLEMISC_SETCLIENTSITEFIRST: OLEMISC = OLEMISC(131072i32);
pub const OLEMISC_IMEMODE: OLEMISC = OLEMISC(262144i32);
pub const OLEMISC_IGNOREACTIVATEWHENVISIBLE: OLEMISC = OLEMISC(524288i32);
pub const OLEMISC_WANTSTOMENUMERGE: OLEMISC = OLEMISC(1048576i32);
pub const OLEMISC_SUPPORTSMULTILEVELUNDO: OLEMISC = OLEMISC(2097152i32);
impl ::std::convert::From<i32> for OLEMISC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLEMISC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLERENDER(pub i32);
pub const OLERENDER_NONE: OLERENDER = OLERENDER(0i32);
pub const OLERENDER_DRAW: OLERENDER = OLERENDER(1i32);
pub const OLERENDER_FORMAT: OLERENDER = OLERENDER(2i32);
pub const OLERENDER_ASIS: OLERENDER = OLERENDER(3i32);
impl ::std::convert::From<i32> for OLERENDER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLERENDER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Media_Audio_CoreAudio`, `Win32_System_LibraryLoader`*"]
pub struct OLEUIBUSYA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub hTask: super::super::Media::Audio::CoreAudio::HTASK,
    pub lphWndDialog: *mut super::super::Foundation::HWND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
impl OLEUIBUSYA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
impl ::std::default::Default for OLEUIBUSYA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
impl ::std::fmt::Debug for OLEUIBUSYA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIBUSYA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("hTask", &self.hTask)
            .field("lphWndDialog", &self.lphWndDialog)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::PartialEq for OLEUIBUSYA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.hWndOwner == other.hWndOwner && self.lpszCaption == other.lpszCaption && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lCustData == other.lCustData && self.hInstance == other.hInstance && self.lpszTemplate == other.lpszTemplate && self.hResource == other.hResource && self.hTask == other.hTask && self.lphWndDialog == other.lphWndDialog
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::Eq for OLEUIBUSYA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
unsafe impl ::windows::runtime::Abi for OLEUIBUSYA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Media_Audio_CoreAudio`, `Win32_System_LibraryLoader`*"]
pub struct OLEUIBUSYW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PWSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PWSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub hTask: super::super::Media::Audio::CoreAudio::HTASK,
    pub lphWndDialog: *mut super::super::Foundation::HWND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
impl OLEUIBUSYW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
impl ::std::default::Default for OLEUIBUSYW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
impl ::std::fmt::Debug for OLEUIBUSYW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIBUSYW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("hTask", &self.hTask)
            .field("lphWndDialog", &self.lphWndDialog)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::PartialEq for OLEUIBUSYW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.hWndOwner == other.hWndOwner && self.lpszCaption == other.lpszCaption && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lCustData == other.lCustData && self.hInstance == other.hInstance && self.lpszTemplate == other.lpszTemplate && self.hResource == other.hResource && self.hTask == other.hTask && self.lphWndDialog == other.lphWndDialog
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::Eq for OLEUIBUSYW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
unsafe impl ::windows::runtime::Abi for OLEUIBUSYW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`*"]
pub struct OLEUICHANGEICONA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub hMetaPict: isize,
    pub clsid: ::windows::runtime::GUID,
    pub szIconExe: [super::super::Foundation::CHAR; 260],
    pub cchIconExe: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl OLEUICHANGEICONA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::default::Default for OLEUICHANGEICONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::fmt::Debug for OLEUICHANGEICONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUICHANGEICONA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("hMetaPict", &self.hMetaPict)
            .field("clsid", &self.clsid)
            .field("szIconExe", &self.szIconExe)
            .field("cchIconExe", &self.cchIconExe)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::PartialEq for OLEUICHANGEICONA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFlags == other.dwFlags
            && self.hWndOwner == other.hWndOwner
            && self.lpszCaption == other.lpszCaption
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lCustData == other.lCustData
            && self.hInstance == other.hInstance
            && self.lpszTemplate == other.lpszTemplate
            && self.hResource == other.hResource
            && self.hMetaPict == other.hMetaPict
            && self.clsid == other.clsid
            && self.szIconExe == other.szIconExe
            && self.cchIconExe == other.cchIconExe
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::Eq for OLEUICHANGEICONA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
unsafe impl ::windows::runtime::Abi for OLEUICHANGEICONA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`*"]
pub struct OLEUICHANGEICONW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PWSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PWSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub hMetaPict: isize,
    pub clsid: ::windows::runtime::GUID,
    pub szIconExe: [u16; 260],
    pub cchIconExe: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl OLEUICHANGEICONW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::default::Default for OLEUICHANGEICONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::fmt::Debug for OLEUICHANGEICONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUICHANGEICONW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("hMetaPict", &self.hMetaPict)
            .field("clsid", &self.clsid)
            .field("szIconExe", &self.szIconExe)
            .field("cchIconExe", &self.cchIconExe)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::PartialEq for OLEUICHANGEICONW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFlags == other.dwFlags
            && self.hWndOwner == other.hWndOwner
            && self.lpszCaption == other.lpszCaption
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lCustData == other.lCustData
            && self.hInstance == other.hInstance
            && self.lpszTemplate == other.lpszTemplate
            && self.hResource == other.hResource
            && self.hMetaPict == other.hMetaPict
            && self.clsid == other.clsid
            && self.szIconExe == other.szIconExe
            && self.cchIconExe == other.cchIconExe
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::Eq for OLEUICHANGEICONW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
unsafe impl ::windows::runtime::Abi for OLEUICHANGEICONW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`, `Win32_UI_Controls_Dialogs`*"]
pub struct OLEUICHANGESOURCEA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub lpOFN: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEA,
    pub dwReserved1: [u32; 4],
    pub lpOleUILinkContainer: ::std::option::Option<IOleUILinkContainerA>,
    pub dwLink: u32,
    pub lpszDisplayName: super::super::Foundation::PSTR,
    pub nFileLength: u32,
    pub lpszFrom: super::super::Foundation::PSTR,
    pub lpszTo: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
impl OLEUICHANGESOURCEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
impl ::std::default::Default for OLEUICHANGESOURCEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
impl ::std::fmt::Debug for OLEUICHANGESOURCEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUICHANGESOURCEA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("lpOFN", &self.lpOFN)
            .field("dwReserved1", &self.dwReserved1)
            .field("lpOleUILinkContainer", &self.lpOleUILinkContainer)
            .field("dwLink", &self.dwLink)
            .field("lpszDisplayName", &self.lpszDisplayName)
            .field("nFileLength", &self.nFileLength)
            .field("lpszFrom", &self.lpszFrom)
            .field("lpszTo", &self.lpszTo)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
impl ::std::cmp::PartialEq for OLEUICHANGESOURCEA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFlags == other.dwFlags
            && self.hWndOwner == other.hWndOwner
            && self.lpszCaption == other.lpszCaption
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lCustData == other.lCustData
            && self.hInstance == other.hInstance
            && self.lpszTemplate == other.lpszTemplate
            && self.hResource == other.hResource
            && self.lpOFN == other.lpOFN
            && self.dwReserved1 == other.dwReserved1
            && self.lpOleUILinkContainer == other.lpOleUILinkContainer
            && self.dwLink == other.dwLink
            && self.lpszDisplayName == other.lpszDisplayName
            && self.nFileLength == other.nFileLength
            && self.lpszFrom == other.lpszFrom
            && self.lpszTo == other.lpszTo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
impl ::std::cmp::Eq for OLEUICHANGESOURCEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
unsafe impl ::windows::runtime::Abi for OLEUICHANGESOURCEA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`, `Win32_UI_Controls_Dialogs`*"]
pub struct OLEUICHANGESOURCEW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PWSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PWSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub lpOFN: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEW,
    pub dwReserved1: [u32; 4],
    pub lpOleUILinkContainer: ::std::option::Option<IOleUILinkContainerW>,
    pub dwLink: u32,
    pub lpszDisplayName: super::super::Foundation::PWSTR,
    pub nFileLength: u32,
    pub lpszFrom: super::super::Foundation::PWSTR,
    pub lpszTo: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
impl OLEUICHANGESOURCEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
impl ::std::default::Default for OLEUICHANGESOURCEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
impl ::std::fmt::Debug for OLEUICHANGESOURCEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUICHANGESOURCEW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("lpOFN", &self.lpOFN)
            .field("dwReserved1", &self.dwReserved1)
            .field("lpOleUILinkContainer", &self.lpOleUILinkContainer)
            .field("dwLink", &self.dwLink)
            .field("lpszDisplayName", &self.lpszDisplayName)
            .field("nFileLength", &self.nFileLength)
            .field("lpszFrom", &self.lpszFrom)
            .field("lpszTo", &self.lpszTo)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
impl ::std::cmp::PartialEq for OLEUICHANGESOURCEW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFlags == other.dwFlags
            && self.hWndOwner == other.hWndOwner
            && self.lpszCaption == other.lpszCaption
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lCustData == other.lCustData
            && self.hInstance == other.hInstance
            && self.lpszTemplate == other.lpszTemplate
            && self.hResource == other.hResource
            && self.lpOFN == other.lpOFN
            && self.dwReserved1 == other.dwReserved1
            && self.lpOleUILinkContainer == other.lpOleUILinkContainer
            && self.dwLink == other.dwLink
            && self.lpszDisplayName == other.lpszDisplayName
            && self.nFileLength == other.nFileLength
            && self.lpszFrom == other.lpszFrom
            && self.lpszTo == other.lpszTo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
impl ::std::cmp::Eq for OLEUICHANGESOURCEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
unsafe impl ::windows::runtime::Abi for OLEUICHANGESOURCEW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`*"]
pub struct OLEUICONVERTA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub clsid: ::windows::runtime::GUID,
    pub clsidConvertDefault: ::windows::runtime::GUID,
    pub clsidActivateDefault: ::windows::runtime::GUID,
    pub clsidNew: ::windows::runtime::GUID,
    pub dvAspect: u32,
    pub wFormat: u16,
    pub fIsLinkedObject: super::super::Foundation::BOOL,
    pub hMetaPict: isize,
    pub lpszUserType: super::super::Foundation::PSTR,
    pub fObjectsIconChanged: super::super::Foundation::BOOL,
    pub lpszDefLabel: super::super::Foundation::PSTR,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows::runtime::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl OLEUICONVERTA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::default::Default for OLEUICONVERTA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::fmt::Debug for OLEUICONVERTA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUICONVERTA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("clsid", &self.clsid)
            .field("clsidConvertDefault", &self.clsidConvertDefault)
            .field("clsidActivateDefault", &self.clsidActivateDefault)
            .field("clsidNew", &self.clsidNew)
            .field("dvAspect", &self.dvAspect)
            .field("wFormat", &self.wFormat)
            .field("fIsLinkedObject", &self.fIsLinkedObject)
            .field("hMetaPict", &self.hMetaPict)
            .field("lpszUserType", &self.lpszUserType)
            .field("fObjectsIconChanged", &self.fObjectsIconChanged)
            .field("lpszDefLabel", &self.lpszDefLabel)
            .field("cClsidExclude", &self.cClsidExclude)
            .field("lpClsidExclude", &self.lpClsidExclude)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::PartialEq for OLEUICONVERTA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFlags == other.dwFlags
            && self.hWndOwner == other.hWndOwner
            && self.lpszCaption == other.lpszCaption
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lCustData == other.lCustData
            && self.hInstance == other.hInstance
            && self.lpszTemplate == other.lpszTemplate
            && self.hResource == other.hResource
            && self.clsid == other.clsid
            && self.clsidConvertDefault == other.clsidConvertDefault
            && self.clsidActivateDefault == other.clsidActivateDefault
            && self.clsidNew == other.clsidNew
            && self.dvAspect == other.dvAspect
            && self.wFormat == other.wFormat
            && self.fIsLinkedObject == other.fIsLinkedObject
            && self.hMetaPict == other.hMetaPict
            && self.lpszUserType == other.lpszUserType
            && self.fObjectsIconChanged == other.fObjectsIconChanged
            && self.lpszDefLabel == other.lpszDefLabel
            && self.cClsidExclude == other.cClsidExclude
            && self.lpClsidExclude == other.lpClsidExclude
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::Eq for OLEUICONVERTA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
unsafe impl ::windows::runtime::Abi for OLEUICONVERTA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`*"]
pub struct OLEUICONVERTW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PWSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PWSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub clsid: ::windows::runtime::GUID,
    pub clsidConvertDefault: ::windows::runtime::GUID,
    pub clsidActivateDefault: ::windows::runtime::GUID,
    pub clsidNew: ::windows::runtime::GUID,
    pub dvAspect: u32,
    pub wFormat: u16,
    pub fIsLinkedObject: super::super::Foundation::BOOL,
    pub hMetaPict: isize,
    pub lpszUserType: super::super::Foundation::PWSTR,
    pub fObjectsIconChanged: super::super::Foundation::BOOL,
    pub lpszDefLabel: super::super::Foundation::PWSTR,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows::runtime::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl OLEUICONVERTW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::default::Default for OLEUICONVERTW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::fmt::Debug for OLEUICONVERTW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUICONVERTW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("clsid", &self.clsid)
            .field("clsidConvertDefault", &self.clsidConvertDefault)
            .field("clsidActivateDefault", &self.clsidActivateDefault)
            .field("clsidNew", &self.clsidNew)
            .field("dvAspect", &self.dvAspect)
            .field("wFormat", &self.wFormat)
            .field("fIsLinkedObject", &self.fIsLinkedObject)
            .field("hMetaPict", &self.hMetaPict)
            .field("lpszUserType", &self.lpszUserType)
            .field("fObjectsIconChanged", &self.fObjectsIconChanged)
            .field("lpszDefLabel", &self.lpszDefLabel)
            .field("cClsidExclude", &self.cClsidExclude)
            .field("lpClsidExclude", &self.lpClsidExclude)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::PartialEq for OLEUICONVERTW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFlags == other.dwFlags
            && self.hWndOwner == other.hWndOwner
            && self.lpszCaption == other.lpszCaption
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lCustData == other.lCustData
            && self.hInstance == other.hInstance
            && self.lpszTemplate == other.lpszTemplate
            && self.hResource == other.hResource
            && self.clsid == other.clsid
            && self.clsidConvertDefault == other.clsidConvertDefault
            && self.clsidActivateDefault == other.clsidActivateDefault
            && self.clsidNew == other.clsidNew
            && self.dvAspect == other.dvAspect
            && self.wFormat == other.wFormat
            && self.fIsLinkedObject == other.fIsLinkedObject
            && self.hMetaPict == other.hMetaPict
            && self.lpszUserType == other.lpszUserType
            && self.fObjectsIconChanged == other.fObjectsIconChanged
            && self.lpszDefLabel == other.lpszDefLabel
            && self.cClsidExclude == other.cClsidExclude
            && self.lpClsidExclude == other.lpClsidExclude
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::Eq for OLEUICONVERTW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
unsafe impl ::windows::runtime::Abi for OLEUICONVERTW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`*"]
pub struct OLEUIEDITLINKSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub lpOleUILinkContainer: ::std::option::Option<IOleUILinkContainerA>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl OLEUIEDITLINKSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::default::Default for OLEUIEDITLINKSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::fmt::Debug for OLEUIEDITLINKSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIEDITLINKSA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("lpOleUILinkContainer", &self.lpOleUILinkContainer)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::PartialEq for OLEUIEDITLINKSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.hWndOwner == other.hWndOwner && self.lpszCaption == other.lpszCaption && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lCustData == other.lCustData && self.hInstance == other.hInstance && self.lpszTemplate == other.lpszTemplate && self.hResource == other.hResource && self.lpOleUILinkContainer == other.lpOleUILinkContainer
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::Eq for OLEUIEDITLINKSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
unsafe impl ::windows::runtime::Abi for OLEUIEDITLINKSA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`*"]
pub struct OLEUIEDITLINKSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PWSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PWSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub lpOleUILinkContainer: ::std::option::Option<IOleUILinkContainerW>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl OLEUIEDITLINKSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::default::Default for OLEUIEDITLINKSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::fmt::Debug for OLEUIEDITLINKSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIEDITLINKSW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("lpOleUILinkContainer", &self.lpOleUILinkContainer)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::PartialEq for OLEUIEDITLINKSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.hWndOwner == other.hWndOwner && self.lpszCaption == other.lpszCaption && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lCustData == other.lCustData && self.hInstance == other.hInstance && self.lpszTemplate == other.lpszTemplate && self.hResource == other.hResource && self.lpOleUILinkContainer == other.lpOleUILinkContainer
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::Eq for OLEUIEDITLINKSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
unsafe impl ::windows::runtime::Abi for OLEUIEDITLINKSW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
pub struct OLEUIGNRLPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl OLEUIGNRLPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for OLEUIGNRLPROPSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for OLEUIGNRLPROPSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIGNRLPROPSA").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("lCustData", &self.lCustData).field("dwReserved2", &self.dwReserved2).field("lpOP", &self.lpOP).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for OLEUIGNRLPROPSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1 && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lCustData == other.lCustData && self.dwReserved2 == other.dwReserved2 && self.lpOP == other.lpOP
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for OLEUIGNRLPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for OLEUIGNRLPROPSA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
pub struct OLEUIGNRLPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl OLEUIGNRLPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for OLEUIGNRLPROPSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for OLEUIGNRLPROPSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIGNRLPROPSW").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("lCustData", &self.lCustData).field("dwReserved2", &self.dwReserved2).field("lpOP", &self.lpOP).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for OLEUIGNRLPROPSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1 && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lCustData == other.lCustData && self.dwReserved2 == other.dwReserved2 && self.lpOP == other.lpOP
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for OLEUIGNRLPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for OLEUIGNRLPROPSW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_LibraryLoader`*"]
pub struct OLEUIINSERTOBJECTA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub clsid: ::windows::runtime::GUID,
    pub lpszFile: super::super::Foundation::PSTR,
    pub cchFile: u32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows::runtime::GUID,
    pub iid: ::windows::runtime::GUID,
    pub oleRender: u32,
    pub lpFormatEtc: *mut super::Com::FORMATETC,
    pub lpIOleClientSite: ::std::option::Option<IOleClientSite>,
    pub lpIStorage: ::std::option::Option<super::Com::StructuredStorage::IStorage>,
    pub ppvObj: *mut *mut ::std::ffi::c_void,
    pub sc: i32,
    pub hMetaPict: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
impl OLEUIINSERTOBJECTA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
impl ::std::default::Default for OLEUIINSERTOBJECTA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
impl ::std::fmt::Debug for OLEUIINSERTOBJECTA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIINSERTOBJECTA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("clsid", &self.clsid)
            .field("lpszFile", &self.lpszFile)
            .field("cchFile", &self.cchFile)
            .field("cClsidExclude", &self.cClsidExclude)
            .field("lpClsidExclude", &self.lpClsidExclude)
            .field("iid", &self.iid)
            .field("oleRender", &self.oleRender)
            .field("lpFormatEtc", &self.lpFormatEtc)
            .field("lpIOleClientSite", &self.lpIOleClientSite)
            .field("lpIStorage", &self.lpIStorage)
            .field("ppvObj", &self.ppvObj)
            .field("sc", &self.sc)
            .field("hMetaPict", &self.hMetaPict)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::PartialEq for OLEUIINSERTOBJECTA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFlags == other.dwFlags
            && self.hWndOwner == other.hWndOwner
            && self.lpszCaption == other.lpszCaption
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lCustData == other.lCustData
            && self.hInstance == other.hInstance
            && self.lpszTemplate == other.lpszTemplate
            && self.hResource == other.hResource
            && self.clsid == other.clsid
            && self.lpszFile == other.lpszFile
            && self.cchFile == other.cchFile
            && self.cClsidExclude == other.cClsidExclude
            && self.lpClsidExclude == other.lpClsidExclude
            && self.iid == other.iid
            && self.oleRender == other.oleRender
            && self.lpFormatEtc == other.lpFormatEtc
            && self.lpIOleClientSite == other.lpIOleClientSite
            && self.lpIStorage == other.lpIStorage
            && self.ppvObj == other.ppvObj
            && self.sc == other.sc
            && self.hMetaPict == other.hMetaPict
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::Eq for OLEUIINSERTOBJECTA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
unsafe impl ::windows::runtime::Abi for OLEUIINSERTOBJECTA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_LibraryLoader`*"]
pub struct OLEUIINSERTOBJECTW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PWSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PWSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub clsid: ::windows::runtime::GUID,
    pub lpszFile: super::super::Foundation::PWSTR,
    pub cchFile: u32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows::runtime::GUID,
    pub iid: ::windows::runtime::GUID,
    pub oleRender: u32,
    pub lpFormatEtc: *mut super::Com::FORMATETC,
    pub lpIOleClientSite: ::std::option::Option<IOleClientSite>,
    pub lpIStorage: ::std::option::Option<super::Com::StructuredStorage::IStorage>,
    pub ppvObj: *mut *mut ::std::ffi::c_void,
    pub sc: i32,
    pub hMetaPict: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
impl OLEUIINSERTOBJECTW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
impl ::std::default::Default for OLEUIINSERTOBJECTW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
impl ::std::fmt::Debug for OLEUIINSERTOBJECTW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIINSERTOBJECTW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("clsid", &self.clsid)
            .field("lpszFile", &self.lpszFile)
            .field("cchFile", &self.cchFile)
            .field("cClsidExclude", &self.cClsidExclude)
            .field("lpClsidExclude", &self.lpClsidExclude)
            .field("iid", &self.iid)
            .field("oleRender", &self.oleRender)
            .field("lpFormatEtc", &self.lpFormatEtc)
            .field("lpIOleClientSite", &self.lpIOleClientSite)
            .field("lpIStorage", &self.lpIStorage)
            .field("ppvObj", &self.ppvObj)
            .field("sc", &self.sc)
            .field("hMetaPict", &self.hMetaPict)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::PartialEq for OLEUIINSERTOBJECTW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFlags == other.dwFlags
            && self.hWndOwner == other.hWndOwner
            && self.lpszCaption == other.lpszCaption
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lCustData == other.lCustData
            && self.hInstance == other.hInstance
            && self.lpszTemplate == other.lpszTemplate
            && self.hResource == other.hResource
            && self.clsid == other.clsid
            && self.lpszFile == other.lpszFile
            && self.cchFile == other.cchFile
            && self.cClsidExclude == other.cClsidExclude
            && self.lpClsidExclude == other.lpClsidExclude
            && self.iid == other.iid
            && self.oleRender == other.oleRender
            && self.lpFormatEtc == other.lpFormatEtc
            && self.lpIOleClientSite == other.lpIOleClientSite
            && self.lpIStorage == other.lpIStorage
            && self.ppvObj == other.ppvObj
            && self.sc == other.sc
            && self.hMetaPict == other.hMetaPict
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::Eq for OLEUIINSERTOBJECTW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
unsafe impl ::windows::runtime::Abi for OLEUIINSERTOBJECTW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
pub struct OLEUILINKPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl OLEUILINKPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for OLEUILINKPROPSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for OLEUILINKPROPSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUILINKPROPSA").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("lCustData", &self.lCustData).field("dwReserved2", &self.dwReserved2).field("lpOP", &self.lpOP).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for OLEUILINKPROPSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1 && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lCustData == other.lCustData && self.dwReserved2 == other.dwReserved2 && self.lpOP == other.lpOP
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for OLEUILINKPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for OLEUILINKPROPSA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
pub struct OLEUILINKPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl OLEUILINKPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for OLEUILINKPROPSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for OLEUILINKPROPSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUILINKPROPSW").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("lCustData", &self.lCustData).field("dwReserved2", &self.dwReserved2).field("lpOP", &self.lpOP).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for OLEUILINKPROPSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1 && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lCustData == other.lCustData && self.dwReserved2 == other.dwReserved2 && self.lpOP == other.lpOP
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for OLEUILINKPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for OLEUILINKPROPSW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
pub struct OLEUIOBJECTPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub lpPS: *mut super::super::UI::Controls::PROPSHEETHEADERA_V2,
    pub dwObject: u32,
    pub lpObjInfo: ::std::option::Option<IOleUIObjInfoA>,
    pub dwLink: u32,
    pub lpLinkInfo: ::std::option::Option<IOleUILinkInfoA>,
    pub lpGP: *mut OLEUIGNRLPROPSA,
    pub lpVP: *mut OLEUIVIEWPROPSA,
    pub lpLP: *mut OLEUILINKPROPSA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl OLEUIOBJECTPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for OLEUIOBJECTPROPSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for OLEUIOBJECTPROPSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIOBJECTPROPSA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("lpPS", &self.lpPS)
            .field("dwObject", &self.dwObject)
            .field("lpObjInfo", &self.lpObjInfo)
            .field("dwLink", &self.dwLink)
            .field("lpLinkInfo", &self.lpLinkInfo)
            .field("lpGP", &self.lpGP)
            .field("lpVP", &self.lpVP)
            .field("lpLP", &self.lpLP)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for OLEUIOBJECTPROPSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.lpPS == other.lpPS && self.dwObject == other.dwObject && self.lpObjInfo == other.lpObjInfo && self.dwLink == other.dwLink && self.lpLinkInfo == other.lpLinkInfo && self.lpGP == other.lpGP && self.lpVP == other.lpVP && self.lpLP == other.lpLP
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for OLEUIOBJECTPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for OLEUIOBJECTPROPSA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
pub struct OLEUIOBJECTPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub lpPS: *mut super::super::UI::Controls::PROPSHEETHEADERW_V2,
    pub dwObject: u32,
    pub lpObjInfo: ::std::option::Option<IOleUIObjInfoW>,
    pub dwLink: u32,
    pub lpLinkInfo: ::std::option::Option<IOleUILinkInfoW>,
    pub lpGP: *mut OLEUIGNRLPROPSW,
    pub lpVP: *mut OLEUIVIEWPROPSW,
    pub lpLP: *mut OLEUILINKPROPSW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl OLEUIOBJECTPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for OLEUIOBJECTPROPSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for OLEUIOBJECTPROPSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIOBJECTPROPSW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("lpPS", &self.lpPS)
            .field("dwObject", &self.dwObject)
            .field("lpObjInfo", &self.lpObjInfo)
            .field("dwLink", &self.dwLink)
            .field("lpLinkInfo", &self.lpLinkInfo)
            .field("lpGP", &self.lpGP)
            .field("lpVP", &self.lpVP)
            .field("lpLP", &self.lpLP)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for OLEUIOBJECTPROPSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.lpPS == other.lpPS && self.dwObject == other.dwObject && self.lpObjInfo == other.lpObjInfo && self.dwLink == other.dwLink && self.lpLinkInfo == other.lpLinkInfo && self.lpGP == other.lpGP && self.lpVP == other.lpVP && self.lpLP == other.lpLP
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for OLEUIOBJECTPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for OLEUIOBJECTPROPSW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct OLEUIPASTEENTRYA {
    pub fmtetc: super::Com::FORMATETC,
    pub lpstrFormatName: super::super::Foundation::PSTR,
    pub lpstrResultText: super::super::Foundation::PSTR,
    pub dwFlags: u32,
    pub dwScratchSpace: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl OLEUIPASTEENTRYA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for OLEUIPASTEENTRYA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for OLEUIPASTEENTRYA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIPASTEENTRYA").field("fmtetc", &self.fmtetc).field("lpstrFormatName", &self.lpstrFormatName).field("lpstrResultText", &self.lpstrResultText).field("dwFlags", &self.dwFlags).field("dwScratchSpace", &self.dwScratchSpace).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for OLEUIPASTEENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.fmtetc == other.fmtetc && self.lpstrFormatName == other.lpstrFormatName && self.lpstrResultText == other.lpstrResultText && self.dwFlags == other.dwFlags && self.dwScratchSpace == other.dwScratchSpace
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for OLEUIPASTEENTRYA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for OLEUIPASTEENTRYA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct OLEUIPASTEENTRYW {
    pub fmtetc: super::Com::FORMATETC,
    pub lpstrFormatName: super::super::Foundation::PWSTR,
    pub lpstrResultText: super::super::Foundation::PWSTR,
    pub dwFlags: u32,
    pub dwScratchSpace: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl OLEUIPASTEENTRYW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for OLEUIPASTEENTRYW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for OLEUIPASTEENTRYW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIPASTEENTRYW").field("fmtetc", &self.fmtetc).field("lpstrFormatName", &self.lpstrFormatName).field("lpstrResultText", &self.lpstrResultText).field("dwFlags", &self.dwFlags).field("dwScratchSpace", &self.dwScratchSpace).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for OLEUIPASTEENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.fmtetc == other.fmtetc && self.lpstrFormatName == other.lpstrFormatName && self.lpstrResultText == other.lpstrResultText && self.dwFlags == other.dwFlags && self.dwScratchSpace == other.dwScratchSpace
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for OLEUIPASTEENTRYW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for OLEUIPASTEENTRYW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLEUIPASTEFLAG(pub i32);
pub const OLEUIPASTE_ENABLEICON: OLEUIPASTEFLAG = OLEUIPASTEFLAG(2048i32);
pub const OLEUIPASTE_PASTEONLY: OLEUIPASTEFLAG = OLEUIPASTEFLAG(0i32);
pub const OLEUIPASTE_PASTE: OLEUIPASTEFLAG = OLEUIPASTEFLAG(512i32);
pub const OLEUIPASTE_LINKANYTYPE: OLEUIPASTEFLAG = OLEUIPASTEFLAG(1024i32);
pub const OLEUIPASTE_LINKTYPE1: OLEUIPASTEFLAG = OLEUIPASTEFLAG(1i32);
pub const OLEUIPASTE_LINKTYPE2: OLEUIPASTEFLAG = OLEUIPASTEFLAG(2i32);
pub const OLEUIPASTE_LINKTYPE3: OLEUIPASTEFLAG = OLEUIPASTEFLAG(4i32);
pub const OLEUIPASTE_LINKTYPE4: OLEUIPASTEFLAG = OLEUIPASTEFLAG(8i32);
pub const OLEUIPASTE_LINKTYPE5: OLEUIPASTEFLAG = OLEUIPASTEFLAG(16i32);
pub const OLEUIPASTE_LINKTYPE6: OLEUIPASTEFLAG = OLEUIPASTEFLAG(32i32);
pub const OLEUIPASTE_LINKTYPE7: OLEUIPASTEFLAG = OLEUIPASTEFLAG(64i32);
pub const OLEUIPASTE_LINKTYPE8: OLEUIPASTEFLAG = OLEUIPASTEFLAG(128i32);
impl ::std::convert::From<i32> for OLEUIPASTEFLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLEUIPASTEFLAG {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_LibraryLoader`*"]
pub struct OLEUIPASTESPECIALA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub lpSrcDataObj: ::std::option::Option<super::Com::IDataObject>,
    pub arrPasteEntries: *mut OLEUIPASTEENTRYA,
    pub cPasteEntries: i32,
    pub arrLinkTypes: *mut u32,
    pub cLinkTypes: i32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows::runtime::GUID,
    pub nSelectedIndex: i32,
    pub fLink: super::super::Foundation::BOOL,
    pub hMetaPict: isize,
    pub sizel: super::super::Foundation::SIZE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
impl OLEUIPASTESPECIALA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
impl ::std::default::Default for OLEUIPASTESPECIALA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
impl ::std::fmt::Debug for OLEUIPASTESPECIALA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIPASTESPECIALA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("lpSrcDataObj", &self.lpSrcDataObj)
            .field("arrPasteEntries", &self.arrPasteEntries)
            .field("cPasteEntries", &self.cPasteEntries)
            .field("arrLinkTypes", &self.arrLinkTypes)
            .field("cLinkTypes", &self.cLinkTypes)
            .field("cClsidExclude", &self.cClsidExclude)
            .field("lpClsidExclude", &self.lpClsidExclude)
            .field("nSelectedIndex", &self.nSelectedIndex)
            .field("fLink", &self.fLink)
            .field("hMetaPict", &self.hMetaPict)
            .field("sizel", &self.sizel)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::PartialEq for OLEUIPASTESPECIALA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFlags == other.dwFlags
            && self.hWndOwner == other.hWndOwner
            && self.lpszCaption == other.lpszCaption
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lCustData == other.lCustData
            && self.hInstance == other.hInstance
            && self.lpszTemplate == other.lpszTemplate
            && self.hResource == other.hResource
            && self.lpSrcDataObj == other.lpSrcDataObj
            && self.arrPasteEntries == other.arrPasteEntries
            && self.cPasteEntries == other.cPasteEntries
            && self.arrLinkTypes == other.arrLinkTypes
            && self.cLinkTypes == other.cLinkTypes
            && self.cClsidExclude == other.cClsidExclude
            && self.lpClsidExclude == other.lpClsidExclude
            && self.nSelectedIndex == other.nSelectedIndex
            && self.fLink == other.fLink
            && self.hMetaPict == other.hMetaPict
            && self.sizel == other.sizel
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::Eq for OLEUIPASTESPECIALA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
unsafe impl ::windows::runtime::Abi for OLEUIPASTESPECIALA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_LibraryLoader`*"]
pub struct OLEUIPASTESPECIALW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: super::super::Foundation::PWSTR,
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: super::super::Foundation::PWSTR,
    pub hResource: super::LibraryLoader::HRSRC,
    pub lpSrcDataObj: ::std::option::Option<super::Com::IDataObject>,
    pub arrPasteEntries: *mut OLEUIPASTEENTRYW,
    pub cPasteEntries: i32,
    pub arrLinkTypes: *mut u32,
    pub cLinkTypes: i32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows::runtime::GUID,
    pub nSelectedIndex: i32,
    pub fLink: super::super::Foundation::BOOL,
    pub hMetaPict: isize,
    pub sizel: super::super::Foundation::SIZE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
impl OLEUIPASTESPECIALW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
impl ::std::default::Default for OLEUIPASTESPECIALW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
impl ::std::fmt::Debug for OLEUIPASTESPECIALW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIPASTESPECIALW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("lpSrcDataObj", &self.lpSrcDataObj)
            .field("arrPasteEntries", &self.arrPasteEntries)
            .field("cPasteEntries", &self.cPasteEntries)
            .field("arrLinkTypes", &self.arrLinkTypes)
            .field("cLinkTypes", &self.cLinkTypes)
            .field("cClsidExclude", &self.cClsidExclude)
            .field("lpClsidExclude", &self.lpClsidExclude)
            .field("nSelectedIndex", &self.nSelectedIndex)
            .field("fLink", &self.fLink)
            .field("hMetaPict", &self.hMetaPict)
            .field("sizel", &self.sizel)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::PartialEq for OLEUIPASTESPECIALW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFlags == other.dwFlags
            && self.hWndOwner == other.hWndOwner
            && self.lpszCaption == other.lpszCaption
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lCustData == other.lCustData
            && self.hInstance == other.hInstance
            && self.lpszTemplate == other.lpszTemplate
            && self.hResource == other.hResource
            && self.lpSrcDataObj == other.lpSrcDataObj
            && self.arrPasteEntries == other.arrPasteEntries
            && self.cPasteEntries == other.cPasteEntries
            && self.arrLinkTypes == other.arrLinkTypes
            && self.cLinkTypes == other.cLinkTypes
            && self.cClsidExclude == other.cClsidExclude
            && self.lpClsidExclude == other.lpClsidExclude
            && self.nSelectedIndex == other.nSelectedIndex
            && self.fLink == other.fLink
            && self.hMetaPict == other.hMetaPict
            && self.sizel == other.sizel
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
impl ::std::cmp::Eq for OLEUIPASTESPECIALW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
unsafe impl ::windows::runtime::Abi for OLEUIPASTESPECIALW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
pub struct OLEUIVIEWPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
    pub nScaleMin: i32,
    pub nScaleMax: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl OLEUIVIEWPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for OLEUIVIEWPROPSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for OLEUIVIEWPROPSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIVIEWPROPSA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("dwReserved1", &self.dwReserved1)
            .field("lCustData", &self.lCustData)
            .field("dwReserved2", &self.dwReserved2)
            .field("lpOP", &self.lpOP)
            .field("nScaleMin", &self.nScaleMin)
            .field("nScaleMax", &self.nScaleMax)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for OLEUIVIEWPROPSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1 && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lCustData == other.lCustData && self.dwReserved2 == other.dwReserved2 && self.lpOP == other.lpOP && self.nScaleMin == other.nScaleMin && self.nScaleMax == other.nScaleMax
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for OLEUIVIEWPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for OLEUIVIEWPROPSA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
pub struct OLEUIVIEWPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: ::std::option::Option<LPFNOLEUIHOOK>,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
    pub nScaleMin: i32,
    pub nScaleMax: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl OLEUIVIEWPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for OLEUIVIEWPROPSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for OLEUIVIEWPROPSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEUIVIEWPROPSW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("dwReserved1", &self.dwReserved1)
            .field("lCustData", &self.lCustData)
            .field("dwReserved2", &self.dwReserved2)
            .field("lpOP", &self.lpOP)
            .field("nScaleMin", &self.nScaleMin)
            .field("nScaleMax", &self.nScaleMax)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for OLEUIVIEWPROPSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1 && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lCustData == other.lCustData && self.dwReserved2 == other.dwReserved2 && self.lpOP == other.lpOP && self.nScaleMin == other.nScaleMin && self.nScaleMax == other.nScaleMax
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for OLEUIVIEWPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for OLEUIVIEWPROPSW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_BZERR_HTASKINVALID: u32 = 116u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_BZ_CALLUNBLOCKED: u32 = 119u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_BZ_RETRYSELECTED: u32 = 118u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_BZ_SWITCHTOSELECTED: u32 = 117u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CANCEL: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CIERR_MUSTHAVECLSID: u32 = 116u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CIERR_MUSTHAVECURRENTMETAFILE: u32 = 117u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CIERR_SZICONEXEINVALID: u32 = 118u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CSERR_FROMNOTNULL: u32 = 118u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CSERR_LINKCNTRINVALID: u32 = 117u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CSERR_LINKCNTRNULL: u32 = 116u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CSERR_SOURCEINVALID: u32 = 121u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CSERR_SOURCENULL: u32 = 120u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CSERR_SOURCEPARSEERROR: u32 = 122u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CSERR_SOURCEPARSERROR: u32 = 122u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CSERR_TONOTNULL: u32 = 119u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CTERR_CBFORMATINVALID: u32 = 119u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CTERR_CLASSIDINVALID: u32 = 117u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CTERR_DVASPECTINVALID: u32 = 118u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CTERR_HMETAPICTINVALID: u32 = 120u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_CTERR_STRINGINVALID: u32 = 121u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ELERR_LINKCNTRINVALID: u32 = 117u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ELERR_LINKCNTRNULL: u32 = 116u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_CBSTRUCTINCORRECT: u32 = 103u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_DIALOGFAILURE: u32 = 112u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_FINDTEMPLATEFAILURE: u32 = 110u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_GLOBALMEMALLOC: u32 = 114u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_HINSTANCEINVALID: u32 = 107u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_HRESOURCEINVALID: u32 = 109u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_HWNDOWNERINVALID: u32 = 104u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_LOADSTRING: u32 = 115u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_LOADTEMPLATEFAILURE: u32 = 111u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_LOCALMEMALLOC: u32 = 113u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_LPFNHOOKINVALID: u32 = 106u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_LPSZCAPTIONINVALID: u32 = 105u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_LPSZTEMPLATEINVALID: u32 = 108u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_OLEMEMALLOC: u32 = 100u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_STANDARDMAX: u32 = 116u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_STANDARDMIN: u32 = 100u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_STRUCTUREINVALID: u32 = 102u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_ERR_STRUCTURENULL: u32 = 101u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_FALSE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_GPERR_CBFORMATINVALID: u32 = 130u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_GPERR_CLASSIDINVALID: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_GPERR_LPCLSIDEXCLUDEINVALID: u32 = 129u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_GPERR_STRINGINVALID: u32 = 127u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_IOERR_ARRLINKTYPESINVALID: u32 = 118u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_IOERR_ARRPASTEENTRIESINVALID: u32 = 117u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_IOERR_CCHFILEINVALID: u32 = 125u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_IOERR_HICONINVALID: u32 = 118u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_IOERR_LPCLSIDEXCLUDEINVALID: u32 = 124u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_IOERR_LPFORMATETCINVALID: u32 = 119u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_IOERR_LPIOLECLIENTSITEINVALID: u32 = 121u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_IOERR_LPISTORAGEINVALID: u32 = 122u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_IOERR_LPSZFILEINVALID: u32 = 116u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_IOERR_LPSZLABELINVALID: u32 = 117u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_IOERR_PPVOBJINVALID: u32 = 120u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_IOERR_SCODEHASERROR: u32 = 123u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_IOERR_SRCDATAOBJECTINVALID: u32 = 116u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_LPERR_LINKCNTRINVALID: u32 = 134u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_LPERR_LINKCNTRNULL: u32 = 133u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OK: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_DLGPROCNOTNULL: u32 = 125u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_INVALIDPAGES: u32 = 123u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_LINKINFOINVALID: u32 = 137u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_LPARAMNOTZERO: u32 = 126u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_NOTSUPPORTED: u32 = 124u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_OBJINFOINVALID: u32 = 136u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_PAGESINCORRECT: u32 = 122u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_PROPERTYSHEET: u32 = 135u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_PROPSHEETINVALID: u32 = 119u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_PROPSHEETNULL: u32 = 118u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_PROPSINVALID: u32 = 121u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_SUBPROPINVALID: u32 = 117u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_SUBPROPNULL: u32 = 116u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_OPERR_SUPPROP: u32 = 120u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_PSERR_CLIPBOARDCHANGED: u32 = 119u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_PSERR_GETCLIPBOARDFAILED: u32 = 120u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_QUERY_GETCLASSID: u32 = 65280u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_QUERY_LINKBROKEN: u32 = 65281u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_VPERR_DVASPECTINVALID: u32 = 132u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEUI_VPERR_METAPICTINVALID: u32 = 131u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLEUPDATE(pub i32);
pub const OLEUPDATE_ALWAYS: OLEUPDATE = OLEUPDATE(1i32);
pub const OLEUPDATE_ONCALL: OLEUPDATE = OLEUPDATE(3i32);
impl ::std::convert::From<i32> for OLEUPDATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLEUPDATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
pub struct OLEVERB {
    pub lVerb: i32,
    pub lpszVerbName: super::super::Foundation::PWSTR,
    pub fuFlags: u32,
    pub grfAttribs: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl OLEVERB {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OLEVERB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OLEVERB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLEVERB").field("lVerb", &self.lVerb).field("lpszVerbName", &self.lpszVerbName).field("fuFlags", &self.fuFlags).field("grfAttribs", &self.grfAttribs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OLEVERB {
    fn eq(&self, other: &Self) -> bool {
        self.lVerb == other.lVerb && self.lpszVerbName == other.lpszVerbName && self.fuFlags == other.fuFlags && self.grfAttribs == other.grfAttribs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OLEVERB {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OLEVERB {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLEVERBATTRIB(pub i32);
pub const OLEVERBATTRIB_NEVERDIRTIES: OLEVERBATTRIB = OLEVERBATTRIB(1i32);
pub const OLEVERBATTRIB_ONCONTAINERMENU: OLEVERBATTRIB = OLEVERBATTRIB(2i32);
impl ::std::convert::From<i32> for OLEVERBATTRIB {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLEVERBATTRIB {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OLEVERB_PRIMARY: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLEWHICHMK(pub i32);
pub const OLEWHICHMK_CONTAINER: OLEWHICHMK = OLEWHICHMK(1i32);
pub const OLEWHICHMK_OBJREL: OLEWHICHMK = OLEWHICHMK(2i32);
pub const OLEWHICHMK_OBJFULL: OLEWHICHMK = OLEWHICHMK(3i32);
impl ::std::convert::From<i32> for OLEWHICHMK {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLEWHICHMK {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OLE_TRISTATE(pub i32);
pub const triUnchecked: OLE_TRISTATE = OLE_TRISTATE(0i32);
pub const triChecked: OLE_TRISTATE = OLE_TRISTATE(1i32);
pub const triGray: OLE_TRISTATE = OLE_TRISTATE(2i32);
impl ::std::convert::From<i32> for OLE_TRISTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OLE_TRISTATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OPF_DISABLECONVERT: i32 = 8i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OPF_NOFILLDEFAULT: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OPF_OBJECTISLINK: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OPF_SHOWHELP: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OT_EMBEDDED: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OT_LINK: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const OT_STATIC: i32 = 3i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[inline]
pub unsafe fn OleBuildVersion() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleBuildVersion() -> u32;
        }
        ::std::mem::transmute(OleBuildVersion())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleCreate<'a, Param4: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param5: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: Param4, pstg: Param5, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreate(rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreate(::std::mem::transmute(rclsid), ::std::mem::transmute(riid), ::std::mem::transmute(renderopt), ::std::mem::transmute(pformatetc), pclientsite.into_param().abi(), pstg.into_param().abi(), ::std::mem::transmute(ppvobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[inline]
pub unsafe fn OleCreateDefaultHandler<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(clsid: *const ::windows::runtime::GUID, punkouter: Param1, riid: *const ::windows::runtime::GUID, lplpobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateDefaultHandler(clsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, lplpobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateDefaultHandler(::std::mem::transmute(clsid), punkouter.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(lplpobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleCreateEmbeddingHelper<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, super::Com::IClassFactory>>(clsid: *const ::windows::runtime::GUID, punkouter: Param1, flags: u32, pcf: Param3, riid: *const ::windows::runtime::GUID, lplpobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateEmbeddingHelper(clsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, flags: u32, pcf: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, lplpobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateEmbeddingHelper(::std::mem::transmute(clsid), punkouter.into_param().abi(), ::std::mem::transmute(flags), pcf.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(lplpobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleCreateEx<'a, Param7: ::windows::runtime::IntoParam<'a, super::Com::IAdviseSink>, Param9: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param10: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(
    rclsid: *const ::windows::runtime::GUID,
    riid: *const ::windows::runtime::GUID,
    dwflags: u32,
    renderopt: u32,
    cformats: u32,
    rgadvf: *mut u32,
    rgformatetc: *mut super::Com::FORMATETC,
    lpadvisesink: Param7,
    rgdwconnection: *mut u32,
    pclientsite: Param9,
    pstg: Param10,
    ppvobj: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateEx(rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: ::windows::runtime::RawPtr, rgdwconnection: *mut u32, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateEx(
            ::std::mem::transmute(rclsid),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(renderopt),
            ::std::mem::transmute(cformats),
            ::std::mem::transmute(rgadvf),
            ::std::mem::transmute(rgformatetc),
            lpadvisesink.into_param().abi(),
            ::std::mem::transmute(rgdwconnection),
            pclientsite.into_param().abi(),
            pstg.into_param().abi(),
            ::std::mem::transmute(ppvobj),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OleCreateFontIndirect(lpfontdesc: *mut FONTDESC, riid: *const ::windows::runtime::GUID, lplpvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateFontIndirect(lpfontdesc: *mut FONTDESC, riid: *const ::windows::runtime::GUID, lplpvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateFontIndirect(::std::mem::transmute(lpfontdesc), ::std::mem::transmute(riid), ::std::mem::transmute(lplpvobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleCreateFromData<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>, Param4: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param5: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(psrcdataobj: Param0, riid: *const ::windows::runtime::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: Param4, pstg: Param5, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateFromData(psrcdataobj: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateFromData(psrcdataobj.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(renderopt), ::std::mem::transmute(pformatetc), pclientsite.into_param().abi(), pstg.into_param().abi(), ::std::mem::transmute(ppvobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleCreateFromDataEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>, Param7: ::windows::runtime::IntoParam<'a, super::Com::IAdviseSink>, Param9: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param10: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(
    psrcdataobj: Param0,
    riid: *const ::windows::runtime::GUID,
    dwflags: u32,
    renderopt: u32,
    cformats: u32,
    rgadvf: *mut u32,
    rgformatetc: *mut super::Com::FORMATETC,
    lpadvisesink: Param7,
    rgdwconnection: *mut u32,
    pclientsite: Param9,
    pstg: Param10,
    ppvobj: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateFromDataEx(psrcdataobj: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: ::windows::runtime::RawPtr, rgdwconnection: *mut u32, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateFromDataEx(
            psrcdataobj.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(renderopt),
            ::std::mem::transmute(cformats),
            ::std::mem::transmute(rgadvf),
            ::std::mem::transmute(rgformatetc),
            lpadvisesink.into_param().abi(),
            ::std::mem::transmute(rgdwconnection),
            pclientsite.into_param().abi(),
            pstg.into_param().abi(),
            ::std::mem::transmute(ppvobj),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleCreateFromFile<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param6: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(
    rclsid: *const ::windows::runtime::GUID,
    lpszfilename: Param1,
    riid: *const ::windows::runtime::GUID,
    renderopt: u32,
    lpformatetc: *mut super::Com::FORMATETC,
    pclientsite: Param5,
    pstg: Param6,
    ppvobj: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateFromFile(rclsid: *const ::windows::runtime::GUID, lpszfilename: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateFromFile(::std::mem::transmute(rclsid), lpszfilename.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(renderopt), ::std::mem::transmute(lpformatetc), pclientsite.into_param().abi(), pstg.into_param().abi(), ::std::mem::transmute(ppvobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleCreateFromFileEx<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param8: ::windows::runtime::IntoParam<'a, super::Com::IAdviseSink>, Param10: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param11: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(
    rclsid: *const ::windows::runtime::GUID,
    lpszfilename: Param1,
    riid: *const ::windows::runtime::GUID,
    dwflags: u32,
    renderopt: u32,
    cformats: u32,
    rgadvf: *mut u32,
    rgformatetc: *mut super::Com::FORMATETC,
    lpadvisesink: Param8,
    rgdwconnection: *mut u32,
    pclientsite: Param10,
    pstg: Param11,
    ppvobj: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateFromFileEx(rclsid: *const ::windows::runtime::GUID, lpszfilename: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: ::windows::runtime::RawPtr, rgdwconnection: *mut u32, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateFromFileEx(
            ::std::mem::transmute(rclsid),
            lpszfilename.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(renderopt),
            ::std::mem::transmute(cformats),
            ::std::mem::transmute(rgadvf),
            ::std::mem::transmute(rgformatetc),
            lpadvisesink.into_param().abi(),
            ::std::mem::transmute(rgdwconnection),
            pclientsite.into_param().abi(),
            pstg.into_param().abi(),
            ::std::mem::transmute(ppvobj),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleCreateLink<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IMoniker>, Param4: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param5: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(pmklinksrc: Param0, riid: *const ::windows::runtime::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: Param4, pstg: Param5, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateLink(pmklinksrc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateLink(pmklinksrc.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(renderopt), ::std::mem::transmute(lpformatetc), pclientsite.into_param().abi(), pstg.into_param().abi(), ::std::mem::transmute(ppvobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleCreateLinkEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IMoniker>, Param7: ::windows::runtime::IntoParam<'a, super::Com::IAdviseSink>, Param9: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param10: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(
    pmklinksrc: Param0,
    riid: *const ::windows::runtime::GUID,
    dwflags: u32,
    renderopt: u32,
    cformats: u32,
    rgadvf: *mut u32,
    rgformatetc: *mut super::Com::FORMATETC,
    lpadvisesink: Param7,
    rgdwconnection: *mut u32,
    pclientsite: Param9,
    pstg: Param10,
    ppvobj: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateLinkEx(pmklinksrc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: ::windows::runtime::RawPtr, rgdwconnection: *mut u32, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateLinkEx(
            pmklinksrc.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(renderopt),
            ::std::mem::transmute(cformats),
            ::std::mem::transmute(rgadvf),
            ::std::mem::transmute(rgformatetc),
            lpadvisesink.into_param().abi(),
            ::std::mem::transmute(rgdwconnection),
            pclientsite.into_param().abi(),
            pstg.into_param().abi(),
            ::std::mem::transmute(ppvobj),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleCreateLinkFromData<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>, Param4: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param5: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(psrcdataobj: Param0, riid: *const ::windows::runtime::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: Param4, pstg: Param5, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateLinkFromData(psrcdataobj: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateLinkFromData(psrcdataobj.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(renderopt), ::std::mem::transmute(pformatetc), pclientsite.into_param().abi(), pstg.into_param().abi(), ::std::mem::transmute(ppvobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleCreateLinkFromDataEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>, Param7: ::windows::runtime::IntoParam<'a, super::Com::IAdviseSink>, Param9: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param10: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(
    psrcdataobj: Param0,
    riid: *const ::windows::runtime::GUID,
    dwflags: u32,
    renderopt: u32,
    cformats: u32,
    rgadvf: *mut u32,
    rgformatetc: *mut super::Com::FORMATETC,
    lpadvisesink: Param7,
    rgdwconnection: *mut u32,
    pclientsite: Param9,
    pstg: Param10,
    ppvobj: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateLinkFromDataEx(psrcdataobj: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: ::windows::runtime::RawPtr, rgdwconnection: *mut u32, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateLinkFromDataEx(
            psrcdataobj.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(renderopt),
            ::std::mem::transmute(cformats),
            ::std::mem::transmute(rgadvf),
            ::std::mem::transmute(rgformatetc),
            lpadvisesink.into_param().abi(),
            ::std::mem::transmute(rgdwconnection),
            pclientsite.into_param().abi(),
            pstg.into_param().abi(),
            ::std::mem::transmute(ppvobj),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleCreateLinkToFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param5: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(lpszfilename: Param0, riid: *const ::windows::runtime::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: Param4, pstg: Param5, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateLinkToFile(lpszfilename: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateLinkToFile(lpszfilename.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(renderopt), ::std::mem::transmute(lpformatetc), pclientsite.into_param().abi(), pstg.into_param().abi(), ::std::mem::transmute(ppvobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleCreateLinkToFileEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param7: ::windows::runtime::IntoParam<'a, super::Com::IAdviseSink>, Param9: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param10: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(
    lpszfilename: Param0,
    riid: *const ::windows::runtime::GUID,
    dwflags: u32,
    renderopt: u32,
    cformats: u32,
    rgadvf: *mut u32,
    rgformatetc: *mut super::Com::FORMATETC,
    lpadvisesink: Param7,
    rgdwconnection: *mut u32,
    pclientsite: Param9,
    pstg: Param10,
    ppvobj: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateLinkToFileEx(lpszfilename: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: ::windows::runtime::RawPtr, rgdwconnection: *mut u32, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateLinkToFileEx(
            lpszfilename.into_param().abi(),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(renderopt),
            ::std::mem::transmute(cformats),
            ::std::mem::transmute(rgadvf),
            ::std::mem::transmute(rgformatetc),
            lpadvisesink.into_param().abi(),
            ::std::mem::transmute(rgdwconnection),
            pclientsite.into_param().abi(),
            pstg.into_param().abi(),
            ::std::mem::transmute(ppvobj),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn OleCreateMenuDescriptor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::WindowsAndMessaging::HMENU>>(hmenucombined: Param0, lpmenuwidths: *mut OleMenuGroupWidths) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateMenuDescriptor(hmenucombined: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OleMenuGroupWidths) -> isize;
        }
        ::std::mem::transmute(OleCreateMenuDescriptor(hmenucombined.into_param().abi(), ::std::mem::transmute(lpmenuwidths)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleCreatePictureIndirect<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lppictdesc: *mut PICTDESC, riid: *const ::windows::runtime::GUID, fown: Param2, lplpvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreatePictureIndirect(lppictdesc: *mut PICTDESC, riid: *const ::windows::runtime::GUID, fown: super::super::Foundation::BOOL, lplpvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreatePictureIndirect(::std::mem::transmute(lppictdesc), ::std::mem::transmute(riid), fown.into_param().abi(), ::std::mem::transmute(lplpvobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleCreatePropertyFrame<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hwndowner: Param0, x: u32, y: u32, lpszcaption: Param3, cobjects: u32, ppunk: *mut ::std::option::Option<::windows::runtime::IUnknown>, cpages: u32, ppageclsid: *mut ::windows::runtime::GUID, lcid: u32, dwreserved: u32, pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreatePropertyFrame(hwndowner: super::super::Foundation::HWND, x: u32, y: u32, lpszcaption: super::super::Foundation::PWSTR, cobjects: u32, ppunk: *mut ::windows::runtime::RawPtr, cpages: u32, ppageclsid: *mut ::windows::runtime::GUID, lcid: u32, dwreserved: u32, pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreatePropertyFrame(
            hwndowner.into_param().abi(),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
            lpszcaption.into_param().abi(),
            ::std::mem::transmute(cobjects),
            ::std::mem::transmute(ppunk),
            ::std::mem::transmute(cpages),
            ::std::mem::transmute(ppageclsid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(dwreserved),
            ::std::mem::transmute(pvreserved),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleCreatePropertyFrameIndirect(lpparams: *mut OCPFIPARAMS) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreatePropertyFrameIndirect(lpparams: *mut OCPFIPARAMS) -> ::windows::runtime::HRESULT;
        }
        OleCreatePropertyFrameIndirect(::std::mem::transmute(lpparams)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleCreateStaticFromData<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>, Param4: ::windows::runtime::IntoParam<'a, IOleClientSite>, Param5: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(psrcdataobj: Param0, iid: *const ::windows::runtime::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: Param4, pstg: Param5, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleCreateStaticFromData(psrcdataobj: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleCreateStaticFromData(psrcdataobj.into_param().abi(), ::std::mem::transmute(iid), ::std::mem::transmute(renderopt), ::std::mem::transmute(pformatetc), pclientsite.into_param().abi(), pstg.into_param().abi(), ::std::mem::transmute(ppvobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[inline]
pub unsafe fn OleDestroyMenuDescriptor(holemenu: isize) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleDestroyMenuDescriptor(holemenu: isize) -> ::windows::runtime::HRESULT;
        }
        OleDestroyMenuDescriptor(::std::mem::transmute(holemenu)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleDoAutoConvert<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>>(pstg: Param0, pclsidnew: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleDoAutoConvert(pstg: ::windows::runtime::RawPtr, pclsidnew: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        OleDoAutoConvert(pstg.into_param().abi(), ::std::mem::transmute(pclsidnew)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn OleDraw<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(punknown: Param0, dwaspect: u32, hdcdraw: Param2, lprcbounds: *mut super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleDraw(punknown: ::windows::runtime::RawPtr, dwaspect: u32, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
        }
        OleDraw(punknown.into_param().abi(), ::std::mem::transmute(dwaspect), hdcdraw.into_param().abi(), ::std::mem::transmute(lprcbounds)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleDuplicateData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hsrc: Param0, cfformat: u16, uiflags: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleDuplicateData(hsrc: super::super::Foundation::HANDLE, cfformat: u16, uiflags: u32) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OleDuplicateData(hsrc.into_param().abi(), ::std::mem::transmute(cfformat), ::std::mem::transmute(uiflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[inline]
pub unsafe fn OleFlushClipboard() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleFlushClipboard() -> ::windows::runtime::HRESULT;
        }
        OleFlushClipboard().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[inline]
pub unsafe fn OleGetAutoConvert(clsidold: *const ::windows::runtime::GUID, pclsidnew: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleGetAutoConvert(clsidold: *const ::windows::runtime::GUID, pclsidnew: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        OleGetAutoConvert(::std::mem::transmute(clsidold), ::std::mem::transmute(pclsidnew)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleGetClipboard() -> ::windows::runtime::Result<super::Com::IDataObject> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleGetClipboard(ppdataobj: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::IDataObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        OleGetClipboard(&mut result__).from_abi::<super::Com::IDataObject>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OleGetClipboardWithEnterpriseInfo(dataobject: *mut ::std::option::Option<super::Com::IDataObject>, dataenterpriseid: *mut super::super::Foundation::PWSTR, sourcedescription: *mut super::super::Foundation::PWSTR, targetdescription: *mut super::super::Foundation::PWSTR, datadescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleGetClipboardWithEnterpriseInfo(dataobject: *mut ::windows::runtime::RawPtr, dataenterpriseid: *mut super::super::Foundation::PWSTR, sourcedescription: *mut super::super::Foundation::PWSTR, targetdescription: *mut super::super::Foundation::PWSTR, datadescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        OleGetClipboardWithEnterpriseInfo(::std::mem::transmute(dataobject), ::std::mem::transmute(dataenterpriseid), ::std::mem::transmute(sourcedescription), ::std::mem::transmute(targetdescription), ::std::mem::transmute(datadescription)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleGetIconOfClass<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(rclsid: *const ::windows::runtime::GUID, lpszlabel: Param1, fusetypeaslabel: Param2) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleGetIconOfClass(rclsid: *const ::windows::runtime::GUID, lpszlabel: super::super::Foundation::PWSTR, fusetypeaslabel: super::super::Foundation::BOOL) -> isize;
        }
        ::std::mem::transmute(OleGetIconOfClass(::std::mem::transmute(rclsid), lpszlabel.into_param().abi(), fusetypeaslabel.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleGetIconOfFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpszpath: Param0, fusefileaslabel: Param1) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleGetIconOfFile(lpszpath: super::super::Foundation::PWSTR, fusefileaslabel: super::super::Foundation::BOOL) -> isize;
        }
        ::std::mem::transmute(OleGetIconOfFile(lpszpath.into_param().abi(), fusefileaslabel.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleIconToCursor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::runtime::IntoParam<'a, super::super::UI::WindowsAndMessaging::HICON>>(hinstexe: Param0, hicon: Param1) -> super::super::UI::WindowsAndMessaging::HCURSOR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleIconToCursor(hinstexe: super::super::Foundation::HINSTANCE, hicon: super::super::UI::WindowsAndMessaging::HICON) -> super::super::UI::WindowsAndMessaging::HCURSOR;
        }
        ::std::mem::transmute(OleIconToCursor(hinstexe.into_param().abi(), hicon.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[inline]
pub unsafe fn OleInitialize(pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleInitialize(pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleInitialize(::std::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleIsCurrentClipboard<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>>(pdataobj: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleIsCurrentClipboard(pdataobj: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        OleIsCurrentClipboard(pdataobj.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleIsRunning<'a, Param0: ::windows::runtime::IntoParam<'a, IOleObject>>(pobject: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleIsRunning(pobject: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(OleIsRunning(pobject.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleLoad<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>, Param2: ::windows::runtime::IntoParam<'a, IOleClientSite>>(pstg: Param0, riid: *const ::windows::runtime::GUID, pclientsite: Param2, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleLoad(pstg: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pclientsite: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleLoad(pstg.into_param().abi(), ::std::mem::transmute(riid), pclientsite.into_param().abi(), ::std::mem::transmute(ppvobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleLoadFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>>(pstm: Param0, iidinterface: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleLoadFromStream(pstm: ::windows::runtime::RawPtr, iidinterface: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleLoadFromStream(pstm.into_param().abi(), ::std::mem::transmute(iidinterface), ::std::mem::transmute(ppvobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OleLoadPicture<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpstream: Param0, lsize: i32, frunmode: Param2, riid: *const ::windows::runtime::GUID, lplpvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleLoadPicture(lpstream: ::windows::runtime::RawPtr, lsize: i32, frunmode: super::super::Foundation::BOOL, riid: *const ::windows::runtime::GUID, lplpvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleLoadPicture(lpstream.into_param().abi(), ::std::mem::transmute(lsize), frunmode.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(lplpvobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OleLoadPictureEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpstream: Param0, lsize: i32, frunmode: Param2, riid: *const ::windows::runtime::GUID, xsizedesired: u32, ysizedesired: u32, dwflags: u32, lplpvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleLoadPictureEx(lpstream: ::windows::runtime::RawPtr, lsize: i32, frunmode: super::super::Foundation::BOOL, riid: *const ::windows::runtime::GUID, xsizedesired: u32, ysizedesired: u32, dwflags: u32, lplpvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleLoadPictureEx(lpstream.into_param().abi(), ::std::mem::transmute(lsize), frunmode.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(xsizedesired), ::std::mem::transmute(ysizedesired), ::std::mem::transmute(dwflags), ::std::mem::transmute(lplpvobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn OleLoadPictureFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(varfilename: Param0) -> ::windows::runtime::Result<Automation::IDispatch> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleLoadPictureFile(varfilename: ::std::mem::ManuallyDrop<super::Com::VARIANT>, lplpdisppicture: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        OleLoadPictureFile(varfilename.into_param().abi(), &mut result__).from_abi::<Automation::IDispatch>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn OleLoadPictureFileEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(varfilename: Param0, xsizedesired: u32, ysizedesired: u32, dwflags: u32) -> ::windows::runtime::Result<Automation::IDispatch> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleLoadPictureFileEx(varfilename: ::std::mem::ManuallyDrop<super::Com::VARIANT>, xsizedesired: u32, ysizedesired: u32, dwflags: u32, lplpdisppicture: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        OleLoadPictureFileEx(varfilename.into_param().abi(), ::std::mem::transmute(xsizedesired), ::std::mem::transmute(ysizedesired), ::std::mem::transmute(dwflags), &mut result__).from_abi::<Automation::IDispatch>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleLoadPicturePath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(szurlorpath: Param0, punkcaller: Param1, dwreserved: u32, clrreserved: u32, riid: *const ::windows::runtime::GUID, ppvret: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleLoadPicturePath(szurlorpath: super::super::Foundation::PWSTR, punkcaller: ::windows::runtime::RawPtr, dwreserved: u32, clrreserved: u32, riid: *const ::windows::runtime::GUID, ppvret: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OleLoadPicturePath(szurlorpath.into_param().abi(), punkcaller.into_param().abi(), ::std::mem::transmute(dwreserved), ::std::mem::transmute(clrreserved), ::std::mem::transmute(riid), ::std::mem::transmute(ppvret)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleLockRunning<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(punknown: Param0, flock: Param1, flastunlockcloses: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleLockRunning(punknown: ::windows::runtime::RawPtr, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        OleLockRunning(punknown.into_param().abi(), flock.into_param().abi(), flastunlockcloses.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Ole`*"]
pub struct OleMenuGroupWidths {
    pub width: [i32; 6],
}
impl OleMenuGroupWidths {}
impl ::std::default::Default for OleMenuGroupWidths {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OleMenuGroupWidths {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OleMenuGroupWidths").field("width", &self.width).finish()
    }
}
impl ::std::cmp::PartialEq for OleMenuGroupWidths {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
    }
}
impl ::std::cmp::Eq for OleMenuGroupWidths {}
unsafe impl ::windows::runtime::Abi for OleMenuGroupWidths {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleMetafilePictFromIconAndLabel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::WindowsAndMessaging::HICON>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hicon: Param0, lpszlabel: Param1, lpszsourcefile: Param2, iiconindex: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleMetafilePictFromIconAndLabel(hicon: super::super::UI::WindowsAndMessaging::HICON, lpszlabel: super::super::Foundation::PWSTR, lpszsourcefile: super::super::Foundation::PWSTR, iiconindex: u32) -> isize;
        }
        ::std::mem::transmute(OleMetafilePictFromIconAndLabel(hicon.into_param().abi(), lpszlabel.into_param().abi(), lpszsourcefile.into_param().abi(), ::std::mem::transmute(iiconindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleNoteObjectVisible<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(punknown: Param0, fvisible: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleNoteObjectVisible(punknown: ::windows::runtime::RawPtr, fvisible: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        OleNoteObjectVisible(punknown.into_param().abi(), fvisible.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleQueryCreateFromData<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>>(psrcdataobject: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleQueryCreateFromData(psrcdataobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        OleQueryCreateFromData(psrcdataobject.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleQueryLinkFromData<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>>(psrcdataobject: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleQueryLinkFromData(psrcdataobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        OleQueryLinkFromData(psrcdataobject.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleRegEnumFormatEtc(clsid: *const ::windows::runtime::GUID, dwdirection: u32) -> ::windows::runtime::Result<super::Com::IEnumFORMATETC> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleRegEnumFormatEtc(clsid: *const ::windows::runtime::GUID, dwdirection: u32, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::IEnumFORMATETC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        OleRegEnumFormatEtc(::std::mem::transmute(clsid), ::std::mem::transmute(dwdirection), &mut result__).from_abi::<super::Com::IEnumFORMATETC>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[inline]
pub unsafe fn OleRegEnumVerbs(clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IEnumOLEVERB> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleRegEnumVerbs(clsid: *const ::windows::runtime::GUID, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IEnumOLEVERB as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        OleRegEnumVerbs(::std::mem::transmute(clsid), &mut result__).from_abi::<IEnumOLEVERB>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[inline]
pub unsafe fn OleRegGetMiscStatus(clsid: *const ::windows::runtime::GUID, dwaspect: u32, pdwstatus: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleRegGetMiscStatus(clsid: *const ::windows::runtime::GUID, dwaspect: u32, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT;
        }
        OleRegGetMiscStatus(::std::mem::transmute(clsid), ::std::mem::transmute(dwaspect), ::std::mem::transmute(pdwstatus)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleRegGetUserType(clsid: *const ::windows::runtime::GUID, dwformoftype: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleRegGetUserType(clsid: *const ::windows::runtime::GUID, dwformoftype: u32, pszusertype: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        OleRegGetUserType(::std::mem::transmute(clsid), ::std::mem::transmute(dwformoftype), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[inline]
pub unsafe fn OleRun<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punknown: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleRun(punknown: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        OleRun(punknown.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleSave<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IPersistStorage>, Param1: ::windows::runtime::IntoParam<'a, super::Com::StructuredStorage::IStorage>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(pps: Param0, pstg: Param1, fsameasload: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleSave(pps: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, fsameasload: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        OleSave(pps.into_param().abi(), pstg.into_param().abi(), fsameasload.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn OleSavePictureFile<'a, Param0: ::windows::runtime::IntoParam<'a, Automation::IDispatch>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(lpdisppicture: Param0, bstrfilename: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleSavePictureFile(lpdisppicture: ::windows::runtime::RawPtr, bstrfilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
        }
        OleSavePictureFile(lpdisppicture.into_param().abi(), bstrfilename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleSaveToStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IPersistStream>, Param1: ::windows::runtime::IntoParam<'a, super::Com::IStream>>(ppstm: Param0, pstm: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleSaveToStream(ppstm: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        OleSaveToStream(ppstm.into_param().abi(), pstm.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[inline]
pub unsafe fn OleSetAutoConvert(clsidold: *const ::windows::runtime::GUID, clsidnew: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleSetAutoConvert(clsidold: *const ::windows::runtime::GUID, clsidnew: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        OleSetAutoConvert(::std::mem::transmute(clsidold), ::std::mem::transmute(clsidnew)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleSetClipboard<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IDataObject>>(pdataobj: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleSetClipboard(pdataobj: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        OleSetClipboard(pdataobj.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleSetContainedObject<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(punknown: Param0, fcontained: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleSetContainedObject(punknown: ::windows::runtime::RawPtr, fcontained: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        OleSetContainedObject(punknown.into_param().abi(), fcontained.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleSetMenuDescriptor<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, IOleInPlaceFrame>, Param4: ::windows::runtime::IntoParam<'a, IOleInPlaceActiveObject>>(holemenu: isize, hwndframe: Param1, hwndactiveobject: Param2, lpframe: Param3, lpactiveobj: Param4) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleSetMenuDescriptor(holemenu: isize, hwndframe: super::super::Foundation::HWND, hwndactiveobject: super::super::Foundation::HWND, lpframe: ::windows::runtime::RawPtr, lpactiveobj: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        OleSetMenuDescriptor(::std::mem::transmute(holemenu), hwndframe.into_param().abi(), hwndactiveobject.into_param().abi(), lpframe.into_param().abi(), lpactiveobj.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleTranslateAccelerator<'a, Param0: ::windows::runtime::IntoParam<'a, IOleInPlaceFrame>>(lpframe: Param0, lpframeinfo: *mut OIFI, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleTranslateAccelerator(lpframe: ::windows::runtime::RawPtr, lpframeinfo: *mut OIFI, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::HRESULT;
        }
        OleTranslateAccelerator(lpframe.into_param().abi(), ::std::mem::transmute(lpframeinfo), ::std::mem::transmute(lpmsg)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn OleTranslateColor<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HPALETTE>>(clr: u32, hpal: Param1, lpcolorref: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleTranslateColor(clr: u32, hpal: super::super::Graphics::Gdi::HPALETTE, lpcolorref: *mut u32) -> ::windows::runtime::HRESULT;
        }
        OleTranslateColor(::std::mem::transmute(clr), hpal.into_param().abi(), ::std::mem::transmute(lpcolorref)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleUIAddVerbMenuA<'a, Param0: ::windows::runtime::IntoParam<'a, IOleObject>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::UI::WindowsAndMessaging::HMENU>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
    lpoleobj: Param0,
    lpszshorttype: Param1,
    hmenu: Param2,
    upos: u32,
    uidverbmin: u32,
    uidverbmax: u32,
    baddconvert: Param6,
    idconvert: u32,
    lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIAddVerbMenuA(lpoleobj: ::windows::runtime::RawPtr, lpszshorttype: super::super::Foundation::PSTR, hmenu: super::super::UI::WindowsAndMessaging::HMENU, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: super::super::Foundation::BOOL, idconvert: u32, lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(OleUIAddVerbMenuA(
            lpoleobj.into_param().abi(),
            lpszshorttype.into_param().abi(),
            hmenu.into_param().abi(),
            ::std::mem::transmute(upos),
            ::std::mem::transmute(uidverbmin),
            ::std::mem::transmute(uidverbmax),
            baddconvert.into_param().abi(),
            ::std::mem::transmute(idconvert),
            ::std::mem::transmute(lphmenu),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleUIAddVerbMenuW<'a, Param0: ::windows::runtime::IntoParam<'a, IOleObject>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::UI::WindowsAndMessaging::HMENU>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
    lpoleobj: Param0,
    lpszshorttype: Param1,
    hmenu: Param2,
    upos: u32,
    uidverbmin: u32,
    uidverbmax: u32,
    baddconvert: Param6,
    idconvert: u32,
    lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIAddVerbMenuW(lpoleobj: ::windows::runtime::RawPtr, lpszshorttype: super::super::Foundation::PWSTR, hmenu: super::super::UI::WindowsAndMessaging::HMENU, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: super::super::Foundation::BOOL, idconvert: u32, lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(OleUIAddVerbMenuW(
            lpoleobj.into_param().abi(),
            lpszshorttype.into_param().abi(),
            hmenu.into_param().abi(),
            ::std::mem::transmute(upos),
            ::std::mem::transmute(uidverbmin),
            ::std::mem::transmute(uidverbmax),
            baddconvert.into_param().abi(),
            ::std::mem::transmute(idconvert),
            ::std::mem::transmute(lphmenu),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Media_Audio_CoreAudio`, `Win32_System_LibraryLoader`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
#[inline]
pub unsafe fn OleUIBusyA(param0: *const OLEUIBUSYA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIBusyA(param0: *const ::std::mem::ManuallyDrop<OLEUIBUSYA>) -> u32;
        }
        ::std::mem::transmute(OleUIBusyA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Media_Audio_CoreAudio`, `Win32_System_LibraryLoader`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_LibraryLoader"))]
#[inline]
pub unsafe fn OleUIBusyW(param0: *const OLEUIBUSYW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIBusyW(param0: *const ::std::mem::ManuallyDrop<OLEUIBUSYW>) -> u32;
        }
        ::std::mem::transmute(OleUIBusyW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUICanConvertOrActivateAs<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(rclsid: *const ::windows::runtime::GUID, fislinkedobject: Param1, wformat: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUICanConvertOrActivateAs(rclsid: *const ::windows::runtime::GUID, fislinkedobject: super::super::Foundation::BOOL, wformat: u16) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(OleUICanConvertOrActivateAs(::std::mem::transmute(rclsid), fislinkedobject.into_param().abi(), ::std::mem::transmute(wformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
#[inline]
pub unsafe fn OleUIChangeIconA(param0: *const OLEUICHANGEICONA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIChangeIconA(param0: *const ::std::mem::ManuallyDrop<OLEUICHANGEICONA>) -> u32;
        }
        ::std::mem::transmute(OleUIChangeIconA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
#[inline]
pub unsafe fn OleUIChangeIconW(param0: *const OLEUICHANGEICONW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIChangeIconW(param0: *const ::std::mem::ManuallyDrop<OLEUICHANGEICONW>) -> u32;
        }
        ::std::mem::transmute(OleUIChangeIconW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`, `Win32_UI_Controls_Dialogs`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
#[inline]
pub unsafe fn OleUIChangeSourceA(param0: *const OLEUICHANGESOURCEA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIChangeSourceA(param0: *const ::std::mem::ManuallyDrop<OLEUICHANGESOURCEA>) -> u32;
        }
        ::std::mem::transmute(OleUIChangeSourceA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`, `Win32_UI_Controls_Dialogs`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader", feature = "Win32_UI_Controls_Dialogs"))]
#[inline]
pub unsafe fn OleUIChangeSourceW(param0: *const OLEUICHANGESOURCEW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIChangeSourceW(param0: *const ::std::mem::ManuallyDrop<OLEUICHANGESOURCEW>) -> u32;
        }
        ::std::mem::transmute(OleUIChangeSourceW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
#[inline]
pub unsafe fn OleUIConvertA(param0: *const OLEUICONVERTA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIConvertA(param0: *const ::std::mem::ManuallyDrop<OLEUICONVERTA>) -> u32;
        }
        ::std::mem::transmute(OleUIConvertA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
#[inline]
pub unsafe fn OleUIConvertW(param0: *const OLEUICONVERTW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIConvertW(param0: *const ::std::mem::ManuallyDrop<OLEUICONVERTW>) -> u32;
        }
        ::std::mem::transmute(OleUIConvertW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
#[inline]
pub unsafe fn OleUIEditLinksA(param0: *const OLEUIEDITLINKSA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIEditLinksA(param0: *const ::std::mem::ManuallyDrop<OLEUIEDITLINKSA>) -> u32;
        }
        ::std::mem::transmute(OleUIEditLinksA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_LibraryLoader`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_LibraryLoader"))]
#[inline]
pub unsafe fn OleUIEditLinksW(param0: *const OLEUIEDITLINKSW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIEditLinksW(param0: *const ::std::mem::ManuallyDrop<OLEUIEDITLINKSW>) -> u32;
        }
        ::std::mem::transmute(OleUIEditLinksW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_LibraryLoader`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
#[inline]
pub unsafe fn OleUIInsertObjectA(param0: *const OLEUIINSERTOBJECTA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIInsertObjectA(param0: *const ::std::mem::ManuallyDrop<OLEUIINSERTOBJECTA>) -> u32;
        }
        ::std::mem::transmute(OleUIInsertObjectA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_LibraryLoader`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_LibraryLoader"))]
#[inline]
pub unsafe fn OleUIInsertObjectW(param0: *const OLEUIINSERTOBJECTW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIInsertObjectW(param0: *const ::std::mem::ManuallyDrop<OLEUIINSERTOBJECTW>) -> u32;
        }
        ::std::mem::transmute(OleUIInsertObjectW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleUIObjectPropertiesA(param0: *const OLEUIOBJECTPROPSA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIObjectPropertiesA(param0: *const ::std::mem::ManuallyDrop<OLEUIOBJECTPROPSA>) -> u32;
        }
        ::std::mem::transmute(OleUIObjectPropertiesA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleUIObjectPropertiesW(param0: *const OLEUIOBJECTPROPSW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIObjectPropertiesW(param0: *const ::std::mem::ManuallyDrop<OLEUIOBJECTPROPSW>) -> u32;
        }
        ::std::mem::transmute(OleUIObjectPropertiesW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_LibraryLoader`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
#[inline]
pub unsafe fn OleUIPasteSpecialA(param0: *const OLEUIPASTESPECIALA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIPasteSpecialA(param0: *const ::std::mem::ManuallyDrop<OLEUIPASTESPECIALA>) -> u32;
        }
        ::std::mem::transmute(OleUIPasteSpecialA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_LibraryLoader`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_LibraryLoader"))]
#[inline]
pub unsafe fn OleUIPasteSpecialW(param0: *const OLEUIPASTESPECIALW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIPasteSpecialW(param0: *const ::std::mem::ManuallyDrop<OLEUIPASTESPECIALW>) -> u32;
        }
        ::std::mem::transmute(OleUIPasteSpecialW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIPromptUserA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(ntemplate: i32, hwndparent: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIPromptUserA(ntemplate: i32, hwndparent: super::super::Foundation::HWND) -> i32;
        }
        ::std::mem::transmute(OleUIPromptUserA(::std::mem::transmute(ntemplate), hwndparent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIPromptUserW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(ntemplate: i32, hwndparent: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIPromptUserW(ntemplate: i32, hwndparent: super::super::Foundation::HWND) -> i32;
        }
        ::std::mem::transmute(OleUIPromptUserW(::std::mem::transmute(ntemplate), hwndparent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIUpdateLinksA<'a, Param0: ::windows::runtime::IntoParam<'a, IOleUILinkContainerA>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpoleuilinkcntr: Param0, hwndparent: Param1, lpsztitle: Param2, clinks: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIUpdateLinksA(lpoleuilinkcntr: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, lpsztitle: super::super::Foundation::PSTR, clinks: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(OleUIUpdateLinksA(lpoleuilinkcntr.into_param().abi(), hwndparent.into_param().abi(), lpsztitle.into_param().abi(), ::std::mem::transmute(clinks)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIUpdateLinksW<'a, Param0: ::windows::runtime::IntoParam<'a, IOleUILinkContainerW>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpoleuilinkcntr: Param0, hwndparent: Param1, lpsztitle: Param2, clinks: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUIUpdateLinksW(lpoleuilinkcntr: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, lpsztitle: super::super::Foundation::PWSTR, clinks: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(OleUIUpdateLinksW(lpoleuilinkcntr.into_param().abi(), hwndparent.into_param().abi(), lpsztitle.into_param().abi(), ::std::mem::transmute(clinks)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[inline]
pub unsafe fn OleUninitialize() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleUninitialize();
        }
        ::std::mem::transmute(OleUninitialize())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PAGEACTION_UI(pub i32);
pub const PAGEACTION_UI_DEFAULT: PAGEACTION_UI = PAGEACTION_UI(0i32);
pub const PAGEACTION_UI_MODAL: PAGEACTION_UI = PAGEACTION_UI(1i32);
pub const PAGEACTION_UI_MODELESS: PAGEACTION_UI = PAGEACTION_UI(2i32);
pub const PAGEACTION_UI_SILENT: PAGEACTION_UI = PAGEACTION_UI(3i32);
impl ::std::convert::From<i32> for PAGEACTION_UI {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PAGEACTION_UI {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Ole`*"]
pub struct PAGERANGE {
    pub nFromPage: i32,
    pub nToPage: i32,
}
impl PAGERANGE {}
impl ::std::default::Default for PAGERANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PAGERANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PAGERANGE").field("nFromPage", &self.nFromPage).field("nToPage", &self.nToPage).finish()
    }
}
impl ::std::cmp::PartialEq for PAGERANGE {
    fn eq(&self, other: &Self) -> bool {
        self.nFromPage == other.nFromPage && self.nToPage == other.nToPage
    }
}
impl ::std::cmp::Eq for PAGERANGE {}
unsafe impl ::windows::runtime::Abi for PAGERANGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
pub struct PAGESET {
    pub cbStruct: u32,
    pub fOddPages: super::super::Foundation::BOOL,
    pub fEvenPages: super::super::Foundation::BOOL,
    pub cPageRange: u32,
    pub rgPages: [PAGERANGE; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl PAGESET {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PAGESET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PAGESET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PAGESET").field("cbStruct", &self.cbStruct).field("fOddPages", &self.fOddPages).field("fEvenPages", &self.fEvenPages).field("cPageRange", &self.cPageRange).field("rgPages", &self.rgPages).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PAGESET {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.fOddPages == other.fOddPages && self.fEvenPages == other.fEvenPages && self.cPageRange == other.cPageRange && self.rgPages == other.rgPages
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PAGESET {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PAGESET {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PERPROP_E_FIRST: i32 = -2147220992i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PERPROP_E_LAST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220977i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PERPROP_E_NOPAGEAVAILABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220992i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PERPROP_S_FIRST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(262656i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PERPROP_S_LAST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(262671i32 as _);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub struct PICTDESC {
    pub cbSizeofstruct: u32,
    pub picType: u32,
    pub Anonymous: PICTDESC_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PICTDESC {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PICTDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PICTDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PICTDESC {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PICTDESC {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PICTDESC_0 {
    pub bmp: PICTDESC_0_0,
    pub wmf: PICTDESC_0_3,
    pub icon: PICTDESC_0_2,
    pub emf: PICTDESC_0_1,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PICTDESC_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PICTDESC_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PICTDESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PICTDESC_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PICTDESC_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_0 {
    pub hbitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PICTDESC_0_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PICTDESC_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for PICTDESC_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_bmp_e__Struct").field("hbitmap", &self.hbitmap).field("hpal", &self.hpal).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PICTDESC_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.hbitmap == other.hbitmap && self.hpal == other.hpal
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PICTDESC_0_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PICTDESC_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_1 {
    pub hemf: super::super::Graphics::Gdi::HENHMETAFILE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PICTDESC_0_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PICTDESC_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for PICTDESC_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_emf_e__Struct").field("hemf", &self.hemf).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PICTDESC_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.hemf == other.hemf
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PICTDESC_0_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PICTDESC_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_2 {
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PICTDESC_0_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PICTDESC_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for PICTDESC_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_icon_e__Struct").field("hicon", &self.hicon).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PICTDESC_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.hicon == other.hicon
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PICTDESC_0_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PICTDESC_0_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_3 {
    pub hmeta: super::super::Graphics::Gdi::HMETAFILE,
    pub xExt: i32,
    pub yExt: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PICTDESC_0_3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PICTDESC_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for PICTDESC_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_wmf_e__Struct").field("hmeta", &self.hmeta).field("xExt", &self.xExt).field("yExt", &self.yExt).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PICTDESC_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.hmeta == other.hmeta && self.xExt == other.xExt && self.yExt == other.yExt
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PICTDESC_0_3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PICTDESC_0_3 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PICTYPE_BITMAP: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PICTYPE_ENHMETAFILE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PICTYPE_ICON: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PICTYPE_METAFILE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PICTYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PICTYPE_UNINITIALIZED: i32 = -1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct POINTERINACTIVE(pub i32);
pub const POINTERINACTIVE_ACTIVATEONENTRY: POINTERINACTIVE = POINTERINACTIVE(1i32);
pub const POINTERINACTIVE_DEACTIVATEONLEAVE: POINTERINACTIVE = POINTERINACTIVE(2i32);
pub const POINTERINACTIVE_ACTIVATEONDRAG: POINTERINACTIVE = POINTERINACTIVE(4i32);
impl ::std::convert::From<i32> for POINTERINACTIVE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for POINTERINACTIVE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Ole`*"]
pub struct POINTF {
    pub x: f32,
    pub y: f32,
}
impl POINTF {}
impl ::std::default::Default for POINTF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for POINTF {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POINTF").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::std::cmp::PartialEq for POINTF {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::std::cmp::Eq for POINTF {}
unsafe impl ::windows::runtime::Abi for POINTF {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PRINTFLAG(pub u32);
pub const PRINTFLAG_MAYBOTHERUSER: PRINTFLAG = PRINTFLAG(1u32);
pub const PRINTFLAG_PROMPTUSER: PRINTFLAG = PRINTFLAG(2u32);
pub const PRINTFLAG_USERMAYCHANGEPRINTER: PRINTFLAG = PRINTFLAG(4u32);
pub const PRINTFLAG_RECOMPOSETODEVICE: PRINTFLAG = PRINTFLAG(8u32);
pub const PRINTFLAG_DONTACTUALLYPRINT: PRINTFLAG = PRINTFLAG(16u32);
pub const PRINTFLAG_FORCEPROPERTIES: PRINTFLAG = PRINTFLAG(32u32);
pub const PRINTFLAG_PRINTTOFILE: PRINTFLAG = PRINTFLAG(64u32);
impl ::std::convert::From<u32> for PRINTFLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRINTFLAG {
    type Abi = Self;
}
impl ::std::ops::BitOr for PRINTFLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PRINTFLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PRINTFLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PRINTFLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PRINTFLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
pub struct PROPBAG2 {
    pub dwType: u32,
    pub vt: u16,
    pub cfType: u16,
    pub dwHint: u32,
    pub pstrName: super::super::Foundation::PWSTR,
    pub clsid: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl PROPBAG2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PROPBAG2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PROPBAG2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROPBAG2").field("dwType", &self.dwType).field("vt", &self.vt).field("cfType", &self.cfType).field("dwHint", &self.dwHint).field("pstrName", &self.pstrName).field("clsid", &self.clsid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PROPBAG2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.vt == other.vt && self.cfType == other.cfType && self.dwHint == other.dwHint && self.pstrName == other.pstrName && self.clsid == other.clsid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PROPBAG2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PROPBAG2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPBAG2_TYPE(pub i32);
pub const PROPBAG2_TYPE_UNDEFINED: PROPBAG2_TYPE = PROPBAG2_TYPE(0i32);
pub const PROPBAG2_TYPE_DATA: PROPBAG2_TYPE = PROPBAG2_TYPE(1i32);
pub const PROPBAG2_TYPE_URL: PROPBAG2_TYPE = PROPBAG2_TYPE(2i32);
pub const PROPBAG2_TYPE_OBJECT: PROPBAG2_TYPE = PROPBAG2_TYPE(3i32);
pub const PROPBAG2_TYPE_STREAM: PROPBAG2_TYPE = PROPBAG2_TYPE(4i32);
pub const PROPBAG2_TYPE_STORAGE: PROPBAG2_TYPE = PROPBAG2_TYPE(5i32);
pub const PROPBAG2_TYPE_MONIKER: PROPBAG2_TYPE = PROPBAG2_TYPE(6i32);
impl ::std::convert::From<i32> for PROPBAG2_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPBAG2_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
pub struct PROPPAGEINFO {
    pub cb: u32,
    pub pszTitle: super::super::Foundation::PWSTR,
    pub size: super::super::Foundation::SIZE,
    pub pszDocString: super::super::Foundation::PWSTR,
    pub pszHelpFile: super::super::Foundation::PWSTR,
    pub dwHelpContext: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl PROPPAGEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PROPPAGEINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PROPPAGEINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROPPAGEINFO").field("cb", &self.cb).field("pszTitle", &self.pszTitle).field("size", &self.size).field("pszDocString", &self.pszDocString).field("pszHelpFile", &self.pszHelpFile).field("dwHelpContext", &self.dwHelpContext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PROPPAGEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.pszTitle == other.pszTitle && self.size == other.size && self.pszDocString == other.pszDocString && self.pszHelpFile == other.pszHelpFile && self.dwHelpContext == other.dwHelpContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PROPPAGEINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PROPPAGEINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPPAGESTATUS(pub i32);
pub const PROPPAGESTATUS_DIRTY: PROPPAGESTATUS = PROPPAGESTATUS(1i32);
pub const PROPPAGESTATUS_VALIDATE: PROPPAGESTATUS = PROPPAGESTATUS(2i32);
pub const PROPPAGESTATUS_CLEAN: PROPPAGESTATUS = PROPPAGESTATUS(4i32);
impl ::std::convert::From<i32> for PROPPAGESTATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPPAGESTATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PSF_CHECKDISPLAYASICON: i32 = 8i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PSF_DISABLEDISPLAYASICON: i32 = 16i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PSF_HIDECHANGEICON: i32 = 32i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PSF_NOREFRESHDATAOBJECT: i32 = 128i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PSF_SELECTPASTE: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PSF_SELECTPASTELINK: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PSF_SHOWHELP: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PSF_STAYONCLIPBOARDCHANGE: i32 = 64i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const PS_MAXLINKTYPES: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PictureAttributes(pub i32);
pub const PICTURE_SCALABLE: PictureAttributes = PictureAttributes(1i32);
pub const PICTURE_TRANSPARENT: PictureAttributes = PictureAttributes(2i32);
impl ::std::convert::From<i32> for PictureAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PictureAttributes {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
pub struct QACONTAINER {
    pub cbSize: u32,
    pub pClientSite: ::std::option::Option<IOleClientSite>,
    pub pAdviseSink: ::std::option::Option<IAdviseSinkEx>,
    pub pPropertyNotifySink: ::std::option::Option<IPropertyNotifySink>,
    pub pUnkEventSink: ::std::option::Option<::windows::runtime::IUnknown>,
    pub dwAmbientFlags: u32,
    pub colorFore: u32,
    pub colorBack: u32,
    pub pFont: ::std::option::Option<IFont>,
    pub pUndoMgr: ::std::option::Option<IOleUndoManager>,
    pub dwAppearance: u32,
    pub lcid: i32,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
    pub pBindHost: ::std::option::Option<super::Com::IBindHost>,
    pub pOleControlSite: ::std::option::Option<IOleControlSite>,
    pub pServiceProvider: ::std::option::Option<super::Com::IServiceProvider>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl QACONTAINER {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::std::default::Default for QACONTAINER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for QACONTAINER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("QACONTAINER")
            .field("cbSize", &self.cbSize)
            .field("pClientSite", &self.pClientSite)
            .field("pAdviseSink", &self.pAdviseSink)
            .field("pPropertyNotifySink", &self.pPropertyNotifySink)
            .field("pUnkEventSink", &self.pUnkEventSink)
            .field("dwAmbientFlags", &self.dwAmbientFlags)
            .field("colorFore", &self.colorFore)
            .field("colorBack", &self.colorBack)
            .field("pFont", &self.pFont)
            .field("pUndoMgr", &self.pUndoMgr)
            .field("dwAppearance", &self.dwAppearance)
            .field("lcid", &self.lcid)
            .field("hpal", &self.hpal)
            .field("pBindHost", &self.pBindHost)
            .field("pOleControlSite", &self.pOleControlSite)
            .field("pServiceProvider", &self.pServiceProvider)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for QACONTAINER {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.pClientSite == other.pClientSite
            && self.pAdviseSink == other.pAdviseSink
            && self.pPropertyNotifySink == other.pPropertyNotifySink
            && self.pUnkEventSink == other.pUnkEventSink
            && self.dwAmbientFlags == other.dwAmbientFlags
            && self.colorFore == other.colorFore
            && self.colorBack == other.colorBack
            && self.pFont == other.pFont
            && self.pUndoMgr == other.pUndoMgr
            && self.dwAppearance == other.dwAppearance
            && self.lcid == other.lcid
            && self.hpal == other.hpal
            && self.pBindHost == other.pBindHost
            && self.pOleControlSite == other.pOleControlSite
            && self.pServiceProvider == other.pServiceProvider
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for QACONTAINER {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for QACONTAINER {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct QACONTAINERFLAGS(pub i32);
pub const QACONTAINER_SHOWHATCHING: QACONTAINERFLAGS = QACONTAINERFLAGS(1i32);
pub const QACONTAINER_SHOWGRABHANDLES: QACONTAINERFLAGS = QACONTAINERFLAGS(2i32);
pub const QACONTAINER_USERMODE: QACONTAINERFLAGS = QACONTAINERFLAGS(4i32);
pub const QACONTAINER_DISPLAYASDEFAULT: QACONTAINERFLAGS = QACONTAINERFLAGS(8i32);
pub const QACONTAINER_UIDEAD: QACONTAINERFLAGS = QACONTAINERFLAGS(16i32);
pub const QACONTAINER_AUTOCLIP: QACONTAINERFLAGS = QACONTAINERFLAGS(32i32);
pub const QACONTAINER_MESSAGEREFLECT: QACONTAINERFLAGS = QACONTAINERFLAGS(64i32);
pub const QACONTAINER_SUPPORTSMNEMONICS: QACONTAINERFLAGS = QACONTAINERFLAGS(128i32);
impl ::std::convert::From<i32> for QACONTAINERFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for QACONTAINERFLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Ole`*"]
pub struct QACONTROL {
    pub cbSize: u32,
    pub dwMiscStatus: u32,
    pub dwViewStatus: u32,
    pub dwEventCookie: u32,
    pub dwPropNotifyCookie: u32,
    pub dwPointerActivationPolicy: u32,
}
impl QACONTROL {}
impl ::std::default::Default for QACONTROL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for QACONTROL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("QACONTROL")
            .field("cbSize", &self.cbSize)
            .field("dwMiscStatus", &self.dwMiscStatus)
            .field("dwViewStatus", &self.dwViewStatus)
            .field("dwEventCookie", &self.dwEventCookie)
            .field("dwPropNotifyCookie", &self.dwPropNotifyCookie)
            .field("dwPointerActivationPolicy", &self.dwPointerActivationPolicy)
            .finish()
    }
}
impl ::std::cmp::PartialEq for QACONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMiscStatus == other.dwMiscStatus && self.dwViewStatus == other.dwViewStatus && self.dwEventCookie == other.dwEventCookie && self.dwPropNotifyCookie == other.dwPropNotifyCookie && self.dwPointerActivationPolicy == other.dwPointerActivationPolicy
    }
}
impl ::std::cmp::Eq for QACONTROL {}
unsafe impl ::windows::runtime::Abi for QACONTROL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct READYSTATE(pub i32);
pub const READYSTATE_UNINITIALIZED: READYSTATE = READYSTATE(0i32);
pub const READYSTATE_LOADING: READYSTATE = READYSTATE(1i32);
pub const READYSTATE_LOADED: READYSTATE = READYSTATE(2i32);
pub const READYSTATE_INTERACTIVE: READYSTATE = READYSTATE(3i32);
pub const READYSTATE_COMPLETE: READYSTATE = READYSTATE(4i32);
impl ::std::convert::From<i32> for READYSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for READYSTATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterDragDrop<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, IDropTarget>>(hwnd: Param0, pdroptarget: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDragDrop(hwnd: super::super::Foundation::HWND, pdroptarget: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        RegisterDragDrop(hwnd.into_param().abi(), pdroptarget.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ReleaseStgMedium(param0: *mut super::Com::STGMEDIUM) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseStgMedium(param0: *mut ::std::mem::ManuallyDrop<super::Com::STGMEDIUM>);
        }
        ::std::mem::transmute(ReleaseStgMedium(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RevokeDragDrop<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RevokeDragDrop(hwnd: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
        }
        RevokeDragDrop(hwnd.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const SELFREG_E_CLASS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220991i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const SELFREG_E_FIRST: i32 = -2147220992i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const SELFREG_E_LAST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220977i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const SELFREG_E_TYPELIB: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220992i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const SELFREG_S_FIRST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(262656i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const SELFREG_S_LAST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(262671i32 as _);
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const TIFLAGS_EXTENDDISPATCHONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UASFLAGS(pub i32);
pub const UAS_NORMAL: UASFLAGS = UASFLAGS(0i32);
pub const UAS_BLOCKED: UASFLAGS = UASFLAGS(1i32);
pub const UAS_NOPARENTENABLE: UASFLAGS = UASFLAGS(2i32);
pub const UAS_MASK: UASFLAGS = UASFLAGS(3i32);
impl ::std::convert::From<i32> for UASFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UASFLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UPDFCACHE_FLAGS(pub u32);
pub const UPDFCACHE_ALL: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2147483647u32);
pub const UPDFCACHE_ALLBUTNODATACACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2147483646u32);
pub const UPDFCACHE_NORMALCACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(8u32);
pub const UPDFCACHE_IFBLANK: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(16u32);
pub const UPDFCACHE_ONLYIFBLANK: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2147483648u32);
pub const UPDFCACHE_NODATACACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(1u32);
pub const UPDFCACHE_ONSAVECACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2u32);
pub const UPDFCACHE_ONSTOPCACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(4u32);
pub const UPDFCACHE_IFBLANKORONSAVECACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(18u32);
impl ::std::convert::From<u32> for UPDFCACHE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UPDFCACHE_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for UPDFCACHE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for UPDFCACHE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for UPDFCACHE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for UPDFCACHE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for UPDFCACHE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct USERCLASSTYPE(pub i32);
pub const USERCLASSTYPE_FULL: USERCLASSTYPE = USERCLASSTYPE(1i32);
pub const USERCLASSTYPE_SHORT: USERCLASSTYPE = USERCLASSTYPE(2i32);
pub const USERCLASSTYPE_APPNAME: USERCLASSTYPE = USERCLASSTYPE(3i32);
impl ::std::convert::From<i32> for USERCLASSTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for USERCLASSTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VIEWSTATUS(pub i32);
pub const VIEWSTATUS_OPAQUE: VIEWSTATUS = VIEWSTATUS(1i32);
pub const VIEWSTATUS_SOLIDBKGND: VIEWSTATUS = VIEWSTATUS(2i32);
pub const VIEWSTATUS_DVASPECTOPAQUE: VIEWSTATUS = VIEWSTATUS(4i32);
pub const VIEWSTATUS_DVASPECTTRANSPARENT: VIEWSTATUS = VIEWSTATUS(8i32);
pub const VIEWSTATUS_SURFACE: VIEWSTATUS = VIEWSTATUS(16i32);
pub const VIEWSTATUS_3DSURFACE: VIEWSTATUS = VIEWSTATUS(32i32);
impl ::std::convert::From<i32> for VIEWSTATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VIEWSTATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const VPF_DISABLERELATIVE: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const VPF_DISABLESCALE: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const VPF_SELECTRELATIVE: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const VT_BLOB_PROPSET: u32 = 75u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const VT_STORED_PROPSET: u32 = 74u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const VT_STREAMED_PROPSET: u32 = 73u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const VT_VERBOSE_ENUM: u32 = 76u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
pub const WIN32: u32 = 100u32;
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPCSETTING(pub i32);
pub const WPCSETTING_LOGGING_ENABLED: WPCSETTING = WPCSETTING(1i32);
pub const WPCSETTING_FILEDOWNLOAD_BLOCKED: WPCSETTING = WPCSETTING(2i32);
impl ::std::convert::From<i32> for WPCSETTING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPCSETTING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Ole`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XFORMCOORDS(pub i32);
pub const XFORMCOORDS_POSITION: XFORMCOORDS = XFORMCOORDS(1i32);
pub const XFORMCOORDS_SIZE: XFORMCOORDS = XFORMCOORDS(2i32);
pub const XFORMCOORDS_HIMETRICTOCONTAINER: XFORMCOORDS = XFORMCOORDS(4i32);
pub const XFORMCOORDS_CONTAINERTOHIMETRIC: XFORMCOORDS = XFORMCOORDS(8i32);
pub const XFORMCOORDS_EVENTCOMPAT: XFORMCOORDS = XFORMCOORDS(16i32);
impl ::std::convert::From<i32> for XFORMCOORDS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XFORMCOORDS {
    type Abi = Self;
}
