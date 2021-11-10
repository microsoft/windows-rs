#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ATP_CHANGE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ATP_NOCHANGE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ATP_NODELIMITER: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ATP_REPLACEALLTEXT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const AURL_DISABLEMIXEDLGC: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const AURL_ENABLEDRIVELETTERS: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const AURL_ENABLEEA: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const AURL_ENABLEEAURLS: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const AURL_ENABLEEMAILADDR: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const AURL_ENABLETELNO: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const AURL_ENABLEURL: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type AutoCorrectProc = unsafe extern "system" fn(langid: u16, pszbefore: super::super::super::Foundation::PWSTR, pszafter: super::super::super::Foundation::PWSTR, cchafter: i32, pcchreplaced: *mut i32) -> i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct BIDIOPTIONS {
    pub cbSize: u32,
    pub wMask: u16,
    pub wEffects: u16,
}
impl BIDIOPTIONS {}
impl ::core::default::Default for BIDIOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BIDIOPTIONS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BIDIOPTIONS").field("cbSize", &self.cbSize).field("wMask", &self.wMask).field("wEffects", &self.wEffects).finish()
    }
}
impl ::core::cmp::PartialEq for BIDIOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.wMask == other.wMask && self.wEffects == other.wEffects
    }
}
impl ::core::cmp::Eq for BIDIOPTIONS {}
unsafe impl ::windows::runtime::Abi for BIDIOPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOE_CONTEXTALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOE_CONTEXTREADING: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOE_FORCERECALC: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOE_LEGACYBIDICLASS: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOE_NEUTRALOVERRIDE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOE_PLAINTEXT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOE_RTLDIR: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOE_UNICODEBIDI: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOM_CONTEXTALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOM_CONTEXTREADING: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOM_DEFPARADIR: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOM_LEGACYBIDICLASS: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOM_NEUTRALOVERRIDE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOM_PLAINTEXT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const BOM_UNICODEBIDI: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CARET_FLAGS(pub i32);
pub const CARET_NONE: CARET_FLAGS = CARET_FLAGS(0i32);
pub const CARET_CUSTOM: CARET_FLAGS = CARET_FLAGS(1i32);
pub const CARET_RTL: CARET_FLAGS = CARET_FLAGS(2i32);
pub const CARET_ITALIC: CARET_FLAGS = CARET_FLAGS(32i32);
pub const CARET_NULL: CARET_FLAGS = CARET_FLAGS(64i32);
pub const CARET_ROTATE90: CARET_FLAGS = CARET_FLAGS(128i32);
impl ::core::convert::From<i32> for CARET_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CARET_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Graphics_Gdi`*"]
pub union CARET_INFO {
    pub hbitmap: super::super::super::Graphics::Gdi::HBITMAP,
    pub caretFlags: CARET_FLAGS,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl CARET_INFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for CARET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for CARET_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for CARET_INFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for CARET_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CFE_EFFECTS(pub u32);
pub const CFE_ALLCAPS: CFE_EFFECTS = CFE_EFFECTS(128u32);
pub const CFE_AUTOBACKCOLOR: CFE_EFFECTS = CFE_EFFECTS(67108864u32);
pub const CFE_DISABLED: CFE_EFFECTS = CFE_EFFECTS(8192u32);
pub const CFE_EMBOSS: CFE_EFFECTS = CFE_EFFECTS(2048u32);
pub const CFE_HIDDEN: CFE_EFFECTS = CFE_EFFECTS(256u32);
pub const CFE_IMPRINT: CFE_EFFECTS = CFE_EFFECTS(4096u32);
pub const CFE_OUTLINE: CFE_EFFECTS = CFE_EFFECTS(512u32);
pub const CFE_REVISED: CFE_EFFECTS = CFE_EFFECTS(16384u32);
pub const CFE_SHADOW: CFE_EFFECTS = CFE_EFFECTS(1024u32);
pub const CFE_SMALLCAPS: CFE_EFFECTS = CFE_EFFECTS(64u32);
pub const CFE_AUTOCOLOR: CFE_EFFECTS = CFE_EFFECTS(1073741824u32);
pub const CFE_BOLD: CFE_EFFECTS = CFE_EFFECTS(1u32);
pub const CFE_ITALIC: CFE_EFFECTS = CFE_EFFECTS(2u32);
pub const CFE_STRIKEOUT: CFE_EFFECTS = CFE_EFFECTS(8u32);
pub const CFE_UNDERLINE: CFE_EFFECTS = CFE_EFFECTS(4u32);
pub const CFE_PROTECTED: CFE_EFFECTS = CFE_EFFECTS(16u32);
pub const CFE_LINK: CFE_EFFECTS = CFE_EFFECTS(32u32);
pub const CFE_SUBSCRIPT: CFE_EFFECTS = CFE_EFFECTS(65536u32);
pub const CFE_SUPERSCRIPT: CFE_EFFECTS = CFE_EFFECTS(131072u32);
pub const CFE_FONTBOUND: CFE_EFFECTS = CFE_EFFECTS(1048576u32);
pub const CFE_LINKPROTECTED: CFE_EFFECTS = CFE_EFFECTS(8388608u32);
pub const CFE_EXTENDED: CFE_EFFECTS = CFE_EFFECTS(33554432u32);
pub const CFE_MATHNOBUILDUP: CFE_EFFECTS = CFE_EFFECTS(134217728u32);
pub const CFE_MATH: CFE_EFFECTS = CFE_EFFECTS(268435456u32);
pub const CFE_MATHORDINARY: CFE_EFFECTS = CFE_EFFECTS(536870912u32);
impl ::core::convert::From<u32> for CFE_EFFECTS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CFE_EFFECTS {
    type Abi = Self;
}
impl ::core::ops::BitOr for CFE_EFFECTS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for CFE_EFFECTS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for CFE_EFFECTS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for CFE_EFFECTS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for CFE_EFFECTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CFM_MASK(pub u32);
pub const CFM_SUBSCRIPT: CFM_MASK = CFM_MASK(196608u32);
pub const CFM_SUPERSCRIPT: CFM_MASK = CFM_MASK(196608u32);
pub const CFM_EFFECTS: CFM_MASK = CFM_MASK(1073741887u32);
pub const CFM_ALL: CFM_MASK = CFM_MASK(4160749631u32);
pub const CFM_BOLD: CFM_MASK = CFM_MASK(1u32);
pub const CFM_CHARSET: CFM_MASK = CFM_MASK(134217728u32);
pub const CFM_COLOR: CFM_MASK = CFM_MASK(1073741824u32);
pub const CFM_FACE: CFM_MASK = CFM_MASK(536870912u32);
pub const CFM_ITALIC: CFM_MASK = CFM_MASK(2u32);
pub const CFM_OFFSET: CFM_MASK = CFM_MASK(268435456u32);
pub const CFM_PROTECTED: CFM_MASK = CFM_MASK(16u32);
pub const CFM_SIZE: CFM_MASK = CFM_MASK(2147483648u32);
pub const CFM_STRIKEOUT: CFM_MASK = CFM_MASK(8u32);
pub const CFM_UNDERLINE: CFM_MASK = CFM_MASK(4u32);
pub const CFM_LINK: CFM_MASK = CFM_MASK(32u32);
pub const CFM_SMALLCAPS: CFM_MASK = CFM_MASK(64u32);
pub const CFM_ALLCAPS: CFM_MASK = CFM_MASK(128u32);
pub const CFM_HIDDEN: CFM_MASK = CFM_MASK(256u32);
pub const CFM_OUTLINE: CFM_MASK = CFM_MASK(512u32);
pub const CFM_SHADOW: CFM_MASK = CFM_MASK(1024u32);
pub const CFM_EMBOSS: CFM_MASK = CFM_MASK(2048u32);
pub const CFM_IMPRINT: CFM_MASK = CFM_MASK(4096u32);
pub const CFM_DISABLED: CFM_MASK = CFM_MASK(8192u32);
pub const CFM_REVISED: CFM_MASK = CFM_MASK(16384u32);
pub const CFM_REVAUTHOR: CFM_MASK = CFM_MASK(32768u32);
pub const CFM_ANIMATION: CFM_MASK = CFM_MASK(262144u32);
pub const CFM_STYLE: CFM_MASK = CFM_MASK(524288u32);
pub const CFM_KERNING: CFM_MASK = CFM_MASK(1048576u32);
pub const CFM_SPACING: CFM_MASK = CFM_MASK(2097152u32);
pub const CFM_WEIGHT: CFM_MASK = CFM_MASK(4194304u32);
pub const CFM_UNDERLINETYPE: CFM_MASK = CFM_MASK(8388608u32);
pub const CFM_COOKIE: CFM_MASK = CFM_MASK(16777216u32);
pub const CFM_LCID: CFM_MASK = CFM_MASK(33554432u32);
pub const CFM_BACKCOLOR: CFM_MASK = CFM_MASK(67108864u32);
pub const CFM_EFFECTS2: CFM_MASK = CFM_MASK(1141080063u32);
pub const CFM_ALL2: CFM_MASK = CFM_MASK(4294967295u32);
pub const CFM_FONTBOUND: CFM_MASK = CFM_MASK(1048576u32);
pub const CFM_LINKPROTECTED: CFM_MASK = CFM_MASK(8388608u32);
pub const CFM_EXTENDED: CFM_MASK = CFM_MASK(33554432u32);
pub const CFM_MATHNOBUILDUP: CFM_MASK = CFM_MASK(134217728u32);
pub const CFM_MATH: CFM_MASK = CFM_MASK(268435456u32);
pub const CFM_MATHORDINARY: CFM_MASK = CFM_MASK(536870912u32);
pub const CFM_ALLEFFECTS: CFM_MASK = CFM_MASK(2115207167u32);
impl ::core::convert::From<u32> for CFM_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CFM_MASK {
    type Abi = Self;
}
impl ::core::ops::BitOr for CFM_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for CFM_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for CFM_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for CFM_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for CFM_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct CHANGENOTIFY {
    pub dwChangeType: CHANGETYPE,
    pub pvCookieData: *mut ::core::ffi::c_void,
}
impl CHANGENOTIFY {}
impl ::core::default::Default for CHANGENOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CHANGENOTIFY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CHANGENOTIFY").field("dwChangeType", &self.dwChangeType).field("pvCookieData", &self.pvCookieData).finish()
    }
}
impl ::core::cmp::PartialEq for CHANGENOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.dwChangeType == other.dwChangeType && self.pvCookieData == other.pvCookieData
    }
}
impl ::core::cmp::Eq for CHANGENOTIFY {}
unsafe impl ::windows::runtime::Abi for CHANGENOTIFY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CHANGETYPE(pub i32);
pub const CN_GENERIC: CHANGETYPE = CHANGETYPE(0i32);
pub const CN_TEXTCHANGED: CHANGETYPE = CHANGETYPE(1i32);
pub const CN_NEWUNDO: CHANGETYPE = CHANGETYPE(2i32);
pub const CN_NEWREDO: CHANGETYPE = CHANGETYPE(4i32);
impl ::core::convert::From<i32> for CHANGETYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CHANGETYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct CHARFORMAT2A {
    pub __AnonymousBase_richedit_L736_C23: CHARFORMATA,
    pub wWeight: u16,
    pub sSpacing: i16,
    pub crBackColor: u32,
    pub lcid: u32,
    pub Anonymous: CHARFORMAT2A_0,
    pub sStyle: i16,
    pub wKerning: u16,
    pub bUnderlineType: u8,
    pub bAnimation: u8,
    pub bRevAuthor: u8,
    pub bUnderlineColor: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl CHARFORMAT2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHARFORMAT2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHARFORMAT2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHARFORMAT2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CHARFORMAT2A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union CHARFORMAT2A_0 {
    pub dwReserved: u32,
    pub dwCookie: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CHARFORMAT2A_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHARFORMAT2A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHARFORMAT2A_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHARFORMAT2A_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CHARFORMAT2A_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct CHARFORMAT2W {
    pub __AnonymousBase_richedit_L711_C23: CHARFORMATW,
    pub wWeight: u16,
    pub sSpacing: i16,
    pub crBackColor: u32,
    pub lcid: u32,
    pub Anonymous: CHARFORMAT2W_0,
    pub sStyle: i16,
    pub wKerning: u16,
    pub bUnderlineType: u8,
    pub bAnimation: u8,
    pub bRevAuthor: u8,
    pub bUnderlineColor: u8,
}
impl CHARFORMAT2W {}
impl ::core::default::Default for CHARFORMAT2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHARFORMAT2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for CHARFORMAT2W {}
unsafe impl ::windows::runtime::Abi for CHARFORMAT2W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub union CHARFORMAT2W_0 {
    pub dwReserved: u32,
    pub dwCookie: u32,
}
impl CHARFORMAT2W_0 {}
impl ::core::default::Default for CHARFORMAT2W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHARFORMAT2W_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for CHARFORMAT2W_0 {}
unsafe impl ::windows::runtime::Abi for CHARFORMAT2W_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct CHARFORMATA {
    pub cbSize: u32,
    pub dwMask: CFM_MASK,
    pub dwEffects: CFE_EFFECTS,
    pub yHeight: i32,
    pub yOffset: i32,
    pub crTextColor: u32,
    pub bCharSet: u8,
    pub bPitchAndFamily: u8,
    pub szFaceName: [super::super::super::Foundation::CHAR; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl CHARFORMATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHARFORMATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHARFORMATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CHARFORMATA")
            .field("cbSize", &self.cbSize)
            .field("dwMask", &self.dwMask)
            .field("dwEffects", &self.dwEffects)
            .field("yHeight", &self.yHeight)
            .field("yOffset", &self.yOffset)
            .field("crTextColor", &self.crTextColor)
            .field("bCharSet", &self.bCharSet)
            .field("bPitchAndFamily", &self.bPitchAndFamily)
            .field("szFaceName", &self.szFaceName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHARFORMATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.dwEffects == other.dwEffects && self.yHeight == other.yHeight && self.yOffset == other.yOffset && self.crTextColor == other.crTextColor && self.bCharSet == other.bCharSet && self.bPitchAndFamily == other.bPitchAndFamily && self.szFaceName == other.szFaceName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHARFORMATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CHARFORMATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct CHARFORMATW {
    pub cbSize: u32,
    pub dwMask: CFM_MASK,
    pub dwEffects: CFE_EFFECTS,
    pub yHeight: i32,
    pub yOffset: i32,
    pub crTextColor: u32,
    pub bCharSet: u8,
    pub bPitchAndFamily: u8,
    pub szFaceName: [u16; 32],
}
impl CHARFORMATW {}
impl ::core::default::Default for CHARFORMATW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CHARFORMATW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CHARFORMATW")
            .field("cbSize", &self.cbSize)
            .field("dwMask", &self.dwMask)
            .field("dwEffects", &self.dwEffects)
            .field("yHeight", &self.yHeight)
            .field("yOffset", &self.yOffset)
            .field("crTextColor", &self.crTextColor)
            .field("bCharSet", &self.bCharSet)
            .field("bPitchAndFamily", &self.bPitchAndFamily)
            .field("szFaceName", &self.szFaceName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CHARFORMATW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.dwEffects == other.dwEffects && self.yHeight == other.yHeight && self.yOffset == other.yOffset && self.crTextColor == other.crTextColor && self.bCharSet == other.bCharSet && self.bPitchAndFamily == other.bPitchAndFamily && self.szFaceName == other.szFaceName
    }
}
impl ::core::cmp::Eq for CHARFORMATW {}
unsafe impl ::windows::runtime::Abi for CHARFORMATW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct CHARRANGE {
    pub cpMin: i32,
    pub cpMax: i32,
}
impl CHARRANGE {}
impl ::core::default::Default for CHARRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CHARRANGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CHARRANGE").field("cpMin", &self.cpMin).field("cpMax", &self.cpMax).finish()
    }
}
impl ::core::cmp::PartialEq for CHARRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.cpMin == other.cpMin && self.cpMax == other.cpMax
    }
}
impl ::core::cmp::Eq for CHARRANGE {}
unsafe impl ::windows::runtime::Abi for CHARRANGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct CLIPBOARDFORMAT {
    pub nmhdr: super::NMHDR,
    pub cf: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl CLIPBOARDFORMAT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLIPBOARDFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLIPBOARDFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLIPBOARDFORMAT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CLIPBOARDFORMAT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct COMPCOLOR {
    pub crText: u32,
    pub crBackground: u32,
    pub dwEffects: u32,
}
impl COMPCOLOR {}
impl ::core::default::Default for COMPCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COMPCOLOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COMPCOLOR").field("crText", &self.crText).field("crBackground", &self.crBackground).field("dwEffects", &self.dwEffects).finish()
    }
}
impl ::core::cmp::PartialEq for COMPCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.crText == other.crText && self.crBackground == other.crBackground && self.dwEffects == other.dwEffects
    }
}
impl ::core::cmp::Eq for COMPCOLOR {}
unsafe impl ::windows::runtime::Abi for COMPCOLOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const CTFMODEBIAS_CONVERSATION: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const CTFMODEBIAS_DATETIME: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const CTFMODEBIAS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const CTFMODEBIAS_FILENAME: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const CTFMODEBIAS_FULLWIDTHALPHANUMERIC: u32 = 11u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const CTFMODEBIAS_HALFWIDTHALPHANUMERIC: u32 = 12u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const CTFMODEBIAS_HALFWIDTHKATAKANA: u32 = 10u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const CTFMODEBIAS_HANGUL: u32 = 9u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const CTFMODEBIAS_HIRAGANA: u32 = 7u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const CTFMODEBIAS_KATAKANA: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const CTFMODEBIAS_NAME: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const CTFMODEBIAS_NUMERIC: u32 = 6u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const CTFMODEBIAS_READING: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ECOOP_AND: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ECOOP_OR: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ECOOP_SET: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ECOOP_XOR: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ECO_AUTOHSCROLL: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ECO_AUTOVSCROLL: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ECO_AUTOWORDSELECTION: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ECO_NOHIDESEL: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ECO_READONLY: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ECO_SAVESEL: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ECO_SELECTIONBAR: u32 = 16777216u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ECO_VERTICAL: u32 = 4194304u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ECO_WANTRETURN: u32 = 4096u32;
impl ::core::clone::Clone for EDITSTREAM {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct EDITSTREAM {
    pub dwCookie: usize,
    pub dwError: u32,
    pub pfnCallback: ::core::option::Option<EDITSTREAMCALLBACK>,
}
impl EDITSTREAM {}
impl ::core::default::Default for EDITSTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EDITSTREAM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EDITSTREAM {}
unsafe impl ::windows::runtime::Abi for EDITSTREAM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub type EDITSTREAMCALLBACK = unsafe extern "system" fn(dwcookie: usize, pbbuff: *mut u8, cb: i32, pcb: *mut i32) -> u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type EDITWORDBREAKPROCEX = unsafe extern "system" fn(pchtext: super::super::super::Foundation::PSTR, cchtext: i32, bcharset: u8, action: i32) -> i32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ELLIPSIS_END: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ELLIPSIS_MASK: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ELLIPSIS_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ELLIPSIS_WORD: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EMO_ENTER: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EMO_EXIT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EMO_EXPAND: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EMO_EXPANDDOCUMENT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EMO_EXPANDSELECTION: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EMO_GETVIEWMODE: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EMO_MOVESELECTION: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EMO_PROMOTE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_AUTOURLDETECT: u32 = 1115u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_CALLAUTOCORRECTPROC: u32 = 1279u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_CANPASTE: u32 = 1074u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_CANREDO: u32 = 1109u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_CONVPOSITION: u32 = 1132u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_DISPLAYBAND: u32 = 1075u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_EXGETSEL: u32 = 1076u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_EXLIMITTEXT: u32 = 1077u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_EXLINEFROMCHAR: u32 = 1078u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_EXSETSEL: u32 = 1079u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_FINDTEXT: u32 = 1080u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_FINDTEXTEX: u32 = 1103u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_FINDTEXTEXW: u32 = 1148u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_FINDTEXTW: u32 = 1147u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_FINDWORDBREAK: u32 = 1100u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_FORMATRANGE: u32 = 1081u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETAUTOCORRECTPROC: u32 = 1257u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETAUTOURLDETECT: u32 = 1116u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETBIDIOPTIONS: u32 = 1225u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETCHARFORMAT: u32 = 1082u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETCTFMODEBIAS: u32 = 1261u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETCTFOPENSTATUS: u32 = 1264u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETEDITSTYLE: u32 = 1229u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETEDITSTYLEEX: u32 = 1300u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETELLIPSISMODE: u32 = 1329u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETELLIPSISSTATE: u32 = 1346u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETEVENTMASK: u32 = 1083u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETHYPHENATEINFO: u32 = 1254u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETIMECOLOR: u32 = 1129u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETIMECOMPMODE: u32 = 1146u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETIMECOMPTEXT: u32 = 1266u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETIMEMODEBIAS: u32 = 1151u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETIMEOPTIONS: u32 = 1131u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETIMEPROPERTY: u32 = 1268u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETLANGOPTIONS: u32 = 1145u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETOLEINTERFACE: u32 = 1084u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETOPTIONS: u32 = 1102u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETPAGE: u32 = 1252u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETPAGEROTATE: u32 = 1259u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETPARAFORMAT: u32 = 1085u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETPUNCTUATION: u32 = 1125u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETQUERYRTFOBJ: u32 = 1293u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETREDONAME: u32 = 1111u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETSCROLLPOS: u32 = 1245u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETSELTEXT: u32 = 1086u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETSTORYTYPE: u32 = 1314u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETTABLEPARMS: u32 = 1289u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETTEXTEX: u32 = 1118u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETTEXTLENGTHEX: u32 = 1119u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETTEXTMODE: u32 = 1114u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETTEXTRANGE: u32 = 1099u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETTOUCHOPTIONS: u32 = 1334u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETTYPOGRAPHYOPTIONS: u32 = 1227u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETUNDONAME: u32 = 1110u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETVIEWKIND: u32 = 1250u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETWORDBREAKPROCEX: u32 = 1104u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETWORDWRAPMODE: u32 = 1127u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_GETZOOM: u32 = 1248u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_HIDESELECTION: u32 = 1087u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_INSERTIMAGE: u32 = 1338u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_INSERTTABLE: u32 = 1256u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_ISIME: u32 = 1267u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_OUTLINE: u32 = 1244u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_PASTESPECIAL: u32 = 1088u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_RECONVERSION: u32 = 1149u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_REDO: u32 = 1108u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_REQUESTRESIZE: u32 = 1089u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SELECTIONTYPE: u32 = 1090u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETAUTOCORRECTPROC: u32 = 1258u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETBIDIOPTIONS: u32 = 1224u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETBKGNDCOLOR: u32 = 1091u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETCHARFORMAT: u32 = 1092u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETCTFMODEBIAS: u32 = 1262u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETCTFOPENSTATUS: u32 = 1265u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETEDITSTYLE: u32 = 1228u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETEDITSTYLEEX: u32 = 1299u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETELLIPSISMODE: u32 = 1330u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETEVENTMASK: u32 = 1093u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETFONTSIZE: u32 = 1247u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETHYPHENATEINFO: u32 = 1255u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETIMECOLOR: u32 = 1128u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETIMEMODEBIAS: u32 = 1150u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETIMEOPTIONS: u32 = 1130u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETLANGOPTIONS: u32 = 1144u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETOLECALLBACK: u32 = 1094u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETOPTIONS: u32 = 1101u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETPAGE: u32 = 1253u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETPAGEROTATE: u32 = 1260u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETPALETTE: u32 = 1117u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETPARAFORMAT: u32 = 1095u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETPUNCTUATION: u32 = 1124u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETQUERYRTFOBJ: u32 = 1294u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETSCROLLPOS: u32 = 1246u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETSTORYTYPE: u32 = 1315u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETTABLEPARMS: u32 = 1331u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETTARGETDEVICE: u32 = 1096u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETTEXTEX: u32 = 1121u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETTEXTMODE: u32 = 1113u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETTOUCHOPTIONS: u32 = 1335u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETTYPOGRAPHYOPTIONS: u32 = 1226u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETUIANAME: u32 = 1344u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETUNDOLIMIT: u32 = 1106u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETVIEWKIND: u32 = 1251u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETWORDBREAKPROCEX: u32 = 1105u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETWORDWRAPMODE: u32 = 1126u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SETZOOM: u32 = 1249u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_SHOWSCROLLBAR: u32 = 1120u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_STOPGROUPTYPING: u32 = 1112u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_STREAMIN: u32 = 1097u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EM_STREAMOUT: u32 = 1098u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct ENCORRECTTEXT {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ENCORRECTTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENCORRECTTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENCORRECTTEXT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENCORRECTTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENCORRECTTEXT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct ENDCOMPOSITIONNOTIFY {
    pub nmhdr: super::NMHDR,
    pub dwCode: ENDCOMPOSITIONNOTIFY_CODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ENDCOMPOSITIONNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENDCOMPOSITIONNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENDCOMPOSITIONNOTIFY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENDCOMPOSITIONNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENDCOMPOSITIONNOTIFY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ENDCOMPOSITIONNOTIFY_CODE(pub u32);
pub const ECN_ENDCOMPOSITION: ENDCOMPOSITIONNOTIFY_CODE = ENDCOMPOSITIONNOTIFY_CODE(1u32);
pub const ECN_NEWTEXT: ENDCOMPOSITIONNOTIFY_CODE = ENDCOMPOSITIONNOTIFY_CODE(2u32);
impl ::core::convert::From<u32> for ENDCOMPOSITIONNOTIFY_CODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ENDCOMPOSITIONNOTIFY_CODE {
    type Abi = Self;
}
impl ::core::ops::BitOr for ENDCOMPOSITIONNOTIFY_CODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for ENDCOMPOSITIONNOTIFY_CODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for ENDCOMPOSITIONNOTIFY_CODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for ENDCOMPOSITIONNOTIFY_CODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for ENDCOMPOSITIONNOTIFY_CODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct ENDROPFILES {
    pub nmhdr: super::NMHDR,
    pub hDrop: super::super::super::Foundation::HANDLE,
    pub cp: i32,
    pub fProtected: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ENDROPFILES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENDROPFILES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENDROPFILES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENDROPFILES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENDROPFILES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct ENLINK {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ENLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENLINK {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENLINK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENLINK {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct ENLOWFIRTF {
    pub nmhdr: super::NMHDR,
    pub szControl: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ENLOWFIRTF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENLOWFIRTF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENLOWFIRTF {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENLOWFIRTF {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENLOWFIRTF {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_CHANGE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_CLIPFORMAT: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_CORRECTTEXT: u32 = 4194304u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_DRAGDROPDONE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_DROPFILES: u32 = 1048576u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_ENDCOMPOSITION: u32 = 536870912u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_GROUPTYPINGCHANGE: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_HIDELINKTOOLTIP: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_IMECHANGE: u32 = 8388608u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_KEYEVENTS: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_LANGCHANGE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_LINK: u32 = 67108864u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_LOWFIRTF: u32 = 134217728u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_MOUSEEVENTS: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_OBJECTPOSITIONS: u32 = 33554432u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_PAGECHANGE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_PARAGRAPHEXPANDED: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_PROTECTED: u32 = 2097152u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_REQUESTRESIZE: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_SCROLL: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_SCROLLEVENTS: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_SELCHANGE: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_STARTCOMPOSITION: u32 = 268435456u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ENM_UPDATE: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct ENOLEOPFAILED {
    pub nmhdr: super::NMHDR,
    pub iob: i32,
    pub lOper: i32,
    pub hr: ::windows::runtime::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ENOLEOPFAILED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENOLEOPFAILED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENOLEOPFAILED {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENOLEOPFAILED {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENOLEOPFAILED {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct ENPROTECTED {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ENPROTECTED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENPROTECTED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENPROTECTED {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENPROTECTED {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENPROTECTED {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct ENSAVECLIPBOARD {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub cch: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ENSAVECLIPBOARD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENSAVECLIPBOARD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENSAVECLIPBOARD {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENSAVECLIPBOARD {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENSAVECLIPBOARD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_ALIGNLTR: u32 = 1808u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_ALIGNRTL: u32 = 1809u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_CLIPFORMAT: u32 = 1810u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_CORRECTTEXT: u32 = 1797u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_DRAGDROPDONE: u32 = 1804u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_DROPFILES: u32 = 1795u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_ENDCOMPOSITION: u32 = 1812u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_IMECHANGE: u32 = 1799u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_LINK: u32 = 1803u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_LOWFIRTF: u32 = 1807u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_MSGFILTER: u32 = 1792u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_OBJECTPOSITIONS: u32 = 1802u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_OLEOPFAILED: u32 = 1801u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_PAGECHANGE: u32 = 1806u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_PARAGRAPHEXPANDED: u32 = 1805u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_PROTECTED: u32 = 1796u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_REQUESTRESIZE: u32 = 1793u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_SAVECLIPBOARD: u32 = 1800u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_SELCHANGE: u32 = 1794u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_STARTCOMPOSITION: u32 = 1811u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EN_STOPNOUNDO: u32 = 1798u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EPR_0: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EPR_180: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EPR_270: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EPR_90: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const EPR_SE: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ES_DISABLENOSCROLL: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ES_EX_NOCALLOLEINIT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ES_NOIME: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ES_NOOLEDRAGDROP: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ES_SAVESEL: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ES_SELECTIONBAR: u32 = 16777216u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ES_SELFIME: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ES_SUNKEN: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ES_VERTICAL: u32 = 4194304u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct FINDTEXTA {
    pub chrg: CHARRANGE,
    pub lpstrText: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FINDTEXTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDTEXTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FINDTEXTA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FINDTEXTA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FINDTEXTA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct FINDTEXTEXA {
    pub chrg: CHARRANGE,
    pub lpstrText: super::super::super::Foundation::PSTR,
    pub chrgText: CHARRANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl FINDTEXTEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDTEXTEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FINDTEXTEXA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FINDTEXTEXA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FINDTEXTEXA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct FINDTEXTEXW {
    pub chrg: CHARRANGE,
    pub lpstrText: super::super::super::Foundation::PWSTR,
    pub chrgText: CHARRANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl FINDTEXTEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDTEXTEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FINDTEXTEXW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FINDTEXTEXW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FINDTEXTEXW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct FINDTEXTW {
    pub chrg: CHARRANGE,
    pub lpstrText: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FINDTEXTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDTEXTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FINDTEXTW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FINDTEXTW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FINDTEXTW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct FORMATRANGE {
    pub hdc: super::super::super::Graphics::Gdi::HDC,
    pub hdcTarget: super::super::super::Graphics::Gdi::HDC,
    pub rc: super::super::super::Foundation::RECT,
    pub rcPage: super::super::super::Foundation::RECT,
    pub chrg: CHARRANGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl FORMATRANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for FORMATRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for FORMATRANGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for FORMATRANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for FORMATRANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const FR_MATCHALEFHAMZA: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const FR_MATCHDIAC: u32 = 536870912u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const FR_MATCHKASHIDA: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const GCMF_GRIPPER: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const GCMF_MOUSEMENU: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const GCMF_SPELLING: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const GCMF_TOUCHMENU: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const GCM_MOUSEMENU: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const GCM_TOUCHMENU: u32 = 16384u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct GETCONTEXTMENUEX {
    pub chrg: CHARRANGE,
    pub dwFlags: u32,
    pub pt: super::super::super::Foundation::POINT,
    pub pvReserved: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl GETCONTEXTMENUEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GETCONTEXTMENUEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GETCONTEXTMENUEX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GETCONTEXTMENUEX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GETCONTEXTMENUEX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct GETTEXTEX {
    pub cb: u32,
    pub flags: GETTEXTEX_FLAGS,
    pub codepage: u32,
    pub lpDefaultChar: super::super::super::Foundation::PSTR,
    pub lpUsedDefChar: *mut i32,
}
#[cfg(feature = "Win32_Foundation")]
impl GETTEXTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GETTEXTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GETTEXTEX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GETTEXTEX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GETTEXTEX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GETTEXTEX_FLAGS(pub u32);
pub const GT_DEFAULT: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(0u32);
pub const GT_NOHIDDENTEXT: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(8u32);
pub const GT_RAWTEXT: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(4u32);
pub const GT_SELECTION: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(2u32);
pub const GT_USECRLF: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(1u32);
impl ::core::convert::From<u32> for GETTEXTEX_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GETTEXTEX_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for GETTEXTEX_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for GETTEXTEX_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for GETTEXTEX_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for GETTEXTEX_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for GETTEXTEX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct GETTEXTLENGTHEX {
    pub flags: GETTEXTLENGTHEX_FLAGS,
    pub codepage: u32,
}
impl GETTEXTLENGTHEX {}
impl ::core::default::Default for GETTEXTLENGTHEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GETTEXTLENGTHEX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GETTEXTLENGTHEX").field("flags", &self.flags).field("codepage", &self.codepage).finish()
    }
}
impl ::core::cmp::PartialEq for GETTEXTLENGTHEX {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.codepage == other.codepage
    }
}
impl ::core::cmp::Eq for GETTEXTLENGTHEX {}
unsafe impl ::windows::runtime::Abi for GETTEXTLENGTHEX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GETTEXTLENGTHEX_FLAGS(pub u32);
pub const GTL_DEFAULT: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(0u32);
pub const GTL_USECRLF: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(1u32);
pub const GTL_PRECISE: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(2u32);
pub const GTL_CLOSE: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(4u32);
pub const GTL_NUMCHARS: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(8u32);
pub const GTL_NUMBYTES: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(16u32);
impl ::core::convert::From<u32> for GETTEXTLENGTHEX_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GETTEXTLENGTHEX_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for GETTEXTLENGTHEX_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for GETTEXTLENGTHEX_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for GETTEXTLENGTHEX_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for GETTEXTLENGTHEX_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for GETTEXTLENGTHEX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct HYPHENATEINFO {
    pub cbSize: i16,
    pub dxHyphenateZone: i16,
    pub pfnHyphenate: isize,
}
impl HYPHENATEINFO {}
impl ::core::default::Default for HYPHENATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HYPHENATEINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for HYPHENATEINFO {}
unsafe impl ::windows::runtime::Abi for HYPHENATEINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ICM_CTF: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ICM_LEVEL2: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ICM_LEVEL2_5: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ICM_LEVEL2_SUI: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ICM_LEVEL3: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ICM_NOTOPEN: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct IMECOMPTEXT {
    pub cb: i32,
    pub flags: IMECOMPTEXT_FLAGS,
}
impl IMECOMPTEXT {}
impl ::core::default::Default for IMECOMPTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for IMECOMPTEXT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("IMECOMPTEXT").field("cb", &self.cb).field("flags", &self.flags).finish()
    }
}
impl ::core::cmp::PartialEq for IMECOMPTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for IMECOMPTEXT {}
unsafe impl ::windows::runtime::Abi for IMECOMPTEXT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMECOMPTEXT_FLAGS(pub u32);
pub const ICT_RESULTREADSTR: IMECOMPTEXT_FLAGS = IMECOMPTEXT_FLAGS(1u32);
impl ::core::convert::From<u32> for IMECOMPTEXT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMECOMPTEXT_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for IMECOMPTEXT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for IMECOMPTEXT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for IMECOMPTEXT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for IMECOMPTEXT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for IMECOMPTEXT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_AUTOFONT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_AUTOFONTSIZEADJUST: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_AUTOKEYBOARD: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_CLOSESTATUSWINDOW: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_DUALFONT: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_FORCEACTIVE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_FORCEDISABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_FORCEENABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_FORCEINACTIVE: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_FORCENONE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_FORCEREMEMBER: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_IMEALWAYSSENDNOTIFY: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_IMECANCELCOMPLETE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_IMEUIINTEGRATION: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_MULTIPLEEDIT: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_NOIMPLICITLANG: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_NOKBDLIDFIXUP: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_NORTFFONTSUBSTITUTE: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_SMODE_NONE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_SMODE_PLAURALCLAUSE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_SPELLCHECKING: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_TKBPREDICTION: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_UIFONTS: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const IMF_VERTICAL: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRichEditOle(pub ::windows::runtime::IUnknown);
impl IRichEditOle {
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Ole`*"]
    pub unsafe fn GetClientSite(&self) -> ::windows::runtime::Result<super::super::super::System::Ole::IOleClientSite> {
        let mut result__: <super::super::super::System::Ole::IOleClientSite as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Ole::IOleClientSite>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetObjectCount(&self) -> i32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetLinkCount(&self) -> i32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`*"]
    pub unsafe fn GetObject(&self, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(iob), ::core::mem::transmute(lpreobject), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`*"]
    pub unsafe fn InsertObject(&self, lpreobject: *mut REOBJECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpreobject)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn ConvertObject<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, iob: i32, rclsidnew: *const ::windows::runtime::GUID, lpstrusertypenew: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(iob), ::core::mem::transmute(rclsidnew), lpstrusertypenew.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ActivateAs(&self, rclsid: *const ::windows::runtime::GUID, rclsidas: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rclsidas)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SetHostNames<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, lpstrcontainerapp: Param0, lpstrcontainerobj: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), lpstrcontainerapp.into_param().abi(), lpstrcontainerobj.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SetLinkAvailable<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, iob: i32, favailable: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(iob), favailable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDvaspect(&self, iob: i32, dvaspect: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(iob), ::core::mem::transmute(dvaspect)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn HandsOffStorage(&self, iob: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(iob)).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SaveCompleted<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IStorage>>(&self, iob: i32, lpstg: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(iob), lpstg.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn InPlaceDeactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn ContextSensitiveHelp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fentermode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), fentermode.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com`*"]
    pub unsafe fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::core::option::Option<super::super::super::System::Com::IDataObject>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpchrg), ::core::mem::transmute(reco), ::core::mem::transmute(lplpdataobj)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com`*"]
    pub unsafe fn ImportDataObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDataObject>>(&self, lpdataobj: Param0, cf: u16, hmetapict: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), lpdataobj.into_param().abi(), ::core::mem::transmute(cf), ::core::mem::transmute(hmetapict)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRichEditOle {
    type Vtable = IRichEditOle_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x00020d00_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IRichEditOle> for ::windows::runtime::IUnknown {
    fn from(value: IRichEditOle) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRichEditOle> for ::windows::runtime::IUnknown {
    fn from(value: &IRichEditOle) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRichEditOle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRichEditOle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditOle_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lplpolesite: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> i32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> i32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iob: i32, lpreobject: *mut ::core::mem::ManuallyDrop<REOBJECT>, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpreobject: *mut ::core::mem::ManuallyDrop<REOBJECT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iob: i32, rclsidnew: *const ::windows::runtime::GUID, lpstrusertypenew: super::super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, rclsidas: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpstrcontainerapp: super::super::super::Foundation::PSTR, lpstrcontainerobj: super::super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iob: i32, favailable: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iob: i32, dvaspect: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iob: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iob: i32, lpstg: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fentermode: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpdataobj: ::windows::runtime::RawPtr, cf: u16, hmetapict: isize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRichEditOleCallback(pub ::windows::runtime::IUnknown);
impl IRichEditOleCallback {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetNewStorage(&self) -> ::windows::runtime::Result<super::super::super::System::Com::StructuredStorage::IStorage> {
        let mut result__: <super::super::super::System::Com::StructuredStorage::IStorage as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::StructuredStorage::IStorage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Ole`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn GetInPlaceContext(&self, lplpframe: *mut ::core::option::Option<super::super::super::System::Ole::IOleInPlaceFrame>, lplpdoc: *mut ::core::option::Option<super::super::super::System::Ole::IOleInPlaceUIWindow>, lpframeinfo: *mut super::super::super::System::Ole::OIFI) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lplpframe), ::core::mem::transmute(lplpdoc), ::core::mem::transmute(lpframeinfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn ShowContainerUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn QueryInsertObject<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IStorage>>(&self, lpclsid: *mut ::windows::runtime::GUID, lpstg: Param1, cp: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpclsid), lpstg.into_param().abi(), ::core::mem::transmute(cp)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Ole`*"]
    pub unsafe fn DeleteObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::IOleObject>>(&self, lpoleobj: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), lpoleobj.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn QueryAcceptData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDataObject>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, lpdataobj: Param0, lpcfformat: *mut u16, reco: u32, freally: Param3, hmetapict: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), lpdataobj.into_param().abi(), ::core::mem::transmute(lpcfformat), ::core::mem::transmute(reco), freally.into_param().abi(), ::core::mem::transmute(hmetapict)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn ContextSensitiveHelp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fentermode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), fentermode.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com`*"]
    pub unsafe fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::core::option::Option<super::super::super::System::Com::IDataObject>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpchrg), ::core::mem::transmute(reco), ::core::mem::transmute(lplpdataobj)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetDragDropEffect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdrag: Param0, grfkeystate: u32, pdweffect: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), fdrag.into_param().abi(), ::core::mem::transmute(grfkeystate), ::core::mem::transmute(pdweffect)).ok()
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Ole`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn GetContextMenu<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Ole::IOleObject>>(&self, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: Param1, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(seltype), lpoleobj.into_param().abi(), ::core::mem::transmute(lpchrg), ::core::mem::transmute(lphmenu)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRichEditOleCallback {
    type Vtable = IRichEditOleCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x00020d03_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IRichEditOleCallback> for ::windows::runtime::IUnknown {
    fn from(value: IRichEditOleCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRichEditOleCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IRichEditOleCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRichEditOleCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRichEditOleCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditOleCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lplpstg: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lplpframe: *mut ::windows::runtime::RawPtr, lplpdoc: *mut ::windows::runtime::RawPtr, lpframeinfo: *mut super::super::super::System::Ole::OIFI) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpclsid: *mut ::windows::runtime::GUID, lpstg: ::windows::runtime::RawPtr, cp: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpoleobj: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpdataobj: ::windows::runtime::RawPtr, lpcfformat: *mut u16, reco: u32, freally: super::super::super::Foundation::BOOL, hmetapict: isize) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fentermode: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fdrag: super::super::super::Foundation::BOOL, grfkeystate: u32, pdweffect: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: ::windows::runtime::RawPtr, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRicheditUiaOverrides(pub ::windows::runtime::IUnknown);
impl IRicheditUiaOverrides {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetPropertyOverrideValue(&self, propertyid: i32, pretvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertyid), ::core::mem::transmute(pretvalue)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRicheditUiaOverrides {
    type Vtable = IRicheditUiaOverrides_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::core::convert::From<IRicheditUiaOverrides> for ::windows::runtime::IUnknown {
    fn from(value: IRicheditUiaOverrides) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRicheditUiaOverrides> for ::windows::runtime::IUnknown {
    fn from(value: &IRicheditUiaOverrides) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRicheditUiaOverrides {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRicheditUiaOverrides {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRicheditUiaOverrides_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyid: i32, pretvalue: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextDisplays(pub ::windows::runtime::IUnknown);
impl ITextDisplays {}
unsafe impl ::windows::runtime::Interface for ITextDisplays {
    type Vtable = ITextDisplays_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc241f5f2_7206_11d8_a2c7_00a0d1d6c6b3);
}
impl ::core::convert::From<ITextDisplays> for ::windows::runtime::IUnknown {
    fn from(value: ITextDisplays) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextDisplays> for ::windows::runtime::IUnknown {
    fn from(value: &ITextDisplays) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextDisplays {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextDisplays {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextDisplays> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextDisplays) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextDisplays> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextDisplays) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextDisplays {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextDisplays {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDisplays_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextDocument(pub ::windows::runtime::IUnknown);
impl ITextDocument {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSelection(&self) -> ::windows::runtime::Result<ITextSelection> {
        let mut result__: <ITextSelection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextSelection>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryRanges(&self) -> ::windows::runtime::Result<ITextStoryRanges> {
        let mut result__: <ITextStoryRanges as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextStoryRanges>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSaved(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSaved(&self, value: tomConstants) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDefaultTabStop(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDefaultTabStop(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn New(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Open(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(flags), ::core::mem::transmute(codepage)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Save(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(flags), ::core::mem::transmute(codepage)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Freeze(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Unfreeze(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn BeginEditCollection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn EndEditCollection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Undo(&self, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Redo(&self, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Range(&self, cpactive: i32, cpanchor: i32) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpactive), ::core::mem::transmute(cpanchor), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn RangeFromPoint(&self, x: i32, y: i32) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), &mut result__).from_abi::<ITextRange>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITextDocument {
    type Vtable = ITextDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8cc497c0_a1df_11ce_8098_00aa0047be5d);
}
impl ::core::convert::From<ITextDocument> for ::windows::runtime::IUnknown {
    fn from(value: ITextDocument) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextDocument> for ::windows::runtime::IUnknown {
    fn from(value: &ITextDocument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextDocument> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextDocument> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pname: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsel: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstories: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: tomConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, flags: i32, codepage: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, flags: i32, codepage: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: i32, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: i32, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextDocument2(pub ::windows::runtime::IUnknown);
impl ITextDocument2 {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSelection(&self) -> ::windows::runtime::Result<ITextSelection> {
        let mut result__: <ITextSelection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextSelection>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryRanges(&self) -> ::windows::runtime::Result<ITextStoryRanges> {
        let mut result__: <ITextStoryRanges as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextStoryRanges>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSaved(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSaved(&self, value: tomConstants) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDefaultTabStop(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDefaultTabStop(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn New(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Open(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(flags), ::core::mem::transmute(codepage)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Save(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(flags), ::core::mem::transmute(codepage)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Freeze(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Unfreeze(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn BeginEditCollection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn EndEditCollection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Undo(&self, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Redo(&self, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Range(&self, cpactive: i32, cpanchor: i32) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpactive), ::core::mem::transmute(cpanchor), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn RangeFromPoint(&self, x: i32, y: i32) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCaretType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCaretType(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDisplays(&self) -> ::windows::runtime::Result<ITextDisplays> {
        let mut result__: <ITextDisplays as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextDisplays>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDocumentFont(&self) -> ::windows::runtime::Result<ITextFont2> {
        let mut result__: <ITextFont2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextFont2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDocumentFont<'a, Param0: ::windows::runtime::IntoParam<'a, ITextFont2>>(&self, pfont: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDocumentPara(&self) -> ::windows::runtime::Result<ITextPara2> {
        let mut result__: <ITextPara2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextPara2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDocumentPara<'a, Param0: ::windows::runtime::IntoParam<'a, ITextPara2>>(&self, ppara: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEastAsianFlags(&self) -> ::windows::runtime::Result<tomConstants> {
        let mut result__: <tomConstants as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<tomConstants>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetGenerator(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetIMEInProgress(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetNotificationMode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetNotificationMode(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSelection2(&self) -> ::windows::runtime::Result<ITextSelection2> {
        let mut result__: <ITextSelection2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextSelection2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryRanges2(&self) -> ::windows::runtime::Result<ITextStoryRanges2> {
        let mut result__: <ITextStoryRanges2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextStoryRanges2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTypographyOptions(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn AttachMsgFilter<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pfilter: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), pfilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(cch), ::core::mem::transmute(pcch)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCallManager(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetClientRect(&self, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(pleft), ::core::mem::transmute(ptop), ::core::mem::transmute(pright), ::core::mem::transmute(pbottom)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEffectColor(&self, index: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetImmContext(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetPreferredFont(&self, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(cp),
            ::core::mem::transmute(charrep),
            ::core::mem::transmute(options),
            ::core::mem::transmute(curcharrep),
            ::core::mem::transmute(curfontsize),
            ::core::mem::transmute(pbstr),
            ::core::mem::transmute(ppitchandfamily),
            ::core::mem::transmute(pnewfontsize),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStrings(&self) -> ::windows::runtime::Result<ITextStrings> {
        let mut result__: <ITextStrings as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextStrings>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Notify(&self, notify: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(notify)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Range2(&self, cpactive: i32, cpanchor: i32) -> ::windows::runtime::Result<ITextRange2> {
        let mut result__: <ITextRange2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpactive), ::core::mem::transmute(cpanchor), &mut result__).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn RangeFromPoint2(&self, x: i32, y: i32, r#type: i32) -> ::windows::runtime::Result<ITextRange2> {
        let mut result__: <ITextRange2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(r#type), &mut result__).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ReleaseCallManager<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pvoid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), pvoid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ReleaseImmContext(&self, context: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(context)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetEffectColor(&self, index: i32, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetTypographyOptions(&self, options: i32, mask: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(mask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SysBeep(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).60)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Update(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn UpdateWindow(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).62)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetMathProperties(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetMathProperties(&self, options: i32, mask: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(mask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetActiveStory(&self) -> ::windows::runtime::Result<ITextStory> {
        let mut result__: <ITextStory as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextStory>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetActiveStory<'a, Param0: ::windows::runtime::IntoParam<'a, ITextStory>>(&self, pstory: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).66)(::core::mem::transmute_copy(self), pstory.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetMainStory(&self) -> ::windows::runtime::Result<ITextStory> {
        let mut result__: <ITextStory as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).67)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextStory>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetNewStory(&self) -> ::windows::runtime::Result<ITextStory> {
        let mut result__: <ITextStory as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).68)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextStory>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStory(&self, index: i32) -> ::windows::runtime::Result<ITextStory> {
        let mut result__: <ITextStory as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).69)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<ITextStory>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITextDocument2 {
    type Vtable = ITextDocument2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc241f5e0_7206_11d8_a2c7_00a0d1d6c6b3);
}
impl ::core::convert::From<ITextDocument2> for ::windows::runtime::IUnknown {
    fn from(value: ITextDocument2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextDocument2> for ::windows::runtime::IUnknown {
    fn from(value: &ITextDocument2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextDocument2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextDocument2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITextDocument2> for ITextDocument {
    fn from(value: ITextDocument2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextDocument2> for ITextDocument {
    fn from(value: &ITextDocument2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextDocument> for ITextDocument2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextDocument> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextDocument> for &ITextDocument2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextDocument> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextDocument2> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextDocument2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextDocument2> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextDocument2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextDocument2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextDocument2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pname: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsel: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstories: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: tomConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, flags: i32, codepage: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, flags: i32, codepage: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: i32, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: i32, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisplays: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfont: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfont: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppara: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppara: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflags: *mut tomConstants) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsel: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstories: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poptions: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfilter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cch: i32, pcch: *const i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppvoid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontext: *mut i64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstrs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, notify: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, r#type: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvoid: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: i32, mask: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poptions: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: i32, mask: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstory: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstory: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstory: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstory: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, ppstory: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextDocument2Old(pub ::windows::runtime::IUnknown);
impl ITextDocument2Old {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSelection(&self) -> ::windows::runtime::Result<ITextSelection> {
        let mut result__: <ITextSelection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextSelection>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryRanges(&self) -> ::windows::runtime::Result<ITextStoryRanges> {
        let mut result__: <ITextStoryRanges as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextStoryRanges>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSaved(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSaved(&self, value: tomConstants) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDefaultTabStop(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDefaultTabStop(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn New(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Open(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(flags), ::core::mem::transmute(codepage)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Save(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(flags), ::core::mem::transmute(codepage)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Freeze(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Unfreeze(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn BeginEditCollection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn EndEditCollection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Undo(&self, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Redo(&self, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Range(&self, cpactive: i32, cpanchor: i32) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpactive), ::core::mem::transmute(cpanchor), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn RangeFromPoint(&self, x: i32, y: i32) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn AttachMsgFilter<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pfilter: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), pfilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetEffectColor(&self, index: i32, cr: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(cr)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEffectColor(&self, index: i32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCaretType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCaretType(&self, carettype: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(carettype)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetImmContext(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ReleaseImmContext(&self, context: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(context)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetPreferredFont(&self, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(cp),
            ::core::mem::transmute(charrep),
            ::core::mem::transmute(option),
            ::core::mem::transmute(charrepcur),
            ::core::mem::transmute(curfontsize),
            ::core::mem::transmute(pbstr),
            ::core::mem::transmute(ppitchandfamily),
            ::core::mem::transmute(pnewfontsize),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetNotificationMode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetNotificationMode(&self, mode: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetClientRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(pleft), ::core::mem::transmute(ptop), ::core::mem::transmute(pright), ::core::mem::transmute(pbottom)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSelection2(&self) -> ::windows::runtime::Result<ITextSelection> {
        let mut result__: <ITextSelection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextSelection>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFEFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn UpdateWindow(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(cch), ::core::mem::transmute(pcch)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn IMEInProgress(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SysBeep(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Update(&self, mode: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Notify(&self, notify: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(notify)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDocumentFont(&self) -> ::windows::runtime::Result<ITextFont> {
        let mut result__: <ITextFont as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDocumentPara(&self) -> ::windows::runtime::Result<ITextPara> {
        let mut result__: <ITextPara as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCallManager(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ReleaseCallManager<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pvoid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), pvoid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextDocument2Old {
    type Vtable = ITextDocument2Old_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x01c25500_4268_11d1_883a_3c8b00c10000);
}
impl ::core::convert::From<ITextDocument2Old> for ::windows::runtime::IUnknown {
    fn from(value: ITextDocument2Old) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextDocument2Old> for ::windows::runtime::IUnknown {
    fn from(value: &ITextDocument2Old) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextDocument2Old {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextDocument2Old {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITextDocument2Old> for ITextDocument {
    fn from(value: ITextDocument2Old) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextDocument2Old> for ITextDocument {
    fn from(value: &ITextDocument2Old) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextDocument> for ITextDocument2Old {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextDocument> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextDocument> for &ITextDocument2Old {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextDocument> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextDocument2Old> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextDocument2Old) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextDocument2Old> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextDocument2Old) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextDocument2Old {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextDocument2Old {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument2Old_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pname: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsel: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstories: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: tomConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, flags: i32, codepage: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, flags: i32, codepage: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: i32, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: i32, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfilter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, cr: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, pcr: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcarettype: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, carettype: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontext: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: i64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmode: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsel: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cch: i32, pcch: *const i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, notify: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitextfont: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitextpara: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppvoid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvoid: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextFont(pub ::windows::runtime::IUnknown);
impl ITextFont {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::runtime::Result<ITextFont> {
        let mut result__: <ITextFont as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDuplicate<'a, Param0: ::windows::runtime::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn CanChange(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pfont.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Reset(&self, value: tomConstants) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStyle(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetAllCaps(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetAllCaps(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetAnimation(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetAnimation(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetBackColor(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetBackColor(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetBold(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetBold(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEmboss(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetEmboss(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetForeColor(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetForeColor(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetHidden(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetHidden(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEngrave(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetEngrave(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetItalic(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetItalic(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetKerning(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetKerning(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetLanguageID(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetLanguageID(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetOutline(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetOutline(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPosition(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPosition(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetProtected(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetProtected(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetShadow(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetShadow(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSize(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSize(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSmallCaps(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSmallCaps(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSpacing(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSpacing(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStrikeThrough(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetStrikeThrough(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSubscript(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSubscript(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSuperscript(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSuperscript(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetUnderline(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).58)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetUnderline(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetWeight(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetWeight(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextFont {
    type Vtable = ITextFont_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8cc497c3_a1df_11ce_8098_00aa0047be5d);
}
impl ::core::convert::From<ITextFont> for ::windows::runtime::IUnknown {
    fn from(value: ITextFont) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextFont> for ::windows::runtime::IUnknown {
    fn from(value: &ITextFont) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextFont {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextFont {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextFont> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextFont) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextFont> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextFont) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextFont {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextFont {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextFont_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfont: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfont: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfont: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: tomConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextFont2(pub ::windows::runtime::IUnknown);
impl ITextFont2 {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::runtime::Result<ITextFont> {
        let mut result__: <ITextFont as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDuplicate<'a, Param0: ::windows::runtime::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn CanChange(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pfont.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Reset(&self, value: tomConstants) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStyle(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetAllCaps(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetAllCaps(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetAnimation(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetAnimation(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetBackColor(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetBackColor(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetBold(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetBold(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEmboss(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetEmboss(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetForeColor(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetForeColor(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetHidden(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetHidden(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEngrave(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetEngrave(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetItalic(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetItalic(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetKerning(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetKerning(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetLanguageID(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetLanguageID(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetOutline(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetOutline(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPosition(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPosition(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetProtected(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetProtected(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetShadow(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetShadow(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSize(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSize(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSmallCaps(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSmallCaps(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSpacing(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSpacing(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStrikeThrough(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetStrikeThrough(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSubscript(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSubscript(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSuperscript(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSuperscript(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetUnderline(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).58)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetUnderline(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetWeight(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetWeight(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).62)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetAutoLigatures(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetAutoLigatures(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetAutospaceAlpha(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetAutospaceAlpha(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetAutospaceNumeric(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).67)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetAutospaceNumeric(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetAutospaceParens(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).69)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetAutospaceParens(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCharRep(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).71)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCharRep(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).72)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCompressionMode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).73)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCompressionMode(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).74)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCookie(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).75)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCookie(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).76)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDoubleStrike(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).77)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDoubleStrike(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).78)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDuplicate2(&self) -> ::windows::runtime::Result<ITextFont2> {
        let mut result__: <ITextFont2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).79)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextFont2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDuplicate2<'a, Param0: ::windows::runtime::IntoParam<'a, ITextFont2>>(&self, pfont: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).80)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetLinkType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).81)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetMathZone(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).82)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetMathZone(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).83)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetModWidthPairs(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).84)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetModWidthPairs(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).85)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetModWidthSpace(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).86)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetModWidthSpace(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).87)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetOldNumbers(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).88)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetOldNumbers(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).89)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetOverlapping(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).90)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetOverlapping(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).91)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPositionSubSuper(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).92)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPositionSubSuper(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).93)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetScaling(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).94)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetScaling(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).95)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSpaceExtension(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).96)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSpaceExtension(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).97)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetUnderlinePositionMode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).98)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetUnderlinePositionMode(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).99)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).100)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvalue), ::core::mem::transmute(pmask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEffects2(&self, pvalue: *mut i32, pmask: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).101)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvalue), ::core::mem::transmute(pmask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).102)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPropertyInfo(&self, index: i32, ptype: *mut i32, pvalue: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).103)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn IsEqual2<'a, Param0: ::windows::runtime::IntoParam<'a, ITextFont2>>(&self, pfont: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).104)(::core::mem::transmute_copy(self), pfont.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetEffects(&self, value: i32, mask: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).105)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(mask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetEffects2(&self, value: i32, mask: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).106)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(mask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).107)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextFont2 {
    type Vtable = ITextFont2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc241f5e3_7206_11d8_a2c7_00a0d1d6c6b3);
}
impl ::core::convert::From<ITextFont2> for ::windows::runtime::IUnknown {
    fn from(value: ITextFont2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextFont2> for ::windows::runtime::IUnknown {
    fn from(value: &ITextFont2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextFont2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextFont2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITextFont2> for ITextFont {
    fn from(value: ITextFont2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextFont2> for ITextFont {
    fn from(value: &ITextFont2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextFont> for ITextFont2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextFont> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextFont> for &ITextFont2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextFont> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextFont2> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextFont2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextFont2> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextFont2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextFont2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextFont2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextFont2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfont: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfont: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfont: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: tomConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfont: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfont: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32, pmask: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32, pmask: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, ptype: *mut i32, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfont: ::windows::runtime::RawPtr, pb: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32, mask: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32, mask: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, value: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextHost(pub ::windows::runtime::IUnknown);
impl ITextHost {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn TxGetDC(&self) -> super::super::super::Graphics::Gdi::HDC {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn TxReleaseDC<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, hdc: Param0) -> i32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hdc.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxShowScrollBar<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fnbar: i32, fshow: Param1) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(fnbar), fshow.into_param().abi()))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TxEnableScrollBar(&self, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(fusbflags), ::core::mem::transmute(fuarrowflags)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSetScrollRange<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: Param3) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fnbar), ::core::mem::transmute(nminpos), ::core::mem::transmute(nmaxpos), fredraw.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSetScrollPos<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fnbar: i32, npos: i32, fredraw: Param2) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(fnbar), ::core::mem::transmute(npos), fredraw.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxInvalidateRect<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, prc: *mut super::super::super::Foundation::RECT, fmode: Param1) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), fmode.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxViewChange<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fupdate: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), fupdate.into_param().abi()))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn TxCreateCaret<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HBITMAP>>(&self, hbmp: Param0, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), hbmp.into_param().abi(), ::core::mem::transmute(xwidth), ::core::mem::transmute(yheight)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxShowCaret<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fshow: Param0) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), fshow.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSetCaretPos(&self, x: i32, y: i32) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSetTimer(&self, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(idtimer), ::core::mem::transmute(utimeout)))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxKillTimer(&self, idtimer: u32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(idtimer)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TxScrollWindowEx<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HRGN>>(&self, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: Param4, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).16)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dx),
            ::core::mem::transmute(dy),
            ::core::mem::transmute(lprcscroll),
            ::core::mem::transmute(lprcclip),
            hrgnupdate.into_param().abi(),
            ::core::mem::transmute(lprcupdate),
            ::core::mem::transmute(fuscroll),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSetCapture<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fcapture: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), fcapture.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxSetFocus(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TxSetCursor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::WindowsAndMessaging::HCURSOR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, hcur: Param0, ftext: Param1) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), hcur.into_param().abi(), ftext.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxScreenToClient(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppt)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxClientToScreen(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppt)))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxActivate(&self, ploldstate: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ploldstate)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxDeactivate(&self, lnewstate: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnewstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxGetClientRect(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxGetViewInset(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetCharFormat(&self, ppcf: *const *const CHARFORMATW) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppcf)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetParaFormat(&self, pppf: *const *const PARAFORMAT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppf)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetSysColor(&self, nindex: i32) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex)))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetBackStyle(&self, pstyle: *mut TXTBACKSTYLE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstyle)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetMaxLength(&self, plength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(plength)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetScrollBars(&self, pdwscrollbar: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwscrollbar)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetPasswordChar(&self) -> ::windows::runtime::Result<i8> {
        let mut result__: <i8 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i8>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetAcceleratorPos(&self, pcp: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcp)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxGetExtent(&self, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpextent)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn OnTxCharFormatChange(&self, pcf: *const CHARFORMATW) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcf)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn OnTxParaFormatChange(&self, ppf: *const PARAFORMAT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppf)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetPropertyBits(&self, dwmask: u32, pdwbits: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmask), ::core::mem::transmute(pdwbits)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxNotify(&self, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(inotify), ::core::mem::transmute(pv)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Globalization`*"]
    pub unsafe fn TxImmGetContext(&self) -> super::super::super::Globalization::HIMC {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Globalization")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Globalization`*"]
    pub unsafe fn TxImmReleaseContext<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), himc.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetSelectionBarWidth(&self, lselbarwidth: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(lselbarwidth)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextHost {
    type Vtable = ITextHost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::core::convert::From<ITextHost> for ::windows::runtime::IUnknown {
    fn from(value: ITextHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextHost> for ::windows::runtime::IUnknown {
    fn from(value: &ITextHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHost_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::super::Graphics::Gdi::HDC,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdc: super::super::super::Graphics::Gdi::HDC) -> i32,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fnbar: i32, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fnbar: i32, npos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prc: *mut super::super::super::Foundation::RECT, fmode: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fupdate: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, idtimer: u32),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcapture: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: super::super::super::Foundation::BOOL),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ploldstate: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lnewstate: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prc: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prc: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcf: *const *const CHARFORMATW) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppf: *const *const PARAFORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nindex: i32) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstyle: *mut TXTBACKSTYLE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwscrollbar: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pch: *mut i8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcp: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcf: *const CHARFORMATW) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppf: *const PARAFORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmask: u32, pdwbits: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::super::Globalization::HIMC,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, himc: super::super::super::Globalization::HIMC),
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lselbarwidth: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextHost2(pub ::windows::runtime::IUnknown);
impl ITextHost2 {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn TxGetDC(&self) -> super::super::super::Graphics::Gdi::HDC {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn TxReleaseDC<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, hdc: Param0) -> i32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hdc.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxShowScrollBar<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fnbar: i32, fshow: Param1) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(fnbar), fshow.into_param().abi()))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TxEnableScrollBar(&self, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(fusbflags), ::core::mem::transmute(fuarrowflags)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSetScrollRange<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: Param3) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fnbar), ::core::mem::transmute(nminpos), ::core::mem::transmute(nmaxpos), fredraw.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSetScrollPos<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fnbar: i32, npos: i32, fredraw: Param2) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(fnbar), ::core::mem::transmute(npos), fredraw.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxInvalidateRect<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, prc: *mut super::super::super::Foundation::RECT, fmode: Param1) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), fmode.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxViewChange<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fupdate: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), fupdate.into_param().abi()))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn TxCreateCaret<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HBITMAP>>(&self, hbmp: Param0, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), hbmp.into_param().abi(), ::core::mem::transmute(xwidth), ::core::mem::transmute(yheight)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxShowCaret<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fshow: Param0) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), fshow.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSetCaretPos(&self, x: i32, y: i32) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSetTimer(&self, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(idtimer), ::core::mem::transmute(utimeout)))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxKillTimer(&self, idtimer: u32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(idtimer)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TxScrollWindowEx<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HRGN>>(&self, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: Param4, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).16)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dx),
            ::core::mem::transmute(dy),
            ::core::mem::transmute(lprcscroll),
            ::core::mem::transmute(lprcclip),
            hrgnupdate.into_param().abi(),
            ::core::mem::transmute(lprcupdate),
            ::core::mem::transmute(fuscroll),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSetCapture<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fcapture: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), fcapture.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxSetFocus(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TxSetCursor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::WindowsAndMessaging::HCURSOR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, hcur: Param0, ftext: Param1) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), hcur.into_param().abi(), ftext.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxScreenToClient(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppt)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxClientToScreen(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppt)))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxActivate(&self, ploldstate: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ploldstate)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxDeactivate(&self, lnewstate: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnewstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxGetClientRect(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxGetViewInset(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetCharFormat(&self, ppcf: *const *const CHARFORMATW) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppcf)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetParaFormat(&self, pppf: *const *const PARAFORMAT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppf)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetSysColor(&self, nindex: i32) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex)))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetBackStyle(&self, pstyle: *mut TXTBACKSTYLE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstyle)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetMaxLength(&self, plength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(plength)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetScrollBars(&self, pdwscrollbar: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwscrollbar)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetPasswordChar(&self) -> ::windows::runtime::Result<i8> {
        let mut result__: <i8 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i8>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetAcceleratorPos(&self, pcp: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcp)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxGetExtent(&self, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpextent)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn OnTxCharFormatChange(&self, pcf: *const CHARFORMATW) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcf)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn OnTxParaFormatChange(&self, ppf: *const PARAFORMAT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppf)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetPropertyBits(&self, dwmask: u32, pdwbits: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmask), ::core::mem::transmute(pdwbits)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxNotify(&self, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(inotify), ::core::mem::transmute(pv)).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Globalization`*"]
    pub unsafe fn TxImmGetContext(&self) -> super::super::super::Globalization::HIMC {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Globalization")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Globalization`*"]
    pub unsafe fn TxImmReleaseContext<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), himc.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetSelectionBarWidth(&self, lselbarwidth: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(lselbarwidth)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxIsDoubleClickPending(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxGetWindow(&self, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(phwnd)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxSetForegroundWindow(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn TxGetPalette(&self) -> super::super::super::Graphics::Gdi::HPALETTE {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetEastAsianFlags(&self, pflags: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(pflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TxSetCursor2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::WindowsAndMessaging::HCURSOR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, hcur: Param0, btext: Param1) -> super::super::WindowsAndMessaging::HCURSOR {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), hcur.into_param().abi(), btext.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxFreeTextServicesNotification(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetEditStyle(&self, dwitem: u32, pdwdata: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwitem), ::core::mem::transmute(pdwdata)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetWindowStyles(&self, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwstyle), ::core::mem::transmute(pdwexstyle)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn TxShowDropCaret<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, fshow: Param0, hdc: Param1, prc: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), fshow.into_param().abi(), hdc.into_param().abi(), ::core::mem::transmute(prc)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxDestroyCaret(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetHorzExtent(&self, plhorzextent: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(plhorzextent)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextHost2 {
    type Vtable = ITextHost2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::core::convert::From<ITextHost2> for ::windows::runtime::IUnknown {
    fn from(value: ITextHost2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextHost2> for ::windows::runtime::IUnknown {
    fn from(value: &ITextHost2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextHost2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextHost2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITextHost2> for ITextHost {
    fn from(value: ITextHost2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextHost2> for ITextHost {
    fn from(value: &ITextHost2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextHost> for ITextHost2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextHost> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextHost> for &ITextHost2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextHost> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHost2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::super::Graphics::Gdi::HDC,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdc: super::super::super::Graphics::Gdi::HDC) -> i32,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fnbar: i32, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fnbar: i32, npos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prc: *mut super::super::super::Foundation::RECT, fmode: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fupdate: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, idtimer: u32),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcapture: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: super::super::super::Foundation::BOOL),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ploldstate: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lnewstate: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prc: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prc: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcf: *const *const CHARFORMATW) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppf: *const *const PARAFORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nindex: i32) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstyle: *mut TXTBACKSTYLE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwscrollbar: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pch: *mut i8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcp: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcf: *const CHARFORMATW) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppf: *const PARAFORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmask: u32, pdwbits: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::super::Globalization::HIMC,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, himc: super::super::super::Globalization::HIMC),
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lselbarwidth: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::super::Graphics::Gdi::HPALETTE,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflags: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: super::super::super::Foundation::BOOL) -> super::super::WindowsAndMessaging::HCURSOR,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwitem: u32, pdwdata: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::super::Foundation::BOOL, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plhorzextent: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextPara(pub ::windows::runtime::IUnknown);
impl ITextPara {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::runtime::Result<ITextPara> {
        let mut result__: <ITextPara as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDuplicate<'a, Param0: ::windows::runtime::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn CanChange(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ppara.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Reset(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStyle(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetAlignment(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetAlignment(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetHyphenation(&self) -> ::windows::runtime::Result<tomConstants> {
        let mut result__: <tomConstants as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<tomConstants>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetHyphenation(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFirstLineIndent(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetKeepTogether(&self) -> ::windows::runtime::Result<tomConstants> {
        let mut result__: <tomConstants as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<tomConstants>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetKeepTogether(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetKeepWithNext(&self) -> ::windows::runtime::Result<tomConstants> {
        let mut result__: <tomConstants as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<tomConstants>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetLeftIndent(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetLineSpacing(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetLineSpacingRule(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetListAlignment(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetListAlignment(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetListLevelIndex(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetListLevelIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetListStart(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetListStart(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetListTab(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetListTab(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetListType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetListType(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetNoLineNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetNoLineNumber(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPageBreakBefore(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPageBreakBefore(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetRightIndent(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetRightIndent(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetIndents(&self, first: f32, left: f32, right: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(first), ::core::mem::transmute(left), ::core::mem::transmute(right)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetLineSpacing(&self, rule: i32, spacing: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(rule), ::core::mem::transmute(spacing)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSpaceAfter(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSpaceAfter(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSpaceBefore(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSpaceBefore(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetWidowControl(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetWidowControl(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTabCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn AddTab(&self, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(tbpos), ::core::mem::transmute(tbalign), ::core::mem::transmute(tbleader)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ClearAllTabs(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn DeleteTab(&self, tbpos: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(tbpos)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTab(&self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(itab), ::core::mem::transmute(ptbpos), ::core::mem::transmute(ptbalign), ::core::mem::transmute(ptbleader)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextPara {
    type Vtable = ITextPara_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8cc497c4_a1df_11ce_8098_00aa0047be5d);
}
impl ::core::convert::From<ITextPara> for ::windows::runtime::IUnknown {
    fn from(value: ITextPara) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextPara> for ::windows::runtime::IUnknown {
    fn from(value: &ITextPara) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextPara {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextPara {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextPara> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextPara) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextPara> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextPara) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextPara {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextPara {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPara_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppara: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppara: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppara: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut tomConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut tomConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut tomConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, first: f32, left: f32, right: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rule: i32, spacing: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tbpos: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextPara2(pub ::windows::runtime::IUnknown);
impl ITextPara2 {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::runtime::Result<ITextPara> {
        let mut result__: <ITextPara as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDuplicate<'a, Param0: ::windows::runtime::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn CanChange(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ppara.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Reset(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStyle(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetAlignment(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetAlignment(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetHyphenation(&self) -> ::windows::runtime::Result<tomConstants> {
        let mut result__: <tomConstants as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<tomConstants>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetHyphenation(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFirstLineIndent(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetKeepTogether(&self) -> ::windows::runtime::Result<tomConstants> {
        let mut result__: <tomConstants as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<tomConstants>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetKeepTogether(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetKeepWithNext(&self) -> ::windows::runtime::Result<tomConstants> {
        let mut result__: <tomConstants as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<tomConstants>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetLeftIndent(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetLineSpacing(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetLineSpacingRule(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetListAlignment(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetListAlignment(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetListLevelIndex(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetListLevelIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetListStart(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetListStart(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetListTab(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetListTab(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetListType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetListType(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetNoLineNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetNoLineNumber(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPageBreakBefore(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPageBreakBefore(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetRightIndent(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetRightIndent(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetIndents(&self, first: f32, left: f32, right: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(first), ::core::mem::transmute(left), ::core::mem::transmute(right)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetLineSpacing(&self, rule: i32, spacing: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(rule), ::core::mem::transmute(spacing)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSpaceAfter(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSpaceAfter(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSpaceBefore(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSpaceBefore(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetWidowControl(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetWidowControl(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTabCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn AddTab(&self, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(tbpos), ::core::mem::transmute(tbalign), ::core::mem::transmute(tbleader)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ClearAllTabs(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn DeleteTab(&self, tbpos: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(tbpos)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTab(&self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(itab), ::core::mem::transmute(ptbpos), ::core::mem::transmute(ptbalign), ::core::mem::transmute(ptbleader)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetBorders(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDuplicate2(&self) -> ::windows::runtime::Result<ITextPara2> {
        let mut result__: <ITextPara2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextPara2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDuplicate2<'a, Param0: ::windows::runtime::IntoParam<'a, ITextPara2>>(&self, ppara: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFontAlignment(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).58)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFontAlignment(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetHangingPunctuation(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetHangingPunctuation(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSnapToGrid(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).62)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetSnapToGrid(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTrimPunctuationAtStart(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).64)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetTrimPunctuationAtStart(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvalue), ::core::mem::transmute(pmask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn IsEqual2<'a, Param0: ::windows::runtime::IntoParam<'a, ITextPara2>>(&self, ppara: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ppara.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetEffects(&self, value: i32, mask: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).69)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(mask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextPara2 {
    type Vtable = ITextPara2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc241f5e4_7206_11d8_a2c7_00a0d1d6c6b3);
}
impl ::core::convert::From<ITextPara2> for ::windows::runtime::IUnknown {
    fn from(value: ITextPara2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextPara2> for ::windows::runtime::IUnknown {
    fn from(value: &ITextPara2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextPara2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextPara2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITextPara2> for ITextPara {
    fn from(value: ITextPara2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextPara2> for ITextPara {
    fn from(value: &ITextPara2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextPara> for ITextPara2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextPara> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextPara> for &ITextPara2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextPara> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextPara2> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextPara2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextPara2> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextPara2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextPara2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextPara2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPara2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppara: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppara: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppara: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut tomConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut tomConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut tomConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, first: f32, left: f32, right: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rule: i32, spacing: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tbpos: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppborders: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppara: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppara: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32, pmask: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppara: ::windows::runtime::RawPtr, pb: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32, mask: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, value: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextRange(pub ::windows::runtime::IUnknown);
impl ITextRange {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetText(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetChar(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetChar(&self, char: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(char)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFormattedText(&self) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFormattedText<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStart(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpfirst)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEnd(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(cplim)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFont(&self) -> ::windows::runtime::Result<ITextFont> {
        let mut result__: <ITextFont as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFont<'a, Param0: ::windows::runtime::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPara(&self) -> ::windows::runtime::Result<ITextPara> {
        let mut result__: <ITextPara as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPara<'a, Param0: ::windows::runtime::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryLength(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstart)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Expand(&self, unit: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(index), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpanchor), ::core::mem::transmute(cpactive)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn InRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn InStory<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Select(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn FindText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn FindTextStart<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn FindTextEnd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Cut(&self) -> ::windows::runtime::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Copy(&self) -> ::windows::runtime::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn CanEdit(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ChangeCase(&self, r#type: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPoint(&self, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(px), ::core::mem::transmute(py)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(r#type), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITextRange {
    type Vtable = ITextRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8cc497c2_a1df_11ce_8098_00aa0047be5d);
}
impl ::core::convert::From<ITextRange> for ::windows::runtime::IUnknown {
    fn from(value: ITextRange) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextRange> for ::windows::runtime::IUnknown {
    fn from(value: &ITextRange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextRange> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextRange> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchar: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, char: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcpfirst: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpfirst: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcplim: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cplim: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfont: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfont: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppara: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppara: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstart: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, pindex: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, index: i32, extend: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpanchor: i32, cpactive: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, format: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, format: i32, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextRange2(pub ::windows::runtime::IUnknown);
impl ITextRange2 {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetText(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetChar(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetChar(&self, char: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(char)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFormattedText(&self) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFormattedText<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStart(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpfirst)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEnd(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(cplim)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFont(&self) -> ::windows::runtime::Result<ITextFont> {
        let mut result__: <ITextFont as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFont<'a, Param0: ::windows::runtime::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPara(&self) -> ::windows::runtime::Result<ITextPara> {
        let mut result__: <ITextPara as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPara<'a, Param0: ::windows::runtime::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryLength(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstart)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Expand(&self, unit: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(index), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpanchor), ::core::mem::transmute(cpactive)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn InRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn InStory<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Select(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn FindText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn FindTextStart<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn FindTextEnd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Cut(&self) -> ::windows::runtime::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Copy(&self) -> ::windows::runtime::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn CanEdit(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ChangeCase(&self, r#type: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPoint(&self, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(px), ::core::mem::transmute(py)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(r#type), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).58)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn HomeKey(&self, unit: tomConstants, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn EndKey(&self, unit: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TypeText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).67)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCch(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).68)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCells(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).69)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetColumn(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).70)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).71)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDuplicate2(&self) -> ::windows::runtime::Result<ITextRange2> {
        let mut result__: <ITextRange2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).72)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFont2(&self) -> ::windows::runtime::Result<ITextFont2> {
        let mut result__: <ITextFont2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).73)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextFont2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFont2<'a, Param0: ::windows::runtime::IntoParam<'a, ITextFont2>>(&self, pfont: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).74)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFormattedText2(&self) -> ::windows::runtime::Result<ITextRange2> {
        let mut result__: <ITextRange2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).75)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFormattedText2<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange2>>(&self, prange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).76)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetGravity(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).77)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetGravity(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).78)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPara2(&self) -> ::windows::runtime::Result<ITextPara2> {
        let mut result__: <ITextPara2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).79)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextPara2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPara2<'a, Param0: ::windows::runtime::IntoParam<'a, ITextPara2>>(&self, ppara: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).80)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetRow(&self) -> ::windows::runtime::Result<ITextRow> {
        let mut result__: <ITextRow as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).81)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRow>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStartPara(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).82)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTable(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).83)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetURL(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).84)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SetURL<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).85)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn AddSubrange(&self, cp1: i32, cp2: i32, activate: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).86)(::core::mem::transmute_copy(self), ::core::mem::transmute(cp1), ::core::mem::transmute(cp2), ::core::mem::transmute(activate)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn BuildUpMath(&self, flags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).87)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn DeleteSubrange(&self, cpfirst: i32, cplim: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).88)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpfirst), ::core::mem::transmute(cplim)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Find<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange2>>(&self, prange: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).89)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetChar2(&self, pchar: *mut i32, offset: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).90)(::core::mem::transmute_copy(self), ::core::mem::transmute(pchar), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDropCap(&self, pcline: *mut i32, pposition: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).91)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcline), ::core::mem::transmute(pposition)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetInlineObject(&self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).92)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ptype),
            ::core::mem::transmute(palign),
            ::core::mem::transmute(pchar),
            ::core::mem::transmute(pchar1),
            ::core::mem::transmute(pchar2),
            ::core::mem::transmute(pcount),
            ::core::mem::transmute(ptexstyle),
            ::core::mem::transmute(pccol),
            ::core::mem::transmute(plevel),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).93)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).94)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(pleft), ::core::mem::transmute(ptop), ::core::mem::transmute(pright), ::core::mem::transmute(pbottom), ::core::mem::transmute(phit)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSubrange(&self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).95)(::core::mem::transmute_copy(self), ::core::mem::transmute(isubrange), ::core::mem::transmute(pcpfirst), ::core::mem::transmute(pcplim)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetText2(&self, flags: i32) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).96)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn HexToUnicode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).97)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn InsertTable(&self, ccol: i32, crow: i32, autofit: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).98)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccol), ::core::mem::transmute(crow), ::core::mem::transmute(autofit)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Linearize(&self, flags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).99)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetActiveSubrange(&self, cpanchor: i32, cpactive: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).100)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpanchor), ::core::mem::transmute(cpactive)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDropCap(&self, cline: i32, position: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).101)(::core::mem::transmute_copy(self), ::core::mem::transmute(cline), ::core::mem::transmute(position)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).102)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SetText2<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, flags: i32, bstr: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).103)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn UnicodeToHex(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).104)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetInlineObject(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).105)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(align), ::core::mem::transmute(char), ::core::mem::transmute(char1), ::core::mem::transmute(char2), ::core::mem::transmute(count), ::core::mem::transmute(texstyle), ::core::mem::transmute(ccol)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetMathFunctionType<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).106)(::core::mem::transmute_copy(self), bstr.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn InsertImage<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: Param4, pstream: Param5) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).107)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(ascent), ::core::mem::transmute(r#type), bstralttext.into_param().abi(), pstream.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextRange2 {
    type Vtable = ITextRange2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc241f5e2_7206_11d8_a2c7_00a0d1d6c6b3);
}
impl ::core::convert::From<ITextRange2> for ::windows::runtime::IUnknown {
    fn from(value: ITextRange2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextRange2> for ::windows::runtime::IUnknown {
    fn from(value: &ITextRange2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextRange2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextRange2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITextRange2> for ITextSelection {
    fn from(value: ITextRange2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextRange2> for ITextSelection {
    fn from(value: &ITextRange2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextSelection> for ITextRange2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextSelection> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextSelection> for &ITextRange2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextSelection> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextRange2> for ITextRange {
    fn from(value: ITextRange2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextRange2> for ITextRange {
    fn from(value: &ITextRange2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for ITextRange2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for &ITextRange2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextRange2> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextRange2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextRange2> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextRange2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextRange2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextRange2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRange2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchar: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, char: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcpfirst: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpfirst: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcplim: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cplim: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfont: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfont: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppara: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppara: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstart: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, pindex: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, index: i32, extend: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpanchor: i32, cpactive: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, format: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, format: i32, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: tomConstants, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcch: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcells: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcolumn: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfont: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfont: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppara: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppara: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprow: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptable: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cp1: i32, cp2: i32, activate: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpfirst: i32, cplim: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, count: i32, flags: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchar: *mut i32, offset: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcline: *mut i32, pposition: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ccol: i32, crow: i32, autofit: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpanchor: i32, cpactive: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cline: i32, position: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, value: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextRow(pub ::windows::runtime::IUnknown);
impl ITextRow {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetAlignment(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetAlignment(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCellCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCellCount(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCellCountCache(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCellCountCache(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCellIndex(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCellIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCellMargin(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCellMargin(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetHeight(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetHeight(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetIndent(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetIndent(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetKeepTogether(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetKeepTogether(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetKeepWithNext(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetNestLevel(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetRTL(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetRTL(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCellAlignment(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCellAlignment(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCellColorBack(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCellColorBack(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCellColorFore(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCellColorFore(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCellMergeFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCellMergeFlags(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCellShading(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCellShading(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCellVerticalText(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCellVerticalText(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCellWidth(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCellWidth(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCellBorderColors(&self, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcrleft), ::core::mem::transmute(pcrtop), ::core::mem::transmute(pcrright), ::core::mem::transmute(pcrbottom)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCellBorderWidths(&self, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(pduleft), ::core::mem::transmute(pdutop), ::core::mem::transmute(pduright), ::core::mem::transmute(pdubottom)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCellBorderColors(&self, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(crleft), ::core::mem::transmute(crtop), ::core::mem::transmute(crright), ::core::mem::transmute(crbottom)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetCellBorderWidths(&self, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(duleft), ::core::mem::transmute(dutop), ::core::mem::transmute(duright), ::core::mem::transmute(dubottom)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Apply(&self, crow: i32, flags: tomConstants) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(crow), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn CanChange(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Insert(&self, crow: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(crow)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRow>>(&self, prow: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), prow.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Reset(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextRow {
    type Vtable = ITextRow_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc241f5ef_7206_11d8_a2c7_00a0d1d6c6b3);
}
impl ::core::convert::From<ITextRow> for ::windows::runtime::IUnknown {
    fn from(value: ITextRow) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextRow> for ::windows::runtime::IUnknown {
    fn from(value: &ITextRow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextRow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextRow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextRow> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextRow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextRow> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextRow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextRow {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextRow {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRow_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, crow: i32, flags: tomConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, crow: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prow: ::windows::runtime::RawPtr, pb: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, value: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextSelection(pub ::windows::runtime::IUnknown);
impl ITextSelection {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetText(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetChar(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetChar(&self, char: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(char)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFormattedText(&self) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFormattedText<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStart(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpfirst)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEnd(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(cplim)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFont(&self) -> ::windows::runtime::Result<ITextFont> {
        let mut result__: <ITextFont as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFont<'a, Param0: ::windows::runtime::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPara(&self) -> ::windows::runtime::Result<ITextPara> {
        let mut result__: <ITextPara as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPara<'a, Param0: ::windows::runtime::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryLength(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstart)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Expand(&self, unit: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(index), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpanchor), ::core::mem::transmute(cpactive)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn InRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn InStory<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Select(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn FindText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn FindTextStart<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn FindTextEnd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Cut(&self) -> ::windows::runtime::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Copy(&self) -> ::windows::runtime::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn CanEdit(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ChangeCase(&self, r#type: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPoint(&self, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(px), ::core::mem::transmute(py)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(r#type), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).58)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn HomeKey(&self, unit: tomConstants, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn EndKey(&self, unit: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TypeText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).67)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextSelection {
    type Vtable = ITextSelection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8cc497c1_a1df_11ce_8098_00aa0047be5d);
}
impl ::core::convert::From<ITextSelection> for ::windows::runtime::IUnknown {
    fn from(value: ITextSelection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextSelection> for ::windows::runtime::IUnknown {
    fn from(value: &ITextSelection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITextSelection> for ITextRange {
    fn from(value: ITextSelection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextSelection> for ITextRange {
    fn from(value: &ITextSelection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for &ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextSelection> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextSelection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextSelection> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextSelection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchar: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, char: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcpfirst: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpfirst: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcplim: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cplim: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfont: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfont: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppara: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppara: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstart: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, pindex: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, index: i32, extend: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpanchor: i32, cpactive: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, format: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, format: i32, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: tomConstants, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextSelection2(pub ::windows::runtime::IUnknown);
impl ITextSelection2 {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetText(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetChar(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetChar(&self, char: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(char)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFormattedText(&self) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFormattedText<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStart(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpfirst)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEnd(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(cplim)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFont(&self) -> ::windows::runtime::Result<ITextFont> {
        let mut result__: <ITextFont as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFont<'a, Param0: ::windows::runtime::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPara(&self) -> ::windows::runtime::Result<ITextPara> {
        let mut result__: <ITextPara as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPara<'a, Param0: ::windows::runtime::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryLength(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStoryType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstart)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Expand(&self, unit: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(index), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpanchor), ::core::mem::transmute(cpactive)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn InRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn InStory<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Select(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn FindText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn FindTextStart<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn FindTextEnd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Cut(&self) -> ::windows::runtime::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Copy(&self) -> ::windows::runtime::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: <super::super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn CanEdit(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ChangeCase(&self, r#type: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPoint(&self, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(px), ::core::mem::transmute(py)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(r#type), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).58)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn HomeKey(&self, unit: tomConstants, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn EndKey(&self, unit: i32, extend: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TypeText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).67)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCch(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).68)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCells(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).69)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetColumn(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).70)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).71)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDuplicate2(&self) -> ::windows::runtime::Result<ITextRange2> {
        let mut result__: <ITextRange2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).72)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFont2(&self) -> ::windows::runtime::Result<ITextFont2> {
        let mut result__: <ITextFont2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).73)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextFont2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFont2<'a, Param0: ::windows::runtime::IntoParam<'a, ITextFont2>>(&self, pfont: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).74)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetFormattedText2(&self) -> ::windows::runtime::Result<ITextRange2> {
        let mut result__: <ITextRange2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).75)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFormattedText2<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange2>>(&self, prange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).76)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetGravity(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).77)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetGravity(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).78)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetPara2(&self) -> ::windows::runtime::Result<ITextPara2> {
        let mut result__: <ITextPara2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).79)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextPara2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetPara2<'a, Param0: ::windows::runtime::IntoParam<'a, ITextPara2>>(&self, ppara: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).80)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetRow(&self) -> ::windows::runtime::Result<ITextRow> {
        let mut result__: <ITextRow as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).81)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITextRow>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetStartPara(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).82)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTable(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).83)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetURL(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).84)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SetURL<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).85)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn AddSubrange(&self, cp1: i32, cp2: i32, activate: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).86)(::core::mem::transmute_copy(self), ::core::mem::transmute(cp1), ::core::mem::transmute(cp2), ::core::mem::transmute(activate)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn BuildUpMath(&self, flags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).87)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn DeleteSubrange(&self, cpfirst: i32, cplim: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).88)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpfirst), ::core::mem::transmute(cplim)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Find<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange2>>(&self, prange: Param0, count: i32, flags: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).89)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetChar2(&self, pchar: *mut i32, offset: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).90)(::core::mem::transmute_copy(self), ::core::mem::transmute(pchar), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDropCap(&self, pcline: *mut i32, pposition: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).91)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcline), ::core::mem::transmute(pposition)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetInlineObject(&self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).92)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(ptype),
            ::core::mem::transmute(palign),
            ::core::mem::transmute(pchar),
            ::core::mem::transmute(pchar1),
            ::core::mem::transmute(pchar2),
            ::core::mem::transmute(pcount),
            ::core::mem::transmute(ptexstyle),
            ::core::mem::transmute(pccol),
            ::core::mem::transmute(plevel),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).93)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).94)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(pleft), ::core::mem::transmute(ptop), ::core::mem::transmute(pright), ::core::mem::transmute(pbottom), ::core::mem::transmute(phit)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetSubrange(&self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).95)(::core::mem::transmute_copy(self), ::core::mem::transmute(isubrange), ::core::mem::transmute(pcpfirst), ::core::mem::transmute(pcplim)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetText2(&self, flags: i32) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).96)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn HexToUnicode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).97)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn InsertTable(&self, ccol: i32, crow: i32, autofit: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).98)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccol), ::core::mem::transmute(crow), ::core::mem::transmute(autofit)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Linearize(&self, flags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).99)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetActiveSubrange(&self, cpanchor: i32, cpactive: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).100)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpanchor), ::core::mem::transmute(cpactive)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetDropCap(&self, cline: i32, position: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).101)(::core::mem::transmute_copy(self), ::core::mem::transmute(cline), ::core::mem::transmute(position)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).102)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SetText2<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, flags: i32, bstr: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).103)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn UnicodeToHex(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).104)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetInlineObject(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).105)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(align), ::core::mem::transmute(char), ::core::mem::transmute(char1), ::core::mem::transmute(char2), ::core::mem::transmute(count), ::core::mem::transmute(texstyle), ::core::mem::transmute(ccol)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetMathFunctionType<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).106)(::core::mem::transmute_copy(self), bstr.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn InsertImage<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: Param4, pstream: Param5) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).107)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(ascent), ::core::mem::transmute(r#type), bstralttext.into_param().abi(), pstream.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextSelection2 {
    type Vtable = ITextSelection2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc241f5e1_7206_11d8_a2c7_00a0d1d6c6b3);
}
impl ::core::convert::From<ITextSelection2> for ::windows::runtime::IUnknown {
    fn from(value: ITextSelection2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextSelection2> for ::windows::runtime::IUnknown {
    fn from(value: &ITextSelection2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextSelection2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextSelection2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITextSelection2> for ITextRange2 {
    fn from(value: ITextSelection2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextSelection2> for ITextRange2 {
    fn from(value: &ITextSelection2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange2> for ITextSelection2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange2> for &ITextSelection2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextSelection2> for ITextSelection {
    fn from(value: ITextSelection2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextSelection2> for ITextSelection {
    fn from(value: &ITextSelection2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextSelection> for ITextSelection2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextSelection> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextSelection> for &ITextSelection2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextSelection> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextSelection2> for ITextRange {
    fn from(value: ITextSelection2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextSelection2> for ITextRange {
    fn from(value: &ITextSelection2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for ITextSelection2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for &ITextSelection2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextSelection2> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextSelection2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextSelection2> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextSelection2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextSelection2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextSelection2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelection2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchar: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, char: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcpfirst: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpfirst: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcplim: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cplim: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfont: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfont: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppara: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppara: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstart: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, pindex: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, index: i32, extend: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpanchor: i32, cpactive: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cset: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, format: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, format: i32, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: tomConstants, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcch: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcells: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcolumn: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfont: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfont: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppara: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppara: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprow: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptable: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cp1: i32, cp2: i32, activate: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpfirst: i32, cplim: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, count: i32, flags: i32, pdelta: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchar: *mut i32, offset: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcline: *mut i32, pposition: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ccol: i32, crow: i32, autofit: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpanchor: i32, cpactive: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cline: i32, position: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, value: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextServices(pub ::windows::runtime::IUnknown);
impl ITextServices {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSendMessage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, msg: u32, wparam: Param1, lparam: Param2, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(plresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn TxDraw<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(
        &self,
        dwdrawaspect: super::super::super::System::Com::DVASPECT,
        lindex: i32,
        pvaspect: *mut ::core::ffi::c_void,
        ptd: *mut super::super::super::System::Com::DVTARGETDEVICE,
        hdcdraw: Param4,
        hictargetdev: Param5,
        lprcbounds: *mut super::super::super::Foundation::RECTL,
        lprcwbounds: *mut super::super::super::Foundation::RECTL,
        lprcupdate: *mut super::super::super::Foundation::RECT,
        pfncontinue: isize,
        dwcontinue: u32,
        lviewid: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dwdrawaspect),
            ::core::mem::transmute(lindex),
            ::core::mem::transmute(pvaspect),
            ::core::mem::transmute(ptd),
            hdcdraw.into_param().abi(),
            hictargetdev.into_param().abi(),
            ::core::mem::transmute(lprcbounds),
            ::core::mem::transmute(lprcwbounds),
            ::core::mem::transmute(lprcupdate),
            ::core::mem::transmute(pfncontinue),
            ::core::mem::transmute(dwcontinue),
            ::core::mem::transmute(lviewid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxGetHScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(plmin), ::core::mem::transmute(plmax), ::core::mem::transmute(plpos), ::core::mem::transmute(plpage), ::core::mem::transmute(pfenabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxGetVScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(plmin), ::core::mem::transmute(plmax), ::core::mem::transmute(plpos), ::core::mem::transmute(plpage), ::core::mem::transmute(pfenabled)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn OnTxSetCursor<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(
        &self,
        dwdrawaspect: super::super::super::System::Com::DVASPECT,
        lindex: i32,
        pvaspect: *mut ::core::ffi::c_void,
        ptd: *mut super::super::super::System::Com::DVTARGETDEVICE,
        hdcdraw: Param4,
        hictargetdev: Param5,
        lprcclient: *mut super::super::super::Foundation::RECT,
        x: i32,
        y: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dwdrawaspect),
            ::core::mem::transmute(lindex),
            ::core::mem::transmute(pvaspect),
            ::core::mem::transmute(ptd),
            hdcdraw.into_param().abi(),
            hictargetdev.into_param().abi(),
            ::core::mem::transmute(lprcclient),
            ::core::mem::transmute(x),
            ::core::mem::transmute(y),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn TxQueryHitPoint<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(
        &self,
        dwdrawaspect: super::super::super::System::Com::DVASPECT,
        lindex: i32,
        pvaspect: *mut ::core::ffi::c_void,
        ptd: *mut super::super::super::System::Com::DVTARGETDEVICE,
        hdcdraw: Param4,
        hictargetdev: Param5,
        lprcclient: *mut super::super::super::Foundation::RECT,
        x: i32,
        y: i32,
        phitresult: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dwdrawaspect),
            ::core::mem::transmute(lindex),
            ::core::mem::transmute(pvaspect),
            ::core::mem::transmute(ptd),
            hdcdraw.into_param().abi(),
            hictargetdev.into_param().abi(),
            ::core::mem::transmute(lprcclient),
            ::core::mem::transmute(x),
            ::core::mem::transmute(y),
            ::core::mem::transmute(phitresult),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn OnTxInPlaceActivate(&self, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcclient)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn OnTxInPlaceDeactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn OnTxUIActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn OnTxUIDeactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxGetText(&self, pbstrtext: *mut super::super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrtext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSetText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, psztext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), psztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetCurTargetX(&self, param0: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetBaseLinePos(&self, param0: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn TxGetNaturalSize<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, dwaspect: u32, hdcdraw: Param1, hictargetdev: Param2, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ::core::mem::transmute(ptd), ::core::mem::transmute(dwmode), ::core::mem::transmute(psizelextent), ::core::mem::transmute(pwidth), ::core::mem::transmute(pheight)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Ole`*"]
    pub unsafe fn TxGetDropTarget(&self) -> ::windows::runtime::Result<super::super::super::System::Ole::IDropTarget> {
        let mut result__: <super::super::super::System::Ole::IDropTarget as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Ole::IDropTarget>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn OnTxPropertyBitsChange(&self, dwmask: u32, dwbits: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmask), ::core::mem::transmute(dwbits)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetCachedSize(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwwidth), ::core::mem::transmute(pdwheight)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextServices {
    type Vtable = ITextServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::core::convert::From<ITextServices> for ::windows::runtime::IUnknown {
    fn from(value: ITextServices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextServices> for ::windows::runtime::IUnknown {
    fn from(value: &ITextServices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextServices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwdrawaspect: super::super::super::System::Com::DVASPECT,
        lindex: i32,
        pvaspect: *mut ::core::ffi::c_void,
        ptd: *mut super::super::super::System::Com::DVTARGETDEVICE,
        hdcdraw: super::super::super::Graphics::Gdi::HDC,
        hictargetdev: super::super::super::Graphics::Gdi::HDC,
        lprcbounds: *mut super::super::super::Foundation::RECTL,
        lprcwbounds: *mut super::super::super::Foundation::RECTL,
        lprcupdate: *mut super::super::super::Foundation::RECT,
        pfncontinue: isize,
        dwcontinue: u32,
        lviewid: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtext: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psztext: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdroptarget: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmask: u32, dwbits: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextServices2(pub ::windows::runtime::IUnknown);
impl ITextServices2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSendMessage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, msg: u32, wparam: Param1, lparam: Param2, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(plresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn TxDraw<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(
        &self,
        dwdrawaspect: super::super::super::System::Com::DVASPECT,
        lindex: i32,
        pvaspect: *mut ::core::ffi::c_void,
        ptd: *mut super::super::super::System::Com::DVTARGETDEVICE,
        hdcdraw: Param4,
        hictargetdev: Param5,
        lprcbounds: *mut super::super::super::Foundation::RECTL,
        lprcwbounds: *mut super::super::super::Foundation::RECTL,
        lprcupdate: *mut super::super::super::Foundation::RECT,
        pfncontinue: isize,
        dwcontinue: u32,
        lviewid: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dwdrawaspect),
            ::core::mem::transmute(lindex),
            ::core::mem::transmute(pvaspect),
            ::core::mem::transmute(ptd),
            hdcdraw.into_param().abi(),
            hictargetdev.into_param().abi(),
            ::core::mem::transmute(lprcbounds),
            ::core::mem::transmute(lprcwbounds),
            ::core::mem::transmute(lprcupdate),
            ::core::mem::transmute(pfncontinue),
            ::core::mem::transmute(dwcontinue),
            ::core::mem::transmute(lviewid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxGetHScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(plmin), ::core::mem::transmute(plmax), ::core::mem::transmute(plpos), ::core::mem::transmute(plpage), ::core::mem::transmute(pfenabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxGetVScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(plmin), ::core::mem::transmute(plmax), ::core::mem::transmute(plpos), ::core::mem::transmute(plpage), ::core::mem::transmute(pfenabled)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn OnTxSetCursor<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(
        &self,
        dwdrawaspect: super::super::super::System::Com::DVASPECT,
        lindex: i32,
        pvaspect: *mut ::core::ffi::c_void,
        ptd: *mut super::super::super::System::Com::DVTARGETDEVICE,
        hdcdraw: Param4,
        hictargetdev: Param5,
        lprcclient: *mut super::super::super::Foundation::RECT,
        x: i32,
        y: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dwdrawaspect),
            ::core::mem::transmute(lindex),
            ::core::mem::transmute(pvaspect),
            ::core::mem::transmute(ptd),
            hdcdraw.into_param().abi(),
            hictargetdev.into_param().abi(),
            ::core::mem::transmute(lprcclient),
            ::core::mem::transmute(x),
            ::core::mem::transmute(y),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn TxQueryHitPoint<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(
        &self,
        dwdrawaspect: super::super::super::System::Com::DVASPECT,
        lindex: i32,
        pvaspect: *mut ::core::ffi::c_void,
        ptd: *mut super::super::super::System::Com::DVTARGETDEVICE,
        hdcdraw: Param4,
        hictargetdev: Param5,
        lprcclient: *mut super::super::super::Foundation::RECT,
        x: i32,
        y: i32,
        phitresult: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dwdrawaspect),
            ::core::mem::transmute(lindex),
            ::core::mem::transmute(pvaspect),
            ::core::mem::transmute(ptd),
            hdcdraw.into_param().abi(),
            hictargetdev.into_param().abi(),
            ::core::mem::transmute(lprcclient),
            ::core::mem::transmute(x),
            ::core::mem::transmute(y),
            ::core::mem::transmute(phitresult),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn OnTxInPlaceActivate(&self, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcclient)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn OnTxInPlaceDeactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn OnTxUIActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn OnTxUIDeactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxGetText(&self, pbstrtext: *mut super::super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrtext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn TxSetText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, psztext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), psztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetCurTargetX(&self, param0: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetBaseLinePos(&self, param0: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn TxGetNaturalSize<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, dwaspect: u32, hdcdraw: Param1, hictargetdev: Param2, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ::core::mem::transmute(ptd), ::core::mem::transmute(dwmode), ::core::mem::transmute(psizelextent), ::core::mem::transmute(pwidth), ::core::mem::transmute(pheight)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Ole`*"]
    pub unsafe fn TxGetDropTarget(&self) -> ::windows::runtime::Result<super::super::super::System::Ole::IDropTarget> {
        let mut result__: <super::super::super::System::Ole::IDropTarget as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Ole::IDropTarget>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn OnTxPropertyBitsChange(&self, dwmask: u32, dwbits: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmask), ::core::mem::transmute(dwbits)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn TxGetCachedSize(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwwidth), ::core::mem::transmute(pdwheight)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
    pub unsafe fn TxGetNaturalSize2<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, dwaspect: u32, hdcdraw: Param1, hictargetdev: Param2, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dwaspect),
            hdcdraw.into_param().abi(),
            hictargetdev.into_param().abi(),
            ::core::mem::transmute(ptd),
            ::core::mem::transmute(dwmode),
            ::core::mem::transmute(psizelextent),
            ::core::mem::transmute(pwidth),
            ::core::mem::transmute(pheight),
            ::core::mem::transmute(pascent),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Direct2D`*"]
    pub unsafe fn TxDrawD2D<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct2D::ID2D1RenderTarget>>(&self, prendertarget: Param0, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), prendertarget.into_param().abi(), ::core::mem::transmute(lprcbounds), ::core::mem::transmute(lprcupdate), ::core::mem::transmute(lviewid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextServices2 {
    type Vtable = ITextServices2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::core::convert::From<ITextServices2> for ::windows::runtime::IUnknown {
    fn from(value: ITextServices2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextServices2> for ::windows::runtime::IUnknown {
    fn from(value: &ITextServices2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextServices2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextServices2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITextServices2> for ITextServices {
    fn from(value: ITextServices2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextServices2> for ITextServices {
    fn from(value: &ITextServices2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextServices> for ITextServices2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextServices> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextServices> for &ITextServices2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextServices> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextServices2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwdrawaspect: super::super::super::System::Com::DVASPECT,
        lindex: i32,
        pvaspect: *mut ::core::ffi::c_void,
        ptd: *mut super::super::super::System::Com::DVTARGETDEVICE,
        hdcdraw: super::super::super::Graphics::Gdi::HDC,
        hictargetdev: super::super::super::Graphics::Gdi::HDC,
        lprcbounds: *mut super::super::super::Foundation::RECTL,
        lprcwbounds: *mut super::super::super::Foundation::RECTL,
        lprcupdate: *mut super::super::super::Foundation::RECT,
        pfncontinue: isize,
        dwcontinue: u32,
        lviewid: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtext: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psztext: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdroptarget: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmask: u32, dwbits: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prendertarget: ::windows::runtime::RawPtr, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D")))] usize,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStory(pub ::windows::runtime::IUnknown);
impl ITextStory {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetActive(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetActive(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetDisplay(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetIndex(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetType(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetRange(&self, cpactive: i32, cpanchor: i32) -> ::windows::runtime::Result<ITextRange2> {
        let mut result__: <ITextRange2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpactive), ::core::mem::transmute(cpanchor), &mut result__).from_abi::<ITextRange2>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetText(&self, flags: i32) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFormattedText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SetText<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, flags: i32, bstr: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), bstr.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextStory {
    type Vtable = ITextStory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc241f5f3_7206_11d8_a2c7_00a0d1d6c6b3);
}
impl ::core::convert::From<ITextStory> for ::windows::runtime::IUnknown {
    fn from(value: ITextStory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStory> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisplay: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32, pbstr: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, value: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStoryRanges(pub ::windows::runtime::IUnknown);
impl ITextStoryRanges {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITextStoryRanges {
    type Vtable = ITextStoryRanges_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8cc497c5_a1df_11ce_8098_00aa0047be5d);
}
impl ::core::convert::From<ITextStoryRanges> for ::windows::runtime::IUnknown {
    fn from(value: ITextStoryRanges) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStoryRanges> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStoryRanges) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStoryRanges {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStoryRanges {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextStoryRanges> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextStoryRanges) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextStoryRanges> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextStoryRanges) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextStoryRanges {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextStoryRanges {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoryRanges_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunkenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStoryRanges2(pub ::windows::runtime::IUnknown);
impl ITextStoryRanges2 {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: <super::super::super::System::Com::ITypeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<ITextRange> {
        let mut result__: <ITextRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Item2(&self, index: i32) -> ::windows::runtime::Result<ITextRange2> {
        let mut result__: <ITextRange2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<ITextRange2>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITextStoryRanges2 {
    type Vtable = ITextStoryRanges2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc241f5e5_7206_11d8_a2c7_00a0d1d6c6b3);
}
impl ::core::convert::From<ITextStoryRanges2> for ::windows::runtime::IUnknown {
    fn from(value: ITextStoryRanges2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStoryRanges2> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStoryRanges2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStoryRanges2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStoryRanges2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITextStoryRanges2> for ITextStoryRanges {
    fn from(value: ITextStoryRanges2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoryRanges2> for ITextStoryRanges {
    fn from(value: &ITextStoryRanges2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextStoryRanges> for ITextStoryRanges2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextStoryRanges> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextStoryRanges> for &ITextStoryRanges2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextStoryRanges> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextStoryRanges2> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextStoryRanges2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextStoryRanges2> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextStoryRanges2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextStoryRanges2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextStoryRanges2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoryRanges2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunkenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStrings(pub ::windows::runtime::IUnknown);
impl ITextStrings {
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<ITextRange2> {
        let mut result__: <ITextRange2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange2>>(&self, prange: Param0, istring: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(istring)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Cat2(&self, istring: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(istring)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn CatTop2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn DeleteRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange2>>(&self, prange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn EncodeFunction<'a, Param8: ::windows::runtime::IntoParam<'a, ITextRange2>>(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: Param8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(r#type),
            ::core::mem::transmute(align),
            ::core::mem::transmute(char),
            ::core::mem::transmute(char1),
            ::core::mem::transmute(char2),
            ::core::mem::transmute(count),
            ::core::mem::transmute(texstyle),
            ::core::mem::transmute(ccol),
            prange.into_param().abi(),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn GetCch(&self, istring: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(istring), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn InsertNullStr(&self, istring: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(istring)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn MoveBoundary(&self, istring: i32, cch: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(istring), ::core::mem::transmute(cch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn PrefixTop<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Remove(&self, istring: i32, cstring: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(istring), ::core::mem::transmute(cstring)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetFormattedText<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange2>, Param1: ::windows::runtime::IntoParam<'a, ITextRange2>>(&self, pranged: Param0, pranges: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), pranged.into_param().abi(), pranges.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn SetOpCp(&self, istring: i32, cp: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(istring), ::core::mem::transmute(cp)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
    pub unsafe fn SuffixTop<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, ITextRange2>>(&self, bstr: Param0, prange: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), bstr.into_param().abi(), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
    pub unsafe fn Swap(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextStrings {
    type Vtable = ITextStrings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc241f5e7_7206_11d8_a2c7_00a0d1d6c6b3);
}
impl ::core::convert::From<ITextStrings> for ::windows::runtime::IUnknown {
    fn from(value: ITextStrings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStrings> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStrings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStrings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStrings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITextStrings> for super::super::super::System::Com::IDispatch {
    fn from(value: ITextStrings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITextStrings> for super::super::super::System::Com::IDispatch {
    fn from(value: &ITextStrings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextStrings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextStrings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStrings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, istring: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, istring: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, istring: i32, pcch: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, istring: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, istring: i32, cch: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, istring: i32, cstring: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pranged: ::windows::runtime::RawPtr, pranges: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, istring: i32, cp: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KHYPH(pub i32);
pub const khyphNil: KHYPH = KHYPH(0i32);
pub const khyphNormal: KHYPH = KHYPH(1i32);
pub const khyphAddBefore: KHYPH = KHYPH(2i32);
pub const khyphChangeBefore: KHYPH = KHYPH(3i32);
pub const khyphDeleteBefore: KHYPH = KHYPH(4i32);
pub const khyphChangeAfter: KHYPH = KHYPH(5i32);
pub const khyphDelAndChange: KHYPH = KHYPH(6i32);
impl ::core::convert::From<i32> for KHYPH {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KHYPH {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MANCODE(pub i32);
pub const MBOLD: MANCODE = MANCODE(16i32);
pub const MITAL: MANCODE = MANCODE(32i32);
pub const MGREEK: MANCODE = MANCODE(64i32);
pub const MROMN: MANCODE = MANCODE(0i32);
pub const MSCRP: MANCODE = MANCODE(1i32);
pub const MFRAK: MANCODE = MANCODE(2i32);
pub const MOPEN: MANCODE = MANCODE(3i32);
pub const MSANS: MANCODE = MANCODE(4i32);
pub const MMONO: MANCODE = MANCODE(5i32);
pub const MMATH: MANCODE = MANCODE(6i32);
pub const MISOL: MANCODE = MANCODE(7i32);
pub const MINIT: MANCODE = MANCODE(8i32);
pub const MTAIL: MANCODE = MANCODE(9i32);
pub const MSTRCH: MANCODE = MANCODE(10i32);
pub const MLOOP: MANCODE = MANCODE(11i32);
pub const MOPENA: MANCODE = MANCODE(12i32);
impl ::core::convert::From<i32> for MANCODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MANCODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const MAX_TABLE_CELLS: u32 = 63u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const MAX_TAB_STOPS: u32 = 32u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct MSGFILTER {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl MSGFILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSGFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSGFILTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSGFILTER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MSGFILTER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct OBJECTPOSITIONS {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub pcpPositions: *mut i32,
}
#[cfg(feature = "Win32_Foundation")]
impl OBJECTPOSITIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OBJECTPOSITIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OBJECTPOSITIONS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OBJECTPOSITIONS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OBJECTPOSITIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OBJECTTYPE(pub i32);
pub const tomSimpleText: OBJECTTYPE = OBJECTTYPE(0i32);
pub const tomRuby: OBJECTTYPE = OBJECTTYPE(1i32);
pub const tomHorzVert: OBJECTTYPE = OBJECTTYPE(2i32);
pub const tomWarichu: OBJECTTYPE = OBJECTTYPE(3i32);
pub const tomEq: OBJECTTYPE = OBJECTTYPE(9i32);
pub const tomMath: OBJECTTYPE = OBJECTTYPE(10i32);
pub const tomAccent: OBJECTTYPE = OBJECTTYPE(10i32);
pub const tomBox: OBJECTTYPE = OBJECTTYPE(11i32);
pub const tomBoxedFormula: OBJECTTYPE = OBJECTTYPE(12i32);
pub const tomBrackets: OBJECTTYPE = OBJECTTYPE(13i32);
pub const tomBracketsWithSeps: OBJECTTYPE = OBJECTTYPE(14i32);
pub const tomEquationArray: OBJECTTYPE = OBJECTTYPE(15i32);
pub const tomFraction: OBJECTTYPE = OBJECTTYPE(16i32);
pub const tomFunctionApply: OBJECTTYPE = OBJECTTYPE(17i32);
pub const tomLeftSubSup: OBJECTTYPE = OBJECTTYPE(18i32);
pub const tomLowerLimit: OBJECTTYPE = OBJECTTYPE(19i32);
pub const tomMatrix: OBJECTTYPE = OBJECTTYPE(20i32);
pub const tomNary: OBJECTTYPE = OBJECTTYPE(21i32);
pub const tomOpChar: OBJECTTYPE = OBJECTTYPE(22i32);
pub const tomOverbar: OBJECTTYPE = OBJECTTYPE(23i32);
pub const tomPhantom: OBJECTTYPE = OBJECTTYPE(24i32);
pub const tomRadical: OBJECTTYPE = OBJECTTYPE(25i32);
pub const tomSlashedFraction: OBJECTTYPE = OBJECTTYPE(26i32);
pub const tomStack: OBJECTTYPE = OBJECTTYPE(27i32);
pub const tomStretchStack: OBJECTTYPE = OBJECTTYPE(28i32);
pub const tomSubscript: OBJECTTYPE = OBJECTTYPE(29i32);
pub const tomSubSup: OBJECTTYPE = OBJECTTYPE(30i32);
pub const tomSuperscript: OBJECTTYPE = OBJECTTYPE(31i32);
pub const tomUnderbar: OBJECTTYPE = OBJECTTYPE(32i32);
pub const tomUpperLimit: OBJECTTYPE = OBJECTTYPE(33i32);
pub const tomObjectMax: OBJECTTYPE = OBJECTTYPE(33i32);
impl ::core::convert::From<i32> for OBJECTTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OBJECTTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const OLEOP_DOVERB: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct PARAFORMAT {
    pub cbSize: u32,
    pub dwMask: PARAFORMAT_MASK,
    pub wNumbering: u16,
    pub Anonymous: PARAFORMAT_0,
    pub dxStartIndent: i32,
    pub dxRightIndent: i32,
    pub dxOffset: i32,
    pub wAlignment: PARAFORMAT_ALIGNMENT,
    pub cTabCount: i16,
    pub rgxTabs: [u32; 32],
}
impl PARAFORMAT {}
impl ::core::default::Default for PARAFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PARAFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PARAFORMAT {}
unsafe impl ::windows::runtime::Abi for PARAFORMAT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub union PARAFORMAT_0 {
    pub wReserved: u16,
    pub wEffects: u16,
}
impl PARAFORMAT_0 {}
impl ::core::default::Default for PARAFORMAT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PARAFORMAT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PARAFORMAT_0 {}
unsafe impl ::windows::runtime::Abi for PARAFORMAT_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct PARAFORMAT2 {
    pub __AnonymousBase_richedit_L1149_C22: PARAFORMAT,
    pub dySpaceBefore: i32,
    pub dySpaceAfter: i32,
    pub dyLineSpacing: i32,
    pub sStyle: i16,
    pub bLineSpacingRule: u8,
    pub bOutlineLevel: u8,
    pub wShadingWeight: u16,
    pub wShadingStyle: PARAFORMAT_SHADING_STYLE,
    pub wNumberingStart: u16,
    pub wNumberingStyle: PARAFORMAT_NUMBERING_STYLE,
    pub wNumberingTab: u16,
    pub wBorderSpace: u16,
    pub wBorderWidth: u16,
    pub wBorders: PARAFORMAT_BORDERS,
}
impl PARAFORMAT2 {}
impl ::core::default::Default for PARAFORMAT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PARAFORMAT2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PARAFORMAT2 {}
unsafe impl ::windows::runtime::Abi for PARAFORMAT2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PARAFORMAT_ALIGNMENT(pub u16);
pub const PFA_CENTER: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(3u16);
pub const PFA_LEFT: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(1u16);
pub const PFA_RIGHT: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(2u16);
impl ::core::convert::From<u16> for PARAFORMAT_ALIGNMENT {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PARAFORMAT_ALIGNMENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PARAFORMAT_BORDERS(pub u16);
pub const PARAFORMAT_BORDERS_LEFT: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(1u16);
pub const PARAFORMAT_BORDERS_RIGHT: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(2u16);
pub const PARAFORMAT_BORDERS_TOP: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(4u16);
pub const PARAFORMAT_BORDERS_BOTTOM: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(8u16);
pub const PARAFORMAT_BORDERS_INSIDE: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(16u16);
pub const PARAFORMAT_BORDERS_OUTSIDE: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(32u16);
pub const PARAFORMAT_BORDERS_AUTOCOLOR: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(64u16);
impl ::core::convert::From<u16> for PARAFORMAT_BORDERS {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PARAFORMAT_BORDERS {
    type Abi = Self;
}
impl ::core::ops::BitOr for PARAFORMAT_BORDERS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PARAFORMAT_BORDERS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PARAFORMAT_BORDERS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PARAFORMAT_BORDERS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PARAFORMAT_BORDERS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PARAFORMAT_MASK(pub u32);
pub const PFM_ALIGNMENT: PARAFORMAT_MASK = PARAFORMAT_MASK(8u32);
pub const PFM_NUMBERING: PARAFORMAT_MASK = PARAFORMAT_MASK(32u32);
pub const PFM_OFFSET: PARAFORMAT_MASK = PARAFORMAT_MASK(4u32);
pub const PFM_OFFSETINDENT: PARAFORMAT_MASK = PARAFORMAT_MASK(2147483648u32);
pub const PFM_RIGHTINDENT: PARAFORMAT_MASK = PARAFORMAT_MASK(2u32);
pub const PFM_RTLPARA: PARAFORMAT_MASK = PARAFORMAT_MASK(65536u32);
pub const PFM_STARTINDENT: PARAFORMAT_MASK = PARAFORMAT_MASK(1u32);
pub const PFM_TABSTOPS: PARAFORMAT_MASK = PARAFORMAT_MASK(16u32);
impl ::core::convert::From<u32> for PARAFORMAT_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PARAFORMAT_MASK {
    type Abi = Self;
}
impl ::core::ops::BitOr for PARAFORMAT_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PARAFORMAT_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PARAFORMAT_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PARAFORMAT_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PARAFORMAT_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PARAFORMAT_NUMBERING_STYLE(pub u16);
pub const PFNS_PAREN: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(0u16);
pub const PFNS_PARENS: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(256u16);
pub const PFNS_PERIOD: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(512u16);
pub const PFNS_PLAIN: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(768u16);
pub const PFNS_NONUMBER: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(1024u16);
pub const PFNS_NEWNUMBER: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(32768u16);
impl ::core::convert::From<u16> for PARAFORMAT_NUMBERING_STYLE {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PARAFORMAT_NUMBERING_STYLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PARAFORMAT_SHADING_STYLE(pub u16);
pub const PARAFORMAT_SHADING_STYLE_NONE: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(0u16);
pub const PARAFORMAT_SHADING_STYLE_DARK_HORIZ: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(1u16);
pub const PARAFORMAT_SHADING_STYLE_DARK_VERT: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(2u16);
pub const PARAFORMAT_SHADING_STYLE_DARK_DOWN_DIAG: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(3u16);
pub const PARAFORMAT_SHADING_STYLE_DARK_UP_DIAG: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(4u16);
pub const PARAFORMAT_SHADING_STYLE_DARK_GRID: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(5u16);
pub const PARAFORMAT_SHADING_STYLE_DARK_TRELLIS: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(6u16);
pub const PARAFORMAT_SHADING_STYLE_LIGHT_HORZ: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(7u16);
pub const PARAFORMAT_SHADING_STYLE_LIGHT_VERT: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(8u16);
pub const PARAFORMAT_SHADING_STYLE_LIGHT_DOWN_DIAG: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(9u16);
pub const PARAFORMAT_SHADING_STYLE_LIGHT_UP_DIAG: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(10u16);
pub const PARAFORMAT_SHADING_STYLE_LIGHT_GRID: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(11u16);
pub const PARAFORMAT_SHADING_STYLE_LIGHT_TRELLIS: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(12u16);
impl ::core::convert::From<u16> for PARAFORMAT_SHADING_STYLE {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PARAFORMAT_SHADING_STYLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PC_DELIMITER: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PC_FOLLOWING: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PC_LEADING: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PC_OVERFLOW: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub type PCreateTextServices = unsafe extern "system" fn(punkouter: ::windows::runtime::RawPtr, pitexthost: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFA_FULL_GLYPHS: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFA_FULL_INTERLETTER: u32 = 6u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFA_FULL_INTERWORD: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFA_FULL_NEWSPAPER: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFA_FULL_SCALED: u32 = 7u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFA_JUSTIFY: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_BORDER: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_BOX: u32 = 67108864u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_COLLAPSED: u32 = 16777216u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_DONOTHYPHEN: u32 = 4194304u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_KEEP: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_KEEPNEXT: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_LINESPACING: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_NOLINENUMBER: u32 = 1048576u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_NOWIDOWCONTROL: u32 = 2097152u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_NUMBERINGSTART: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_NUMBERINGSTYLE: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_NUMBERINGTAB: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_OUTLINELEVEL: u32 = 33554432u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_PAGEBREAKBEFORE: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_RESERVED2: u32 = 134217728u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_SHADING: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_SIDEBYSIDE: u32 = 8388608u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_SPACEAFTER: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_SPACEBEFORE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_STYLE: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_TABLE: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_TABLEROWDELIMITER: u32 = 268435456u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFM_TEXTWRAPPINGBREAK: u32 = 536870912u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFN_ARABIC: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFN_BULLET: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFN_LCLETTER: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFN_LCROMAN: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFN_UCLETTER: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const PFN_UCROMAN: u32 = 6u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub type PShutdownTextServices = unsafe extern "system" fn(ptextservices: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct PUNCTUATION {
    pub iSize: u32,
    pub szPunctuation: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PUNCTUATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PUNCTUATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PUNCTUATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PUNCTUATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PUNCTUATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const RECO_COPY: i32 = 2i32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const RECO_CUT: i32 = 3i32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const RECO_DRAG: i32 = 4i32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const RECO_DROP: i32 = 1i32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const RECO_PASTE: i32 = 0i32;
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`*"]
pub struct REOBJECT {
    pub cbStruct: u32,
    pub cp: i32,
    pub clsid: ::windows::runtime::GUID,
    pub poleobj: ::core::option::Option<super::super::super::System::Ole::IOleObject>,
    pub pstg: ::core::option::Option<super::super::super::System::Com::StructuredStorage::IStorage>,
    pub polesite: ::core::option::Option<super::super::super::System::Ole::IOleClientSite>,
    pub sizel: super::super::super::Foundation::SIZE,
    pub dvaspect: u32,
    pub dwFlags: REOBJECT_FLAGS,
    pub dwUser: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl REOBJECT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::default::Default for REOBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for REOBJECT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("REOBJECT")
            .field("cbStruct", &self.cbStruct)
            .field("cp", &self.cp)
            .field("clsid", &self.clsid)
            .field("poleobj", &self.poleobj)
            .field("pstg", &self.pstg)
            .field("polesite", &self.polesite)
            .field("sizel", &self.sizel)
            .field("dvaspect", &self.dvaspect)
            .field("dwFlags", &self.dwFlags)
            .field("dwUser", &self.dwUser)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for REOBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.cp == other.cp && self.clsid == other.clsid && self.poleobj == other.poleobj && self.pstg == other.pstg && self.polesite == other.polesite && self.sizel == other.sizel && self.dvaspect == other.dvaspect && self.dwFlags == other.dwFlags && self.dwUser == other.dwUser
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for REOBJECT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
unsafe impl ::windows::runtime::Abi for REOBJECT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct REOBJECT_FLAGS(pub u32);
pub const REO_ALIGNTORIGHT: REOBJECT_FLAGS = REOBJECT_FLAGS(256u32);
pub const REO_BELOWBASELINE: REOBJECT_FLAGS = REOBJECT_FLAGS(2u32);
pub const REO_BLANK: REOBJECT_FLAGS = REOBJECT_FLAGS(16u32);
pub const REO_CANROTATE: REOBJECT_FLAGS = REOBJECT_FLAGS(128u32);
pub const REO_DONTNEEDPALETTE: REOBJECT_FLAGS = REOBJECT_FLAGS(32u32);
pub const REO_DYNAMICSIZE: REOBJECT_FLAGS = REOBJECT_FLAGS(8u32);
pub const REO_GETMETAFILE: REOBJECT_FLAGS = REOBJECT_FLAGS(4194304u32);
pub const REO_HILITED: REOBJECT_FLAGS = REOBJECT_FLAGS(16777216u32);
pub const REO_INPLACEACTIVE: REOBJECT_FLAGS = REOBJECT_FLAGS(33554432u32);
pub const REO_INVERTEDSELECT: REOBJECT_FLAGS = REOBJECT_FLAGS(4u32);
pub const REO_LINK: REOBJECT_FLAGS = REOBJECT_FLAGS(2147483648u32);
pub const REO_LINKAVAILABLE: REOBJECT_FLAGS = REOBJECT_FLAGS(8388608u32);
pub const REO_OPEN: REOBJECT_FLAGS = REOBJECT_FLAGS(67108864u32);
pub const REO_OWNERDRAWSELECT: REOBJECT_FLAGS = REOBJECT_FLAGS(64u32);
pub const REO_RESIZABLE: REOBJECT_FLAGS = REOBJECT_FLAGS(1u32);
pub const REO_SELECTED: REOBJECT_FLAGS = REOBJECT_FLAGS(134217728u32);
pub const REO_STATIC: REOBJECT_FLAGS = REOBJECT_FLAGS(1073741824u32);
pub const REO_USEASBACKGROUND: REOBJECT_FLAGS = REOBJECT_FLAGS(1024u32);
pub const REO_WRAPTEXTAROUND: REOBJECT_FLAGS = REOBJECT_FLAGS(512u32);
impl ::core::convert::From<u32> for REOBJECT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REOBJECT_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for REOBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for REOBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for REOBJECT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for REOBJECT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for REOBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const REO_NULL: i32 = 0i32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const REO_READWRITEMASK: i32 = 2047i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_System_Com`*"]
pub struct REPASTESPECIAL {
    pub dwAspect: super::super::super::System::Com::DVASPECT,
    pub dwParam: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl REPASTESPECIAL {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for REPASTESPECIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for REPASTESPECIAL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for REPASTESPECIAL {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for REPASTESPECIAL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct REQRESIZE {
    pub nmhdr: super::NMHDR,
    pub rc: super::super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl REQRESIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REQRESIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REQRESIZE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REQRESIZE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for REQRESIZE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for RICHEDIT_IMAGE_PARAMETERS {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`*"]
pub struct RICHEDIT_IMAGE_PARAMETERS {
    pub xWidth: i32,
    pub yHeight: i32,
    pub Ascent: i32,
    pub Type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS,
    pub pwszAlternateText: super::super::super::Foundation::PWSTR,
    pub pIStream: ::core::option::Option<super::super::super::System::Com::IStream>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl RICHEDIT_IMAGE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::default::Default for RICHEDIT_IMAGE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for RICHEDIT_IMAGE_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for RICHEDIT_IMAGE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for RICHEDIT_IMAGE_PARAMETERS {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(pub u16);
pub const SEL_EMPTY: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(0u16);
pub const SEL_TEXT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(1u16);
pub const SEL_OBJECT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(2u16);
pub const SEL_MULTICHAR: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(4u16);
pub const SEL_MULTIOBJECT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(8u16);
pub const GCM_RIGHTMOUSEDROP: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(32768u16);
impl ::core::convert::From<u16> for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type Abi = Self;
}
impl ::core::ops::BitOr for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RICH_EDIT_GET_OBJECT_FLAGS(pub u32);
pub const REO_GETOBJ_POLEOBJ: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(1u32);
pub const REO_GETOBJ_PSTG: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(2u32);
pub const REO_GETOBJ_POLESITE: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(4u32);
pub const REO_GETOBJ_NO_INTERFACES: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(0u32);
pub const REO_GETOBJ_ALL_INTERFACES: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(7u32);
impl ::core::convert::From<u32> for RICH_EDIT_GET_OBJECT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RICH_EDIT_GET_OBJECT_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for RICH_EDIT_GET_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for RICH_EDIT_GET_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for RICH_EDIT_GET_OBJECT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for RICH_EDIT_GET_OBJECT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for RICH_EDIT_GET_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const RTO_DISABLEHANDLES: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const RTO_READINGMODE: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const RTO_SHOWHANDLES: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SCF_ALL: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SCF_ASSOCIATEFONT: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SCF_ASSOCIATEFONT2: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SCF_CHARREPFROMLCID: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SCF_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SCF_NOKBUPDATE: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SCF_SELECTION: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SCF_SMARTFONT: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SCF_USEUIRULES: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SCF_WORD: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct SELCHANGE {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl SELCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SELCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SELCHANGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SELCHANGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SELCHANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_ALLOWBEEPS: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_BEEPONMAXTEXT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_BIDI: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_CTFALLOWEMBED: u32 = 2097152u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_CTFALLOWPROOFING: u32 = 8388608u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_CTFALLOWSMARTTAG: u32 = 4194304u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_CTFNOLOCK: u32 = 268435456u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_CUSTOMLOOK: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_DEFAULTLATINLIGA: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_DRAFTMODE: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_EMULATE10: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_EMULATESYSEDIT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_EXTENDBACKCOLOR: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_EX_HANDLEFRIENDLYURL: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_EX_HIDETEMPFORMAT: u32 = 268435456u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_EX_MULTITOUCH: u32 = 134217728u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_EX_NOACETATESELECTION: u32 = 1048576u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_EX_NOMATH: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_EX_NOTABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_EX_NOTHEMING: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_EX_USEMOUSEWPARAM: u32 = 536870912u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_EX_USESINGLELINE: u32 = 2097152u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_HIDEGRIDLINES: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_HYPERLINKTOOLTIPS: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_LBSCROLLNOTIFY: u32 = 1048576u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_LOGICALCARET: u32 = 16777216u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_LOWERCASE: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_MAPCPS: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_MAX: u32 = 536870912u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_MULTISELECT: u32 = 134217728u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_NOEALINEHEIGHTADJUST: u32 = 536870912u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_NOFOCUSLINKNOTIFY: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_NOIME: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_NOINPUTSEQUENCECHK: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_SCROLLONKILLFOCUS: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_SMARTDRAGDROP: u32 = 67108864u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_UPPERCASE: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_USEAIMM: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_USEATFONT: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_USECRLF: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_USECTF: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_WORDDRAGDROP: u32 = 33554432u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SES_XLTCRCRLFTOCR: u32 = 16384u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct SETTEXTEX {
    pub flags: u32,
    pub codepage: u32,
}
impl SETTEXTEX {}
impl ::core::default::Default for SETTEXTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SETTEXTEX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SETTEXTEX").field("flags", &self.flags).field("codepage", &self.codepage).finish()
    }
}
impl ::core::cmp::PartialEq for SETTEXTEX {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.codepage == other.codepage
    }
}
impl ::core::cmp::Eq for SETTEXTEX {}
unsafe impl ::windows::runtime::Abi for SETTEXTEX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SFF_KEEPDOCINFO: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SFF_PERSISTVIEWSCALE: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SFF_PLAINRTF: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SFF_PWD: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SFF_SELECTION: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SFF_WRITEXTRAPAR: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SF_NCRFORNONASCII: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SF_RTF: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SF_RTFNOOBJS: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SF_RTFVAL: u32 = 1792u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SF_TEXT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SF_TEXTIZED: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SF_UNICODE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SF_USECODEPAGE: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SPF_DONTSETDEFAULT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const SPF_SETDEFAULT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ST_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ST_KEEPUNDO: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ST_NEWCHARS: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ST_SELECTION: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const ST_UNICODE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const S_MSG_KEY_IGNORED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(262657i32 as _);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct TABLECELLPARMS {
    pub dxWidth: i32,
    pub _bitfield: u16,
    pub wShading: u16,
    pub dxBrdrLeft: i16,
    pub dyBrdrTop: i16,
    pub dxBrdrRight: i16,
    pub dyBrdrBottom: i16,
    pub crBrdrLeft: u32,
    pub crBrdrTop: u32,
    pub crBrdrRight: u32,
    pub crBrdrBottom: u32,
    pub crBackPat: u32,
    pub crForePat: u32,
}
impl TABLECELLPARMS {}
impl ::core::default::Default for TABLECELLPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TABLECELLPARMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TABLECELLPARMS")
            .field("dxWidth", &self.dxWidth)
            .field("_bitfield", &self._bitfield)
            .field("wShading", &self.wShading)
            .field("dxBrdrLeft", &self.dxBrdrLeft)
            .field("dyBrdrTop", &self.dyBrdrTop)
            .field("dxBrdrRight", &self.dxBrdrRight)
            .field("dyBrdrBottom", &self.dyBrdrBottom)
            .field("crBrdrLeft", &self.crBrdrLeft)
            .field("crBrdrTop", &self.crBrdrTop)
            .field("crBrdrRight", &self.crBrdrRight)
            .field("crBrdrBottom", &self.crBrdrBottom)
            .field("crBackPat", &self.crBackPat)
            .field("crForePat", &self.crForePat)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TABLECELLPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dxWidth == other.dxWidth
            && self._bitfield == other._bitfield
            && self.wShading == other.wShading
            && self.dxBrdrLeft == other.dxBrdrLeft
            && self.dyBrdrTop == other.dyBrdrTop
            && self.dxBrdrRight == other.dxBrdrRight
            && self.dyBrdrBottom == other.dyBrdrBottom
            && self.crBrdrLeft == other.crBrdrLeft
            && self.crBrdrTop == other.crBrdrTop
            && self.crBrdrRight == other.crBrdrRight
            && self.crBrdrBottom == other.crBrdrBottom
            && self.crBackPat == other.crBackPat
            && self.crForePat == other.crForePat
    }
}
impl ::core::cmp::Eq for TABLECELLPARMS {}
unsafe impl ::windows::runtime::Abi for TABLECELLPARMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct TABLEROWPARMS {
    pub cbRow: u8,
    pub cbCell: u8,
    pub cCell: u8,
    pub cRow: u8,
    pub dxCellMargin: i32,
    pub dxIndent: i32,
    pub dyHeight: i32,
    pub _bitfield: u32,
    pub cpStartRow: i32,
    pub bTableLevel: u8,
    pub iCell: u8,
}
impl TABLEROWPARMS {}
impl ::core::default::Default for TABLEROWPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TABLEROWPARMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TABLEROWPARMS")
            .field("cbRow", &self.cbRow)
            .field("cbCell", &self.cbCell)
            .field("cCell", &self.cCell)
            .field("cRow", &self.cRow)
            .field("dxCellMargin", &self.dxCellMargin)
            .field("dxIndent", &self.dxIndent)
            .field("dyHeight", &self.dyHeight)
            .field("_bitfield", &self._bitfield)
            .field("cpStartRow", &self.cpStartRow)
            .field("bTableLevel", &self.bTableLevel)
            .field("iCell", &self.iCell)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TABLEROWPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbRow == other.cbRow && self.cbCell == other.cbCell && self.cCell == other.cCell && self.cRow == other.cRow && self.dxCellMargin == other.dxCellMargin && self.dxIndent == other.dxIndent && self.dyHeight == other.dyHeight && self._bitfield == other._bitfield && self.cpStartRow == other.cpStartRow && self.bTableLevel == other.bTableLevel && self.iCell == other.iCell
    }
}
impl ::core::cmp::Eq for TABLEROWPARMS {}
unsafe impl ::windows::runtime::Abi for TABLEROWPARMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TEXTMODE(pub i32);
pub const TM_PLAINTEXT: TEXTMODE = TEXTMODE(1i32);
pub const TM_RICHTEXT: TEXTMODE = TEXTMODE(2i32);
pub const TM_SINGLELEVELUNDO: TEXTMODE = TEXTMODE(4i32);
pub const TM_MULTILEVELUNDO: TEXTMODE = TEXTMODE(8i32);
pub const TM_SINGLECODEPAGE: TEXTMODE = TEXTMODE(16i32);
pub const TM_MULTICODEPAGE: TEXTMODE = TEXTMODE(32i32);
impl ::core::convert::From<i32> for TEXTMODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TEXTMODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct TEXTRANGEA {
    pub chrg: CHARRANGE,
    pub lpstrText: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl TEXTRANGEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TEXTRANGEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TEXTRANGEA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TEXTRANGEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TEXTRANGEA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct TEXTRANGEW {
    pub chrg: CHARRANGE,
    pub lpstrText: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl TEXTRANGEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TEXTRANGEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TEXTRANGEW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TEXTRANGEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TEXTRANGEW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TO_ADVANCEDLAYOUT: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TO_ADVANCEDTYPOGRAPHY: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TO_DISABLECUSTOMTEXTOUT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TO_SIMPLELINEBREAK: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXES_ISDIALOG: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TXTBACKSTYLE(pub i32);
pub const TXTBACK_TRANSPARENT: TXTBACKSTYLE = TXTBACKSTYLE(0i32);
pub const TXTBACK_OPAQUE: TXTBACKSTYLE = TXTBACKSTYLE(1i32);
impl ::core::convert::From<i32> for TXTBACKSTYLE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TXTBACKSTYLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_ADVANCEDINPUT: u32 = 536870912u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_ALLOWBEEP: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_AUTOWORDSEL: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_BACKSTYLECHANGE: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_CHARFORMATCHANGE: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_CLIENTRECTCHANGE: u32 = 1048576u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_D2DDWRITE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_D2DPIXELSNAPPED: u32 = 67108864u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_D2DSIMPLETYPOGRAPHY: u32 = 33554432u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_D2DSUBPIXELLINES: u32 = 134217728u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_DISABLEDRAG: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_EXTENTCHANGE: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_FLASHLASTPASSWORDCHAR: u32 = 268435456u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_HIDESELECTION: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_MAXLENGTHCHANGE: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_MULTILINE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_NOTHREADREFCOUNT: u32 = 4194304u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_PARAFORMATCHANGE: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_READONLY: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_RICHTEXT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_SAVESELECTION: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_SCROLLBARCHANGE: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_SELBARCHANGE: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_SHOWACCELERATOR: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_SHOWPASSWORD: u32 = 8388608u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_USECURRENTBKG: u32 = 2097152u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_USEPASSWORD: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_VERTICAL: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_VIEWINSETCHANGE: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const TXTBIT_WORDWRAP: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TXTHITRESULT(pub i32);
pub const TXTHITRESULT_NOHIT: TXTHITRESULT = TXTHITRESULT(0i32);
pub const TXTHITRESULT_TRANSPARENT: TXTHITRESULT = TXTHITRESULT(1i32);
pub const TXTHITRESULT_CLOSE: TXTHITRESULT = TXTHITRESULT(2i32);
pub const TXTHITRESULT_HIT: TXTHITRESULT = TXTHITRESULT(3i32);
impl ::core::convert::From<i32> for TXTHITRESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TXTHITRESULT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TXTNATURALSIZE(pub i32);
pub const TXTNS_FITTOCONTENT2: TXTNATURALSIZE = TXTNATURALSIZE(0i32);
pub const TXTNS_FITTOCONTENT: TXTNATURALSIZE = TXTNATURALSIZE(1i32);
pub const TXTNS_ROUNDTOLINE: TXTNATURALSIZE = TXTNATURALSIZE(2i32);
pub const TXTNS_FITTOCONTENT3: TXTNATURALSIZE = TXTNATURALSIZE(3i32);
pub const TXTNS_FITTOCONTENTWSP: TXTNATURALSIZE = TXTNATURALSIZE(4i32);
pub const TXTNS_INCLUDELASTLINE: TXTNATURALSIZE = TXTNATURALSIZE(1073741824i32);
pub const TXTNS_EMU: TXTNATURALSIZE = TXTNATURALSIZE(-2147483648i32);
impl ::core::convert::From<i32> for TXTNATURALSIZE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TXTNATURALSIZE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TXTVIEW(pub i32);
pub const TXTVIEW_ACTIVE: TXTVIEW = TXTVIEW(0i32);
pub const TXTVIEW_INACTIVE: TXTVIEW = TXTVIEW(-1i32);
impl ::core::convert::From<i32> for TXTVIEW {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TXTVIEW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UNDONAMEID(pub i32);
pub const UID_UNKNOWN: UNDONAMEID = UNDONAMEID(0i32);
pub const UID_TYPING: UNDONAMEID = UNDONAMEID(1i32);
pub const UID_DELETE: UNDONAMEID = UNDONAMEID(2i32);
pub const UID_DRAGDROP: UNDONAMEID = UNDONAMEID(3i32);
pub const UID_CUT: UNDONAMEID = UNDONAMEID(4i32);
pub const UID_PASTE: UNDONAMEID = UNDONAMEID(5i32);
pub const UID_AUTOTABLE: UNDONAMEID = UNDONAMEID(6i32);
impl ::core::convert::From<i32> for UNDONAMEID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UNDONAMEID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const VM_NORMAL: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const VM_OUTLINE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const VM_PAGE: u32 = 9u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WBF_CUSTOM: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WBF_LEVEL1: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WBF_LEVEL2: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WBF_OVERFLOW: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WBF_WORDBREAK: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WBF_WORDWRAP: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WB_MOVEWORDNEXT: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WB_MOVEWORDPREV: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WB_NEXTBREAK: u32 = 7u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WB_PREVBREAK: u32 = 6u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WM_CONTEXTMENU: u32 = 123u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WM_NOTIFY: u32 = 78u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WM_PRINTCLIENT: u32 = 792u32;
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub const WM_UNICHAR: u32 = 265u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`, `Win32_Foundation`*"]
pub struct _grouptypingchange {
    pub nmhdr: super::NMHDR,
    pub fGroupTyping: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl _grouptypingchange {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _grouptypingchange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for _grouptypingchange {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for _grouptypingchange {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for _grouptypingchange {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
pub struct hyphresult {
    pub khyph: KHYPH,
    pub ichHyph: i32,
    pub chHyph: u16,
}
impl hyphresult {}
impl ::core::default::Default for hyphresult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for hyphresult {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("hyphresult").field("khyph", &self.khyph).field("ichHyph", &self.ichHyph).field("chHyph", &self.chHyph).finish()
    }
}
impl ::core::cmp::PartialEq for hyphresult {
    fn eq(&self, other: &Self) -> bool {
        self.khyph == other.khyph && self.ichHyph == other.ichHyph && self.chHyph == other.chHyph
    }
}
impl ::core::cmp::Eq for hyphresult {}
unsafe impl ::windows::runtime::Abi for hyphresult {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls_RichEdit`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct tomConstants(pub i32);
pub const tomFalse: tomConstants = tomConstants(0i32);
pub const tomTrue: tomConstants = tomConstants(-1i32);
pub const tomUndefined: tomConstants = tomConstants(-9999999i32);
pub const tomToggle: tomConstants = tomConstants(-9999998i32);
pub const tomAutoColor: tomConstants = tomConstants(-9999997i32);
pub const tomDefault: tomConstants = tomConstants(-9999996i32);
pub const tomSuspend: tomConstants = tomConstants(-9999995i32);
pub const tomResume: tomConstants = tomConstants(-9999994i32);
pub const tomApplyNow: tomConstants = tomConstants(0i32);
pub const tomApplyLater: tomConstants = tomConstants(1i32);
pub const tomTrackParms: tomConstants = tomConstants(2i32);
pub const tomCacheParms: tomConstants = tomConstants(3i32);
pub const tomApplyTmp: tomConstants = tomConstants(4i32);
pub const tomDisableSmartFont: tomConstants = tomConstants(8i32);
pub const tomEnableSmartFont: tomConstants = tomConstants(9i32);
pub const tomUsePoints: tomConstants = tomConstants(10i32);
pub const tomUseTwips: tomConstants = tomConstants(11i32);
pub const tomBackward: tomConstants = tomConstants(-1073741823i32);
pub const tomForward: tomConstants = tomConstants(1073741823i32);
pub const tomMove: tomConstants = tomConstants(0i32);
pub const tomExtend: tomConstants = tomConstants(1i32);
pub const tomNoSelection: tomConstants = tomConstants(0i32);
pub const tomSelectionIP: tomConstants = tomConstants(1i32);
pub const tomSelectionNormal: tomConstants = tomConstants(2i32);
pub const tomSelectionFrame: tomConstants = tomConstants(3i32);
pub const tomSelectionColumn: tomConstants = tomConstants(4i32);
pub const tomSelectionRow: tomConstants = tomConstants(5i32);
pub const tomSelectionBlock: tomConstants = tomConstants(6i32);
pub const tomSelectionInlineShape: tomConstants = tomConstants(7i32);
pub const tomSelectionShape: tomConstants = tomConstants(8i32);
pub const tomSelStartActive: tomConstants = tomConstants(1i32);
pub const tomSelAtEOL: tomConstants = tomConstants(2i32);
pub const tomSelOvertype: tomConstants = tomConstants(4i32);
pub const tomSelActive: tomConstants = tomConstants(8i32);
pub const tomSelReplace: tomConstants = tomConstants(16i32);
pub const tomEnd: tomConstants = tomConstants(0i32);
pub const tomStart: tomConstants = tomConstants(32i32);
pub const tomCollapseEnd: tomConstants = tomConstants(0i32);
pub const tomCollapseStart: tomConstants = tomConstants(1i32);
pub const tomClientCoord: tomConstants = tomConstants(256i32);
pub const tomAllowOffClient: tomConstants = tomConstants(512i32);
pub const tomTransform: tomConstants = tomConstants(1024i32);
pub const tomObjectArg: tomConstants = tomConstants(2048i32);
pub const tomAtEnd: tomConstants = tomConstants(4096i32);
pub const tomNone: tomConstants = tomConstants(0i32);
pub const tomSingle: tomConstants = tomConstants(1i32);
pub const tomWords: tomConstants = tomConstants(2i32);
pub const tomDouble: tomConstants = tomConstants(3i32);
pub const tomDotted: tomConstants = tomConstants(4i32);
pub const tomDash: tomConstants = tomConstants(5i32);
pub const tomDashDot: tomConstants = tomConstants(6i32);
pub const tomDashDotDot: tomConstants = tomConstants(7i32);
pub const tomWave: tomConstants = tomConstants(8i32);
pub const tomThick: tomConstants = tomConstants(9i32);
pub const tomHair: tomConstants = tomConstants(10i32);
pub const tomDoubleWave: tomConstants = tomConstants(11i32);
pub const tomHeavyWave: tomConstants = tomConstants(12i32);
pub const tomLongDash: tomConstants = tomConstants(13i32);
pub const tomThickDash: tomConstants = tomConstants(14i32);
pub const tomThickDashDot: tomConstants = tomConstants(15i32);
pub const tomThickDashDotDot: tomConstants = tomConstants(16i32);
pub const tomThickDotted: tomConstants = tomConstants(17i32);
pub const tomThickLongDash: tomConstants = tomConstants(18i32);
pub const tomLineSpaceSingle: tomConstants = tomConstants(0i32);
pub const tomLineSpace1pt5: tomConstants = tomConstants(1i32);
pub const tomLineSpaceDouble: tomConstants = tomConstants(2i32);
pub const tomLineSpaceAtLeast: tomConstants = tomConstants(3i32);
pub const tomLineSpaceExactly: tomConstants = tomConstants(4i32);
pub const tomLineSpaceMultiple: tomConstants = tomConstants(5i32);
pub const tomLineSpacePercent: tomConstants = tomConstants(6i32);
pub const tomAlignLeft: tomConstants = tomConstants(0i32);
pub const tomAlignCenter: tomConstants = tomConstants(1i32);
pub const tomAlignRight: tomConstants = tomConstants(2i32);
pub const tomAlignJustify: tomConstants = tomConstants(3i32);
pub const tomAlignDecimal: tomConstants = tomConstants(3i32);
pub const tomAlignBar: tomConstants = tomConstants(4i32);
pub const tomDefaultTab: tomConstants = tomConstants(5i32);
pub const tomAlignInterWord: tomConstants = tomConstants(3i32);
pub const tomAlignNewspaper: tomConstants = tomConstants(4i32);
pub const tomAlignInterLetter: tomConstants = tomConstants(5i32);
pub const tomAlignScaled: tomConstants = tomConstants(6i32);
pub const tomSpaces: tomConstants = tomConstants(0i32);
pub const tomDots: tomConstants = tomConstants(1i32);
pub const tomDashes: tomConstants = tomConstants(2i32);
pub const tomLines: tomConstants = tomConstants(3i32);
pub const tomThickLines: tomConstants = tomConstants(4i32);
pub const tomEquals: tomConstants = tomConstants(5i32);
pub const tomTabBack: tomConstants = tomConstants(-3i32);
pub const tomTabNext: tomConstants = tomConstants(-2i32);
pub const tomTabHere: tomConstants = tomConstants(-1i32);
pub const tomListNone: tomConstants = tomConstants(0i32);
pub const tomListBullet: tomConstants = tomConstants(1i32);
pub const tomListNumberAsArabic: tomConstants = tomConstants(2i32);
pub const tomListNumberAsLCLetter: tomConstants = tomConstants(3i32);
pub const tomListNumberAsUCLetter: tomConstants = tomConstants(4i32);
pub const tomListNumberAsLCRoman: tomConstants = tomConstants(5i32);
pub const tomListNumberAsUCRoman: tomConstants = tomConstants(6i32);
pub const tomListNumberAsSequence: tomConstants = tomConstants(7i32);
pub const tomListNumberedCircle: tomConstants = tomConstants(8i32);
pub const tomListNumberedBlackCircleWingding: tomConstants = tomConstants(9i32);
pub const tomListNumberedWhiteCircleWingding: tomConstants = tomConstants(10i32);
pub const tomListNumberedArabicWide: tomConstants = tomConstants(11i32);
pub const tomListNumberedChS: tomConstants = tomConstants(12i32);
pub const tomListNumberedChT: tomConstants = tomConstants(13i32);
pub const tomListNumberedJpnChS: tomConstants = tomConstants(14i32);
pub const tomListNumberedJpnKor: tomConstants = tomConstants(15i32);
pub const tomListNumberedArabic1: tomConstants = tomConstants(16i32);
pub const tomListNumberedArabic2: tomConstants = tomConstants(17i32);
pub const tomListNumberedHebrew: tomConstants = tomConstants(18i32);
pub const tomListNumberedThaiAlpha: tomConstants = tomConstants(19i32);
pub const tomListNumberedThaiNum: tomConstants = tomConstants(20i32);
pub const tomListNumberedHindiAlpha: tomConstants = tomConstants(21i32);
pub const tomListNumberedHindiAlpha1: tomConstants = tomConstants(22i32);
pub const tomListNumberedHindiNum: tomConstants = tomConstants(23i32);
pub const tomListParentheses: tomConstants = tomConstants(65536i32);
pub const tomListPeriod: tomConstants = tomConstants(131072i32);
pub const tomListPlain: tomConstants = tomConstants(196608i32);
pub const tomListNoNumber: tomConstants = tomConstants(262144i32);
pub const tomListMinus: tomConstants = tomConstants(524288i32);
pub const tomIgnoreNumberStyle: tomConstants = tomConstants(16777216i32);
pub const tomParaStyleNormal: tomConstants = tomConstants(-1i32);
pub const tomParaStyleHeading1: tomConstants = tomConstants(-2i32);
pub const tomParaStyleHeading2: tomConstants = tomConstants(-3i32);
pub const tomParaStyleHeading3: tomConstants = tomConstants(-4i32);
pub const tomParaStyleHeading4: tomConstants = tomConstants(-5i32);
pub const tomParaStyleHeading5: tomConstants = tomConstants(-6i32);
pub const tomParaStyleHeading6: tomConstants = tomConstants(-7i32);
pub const tomParaStyleHeading7: tomConstants = tomConstants(-8i32);
pub const tomParaStyleHeading8: tomConstants = tomConstants(-9i32);
pub const tomParaStyleHeading9: tomConstants = tomConstants(-10i32);
pub const tomCharacter: tomConstants = tomConstants(1i32);
pub const tomWord: tomConstants = tomConstants(2i32);
pub const tomSentence: tomConstants = tomConstants(3i32);
pub const tomParagraph: tomConstants = tomConstants(4i32);
pub const tomLine: tomConstants = tomConstants(5i32);
pub const tomStory: tomConstants = tomConstants(6i32);
pub const tomScreen: tomConstants = tomConstants(7i32);
pub const tomSection: tomConstants = tomConstants(8i32);
pub const tomTableColumn: tomConstants = tomConstants(9i32);
pub const tomColumn: tomConstants = tomConstants(9i32);
pub const tomRow: tomConstants = tomConstants(10i32);
pub const tomWindow: tomConstants = tomConstants(11i32);
pub const tomCell: tomConstants = tomConstants(12i32);
pub const tomCharFormat: tomConstants = tomConstants(13i32);
pub const tomParaFormat: tomConstants = tomConstants(14i32);
pub const tomTable: tomConstants = tomConstants(15i32);
pub const tomObject: tomConstants = tomConstants(16i32);
pub const tomPage: tomConstants = tomConstants(17i32);
pub const tomHardParagraph: tomConstants = tomConstants(18i32);
pub const tomCluster: tomConstants = tomConstants(19i32);
pub const tomInlineObject: tomConstants = tomConstants(20i32);
pub const tomInlineObjectArg: tomConstants = tomConstants(21i32);
pub const tomLeafLine: tomConstants = tomConstants(22i32);
pub const tomLayoutColumn: tomConstants = tomConstants(23i32);
pub const tomProcessId: tomConstants = tomConstants(1073741825i32);
pub const tomMatchWord: tomConstants = tomConstants(2i32);
pub const tomMatchCase: tomConstants = tomConstants(4i32);
pub const tomMatchPattern: tomConstants = tomConstants(8i32);
pub const tomUnknownStory: tomConstants = tomConstants(0i32);
pub const tomMainTextStory: tomConstants = tomConstants(1i32);
pub const tomFootnotesStory: tomConstants = tomConstants(2i32);
pub const tomEndnotesStory: tomConstants = tomConstants(3i32);
pub const tomCommentsStory: tomConstants = tomConstants(4i32);
pub const tomTextFrameStory: tomConstants = tomConstants(5i32);
pub const tomEvenPagesHeaderStory: tomConstants = tomConstants(6i32);
pub const tomPrimaryHeaderStory: tomConstants = tomConstants(7i32);
pub const tomEvenPagesFooterStory: tomConstants = tomConstants(8i32);
pub const tomPrimaryFooterStory: tomConstants = tomConstants(9i32);
pub const tomFirstPageHeaderStory: tomConstants = tomConstants(10i32);
pub const tomFirstPageFooterStory: tomConstants = tomConstants(11i32);
pub const tomScratchStory: tomConstants = tomConstants(127i32);
pub const tomFindStory: tomConstants = tomConstants(128i32);
pub const tomReplaceStory: tomConstants = tomConstants(129i32);
pub const tomStoryInactive: tomConstants = tomConstants(0i32);
pub const tomStoryActiveDisplay: tomConstants = tomConstants(1i32);
pub const tomStoryActiveUI: tomConstants = tomConstants(2i32);
pub const tomStoryActiveDisplayUI: tomConstants = tomConstants(3i32);
pub const tomNoAnimation: tomConstants = tomConstants(0i32);
pub const tomLasVegasLights: tomConstants = tomConstants(1i32);
pub const tomBlinkingBackground: tomConstants = tomConstants(2i32);
pub const tomSparkleText: tomConstants = tomConstants(3i32);
pub const tomMarchingBlackAnts: tomConstants = tomConstants(4i32);
pub const tomMarchingRedAnts: tomConstants = tomConstants(5i32);
pub const tomShimmer: tomConstants = tomConstants(6i32);
pub const tomWipeDown: tomConstants = tomConstants(7i32);
pub const tomWipeRight: tomConstants = tomConstants(8i32);
pub const tomAnimationMax: tomConstants = tomConstants(8i32);
pub const tomLowerCase: tomConstants = tomConstants(0i32);
pub const tomUpperCase: tomConstants = tomConstants(1i32);
pub const tomTitleCase: tomConstants = tomConstants(2i32);
pub const tomSentenceCase: tomConstants = tomConstants(4i32);
pub const tomToggleCase: tomConstants = tomConstants(5i32);
pub const tomReadOnly: tomConstants = tomConstants(256i32);
pub const tomShareDenyRead: tomConstants = tomConstants(512i32);
pub const tomShareDenyWrite: tomConstants = tomConstants(1024i32);
pub const tomPasteFile: tomConstants = tomConstants(4096i32);
pub const tomCreateNew: tomConstants = tomConstants(16i32);
pub const tomCreateAlways: tomConstants = tomConstants(32i32);
pub const tomOpenExisting: tomConstants = tomConstants(48i32);
pub const tomOpenAlways: tomConstants = tomConstants(64i32);
pub const tomTruncateExisting: tomConstants = tomConstants(80i32);
pub const tomRTF: tomConstants = tomConstants(1i32);
pub const tomText: tomConstants = tomConstants(2i32);
pub const tomHTML: tomConstants = tomConstants(3i32);
pub const tomWordDocument: tomConstants = tomConstants(4i32);
pub const tomBold: tomConstants = tomConstants(-2147483647i32);
pub const tomItalic: tomConstants = tomConstants(-2147483646i32);
pub const tomUnderline: tomConstants = tomConstants(-2147483644i32);
pub const tomStrikeout: tomConstants = tomConstants(-2147483640i32);
pub const tomProtected: tomConstants = tomConstants(-2147483632i32);
pub const tomLink: tomConstants = tomConstants(-2147483616i32);
pub const tomSmallCaps: tomConstants = tomConstants(-2147483584i32);
pub const tomAllCaps: tomConstants = tomConstants(-2147483520i32);
pub const tomHidden: tomConstants = tomConstants(-2147483392i32);
pub const tomOutline: tomConstants = tomConstants(-2147483136i32);
pub const tomShadow: tomConstants = tomConstants(-2147482624i32);
pub const tomEmboss: tomConstants = tomConstants(-2147481600i32);
pub const tomImprint: tomConstants = tomConstants(-2147479552i32);
pub const tomDisabled: tomConstants = tomConstants(-2147475456i32);
pub const tomRevised: tomConstants = tomConstants(-2147467264i32);
pub const tomSubscriptCF: tomConstants = tomConstants(-2147418112i32);
pub const tomSuperscriptCF: tomConstants = tomConstants(-2147352576i32);
pub const tomFontBound: tomConstants = tomConstants(-2146435072i32);
pub const tomLinkProtected: tomConstants = tomConstants(-2139095040i32);
pub const tomInlineObjectStart: tomConstants = tomConstants(-2130706432i32);
pub const tomExtendedChar: tomConstants = tomConstants(-2113929216i32);
pub const tomAutoBackColor: tomConstants = tomConstants(-2080374784i32);
pub const tomMathZoneNoBuildUp: tomConstants = tomConstants(-2013265920i32);
pub const tomMathZone: tomConstants = tomConstants(-1879048192i32);
pub const tomMathZoneOrdinary: tomConstants = tomConstants(-1610612736i32);
pub const tomAutoTextColor: tomConstants = tomConstants(-1073741824i32);
pub const tomMathZoneDisplay: tomConstants = tomConstants(262144i32);
pub const tomParaEffectRTL: tomConstants = tomConstants(1i32);
pub const tomParaEffectKeep: tomConstants = tomConstants(2i32);
pub const tomParaEffectKeepNext: tomConstants = tomConstants(4i32);
pub const tomParaEffectPageBreakBefore: tomConstants = tomConstants(8i32);
pub const tomParaEffectNoLineNumber: tomConstants = tomConstants(16i32);
pub const tomParaEffectNoWidowControl: tomConstants = tomConstants(32i32);
pub const tomParaEffectDoNotHyphen: tomConstants = tomConstants(64i32);
pub const tomParaEffectSideBySide: tomConstants = tomConstants(128i32);
pub const tomParaEffectCollapsed: tomConstants = tomConstants(256i32);
pub const tomParaEffectOutlineLevel: tomConstants = tomConstants(512i32);
pub const tomParaEffectBox: tomConstants = tomConstants(1024i32);
pub const tomParaEffectTableRowDelimiter: tomConstants = tomConstants(4096i32);
pub const tomParaEffectTable: tomConstants = tomConstants(16384i32);
pub const tomModWidthPairs: tomConstants = tomConstants(1i32);
pub const tomModWidthSpace: tomConstants = tomConstants(2i32);
pub const tomAutoSpaceAlpha: tomConstants = tomConstants(4i32);
pub const tomAutoSpaceNumeric: tomConstants = tomConstants(8i32);
pub const tomAutoSpaceParens: tomConstants = tomConstants(16i32);
pub const tomEmbeddedFont: tomConstants = tomConstants(32i32);
pub const tomDoublestrike: tomConstants = tomConstants(64i32);
pub const tomOverlapping: tomConstants = tomConstants(128i32);
pub const tomNormalCaret: tomConstants = tomConstants(0i32);
pub const tomKoreanBlockCaret: tomConstants = tomConstants(1i32);
pub const tomNullCaret: tomConstants = tomConstants(2i32);
pub const tomIncludeInset: tomConstants = tomConstants(1i32);
pub const tomUnicodeBiDi: tomConstants = tomConstants(1i32);
pub const tomMathCFCheck: tomConstants = tomConstants(4i32);
pub const tomUnlink: tomConstants = tomConstants(8i32);
pub const tomUnhide: tomConstants = tomConstants(16i32);
pub const tomCheckTextLimit: tomConstants = tomConstants(32i32);
pub const tomIgnoreCurrentFont: tomConstants = tomConstants(0i32);
pub const tomMatchCharRep: tomConstants = tomConstants(1i32);
pub const tomMatchFontSignature: tomConstants = tomConstants(2i32);
pub const tomMatchAscii: tomConstants = tomConstants(4i32);
pub const tomGetHeightOnly: tomConstants = tomConstants(8i32);
pub const tomMatchMathFont: tomConstants = tomConstants(16i32);
pub const tomCharset: tomConstants = tomConstants(-2147483648i32);
pub const tomCharRepFromLcid: tomConstants = tomConstants(1073741824i32);
pub const tomAnsi: tomConstants = tomConstants(0i32);
pub const tomEastEurope: tomConstants = tomConstants(1i32);
pub const tomCyrillic: tomConstants = tomConstants(2i32);
pub const tomGreek: tomConstants = tomConstants(3i32);
pub const tomTurkish: tomConstants = tomConstants(4i32);
pub const tomHebrew: tomConstants = tomConstants(5i32);
pub const tomArabic: tomConstants = tomConstants(6i32);
pub const tomBaltic: tomConstants = tomConstants(7i32);
pub const tomVietnamese: tomConstants = tomConstants(8i32);
pub const tomDefaultCharRep: tomConstants = tomConstants(9i32);
pub const tomSymbol: tomConstants = tomConstants(10i32);
pub const tomThai: tomConstants = tomConstants(11i32);
pub const tomShiftJIS: tomConstants = tomConstants(12i32);
pub const tomGB2312: tomConstants = tomConstants(13i32);
pub const tomHangul: tomConstants = tomConstants(14i32);
pub const tomBIG5: tomConstants = tomConstants(15i32);
pub const tomPC437: tomConstants = tomConstants(16i32);
pub const tomOEM: tomConstants = tomConstants(17i32);
pub const tomMac: tomConstants = tomConstants(18i32);
pub const tomArmenian: tomConstants = tomConstants(19i32);
pub const tomSyriac: tomConstants = tomConstants(20i32);
pub const tomThaana: tomConstants = tomConstants(21i32);
pub const tomDevanagari: tomConstants = tomConstants(22i32);
pub const tomBengali: tomConstants = tomConstants(23i32);
pub const tomGurmukhi: tomConstants = tomConstants(24i32);
pub const tomGujarati: tomConstants = tomConstants(25i32);
pub const tomOriya: tomConstants = tomConstants(26i32);
pub const tomTamil: tomConstants = tomConstants(27i32);
pub const tomTelugu: tomConstants = tomConstants(28i32);
pub const tomKannada: tomConstants = tomConstants(29i32);
pub const tomMalayalam: tomConstants = tomConstants(30i32);
pub const tomSinhala: tomConstants = tomConstants(31i32);
pub const tomLao: tomConstants = tomConstants(32i32);
pub const tomTibetan: tomConstants = tomConstants(33i32);
pub const tomMyanmar: tomConstants = tomConstants(34i32);
pub const tomGeorgian: tomConstants = tomConstants(35i32);
pub const tomJamo: tomConstants = tomConstants(36i32);
pub const tomEthiopic: tomConstants = tomConstants(37i32);
pub const tomCherokee: tomConstants = tomConstants(38i32);
pub const tomAboriginal: tomConstants = tomConstants(39i32);
pub const tomOgham: tomConstants = tomConstants(40i32);
pub const tomRunic: tomConstants = tomConstants(41i32);
pub const tomKhmer: tomConstants = tomConstants(42i32);
pub const tomMongolian: tomConstants = tomConstants(43i32);
pub const tomBraille: tomConstants = tomConstants(44i32);
pub const tomYi: tomConstants = tomConstants(45i32);
pub const tomLimbu: tomConstants = tomConstants(46i32);
pub const tomTaiLe: tomConstants = tomConstants(47i32);
pub const tomNewTaiLue: tomConstants = tomConstants(48i32);
pub const tomSylotiNagri: tomConstants = tomConstants(49i32);
pub const tomKharoshthi: tomConstants = tomConstants(50i32);
pub const tomKayahli: tomConstants = tomConstants(51i32);
pub const tomUsymbol: tomConstants = tomConstants(52i32);
pub const tomEmoji: tomConstants = tomConstants(53i32);
pub const tomGlagolitic: tomConstants = tomConstants(54i32);
pub const tomLisu: tomConstants = tomConstants(55i32);
pub const tomVai: tomConstants = tomConstants(56i32);
pub const tomNKo: tomConstants = tomConstants(57i32);
pub const tomOsmanya: tomConstants = tomConstants(58i32);
pub const tomPhagsPa: tomConstants = tomConstants(59i32);
pub const tomGothic: tomConstants = tomConstants(60i32);
pub const tomDeseret: tomConstants = tomConstants(61i32);
pub const tomTifinagh: tomConstants = tomConstants(62i32);
pub const tomCharRepMax: tomConstants = tomConstants(63i32);
pub const tomRE10Mode: tomConstants = tomConstants(1i32);
pub const tomUseAtFont: tomConstants = tomConstants(2i32);
pub const tomTextFlowMask: tomConstants = tomConstants(12i32);
pub const tomTextFlowES: tomConstants = tomConstants(0i32);
pub const tomTextFlowSW: tomConstants = tomConstants(4i32);
pub const tomTextFlowWN: tomConstants = tomConstants(8i32);
pub const tomTextFlowNE: tomConstants = tomConstants(12i32);
pub const tomNoIME: tomConstants = tomConstants(524288i32);
pub const tomSelfIME: tomConstants = tomConstants(262144i32);
pub const tomNoUpScroll: tomConstants = tomConstants(65536i32);
pub const tomNoVpScroll: tomConstants = tomConstants(262144i32);
pub const tomNoLink: tomConstants = tomConstants(0i32);
pub const tomClientLink: tomConstants = tomConstants(1i32);
pub const tomFriendlyLinkName: tomConstants = tomConstants(2i32);
pub const tomFriendlyLinkAddress: tomConstants = tomConstants(3i32);
pub const tomAutoLinkURL: tomConstants = tomConstants(4i32);
pub const tomAutoLinkEmail: tomConstants = tomConstants(5i32);
pub const tomAutoLinkPhone: tomConstants = tomConstants(6i32);
pub const tomAutoLinkPath: tomConstants = tomConstants(7i32);
pub const tomCompressNone: tomConstants = tomConstants(0i32);
pub const tomCompressPunctuation: tomConstants = tomConstants(1i32);
pub const tomCompressPunctuationAndKana: tomConstants = tomConstants(2i32);
pub const tomCompressMax: tomConstants = tomConstants(2i32);
pub const tomUnderlinePositionAuto: tomConstants = tomConstants(0i32);
pub const tomUnderlinePositionBelow: tomConstants = tomConstants(1i32);
pub const tomUnderlinePositionAbove: tomConstants = tomConstants(2i32);
pub const tomUnderlinePositionMax: tomConstants = tomConstants(2i32);
pub const tomFontAlignmentAuto: tomConstants = tomConstants(0i32);
pub const tomFontAlignmentTop: tomConstants = tomConstants(1i32);
pub const tomFontAlignmentBaseline: tomConstants = tomConstants(2i32);
pub const tomFontAlignmentBottom: tomConstants = tomConstants(3i32);
pub const tomFontAlignmentCenter: tomConstants = tomConstants(4i32);
pub const tomFontAlignmentMax: tomConstants = tomConstants(4i32);
pub const tomRubyBelow: tomConstants = tomConstants(128i32);
pub const tomRubyAlignCenter: tomConstants = tomConstants(0i32);
pub const tomRubyAlign010: tomConstants = tomConstants(1i32);
pub const tomRubyAlign121: tomConstants = tomConstants(2i32);
pub const tomRubyAlignLeft: tomConstants = tomConstants(3i32);
pub const tomRubyAlignRight: tomConstants = tomConstants(4i32);
pub const tomLimitsDefault: tomConstants = tomConstants(0i32);
pub const tomLimitsUnderOver: tomConstants = tomConstants(1i32);
pub const tomLimitsSubSup: tomConstants = tomConstants(2i32);
pub const tomUpperLimitAsSuperScript: tomConstants = tomConstants(3i32);
pub const tomLimitsOpposite: tomConstants = tomConstants(4i32);
pub const tomShowLLimPlaceHldr: tomConstants = tomConstants(8i32);
pub const tomShowULimPlaceHldr: tomConstants = tomConstants(16i32);
pub const tomDontGrowWithContent: tomConstants = tomConstants(64i32);
pub const tomGrowWithContent: tomConstants = tomConstants(128i32);
pub const tomSubSupAlign: tomConstants = tomConstants(1i32);
pub const tomLimitAlignMask: tomConstants = tomConstants(3i32);
pub const tomLimitAlignCenter: tomConstants = tomConstants(0i32);
pub const tomLimitAlignLeft: tomConstants = tomConstants(1i32);
pub const tomLimitAlignRight: tomConstants = tomConstants(2i32);
pub const tomShowDegPlaceHldr: tomConstants = tomConstants(8i32);
pub const tomAlignDefault: tomConstants = tomConstants(0i32);
pub const tomAlignMatchAscentDescent: tomConstants = tomConstants(2i32);
pub const tomMathVariant: tomConstants = tomConstants(32i32);
pub const tomStyleDefault: tomConstants = tomConstants(0i32);
pub const tomStyleScriptScriptCramped: tomConstants = tomConstants(1i32);
pub const tomStyleScriptScript: tomConstants = tomConstants(2i32);
pub const tomStyleScriptCramped: tomConstants = tomConstants(3i32);
pub const tomStyleScript: tomConstants = tomConstants(4i32);
pub const tomStyleTextCramped: tomConstants = tomConstants(5i32);
pub const tomStyleText: tomConstants = tomConstants(6i32);
pub const tomStyleDisplayCramped: tomConstants = tomConstants(7i32);
pub const tomStyleDisplay: tomConstants = tomConstants(8i32);
pub const tomMathRelSize: tomConstants = tomConstants(64i32);
pub const tomDecDecSize: tomConstants = tomConstants(254i32);
pub const tomDecSize: tomConstants = tomConstants(255i32);
pub const tomIncSize: tomConstants = tomConstants(65i32);
pub const tomIncIncSize: tomConstants = tomConstants(66i32);
pub const tomGravityUI: tomConstants = tomConstants(0i32);
pub const tomGravityBack: tomConstants = tomConstants(1i32);
pub const tomGravityFore: tomConstants = tomConstants(2i32);
pub const tomGravityIn: tomConstants = tomConstants(3i32);
pub const tomGravityOut: tomConstants = tomConstants(4i32);
pub const tomGravityBackward: tomConstants = tomConstants(536870912i32);
pub const tomGravityForward: tomConstants = tomConstants(1073741824i32);
pub const tomAdjustCRLF: tomConstants = tomConstants(1i32);
pub const tomUseCRLF: tomConstants = tomConstants(2i32);
pub const tomTextize: tomConstants = tomConstants(4i32);
pub const tomAllowFinalEOP: tomConstants = tomConstants(8i32);
pub const tomFoldMathAlpha: tomConstants = tomConstants(16i32);
pub const tomNoHidden: tomConstants = tomConstants(32i32);
pub const tomIncludeNumbering: tomConstants = tomConstants(64i32);
pub const tomTranslateTableCell: tomConstants = tomConstants(128i32);
pub const tomNoMathZoneBrackets: tomConstants = tomConstants(256i32);
pub const tomConvertMathChar: tomConstants = tomConstants(512i32);
pub const tomNoUCGreekItalic: tomConstants = tomConstants(1024i32);
pub const tomAllowMathBold: tomConstants = tomConstants(2048i32);
pub const tomLanguageTag: tomConstants = tomConstants(4096i32);
pub const tomConvertRTF: tomConstants = tomConstants(8192i32);
pub const tomApplyRtfDocProps: tomConstants = tomConstants(16384i32);
pub const tomPhantomShow: tomConstants = tomConstants(1i32);
pub const tomPhantomZeroWidth: tomConstants = tomConstants(2i32);
pub const tomPhantomZeroAscent: tomConstants = tomConstants(4i32);
pub const tomPhantomZeroDescent: tomConstants = tomConstants(8i32);
pub const tomPhantomTransparent: tomConstants = tomConstants(16i32);
pub const tomPhantomASmash: tomConstants = tomConstants(5i32);
pub const tomPhantomDSmash: tomConstants = tomConstants(9i32);
pub const tomPhantomHSmash: tomConstants = tomConstants(3i32);
pub const tomPhantomSmash: tomConstants = tomConstants(13i32);
pub const tomPhantomHorz: tomConstants = tomConstants(12i32);
pub const tomPhantomVert: tomConstants = tomConstants(2i32);
pub const tomBoxHideTop: tomConstants = tomConstants(1i32);
pub const tomBoxHideBottom: tomConstants = tomConstants(2i32);
pub const tomBoxHideLeft: tomConstants = tomConstants(4i32);
pub const tomBoxHideRight: tomConstants = tomConstants(8i32);
pub const tomBoxStrikeH: tomConstants = tomConstants(16i32);
pub const tomBoxStrikeV: tomConstants = tomConstants(32i32);
pub const tomBoxStrikeTLBR: tomConstants = tomConstants(64i32);
pub const tomBoxStrikeBLTR: tomConstants = tomConstants(128i32);
pub const tomBoxAlignCenter: tomConstants = tomConstants(1i32);
pub const tomSpaceMask: tomConstants = tomConstants(28i32);
pub const tomSpaceDefault: tomConstants = tomConstants(0i32);
pub const tomSpaceUnary: tomConstants = tomConstants(4i32);
pub const tomSpaceBinary: tomConstants = tomConstants(8i32);
pub const tomSpaceRelational: tomConstants = tomConstants(12i32);
pub const tomSpaceSkip: tomConstants = tomConstants(16i32);
pub const tomSpaceOrd: tomConstants = tomConstants(20i32);
pub const tomSpaceDifferential: tomConstants = tomConstants(24i32);
pub const tomSizeText: tomConstants = tomConstants(32i32);
pub const tomSizeScript: tomConstants = tomConstants(64i32);
pub const tomSizeScriptScript: tomConstants = tomConstants(96i32);
pub const tomNoBreak: tomConstants = tomConstants(128i32);
pub const tomTransparentForPositioning: tomConstants = tomConstants(256i32);
pub const tomTransparentForSpacing: tomConstants = tomConstants(512i32);
pub const tomStretchCharBelow: tomConstants = tomConstants(0i32);
pub const tomStretchCharAbove: tomConstants = tomConstants(1i32);
pub const tomStretchBaseBelow: tomConstants = tomConstants(2i32);
pub const tomStretchBaseAbove: tomConstants = tomConstants(3i32);
pub const tomMatrixAlignMask: tomConstants = tomConstants(3i32);
pub const tomMatrixAlignCenter: tomConstants = tomConstants(0i32);
pub const tomMatrixAlignTopRow: tomConstants = tomConstants(1i32);
pub const tomMatrixAlignBottomRow: tomConstants = tomConstants(3i32);
pub const tomShowMatPlaceHldr: tomConstants = tomConstants(8i32);
pub const tomEqArrayLayoutWidth: tomConstants = tomConstants(1i32);
pub const tomEqArrayAlignMask: tomConstants = tomConstants(12i32);
pub const tomEqArrayAlignCenter: tomConstants = tomConstants(0i32);
pub const tomEqArrayAlignTopRow: tomConstants = tomConstants(4i32);
pub const tomEqArrayAlignBottomRow: tomConstants = tomConstants(12i32);
pub const tomMathManualBreakMask: tomConstants = tomConstants(127i32);
pub const tomMathBreakLeft: tomConstants = tomConstants(125i32);
pub const tomMathBreakCenter: tomConstants = tomConstants(126i32);
pub const tomMathBreakRight: tomConstants = tomConstants(127i32);
pub const tomMathEqAlign: tomConstants = tomConstants(128i32);
pub const tomMathArgShadingStart: tomConstants = tomConstants(593i32);
pub const tomMathArgShadingEnd: tomConstants = tomConstants(594i32);
pub const tomMathObjShadingStart: tomConstants = tomConstants(595i32);
pub const tomMathObjShadingEnd: tomConstants = tomConstants(596i32);
pub const tomFunctionTypeNone: tomConstants = tomConstants(0i32);
pub const tomFunctionTypeTakesArg: tomConstants = tomConstants(1i32);
pub const tomFunctionTypeTakesLim: tomConstants = tomConstants(2i32);
pub const tomFunctionTypeTakesLim2: tomConstants = tomConstants(3i32);
pub const tomFunctionTypeIsLim: tomConstants = tomConstants(4i32);
pub const tomMathParaAlignDefault: tomConstants = tomConstants(0i32);
pub const tomMathParaAlignCenterGroup: tomConstants = tomConstants(1i32);
pub const tomMathParaAlignCenter: tomConstants = tomConstants(2i32);
pub const tomMathParaAlignLeft: tomConstants = tomConstants(3i32);
pub const tomMathParaAlignRight: tomConstants = tomConstants(4i32);
pub const tomMathDispAlignMask: tomConstants = tomConstants(3i32);
pub const tomMathDispAlignCenterGroup: tomConstants = tomConstants(0i32);
pub const tomMathDispAlignCenter: tomConstants = tomConstants(1i32);
pub const tomMathDispAlignLeft: tomConstants = tomConstants(2i32);
pub const tomMathDispAlignRight: tomConstants = tomConstants(3i32);
pub const tomMathDispIntUnderOver: tomConstants = tomConstants(4i32);
pub const tomMathDispFracTeX: tomConstants = tomConstants(8i32);
pub const tomMathDispNaryGrow: tomConstants = tomConstants(16i32);
pub const tomMathDocEmptyArgMask: tomConstants = tomConstants(96i32);
pub const tomMathDocEmptyArgAuto: tomConstants = tomConstants(0i32);
pub const tomMathDocEmptyArgAlways: tomConstants = tomConstants(32i32);
pub const tomMathDocEmptyArgNever: tomConstants = tomConstants(64i32);
pub const tomMathDocSbSpOpUnchanged: tomConstants = tomConstants(128i32);
pub const tomMathDocDiffMask: tomConstants = tomConstants(768i32);
pub const tomMathDocDiffDefault: tomConstants = tomConstants(0i32);
pub const tomMathDocDiffUpright: tomConstants = tomConstants(256i32);
pub const tomMathDocDiffItalic: tomConstants = tomConstants(512i32);
pub const tomMathDocDiffOpenItalic: tomConstants = tomConstants(768i32);
pub const tomMathDispNarySubSup: tomConstants = tomConstants(1024i32);
pub const tomMathDispDef: tomConstants = tomConstants(2048i32);
pub const tomMathEnableRtl: tomConstants = tomConstants(4096i32);
pub const tomMathBrkBinMask: tomConstants = tomConstants(196608i32);
pub const tomMathBrkBinBefore: tomConstants = tomConstants(0i32);
pub const tomMathBrkBinAfter: tomConstants = tomConstants(65536i32);
pub const tomMathBrkBinDup: tomConstants = tomConstants(131072i32);
pub const tomMathBrkBinSubMask: tomConstants = tomConstants(786432i32);
pub const tomMathBrkBinSubMM: tomConstants = tomConstants(0i32);
pub const tomMathBrkBinSubPM: tomConstants = tomConstants(262144i32);
pub const tomMathBrkBinSubMP: tomConstants = tomConstants(524288i32);
pub const tomSelRange: tomConstants = tomConstants(597i32);
pub const tomHstring: tomConstants = tomConstants(596i32);
pub const tomFontPropTeXStyle: tomConstants = tomConstants(828i32);
pub const tomFontPropAlign: tomConstants = tomConstants(829i32);
pub const tomFontStretch: tomConstants = tomConstants(830i32);
pub const tomFontStyle: tomConstants = tomConstants(831i32);
pub const tomFontStyleUpright: tomConstants = tomConstants(0i32);
pub const tomFontStyleOblique: tomConstants = tomConstants(1i32);
pub const tomFontStyleItalic: tomConstants = tomConstants(2i32);
pub const tomFontStretchDefault: tomConstants = tomConstants(0i32);
pub const tomFontStretchUltraCondensed: tomConstants = tomConstants(1i32);
pub const tomFontStretchExtraCondensed: tomConstants = tomConstants(2i32);
pub const tomFontStretchCondensed: tomConstants = tomConstants(3i32);
pub const tomFontStretchSemiCondensed: tomConstants = tomConstants(4i32);
pub const tomFontStretchNormal: tomConstants = tomConstants(5i32);
pub const tomFontStretchSemiExpanded: tomConstants = tomConstants(6i32);
pub const tomFontStretchExpanded: tomConstants = tomConstants(7i32);
pub const tomFontStretchExtraExpanded: tomConstants = tomConstants(8i32);
pub const tomFontStretchUltraExpanded: tomConstants = tomConstants(9i32);
pub const tomFontWeightDefault: tomConstants = tomConstants(0i32);
pub const tomFontWeightThin: tomConstants = tomConstants(100i32);
pub const tomFontWeightExtraLight: tomConstants = tomConstants(200i32);
pub const tomFontWeightLight: tomConstants = tomConstants(300i32);
pub const tomFontWeightNormal: tomConstants = tomConstants(400i32);
pub const tomFontWeightRegular: tomConstants = tomConstants(400i32);
pub const tomFontWeightMedium: tomConstants = tomConstants(500i32);
pub const tomFontWeightSemiBold: tomConstants = tomConstants(600i32);
pub const tomFontWeightBold: tomConstants = tomConstants(700i32);
pub const tomFontWeightExtraBold: tomConstants = tomConstants(800i32);
pub const tomFontWeightBlack: tomConstants = tomConstants(900i32);
pub const tomFontWeightHeavy: tomConstants = tomConstants(900i32);
pub const tomFontWeightExtraBlack: tomConstants = tomConstants(950i32);
pub const tomParaPropMathAlign: tomConstants = tomConstants(1079i32);
pub const tomDocMathBuild: tomConstants = tomConstants(128i32);
pub const tomMathLMargin: tomConstants = tomConstants(129i32);
pub const tomMathRMargin: tomConstants = tomConstants(130i32);
pub const tomMathWrapIndent: tomConstants = tomConstants(131i32);
pub const tomMathWrapRight: tomConstants = tomConstants(132i32);
pub const tomMathPostSpace: tomConstants = tomConstants(134i32);
pub const tomMathPreSpace: tomConstants = tomConstants(133i32);
pub const tomMathInterSpace: tomConstants = tomConstants(135i32);
pub const tomMathIntraSpace: tomConstants = tomConstants(136i32);
pub const tomCanCopy: tomConstants = tomConstants(137i32);
pub const tomCanRedo: tomConstants = tomConstants(138i32);
pub const tomCanUndo: tomConstants = tomConstants(139i32);
pub const tomUndoLimit: tomConstants = tomConstants(140i32);
pub const tomDocAutoLink: tomConstants = tomConstants(141i32);
pub const tomEllipsisMode: tomConstants = tomConstants(142i32);
pub const tomEllipsisState: tomConstants = tomConstants(143i32);
pub const tomEllipsisNone: tomConstants = tomConstants(0i32);
pub const tomEllipsisEnd: tomConstants = tomConstants(1i32);
pub const tomEllipsisWord: tomConstants = tomConstants(3i32);
pub const tomEllipsisPresent: tomConstants = tomConstants(1i32);
pub const tomVTopCell: tomConstants = tomConstants(1i32);
pub const tomVLowCell: tomConstants = tomConstants(2i32);
pub const tomHStartCell: tomConstants = tomConstants(4i32);
pub const tomHContCell: tomConstants = tomConstants(8i32);
pub const tomRowUpdate: tomConstants = tomConstants(1i32);
pub const tomRowApplyDefault: tomConstants = tomConstants(0i32);
pub const tomCellStructureChangeOnly: tomConstants = tomConstants(1i32);
pub const tomRowHeightActual: tomConstants = tomConstants(2059i32);
impl ::core::convert::From<i32> for tomConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for tomConstants {
    type Abi = Self;
}
