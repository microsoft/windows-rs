#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_FIRST: u32 = 1124u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_GETFILEPATH: u32 = 1125u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_GETFOLDERIDLIST: u32 = 1127u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_GETFOLDERPATH: u32 = 1126u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_GETSPEC: u32 = 1124u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_HIDECONTROL: u32 = 1129u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_LAST: u32 = 1224u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_SETCONTROLTEXT: u32 = 1128u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_SETDEFEXT: u32 = 1130u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CD_LBSELADD: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CD_LBSELCHANGE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CD_LBSELNOITEMS: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CD_LBSELSUB: u32 = 1u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: u32,
    pub lpCustColors: *mut u32,
    pub Flags: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: ::std::option::Option<LPCCHOOKPROC>,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CHOOSECOLORA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CHOOSECOLORA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CHOOSECOLORA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CHOOSECOLORA")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("rgbResult", &self.rgbResult)
            .field("lpCustColors", &self.lpCustColors)
            .field("Flags", &self.Flags)
            .field("lCustData", &self.lCustData)
            .field("lpTemplateName", &self.lpTemplateName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CHOOSECOLORA {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize && self.hwndOwner == other.hwndOwner && self.hInstance == other.hInstance && self.rgbResult == other.rgbResult && self.lpCustColors == other.lpCustColors && self.Flags == other.Flags && self.lCustData == other.lCustData && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lpTemplateName == other.lpTemplateName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CHOOSECOLORA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CHOOSECOLORA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: u32,
    pub lpCustColors: *mut u32,
    pub Flags: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: ::std::option::Option<LPCCHOOKPROC>,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CHOOSECOLORW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CHOOSECOLORW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CHOOSECOLORW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CHOOSECOLORW")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("rgbResult", &self.rgbResult)
            .field("lpCustColors", &self.lpCustColors)
            .field("Flags", &self.Flags)
            .field("lCustData", &self.lCustData)
            .field("lpTemplateName", &self.lpTemplateName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CHOOSECOLORW {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize && self.hwndOwner == other.hwndOwner && self.hInstance == other.hInstance && self.rgbResult == other.rgbResult && self.lpCustColors == other.lpCustColors && self.Flags == other.Flags && self.lCustData == other.lCustData && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lpTemplateName == other.lpTemplateName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CHOOSECOLORW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CHOOSECOLORW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct CHOOSEFONTA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTA,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: ::std::option::Option<LPCFHOOKPROC>,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: super::super::super::Foundation::PSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl CHOOSEFONTA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for CHOOSEFONTA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for CHOOSEFONTA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CHOOSEFONTA")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDC", &self.hDC)
            .field("lpLogFont", &self.lpLogFont)
            .field("iPointSize", &self.iPointSize)
            .field("Flags", &self.Flags)
            .field("rgbColors", &self.rgbColors)
            .field("lCustData", &self.lCustData)
            .field("lpTemplateName", &self.lpTemplateName)
            .field("hInstance", &self.hInstance)
            .field("lpszStyle", &self.lpszStyle)
            .field("nFontType", &self.nFontType)
            .field("___MISSING_ALIGNMENT__", &self.___MISSING_ALIGNMENT__)
            .field("nSizeMin", &self.nSizeMin)
            .field("nSizeMax", &self.nSizeMax)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for CHOOSEFONTA {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hDC == other.hDC
            && self.lpLogFont == other.lpLogFont
            && self.iPointSize == other.iPointSize
            && self.Flags == other.Flags
            && self.rgbColors == other.rgbColors
            && self.lCustData == other.lCustData
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lpTemplateName == other.lpTemplateName
            && self.hInstance == other.hInstance
            && self.lpszStyle == other.lpszStyle
            && self.nFontType == other.nFontType
            && self.___MISSING_ALIGNMENT__ == other.___MISSING_ALIGNMENT__
            && self.nSizeMin == other.nSizeMin
            && self.nSizeMax == other.nSizeMax
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for CHOOSEFONTA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for CHOOSEFONTA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct CHOOSEFONTW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTW,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: ::std::option::Option<LPCFHOOKPROC>,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: super::super::super::Foundation::PWSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl CHOOSEFONTW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for CHOOSEFONTW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for CHOOSEFONTW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CHOOSEFONTW")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDC", &self.hDC)
            .field("lpLogFont", &self.lpLogFont)
            .field("iPointSize", &self.iPointSize)
            .field("Flags", &self.Flags)
            .field("rgbColors", &self.rgbColors)
            .field("lCustData", &self.lCustData)
            .field("lpTemplateName", &self.lpTemplateName)
            .field("hInstance", &self.hInstance)
            .field("lpszStyle", &self.lpszStyle)
            .field("nFontType", &self.nFontType)
            .field("___MISSING_ALIGNMENT__", &self.___MISSING_ALIGNMENT__)
            .field("nSizeMin", &self.nSizeMin)
            .field("nSizeMax", &self.nSizeMax)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for CHOOSEFONTW {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hDC == other.hDC
            && self.lpLogFont == other.lpLogFont
            && self.iPointSize == other.iPointSize
            && self.Flags == other.Flags
            && self.rgbColors == other.rgbColors
            && self.lCustData == other.lCustData
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lpTemplateName == other.lpTemplateName
            && self.hInstance == other.hInstance
            && self.lpszStyle == other.lpszStyle
            && self.nFontType == other.nFontType
            && self.___MISSING_ALIGNMENT__ == other.___MISSING_ALIGNMENT__
            && self.nSizeMin == other.nSizeMin
            && self.nSizeMax == other.nSizeMax
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for CHOOSEFONTW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for CHOOSEFONTW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CHOOSEFONT_FLAGS(pub u32);
pub const CF_APPLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(512u32);
pub const CF_ANSIONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1024u32);
pub const CF_BOTH: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(3u32);
pub const CF_EFFECTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(256u32);
pub const CF_ENABLEHOOK: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(8u32);
pub const CF_ENABLETEMPLATE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(16u32);
pub const CF_ENABLETEMPLATEHANDLE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(32u32);
pub const CF_FIXEDPITCHONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(16384u32);
pub const CF_FORCEFONTEXIST: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(65536u32);
pub const CF_INACTIVEFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(33554432u32);
pub const CF_INITTOLOGFONTSTRUCT: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(64u32);
pub const CF_LIMITSIZE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(8192u32);
pub const CF_NOOEMFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2048u32);
pub const CF_NOFACESEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(524288u32);
pub const CF_NOSCRIPTSEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(8388608u32);
pub const CF_NOSIMULATIONS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(4096u32);
pub const CF_NOSIZESEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2097152u32);
pub const CF_NOSTYLESEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1048576u32);
pub const CF_NOVECTORFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2048u32);
pub const CF_NOVERTFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(16777216u32);
pub const CF_PRINTERFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2u32);
pub const CF_SCALABLEONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(131072u32);
pub const CF_SCREENFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1u32);
pub const CF_SCRIPTSONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1024u32);
pub const CF_SELECTSCRIPT: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(4194304u32);
pub const CF_SHOWHELP: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(4u32);
pub const CF_TTONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(262144u32);
pub const CF_USESTYLE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(128u32);
pub const CF_WYSIWYG: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(32768u32);
impl ::std::convert::From<u32> for CHOOSEFONT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CHOOSEFONT_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for CHOOSEFONT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CHOOSEFONT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CHOOSEFONT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CHOOSEFONT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CHOOSEFONT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CHOOSEFONT_FONT_TYPE(pub u16);
pub const BOLD_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(256u16);
pub const ITALIC_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(512u16);
pub const PRINTER_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(16384u16);
pub const REGULAR_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(1024u16);
pub const SCREEN_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(8192u16);
pub const SIMULATED_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(32768u16);
impl ::std::convert::From<u16> for CHOOSEFONT_FONT_TYPE {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CHOOSEFONT_FONT_TYPE {
    type Abi = Self;
}
impl ::std::ops::BitOr for CHOOSEFONT_FONT_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CHOOSEFONT_FONT_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CHOOSEFONT_FONT_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CHOOSEFONT_FONT_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CHOOSEFONT_FONT_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_ADD: u32 = 712u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_BLUE: u32 = 708u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_BLUEACCEL: u32 = 728u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_BOX1: u32 = 720u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_CURRENT: u32 = 709u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_CUSTOM1: u32 = 721u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_ELEMENT: u32 = 716u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_GREEN: u32 = 707u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_GREENACCEL: u32 = 727u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_HUE: u32 = 703u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_HUEACCEL: u32 = 723u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_HUESCROLL: u32 = 700u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_LUM: u32 = 705u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_LUMACCEL: u32 = 725u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_LUMSCROLL: u32 = 702u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_MIX: u32 = 719u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_PALETTE: u32 = 718u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_RAINBOW: u32 = 710u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_RED: u32 = 706u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_REDACCEL: u32 = 726u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SAMPLES: u32 = 717u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SAT: u32 = 704u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SATACCEL: u32 = 724u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SATSCROLL: u32 = 701u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SAVE: u32 = 711u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SCHEMES: u32 = 715u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SOLID: u32 = 713u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SOLID_LEFT: u32 = 730u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SOLID_RIGHT: u32 = 731u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_TUNE: u32 = 714u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMMON_DLG_ERRORS(pub u32);
pub const CDERR_DIALOGFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(65535u32);
pub const CDERR_GENERALCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(0u32);
pub const CDERR_STRUCTSIZE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(1u32);
pub const CDERR_INITIALIZATION: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(2u32);
pub const CDERR_NOTEMPLATE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(3u32);
pub const CDERR_NOHINSTANCE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4u32);
pub const CDERR_LOADSTRFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(5u32);
pub const CDERR_FINDRESFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(6u32);
pub const CDERR_LOADRESFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(7u32);
pub const CDERR_LOCKRESFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8u32);
pub const CDERR_MEMALLOCFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(9u32);
pub const CDERR_MEMLOCKFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(10u32);
pub const CDERR_NOHOOK: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(11u32);
pub const CDERR_REGISTERMSGFAIL: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12u32);
pub const PDERR_PRINTERCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4096u32);
pub const PDERR_SETUPFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4097u32);
pub const PDERR_PARSEFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4098u32);
pub const PDERR_RETDEFFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4099u32);
pub const PDERR_LOADDRVFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4100u32);
pub const PDERR_GETDEVMODEFAIL: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4101u32);
pub const PDERR_INITFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4102u32);
pub const PDERR_NODEVICES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4103u32);
pub const PDERR_NODEFAULTPRN: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4104u32);
pub const PDERR_DNDMMISMATCH: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4105u32);
pub const PDERR_CREATEICFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4106u32);
pub const PDERR_PRINTERNOTFOUND: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4107u32);
pub const PDERR_DEFAULTDIFFERENT: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4108u32);
pub const CFERR_CHOOSEFONTCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8192u32);
pub const CFERR_NOFONTS: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8193u32);
pub const CFERR_MAXLESSTHANMIN: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8194u32);
pub const FNERR_FILENAMECODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12288u32);
pub const FNERR_SUBCLASSFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12289u32);
pub const FNERR_INVALIDFILENAME: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12290u32);
pub const FNERR_BUFFERTOOSMALL: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12291u32);
pub const FRERR_FINDREPLACECODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(16384u32);
pub const FRERR_BUFFERLENGTHZERO: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(16385u32);
pub const CCERR_CHOOSECOLORCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(20480u32);
impl ::std::convert::From<u32> for COMMON_DLG_ERRORS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMMON_DLG_ERRORS {
    type Abi = Self;
}
impl ::std::ops::BitOr for COMMON_DLG_ERRORS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for COMMON_DLG_ERRORS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for COMMON_DLG_ERRORS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for COMMON_DLG_ERRORS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for COMMON_DLG_ERRORS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn ChooseColorA(param0: *mut CHOOSECOLORA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChooseColorA(param0: *mut ::std::mem::ManuallyDrop<CHOOSECOLORA>) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ChooseColorA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn ChooseColorW(param0: *mut CHOOSECOLORW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChooseColorW(param0: *mut ::std::mem::ManuallyDrop<CHOOSECOLORW>) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ChooseColorW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[inline]
pub unsafe fn ChooseFontA(param0: *mut CHOOSEFONTA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChooseFontA(param0: *mut ::std::mem::ManuallyDrop<CHOOSEFONTA>) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ChooseFontA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[inline]
pub unsafe fn ChooseFontW(param0: *mut CHOOSEFONTW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChooseFontW(param0: *mut ::std::mem::ManuallyDrop<CHOOSEFONTW>) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ChooseFontW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
#[inline]
pub unsafe fn CommDlgExtendedError() -> COMMON_DLG_ERRORS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommDlgExtendedError() -> COMMON_DLG_ERRORS;
        }
        ::std::mem::transmute(CommDlgExtendedError())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub struct DEVNAMES {
    pub wDriverOffset: u16,
    pub wDeviceOffset: u16,
    pub wOutputOffset: u16,
    pub wDefault: u16,
}
impl DEVNAMES {}
impl ::std::default::Default for DEVNAMES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DEVNAMES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DEVNAMES").field("wDriverOffset", &self.wDriverOffset).field("wDeviceOffset", &self.wDeviceOffset).field("wOutputOffset", &self.wOutputOffset).field("wDefault", &self.wDefault).finish()
    }
}
impl ::std::cmp::PartialEq for DEVNAMES {
    fn eq(&self, other: &Self) -> bool {
        self.wDriverOffset == other.wDriverOffset && self.wDeviceOffset == other.wDeviceOffset && self.wOutputOffset == other.wOutputOffset && self.wDefault == other.wDefault
    }
}
impl ::std::cmp::Eq for DEVNAMES {}
unsafe impl ::windows::runtime::Abi for DEVNAMES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const DLG_COLOR: u32 = 10u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const DN_DEFAULTPRN: u32 = 1u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: super::super::super::Foundation::PSTR,
    pub lpstrReplaceWith: super::super::super::Foundation::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: ::std::option::Option<LPFRHOOKPROC>,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FINDREPLACEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FINDREPLACEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FINDREPLACEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FINDREPLACEA")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("Flags", &self.Flags)
            .field("lpstrFindWhat", &self.lpstrFindWhat)
            .field("lpstrReplaceWith", &self.lpstrReplaceWith)
            .field("wFindWhatLen", &self.wFindWhatLen)
            .field("wReplaceWithLen", &self.wReplaceWithLen)
            .field("lCustData", &self.lCustData)
            .field("lpTemplateName", &self.lpTemplateName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FINDREPLACEA {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize && self.hwndOwner == other.hwndOwner && self.hInstance == other.hInstance && self.Flags == other.Flags && self.lpstrFindWhat == other.lpstrFindWhat && self.lpstrReplaceWith == other.lpstrReplaceWith && self.wFindWhatLen == other.wFindWhatLen && self.wReplaceWithLen == other.wReplaceWithLen && self.lCustData == other.lCustData && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lpTemplateName == other.lpTemplateName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FINDREPLACEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FINDREPLACEA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: super::super::super::Foundation::PWSTR,
    pub lpstrReplaceWith: super::super::super::Foundation::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: ::std::option::Option<LPFRHOOKPROC>,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FINDREPLACEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FINDREPLACEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FINDREPLACEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FINDREPLACEW")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("Flags", &self.Flags)
            .field("lpstrFindWhat", &self.lpstrFindWhat)
            .field("lpstrReplaceWith", &self.lpstrReplaceWith)
            .field("wFindWhatLen", &self.wFindWhatLen)
            .field("wReplaceWithLen", &self.wReplaceWithLen)
            .field("lCustData", &self.lCustData)
            .field("lpTemplateName", &self.lpTemplateName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FINDREPLACEW {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize && self.hwndOwner == other.hwndOwner && self.hInstance == other.hInstance && self.Flags == other.Flags && self.lpstrFindWhat == other.lpstrFindWhat && self.lpstrReplaceWith == other.lpstrReplaceWith && self.wFindWhatLen == other.wFindWhatLen && self.wReplaceWithLen == other.wReplaceWithLen && self.lCustData == other.lCustData && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize) && self.lpTemplateName == other.lpTemplateName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FINDREPLACEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FINDREPLACEW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FINDREPLACE_FLAGS(pub u32);
pub const FR_DIALOGTERM: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(64u32);
pub const FR_DOWN: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(1u32);
pub const FR_ENABLEHOOK: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(256u32);
pub const FR_ENABLETEMPLATE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(512u32);
pub const FR_ENABLETEMPLATEHANDLE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(8192u32);
pub const FR_FINDNEXT: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(8u32);
pub const FR_HIDEUPDOWN: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(16384u32);
pub const FR_HIDEMATCHCASE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(32768u32);
pub const FR_HIDEWHOLEWORD: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(65536u32);
pub const FR_MATCHCASE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(4u32);
pub const FR_NOMATCHCASE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(2048u32);
pub const FR_NOUPDOWN: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(1024u32);
pub const FR_NOWHOLEWORD: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(4096u32);
pub const FR_REPLACE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(16u32);
pub const FR_REPLACEALL: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(32u32);
pub const FR_SHOWHELP: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(128u32);
pub const FR_WHOLEWORD: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(2u32);
impl ::std::convert::From<u32> for FINDREPLACE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FINDREPLACE_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for FINDREPLACE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FINDREPLACE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FINDREPLACE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FINDREPLACE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FINDREPLACE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FRM_FIRST: u32 = 1124u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FRM_LAST: u32 = 1224u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FRM_SETOPERATIONRESULT: u32 = 1124u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FRM_SETOPERATIONRESULTTEXT: u32 = 1125u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FR_NOWRAPAROUND: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FR_RAW: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FR_SHOWWRAPAROUND: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FR_WRAPAROUND: u32 = 1048576u32;
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn FindTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindTextA(param0: *mut ::std::mem::ManuallyDrop<FINDREPLACEA>) -> super::super::super::Foundation::HWND;
        }
        ::std::mem::transmute(FindTextA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn FindTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindTextW(param0: *mut ::std::mem::ManuallyDrop<FINDREPLACEW>) -> super::super::super::Foundation::HWND;
        }
        ::std::mem::transmute(FindTextW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetFileTitleA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(param0: Param0, buf: super::super::super::Foundation::PSTR, cchsize: u16) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileTitleA(param0: super::super::super::Foundation::PSTR, buf: super::super::super::Foundation::PSTR, cchsize: u16) -> i16;
        }
        ::std::mem::transmute(GetFileTitleA(param0.into_param().abi(), ::std::mem::transmute(buf), ::std::mem::transmute(cchsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetFileTitleW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(param0: Param0, buf: super::super::super::Foundation::PWSTR, cchsize: u16) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileTitleW(param0: super::super::super::Foundation::PWSTR, buf: super::super::super::Foundation::PWSTR, cchsize: u16) -> i16;
        }
        ::std::mem::transmute(GetFileTitleW(param0.into_param().abi(), ::std::mem::transmute(buf), ::std::mem::transmute(cchsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetOpenFileNameA(param0: *mut OPENFILENAMEA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOpenFileNameA(param0: *mut ::std::mem::ManuallyDrop<OPENFILENAMEA>) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetOpenFileNameA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetOpenFileNameW(param0: *mut OPENFILENAMEW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOpenFileNameW(param0: *mut ::std::mem::ManuallyDrop<OPENFILENAMEW>) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetOpenFileNameW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetSaveFileNameA(param0: *mut OPENFILENAMEA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSaveFileNameA(param0: *mut ::std::mem::ManuallyDrop<OPENFILENAMEA>) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSaveFileNameA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetSaveFileNameW(param0: *mut OPENFILENAMEW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSaveFileNameW(param0: *mut ::std::mem::ManuallyDrop<OPENFILENAMEW>) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSaveFileNameW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPrintDialogCallback(::windows::runtime::IUnknown);
impl IPrintDialogCallback {
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
    pub unsafe fn InitDone(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
    pub unsafe fn SelectionChange(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    pub unsafe fn HandleMessage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hdlg: Param0, umsg: u32, wparam: Param2, lparam: Param3, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), hdlg.into_param().abi(), ::std::mem::transmute(umsg), wparam.into_param().abi(), lparam.into_param().abi(), ::std::mem::transmute(presult)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPrintDialogCallback {
    type Vtable = IPrintDialogCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1481810627, 25904, 4561, [182, 163, 0, 0, 248, 117, 123, 249]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDialogCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPrintDialogServices(::windows::runtime::IUnknown);
impl IPrintDialogServices {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetCurrentDevMode(&self, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdevmode), ::std::mem::transmute(pcbsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    pub unsafe fn GetCurrentPrinterName(&self, pprintername: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprintername), ::std::mem::transmute(pcchsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    pub unsafe fn GetCurrentPortName(&self, pportname: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pportname), ::std::mem::transmute(pcchsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPrintDialogServices {
    type Vtable = IPrintDialogServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1352314586, 22073, 4561, [182, 161, 0, 0, 248, 117, 123, 249]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDialogServices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprintername: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pportname: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[cfg(feature = "Win32_Foundation")]
pub type LPCCHOOKPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPCFHOOKPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPFRHOOKPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPOFNHOOKPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPPAGEPAINTHOOK = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPPAGESETUPHOOK = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPPRINTHOOKPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPSETUPHOOKPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const NUM_BASIC_COLORS: u32 = 48u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const NUM_CUSTOM_COLORS: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct OFNOTIFYA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub pszFile: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl OFNOTIFYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OFNOTIFYA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OFNOTIFYA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OFNOTIFYA").field("hdr", &self.hdr).field("lpOFN", &self.lpOFN).field("pszFile", &self.pszFile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OFNOTIFYA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.lpOFN == other.lpOFN && self.pszFile == other.pszFile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OFNOTIFYA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OFNOTIFYA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct OFNOTIFYEXA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub psf: *mut ::std::ffi::c_void,
    pub pidl: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl OFNOTIFYEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OFNOTIFYEXA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OFNOTIFYEXA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OFNOTIFYEXA").field("hdr", &self.hdr).field("lpOFN", &self.lpOFN).field("psf", &self.psf).field("pidl", &self.pidl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OFNOTIFYEXA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.lpOFN == other.lpOFN && self.psf == other.psf && self.pidl == other.pidl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OFNOTIFYEXA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OFNOTIFYEXA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct OFNOTIFYEXW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub psf: *mut ::std::ffi::c_void,
    pub pidl: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl OFNOTIFYEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OFNOTIFYEXW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OFNOTIFYEXW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OFNOTIFYEXW").field("hdr", &self.hdr).field("lpOFN", &self.lpOFN).field("psf", &self.psf).field("pidl", &self.pidl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OFNOTIFYEXW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.lpOFN == other.lpOFN && self.psf == other.psf && self.pidl == other.pidl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OFNOTIFYEXW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OFNOTIFYEXW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct OFNOTIFYW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub pszFile: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl OFNOTIFYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OFNOTIFYW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OFNOTIFYW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OFNOTIFYW").field("hdr", &self.hdr).field("lpOFN", &self.lpOFN).field("pszFile", &self.pszFile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OFNOTIFYW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.lpOFN == other.lpOFN && self.pszFile == other.pszFile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OFNOTIFYW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OFNOTIFYW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const OFN_SHAREFALLTHROUGH: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const OFN_SHARENOWARN: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const OFN_SHAREWARN: u32 = 0u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PSTR,
    pub lpstrTitle: super::super::super::Foundation::PSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: ::std::option::Option<LPOFNHOOKPROC>,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
    pub pvReserved: *mut ::std::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl OPENFILENAMEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OPENFILENAMEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OPENFILENAMEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OPENFILENAMEA")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("lpstrFilter", &self.lpstrFilter)
            .field("lpstrCustomFilter", &self.lpstrCustomFilter)
            .field("nMaxCustFilter", &self.nMaxCustFilter)
            .field("nFilterIndex", &self.nFilterIndex)
            .field("lpstrFile", &self.lpstrFile)
            .field("nMaxFile", &self.nMaxFile)
            .field("lpstrFileTitle", &self.lpstrFileTitle)
            .field("nMaxFileTitle", &self.nMaxFileTitle)
            .field("lpstrInitialDir", &self.lpstrInitialDir)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("Flags", &self.Flags)
            .field("nFileOffset", &self.nFileOffset)
            .field("nFileExtension", &self.nFileExtension)
            .field("lpstrDefExt", &self.lpstrDefExt)
            .field("lCustData", &self.lCustData)
            .field("lpTemplateName", &self.lpTemplateName)
            .field("pvReserved", &self.pvReserved)
            .field("dwReserved", &self.dwReserved)
            .field("FlagsEx", &self.FlagsEx)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OPENFILENAMEA {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hInstance == other.hInstance
            && self.lpstrFilter == other.lpstrFilter
            && self.lpstrCustomFilter == other.lpstrCustomFilter
            && self.nMaxCustFilter == other.nMaxCustFilter
            && self.nFilterIndex == other.nFilterIndex
            && self.lpstrFile == other.lpstrFile
            && self.nMaxFile == other.nMaxFile
            && self.lpstrFileTitle == other.lpstrFileTitle
            && self.nMaxFileTitle == other.nMaxFileTitle
            && self.lpstrInitialDir == other.lpstrInitialDir
            && self.lpstrTitle == other.lpstrTitle
            && self.Flags == other.Flags
            && self.nFileOffset == other.nFileOffset
            && self.nFileExtension == other.nFileExtension
            && self.lpstrDefExt == other.lpstrDefExt
            && self.lCustData == other.lCustData
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lpTemplateName == other.lpTemplateName
            && self.pvReserved == other.pvReserved
            && self.dwReserved == other.dwReserved
            && self.FlagsEx == other.FlagsEx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OPENFILENAMEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OPENFILENAMEA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PWSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PWSTR,
    pub lpstrTitle: super::super::super::Foundation::PWSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: ::std::option::Option<LPOFNHOOKPROC>,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
    pub pvReserved: *mut ::std::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl OPENFILENAMEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OPENFILENAMEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OPENFILENAMEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OPENFILENAMEW")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("lpstrFilter", &self.lpstrFilter)
            .field("lpstrCustomFilter", &self.lpstrCustomFilter)
            .field("nMaxCustFilter", &self.nMaxCustFilter)
            .field("nFilterIndex", &self.nFilterIndex)
            .field("lpstrFile", &self.lpstrFile)
            .field("nMaxFile", &self.nMaxFile)
            .field("lpstrFileTitle", &self.lpstrFileTitle)
            .field("nMaxFileTitle", &self.nMaxFileTitle)
            .field("lpstrInitialDir", &self.lpstrInitialDir)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("Flags", &self.Flags)
            .field("nFileOffset", &self.nFileOffset)
            .field("nFileExtension", &self.nFileExtension)
            .field("lpstrDefExt", &self.lpstrDefExt)
            .field("lCustData", &self.lCustData)
            .field("lpTemplateName", &self.lpTemplateName)
            .field("pvReserved", &self.pvReserved)
            .field("dwReserved", &self.dwReserved)
            .field("FlagsEx", &self.FlagsEx)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OPENFILENAMEW {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hInstance == other.hInstance
            && self.lpstrFilter == other.lpstrFilter
            && self.lpstrCustomFilter == other.lpstrCustomFilter
            && self.nMaxCustFilter == other.nMaxCustFilter
            && self.nFilterIndex == other.nFilterIndex
            && self.lpstrFile == other.lpstrFile
            && self.nMaxFile == other.nMaxFile
            && self.lpstrFileTitle == other.lpstrFileTitle
            && self.nMaxFileTitle == other.nMaxFileTitle
            && self.lpstrInitialDir == other.lpstrInitialDir
            && self.lpstrTitle == other.lpstrTitle
            && self.Flags == other.Flags
            && self.nFileOffset == other.nFileOffset
            && self.nFileExtension == other.nFileExtension
            && self.lpstrDefExt == other.lpstrDefExt
            && self.lCustData == other.lCustData
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lpTemplateName == other.lpTemplateName
            && self.pvReserved == other.pvReserved
            && self.dwReserved == other.dwReserved
            && self.FlagsEx == other.FlagsEx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OPENFILENAMEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OPENFILENAMEW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PSTR,
    pub lpstrTitle: super::super::super::Foundation::PSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: ::std::option::Option<LPOFNHOOKPROC>,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl OPENFILENAME_NT4A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OPENFILENAME_NT4A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OPENFILENAME_NT4A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OPENFILENAME_NT4A")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("lpstrFilter", &self.lpstrFilter)
            .field("lpstrCustomFilter", &self.lpstrCustomFilter)
            .field("nMaxCustFilter", &self.nMaxCustFilter)
            .field("nFilterIndex", &self.nFilterIndex)
            .field("lpstrFile", &self.lpstrFile)
            .field("nMaxFile", &self.nMaxFile)
            .field("lpstrFileTitle", &self.lpstrFileTitle)
            .field("nMaxFileTitle", &self.nMaxFileTitle)
            .field("lpstrInitialDir", &self.lpstrInitialDir)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("Flags", &self.Flags)
            .field("nFileOffset", &self.nFileOffset)
            .field("nFileExtension", &self.nFileExtension)
            .field("lpstrDefExt", &self.lpstrDefExt)
            .field("lCustData", &self.lCustData)
            .field("lpTemplateName", &self.lpTemplateName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OPENFILENAME_NT4A {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hInstance == other.hInstance
            && self.lpstrFilter == other.lpstrFilter
            && self.lpstrCustomFilter == other.lpstrCustomFilter
            && self.nMaxCustFilter == other.nMaxCustFilter
            && self.nFilterIndex == other.nFilterIndex
            && self.lpstrFile == other.lpstrFile
            && self.nMaxFile == other.nMaxFile
            && self.lpstrFileTitle == other.lpstrFileTitle
            && self.nMaxFileTitle == other.nMaxFileTitle
            && self.lpstrInitialDir == other.lpstrInitialDir
            && self.lpstrTitle == other.lpstrTitle
            && self.Flags == other.Flags
            && self.nFileOffset == other.nFileOffset
            && self.nFileExtension == other.nFileExtension
            && self.lpstrDefExt == other.lpstrDefExt
            && self.lCustData == other.lCustData
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lpTemplateName == other.lpTemplateName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OPENFILENAME_NT4A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OPENFILENAME_NT4A {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PWSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PWSTR,
    pub lpstrTitle: super::super::super::Foundation::PWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: ::std::option::Option<LPOFNHOOKPROC>,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl OPENFILENAME_NT4W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OPENFILENAME_NT4W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OPENFILENAME_NT4W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OPENFILENAME_NT4W")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("lpstrFilter", &self.lpstrFilter)
            .field("lpstrCustomFilter", &self.lpstrCustomFilter)
            .field("nMaxCustFilter", &self.nMaxCustFilter)
            .field("nFilterIndex", &self.nFilterIndex)
            .field("lpstrFile", &self.lpstrFile)
            .field("nMaxFile", &self.nMaxFile)
            .field("lpstrFileTitle", &self.lpstrFileTitle)
            .field("nMaxFileTitle", &self.nMaxFileTitle)
            .field("lpstrInitialDir", &self.lpstrInitialDir)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("Flags", &self.Flags)
            .field("nFileOffset", &self.nFileOffset)
            .field("nFileExtension", &self.nFileExtension)
            .field("lpstrDefExt", &self.lpstrDefExt)
            .field("lCustData", &self.lCustData)
            .field("lpTemplateName", &self.lpTemplateName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OPENFILENAME_NT4W {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hInstance == other.hInstance
            && self.lpstrFilter == other.lpstrFilter
            && self.lpstrCustomFilter == other.lpstrCustomFilter
            && self.nMaxCustFilter == other.nMaxCustFilter
            && self.nFilterIndex == other.nFilterIndex
            && self.lpstrFile == other.lpstrFile
            && self.nMaxFile == other.nMaxFile
            && self.lpstrFileTitle == other.lpstrFileTitle
            && self.nMaxFileTitle == other.nMaxFileTitle
            && self.lpstrInitialDir == other.lpstrInitialDir
            && self.lpstrTitle == other.lpstrTitle
            && self.Flags == other.Flags
            && self.nFileOffset == other.nFileOffset
            && self.nFileExtension == other.nFileExtension
            && self.lpstrDefExt == other.lpstrDefExt
            && self.lCustData == other.lCustData
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lpTemplateName == other.lpTemplateName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OPENFILENAME_NT4W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OPENFILENAME_NT4W {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPEN_FILENAME_FLAGS(pub u32);
pub const OFN_READONLY: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(1u32);
pub const OFN_OVERWRITEPROMPT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(2u32);
pub const OFN_HIDEREADONLY: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(4u32);
pub const OFN_NOCHANGEDIR: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(8u32);
pub const OFN_SHOWHELP: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(16u32);
pub const OFN_ENABLEHOOK: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(32u32);
pub const OFN_ENABLETEMPLATE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(64u32);
pub const OFN_ENABLETEMPLATEHANDLE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(128u32);
pub const OFN_NOVALIDATE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(256u32);
pub const OFN_ALLOWMULTISELECT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(512u32);
pub const OFN_EXTENSIONDIFFERENT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(1024u32);
pub const OFN_PATHMUSTEXIST: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(2048u32);
pub const OFN_FILEMUSTEXIST: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(4096u32);
pub const OFN_CREATEPROMPT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(8192u32);
pub const OFN_SHAREAWARE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(16384u32);
pub const OFN_NOREADONLYRETURN: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(32768u32);
pub const OFN_NOTESTFILECREATE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(65536u32);
pub const OFN_NONETWORKBUTTON: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(131072u32);
pub const OFN_NOLONGNAMES: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(262144u32);
pub const OFN_EXPLORER: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(524288u32);
pub const OFN_NODEREFERENCELINKS: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(1048576u32);
pub const OFN_LONGNAMES: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(2097152u32);
pub const OFN_ENABLEINCLUDENOTIFY: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(4194304u32);
pub const OFN_ENABLESIZING: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(8388608u32);
pub const OFN_DONTADDTORECENT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(33554432u32);
pub const OFN_FORCESHOWHIDDEN: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(268435456u32);
impl ::std::convert::From<u32> for OPEN_FILENAME_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPEN_FILENAME_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for OPEN_FILENAME_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for OPEN_FILENAME_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for OPEN_FILENAME_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for OPEN_FILENAME_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for OPEN_FILENAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPEN_FILENAME_FLAGS_EX(pub u32);
pub const OFN_EX_NONE: OPEN_FILENAME_FLAGS_EX = OPEN_FILENAME_FLAGS_EX(0u32);
pub const OFN_EX_NOPLACESBAR: OPEN_FILENAME_FLAGS_EX = OPEN_FILENAME_FLAGS_EX(1u32);
impl ::std::convert::From<u32> for OPEN_FILENAME_FLAGS_EX {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPEN_FILENAME_FLAGS_EX {
    type Abi = Self;
}
impl ::std::ops::BitOr for OPEN_FILENAME_FLAGS_EX {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for OPEN_FILENAME_FLAGS_EX {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for OPEN_FILENAME_FLAGS_EX {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for OPEN_FILENAME_FLAGS_EX {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for OPEN_FILENAME_FLAGS_EX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct PAGESETUPDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: ::std::option::Option<LPPAGESETUPHOOK>,
    pub lpfnPagePaintHook: ::std::option::Option<LPPAGEPAINTHOOK>,
    pub lpPageSetupTemplateName: super::super::super::Foundation::PSTR,
    pub hPageSetupTemplate: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl PAGESETUPDLGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PAGESETUPDLGA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PAGESETUPDLGA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PAGESETUPDLGA")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDevMode", &self.hDevMode)
            .field("hDevNames", &self.hDevNames)
            .field("Flags", &self.Flags)
            .field("ptPaperSize", &self.ptPaperSize)
            .field("rtMinMargin", &self.rtMinMargin)
            .field("rtMargin", &self.rtMargin)
            .field("hInstance", &self.hInstance)
            .field("lCustData", &self.lCustData)
            .field("lpPageSetupTemplateName", &self.lpPageSetupTemplateName)
            .field("hPageSetupTemplate", &self.hPageSetupTemplate)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PAGESETUPDLGA {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hDevMode == other.hDevMode
            && self.hDevNames == other.hDevNames
            && self.Flags == other.Flags
            && self.ptPaperSize == other.ptPaperSize
            && self.rtMinMargin == other.rtMinMargin
            && self.rtMargin == other.rtMargin
            && self.hInstance == other.hInstance
            && self.lCustData == other.lCustData
            && self.lpfnPageSetupHook.map(|f| f as usize) == other.lpfnPageSetupHook.map(|f| f as usize)
            && self.lpfnPagePaintHook.map(|f| f as usize) == other.lpfnPagePaintHook.map(|f| f as usize)
            && self.lpPageSetupTemplateName == other.lpPageSetupTemplateName
            && self.hPageSetupTemplate == other.hPageSetupTemplate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PAGESETUPDLGA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PAGESETUPDLGA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
pub struct PAGESETUPDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: ::std::option::Option<LPPAGESETUPHOOK>,
    pub lpfnPagePaintHook: ::std::option::Option<LPPAGEPAINTHOOK>,
    pub lpPageSetupTemplateName: super::super::super::Foundation::PWSTR,
    pub hPageSetupTemplate: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl PAGESETUPDLGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PAGESETUPDLGW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PAGESETUPDLGW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PAGESETUPDLGW")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDevMode", &self.hDevMode)
            .field("hDevNames", &self.hDevNames)
            .field("Flags", &self.Flags)
            .field("ptPaperSize", &self.ptPaperSize)
            .field("rtMinMargin", &self.rtMinMargin)
            .field("rtMargin", &self.rtMargin)
            .field("hInstance", &self.hInstance)
            .field("lCustData", &self.lCustData)
            .field("lpPageSetupTemplateName", &self.lpPageSetupTemplateName)
            .field("hPageSetupTemplate", &self.hPageSetupTemplate)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PAGESETUPDLGW {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hDevMode == other.hDevMode
            && self.hDevNames == other.hDevNames
            && self.Flags == other.Flags
            && self.ptPaperSize == other.ptPaperSize
            && self.rtMinMargin == other.rtMinMargin
            && self.rtMargin == other.rtMargin
            && self.hInstance == other.hInstance
            && self.lCustData == other.lCustData
            && self.lpfnPageSetupHook.map(|f| f as usize) == other.lpfnPageSetupHook.map(|f| f as usize)
            && self.lpfnPagePaintHook.map(|f| f as usize) == other.lpfnPagePaintHook.map(|f| f as usize)
            && self.lpPageSetupTemplateName == other.lpPageSetupTemplateName
            && self.hPageSetupTemplate == other.hPageSetupTemplate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PAGESETUPDLGW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PAGESETUPDLGW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PAGESETUPDLG_FLAGS(pub u32);
pub const PSD_DEFAULTMINMARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(0u32);
pub const PSD_DISABLEMARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(16u32);
pub const PSD_DISABLEORIENTATION: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(256u32);
pub const PSD_DISABLEPAGEPAINTING: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(524288u32);
pub const PSD_DISABLEPAPER: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(512u32);
pub const PSD_DISABLEPRINTER: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(32u32);
pub const PSD_ENABLEPAGEPAINTHOOK: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(262144u32);
pub const PSD_ENABLEPAGESETUPHOOK: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(8192u32);
pub const PSD_ENABLEPAGESETUPTEMPLATE: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(32768u32);
pub const PSD_ENABLEPAGESETUPTEMPLATEHANDLE: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(131072u32);
pub const PSD_INHUNDREDTHSOFMILLIMETERS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(8u32);
pub const PSD_INTHOUSANDTHSOFINCHES: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(4u32);
pub const PSD_INWININIINTLMEASURE: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(0u32);
pub const PSD_MARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(2u32);
pub const PSD_MINMARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(1u32);
pub const PSD_NONETWORKBUTTON: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(2097152u32);
pub const PSD_NOWARNING: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(128u32);
pub const PSD_RETURNDEFAULT: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(1024u32);
pub const PSD_SHOWHELP: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(2048u32);
impl ::std::convert::From<u32> for PAGESETUPDLG_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PAGESETUPDLG_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for PAGESETUPDLG_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PAGESETUPDLG_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PAGESETUPDLG_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PAGESETUPDLG_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PAGESETUPDLG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const PD_RESULT_APPLY: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const PD_RESULT_CANCEL: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const PD_RESULT_PRINT: u32 = 1u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct PRINTDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: ::std::option::Option<LPPRINTHOOKPROC>,
    pub lpfnSetupHook: ::std::option::Option<LPSETUPHOOKPROC>,
    pub lpPrintTemplateName: super::super::super::Foundation::PSTR,
    pub lpSetupTemplateName: super::super::super::Foundation::PSTR,
    pub hPrintTemplate: isize,
    pub hSetupTemplate: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl PRINTDLGA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for PRINTDLGA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for PRINTDLGA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRINTDLGA")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDevMode", &self.hDevMode)
            .field("hDevNames", &self.hDevNames)
            .field("hDC", &self.hDC)
            .field("Flags", &self.Flags)
            .field("nFromPage", &self.nFromPage)
            .field("nToPage", &self.nToPage)
            .field("nMinPage", &self.nMinPage)
            .field("nMaxPage", &self.nMaxPage)
            .field("nCopies", &self.nCopies)
            .field("hInstance", &self.hInstance)
            .field("lCustData", &self.lCustData)
            .field("lpPrintTemplateName", &self.lpPrintTemplateName)
            .field("lpSetupTemplateName", &self.lpSetupTemplateName)
            .field("hPrintTemplate", &self.hPrintTemplate)
            .field("hSetupTemplate", &self.hSetupTemplate)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for PRINTDLGA {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hDevMode == other.hDevMode
            && self.hDevNames == other.hDevNames
            && self.hDC == other.hDC
            && self.Flags == other.Flags
            && self.nFromPage == other.nFromPage
            && self.nToPage == other.nToPage
            && self.nMinPage == other.nMinPage
            && self.nMaxPage == other.nMaxPage
            && self.nCopies == other.nCopies
            && self.hInstance == other.hInstance
            && self.lCustData == other.lCustData
            && self.lpfnPrintHook.map(|f| f as usize) == other.lpfnPrintHook.map(|f| f as usize)
            && self.lpfnSetupHook.map(|f| f as usize) == other.lpfnSetupHook.map(|f| f as usize)
            && self.lpPrintTemplateName == other.lpPrintTemplateName
            && self.lpSetupTemplateName == other.lpSetupTemplateName
            && self.hPrintTemplate == other.hPrintTemplate
            && self.hSetupTemplate == other.hSetupTemplate
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for PRINTDLGA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for PRINTDLGA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct PRINTDLGEXA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: super::super::super::Foundation::PSTR,
    pub lpCallback: ::std::option::Option<::windows::runtime::IUnknown>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl PRINTDLGEXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for PRINTDLGEXA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for PRINTDLGEXA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRINTDLGEXA")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDevMode", &self.hDevMode)
            .field("hDevNames", &self.hDevNames)
            .field("hDC", &self.hDC)
            .field("Flags", &self.Flags)
            .field("Flags2", &self.Flags2)
            .field("ExclusionFlags", &self.ExclusionFlags)
            .field("nPageRanges", &self.nPageRanges)
            .field("nMaxPageRanges", &self.nMaxPageRanges)
            .field("lpPageRanges", &self.lpPageRanges)
            .field("nMinPage", &self.nMinPage)
            .field("nMaxPage", &self.nMaxPage)
            .field("nCopies", &self.nCopies)
            .field("hInstance", &self.hInstance)
            .field("lpPrintTemplateName", &self.lpPrintTemplateName)
            .field("lpCallback", &self.lpCallback)
            .field("nPropertyPages", &self.nPropertyPages)
            .field("lphPropertyPages", &self.lphPropertyPages)
            .field("nStartPage", &self.nStartPage)
            .field("dwResultAction", &self.dwResultAction)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for PRINTDLGEXA {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hDevMode == other.hDevMode
            && self.hDevNames == other.hDevNames
            && self.hDC == other.hDC
            && self.Flags == other.Flags
            && self.Flags2 == other.Flags2
            && self.ExclusionFlags == other.ExclusionFlags
            && self.nPageRanges == other.nPageRanges
            && self.nMaxPageRanges == other.nMaxPageRanges
            && self.lpPageRanges == other.lpPageRanges
            && self.nMinPage == other.nMinPage
            && self.nMaxPage == other.nMaxPage
            && self.nCopies == other.nCopies
            && self.hInstance == other.hInstance
            && self.lpPrintTemplateName == other.lpPrintTemplateName
            && self.lpCallback == other.lpCallback
            && self.nPropertyPages == other.nPropertyPages
            && self.lphPropertyPages == other.lphPropertyPages
            && self.nStartPage == other.nStartPage
            && self.dwResultAction == other.dwResultAction
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for PRINTDLGEXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for PRINTDLGEXA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct PRINTDLGEXW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: super::super::super::Foundation::PWSTR,
    pub lpCallback: ::std::option::Option<::windows::runtime::IUnknown>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl PRINTDLGEXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for PRINTDLGEXW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for PRINTDLGEXW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRINTDLGEXW")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDevMode", &self.hDevMode)
            .field("hDevNames", &self.hDevNames)
            .field("hDC", &self.hDC)
            .field("Flags", &self.Flags)
            .field("Flags2", &self.Flags2)
            .field("ExclusionFlags", &self.ExclusionFlags)
            .field("nPageRanges", &self.nPageRanges)
            .field("nMaxPageRanges", &self.nMaxPageRanges)
            .field("lpPageRanges", &self.lpPageRanges)
            .field("nMinPage", &self.nMinPage)
            .field("nMaxPage", &self.nMaxPage)
            .field("nCopies", &self.nCopies)
            .field("hInstance", &self.hInstance)
            .field("lpPrintTemplateName", &self.lpPrintTemplateName)
            .field("lpCallback", &self.lpCallback)
            .field("nPropertyPages", &self.nPropertyPages)
            .field("lphPropertyPages", &self.lphPropertyPages)
            .field("nStartPage", &self.nStartPage)
            .field("dwResultAction", &self.dwResultAction)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for PRINTDLGEXW {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hDevMode == other.hDevMode
            && self.hDevNames == other.hDevNames
            && self.hDC == other.hDC
            && self.Flags == other.Flags
            && self.Flags2 == other.Flags2
            && self.ExclusionFlags == other.ExclusionFlags
            && self.nPageRanges == other.nPageRanges
            && self.nMaxPageRanges == other.nMaxPageRanges
            && self.lpPageRanges == other.lpPageRanges
            && self.nMinPage == other.nMinPage
            && self.nMaxPage == other.nMaxPage
            && self.nCopies == other.nCopies
            && self.hInstance == other.hInstance
            && self.lpPrintTemplateName == other.lpPrintTemplateName
            && self.lpCallback == other.lpCallback
            && self.nPropertyPages == other.nPropertyPages
            && self.lphPropertyPages == other.lphPropertyPages
            && self.nStartPage == other.nStartPage
            && self.dwResultAction == other.dwResultAction
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for PRINTDLGEXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for PRINTDLGEXW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PRINTDLGEX_FLAGS(pub u32);
pub const PD_ALLPAGES: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(0u32);
pub const PD_COLLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(16u32);
pub const PD_CURRENTPAGE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(4194304u32);
pub const PD_DISABLEPRINTTOFILE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(524288u32);
pub const PD_ENABLEPRINTTEMPLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(16384u32);
pub const PD_ENABLEPRINTTEMPLATEHANDLE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(65536u32);
pub const PD_EXCLUSIONFLAGS: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(16777216u32);
pub const PD_HIDEPRINTTOFILE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(1048576u32);
pub const PD_NOCURRENTPAGE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(8388608u32);
pub const PD_NOPAGENUMS: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(8u32);
pub const PD_NOSELECTION: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(4u32);
pub const PD_NOWARNING: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(128u32);
pub const PD_PAGENUMS: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(2u32);
pub const PD_PRINTTOFILE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(32u32);
pub const PD_RETURNDC: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(256u32);
pub const PD_RETURNDEFAULT: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(1024u32);
pub const PD_RETURNIC: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(512u32);
pub const PD_SELECTION: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(1u32);
pub const PD_USEDEVMODECOPIES: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(262144u32);
pub const PD_USEDEVMODECOPIESANDCOLLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(262144u32);
pub const PD_USELARGETEMPLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(268435456u32);
pub const PD_ENABLEPRINTHOOK: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(4096u32);
pub const PD_ENABLESETUPHOOK: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(8192u32);
pub const PD_ENABLESETUPTEMPLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(32768u32);
pub const PD_ENABLESETUPTEMPLATEHANDLE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(131072u32);
pub const PD_NONETWORKBUTTON: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(2097152u32);
pub const PD_PRINTSETUP: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(64u32);
pub const PD_SHOWHELP: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(2048u32);
impl ::std::convert::From<u32> for PRINTDLGEX_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PRINTDLGEX_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for PRINTDLGEX_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PRINTDLGEX_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PRINTDLGEX_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PRINTDLGEX_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PRINTDLGEX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct PRINTDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: ::std::option::Option<LPPRINTHOOKPROC>,
    pub lpfnSetupHook: ::std::option::Option<LPSETUPHOOKPROC>,
    pub lpPrintTemplateName: super::super::super::Foundation::PWSTR,
    pub lpSetupTemplateName: super::super::super::Foundation::PWSTR,
    pub hPrintTemplate: isize,
    pub hSetupTemplate: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl PRINTDLGW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for PRINTDLGW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for PRINTDLGW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRINTDLGW")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDevMode", &self.hDevMode)
            .field("hDevNames", &self.hDevNames)
            .field("hDC", &self.hDC)
            .field("Flags", &self.Flags)
            .field("nFromPage", &self.nFromPage)
            .field("nToPage", &self.nToPage)
            .field("nMinPage", &self.nMinPage)
            .field("nMaxPage", &self.nMaxPage)
            .field("nCopies", &self.nCopies)
            .field("hInstance", &self.hInstance)
            .field("lCustData", &self.lCustData)
            .field("lpPrintTemplateName", &self.lpPrintTemplateName)
            .field("lpSetupTemplateName", &self.lpSetupTemplateName)
            .field("hPrintTemplate", &self.hPrintTemplate)
            .field("hSetupTemplate", &self.hSetupTemplate)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for PRINTDLGW {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hDevMode == other.hDevMode
            && self.hDevNames == other.hDevNames
            && self.hDC == other.hDC
            && self.Flags == other.Flags
            && self.nFromPage == other.nFromPage
            && self.nToPage == other.nToPage
            && self.nMinPage == other.nMinPage
            && self.nMaxPage == other.nMaxPage
            && self.nCopies == other.nCopies
            && self.hInstance == other.hInstance
            && self.lCustData == other.lCustData
            && self.lpfnPrintHook.map(|f| f as usize) == other.lpfnPrintHook.map(|f| f as usize)
            && self.lpfnSetupHook.map(|f| f as usize) == other.lpfnSetupHook.map(|f| f as usize)
            && self.lpPrintTemplateName == other.lpPrintTemplateName
            && self.lpSetupTemplateName == other.lpSetupTemplateName
            && self.hPrintTemplate == other.hPrintTemplate
            && self.hSetupTemplate == other.hSetupTemplate
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for PRINTDLGW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for PRINTDLGW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub struct PRINTPAGERANGE {
    pub nFromPage: u32,
    pub nToPage: u32,
}
impl PRINTPAGERANGE {}
impl ::std::default::Default for PRINTPAGERANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PRINTPAGERANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRINTPAGERANGE").field("nFromPage", &self.nFromPage).field("nToPage", &self.nToPage).finish()
    }
}
impl ::std::cmp::PartialEq for PRINTPAGERANGE {
    fn eq(&self, other: &Self) -> bool {
        self.nFromPage == other.nFromPage && self.nToPage == other.nToPage
    }
}
impl ::std::cmp::Eq for PRINTPAGERANGE {}
unsafe impl ::windows::runtime::Abi for PRINTPAGERANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const PS_OPENTYPE_FONTTYPE: u32 = 65536u32;
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PageSetupDlgA(param0: *mut PAGESETUPDLGA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PageSetupDlgA(param0: *mut ::std::mem::ManuallyDrop<PAGESETUPDLGA>) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PageSetupDlgA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PageSetupDlgW(param0: *mut PAGESETUPDLGW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PageSetupDlgW(param0: *mut ::std::mem::ManuallyDrop<PAGESETUPDLGW>) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PageSetupDlgW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[inline]
pub unsafe fn PrintDlgA(ppd: *mut PRINTDLGA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrintDlgA(ppd: *mut ::std::mem::ManuallyDrop<PRINTDLGA>) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrintDlgA(::std::mem::transmute(ppd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[inline]
pub unsafe fn PrintDlgExA(ppd: *mut PRINTDLGEXA) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrintDlgExA(ppd: *mut ::std::mem::ManuallyDrop<PRINTDLGEXA>) -> ::windows::runtime::HRESULT;
        }
        PrintDlgExA(::std::mem::transmute(ppd)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[inline]
pub unsafe fn PrintDlgExW(ppd: *mut PRINTDLGEXW) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrintDlgExW(ppd: *mut ::std::mem::ManuallyDrop<PRINTDLGEXW>) -> ::windows::runtime::HRESULT;
        }
        PrintDlgExW(::std::mem::transmute(ppd)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[inline]
pub unsafe fn PrintDlgW(ppd: *mut PRINTDLGW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrintDlgW(ppd: *mut ::std::mem::ManuallyDrop<PRINTDLGW>) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrintDlgW(::std::mem::transmute(ppd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn ReplaceTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReplaceTextA(param0: *mut ::std::mem::ManuallyDrop<FINDREPLACEA>) -> super::super::super::Foundation::HWND;
        }
        ::std::mem::transmute(ReplaceTextA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn ReplaceTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReplaceTextW(param0: *mut ::std::mem::ManuallyDrop<FINDREPLACEW>) -> super::super::super::Foundation::HWND;
        }
        ::std::mem::transmute(ReplaceTextW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const START_PAGE_GENERAL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const SYMBOL_FONTTYPE: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const TT_OPENTYPE_FONTTYPE: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const TYPE1_FONTTYPE: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_CHOOSEFONT_GETLOGFONT: u32 = 1025u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_CHOOSEFONT_SETFLAGS: u32 = 1126u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_CHOOSEFONT_SETLOGFONT: u32 = 1125u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_PSD_ENVSTAMPRECT: u32 = 1029u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_PSD_FULLPAGERECT: u32 = 1025u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_PSD_GREEKTEXTRECT: u32 = 1028u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_PSD_MARGINRECT: u32 = 1027u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_PSD_MINMARGINRECT: u32 = 1026u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_PSD_YAFULLPAGERECT: u32 = 1030u32;
