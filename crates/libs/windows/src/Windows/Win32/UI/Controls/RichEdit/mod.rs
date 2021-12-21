#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ATP_CHANGE: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ATP_NOCHANGE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ATP_NODELIMITER: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ATP_REPLACEALLTEXT: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const AURL_DISABLEMIXEDLGC: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const AURL_ENABLEDRIVELETTERS: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const AURL_ENABLEEA: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const AURL_ENABLEEAURLS: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const AURL_ENABLEEMAILADDR: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const AURL_ENABLETELNO: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const AURL_ENABLEURL: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type AutoCorrectProc = ::core::option::Option<unsafe extern "system" fn(langid: u16, pszbefore: super::super::super::Foundation::PWSTR, pszafter: super::super::super::Foundation::PWSTR, cchafter: i32, pcchreplaced: *mut i32) -> i32>;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub struct BIDIOPTIONS {
    pub cbSize: u32,
    pub wMask: u16,
    pub wEffects: u16,
}
impl ::core::marker::Copy for BIDIOPTIONS {}
impl ::core::clone::Clone for BIDIOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BIDIOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIDIOPTIONS").field("cbSize", &self.cbSize).field("wMask", &self.wMask).field("wEffects", &self.wEffects).finish()
    }
}
unsafe impl ::windows::core::Abi for BIDIOPTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BIDIOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BIDIOPTIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for BIDIOPTIONS {}
impl ::core::default::Default for BIDIOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOE_CONTEXTALIGNMENT: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOE_CONTEXTREADING: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOE_FORCERECALC: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOE_LEGACYBIDICLASS: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOE_NEUTRALOVERRIDE: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOE_PLAINTEXT: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOE_RTLDIR: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOE_UNICODEBIDI: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOM_CONTEXTALIGNMENT: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOM_CONTEXTREADING: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOM_DEFPARADIR: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOM_LEGACYBIDICLASS: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOM_NEUTRALOVERRIDE: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOM_PLAINTEXT: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const BOM_UNICODEBIDI: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type CARET_FLAGS = i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CARET_NONE: CARET_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CARET_CUSTOM: CARET_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CARET_RTL: CARET_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CARET_ITALIC: CARET_FLAGS = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CARET_NULL: CARET_FLAGS = 64i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CARET_ROTATE90: CARET_FLAGS = 128i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Graphics_Gdi'*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub union CARET_INFO {
    pub hbitmap: super::super::super::Graphics::Gdi::HBITMAP,
    pub caretFlags: CARET_FLAGS,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for CARET_INFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for CARET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for CARET_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for CARET_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CARET_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for CARET_INFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for CARET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type CFE_EFFECTS = u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_ALLCAPS: CFE_EFFECTS = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_AUTOBACKCOLOR: CFE_EFFECTS = 67108864u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_DISABLED: CFE_EFFECTS = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_EMBOSS: CFE_EFFECTS = 2048u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_HIDDEN: CFE_EFFECTS = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_IMPRINT: CFE_EFFECTS = 4096u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_OUTLINE: CFE_EFFECTS = 512u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_REVISED: CFE_EFFECTS = 16384u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_SHADOW: CFE_EFFECTS = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_SMALLCAPS: CFE_EFFECTS = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_AUTOCOLOR: CFE_EFFECTS = 1073741824u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_BOLD: CFE_EFFECTS = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_ITALIC: CFE_EFFECTS = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_STRIKEOUT: CFE_EFFECTS = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_UNDERLINE: CFE_EFFECTS = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_PROTECTED: CFE_EFFECTS = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_LINK: CFE_EFFECTS = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_SUBSCRIPT: CFE_EFFECTS = 65536u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_SUPERSCRIPT: CFE_EFFECTS = 131072u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_FONTBOUND: CFE_EFFECTS = 1048576u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_LINKPROTECTED: CFE_EFFECTS = 8388608u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_EXTENDED: CFE_EFFECTS = 33554432u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_MATHNOBUILDUP: CFE_EFFECTS = 134217728u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_MATH: CFE_EFFECTS = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFE_MATHORDINARY: CFE_EFFECTS = 536870912u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type CFM_MASK = u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_SUBSCRIPT: CFM_MASK = 196608u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_SUPERSCRIPT: CFM_MASK = 196608u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_EFFECTS: CFM_MASK = 1073741887u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_ALL: CFM_MASK = 4160749631u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_BOLD: CFM_MASK = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_CHARSET: CFM_MASK = 134217728u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_COLOR: CFM_MASK = 1073741824u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_FACE: CFM_MASK = 536870912u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_ITALIC: CFM_MASK = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_OFFSET: CFM_MASK = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_PROTECTED: CFM_MASK = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_SIZE: CFM_MASK = 2147483648u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_STRIKEOUT: CFM_MASK = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_UNDERLINE: CFM_MASK = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_LINK: CFM_MASK = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_SMALLCAPS: CFM_MASK = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_ALLCAPS: CFM_MASK = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_HIDDEN: CFM_MASK = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_OUTLINE: CFM_MASK = 512u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_SHADOW: CFM_MASK = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_EMBOSS: CFM_MASK = 2048u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_IMPRINT: CFM_MASK = 4096u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_DISABLED: CFM_MASK = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_REVISED: CFM_MASK = 16384u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_REVAUTHOR: CFM_MASK = 32768u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_ANIMATION: CFM_MASK = 262144u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_STYLE: CFM_MASK = 524288u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_KERNING: CFM_MASK = 1048576u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_SPACING: CFM_MASK = 2097152u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_WEIGHT: CFM_MASK = 4194304u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_UNDERLINETYPE: CFM_MASK = 8388608u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_COOKIE: CFM_MASK = 16777216u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_LCID: CFM_MASK = 33554432u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_BACKCOLOR: CFM_MASK = 67108864u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_EFFECTS2: CFM_MASK = 1141080063u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_ALL2: CFM_MASK = 4294967295u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_FONTBOUND: CFM_MASK = 1048576u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_LINKPROTECTED: CFM_MASK = 8388608u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_EXTENDED: CFM_MASK = 33554432u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_MATHNOBUILDUP: CFM_MASK = 134217728u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_MATH: CFM_MASK = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_MATHORDINARY: CFM_MASK = 536870912u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CFM_ALLEFFECTS: CFM_MASK = 2115207167u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub struct CHANGENOTIFY {
    pub dwChangeType: CHANGETYPE,
    pub pvCookieData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CHANGENOTIFY {}
impl ::core::clone::Clone for CHANGENOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHANGENOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGENOTIFY").field("dwChangeType", &self.dwChangeType).field("pvCookieData", &self.pvCookieData).finish()
    }
}
unsafe impl ::windows::core::Abi for CHANGENOTIFY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHANGENOTIFY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGENOTIFY>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHANGENOTIFY {}
impl ::core::default::Default for CHANGENOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type CHANGETYPE = i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CN_GENERIC: CHANGETYPE = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CN_TEXTCHANGED: CHANGETYPE = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CN_NEWUNDO: CHANGETYPE = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CN_NEWREDO: CHANGETYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for CHARFORMAT2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHARFORMAT2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHARFORMAT2A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHARFORMAT2A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHARFORMAT2A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHARFORMAT2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHARFORMAT2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union CHARFORMAT2A_0 {
    pub dwReserved: u32,
    pub dwCookie: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHARFORMAT2A_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHARFORMAT2A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHARFORMAT2A_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHARFORMAT2A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHARFORMAT2A_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHARFORMAT2A_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHARFORMAT2A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
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
impl ::core::marker::Copy for CHARFORMAT2W {}
impl ::core::clone::Clone for CHARFORMAT2W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CHARFORMAT2W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHARFORMAT2W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHARFORMAT2W>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHARFORMAT2W {}
impl ::core::default::Default for CHARFORMAT2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub union CHARFORMAT2W_0 {
    pub dwReserved: u32,
    pub dwCookie: u32,
}
impl ::core::marker::Copy for CHARFORMAT2W_0 {}
impl ::core::clone::Clone for CHARFORMAT2W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CHARFORMAT2W_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHARFORMAT2W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHARFORMAT2W_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHARFORMAT2W_0 {}
impl ::core::default::Default for CHARFORMAT2W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for CHARFORMATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHARFORMATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHARFORMATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHARFORMATA").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("dwEffects", &self.dwEffects).field("yHeight", &self.yHeight).field("yOffset", &self.yOffset).field("crTextColor", &self.crTextColor).field("bCharSet", &self.bCharSet).field("bPitchAndFamily", &self.bPitchAndFamily).field("szFaceName", &self.szFaceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHARFORMATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHARFORMATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHARFORMATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHARFORMATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHARFORMATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
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
impl ::core::marker::Copy for CHARFORMATW {}
impl ::core::clone::Clone for CHARFORMATW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHARFORMATW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHARFORMATW").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("dwEffects", &self.dwEffects).field("yHeight", &self.yHeight).field("yOffset", &self.yOffset).field("crTextColor", &self.crTextColor).field("bCharSet", &self.bCharSet).field("bPitchAndFamily", &self.bPitchAndFamily).field("szFaceName", &self.szFaceName).finish()
    }
}
unsafe impl ::windows::core::Abi for CHARFORMATW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHARFORMATW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHARFORMATW>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHARFORMATW {}
impl ::core::default::Default for CHARFORMATW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub struct CHARRANGE {
    pub cpMin: i32,
    pub cpMax: i32,
}
impl ::core::marker::Copy for CHARRANGE {}
impl ::core::clone::Clone for CHARRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHARRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHARRANGE").field("cpMin", &self.cpMin).field("cpMax", &self.cpMax).finish()
    }
}
unsafe impl ::windows::core::Abi for CHARRANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHARRANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHARRANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHARRANGE {}
impl ::core::default::Default for CHARRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLIPBOARDFORMAT {
    pub nmhdr: super::NMHDR,
    pub cf: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLIPBOARDFORMAT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLIPBOARDFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CLIPBOARDFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLIPBOARDFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CLIPBOARDFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLIPBOARDFORMAT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLIPBOARDFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub struct COMPCOLOR {
    pub crText: u32,
    pub crBackground: u32,
    pub dwEffects: u32,
}
impl ::core::marker::Copy for COMPCOLOR {}
impl ::core::clone::Clone for COMPCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPCOLOR").field("crText", &self.crText).field("crBackground", &self.crBackground).field("dwEffects", &self.dwEffects).finish()
    }
}
unsafe impl ::windows::core::Abi for COMPCOLOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMPCOLOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPCOLOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMPCOLOR {}
impl ::core::default::Default for COMPCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CTFMODEBIAS_CONVERSATION: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CTFMODEBIAS_DATETIME: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CTFMODEBIAS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CTFMODEBIAS_FILENAME: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CTFMODEBIAS_FULLWIDTHALPHANUMERIC: u32 = 11u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CTFMODEBIAS_HALFWIDTHALPHANUMERIC: u32 = 12u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CTFMODEBIAS_HALFWIDTHKATAKANA: u32 = 10u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CTFMODEBIAS_HANGUL: u32 = 9u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CTFMODEBIAS_HIRAGANA: u32 = 7u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CTFMODEBIAS_KATAKANA: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CTFMODEBIAS_NAME: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CTFMODEBIAS_NUMERIC: u32 = 6u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const CTFMODEBIAS_READING: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECOOP_AND: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECOOP_OR: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECOOP_SET: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECOOP_XOR: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECO_AUTOHSCROLL: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECO_AUTOVSCROLL: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECO_AUTOWORDSELECTION: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECO_NOHIDESEL: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECO_READONLY: u32 = 2048u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECO_SAVESEL: u32 = 32768u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECO_SELECTIONBAR: u32 = 16777216u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECO_VERTICAL: u32 = 4194304u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECO_WANTRETURN: u32 = 4096u32;
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub struct EDITSTREAM {
    pub dwCookie: usize,
    pub dwError: u32,
    pub pfnCallback: EDITSTREAMCALLBACK,
}
impl ::core::marker::Copy for EDITSTREAM {}
impl ::core::clone::Clone for EDITSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EDITSTREAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EDITSTREAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EDITSTREAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for EDITSTREAM {}
impl ::core::default::Default for EDITSTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type EDITSTREAMCALLBACK = ::core::option::Option<unsafe extern "system" fn(dwcookie: usize, pbbuff: *mut u8, cb: i32, pcb: *mut i32) -> u32>;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type EDITWORDBREAKPROCEX = ::core::option::Option<unsafe extern "system" fn(pchtext: super::super::super::Foundation::PSTR, cchtext: i32, bcharset: u8, action: i32) -> i32>;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ELLIPSIS_END: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ELLIPSIS_MASK: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ELLIPSIS_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ELLIPSIS_WORD: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EMO_ENTER: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EMO_EXIT: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EMO_EXPAND: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EMO_EXPANDDOCUMENT: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EMO_EXPANDSELECTION: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EMO_GETVIEWMODE: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EMO_MOVESELECTION: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EMO_PROMOTE: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_AUTOURLDETECT: u32 = 1115u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_CALLAUTOCORRECTPROC: u32 = 1279u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_CANPASTE: u32 = 1074u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_CANREDO: u32 = 1109u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_CONVPOSITION: u32 = 1132u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_DISPLAYBAND: u32 = 1075u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_EXGETSEL: u32 = 1076u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_EXLIMITTEXT: u32 = 1077u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_EXLINEFROMCHAR: u32 = 1078u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_EXSETSEL: u32 = 1079u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_FINDTEXT: u32 = 1080u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_FINDTEXTEX: u32 = 1103u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_FINDTEXTEXW: u32 = 1148u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_FINDTEXTW: u32 = 1147u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_FINDWORDBREAK: u32 = 1100u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_FORMATRANGE: u32 = 1081u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETAUTOCORRECTPROC: u32 = 1257u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETAUTOURLDETECT: u32 = 1116u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETBIDIOPTIONS: u32 = 1225u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETCHARFORMAT: u32 = 1082u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETCTFMODEBIAS: u32 = 1261u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETCTFOPENSTATUS: u32 = 1264u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETEDITSTYLE: u32 = 1229u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETEDITSTYLEEX: u32 = 1300u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETELLIPSISMODE: u32 = 1329u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETELLIPSISSTATE: u32 = 1346u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETEVENTMASK: u32 = 1083u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETHYPHENATEINFO: u32 = 1254u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETIMECOLOR: u32 = 1129u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETIMECOMPMODE: u32 = 1146u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETIMECOMPTEXT: u32 = 1266u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETIMEMODEBIAS: u32 = 1151u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETIMEOPTIONS: u32 = 1131u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETIMEPROPERTY: u32 = 1268u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETLANGOPTIONS: u32 = 1145u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETOLEINTERFACE: u32 = 1084u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETOPTIONS: u32 = 1102u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETPAGE: u32 = 1252u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETPAGEROTATE: u32 = 1259u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETPARAFORMAT: u32 = 1085u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETPUNCTUATION: u32 = 1125u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETQUERYRTFOBJ: u32 = 1293u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETREDONAME: u32 = 1111u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETSCROLLPOS: u32 = 1245u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETSELTEXT: u32 = 1086u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETSTORYTYPE: u32 = 1314u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETTABLEPARMS: u32 = 1289u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETTEXTEX: u32 = 1118u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETTEXTLENGTHEX: u32 = 1119u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETTEXTMODE: u32 = 1114u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETTEXTRANGE: u32 = 1099u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETTOUCHOPTIONS: u32 = 1334u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETTYPOGRAPHYOPTIONS: u32 = 1227u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETUNDONAME: u32 = 1110u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETVIEWKIND: u32 = 1250u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETWORDBREAKPROCEX: u32 = 1104u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETWORDWRAPMODE: u32 = 1127u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_GETZOOM: u32 = 1248u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_HIDESELECTION: u32 = 1087u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_INSERTIMAGE: u32 = 1338u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_INSERTTABLE: u32 = 1256u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_ISIME: u32 = 1267u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_OUTLINE: u32 = 1244u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_PASTESPECIAL: u32 = 1088u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_RECONVERSION: u32 = 1149u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_REDO: u32 = 1108u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_REQUESTRESIZE: u32 = 1089u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SELECTIONTYPE: u32 = 1090u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETAUTOCORRECTPROC: u32 = 1258u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETBIDIOPTIONS: u32 = 1224u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETBKGNDCOLOR: u32 = 1091u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETCHARFORMAT: u32 = 1092u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETCTFMODEBIAS: u32 = 1262u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETCTFOPENSTATUS: u32 = 1265u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETEDITSTYLE: u32 = 1228u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETEDITSTYLEEX: u32 = 1299u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETELLIPSISMODE: u32 = 1330u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETEVENTMASK: u32 = 1093u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETFONTSIZE: u32 = 1247u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETHYPHENATEINFO: u32 = 1255u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETIMECOLOR: u32 = 1128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETIMEMODEBIAS: u32 = 1150u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETIMEOPTIONS: u32 = 1130u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETLANGOPTIONS: u32 = 1144u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETOLECALLBACK: u32 = 1094u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETOPTIONS: u32 = 1101u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETPAGE: u32 = 1253u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETPAGEROTATE: u32 = 1260u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETPALETTE: u32 = 1117u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETPARAFORMAT: u32 = 1095u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETPUNCTUATION: u32 = 1124u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETQUERYRTFOBJ: u32 = 1294u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETSCROLLPOS: u32 = 1246u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETSTORYTYPE: u32 = 1315u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETTABLEPARMS: u32 = 1331u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETTARGETDEVICE: u32 = 1096u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETTEXTEX: u32 = 1121u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETTEXTMODE: u32 = 1113u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETTOUCHOPTIONS: u32 = 1335u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETTYPOGRAPHYOPTIONS: u32 = 1226u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETUIANAME: u32 = 1344u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETUNDOLIMIT: u32 = 1106u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETVIEWKIND: u32 = 1251u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETWORDBREAKPROCEX: u32 = 1105u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETWORDWRAPMODE: u32 = 1126u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SETZOOM: u32 = 1249u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_SHOWSCROLLBAR: u32 = 1120u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_STOPGROUPTYPING: u32 = 1112u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_STREAMIN: u32 = 1097u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EM_STREAMOUT: u32 = 1098u32;
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENCORRECTTEXT {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENCORRECTTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENCORRECTTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENCORRECTTEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENCORRECTTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENCORRECTTEXT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENCORRECTTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENCORRECTTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENDCOMPOSITIONNOTIFY {
    pub nmhdr: super::NMHDR,
    pub dwCode: ENDCOMPOSITIONNOTIFY_CODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENDCOMPOSITIONNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENDCOMPOSITIONNOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENDCOMPOSITIONNOTIFY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENDCOMPOSITIONNOTIFY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENDCOMPOSITIONNOTIFY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENDCOMPOSITIONNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENDCOMPOSITIONNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type ENDCOMPOSITIONNOTIFY_CODE = u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECN_ENDCOMPOSITION: ENDCOMPOSITIONNOTIFY_CODE = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ECN_NEWTEXT: ENDCOMPOSITIONNOTIFY_CODE = 2u32;
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENDROPFILES {
    pub nmhdr: super::NMHDR,
    pub hDrop: super::super::super::Foundation::HANDLE,
    pub cp: i32,
    pub fProtected: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENDROPFILES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENDROPFILES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENDROPFILES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENDROPFILES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENDROPFILES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENDROPFILES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENDROPFILES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENLINK {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENLINK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENLINK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENLINK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENLINK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENLOWFIRTF {
    pub nmhdr: super::NMHDR,
    pub szControl: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENLOWFIRTF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENLOWFIRTF {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENLOWFIRTF {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENLOWFIRTF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENLOWFIRTF>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENLOWFIRTF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENLOWFIRTF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_CHANGE: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_CLIPFORMAT: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_CORRECTTEXT: u32 = 4194304u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_DRAGDROPDONE: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_DROPFILES: u32 = 1048576u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_ENDCOMPOSITION: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_GROUPTYPINGCHANGE: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_HIDELINKTOOLTIP: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_IMECHANGE: u32 = 8388608u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_KEYEVENTS: u32 = 65536u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_LANGCHANGE: u32 = 16777216u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_LINK: u32 = 67108864u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_LOWFIRTF: u32 = 134217728u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_MOUSEEVENTS: u32 = 131072u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_OBJECTPOSITIONS: u32 = 33554432u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_PAGECHANGE: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_PARAGRAPHEXPANDED: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_PROTECTED: u32 = 2097152u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_REQUESTRESIZE: u32 = 262144u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_SCROLL: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_SCROLLEVENTS: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_SELCHANGE: u32 = 524288u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_STARTCOMPOSITION: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ENM_UPDATE: u32 = 2u32;
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENOLEOPFAILED {
    pub nmhdr: super::NMHDR,
    pub iob: i32,
    pub lOper: i32,
    pub hr: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENOLEOPFAILED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENOLEOPFAILED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENOLEOPFAILED {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENOLEOPFAILED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENOLEOPFAILED>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENOLEOPFAILED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENOLEOPFAILED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENPROTECTED {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENPROTECTED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENPROTECTED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENPROTECTED {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENPROTECTED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENPROTECTED>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENPROTECTED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENPROTECTED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENSAVECLIPBOARD {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub cch: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENSAVECLIPBOARD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENSAVECLIPBOARD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENSAVECLIPBOARD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENSAVECLIPBOARD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENSAVECLIPBOARD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENSAVECLIPBOARD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENSAVECLIPBOARD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_ALIGNLTR: u32 = 1808u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_ALIGNRTL: u32 = 1809u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_CLIPFORMAT: u32 = 1810u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_CORRECTTEXT: u32 = 1797u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_DRAGDROPDONE: u32 = 1804u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_DROPFILES: u32 = 1795u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_ENDCOMPOSITION: u32 = 1812u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_IMECHANGE: u32 = 1799u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_LINK: u32 = 1803u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_LOWFIRTF: u32 = 1807u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_MSGFILTER: u32 = 1792u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_OBJECTPOSITIONS: u32 = 1802u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_OLEOPFAILED: u32 = 1801u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_PAGECHANGE: u32 = 1806u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_PARAGRAPHEXPANDED: u32 = 1805u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_PROTECTED: u32 = 1796u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_REQUESTRESIZE: u32 = 1793u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_SAVECLIPBOARD: u32 = 1800u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_SELCHANGE: u32 = 1794u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_STARTCOMPOSITION: u32 = 1811u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EN_STOPNOUNDO: u32 = 1798u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EPR_0: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EPR_180: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EPR_270: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EPR_90: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const EPR_SE: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ES_DISABLENOSCROLL: u32 = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ES_EX_NOCALLOLEINIT: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ES_NOIME: u32 = 524288u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ES_NOOLEDRAGDROP: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ES_SAVESEL: u32 = 32768u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ES_SELECTIONBAR: u32 = 16777216u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ES_SELFIME: u32 = 262144u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ES_SUNKEN: u32 = 16384u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ES_VERTICAL: u32 = 4194304u32;
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDTEXTA {
    pub chrg: CHARRANGE,
    pub lpstrText: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDTEXTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDTEXTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FINDTEXTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FINDTEXTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FINDTEXTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FINDTEXTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDTEXTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDTEXTEXA {
    pub chrg: CHARRANGE,
    pub lpstrText: super::super::super::Foundation::PSTR,
    pub chrgText: CHARRANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDTEXTEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDTEXTEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FINDTEXTEXA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FINDTEXTEXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FINDTEXTEXA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FINDTEXTEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDTEXTEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDTEXTEXW {
    pub chrg: CHARRANGE,
    pub lpstrText: super::super::super::Foundation::PWSTR,
    pub chrgText: CHARRANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDTEXTEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDTEXTEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FINDTEXTEXW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FINDTEXTEXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FINDTEXTEXW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FINDTEXTEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDTEXTEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDTEXTW {
    pub chrg: CHARRANGE,
    pub lpstrText: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDTEXTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDTEXTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FINDTEXTW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FINDTEXTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FINDTEXTW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FINDTEXTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDTEXTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct FORMATRANGE {
    pub hdc: super::super::super::Graphics::Gdi::HDC,
    pub hdcTarget: super::super::super::Graphics::Gdi::HDC,
    pub rc: super::super::super::Foundation::RECT,
    pub rcPage: super::super::super::Foundation::RECT,
    pub chrg: CHARRANGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for FORMATRANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for FORMATRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for FORMATRANGE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for FORMATRANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FORMATRANGE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for FORMATRANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for FORMATRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const FR_MATCHALEFHAMZA: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const FR_MATCHDIAC: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const FR_MATCHKASHIDA: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GCMF_GRIPPER: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GCMF_MOUSEMENU: u32 = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GCMF_SPELLING: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GCMF_TOUCHMENU: u32 = 16384u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GCM_MOUSEMENU: u32 = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GCM_TOUCHMENU: u32 = 16384u32;
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GETCONTEXTMENUEX {
    pub chrg: CHARRANGE,
    pub dwFlags: u32,
    pub pt: super::super::super::Foundation::POINT,
    pub pvReserved: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GETCONTEXTMENUEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GETCONTEXTMENUEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GETCONTEXTMENUEX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GETCONTEXTMENUEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GETCONTEXTMENUEX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GETCONTEXTMENUEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GETCONTEXTMENUEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GETTEXTEX {
    pub cb: u32,
    pub flags: GETTEXTEX_FLAGS,
    pub codepage: u32,
    pub lpDefaultChar: super::super::super::Foundation::PSTR,
    pub lpUsedDefChar: *mut i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GETTEXTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GETTEXTEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GETTEXTEX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GETTEXTEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GETTEXTEX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GETTEXTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GETTEXTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type GETTEXTEX_FLAGS = u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GT_DEFAULT: GETTEXTEX_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GT_NOHIDDENTEXT: GETTEXTEX_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GT_RAWTEXT: GETTEXTEX_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GT_SELECTION: GETTEXTEX_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GT_USECRLF: GETTEXTEX_FLAGS = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub struct GETTEXTLENGTHEX {
    pub flags: GETTEXTLENGTHEX_FLAGS,
    pub codepage: u32,
}
impl ::core::marker::Copy for GETTEXTLENGTHEX {}
impl ::core::clone::Clone for GETTEXTLENGTHEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GETTEXTLENGTHEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GETTEXTLENGTHEX").field("flags", &self.flags).field("codepage", &self.codepage).finish()
    }
}
unsafe impl ::windows::core::Abi for GETTEXTLENGTHEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GETTEXTLENGTHEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GETTEXTLENGTHEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for GETTEXTLENGTHEX {}
impl ::core::default::Default for GETTEXTLENGTHEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type GETTEXTLENGTHEX_FLAGS = u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GTL_DEFAULT: GETTEXTLENGTHEX_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GTL_USECRLF: GETTEXTLENGTHEX_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GTL_PRECISE: GETTEXTLENGTHEX_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GTL_CLOSE: GETTEXTLENGTHEX_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GTL_NUMCHARS: GETTEXTLENGTHEX_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GTL_NUMBYTES: GETTEXTLENGTHEX_FLAGS = 16u32;
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub struct HYPHENATEINFO {
    pub cbSize: i16,
    pub dxHyphenateZone: i16,
    pub pfnHyphenate: isize,
}
impl ::core::marker::Copy for HYPHENATEINFO {}
impl ::core::clone::Clone for HYPHENATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HYPHENATEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HYPHENATEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HYPHENATEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HYPHENATEINFO {}
impl ::core::default::Default for HYPHENATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ICM_CTF: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ICM_LEVEL2: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ICM_LEVEL2_5: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ICM_LEVEL2_SUI: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ICM_LEVEL3: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ICM_NOTOPEN: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub struct IMECOMPTEXT {
    pub cb: i32,
    pub flags: IMECOMPTEXT_FLAGS,
}
impl ::core::marker::Copy for IMECOMPTEXT {}
impl ::core::clone::Clone for IMECOMPTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMECOMPTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMECOMPTEXT").field("cb", &self.cb).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows::core::Abi for IMECOMPTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMECOMPTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMECOMPTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMECOMPTEXT {}
impl ::core::default::Default for IMECOMPTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type IMECOMPTEXT_FLAGS = u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ICT_RESULTREADSTR: IMECOMPTEXT_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_AUTOFONT: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_AUTOFONTSIZEADJUST: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_AUTOKEYBOARD: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_CLOSESTATUSWINDOW: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_DUALFONT: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_FORCEACTIVE: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_FORCEDISABLE: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_FORCEENABLE: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_FORCEINACTIVE: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_FORCENONE: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_FORCEREMEMBER: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_IMEALWAYSSENDNOTIFY: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_IMECANCELCOMPLETE: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_IMEUIINTEGRATION: u32 = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_MULTIPLEEDIT: u32 = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_NOIMPLICITLANG: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_NOKBDLIDFIXUP: u32 = 512u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_NORTFFONTSUBSTITUTE: u32 = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_SMODE_NONE: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_SMODE_PLAURALCLAUSE: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_SPELLCHECKING: u32 = 2048u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_TKBPREDICTION: u32 = 4096u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_UIFONTS: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const IMF_VERTICAL: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct IRichEditOle(::windows::core::IUnknown);
impl IRichEditOle {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Ole'*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetClientSite(&self) -> ::windows::core::Result<super::super::super::System::Ole::IOleClientSite> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Ole::IOleClientSite>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetObjectCount(&self) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetLinkCount(&self) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com_StructuredStorage', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn GetObject(&self, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(iob), ::core::mem::transmute(lpreobject), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com_StructuredStorage', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn InsertObject(&self, lpreobject: *mut REOBJECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpreobject)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConvertObject<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, iob: i32, rclsidnew: *const ::windows::core::GUID, lpstrusertypenew: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(iob), ::core::mem::transmute(rclsidnew), lpstrusertypenew.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ActivateAs(&self, rclsid: *const ::windows::core::GUID, rclsidas: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rclsidas)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHostNames<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, lpstrcontainerapp: Param0, lpstrcontainerobj: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), lpstrcontainerapp.into_param().abi(), lpstrcontainerobj.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLinkAvailable<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, iob: i32, favailable: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(iob), favailable.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDvaspect(&self, iob: i32, dvaspect: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(iob), ::core::mem::transmute(dvaspect)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn HandsOffStorage(&self, iob: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(iob)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SaveCompleted<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IStorage>>(&self, iob: i32, lpstg: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(iob), lpstg.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn InPlaceDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fentermode: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), fentermode.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::core::option::Option<super::super::super::System::Com::IDataObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpchrg), ::core::mem::transmute(reco), ::core::mem::transmute(lplpdataobj)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportDataObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IDataObject>>(&self, lpdataobj: Param0, cf: u16, hmetapict: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), lpdataobj.into_param().abi(), ::core::mem::transmute(cf), ::core::mem::transmute(hmetapict)).ok()
    }
}
impl ::core::convert::From<IRichEditOle> for ::windows::core::IUnknown {
    fn from(value: IRichEditOle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRichEditOle> for ::windows::core::IUnknown {
    fn from(value: &IRichEditOle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRichEditOle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRichEditOle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRichEditOle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRichEditOle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRichEditOle {}
impl ::core::fmt::Debug for IRichEditOle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRichEditOle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRichEditOle {
    type Vtable = IRichEditOleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020d00_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditOleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lplpolesite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> i32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> i32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpreobject: *mut REOBJECT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iob: i32, rclsidnew: *const ::windows::core::GUID, lpstrusertypenew: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rclsidas: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpstrcontainerapp: super::super::super::Foundation::PSTR, lpstrcontainerobj: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iob: i32, favailable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iob: i32, dvaspect: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iob: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iob: i32, lpstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobj: ::windows::core::RawPtr, cf: u16, hmetapict: isize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct IRichEditOleCallback(::windows::core::IUnknown);
impl IRichEditOleCallback {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetNewStorage(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::IStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::StructuredStorage::IStorage>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Ole', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetInPlaceContext(&self, lplpframe: *mut ::core::option::Option<super::super::super::System::Ole::IOleInPlaceFrame>, lplpdoc: *mut ::core::option::Option<super::super::super::System::Ole::IOleInPlaceUIWindow>, lpframeinfo: *mut super::super::super::System::Ole::OIFI) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lplpframe), ::core::mem::transmute(lplpdoc), ::core::mem::transmute(lpframeinfo)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowContainerUI<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com_StructuredStorage'*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn QueryInsertObject<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::StructuredStorage::IStorage>>(&self, lpclsid: *mut ::windows::core::GUID, lpstg: Param1, cp: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpclsid), lpstg.into_param().abi(), ::core::mem::transmute(cp)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Ole'*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn DeleteObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Ole::IOleObject>>(&self, lpoleobj: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), lpoleobj.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryAcceptData<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IDataObject>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, lpdataobj: Param0, lpcfformat: *mut u16, reco: u32, freally: Param3, hmetapict: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), lpdataobj.into_param().abi(), ::core::mem::transmute(lpcfformat), ::core::mem::transmute(reco), freally.into_param().abi(), ::core::mem::transmute(hmetapict)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fentermode: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), fentermode.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::core::option::Option<super::super::super::System::Com::IDataObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpchrg), ::core::mem::transmute(reco), ::core::mem::transmute(lplpdataobj)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDragDropEffect<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdrag: Param0, grfkeystate: u32, pdweffect: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), fdrag.into_param().abi(), ::core::mem::transmute(grfkeystate), ::core::mem::transmute(pdweffect)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Ole', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetContextMenu<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Ole::IOleObject>>(&self, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: Param1, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(seltype), lpoleobj.into_param().abi(), ::core::mem::transmute(lpchrg), ::core::mem::transmute(lphmenu)).ok()
    }
}
impl ::core::convert::From<IRichEditOleCallback> for ::windows::core::IUnknown {
    fn from(value: IRichEditOleCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRichEditOleCallback> for ::windows::core::IUnknown {
    fn from(value: &IRichEditOleCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRichEditOleCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRichEditOleCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRichEditOleCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRichEditOleCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRichEditOleCallback {}
impl ::core::fmt::Debug for IRichEditOleCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRichEditOleCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRichEditOleCallback {
    type Vtable = IRichEditOleCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020d03_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditOleCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lplpstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lplpframe: *mut ::windows::core::RawPtr, lplpdoc: *mut ::windows::core::RawPtr, lpframeinfo: *mut super::super::super::System::Ole::OIFI) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows::core::GUID, lpstg: ::windows::core::RawPtr, cp: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpoleobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobj: ::windows::core::RawPtr, lpcfformat: *mut u16, reco: u32, freally: super::super::super::Foundation::BOOL, hmetapict: isize) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdrag: super::super::super::Foundation::BOOL, grfkeystate: u32, pdweffect: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: ::windows::core::RawPtr, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct IRicheditUiaOverrides(::windows::core::IUnknown);
impl IRicheditUiaOverrides {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPropertyOverrideValue(&self, propertyid: i32, pretvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertyid), ::core::mem::transmute(pretvalue)).ok()
    }
}
impl ::core::convert::From<IRicheditUiaOverrides> for ::windows::core::IUnknown {
    fn from(value: IRicheditUiaOverrides) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRicheditUiaOverrides> for ::windows::core::IUnknown {
    fn from(value: &IRicheditUiaOverrides) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRicheditUiaOverrides {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRicheditUiaOverrides {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRicheditUiaOverrides {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRicheditUiaOverrides {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRicheditUiaOverrides {}
impl ::core::fmt::Debug for IRicheditUiaOverrides {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRicheditUiaOverrides").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRicheditUiaOverrides {
    type Vtable = IRicheditUiaOverridesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRicheditUiaOverridesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: i32, pretvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextDisplays(::windows::core::IUnknown);
impl ITextDisplays {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextDisplays {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextDisplays {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextDisplays> for ::windows::core::IUnknown {
    fn from(value: ITextDisplays) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextDisplays> for ::windows::core::IUnknown {
    fn from(value: &ITextDisplays) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextDisplays {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextDisplays {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextDisplays {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextDisplays {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextDisplays {}
impl ::core::fmt::Debug for ITextDisplays {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDisplays").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextDisplays {
    type Vtable = ITextDisplaysVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc241f5f2_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDisplaysVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextDocument(::windows::core::IUnknown);
impl ITextDocument {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<ITextSelection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextSelection>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryRanges(&self) -> ::windows::core::Result<ITextStoryRanges> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextStoryRanges>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSaved(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSaved(&self, value: tomConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDefaultTabStop(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDefaultTabStop(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn New(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Open(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(flags), ::core::mem::transmute(codepage)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Save(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(flags), ::core::mem::transmute(codepage)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Freeze(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Unfreeze(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn BeginEditCollection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn EndEditCollection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Undo(&self, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Redo(&self, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Range(&self, cpactive: i32, cpanchor: i32) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpactive), ::core::mem::transmute(cpanchor), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn RangeFromPoint(&self, x: i32, y: i32) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextDocument> for ::windows::core::IUnknown {
    fn from(value: ITextDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextDocument> for ::windows::core::IUnknown {
    fn from(value: &ITextDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextDocument {}
impl ::core::fmt::Debug for ITextDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextDocument {
    type Vtable = ITextDocumentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cc497c0_a1df_11ce_8098_00aa0047be5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocumentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextDocument2(::windows::core::IUnknown);
impl ITextDocument2 {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<ITextSelection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextSelection>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryRanges(&self) -> ::windows::core::Result<ITextStoryRanges> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextStoryRanges>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSaved(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSaved(&self, value: tomConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDefaultTabStop(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDefaultTabStop(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn New(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Open(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(flags), ::core::mem::transmute(codepage)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Save(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(flags), ::core::mem::transmute(codepage)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Freeze(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Unfreeze(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn BeginEditCollection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn EndEditCollection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Undo(&self, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Redo(&self, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Range(&self, cpactive: i32, cpanchor: i32) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpactive), ::core::mem::transmute(cpanchor), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn RangeFromPoint(&self, x: i32, y: i32) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCaretType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCaretType(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDisplays(&self) -> ::windows::core::Result<ITextDisplays> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextDisplays>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDocumentFont(&self) -> ::windows::core::Result<ITextFont2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextFont2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDocumentFont<'a, Param0: ::windows::core::IntoParam<'a, ITextFont2>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDocumentPara(&self) -> ::windows::core::Result<ITextPara2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextPara2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDocumentPara<'a, Param0: ::windows::core::IntoParam<'a, ITextPara2>>(&self, ppara: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEastAsianFlags(&self) -> ::windows::core::Result<tomConstants> {
        let mut result__: tomConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<tomConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGenerator(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetIMEInProgress(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetNotificationMode(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetNotificationMode(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSelection2(&self) -> ::windows::core::Result<ITextSelection2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextSelection2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryRanges2(&self) -> ::windows::core::Result<ITextStoryRanges2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextStoryRanges2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetTypographyOptions(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn AttachMsgFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pfilter: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), pfilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(cch), ::core::mem::transmute(pcch)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCallManager(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetClientRect(&self, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(pleft), ::core::mem::transmute(ptop), ::core::mem::transmute(pright), ::core::mem::transmute(pbottom)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEffectColor(&self, index: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetImmContext(&self) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreferredFont(&self, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(cp), ::core::mem::transmute(charrep), ::core::mem::transmute(options), ::core::mem::transmute(curcharrep), ::core::mem::transmute(curfontsize), ::core::mem::transmute(pbstr), ::core::mem::transmute(ppitchandfamily), ::core::mem::transmute(pnewfontsize)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStrings(&self) -> ::windows::core::Result<ITextStrings> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextStrings>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Notify(&self, notify: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(notify)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Range2(&self, cpactive: i32, cpanchor: i32) -> ::windows::core::Result<ITextRange2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpactive), ::core::mem::transmute(cpanchor), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn RangeFromPoint2(&self, x: i32, y: i32, r#type: i32) -> ::windows::core::Result<ITextRange2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ReleaseCallManager<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pvoid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), pvoid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ReleaseImmContext(&self, context: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(context)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetEffectColor(&self, index: i32, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetTypographyOptions(&self, options: i32, mask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(mask)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SysBeep(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Update(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn UpdateWindow(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetMathProperties(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetMathProperties(&self, options: i32, mask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(mask)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetActiveStory(&self) -> ::windows::core::Result<ITextStory> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextStory>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetActiveStory<'a, Param0: ::windows::core::IntoParam<'a, ITextStory>>(&self, pstory: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), pstory.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetMainStory(&self) -> ::windows::core::Result<ITextStory> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextStory>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetNewStory(&self) -> ::windows::core::Result<ITextStory> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextStory>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStory(&self, index: i32) -> ::windows::core::Result<ITextStory> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<ITextStory>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITextDocument> for ITextDocument2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextDocument> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextDocument> for &ITextDocument2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextDocument> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextDocument2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextDocument2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextDocument2> for ::windows::core::IUnknown {
    fn from(value: ITextDocument2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextDocument2> for ::windows::core::IUnknown {
    fn from(value: &ITextDocument2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextDocument2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextDocument2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextDocument2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextDocument2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextDocument2 {}
impl ::core::fmt::Debug for ITextDocument2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDocument2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextDocument2 {
    type Vtable = ITextDocument2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc241f5e0_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdisplays: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut tomConstants) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poptions: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cch: i32, pcch: *const i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvoid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notify: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvoid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: i32, mask: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poptions: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: i32, mask: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstory: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextDocument2Old(::windows::core::IUnknown);
impl ITextDocument2Old {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<ITextSelection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextSelection>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryRanges(&self) -> ::windows::core::Result<ITextStoryRanges> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextStoryRanges>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSaved(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSaved(&self, value: tomConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDefaultTabStop(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDefaultTabStop(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn New(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Open(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(flags), ::core::mem::transmute(codepage)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Save(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(flags), ::core::mem::transmute(codepage)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Freeze(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Unfreeze(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn BeginEditCollection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn EndEditCollection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Undo(&self, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Redo(&self, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Range(&self, cpactive: i32, cpanchor: i32) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpactive), ::core::mem::transmute(cpanchor), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn RangeFromPoint(&self, x: i32, y: i32) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn AttachMsgFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pfilter: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), pfilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetEffectColor(&self, index: i32, cr: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(cr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEffectColor(&self, index: i32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCaretType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCaretType(&self, carettype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(carettype)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetImmContext(&self) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ReleaseImmContext(&self, context: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(context)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreferredFont(&self, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(cp), ::core::mem::transmute(charrep), ::core::mem::transmute(option), ::core::mem::transmute(charrepcur), ::core::mem::transmute(curfontsize), ::core::mem::transmute(pbstr), ::core::mem::transmute(ppitchandfamily), ::core::mem::transmute(pnewfontsize)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetNotificationMode(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetNotificationMode(&self, mode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetClientRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(pleft), ::core::mem::transmute(ptop), ::core::mem::transmute(pright), ::core::mem::transmute(pbottom)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSelection2(&self) -> ::windows::core::Result<ITextSelection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextSelection>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFEFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn UpdateWindow(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(cch), ::core::mem::transmute(pcch)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn IMEInProgress(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SysBeep(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Update(&self, mode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Notify(&self, notify: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(notify)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDocumentFont(&self) -> ::windows::core::Result<ITextFont> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDocumentPara(&self) -> ::windows::core::Result<ITextPara> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCallManager(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ReleaseCallManager<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pvoid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), pvoid.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITextDocument> for ITextDocument2Old {
    fn into_param(self) -> ::windows::core::Param<'a, ITextDocument> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextDocument> for &ITextDocument2Old {
    fn into_param(self) -> ::windows::core::Param<'a, ITextDocument> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextDocument2Old {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextDocument2Old {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextDocument2Old> for ::windows::core::IUnknown {
    fn from(value: ITextDocument2Old) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextDocument2Old> for ::windows::core::IUnknown {
    fn from(value: &ITextDocument2Old) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextDocument2Old {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextDocument2Old {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextDocument2Old {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextDocument2Old {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextDocument2Old {}
impl ::core::fmt::Debug for ITextDocument2Old {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDocument2Old").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextDocument2Old {
    type Vtable = ITextDocument2OldVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01c25500_4268_11d1_883a_3c8b00c10000);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument2OldVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, cr: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pcr: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcarettype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, carettype: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cch: i32, pcch: *const i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notify: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitextfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitextpara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvoid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvoid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextFont(::windows::core::IUnknown);
impl ITextFont {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::core::Result<ITextFont> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDuplicate<'a, Param0: ::windows::core::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn CanChange(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pfont.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Reset(&self, value: tomConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetAllCaps(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetAllCaps(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetAnimation(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetAnimation(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetBackColor(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetBackColor(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetBold(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetBold(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEmboss(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetEmboss(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetForeColor(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetForeColor(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetHidden(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetHidden(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEngrave(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetEngrave(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetItalic(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetItalic(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetKerning(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetKerning(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetLanguageID(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetLanguageID(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetOutline(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetOutline(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPosition(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPosition(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetProtected(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetProtected(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetShadow(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetShadow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSize(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSmallCaps(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSmallCaps(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSpacing(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSpacing(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStrikeThrough(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetStrikeThrough(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSubscript(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSubscript(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSuperscript(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSuperscript(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetUnderline(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetUnderline(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetWeight(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetWeight(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextFont {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextFont {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextFont> for ::windows::core::IUnknown {
    fn from(value: ITextFont) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextFont> for ::windows::core::IUnknown {
    fn from(value: &ITextFont) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextFont {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextFont {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextFont {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextFont {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextFont {}
impl ::core::fmt::Debug for ITextFont {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextFont").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextFont {
    type Vtable = ITextFontVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cc497c3_a1df_11ce_8098_00aa0047be5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextFontVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextFont2(::windows::core::IUnknown);
impl ITextFont2 {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::core::Result<ITextFont> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDuplicate<'a, Param0: ::windows::core::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn CanChange(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pfont.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Reset(&self, value: tomConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetAllCaps(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetAllCaps(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetAnimation(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetAnimation(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetBackColor(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetBackColor(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetBold(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetBold(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEmboss(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetEmboss(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetForeColor(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetForeColor(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetHidden(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetHidden(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEngrave(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetEngrave(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetItalic(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetItalic(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetKerning(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetKerning(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetLanguageID(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetLanguageID(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetOutline(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetOutline(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPosition(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPosition(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetProtected(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetProtected(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetShadow(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetShadow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSize(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSmallCaps(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSmallCaps(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSpacing(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSpacing(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStrikeThrough(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetStrikeThrough(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSubscript(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSubscript(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSuperscript(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSuperscript(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetUnderline(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetUnderline(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetWeight(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetWeight(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetAutoLigatures(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetAutoLigatures(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetAutospaceAlpha(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetAutospaceAlpha(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetAutospaceNumeric(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetAutospaceNumeric(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetAutospaceParens(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetAutospaceParens(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCharRep(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCharRep(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCompressionMode(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCompressionMode(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCookie(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCookie(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDoubleStrike(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDoubleStrike(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDuplicate2(&self) -> ::windows::core::Result<ITextFont2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).79)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextFont2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDuplicate2<'a, Param0: ::windows::core::IntoParam<'a, ITextFont2>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).80)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetLinkType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).81)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetMathZone(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).82)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetMathZone(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).83)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetModWidthPairs(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).84)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetModWidthPairs(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).85)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetModWidthSpace(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).86)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetModWidthSpace(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).87)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetOldNumbers(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).88)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetOldNumbers(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).89)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetOverlapping(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).90)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetOverlapping(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).91)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPositionSubSuper(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).92)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPositionSubSuper(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).93)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetScaling(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).94)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetScaling(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).95)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSpaceExtension(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).96)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSpaceExtension(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).97)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetUnderlinePositionMode(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).98)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetUnderlinePositionMode(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).99)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).100)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvalue), ::core::mem::transmute(pmask)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEffects2(&self, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).101)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvalue), ::core::mem::transmute(pmask)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).102)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPropertyInfo(&self, index: i32, ptype: *mut i32, pvalue: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).103)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn IsEqual2<'a, Param0: ::windows::core::IntoParam<'a, ITextFont2>>(&self, pfont: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).104)(::core::mem::transmute_copy(self), pfont.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetEffects(&self, value: i32, mask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).105)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(mask)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetEffects2(&self, value: i32, mask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).106)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(mask)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).107)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITextFont> for ITextFont2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextFont> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextFont> for &ITextFont2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextFont> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextFont2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextFont2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextFont2> for ::windows::core::IUnknown {
    fn from(value: ITextFont2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextFont2> for ::windows::core::IUnknown {
    fn from(value: &ITextFont2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextFont2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextFont2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextFont2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextFont2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextFont2 {}
impl ::core::fmt::Debug for ITextFont2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextFont2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextFont2 {
    type Vtable = ITextFont2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc241f5e3_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextFont2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, ptype: *mut i32, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextHost(::windows::core::IUnknown);
impl ITextHost {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Graphics_Gdi'*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxGetDC(&self) -> super::super::super::Graphics::Gdi::HDC {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Graphics_Gdi'*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxReleaseDC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, hdc: Param0) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hdc.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxShowScrollBar<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fnbar: i32, fshow: Param1) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(fnbar), fshow.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxEnableScrollBar(&self, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(fusbflags), ::core::mem::transmute(fuarrowflags)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetScrollRange<'a, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: Param3) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fnbar), ::core::mem::transmute(nminpos), ::core::mem::transmute(nmaxpos), fredraw.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetScrollPos<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fnbar: i32, npos: i32, fredraw: Param2) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(fnbar), ::core::mem::transmute(npos), fredraw.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxInvalidateRect<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, prc: *mut super::super::super::Foundation::RECT, fmode: Param1) {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), fmode.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxViewChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fupdate: Param0) {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), fupdate.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn TxCreateCaret<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HBITMAP>>(&self, hbmp: Param0, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), hbmp.into_param().abi(), ::core::mem::transmute(xwidth), ::core::mem::transmute(yheight)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxShowCaret<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fshow: Param0) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), fshow.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetCaretPos(&self, x: i32, y: i32) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetTimer(&self, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(idtimer), ::core::mem::transmute(utimeout)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxKillTimer(&self, idtimer: u32) {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(idtimer))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxScrollWindowEx<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HRGN>>(&self, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: Param4, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD) {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dx), ::core::mem::transmute(dy), ::core::mem::transmute(lprcscroll), ::core::mem::transmute(lprcclip), hrgnupdate.into_param().abi(), ::core::mem::transmute(lprcupdate), ::core::mem::transmute(fuscroll))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetCapture<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fcapture: Param0) {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), fcapture.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxSetFocus(&self) {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxSetCursor<'a, Param0: ::windows::core::IntoParam<'a, super::super::WindowsAndMessaging::HCURSOR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, hcur: Param0, ftext: Param1) {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), hcur.into_param().abi(), ftext.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxScreenToClient(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppt)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxClientToScreen(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppt)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxActivate(&self, ploldstate: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ploldstate)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxDeactivate(&self, lnewstate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnewstate)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetClientRect(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetViewInset(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetCharFormat(&self, ppcf: *const *const CHARFORMATW) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppcf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetParaFormat(&self, pppf: *const *const PARAFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetSysColor(&self, nindex: i32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetBackStyle(&self, pstyle: *mut TXTBACKSTYLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstyle)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetMaxLength(&self, plength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(plength)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetScrollBars(&self, pdwscrollbar: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwscrollbar)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetPasswordChar(&self) -> ::windows::core::Result<i8> {
        let mut result__: i8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i8>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetAcceleratorPos(&self, pcp: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcp)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetExtent(&self, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpextent)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn OnTxCharFormatChange(&self, pcf: *const CHARFORMATW) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn OnTxParaFormatChange(&self, ppf: *const PARAFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetPropertyBits(&self, dwmask: u32, pdwbits: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmask), ::core::mem::transmute(pdwbits)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxNotify(&self, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(inotify), ::core::mem::transmute(pv)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn TxImmGetContext(&self) -> super::super::super::Globalization::HIMC {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn TxImmReleaseContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), himc.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetSelectionBarWidth(&self, lselbarwidth: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(lselbarwidth)).ok()
    }
}
impl ::core::convert::From<ITextHost> for ::windows::core::IUnknown {
    fn from(value: ITextHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextHost> for ::windows::core::IUnknown {
    fn from(value: &ITextHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextHost {}
impl ::core::fmt::Debug for ITextHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextHost {
    type Vtable = ITextHostVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHostVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Graphics::Gdi::HDC,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: super::super::super::Graphics::Gdi::HDC) -> i32,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fnbar: i32, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fnbar: i32, npos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT, fmode: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fupdate: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idtimer: u32),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcapture: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: super::super::super::Foundation::BOOL),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploldstate: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnewstate: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcf: *const *const CHARFORMATW) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppf: *const *const PARAFORMAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstyle: *mut TXTBACKSTYLE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwscrollbar: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pch: *mut i8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcp: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcf: *const CHARFORMATW) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppf: *const PARAFORMAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmask: u32, pdwbits: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Globalization::HIMC,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC),
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lselbarwidth: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextHost2(::windows::core::IUnknown);
impl ITextHost2 {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Graphics_Gdi'*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxGetDC(&self) -> super::super::super::Graphics::Gdi::HDC {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Graphics_Gdi'*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxReleaseDC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, hdc: Param0) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hdc.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxShowScrollBar<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fnbar: i32, fshow: Param1) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(fnbar), fshow.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxEnableScrollBar(&self, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(fusbflags), ::core::mem::transmute(fuarrowflags)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetScrollRange<'a, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: Param3) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fnbar), ::core::mem::transmute(nminpos), ::core::mem::transmute(nmaxpos), fredraw.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetScrollPos<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fnbar: i32, npos: i32, fredraw: Param2) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(fnbar), ::core::mem::transmute(npos), fredraw.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxInvalidateRect<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, prc: *mut super::super::super::Foundation::RECT, fmode: Param1) {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc), fmode.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxViewChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fupdate: Param0) {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), fupdate.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn TxCreateCaret<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HBITMAP>>(&self, hbmp: Param0, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), hbmp.into_param().abi(), ::core::mem::transmute(xwidth), ::core::mem::transmute(yheight)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxShowCaret<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fshow: Param0) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), fshow.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetCaretPos(&self, x: i32, y: i32) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetTimer(&self, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(idtimer), ::core::mem::transmute(utimeout)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxKillTimer(&self, idtimer: u32) {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(idtimer))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxScrollWindowEx<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HRGN>>(&self, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: Param4, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD) {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dx), ::core::mem::transmute(dy), ::core::mem::transmute(lprcscroll), ::core::mem::transmute(lprcclip), hrgnupdate.into_param().abi(), ::core::mem::transmute(lprcupdate), ::core::mem::transmute(fuscroll))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetCapture<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fcapture: Param0) {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), fcapture.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxSetFocus(&self) {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxSetCursor<'a, Param0: ::windows::core::IntoParam<'a, super::super::WindowsAndMessaging::HCURSOR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, hcur: Param0, ftext: Param1) {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), hcur.into_param().abi(), ftext.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxScreenToClient(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppt)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxClientToScreen(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(lppt)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxActivate(&self, ploldstate: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ploldstate)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxDeactivate(&self, lnewstate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnewstate)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetClientRect(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetViewInset(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(prc)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetCharFormat(&self, ppcf: *const *const CHARFORMATW) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppcf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetParaFormat(&self, pppf: *const *const PARAFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetSysColor(&self, nindex: i32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetBackStyle(&self, pstyle: *mut TXTBACKSTYLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstyle)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetMaxLength(&self, plength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(plength)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetScrollBars(&self, pdwscrollbar: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwscrollbar)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetPasswordChar(&self) -> ::windows::core::Result<i8> {
        let mut result__: i8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i8>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetAcceleratorPos(&self, pcp: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcp)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetExtent(&self, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpextent)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn OnTxCharFormatChange(&self, pcf: *const CHARFORMATW) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn OnTxParaFormatChange(&self, ppf: *const PARAFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetPropertyBits(&self, dwmask: u32, pdwbits: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmask), ::core::mem::transmute(pdwbits)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxNotify(&self, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(inotify), ::core::mem::transmute(pv)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn TxImmGetContext(&self) -> super::super::super::Globalization::HIMC {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn TxImmReleaseContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), himc.into_param().abi())
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetSelectionBarWidth(&self, lselbarwidth: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(lselbarwidth)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxIsDoubleClickPending(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetWindow(&self, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(phwnd)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxSetForegroundWindow(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Graphics_Gdi'*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxGetPalette(&self) -> super::super::super::Graphics::Gdi::HPALETTE {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetEastAsianFlags(&self, pflags: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(pflags)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxSetCursor2<'a, Param0: ::windows::core::IntoParam<'a, super::super::WindowsAndMessaging::HCURSOR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, hcur: Param0, btext: Param1) -> super::super::WindowsAndMessaging::HCURSOR {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), hcur.into_param().abi(), btext.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxFreeTextServicesNotification(&self) {
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetEditStyle(&self, dwitem: u32, pdwdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwitem), ::core::mem::transmute(pdwdata)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetWindowStyles(&self, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwstyle), ::core::mem::transmute(pdwexstyle)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn TxShowDropCaret<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, fshow: Param0, hdc: Param1, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), fshow.into_param().abi(), hdc.into_param().abi(), ::core::mem::transmute(prc)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxDestroyCaret(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetHorzExtent(&self, plhorzextent: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(plhorzextent)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITextHost> for ITextHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextHost> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextHost> for &ITextHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextHost> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextHost2> for ::windows::core::IUnknown {
    fn from(value: ITextHost2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextHost2> for ::windows::core::IUnknown {
    fn from(value: &ITextHost2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextHost2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextHost2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextHost2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextHost2 {}
impl ::core::fmt::Debug for ITextHost2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextHost2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextHost2 {
    type Vtable = ITextHost2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHost2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Graphics::Gdi::HDC,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: super::super::super::Graphics::Gdi::HDC) -> i32,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fnbar: i32, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fnbar: i32, npos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT, fmode: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fupdate: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idtimer: u32),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcapture: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: super::super::super::Foundation::BOOL),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploldstate: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnewstate: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcf: *const *const CHARFORMATW) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppf: *const *const PARAFORMAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstyle: *mut TXTBACKSTYLE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwscrollbar: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pch: *mut i8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcp: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcf: *const CHARFORMATW) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppf: *const PARAFORMAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmask: u32, pdwbits: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Globalization::HIMC,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC),
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lselbarwidth: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Graphics::Gdi::HPALETTE,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: super::super::super::Foundation::BOOL) -> super::super::WindowsAndMessaging::HCURSOR,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwitem: u32, pdwdata: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhorzextent: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextPara(::windows::core::IUnknown);
impl ITextPara {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::core::Result<ITextPara> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDuplicate<'a, Param0: ::windows::core::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn CanChange(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ppara.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Reset(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetAlignment(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetAlignment(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetHyphenation(&self) -> ::windows::core::Result<tomConstants> {
        let mut result__: tomConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<tomConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetHyphenation(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFirstLineIndent(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetKeepTogether(&self) -> ::windows::core::Result<tomConstants> {
        let mut result__: tomConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<tomConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetKeepTogether(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetKeepWithNext(&self) -> ::windows::core::Result<tomConstants> {
        let mut result__: tomConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<tomConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetLeftIndent(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetLineSpacing(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetLineSpacingRule(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetListAlignment(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetListAlignment(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetListLevelIndex(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetListLevelIndex(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetListStart(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetListStart(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetListTab(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetListTab(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetListType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetListType(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetNoLineNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetNoLineNumber(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPageBreakBefore(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPageBreakBefore(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetRightIndent(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetRightIndent(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetIndents(&self, first: f32, left: f32, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(first), ::core::mem::transmute(left), ::core::mem::transmute(right)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetLineSpacing(&self, rule: i32, spacing: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(rule), ::core::mem::transmute(spacing)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSpaceAfter(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSpaceAfter(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSpaceBefore(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSpaceBefore(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetWidowControl(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetWidowControl(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetTabCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn AddTab(&self, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(tbpos), ::core::mem::transmute(tbalign), ::core::mem::transmute(tbleader)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ClearAllTabs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn DeleteTab(&self, tbpos: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(tbpos)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetTab(&self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(itab), ::core::mem::transmute(ptbpos), ::core::mem::transmute(ptbalign), ::core::mem::transmute(ptbleader)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextPara {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextPara {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextPara> for ::windows::core::IUnknown {
    fn from(value: ITextPara) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextPara> for ::windows::core::IUnknown {
    fn from(value: &ITextPara) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextPara {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextPara {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextPara {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextPara {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextPara {}
impl ::core::fmt::Debug for ITextPara {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextPara").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextPara {
    type Vtable = ITextParaVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cc497c4_a1df_11ce_8098_00aa0047be5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextParaVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, first: f32, left: f32, right: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rule: i32, spacing: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tbpos: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextPara2(::windows::core::IUnknown);
impl ITextPara2 {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::core::Result<ITextPara> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDuplicate<'a, Param0: ::windows::core::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn CanChange(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ppara.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Reset(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetAlignment(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetAlignment(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetHyphenation(&self) -> ::windows::core::Result<tomConstants> {
        let mut result__: tomConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<tomConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetHyphenation(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFirstLineIndent(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetKeepTogether(&self) -> ::windows::core::Result<tomConstants> {
        let mut result__: tomConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<tomConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetKeepTogether(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetKeepWithNext(&self) -> ::windows::core::Result<tomConstants> {
        let mut result__: tomConstants = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<tomConstants>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetLeftIndent(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetLineSpacing(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetLineSpacingRule(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetListAlignment(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetListAlignment(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetListLevelIndex(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetListLevelIndex(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetListStart(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetListStart(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetListTab(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetListTab(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetListType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetListType(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetNoLineNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetNoLineNumber(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPageBreakBefore(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPageBreakBefore(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetRightIndent(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetRightIndent(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetIndents(&self, first: f32, left: f32, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(first), ::core::mem::transmute(left), ::core::mem::transmute(right)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetLineSpacing(&self, rule: i32, spacing: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(rule), ::core::mem::transmute(spacing)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSpaceAfter(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSpaceAfter(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSpaceBefore(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSpaceBefore(&self, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetWidowControl(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetWidowControl(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetTabCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn AddTab(&self, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(tbpos), ::core::mem::transmute(tbalign), ::core::mem::transmute(tbleader)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ClearAllTabs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn DeleteTab(&self, tbpos: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(tbpos)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetTab(&self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(itab), ::core::mem::transmute(ptbpos), ::core::mem::transmute(ptbalign), ::core::mem::transmute(ptbleader)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetBorders(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDuplicate2(&self) -> ::windows::core::Result<ITextPara2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextPara2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDuplicate2<'a, Param0: ::windows::core::IntoParam<'a, ITextPara2>>(&self, ppara: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFontAlignment(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFontAlignment(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetHangingPunctuation(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetHangingPunctuation(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSnapToGrid(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetSnapToGrid(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetTrimPunctuationAtStart(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetTrimPunctuationAtStart(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvalue), ::core::mem::transmute(pmask)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn IsEqual2<'a, Param0: ::windows::core::IntoParam<'a, ITextPara2>>(&self, ppara: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ppara.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetEffects(&self, value: i32, mask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(mask)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITextPara> for ITextPara2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextPara> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextPara> for &ITextPara2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextPara> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextPara2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextPara2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextPara2> for ::windows::core::IUnknown {
    fn from(value: ITextPara2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextPara2> for ::windows::core::IUnknown {
    fn from(value: &ITextPara2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextPara2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextPara2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextPara2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextPara2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextPara2 {}
impl ::core::fmt::Debug for ITextPara2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextPara2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextPara2 {
    type Vtable = ITextPara2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc241f5e4_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPara2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, first: f32, left: f32, right: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rule: i32, spacing: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tbpos: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppborders: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextRange(::windows::core::IUnknown);
impl ITextRange {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetText<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetChar(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetChar(&self, char: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(char)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFormattedText(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFormattedText<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStart(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpfirst)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEnd(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(cplim)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFont(&self) -> ::windows::core::Result<ITextFont> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFont<'a, Param0: ::windows::core::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPara(&self) -> ::windows::core::Result<ITextPara> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPara<'a, Param0: ::windows::core::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryLength(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstart)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Expand(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(index), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpanchor), ::core::mem::transmute(cpactive)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn InRange<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn InStory<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindText<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindTextStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindTextEnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Cut(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Copy(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn CanEdit(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ChangeCase(&self, r#type: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPoint(&self, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(px), ::core::mem::transmute(py)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(r#type), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextRange {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextRange {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextRange> for ::windows::core::IUnknown {
    fn from(value: ITextRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextRange> for ::windows::core::IUnknown {
    fn from(value: &ITextRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextRange {}
impl ::core::fmt::Debug for ITextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextRange {
    type Vtable = ITextRangeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cc497c2_a1df_11ce_8098_00aa0047be5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRangeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchar: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, char: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcpfirst: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpfirst: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcplim: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cplim: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstart: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, pindex: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, index: i32, extend: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32, pvalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextRange2(::windows::core::IUnknown);
impl ITextRange2 {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetText<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetChar(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetChar(&self, char: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(char)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFormattedText(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFormattedText<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStart(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpfirst)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEnd(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(cplim)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFont(&self) -> ::windows::core::Result<ITextFont> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFont<'a, Param0: ::windows::core::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPara(&self) -> ::windows::core::Result<ITextPara> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPara<'a, Param0: ::windows::core::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryLength(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstart)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Expand(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(index), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpanchor), ::core::mem::transmute(cpactive)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn InRange<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn InStory<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindText<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindTextStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindTextEnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Cut(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Copy(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn CanEdit(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ChangeCase(&self, r#type: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPoint(&self, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(px), ::core::mem::transmute(py)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(r#type), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn HomeKey(&self, unit: tomConstants, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn EndKey(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TypeText<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCch(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCells(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetColumn(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDuplicate2(&self) -> ::windows::core::Result<ITextRange2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFont2(&self) -> ::windows::core::Result<ITextFont2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextFont2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFont2<'a, Param0: ::windows::core::IntoParam<'a, ITextFont2>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFormattedText2(&self) -> ::windows::core::Result<ITextRange2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFormattedText2<'a, Param0: ::windows::core::IntoParam<'a, ITextRange2>>(&self, prange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetGravity(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetGravity(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPara2(&self) -> ::windows::core::Result<ITextPara2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).79)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextPara2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPara2<'a, Param0: ::windows::core::IntoParam<'a, ITextPara2>>(&self, ppara: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).80)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetRow(&self) -> ::windows::core::Result<ITextRow> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).81)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRow>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStartPara(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).82)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetTable(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).83)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).84)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetURL<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).85)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn AddSubrange(&self, cp1: i32, cp2: i32, activate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).86)(::core::mem::transmute_copy(self), ::core::mem::transmute(cp1), ::core::mem::transmute(cp2), ::core::mem::transmute(activate)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn BuildUpMath(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).87)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn DeleteSubrange(&self, cpfirst: i32, cplim: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).88)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpfirst), ::core::mem::transmute(cplim)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Find<'a, Param0: ::windows::core::IntoParam<'a, ITextRange2>>(&self, prange: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).89)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetChar2(&self, pchar: *mut i32, offset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).90)(::core::mem::transmute_copy(self), ::core::mem::transmute(pchar), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDropCap(&self, pcline: *mut i32, pposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).91)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcline), ::core::mem::transmute(pposition)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetInlineObject(&self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).92)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype), ::core::mem::transmute(palign), ::core::mem::transmute(pchar), ::core::mem::transmute(pchar1), ::core::mem::transmute(pchar2), ::core::mem::transmute(pcount), ::core::mem::transmute(ptexstyle), ::core::mem::transmute(pccol), ::core::mem::transmute(plevel)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).93)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).94)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(pleft), ::core::mem::transmute(ptop), ::core::mem::transmute(pright), ::core::mem::transmute(pbottom), ::core::mem::transmute(phit)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSubrange(&self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).95)(::core::mem::transmute_copy(self), ::core::mem::transmute(isubrange), ::core::mem::transmute(pcpfirst), ::core::mem::transmute(pcplim)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText2(&self, flags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).96)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn HexToUnicode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).97)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn InsertTable(&self, ccol: i32, crow: i32, autofit: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).98)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccol), ::core::mem::transmute(crow), ::core::mem::transmute(autofit)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Linearize(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).99)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetActiveSubrange(&self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).100)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpanchor), ::core::mem::transmute(cpactive)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDropCap(&self, cline: i32, position: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).101)(::core::mem::transmute_copy(self), ::core::mem::transmute(cline), ::core::mem::transmute(position)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).102)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetText2<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, flags: i32, bstr: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).103)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn UnicodeToHex(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).104)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetInlineObject(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).105)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(align), ::core::mem::transmute(char), ::core::mem::transmute(char1), ::core::mem::transmute(char2), ::core::mem::transmute(count), ::core::mem::transmute(texstyle), ::core::mem::transmute(ccol)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMathFunctionType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).106)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn InsertImage<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: Param4, pstream: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).107)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(ascent), ::core::mem::transmute(r#type), bstralttext.into_param().abi(), pstream.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITextSelection> for ITextRange2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextSelection> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextSelection> for &ITextRange2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextSelection> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ITextRange> for ITextRange2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRange> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextRange> for &ITextRange2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRange> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextRange2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextRange2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextRange2> for ::windows::core::IUnknown {
    fn from(value: ITextRange2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextRange2> for ::windows::core::IUnknown {
    fn from(value: &ITextRange2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextRange2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextRange2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextRange2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextRange2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextRange2 {}
impl ::core::fmt::Debug for ITextRange2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRange2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextRange2 {
    type Vtable = ITextRange2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc241f5e2_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRange2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchar: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, char: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcpfirst: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpfirst: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcplim: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cplim: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstart: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, pindex: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, index: i32, extend: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32, pvalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: tomConstants, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcch: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcells: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolumn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cp1: i32, cp2: i32, activate: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpfirst: i32, cplim: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, count: i32, flags: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchar: *mut i32, offset: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcline: *mut i32, pposition: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccol: i32, crow: i32, autofit: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cline: i32, position: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextRow(::windows::core::IUnknown);
impl ITextRow {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetAlignment(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetAlignment(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCellCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCellCount(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCellCountCache(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCellCountCache(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCellIndex(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCellIndex(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCellMargin(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCellMargin(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetHeight(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetHeight(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetIndent(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetIndent(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetKeepTogether(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetKeepTogether(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetKeepWithNext(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetNestLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetRTL(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetRTL(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCellAlignment(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCellAlignment(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCellColorBack(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCellColorBack(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCellColorFore(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCellColorFore(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCellMergeFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCellMergeFlags(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCellShading(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCellShading(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCellVerticalText(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCellVerticalText(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCellWidth(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCellWidth(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCellBorderColors(&self, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcrleft), ::core::mem::transmute(pcrtop), ::core::mem::transmute(pcrright), ::core::mem::transmute(pcrbottom)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCellBorderWidths(&self, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(pduleft), ::core::mem::transmute(pdutop), ::core::mem::transmute(pduright), ::core::mem::transmute(pdubottom)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCellBorderColors(&self, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(crleft), ::core::mem::transmute(crtop), ::core::mem::transmute(crright), ::core::mem::transmute(crbottom)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetCellBorderWidths(&self, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(duleft), ::core::mem::transmute(dutop), ::core::mem::transmute(duright), ::core::mem::transmute(dubottom)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Apply(&self, crow: i32, flags: tomConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(crow), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn CanChange(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Insert(&self, crow: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(crow)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextRow>>(&self, prow: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), prow.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Reset(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextRow {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextRow {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextRow> for ::windows::core::IUnknown {
    fn from(value: ITextRow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextRow> for ::windows::core::IUnknown {
    fn from(value: &ITextRow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextRow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextRow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextRow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextRow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextRow {}
impl ::core::fmt::Debug for ITextRow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRow").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextRow {
    type Vtable = ITextRowVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc241f5ef_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRowVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crow: i32, flags: tomConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crow: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prow: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextSelection(::windows::core::IUnknown);
impl ITextSelection {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetText<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetChar(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetChar(&self, char: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(char)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFormattedText(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFormattedText<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStart(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpfirst)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEnd(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(cplim)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFont(&self) -> ::windows::core::Result<ITextFont> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFont<'a, Param0: ::windows::core::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPara(&self) -> ::windows::core::Result<ITextPara> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPara<'a, Param0: ::windows::core::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryLength(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstart)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Expand(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(index), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpanchor), ::core::mem::transmute(cpactive)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn InRange<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn InStory<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindText<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindTextStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindTextEnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Cut(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Copy(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn CanEdit(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ChangeCase(&self, r#type: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPoint(&self, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(px), ::core::mem::transmute(py)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(r#type), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn HomeKey(&self, unit: tomConstants, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn EndKey(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TypeText<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITextRange> for ITextSelection {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRange> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextRange> for &ITextSelection {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRange> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextSelection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextSelection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextSelection> for ::windows::core::IUnknown {
    fn from(value: ITextSelection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextSelection> for ::windows::core::IUnknown {
    fn from(value: &ITextSelection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextSelection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextSelection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextSelection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextSelection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextSelection {}
impl ::core::fmt::Debug for ITextSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextSelection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextSelection {
    type Vtable = ITextSelectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cc497c1_a1df_11ce_8098_00aa0047be5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchar: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, char: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcpfirst: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpfirst: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcplim: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cplim: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstart: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, pindex: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, index: i32, extend: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32, pvalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: tomConstants, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextSelection2(::windows::core::IUnknown);
impl ITextSelection2 {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetText<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetChar(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetChar(&self, char: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(char)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDuplicate(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFormattedText(&self) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFormattedText<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStart(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpfirst)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEnd(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(cplim)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFont(&self) -> ::windows::core::Result<ITextFont> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextFont>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFont<'a, Param0: ::windows::core::IntoParam<'a, ITextFont>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPara(&self) -> ::windows::core::Result<ITextPara> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextPara>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPara<'a, Param0: ::windows::core::IntoParam<'a, ITextPara>>(&self, ppara: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryLength(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStoryType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstart)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Expand(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(index), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpanchor), ::core::mem::transmute(cpactive)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn InRange<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn InStory<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, prange: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(cset), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindText<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindTextStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindTextEnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Cut(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Copy(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvar), ::core::mem::transmute(format), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn CanEdit(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ChangeCase(&self, r#type: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPoint(&self, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(px), ::core::mem::transmute(py)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(r#type), ::core::mem::transmute(extend)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(count), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn HomeKey(&self, unit: tomConstants, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn EndKey(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(unit), ::core::mem::transmute(extend), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TypeText<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCch(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCells(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetColumn(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDuplicate2(&self) -> ::windows::core::Result<ITextRange2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFont2(&self) -> ::windows::core::Result<ITextFont2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextFont2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFont2<'a, Param0: ::windows::core::IntoParam<'a, ITextFont2>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetFormattedText2(&self) -> ::windows::core::Result<ITextRange2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFormattedText2<'a, Param0: ::windows::core::IntoParam<'a, ITextRange2>>(&self, prange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetGravity(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetGravity(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetPara2(&self) -> ::windows::core::Result<ITextPara2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).79)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextPara2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetPara2<'a, Param0: ::windows::core::IntoParam<'a, ITextPara2>>(&self, ppara: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).80)(::core::mem::transmute_copy(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetRow(&self) -> ::windows::core::Result<ITextRow> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).81)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITextRow>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetStartPara(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).82)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetTable(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).83)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).84)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetURL<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).85)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn AddSubrange(&self, cp1: i32, cp2: i32, activate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).86)(::core::mem::transmute_copy(self), ::core::mem::transmute(cp1), ::core::mem::transmute(cp2), ::core::mem::transmute(activate)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn BuildUpMath(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).87)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn DeleteSubrange(&self, cpfirst: i32, cplim: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).88)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpfirst), ::core::mem::transmute(cplim)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Find<'a, Param0: ::windows::core::IntoParam<'a, ITextRange2>>(&self, prange: Param0, count: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).89)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetChar2(&self, pchar: *mut i32, offset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).90)(::core::mem::transmute_copy(self), ::core::mem::transmute(pchar), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDropCap(&self, pcline: *mut i32, pposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).91)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcline), ::core::mem::transmute(pposition)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetInlineObject(&self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).92)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype), ::core::mem::transmute(palign), ::core::mem::transmute(pchar), ::core::mem::transmute(pchar1), ::core::mem::transmute(pchar2), ::core::mem::transmute(pcount), ::core::mem::transmute(ptexstyle), ::core::mem::transmute(pccol), ::core::mem::transmute(plevel)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).93)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).94)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(pleft), ::core::mem::transmute(ptop), ::core::mem::transmute(pright), ::core::mem::transmute(pbottom), ::core::mem::transmute(phit)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetSubrange(&self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).95)(::core::mem::transmute_copy(self), ::core::mem::transmute(isubrange), ::core::mem::transmute(pcpfirst), ::core::mem::transmute(pcplim)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText2(&self, flags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).96)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn HexToUnicode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).97)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn InsertTable(&self, ccol: i32, crow: i32, autofit: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).98)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccol), ::core::mem::transmute(crow), ::core::mem::transmute(autofit)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Linearize(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).99)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetActiveSubrange(&self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).100)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpanchor), ::core::mem::transmute(cpactive)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetDropCap(&self, cline: i32, position: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).101)(::core::mem::transmute_copy(self), ::core::mem::transmute(cline), ::core::mem::transmute(position)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).102)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetText2<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, flags: i32, bstr: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).103)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn UnicodeToHex(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).104)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetInlineObject(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).105)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(align), ::core::mem::transmute(char), ::core::mem::transmute(char1), ::core::mem::transmute(char2), ::core::mem::transmute(count), ::core::mem::transmute(texstyle), ::core::mem::transmute(ccol)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMathFunctionType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).106)(::core::mem::transmute_copy(self), bstr.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn InsertImage<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: Param4, pstream: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).107)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(ascent), ::core::mem::transmute(r#type), bstralttext.into_param().abi(), pstream.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITextRange2> for ITextSelection2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRange2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextRange2> for &ITextSelection2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRange2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ITextSelection> for ITextSelection2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextSelection> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextSelection> for &ITextSelection2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextSelection> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ITextRange> for ITextSelection2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRange> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextRange> for &ITextSelection2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRange> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextSelection2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextSelection2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextSelection2> for ::windows::core::IUnknown {
    fn from(value: ITextSelection2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextSelection2> for ::windows::core::IUnknown {
    fn from(value: &ITextSelection2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextSelection2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextSelection2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextSelection2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextSelection2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextSelection2 {}
impl ::core::fmt::Debug for ITextSelection2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextSelection2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextSelection2 {
    type Vtable = ITextSelection2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc241f5e1_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelection2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchar: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, char: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcpfirst: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpfirst: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcplim: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cplim: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstart: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, pindex: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, index: i32, extend: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32, pvalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: tomConstants, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcch: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcells: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolumn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cp1: i32, cp2: i32, activate: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpfirst: i32, cplim: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, count: i32, flags: i32, pdelta: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchar: *mut i32, offset: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcline: *mut i32, pposition: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccol: i32, crow: i32, autofit: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cline: i32, position: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextServices(::windows::core::IUnknown);
impl ITextServices {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSendMessage<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, msg: u32, wparam: Param1, lparam: Param2, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(plresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxDraw<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param5: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: Param4, hictargetdev: Param5, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdrawaspect), ::core::mem::transmute(lindex), ::core::mem::transmute(pvaspect), ::core::mem::transmute(ptd), hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ::core::mem::transmute(lprcbounds), ::core::mem::transmute(lprcwbounds), ::core::mem::transmute(lprcupdate), ::core::mem::transmute(pfncontinue), ::core::mem::transmute(dwcontinue), ::core::mem::transmute(lviewid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetHScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(plmin), ::core::mem::transmute(plmax), ::core::mem::transmute(plpos), ::core::mem::transmute(plpage), ::core::mem::transmute(pfenabled)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetVScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(plmin), ::core::mem::transmute(plmax), ::core::mem::transmute(plpos), ::core::mem::transmute(plpage), ::core::mem::transmute(pfenabled)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn OnTxSetCursor<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param5: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: Param4, hictargetdev: Param5, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdrawaspect), ::core::mem::transmute(lindex), ::core::mem::transmute(pvaspect), ::core::mem::transmute(ptd), hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ::core::mem::transmute(lprcclient), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxQueryHitPoint<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param5: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: Param4, hictargetdev: Param5, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdrawaspect), ::core::mem::transmute(lindex), ::core::mem::transmute(pvaspect), ::core::mem::transmute(ptd), hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ::core::mem::transmute(lprcclient), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(phitresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTxInPlaceActivate(&self, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcclient)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn OnTxInPlaceDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn OnTxUIActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn OnTxUIDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetText(&self, pbstrtext: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrtext)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetText<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, psztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), psztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetCurTargetX(&self, param0: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetBaseLinePos(&self, param0: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxGetNaturalSize<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param2: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, dwaspect: u32, hdcdraw: Param1, hictargetdev: Param2, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ::core::mem::transmute(ptd), ::core::mem::transmute(dwmode), ::core::mem::transmute(psizelextent), ::core::mem::transmute(pwidth), ::core::mem::transmute(pheight)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Ole'*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn TxGetDropTarget(&self) -> ::windows::core::Result<super::super::super::System::Ole::IDropTarget> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Ole::IDropTarget>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn OnTxPropertyBitsChange(&self, dwmask: u32, dwbits: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmask), ::core::mem::transmute(dwbits)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetCachedSize(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwwidth), ::core::mem::transmute(pdwheight)).ok()
    }
}
impl ::core::convert::From<ITextServices> for ::windows::core::IUnknown {
    fn from(value: ITextServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextServices> for ::windows::core::IUnknown {
    fn from(value: &ITextServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextServices {}
impl ::core::fmt::Debug for ITextServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextServices {
    type Vtable = ITextServicesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextServicesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdroptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmask: u32, dwbits: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextServices2(::windows::core::IUnknown);
impl ITextServices2 {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSendMessage<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, msg: u32, wparam: Param1, lparam: Param2, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(plresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxDraw<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param5: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: Param4, hictargetdev: Param5, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdrawaspect), ::core::mem::transmute(lindex), ::core::mem::transmute(pvaspect), ::core::mem::transmute(ptd), hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ::core::mem::transmute(lprcbounds), ::core::mem::transmute(lprcwbounds), ::core::mem::transmute(lprcupdate), ::core::mem::transmute(pfncontinue), ::core::mem::transmute(dwcontinue), ::core::mem::transmute(lviewid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetHScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(plmin), ::core::mem::transmute(plmax), ::core::mem::transmute(plpos), ::core::mem::transmute(plpage), ::core::mem::transmute(pfenabled)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetVScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(plmin), ::core::mem::transmute(plmax), ::core::mem::transmute(plpos), ::core::mem::transmute(plpage), ::core::mem::transmute(pfenabled)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn OnTxSetCursor<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param5: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: Param4, hictargetdev: Param5, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdrawaspect), ::core::mem::transmute(lindex), ::core::mem::transmute(pvaspect), ::core::mem::transmute(ptd), hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ::core::mem::transmute(lprcclient), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxQueryHitPoint<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param5: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: Param4, hictargetdev: Param5, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdrawaspect), ::core::mem::transmute(lindex), ::core::mem::transmute(pvaspect), ::core::mem::transmute(ptd), hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ::core::mem::transmute(lprcclient), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(phitresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTxInPlaceActivate(&self, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcclient)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn OnTxInPlaceDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn OnTxUIActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn OnTxUIDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetText(&self, pbstrtext: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrtext)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetText<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, psztext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), psztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetCurTargetX(&self, param0: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetBaseLinePos(&self, param0: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxGetNaturalSize<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param2: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, dwaspect: u32, hdcdraw: Param1, hictargetdev: Param2, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ::core::mem::transmute(ptd), ::core::mem::transmute(dwmode), ::core::mem::transmute(psizelextent), ::core::mem::transmute(pwidth), ::core::mem::transmute(pheight)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Ole'*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn TxGetDropTarget(&self) -> ::windows::core::Result<super::super::super::System::Ole::IDropTarget> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Ole::IDropTarget>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn OnTxPropertyBitsChange(&self, dwmask: u32, dwbits: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmask), ::core::mem::transmute(dwbits)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn TxGetCachedSize(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwwidth), ::core::mem::transmute(pdwheight)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxGetNaturalSize2<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>, Param2: ::windows::core::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>>(&self, dwaspect: u32, hdcdraw: Param1, hictargetdev: Param2, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ::core::mem::transmute(ptd), ::core::mem::transmute(dwmode), ::core::mem::transmute(psizelextent), ::core::mem::transmute(pwidth), ::core::mem::transmute(pheight), ::core::mem::transmute(pascent)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Direct2D'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))]
    pub unsafe fn TxDrawD2D<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct2D::ID2D1RenderTarget>>(&self, prendertarget: Param0, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), prendertarget.into_param().abi(), ::core::mem::transmute(lprcbounds), ::core::mem::transmute(lprcupdate), ::core::mem::transmute(lviewid)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITextServices> for ITextServices2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextServices> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextServices> for &ITextServices2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextServices> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextServices2> for ::windows::core::IUnknown {
    fn from(value: ITextServices2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextServices2> for ::windows::core::IUnknown {
    fn from(value: &ITextServices2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextServices2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextServices2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextServices2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextServices2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextServices2 {}
impl ::core::fmt::Debug for ITextServices2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextServices2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextServices2 {
    type Vtable = ITextServices2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextServices2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdroptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmask: u32, dwbits: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prendertarget: ::windows::core::RawPtr, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D")))] usize,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextStory(::windows::core::IUnknown);
impl ITextStory {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetActive(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetActive(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetDisplay(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetType(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetRange(&self, cpactive: i32, cpanchor: i32) -> ::windows::core::Result<ITextRange2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpactive), ::core::mem::transmute(cpanchor), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self, flags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFormattedText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetText<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, flags: i32, bstr: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), bstr.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITextStory> for ::windows::core::IUnknown {
    fn from(value: ITextStory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStory> for ::windows::core::IUnknown {
    fn from(value: &ITextStory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStory {}
impl ::core::fmt::Debug for ITextStory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStory {
    type Vtable = ITextStoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc241f5f3_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdisplay: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextStoryRanges(::windows::core::IUnknown);
impl ITextStoryRanges {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextStoryRanges {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextStoryRanges {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextStoryRanges> for ::windows::core::IUnknown {
    fn from(value: ITextStoryRanges) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoryRanges> for ::windows::core::IUnknown {
    fn from(value: &ITextStoryRanges) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStoryRanges {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStoryRanges {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStoryRanges {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStoryRanges {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoryRanges {}
impl ::core::fmt::Debug for ITextStoryRanges {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoryRanges").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStoryRanges {
    type Vtable = ITextStoryRangesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cc497c5_a1df_11ce_8098_00aa0047be5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoryRangesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextStoryRanges2(::windows::core::IUnknown);
impl ITextStoryRanges2 {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ITextRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Item2(&self, index: i32) -> ::windows::core::Result<ITextRange2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange2>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITextStoryRanges> for ITextStoryRanges2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextStoryRanges> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextStoryRanges> for &ITextStoryRanges2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextStoryRanges> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextStoryRanges2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextStoryRanges2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextStoryRanges2> for ::windows::core::IUnknown {
    fn from(value: ITextStoryRanges2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoryRanges2> for ::windows::core::IUnknown {
    fn from(value: &ITextStoryRanges2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStoryRanges2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStoryRanges2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStoryRanges2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStoryRanges2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoryRanges2 {}
impl ::core::fmt::Debug for ITextStoryRanges2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoryRanges2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStoryRanges2 {
    type Vtable = ITextStoryRanges2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc241f5e5_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoryRanges2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
#[repr(transparent)]
pub struct ITextStrings(::windows::core::IUnknown);
impl ITextStrings {
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ITextRange2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<ITextRange2>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Append<'a, Param0: ::windows::core::IntoParam<'a, ITextRange2>>(&self, prange: Param0, istring: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(istring)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Cat2(&self, istring: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(istring)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CatTop2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn DeleteRange<'a, Param0: ::windows::core::IntoParam<'a, ITextRange2>>(&self, prange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn EncodeFunction<'a, Param8: ::windows::core::IntoParam<'a, ITextRange2>>(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: Param8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(align), ::core::mem::transmute(char), ::core::mem::transmute(char1), ::core::mem::transmute(char2), ::core::mem::transmute(count), ::core::mem::transmute(texstyle), ::core::mem::transmute(ccol), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn GetCch(&self, istring: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(istring), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn InsertNullStr(&self, istring: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(istring)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn MoveBoundary(&self, istring: i32, cch: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(istring), ::core::mem::transmute(cch)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PrefixTop<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Remove(&self, istring: i32, cstring: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(istring), ::core::mem::transmute(cstring)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetFormattedText<'a, Param0: ::windows::core::IntoParam<'a, ITextRange2>, Param1: ::windows::core::IntoParam<'a, ITextRange2>>(&self, pranged: Param0, pranges: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), pranged.into_param().abi(), pranges.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn SetOpCp(&self, istring: i32, cp: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(istring), ::core::mem::transmute(cp)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuffixTop<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, ITextRange2>>(&self, bstr: Param0, prange: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), bstr.into_param().abi(), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
    pub unsafe fn Swap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for ITextStrings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IDispatch> for &ITextStrings {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextStrings> for ::windows::core::IUnknown {
    fn from(value: ITextStrings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStrings> for ::windows::core::IUnknown {
    fn from(value: &ITextStrings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStrings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStrings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStrings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStrings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStrings {}
impl ::core::fmt::Debug for ITextStrings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStrings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStrings {
    type Vtable = ITextStringsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc241f5e7_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStringsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, istring: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istring: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istring: i32, pcch: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istring: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istring: i32, cch: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istring: i32, cstring: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pranged: ::windows::core::RawPtr, pranges: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istring: i32, cp: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type KHYPH = i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const khyphNil: KHYPH = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const khyphNormal: KHYPH = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const khyphAddBefore: KHYPH = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const khyphChangeBefore: KHYPH = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const khyphDeleteBefore: KHYPH = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const khyphChangeAfter: KHYPH = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const khyphDelAndChange: KHYPH = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type MANCODE = i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MBOLD: MANCODE = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MITAL: MANCODE = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MGREEK: MANCODE = 64i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MROMN: MANCODE = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MSCRP: MANCODE = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MFRAK: MANCODE = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MOPEN: MANCODE = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MSANS: MANCODE = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MMONO: MANCODE = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MMATH: MANCODE = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MISOL: MANCODE = 7i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MINIT: MANCODE = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MTAIL: MANCODE = 9i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MSTRCH: MANCODE = 10i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MLOOP: MANCODE = 11i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MOPENA: MANCODE = 12i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MAX_TABLE_CELLS: u32 = 63u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const MAX_TAB_STOPS: u32 = 32u32;
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSGFILTER {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSGFILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSGFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MSGFILTER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSGFILTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MSGFILTER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSGFILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSGFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OBJECTPOSITIONS {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub pcpPositions: *mut i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OBJECTPOSITIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OBJECTPOSITIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OBJECTPOSITIONS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OBJECTPOSITIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OBJECTPOSITIONS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OBJECTPOSITIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OBJECTPOSITIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type OBJECTTYPE = i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSimpleText: OBJECTTYPE = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRuby: OBJECTTYPE = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomHorzVert: OBJECTTYPE = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomWarichu: OBJECTTYPE = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEq: OBJECTTYPE = 9i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMath: OBJECTTYPE = 10i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAccent: OBJECTTYPE = 10i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBox: OBJECTTYPE = 11i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBoxedFormula: OBJECTTYPE = 12i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBrackets: OBJECTTYPE = 13i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBracketsWithSeps: OBJECTTYPE = 14i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEquationArray: OBJECTTYPE = 15i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFraction: OBJECTTYPE = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFunctionApply: OBJECTTYPE = 17i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLeftSubSup: OBJECTTYPE = 18i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLowerLimit: OBJECTTYPE = 19i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMatrix: OBJECTTYPE = 20i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNary: OBJECTTYPE = 21i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomOpChar: OBJECTTYPE = 22i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomOverbar: OBJECTTYPE = 23i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPhantom: OBJECTTYPE = 24i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRadical: OBJECTTYPE = 25i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSlashedFraction: OBJECTTYPE = 26i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStack: OBJECTTYPE = 27i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStretchStack: OBJECTTYPE = 28i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSubscript: OBJECTTYPE = 29i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSubSup: OBJECTTYPE = 30i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSuperscript: OBJECTTYPE = 31i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUnderbar: OBJECTTYPE = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUpperLimit: OBJECTTYPE = 33i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomObjectMax: OBJECTTYPE = 33i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const OLEOP_DOVERB: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
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
impl ::core::marker::Copy for PARAFORMAT {}
impl ::core::clone::Clone for PARAFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PARAFORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PARAFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PARAFORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for PARAFORMAT {}
impl ::core::default::Default for PARAFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub union PARAFORMAT_0 {
    pub wReserved: u16,
    pub wEffects: u16,
}
impl ::core::marker::Copy for PARAFORMAT_0 {}
impl ::core::clone::Clone for PARAFORMAT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PARAFORMAT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PARAFORMAT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PARAFORMAT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PARAFORMAT_0 {}
impl ::core::default::Default for PARAFORMAT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
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
impl ::core::marker::Copy for PARAFORMAT2 {}
impl ::core::clone::Clone for PARAFORMAT2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PARAFORMAT2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PARAFORMAT2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PARAFORMAT2>()) == 0 }
    }
}
impl ::core::cmp::Eq for PARAFORMAT2 {}
impl ::core::default::Default for PARAFORMAT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type PARAFORMAT_ALIGNMENT = u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFA_CENTER: PARAFORMAT_ALIGNMENT = 3u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFA_LEFT: PARAFORMAT_ALIGNMENT = 1u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFA_RIGHT: PARAFORMAT_ALIGNMENT = 2u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type PARAFORMAT_BORDERS = u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_BORDERS_LEFT: PARAFORMAT_BORDERS = 1u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_BORDERS_RIGHT: PARAFORMAT_BORDERS = 2u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_BORDERS_TOP: PARAFORMAT_BORDERS = 4u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_BORDERS_BOTTOM: PARAFORMAT_BORDERS = 8u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_BORDERS_INSIDE: PARAFORMAT_BORDERS = 16u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_BORDERS_OUTSIDE: PARAFORMAT_BORDERS = 32u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_BORDERS_AUTOCOLOR: PARAFORMAT_BORDERS = 64u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type PARAFORMAT_MASK = u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_ALIGNMENT: PARAFORMAT_MASK = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_NUMBERING: PARAFORMAT_MASK = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_OFFSET: PARAFORMAT_MASK = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_OFFSETINDENT: PARAFORMAT_MASK = 2147483648u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_RIGHTINDENT: PARAFORMAT_MASK = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_RTLPARA: PARAFORMAT_MASK = 65536u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_STARTINDENT: PARAFORMAT_MASK = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_TABSTOPS: PARAFORMAT_MASK = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type PARAFORMAT_NUMBERING_STYLE = u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFNS_PAREN: PARAFORMAT_NUMBERING_STYLE = 0u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFNS_PARENS: PARAFORMAT_NUMBERING_STYLE = 256u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFNS_PERIOD: PARAFORMAT_NUMBERING_STYLE = 512u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFNS_PLAIN: PARAFORMAT_NUMBERING_STYLE = 768u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFNS_NONUMBER: PARAFORMAT_NUMBERING_STYLE = 1024u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFNS_NEWNUMBER: PARAFORMAT_NUMBERING_STYLE = 32768u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type PARAFORMAT_SHADING_STYLE = u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_SHADING_STYLE_NONE: PARAFORMAT_SHADING_STYLE = 0u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_HORIZ: PARAFORMAT_SHADING_STYLE = 1u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_VERT: PARAFORMAT_SHADING_STYLE = 2u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_DOWN_DIAG: PARAFORMAT_SHADING_STYLE = 3u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_UP_DIAG: PARAFORMAT_SHADING_STYLE = 4u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_GRID: PARAFORMAT_SHADING_STYLE = 5u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_TRELLIS: PARAFORMAT_SHADING_STYLE = 6u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_HORZ: PARAFORMAT_SHADING_STYLE = 7u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_VERT: PARAFORMAT_SHADING_STYLE = 8u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_DOWN_DIAG: PARAFORMAT_SHADING_STYLE = 9u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_UP_DIAG: PARAFORMAT_SHADING_STYLE = 10u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_GRID: PARAFORMAT_SHADING_STYLE = 11u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_TRELLIS: PARAFORMAT_SHADING_STYLE = 12u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PC_DELIMITER: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PC_FOLLOWING: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PC_LEADING: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PC_OVERFLOW: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type PCreateTextServices = ::core::option::Option<unsafe extern "system" fn(punkouter: ::core::option::Option<::windows::core::IUnknown>, pitexthost: ::core::option::Option<ITextHost>, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFA_FULL_GLYPHS: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFA_FULL_INTERLETTER: u32 = 6u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFA_FULL_INTERWORD: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFA_FULL_NEWSPAPER: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFA_FULL_SCALED: u32 = 7u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFA_JUSTIFY: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_BORDER: u32 = 2048u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_BOX: u32 = 67108864u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_COLLAPSED: u32 = 16777216u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_DONOTHYPHEN: u32 = 4194304u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_KEEP: u32 = 131072u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_KEEPNEXT: u32 = 262144u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_LINESPACING: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_NOLINENUMBER: u32 = 1048576u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_NOWIDOWCONTROL: u32 = 2097152u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_NUMBERINGSTART: u32 = 32768u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_NUMBERINGSTYLE: u32 = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_NUMBERINGTAB: u32 = 16384u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_OUTLINELEVEL: u32 = 33554432u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_PAGEBREAKBEFORE: u32 = 524288u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_RESERVED2: u32 = 134217728u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_SHADING: u32 = 4096u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_SIDEBYSIDE: u32 = 8388608u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_SPACEAFTER: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_SPACEBEFORE: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_STYLE: u32 = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_TABLE: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_TABLEROWDELIMITER: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFM_TEXTWRAPPINGBREAK: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFN_ARABIC: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFN_BULLET: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFN_LCLETTER: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFN_LCROMAN: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFN_UCLETTER: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const PFN_UCROMAN: u32 = 6u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type PShutdownTextServices = ::core::option::Option<unsafe extern "system" fn(ptextservices: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::HRESULT>;
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PUNCTUATION {
    pub iSize: u32,
    pub szPunctuation: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PUNCTUATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PUNCTUATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PUNCTUATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PUNCTUATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PUNCTUATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PUNCTUATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PUNCTUATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const RECO_COPY: i32 = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const RECO_CUT: i32 = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const RECO_DRAG: i32 = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const RECO_DROP: i32 = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const RECO_PASTE: i32 = 0i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_System_Com_StructuredStorage', 'Win32_System_Ole'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub struct REOBJECT {
    pub cbStruct: u32,
    pub cp: i32,
    pub clsid: ::windows::core::GUID,
    pub poleobj: ::core::option::Option<super::super::super::System::Ole::IOleObject>,
    pub pstg: ::core::option::Option<super::super::super::System::Com::StructuredStorage::IStorage>,
    pub polesite: ::core::option::Option<super::super::super::System::Ole::IOleClientSite>,
    pub sizel: super::super::super::Foundation::SIZE,
    pub dvaspect: u32,
    pub dwFlags: REOBJECT_FLAGS,
    pub dwUser: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for REOBJECT {
    fn clone(&self) -> Self {
        Self {
            cbStruct: self.cbStruct,
            cp: self.cp,
            clsid: self.clsid,
            poleobj: self.poleobj.clone(),
            pstg: self.pstg.clone(),
            polesite: self.polesite.clone(),
            sizel: self.sizel,
            dvaspect: self.dvaspect,
            dwFlags: self.dwFlags,
            dwUser: self.dwUser,
        }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for REOBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REOBJECT").field("cbStruct", &self.cbStruct).field("cp", &self.cp).field("clsid", &self.clsid).field("poleobj", &self.poleobj).field("pstg", &self.pstg).field("polesite", &self.polesite).field("sizel", &self.sizel).field("dvaspect", &self.dvaspect).field("dwFlags", &self.dwFlags).field("dwUser", &self.dwUser).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for REOBJECT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for REOBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type REOBJECT_FLAGS = u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_ALIGNTORIGHT: REOBJECT_FLAGS = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_BELOWBASELINE: REOBJECT_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_BLANK: REOBJECT_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_CANROTATE: REOBJECT_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_DONTNEEDPALETTE: REOBJECT_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_DYNAMICSIZE: REOBJECT_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_GETMETAFILE: REOBJECT_FLAGS = 4194304u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_HILITED: REOBJECT_FLAGS = 16777216u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_INPLACEACTIVE: REOBJECT_FLAGS = 33554432u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_INVERTEDSELECT: REOBJECT_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_LINK: REOBJECT_FLAGS = 2147483648u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_LINKAVAILABLE: REOBJECT_FLAGS = 8388608u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_OPEN: REOBJECT_FLAGS = 67108864u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_OWNERDRAWSELECT: REOBJECT_FLAGS = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_RESIZABLE: REOBJECT_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_SELECTED: REOBJECT_FLAGS = 134217728u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_STATIC: REOBJECT_FLAGS = 1073741824u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_USEASBACKGROUND: REOBJECT_FLAGS = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_WRAPTEXTAROUND: REOBJECT_FLAGS = 512u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_NULL: i32 = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_READWRITEMASK: i32 = 2047i32;
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_System_Com'*"]
#[cfg(feature = "Win32_System_Com")]
pub struct REPASTESPECIAL {
    pub dwAspect: super::super::super::System::Com::DVASPECT,
    pub dwParam: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for REPASTESPECIAL {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for REPASTESPECIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for REPASTESPECIAL {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for REPASTESPECIAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPASTESPECIAL>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for REPASTESPECIAL {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for REPASTESPECIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REQRESIZE {
    pub nmhdr: super::NMHDR,
    pub rc: super::super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REQRESIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REQRESIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REQRESIZE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REQRESIZE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REQRESIZE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REQRESIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REQRESIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation', 'Win32_Graphics_Gdi', 'Win32_System_Com'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub struct RICHEDIT_IMAGE_PARAMETERS {
    pub xWidth: i32,
    pub yHeight: i32,
    pub Ascent: i32,
    pub Type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS,
    pub pwszAlternateText: super::super::super::Foundation::PWSTR,
    pub pIStream: ::core::option::Option<super::super::super::System::Com::IStream>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for RICHEDIT_IMAGE_PARAMETERS {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for RICHEDIT_IMAGE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RICHEDIT_IMAGE_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for RICHEDIT_IMAGE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::default::Default for RICHEDIT_IMAGE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SEL_EMPTY: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = 0u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SEL_TEXT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = 1u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SEL_OBJECT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = 2u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SEL_MULTICHAR: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = 4u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SEL_MULTIOBJECT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = 8u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const GCM_RIGHTMOUSEDROP: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = 32768u16;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type RICH_EDIT_GET_OBJECT_FLAGS = u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_GETOBJ_POLEOBJ: RICH_EDIT_GET_OBJECT_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_GETOBJ_PSTG: RICH_EDIT_GET_OBJECT_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_GETOBJ_POLESITE: RICH_EDIT_GET_OBJECT_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_GETOBJ_NO_INTERFACES: RICH_EDIT_GET_OBJECT_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const REO_GETOBJ_ALL_INTERFACES: RICH_EDIT_GET_OBJECT_FLAGS = 7u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const RTO_DISABLEHANDLES: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const RTO_READINGMODE: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const RTO_SHOWHANDLES: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SCF_ALL: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SCF_ASSOCIATEFONT: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SCF_ASSOCIATEFONT2: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SCF_CHARREPFROMLCID: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SCF_DEFAULT: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SCF_NOKBUPDATE: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SCF_SELECTION: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SCF_SMARTFONT: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SCF_USEUIRULES: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SCF_WORD: u32 = 2u32;
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SELCHANGE {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SELCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SELCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SELCHANGE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SELCHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SELCHANGE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SELCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SELCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_ALLOWBEEPS: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_BEEPONMAXTEXT: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_BIDI: u32 = 4096u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_CTFALLOWEMBED: u32 = 2097152u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_CTFALLOWPROOFING: u32 = 8388608u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_CTFALLOWSMARTTAG: u32 = 4194304u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_CTFNOLOCK: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_CUSTOMLOOK: u32 = 524288u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_DEFAULTLATINLIGA: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_DRAFTMODE: u32 = 32768u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_EMULATE10: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_EMULATESYSEDIT: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_EXTENDBACKCOLOR: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_EX_HANDLEFRIENDLYURL: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_EX_HIDETEMPFORMAT: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_EX_MULTITOUCH: u32 = 134217728u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_EX_NOACETATESELECTION: u32 = 1048576u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_EX_NOMATH: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_EX_NOTABLE: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_EX_NOTHEMING: u32 = 524288u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_EX_USEMOUSEWPARAM: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_EX_USESINGLELINE: u32 = 2097152u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_HIDEGRIDLINES: u32 = 131072u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_HYPERLINKTOOLTIPS: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_LBSCROLLNOTIFY: u32 = 1048576u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_LOGICALCARET: u32 = 16777216u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_LOWERCASE: u32 = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_MAPCPS: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_MAX: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_MULTISELECT: u32 = 134217728u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_NOEALINEHEIGHTADJUST: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_NOFOCUSLINKNOTIFY: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_NOIME: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_NOINPUTSEQUENCECHK: u32 = 2048u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_SCROLLONKILLFOCUS: u32 = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_SMARTDRAGDROP: u32 = 67108864u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_UPPERCASE: u32 = 512u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_USEAIMM: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_USEATFONT: u32 = 262144u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_USECRLF: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_USECTF: u32 = 65536u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_WORDDRAGDROP: u32 = 33554432u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SES_XLTCRCRLFTOCR: u32 = 16384u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub struct SETTEXTEX {
    pub flags: u32,
    pub codepage: u32,
}
impl ::core::marker::Copy for SETTEXTEX {}
impl ::core::clone::Clone for SETTEXTEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SETTEXTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SETTEXTEX").field("flags", &self.flags).field("codepage", &self.codepage).finish()
    }
}
unsafe impl ::windows::core::Abi for SETTEXTEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SETTEXTEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SETTEXTEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for SETTEXTEX {}
impl ::core::default::Default for SETTEXTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SFF_KEEPDOCINFO: u32 = 4096u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SFF_PERSISTVIEWSCALE: u32 = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SFF_PLAINRTF: u32 = 16384u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SFF_PWD: u32 = 2048u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SFF_SELECTION: u32 = 32768u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SFF_WRITEXTRAPAR: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SF_NCRFORNONASCII: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SF_RTF: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SF_RTFNOOBJS: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SF_RTFVAL: u32 = 1792u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SF_TEXT: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SF_TEXTIZED: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SF_UNICODE: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SF_USECODEPAGE: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SPF_DONTSETDEFAULT: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const SPF_SETDEFAULT: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ST_DEFAULT: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ST_KEEPUNDO: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ST_NEWCHARS: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ST_SELECTION: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const ST_UNICODE: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const S_MSG_KEY_IGNORED: ::windows::core::HRESULT = ::windows::core::HRESULT(262657i32);
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
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
impl ::core::marker::Copy for TABLECELLPARMS {}
impl ::core::clone::Clone for TABLECELLPARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TABLECELLPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TABLECELLPARMS")
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
unsafe impl ::windows::core::Abi for TABLECELLPARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TABLECELLPARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TABLECELLPARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for TABLECELLPARMS {}
impl ::core::default::Default for TABLECELLPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
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
impl ::core::marker::Copy for TABLEROWPARMS {}
impl ::core::clone::Clone for TABLEROWPARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TABLEROWPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TABLEROWPARMS").field("cbRow", &self.cbRow).field("cbCell", &self.cbCell).field("cCell", &self.cCell).field("cRow", &self.cRow).field("dxCellMargin", &self.dxCellMargin).field("dxIndent", &self.dxIndent).field("dyHeight", &self.dyHeight).field("_bitfield", &self._bitfield).field("cpStartRow", &self.cpStartRow).field("bTableLevel", &self.bTableLevel).field("iCell", &self.iCell).finish()
    }
}
unsafe impl ::windows::core::Abi for TABLEROWPARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TABLEROWPARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TABLEROWPARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for TABLEROWPARMS {}
impl ::core::default::Default for TABLEROWPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type TEXTMODE = i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TM_PLAINTEXT: TEXTMODE = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TM_RICHTEXT: TEXTMODE = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TM_SINGLELEVELUNDO: TEXTMODE = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TM_MULTILEVELUNDO: TEXTMODE = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TM_SINGLECODEPAGE: TEXTMODE = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TM_MULTICODEPAGE: TEXTMODE = 32i32;
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TEXTRANGEA {
    pub chrg: CHARRANGE,
    pub lpstrText: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TEXTRANGEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TEXTRANGEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TEXTRANGEA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TEXTRANGEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TEXTRANGEA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TEXTRANGEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TEXTRANGEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TEXTRANGEW {
    pub chrg: CHARRANGE,
    pub lpstrText: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TEXTRANGEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TEXTRANGEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TEXTRANGEW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TEXTRANGEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TEXTRANGEW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TEXTRANGEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TEXTRANGEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TO_ADVANCEDLAYOUT: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TO_ADVANCEDTYPOGRAPHY: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TO_DISABLECUSTOMTEXTOUT: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TO_SIMPLELINEBREAK: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXES_ISDIALOG: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type TXTBACKSTYLE = i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBACK_TRANSPARENT: TXTBACKSTYLE = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBACK_OPAQUE: TXTBACKSTYLE = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_ADVANCEDINPUT: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_ALLOWBEEP: u32 = 2048u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_AUTOWORDSEL: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_BACKSTYLECHANGE: u32 = 16384u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_CHARFORMATCHANGE: u32 = 131072u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_CLIENTRECTCHANGE: u32 = 1048576u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_D2DDWRITE: u32 = 16777216u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_D2DPIXELSNAPPED: u32 = 67108864u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_D2DSIMPLETYPOGRAPHY: u32 = 33554432u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_D2DSUBPIXELLINES: u32 = 134217728u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_DISABLEDRAG: u32 = 4096u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_EXTENTCHANGE: u32 = 524288u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_FLASHLASTPASSWORDCHAR: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_HIDESELECTION: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_MAXLENGTHCHANGE: u32 = 32768u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_MULTILINE: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_NOTHREADREFCOUNT: u32 = 4194304u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_PARAFORMATCHANGE: u32 = 262144u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_READONLY: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_RICHTEXT: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_SAVESELECTION: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_SCROLLBARCHANGE: u32 = 65536u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_SELBARCHANGE: u32 = 512u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_SHOWACCELERATOR: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_SHOWPASSWORD: u32 = 8388608u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_USECURRENTBKG: u32 = 2097152u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_USEPASSWORD: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_VERTICAL: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_VIEWINSETCHANGE: u32 = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTBIT_WORDWRAP: u32 = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type TXTHITRESULT = i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTHITRESULT_NOHIT: TXTHITRESULT = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTHITRESULT_TRANSPARENT: TXTHITRESULT = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTHITRESULT_CLOSE: TXTHITRESULT = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTHITRESULT_HIT: TXTHITRESULT = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type TXTNATURALSIZE = i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTNS_FITTOCONTENT2: TXTNATURALSIZE = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTNS_FITTOCONTENT: TXTNATURALSIZE = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTNS_ROUNDTOLINE: TXTNATURALSIZE = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTNS_FITTOCONTENT3: TXTNATURALSIZE = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTNS_FITTOCONTENTWSP: TXTNATURALSIZE = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTNS_INCLUDELASTLINE: TXTNATURALSIZE = 1073741824i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTNS_EMU: TXTNATURALSIZE = -2147483648i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type TXTVIEW = i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTVIEW_ACTIVE: TXTVIEW = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const TXTVIEW_INACTIVE: TXTVIEW = -1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type UNDONAMEID = i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const UID_UNKNOWN: UNDONAMEID = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const UID_TYPING: UNDONAMEID = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const UID_DELETE: UNDONAMEID = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const UID_DRAGDROP: UNDONAMEID = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const UID_CUT: UNDONAMEID = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const UID_PASTE: UNDONAMEID = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const UID_AUTOTABLE: UNDONAMEID = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const VM_NORMAL: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const VM_OUTLINE: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const VM_PAGE: u32 = 9u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WBF_CUSTOM: u32 = 512u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WBF_LEVEL1: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WBF_LEVEL2: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WBF_OVERFLOW: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WBF_WORDBREAK: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WBF_WORDWRAP: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WB_MOVEWORDNEXT: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WB_MOVEWORDPREV: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WB_NEXTBREAK: u32 = 7u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WB_PREVBREAK: u32 = 6u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WM_CONTEXTMENU: u32 = 123u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WM_NOTIFY: u32 = 78u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WM_PRINTCLIENT: u32 = 792u32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const WM_UNICHAR: u32 = 265u32;
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct _grouptypingchange {
    pub nmhdr: super::NMHDR,
    pub fGroupTyping: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _grouptypingchange {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _grouptypingchange {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for _grouptypingchange {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for _grouptypingchange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_grouptypingchange>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for _grouptypingchange {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _grouptypingchange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub struct hyphresult {
    pub khyph: KHYPH,
    pub ichHyph: i32,
    pub chHyph: u16,
}
impl ::core::marker::Copy for hyphresult {}
impl ::core::clone::Clone for hyphresult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for hyphresult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("hyphresult").field("khyph", &self.khyph).field("ichHyph", &self.ichHyph).field("chHyph", &self.chHyph).finish()
    }
}
unsafe impl ::windows::core::Abi for hyphresult {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for hyphresult {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<hyphresult>()) == 0 }
    }
}
impl ::core::cmp::Eq for hyphresult {}
impl ::core::default::Default for hyphresult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub type tomConstants = i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFalse: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTrue: tomConstants = -1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUndefined: tomConstants = -9999999i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomToggle: tomConstants = -9999998i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAutoColor: tomConstants = -9999997i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDefault: tomConstants = -9999996i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSuspend: tomConstants = -9999995i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomResume: tomConstants = -9999994i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomApplyNow: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomApplyLater: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTrackParms: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCacheParms: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomApplyTmp: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDisableSmartFont: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEnableSmartFont: tomConstants = 9i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUsePoints: tomConstants = 10i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUseTwips: tomConstants = 11i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBackward: tomConstants = -1073741823i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomForward: tomConstants = 1073741823i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMove: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomExtend: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNoSelection: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelectionIP: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelectionNormal: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelectionFrame: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelectionColumn: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelectionRow: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelectionBlock: tomConstants = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelectionInlineShape: tomConstants = 7i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelectionShape: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelStartActive: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelAtEOL: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelOvertype: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelActive: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelReplace: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEnd: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStart: tomConstants = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCollapseEnd: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCollapseStart: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomClientCoord: tomConstants = 256i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAllowOffClient: tomConstants = 512i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTransform: tomConstants = 1024i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomObjectArg: tomConstants = 2048i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAtEnd: tomConstants = 4096i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNone: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSingle: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomWords: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDouble: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDotted: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDash: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDashDot: tomConstants = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDashDotDot: tomConstants = 7i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomWave: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomThick: tomConstants = 9i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomHair: tomConstants = 10i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDoubleWave: tomConstants = 11i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomHeavyWave: tomConstants = 12i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLongDash: tomConstants = 13i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomThickDash: tomConstants = 14i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomThickDashDot: tomConstants = 15i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomThickDashDotDot: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomThickDotted: tomConstants = 17i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomThickLongDash: tomConstants = 18i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLineSpaceSingle: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLineSpace1pt5: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLineSpaceDouble: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLineSpaceAtLeast: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLineSpaceExactly: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLineSpaceMultiple: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLineSpacePercent: tomConstants = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAlignLeft: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAlignCenter: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAlignRight: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAlignJustify: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAlignDecimal: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAlignBar: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDefaultTab: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAlignInterWord: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAlignNewspaper: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAlignInterLetter: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAlignScaled: tomConstants = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSpaces: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDots: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDashes: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLines: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomThickLines: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEquals: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTabBack: tomConstants = -3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTabNext: tomConstants = -2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTabHere: tomConstants = -1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNone: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListBullet: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberAsArabic: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberAsLCLetter: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberAsUCLetter: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberAsLCRoman: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberAsUCRoman: tomConstants = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberAsSequence: tomConstants = 7i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedCircle: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedBlackCircleWingding: tomConstants = 9i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedWhiteCircleWingding: tomConstants = 10i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedArabicWide: tomConstants = 11i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedChS: tomConstants = 12i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedChT: tomConstants = 13i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedJpnChS: tomConstants = 14i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedJpnKor: tomConstants = 15i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedArabic1: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedArabic2: tomConstants = 17i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedHebrew: tomConstants = 18i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedThaiAlpha: tomConstants = 19i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedThaiNum: tomConstants = 20i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedHindiAlpha: tomConstants = 21i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedHindiAlpha1: tomConstants = 22i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNumberedHindiNum: tomConstants = 23i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListParentheses: tomConstants = 65536i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListPeriod: tomConstants = 131072i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListPlain: tomConstants = 196608i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListNoNumber: tomConstants = 262144i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomListMinus: tomConstants = 524288i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomIgnoreNumberStyle: tomConstants = 16777216i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaStyleNormal: tomConstants = -1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaStyleHeading1: tomConstants = -2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaStyleHeading2: tomConstants = -3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaStyleHeading3: tomConstants = -4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaStyleHeading4: tomConstants = -5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaStyleHeading5: tomConstants = -6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaStyleHeading6: tomConstants = -7i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaStyleHeading7: tomConstants = -8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaStyleHeading8: tomConstants = -9i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaStyleHeading9: tomConstants = -10i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCharacter: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomWord: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSentence: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParagraph: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLine: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStory: tomConstants = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomScreen: tomConstants = 7i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSection: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTableColumn: tomConstants = 9i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomColumn: tomConstants = 9i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRow: tomConstants = 10i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomWindow: tomConstants = 11i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCell: tomConstants = 12i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCharFormat: tomConstants = 13i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaFormat: tomConstants = 14i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTable: tomConstants = 15i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomObject: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPage: tomConstants = 17i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomHardParagraph: tomConstants = 18i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCluster: tomConstants = 19i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomInlineObject: tomConstants = 20i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomInlineObjectArg: tomConstants = 21i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLeafLine: tomConstants = 22i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLayoutColumn: tomConstants = 23i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomProcessId: tomConstants = 1073741825i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMatchWord: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMatchCase: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMatchPattern: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUnknownStory: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMainTextStory: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFootnotesStory: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEndnotesStory: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCommentsStory: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTextFrameStory: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEvenPagesHeaderStory: tomConstants = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPrimaryHeaderStory: tomConstants = 7i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEvenPagesFooterStory: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPrimaryFooterStory: tomConstants = 9i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFirstPageHeaderStory: tomConstants = 10i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFirstPageFooterStory: tomConstants = 11i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomScratchStory: tomConstants = 127i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFindStory: tomConstants = 128i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomReplaceStory: tomConstants = 129i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStoryInactive: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStoryActiveDisplay: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStoryActiveUI: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStoryActiveDisplayUI: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNoAnimation: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLasVegasLights: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBlinkingBackground: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSparkleText: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMarchingBlackAnts: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMarchingRedAnts: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomShimmer: tomConstants = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomWipeDown: tomConstants = 7i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomWipeRight: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAnimationMax: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLowerCase: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUpperCase: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTitleCase: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSentenceCase: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomToggleCase: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomReadOnly: tomConstants = 256i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomShareDenyRead: tomConstants = 512i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomShareDenyWrite: tomConstants = 1024i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPasteFile: tomConstants = 4096i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCreateNew: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCreateAlways: tomConstants = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomOpenExisting: tomConstants = 48i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomOpenAlways: tomConstants = 64i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTruncateExisting: tomConstants = 80i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRTF: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomText: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomHTML: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomWordDocument: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBold: tomConstants = -2147483647i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomItalic: tomConstants = -2147483646i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUnderline: tomConstants = -2147483644i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStrikeout: tomConstants = -2147483640i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomProtected: tomConstants = -2147483632i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLink: tomConstants = -2147483616i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSmallCaps: tomConstants = -2147483584i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAllCaps: tomConstants = -2147483520i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomHidden: tomConstants = -2147483392i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomOutline: tomConstants = -2147483136i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomShadow: tomConstants = -2147482624i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEmboss: tomConstants = -2147481600i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomImprint: tomConstants = -2147479552i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDisabled: tomConstants = -2147475456i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRevised: tomConstants = -2147467264i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSubscriptCF: tomConstants = -2147418112i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSuperscriptCF: tomConstants = -2147352576i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontBound: tomConstants = -2146435072i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLinkProtected: tomConstants = -2139095040i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomInlineObjectStart: tomConstants = -2130706432i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomExtendedChar: tomConstants = -2113929216i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAutoBackColor: tomConstants = -2080374784i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathZoneNoBuildUp: tomConstants = -2013265920i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathZone: tomConstants = -1879048192i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathZoneOrdinary: tomConstants = -1610612736i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAutoTextColor: tomConstants = -1073741824i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathZoneDisplay: tomConstants = 262144i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaEffectRTL: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaEffectKeep: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaEffectKeepNext: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaEffectPageBreakBefore: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaEffectNoLineNumber: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaEffectNoWidowControl: tomConstants = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaEffectDoNotHyphen: tomConstants = 64i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaEffectSideBySide: tomConstants = 128i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaEffectCollapsed: tomConstants = 256i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaEffectOutlineLevel: tomConstants = 512i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaEffectBox: tomConstants = 1024i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaEffectTableRowDelimiter: tomConstants = 4096i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaEffectTable: tomConstants = 16384i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomModWidthPairs: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomModWidthSpace: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAutoSpaceAlpha: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAutoSpaceNumeric: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAutoSpaceParens: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEmbeddedFont: tomConstants = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDoublestrike: tomConstants = 64i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomOverlapping: tomConstants = 128i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNormalCaret: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomKoreanBlockCaret: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNullCaret: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomIncludeInset: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUnicodeBiDi: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathCFCheck: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUnlink: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUnhide: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCheckTextLimit: tomConstants = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomIgnoreCurrentFont: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMatchCharRep: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMatchFontSignature: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMatchAscii: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGetHeightOnly: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMatchMathFont: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCharset: tomConstants = -2147483648i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCharRepFromLcid: tomConstants = 1073741824i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAnsi: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEastEurope: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCyrillic: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGreek: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTurkish: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomHebrew: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomArabic: tomConstants = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBaltic: tomConstants = 7i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomVietnamese: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDefaultCharRep: tomConstants = 9i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSymbol: tomConstants = 10i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomThai: tomConstants = 11i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomShiftJIS: tomConstants = 12i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGB2312: tomConstants = 13i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomHangul: tomConstants = 14i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBIG5: tomConstants = 15i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPC437: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomOEM: tomConstants = 17i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMac: tomConstants = 18i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomArmenian: tomConstants = 19i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSyriac: tomConstants = 20i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomThaana: tomConstants = 21i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDevanagari: tomConstants = 22i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBengali: tomConstants = 23i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGurmukhi: tomConstants = 24i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGujarati: tomConstants = 25i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomOriya: tomConstants = 26i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTamil: tomConstants = 27i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTelugu: tomConstants = 28i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomKannada: tomConstants = 29i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMalayalam: tomConstants = 30i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSinhala: tomConstants = 31i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLao: tomConstants = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTibetan: tomConstants = 33i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMyanmar: tomConstants = 34i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGeorgian: tomConstants = 35i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomJamo: tomConstants = 36i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEthiopic: tomConstants = 37i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCherokee: tomConstants = 38i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAboriginal: tomConstants = 39i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomOgham: tomConstants = 40i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRunic: tomConstants = 41i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomKhmer: tomConstants = 42i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMongolian: tomConstants = 43i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBraille: tomConstants = 44i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomYi: tomConstants = 45i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLimbu: tomConstants = 46i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTaiLe: tomConstants = 47i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNewTaiLue: tomConstants = 48i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSylotiNagri: tomConstants = 49i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomKharoshthi: tomConstants = 50i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomKayahli: tomConstants = 51i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUsymbol: tomConstants = 52i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEmoji: tomConstants = 53i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGlagolitic: tomConstants = 54i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLisu: tomConstants = 55i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomVai: tomConstants = 56i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNKo: tomConstants = 57i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomOsmanya: tomConstants = 58i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPhagsPa: tomConstants = 59i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGothic: tomConstants = 60i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDeseret: tomConstants = 61i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTifinagh: tomConstants = 62i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCharRepMax: tomConstants = 63i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRE10Mode: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUseAtFont: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTextFlowMask: tomConstants = 12i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTextFlowES: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTextFlowSW: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTextFlowWN: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTextFlowNE: tomConstants = 12i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNoIME: tomConstants = 524288i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelfIME: tomConstants = 262144i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNoUpScroll: tomConstants = 65536i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNoVpScroll: tomConstants = 262144i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNoLink: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomClientLink: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFriendlyLinkName: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFriendlyLinkAddress: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAutoLinkURL: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAutoLinkEmail: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAutoLinkPhone: tomConstants = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAutoLinkPath: tomConstants = 7i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCompressNone: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCompressPunctuation: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCompressPunctuationAndKana: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCompressMax: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUnderlinePositionAuto: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUnderlinePositionBelow: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUnderlinePositionAbove: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUnderlinePositionMax: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontAlignmentAuto: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontAlignmentTop: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontAlignmentBaseline: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontAlignmentBottom: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontAlignmentCenter: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontAlignmentMax: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRubyBelow: tomConstants = 128i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRubyAlignCenter: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRubyAlign010: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRubyAlign121: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRubyAlignLeft: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRubyAlignRight: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLimitsDefault: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLimitsUnderOver: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLimitsSubSup: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUpperLimitAsSuperScript: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLimitsOpposite: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomShowLLimPlaceHldr: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomShowULimPlaceHldr: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDontGrowWithContent: tomConstants = 64i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGrowWithContent: tomConstants = 128i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSubSupAlign: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLimitAlignMask: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLimitAlignCenter: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLimitAlignLeft: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLimitAlignRight: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomShowDegPlaceHldr: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAlignDefault: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAlignMatchAscentDescent: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathVariant: tomConstants = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStyleDefault: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStyleScriptScriptCramped: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStyleScriptScript: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStyleScriptCramped: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStyleScript: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStyleTextCramped: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStyleText: tomConstants = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStyleDisplayCramped: tomConstants = 7i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStyleDisplay: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathRelSize: tomConstants = 64i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDecDecSize: tomConstants = 254i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDecSize: tomConstants = 255i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomIncSize: tomConstants = 65i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomIncIncSize: tomConstants = 66i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGravityUI: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGravityBack: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGravityFore: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGravityIn: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGravityOut: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGravityBackward: tomConstants = 536870912i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomGravityForward: tomConstants = 1073741824i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAdjustCRLF: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUseCRLF: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTextize: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAllowFinalEOP: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFoldMathAlpha: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNoHidden: tomConstants = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomIncludeNumbering: tomConstants = 64i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTranslateTableCell: tomConstants = 128i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNoMathZoneBrackets: tomConstants = 256i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomConvertMathChar: tomConstants = 512i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNoUCGreekItalic: tomConstants = 1024i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomAllowMathBold: tomConstants = 2048i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomLanguageTag: tomConstants = 4096i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomConvertRTF: tomConstants = 8192i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomApplyRtfDocProps: tomConstants = 16384i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPhantomShow: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPhantomZeroWidth: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPhantomZeroAscent: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPhantomZeroDescent: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPhantomTransparent: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPhantomASmash: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPhantomDSmash: tomConstants = 9i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPhantomHSmash: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPhantomSmash: tomConstants = 13i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPhantomHorz: tomConstants = 12i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomPhantomVert: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBoxHideTop: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBoxHideBottom: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBoxHideLeft: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBoxHideRight: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBoxStrikeH: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBoxStrikeV: tomConstants = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBoxStrikeTLBR: tomConstants = 64i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBoxStrikeBLTR: tomConstants = 128i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomBoxAlignCenter: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSpaceMask: tomConstants = 28i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSpaceDefault: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSpaceUnary: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSpaceBinary: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSpaceRelational: tomConstants = 12i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSpaceSkip: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSpaceOrd: tomConstants = 20i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSpaceDifferential: tomConstants = 24i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSizeText: tomConstants = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSizeScript: tomConstants = 64i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSizeScriptScript: tomConstants = 96i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomNoBreak: tomConstants = 128i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTransparentForPositioning: tomConstants = 256i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomTransparentForSpacing: tomConstants = 512i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStretchCharBelow: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStretchCharAbove: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStretchBaseBelow: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomStretchBaseAbove: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMatrixAlignMask: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMatrixAlignCenter: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMatrixAlignTopRow: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMatrixAlignBottomRow: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomShowMatPlaceHldr: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEqArrayLayoutWidth: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEqArrayAlignMask: tomConstants = 12i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEqArrayAlignCenter: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEqArrayAlignTopRow: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEqArrayAlignBottomRow: tomConstants = 12i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathManualBreakMask: tomConstants = 127i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathBreakLeft: tomConstants = 125i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathBreakCenter: tomConstants = 126i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathBreakRight: tomConstants = 127i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathEqAlign: tomConstants = 128i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathArgShadingStart: tomConstants = 593i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathArgShadingEnd: tomConstants = 594i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathObjShadingStart: tomConstants = 595i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathObjShadingEnd: tomConstants = 596i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFunctionTypeNone: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFunctionTypeTakesArg: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFunctionTypeTakesLim: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFunctionTypeTakesLim2: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFunctionTypeIsLim: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathParaAlignDefault: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathParaAlignCenterGroup: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathParaAlignCenter: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathParaAlignLeft: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathParaAlignRight: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDispAlignMask: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDispAlignCenterGroup: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDispAlignCenter: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDispAlignLeft: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDispAlignRight: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDispIntUnderOver: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDispFracTeX: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDispNaryGrow: tomConstants = 16i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDocEmptyArgMask: tomConstants = 96i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDocEmptyArgAuto: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDocEmptyArgAlways: tomConstants = 32i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDocEmptyArgNever: tomConstants = 64i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDocSbSpOpUnchanged: tomConstants = 128i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDocDiffMask: tomConstants = 768i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDocDiffDefault: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDocDiffUpright: tomConstants = 256i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDocDiffItalic: tomConstants = 512i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDocDiffOpenItalic: tomConstants = 768i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDispNarySubSup: tomConstants = 1024i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathDispDef: tomConstants = 2048i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathEnableRtl: tomConstants = 4096i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathBrkBinMask: tomConstants = 196608i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathBrkBinBefore: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathBrkBinAfter: tomConstants = 65536i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathBrkBinDup: tomConstants = 131072i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathBrkBinSubMask: tomConstants = 786432i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathBrkBinSubMM: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathBrkBinSubPM: tomConstants = 262144i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathBrkBinSubMP: tomConstants = 524288i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomSelRange: tomConstants = 597i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomHstring: tomConstants = 596i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontPropTeXStyle: tomConstants = 828i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontPropAlign: tomConstants = 829i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStretch: tomConstants = 830i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStyle: tomConstants = 831i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStyleUpright: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStyleOblique: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStyleItalic: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStretchDefault: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStretchUltraCondensed: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStretchExtraCondensed: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStretchCondensed: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStretchSemiCondensed: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStretchNormal: tomConstants = 5i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStretchSemiExpanded: tomConstants = 6i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStretchExpanded: tomConstants = 7i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStretchExtraExpanded: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontStretchUltraExpanded: tomConstants = 9i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontWeightDefault: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontWeightThin: tomConstants = 100i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontWeightExtraLight: tomConstants = 200i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontWeightLight: tomConstants = 300i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontWeightNormal: tomConstants = 400i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontWeightRegular: tomConstants = 400i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontWeightMedium: tomConstants = 500i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontWeightSemiBold: tomConstants = 600i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontWeightBold: tomConstants = 700i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontWeightExtraBold: tomConstants = 800i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontWeightBlack: tomConstants = 900i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontWeightHeavy: tomConstants = 900i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomFontWeightExtraBlack: tomConstants = 950i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomParaPropMathAlign: tomConstants = 1079i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDocMathBuild: tomConstants = 128i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathLMargin: tomConstants = 129i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathRMargin: tomConstants = 130i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathWrapIndent: tomConstants = 131i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathWrapRight: tomConstants = 132i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathPostSpace: tomConstants = 134i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathPreSpace: tomConstants = 133i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathInterSpace: tomConstants = 135i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomMathIntraSpace: tomConstants = 136i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCanCopy: tomConstants = 137i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCanRedo: tomConstants = 138i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCanUndo: tomConstants = 139i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomUndoLimit: tomConstants = 140i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomDocAutoLink: tomConstants = 141i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEllipsisMode: tomConstants = 142i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEllipsisState: tomConstants = 143i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEllipsisNone: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEllipsisEnd: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEllipsisWord: tomConstants = 3i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomEllipsisPresent: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomVTopCell: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomVLowCell: tomConstants = 2i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomHStartCell: tomConstants = 4i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomHContCell: tomConstants = 8i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRowUpdate: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRowApplyDefault: tomConstants = 0i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomCellStructureChangeOnly: tomConstants = 1i32;
#[doc = "*Required features: 'Win32_UI_Controls_RichEdit'*"]
pub const tomRowHeightActual: tomConstants = 2059i32;
