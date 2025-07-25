pub const ATP_CHANGE: u32 = 1u32;
pub const ATP_NOCHANGE: u32 = 0u32;
pub const ATP_NODELIMITER: u32 = 2u32;
pub const ATP_REPLACEALLTEXT: u32 = 4u32;
pub const AURL_DISABLEMIXEDLGC: u32 = 32u32;
pub const AURL_ENABLEDRIVELETTERS: u32 = 16u32;
pub const AURL_ENABLEEA: u32 = 1u32;
pub const AURL_ENABLEEAURLS: u32 = 8u32;
pub const AURL_ENABLEEMAILADDR: u32 = 2u32;
pub const AURL_ENABLETELNO: u32 = 4u32;
pub const AURL_ENABLEURL: u32 = 1u32;
pub type AutoCorrectProc = Option<unsafe extern "system" fn(langid: u16, pszbefore: windows_core::PCWSTR, pszafter: windows_core::PCWSTR, cchafter: i32, pcchreplaced: *mut i32) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BIDIOPTIONS {
    pub cbSize: u32,
    pub wMask: u16,
    pub wEffects: u16,
}
pub const BOE_CONTEXTALIGNMENT: u32 = 16u32;
pub const BOE_CONTEXTREADING: u32 = 8u32;
pub const BOE_FORCERECALC: u32 = 32u32;
pub const BOE_LEGACYBIDICLASS: u32 = 64u32;
pub const BOE_NEUTRALOVERRIDE: u32 = 4u32;
pub const BOE_PLAINTEXT: u32 = 2u32;
pub const BOE_RTLDIR: u32 = 1u32;
pub const BOE_UNICODEBIDI: u32 = 128u32;
pub const BOM_CONTEXTALIGNMENT: u32 = 16u32;
pub const BOM_CONTEXTREADING: u32 = 8u32;
pub const BOM_DEFPARADIR: u32 = 1u32;
pub const BOM_LEGACYBIDICLASS: u32 = 64u32;
pub const BOM_NEUTRALOVERRIDE: u32 = 4u32;
pub const BOM_PLAINTEXT: u32 = 2u32;
pub const BOM_UNICODEBIDI: u32 = 128u32;
pub const CARET_CUSTOM: CARET_FLAGS = CARET_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CARET_FLAGS(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub union CARET_INFO {
    pub hbitmap: super::super::super::Graphics::Gdi::HBITMAP,
    pub caretFlags: CARET_FLAGS,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CARET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CARET_ITALIC: CARET_FLAGS = CARET_FLAGS(32i32);
pub const CARET_NONE: CARET_FLAGS = CARET_FLAGS(0i32);
pub const CARET_NULL: CARET_FLAGS = CARET_FLAGS(64i32);
pub const CARET_ROTATE90: CARET_FLAGS = CARET_FLAGS(128i32);
pub const CARET_RTL: CARET_FLAGS = CARET_FLAGS(2i32);
pub const CERICHEDIT_CLASSA: windows_core::PCSTR = windows_core::s!("RichEditCEA");
pub const CERICHEDIT_CLASSW: windows_core::PCWSTR = windows_core::w!("RichEditCEW");
pub const CFE_ALLCAPS: CFE_EFFECTS = CFE_EFFECTS(128u32);
pub const CFE_AUTOBACKCOLOR: CFE_EFFECTS = CFE_EFFECTS(67108864u32);
pub const CFE_AUTOCOLOR: CFE_EFFECTS = CFE_EFFECTS(1073741824u32);
pub const CFE_BOLD: CFE_EFFECTS = CFE_EFFECTS(1u32);
pub const CFE_DISABLED: CFE_EFFECTS = CFE_EFFECTS(8192u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CFE_EFFECTS(pub u32);
impl CFE_EFFECTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CFE_EFFECTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CFE_EFFECTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CFE_EFFECTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CFE_EFFECTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CFE_EFFECTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CFE_EMBOSS: CFE_EFFECTS = CFE_EFFECTS(2048u32);
pub const CFE_EXTENDED: CFE_EFFECTS = CFE_EFFECTS(33554432u32);
pub const CFE_FONTBOUND: CFE_EFFECTS = CFE_EFFECTS(1048576u32);
pub const CFE_HIDDEN: CFE_EFFECTS = CFE_EFFECTS(256u32);
pub const CFE_IMPRINT: CFE_EFFECTS = CFE_EFFECTS(4096u32);
pub const CFE_ITALIC: CFE_EFFECTS = CFE_EFFECTS(2u32);
pub const CFE_LINK: CFE_EFFECTS = CFE_EFFECTS(32u32);
pub const CFE_LINKPROTECTED: CFE_EFFECTS = CFE_EFFECTS(8388608u32);
pub const CFE_MATH: CFE_EFFECTS = CFE_EFFECTS(268435456u32);
pub const CFE_MATHNOBUILDUP: CFE_EFFECTS = CFE_EFFECTS(134217728u32);
pub const CFE_MATHORDINARY: CFE_EFFECTS = CFE_EFFECTS(536870912u32);
pub const CFE_OUTLINE: CFE_EFFECTS = CFE_EFFECTS(512u32);
pub const CFE_PROTECTED: CFE_EFFECTS = CFE_EFFECTS(16u32);
pub const CFE_REVISED: CFE_EFFECTS = CFE_EFFECTS(16384u32);
pub const CFE_SHADOW: CFE_EFFECTS = CFE_EFFECTS(1024u32);
pub const CFE_SMALLCAPS: CFE_EFFECTS = CFE_EFFECTS(64u32);
pub const CFE_STRIKEOUT: CFE_EFFECTS = CFE_EFFECTS(8u32);
pub const CFE_SUBSCRIPT: CFE_EFFECTS = CFE_EFFECTS(65536u32);
pub const CFE_SUPERSCRIPT: CFE_EFFECTS = CFE_EFFECTS(131072u32);
pub const CFE_UNDERLINE: CFE_EFFECTS = CFE_EFFECTS(4u32);
pub const CFM_ALL: CFM_MASK = CFM_MASK(4160749631u32);
pub const CFM_ALL2: CFM_MASK = CFM_MASK(4294967295u32);
pub const CFM_ALLCAPS: CFM_MASK = CFM_MASK(128u32);
pub const CFM_ALLEFFECTS: CFM_MASK = CFM_MASK(2115207167u32);
pub const CFM_ANIMATION: CFM_MASK = CFM_MASK(262144u32);
pub const CFM_BACKCOLOR: CFM_MASK = CFM_MASK(67108864u32);
pub const CFM_BOLD: CFM_MASK = CFM_MASK(1u32);
pub const CFM_CHARSET: CFM_MASK = CFM_MASK(134217728u32);
pub const CFM_COLOR: CFM_MASK = CFM_MASK(1073741824u32);
pub const CFM_COOKIE: CFM_MASK = CFM_MASK(16777216u32);
pub const CFM_DISABLED: CFM_MASK = CFM_MASK(8192u32);
pub const CFM_EFFECTS: CFM_MASK = CFM_MASK(1073741887u32);
pub const CFM_EFFECTS2: CFM_MASK = CFM_MASK(1141080063u32);
pub const CFM_EMBOSS: CFM_MASK = CFM_MASK(2048u32);
pub const CFM_EXTENDED: CFM_MASK = CFM_MASK(33554432u32);
pub const CFM_FACE: CFM_MASK = CFM_MASK(536870912u32);
pub const CFM_FONTBOUND: CFM_MASK = CFM_MASK(1048576u32);
pub const CFM_HIDDEN: CFM_MASK = CFM_MASK(256u32);
pub const CFM_IMPRINT: CFM_MASK = CFM_MASK(4096u32);
pub const CFM_ITALIC: CFM_MASK = CFM_MASK(2u32);
pub const CFM_KERNING: CFM_MASK = CFM_MASK(1048576u32);
pub const CFM_LCID: CFM_MASK = CFM_MASK(33554432u32);
pub const CFM_LINK: CFM_MASK = CFM_MASK(32u32);
pub const CFM_LINKPROTECTED: CFM_MASK = CFM_MASK(8388608u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CFM_MASK(pub u32);
impl CFM_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CFM_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CFM_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CFM_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CFM_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CFM_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CFM_MATH: CFM_MASK = CFM_MASK(268435456u32);
pub const CFM_MATHNOBUILDUP: CFM_MASK = CFM_MASK(134217728u32);
pub const CFM_MATHORDINARY: CFM_MASK = CFM_MASK(536870912u32);
pub const CFM_OFFSET: CFM_MASK = CFM_MASK(268435456u32);
pub const CFM_OUTLINE: CFM_MASK = CFM_MASK(512u32);
pub const CFM_PROTECTED: CFM_MASK = CFM_MASK(16u32);
pub const CFM_REVAUTHOR: CFM_MASK = CFM_MASK(32768u32);
pub const CFM_REVISED: CFM_MASK = CFM_MASK(16384u32);
pub const CFM_SHADOW: CFM_MASK = CFM_MASK(1024u32);
pub const CFM_SIZE: CFM_MASK = CFM_MASK(2147483648u32);
pub const CFM_SMALLCAPS: CFM_MASK = CFM_MASK(64u32);
pub const CFM_SPACING: CFM_MASK = CFM_MASK(2097152u32);
pub const CFM_STRIKEOUT: CFM_MASK = CFM_MASK(8u32);
pub const CFM_STYLE: CFM_MASK = CFM_MASK(524288u32);
pub const CFM_SUBSCRIPT: CFM_MASK = CFM_MASK(196608u32);
pub const CFM_SUPERSCRIPT: CFM_MASK = CFM_MASK(196608u32);
pub const CFM_UNDERLINE: CFM_MASK = CFM_MASK(4u32);
pub const CFM_UNDERLINETYPE: CFM_MASK = CFM_MASK(8388608u32);
pub const CFM_WEIGHT: CFM_MASK = CFM_MASK(4194304u32);
pub const CF_RETEXTOBJ: windows_core::PCWSTR = windows_core::w!("RichEdit Text and Objects");
pub const CF_RTF: windows_core::PCWSTR = windows_core::w!("Rich Text Format");
pub const CF_RTFNOOBJS: windows_core::PCWSTR = windows_core::w!("Rich Text Format Without Objects");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CHANGENOTIFY {
    pub dwChangeType: u32,
    pub pvCookieData: *mut core::ffi::c_void,
}
impl Default for CHANGENOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGETYPE(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct CHARFORMAT2A {
    pub Base: CHARFORMATA,
    pub wWeight: u16,
    pub sSpacing: i16,
    pub crBackColor: super::super::super::Foundation::COLORREF,
    pub lcid: u32,
    pub Anonymous: CHARFORMAT2A_0,
    pub sStyle: i16,
    pub wKerning: u16,
    pub bUnderlineType: u8,
    pub bAnimation: u8,
    pub bRevAuthor: u8,
    pub bUnderlineColor: u8,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHARFORMAT2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub union CHARFORMAT2A_0 {
    pub dwReserved: u32,
    pub dwCookie: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHARFORMAT2A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct CHARFORMAT2W {
    pub Base: CHARFORMATW,
    pub wWeight: u16,
    pub sSpacing: i16,
    pub crBackColor: super::super::super::Foundation::COLORREF,
    pub lcid: u32,
    pub Anonymous: CHARFORMAT2W_0,
    pub sStyle: i16,
    pub wKerning: u16,
    pub bUnderlineType: u8,
    pub bAnimation: u8,
    pub bRevAuthor: u8,
    pub bUnderlineColor: u8,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHARFORMAT2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub union CHARFORMAT2W_0 {
    pub dwReserved: u32,
    pub dwCookie: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHARFORMAT2W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CHARFORMATA {
    pub cbSize: u32,
    pub dwMask: CFM_MASK,
    pub dwEffects: CFE_EFFECTS,
    pub yHeight: i32,
    pub yOffset: i32,
    pub crTextColor: super::super::super::Foundation::COLORREF,
    pub bCharSet: super::super::super::Graphics::Gdi::FONT_CHARSET,
    pub bPitchAndFamily: u8,
    pub szFaceName: [i8; 32],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHARFORMATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CHARFORMATW {
    pub cbSize: u32,
    pub dwMask: CFM_MASK,
    pub dwEffects: CFE_EFFECTS,
    pub yHeight: i32,
    pub yOffset: i32,
    pub crTextColor: super::super::super::Foundation::COLORREF,
    pub bCharSet: super::super::super::Graphics::Gdi::FONT_CHARSET,
    pub bPitchAndFamily: u8,
    pub szFaceName: [u16; 32],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHARFORMATW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CHARRANGE {
    pub cpMin: i32,
    pub cpMax: i32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLIPBOARDFORMAT {
    pub nmhdr: super::NMHDR,
    pub cf: u16,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct CLIPBOARDFORMAT {
    pub nmhdr: super::NMHDR,
    pub cf: u16,
}
pub const CN_GENERIC: CHANGETYPE = CHANGETYPE(0i32);
pub const CN_NEWREDO: CHANGETYPE = CHANGETYPE(4i32);
pub const CN_NEWUNDO: CHANGETYPE = CHANGETYPE(2i32);
pub const CN_TEXTCHANGED: CHANGETYPE = CHANGETYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMPCOLOR {
    pub crText: super::super::super::Foundation::COLORREF,
    pub crBackground: super::super::super::Foundation::COLORREF,
    pub dwEffects: u32,
}
pub const CTFMODEBIAS_CONVERSATION: u32 = 5u32;
pub const CTFMODEBIAS_DATETIME: u32 = 4u32;
pub const CTFMODEBIAS_DEFAULT: u32 = 0u32;
pub const CTFMODEBIAS_FILENAME: u32 = 1u32;
pub const CTFMODEBIAS_FULLWIDTHALPHANUMERIC: u32 = 11u32;
pub const CTFMODEBIAS_HALFWIDTHALPHANUMERIC: u32 = 12u32;
pub const CTFMODEBIAS_HALFWIDTHKATAKANA: u32 = 10u32;
pub const CTFMODEBIAS_HANGUL: u32 = 9u32;
pub const CTFMODEBIAS_HIRAGANA: u32 = 7u32;
pub const CTFMODEBIAS_KATAKANA: u32 = 8u32;
pub const CTFMODEBIAS_NAME: u32 = 2u32;
pub const CTFMODEBIAS_NUMERIC: u32 = 6u32;
pub const CTFMODEBIAS_READING: u32 = 3u32;
pub const ECN_ENDCOMPOSITION: ENDCOMPOSITIONNOTIFY_CODE = ENDCOMPOSITIONNOTIFY_CODE(1u32);
pub const ECN_NEWTEXT: ENDCOMPOSITIONNOTIFY_CODE = ENDCOMPOSITIONNOTIFY_CODE(2u32);
pub const ECOOP_AND: u32 = 3u32;
pub const ECOOP_OR: u32 = 2u32;
pub const ECOOP_SET: u32 = 1u32;
pub const ECOOP_XOR: u32 = 4u32;
pub const ECO_AUTOHSCROLL: u32 = 128u32;
pub const ECO_AUTOVSCROLL: u32 = 64u32;
pub const ECO_AUTOWORDSELECTION: u32 = 1u32;
pub const ECO_NOHIDESEL: u32 = 256u32;
pub const ECO_READONLY: u32 = 2048u32;
pub const ECO_SAVESEL: u32 = 32768u32;
pub const ECO_SELECTIONBAR: u32 = 16777216u32;
pub const ECO_VERTICAL: u32 = 4194304u32;
pub const ECO_WANTRETURN: u32 = 4096u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default)]
pub struct EDITSTREAM {
    pub dwCookie: usize,
    pub dwError: u32,
    pub pfnCallback: EDITSTREAMCALLBACK,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct EDITSTREAM {
    pub dwCookie: usize,
    pub dwError: u32,
    pub pfnCallback: EDITSTREAMCALLBACK,
}
pub type EDITSTREAMCALLBACK = Option<unsafe extern "system" fn(dwcookie: usize, pbbuff: *mut u8, cb: i32, pcb: *mut i32) -> u32>;
pub type EDITWORDBREAKPROCEX = Option<unsafe extern "system" fn(pchtext: windows_core::PCSTR, cchtext: i32, bcharset: u8, action: i32) -> i32>;
pub const ELLIPSIS_END: u32 = 1u32;
pub const ELLIPSIS_MASK: u32 = 3u32;
pub const ELLIPSIS_NONE: u32 = 0u32;
pub const ELLIPSIS_WORD: u32 = 3u32;
pub const EMO_ENTER: u32 = 1u32;
pub const EMO_EXIT: u32 = 0u32;
pub const EMO_EXPAND: u32 = 3u32;
pub const EMO_EXPANDDOCUMENT: u32 = 1u32;
pub const EMO_EXPANDSELECTION: u32 = 0u32;
pub const EMO_GETVIEWMODE: u32 = 5u32;
pub const EMO_MOVESELECTION: u32 = 4u32;
pub const EMO_PROMOTE: u32 = 2u32;
pub const EM_AUTOURLDETECT: u32 = 1115u32;
pub const EM_CALLAUTOCORRECTPROC: u32 = 1279u32;
pub const EM_CANPASTE: u32 = 1074u32;
pub const EM_CANREDO: u32 = 1109u32;
pub const EM_CONVPOSITION: u32 = 1132u32;
pub const EM_DISPLAYBAND: u32 = 1075u32;
pub const EM_EXGETSEL: u32 = 1076u32;
pub const EM_EXLIMITTEXT: u32 = 1077u32;
pub const EM_EXLINEFROMCHAR: u32 = 1078u32;
pub const EM_EXSETSEL: u32 = 1079u32;
pub const EM_FINDTEXT: u32 = 1080u32;
pub const EM_FINDTEXTEX: u32 = 1103u32;
pub const EM_FINDTEXTEXW: u32 = 1148u32;
pub const EM_FINDTEXTW: u32 = 1147u32;
pub const EM_FINDWORDBREAK: u32 = 1100u32;
pub const EM_FORMATRANGE: u32 = 1081u32;
pub const EM_GETAUTOCORRECTPROC: u32 = 1257u32;
pub const EM_GETAUTOURLDETECT: u32 = 1116u32;
pub const EM_GETBIDIOPTIONS: u32 = 1225u32;
pub const EM_GETCHARFORMAT: u32 = 1082u32;
pub const EM_GETCTFMODEBIAS: u32 = 1261u32;
pub const EM_GETCTFOPENSTATUS: u32 = 1264u32;
pub const EM_GETEDITSTYLE: u32 = 1229u32;
pub const EM_GETEDITSTYLEEX: u32 = 1300u32;
pub const EM_GETELLIPSISMODE: u32 = 1329u32;
pub const EM_GETELLIPSISSTATE: u32 = 1346u32;
pub const EM_GETEVENTMASK: u32 = 1083u32;
pub const EM_GETHYPHENATEINFO: u32 = 1254u32;
pub const EM_GETIMECOLOR: u32 = 1129u32;
pub const EM_GETIMECOMPMODE: u32 = 1146u32;
pub const EM_GETIMECOMPTEXT: u32 = 1266u32;
pub const EM_GETIMEMODEBIAS: u32 = 1151u32;
pub const EM_GETIMEOPTIONS: u32 = 1131u32;
pub const EM_GETIMEPROPERTY: u32 = 1268u32;
pub const EM_GETLANGOPTIONS: u32 = 1145u32;
pub const EM_GETOLEINTERFACE: u32 = 1084u32;
pub const EM_GETOPTIONS: u32 = 1102u32;
pub const EM_GETPAGE: u32 = 1252u32;
pub const EM_GETPAGEROTATE: u32 = 1259u32;
pub const EM_GETPARAFORMAT: u32 = 1085u32;
pub const EM_GETPUNCTUATION: u32 = 1125u32;
pub const EM_GETQUERYRTFOBJ: u32 = 1293u32;
pub const EM_GETREDONAME: u32 = 1111u32;
pub const EM_GETSCROLLPOS: u32 = 1245u32;
pub const EM_GETSELTEXT: u32 = 1086u32;
pub const EM_GETSTORYTYPE: u32 = 1314u32;
pub const EM_GETTABLEPARMS: u32 = 1289u32;
pub const EM_GETTEXTEX: u32 = 1118u32;
pub const EM_GETTEXTLENGTHEX: u32 = 1119u32;
pub const EM_GETTEXTMODE: u32 = 1114u32;
pub const EM_GETTEXTRANGE: u32 = 1099u32;
pub const EM_GETTOUCHOPTIONS: u32 = 1334u32;
pub const EM_GETTYPOGRAPHYOPTIONS: u32 = 1227u32;
pub const EM_GETUNDONAME: u32 = 1110u32;
pub const EM_GETVIEWKIND: u32 = 1250u32;
pub const EM_GETWORDBREAKPROCEX: u32 = 1104u32;
pub const EM_GETWORDWRAPMODE: u32 = 1127u32;
pub const EM_GETZOOM: u32 = 1248u32;
pub const EM_HIDESELECTION: u32 = 1087u32;
pub const EM_INSERTIMAGE: u32 = 1338u32;
pub const EM_INSERTTABLE: u32 = 1256u32;
pub const EM_ISIME: u32 = 1267u32;
pub const EM_OUTLINE: u32 = 1244u32;
pub const EM_PASTESPECIAL: u32 = 1088u32;
pub const EM_RECONVERSION: u32 = 1149u32;
pub const EM_REDO: u32 = 1108u32;
pub const EM_REQUESTRESIZE: u32 = 1089u32;
pub const EM_SELECTIONTYPE: u32 = 1090u32;
pub const EM_SETAUTOCORRECTPROC: u32 = 1258u32;
pub const EM_SETBIDIOPTIONS: u32 = 1224u32;
pub const EM_SETBKGNDCOLOR: u32 = 1091u32;
pub const EM_SETCHARFORMAT: u32 = 1092u32;
pub const EM_SETCTFMODEBIAS: u32 = 1262u32;
pub const EM_SETCTFOPENSTATUS: u32 = 1265u32;
pub const EM_SETDISABLEOLELINKCONVERSION: u32 = 1428u32;
pub const EM_SETEDITSTYLE: u32 = 1228u32;
pub const EM_SETEDITSTYLEEX: u32 = 1299u32;
pub const EM_SETELLIPSISMODE: u32 = 1330u32;
pub const EM_SETEVENTMASK: u32 = 1093u32;
pub const EM_SETFONTSIZE: u32 = 1247u32;
pub const EM_SETHYPHENATEINFO: u32 = 1255u32;
pub const EM_SETIMECOLOR: u32 = 1128u32;
pub const EM_SETIMEMODEBIAS: u32 = 1150u32;
pub const EM_SETIMEOPTIONS: u32 = 1130u32;
pub const EM_SETLANGOPTIONS: u32 = 1144u32;
pub const EM_SETOLECALLBACK: u32 = 1094u32;
pub const EM_SETOPTIONS: u32 = 1101u32;
pub const EM_SETPAGE: u32 = 1253u32;
pub const EM_SETPAGEROTATE: u32 = 1260u32;
pub const EM_SETPALETTE: u32 = 1117u32;
pub const EM_SETPARAFORMAT: u32 = 1095u32;
pub const EM_SETPUNCTUATION: u32 = 1124u32;
pub const EM_SETQUERYCONVERTOLELINKCALLBACK: u32 = 1427u32;
pub const EM_SETQUERYRTFOBJ: u32 = 1294u32;
pub const EM_SETSCROLLPOS: u32 = 1246u32;
pub const EM_SETSTORYTYPE: u32 = 1315u32;
pub const EM_SETTABLEPARMS: u32 = 1331u32;
pub const EM_SETTARGETDEVICE: u32 = 1096u32;
pub const EM_SETTEXTEX: u32 = 1121u32;
pub const EM_SETTEXTMODE: u32 = 1113u32;
pub const EM_SETTOUCHOPTIONS: u32 = 1335u32;
pub const EM_SETTYPOGRAPHYOPTIONS: u32 = 1226u32;
pub const EM_SETUIANAME: u32 = 1344u32;
pub const EM_SETUNDOLIMIT: u32 = 1106u32;
pub const EM_SETVIEWKIND: u32 = 1251u32;
pub const EM_SETWORDBREAKPROCEX: u32 = 1105u32;
pub const EM_SETWORDWRAPMODE: u32 = 1126u32;
pub const EM_SETZOOM: u32 = 1249u32;
pub const EM_SHOWSCROLLBAR: u32 = 1120u32;
pub const EM_STOPGROUPTYPING: u32 = 1112u32;
pub const EM_STREAMIN: u32 = 1097u32;
pub const EM_STREAMOUT: u32 = 1098u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENCORRECTTEXT {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct ENCORRECTTEXT {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENDCOMPOSITIONNOTIFY {
    pub nmhdr: super::NMHDR,
    pub dwCode: ENDCOMPOSITIONNOTIFY_CODE,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct ENDCOMPOSITIONNOTIFY {
    pub nmhdr: super::NMHDR,
    pub dwCode: ENDCOMPOSITIONNOTIFY_CODE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENDCOMPOSITIONNOTIFY_CODE(pub u32);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENDROPFILES {
    pub nmhdr: super::NMHDR,
    pub hDrop: super::super::super::Foundation::HANDLE,
    pub cp: i32,
    pub fProtected: windows_core::BOOL,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct ENDROPFILES {
    pub nmhdr: super::NMHDR,
    pub hDrop: super::super::super::Foundation::HANDLE,
    pub cp: i32,
    pub fProtected: windows_core::BOOL,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENLINK {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct ENLINK {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENLOWFIRTF {
    pub nmhdr: super::NMHDR,
    pub szControl: windows_core::PSTR,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct ENLOWFIRTF {
    pub nmhdr: super::NMHDR,
    pub szControl: windows_core::PSTR,
}
pub const ENM_CHANGE: u32 = 1u32;
pub const ENM_CLIPFORMAT: u32 = 128u32;
pub const ENM_CORRECTTEXT: u32 = 4194304u32;
pub const ENM_DRAGDROPDONE: u32 = 16u32;
pub const ENM_DROPFILES: u32 = 1048576u32;
pub const ENM_ENDCOMPOSITION: u32 = 536870912u32;
pub const ENM_GROUPTYPINGCHANGE: u32 = 1073741824u32;
pub const ENM_HIDELINKTOOLTIP: u32 = 2147483648u32;
pub const ENM_IMECHANGE: u32 = 8388608u32;
pub const ENM_KEYEVENTS: u32 = 65536u32;
pub const ENM_LANGCHANGE: u32 = 16777216u32;
pub const ENM_LINK: u32 = 67108864u32;
pub const ENM_LOWFIRTF: u32 = 134217728u32;
pub const ENM_MOUSEEVENTS: u32 = 131072u32;
pub const ENM_NONE: u32 = 0u32;
pub const ENM_OBJECTPOSITIONS: u32 = 33554432u32;
pub const ENM_PAGECHANGE: u32 = 64u32;
pub const ENM_PARAGRAPHEXPANDED: u32 = 32u32;
pub const ENM_PROTECTED: u32 = 2097152u32;
pub const ENM_REQUESTRESIZE: u32 = 262144u32;
pub const ENM_SCROLL: u32 = 4u32;
pub const ENM_SCROLLEVENTS: u32 = 8u32;
pub const ENM_SELCHANGE: u32 = 524288u32;
pub const ENM_STARTCOMPOSITION: u32 = 268435456u32;
pub const ENM_UPDATE: u32 = 2u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENOLEOPFAILED {
    pub nmhdr: super::NMHDR,
    pub iob: i32,
    pub lOper: i32,
    pub hr: windows_core::HRESULT,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct ENOLEOPFAILED {
    pub nmhdr: super::NMHDR,
    pub iob: i32,
    pub lOper: i32,
    pub hr: windows_core::HRESULT,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENPROTECTED {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct ENPROTECTED {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENSAVECLIPBOARD {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub cch: i32,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct ENSAVECLIPBOARD {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub cch: i32,
}
pub const EN_ALIGNLTR: u32 = 1808u32;
pub const EN_ALIGNRTL: u32 = 1809u32;
pub const EN_CLIPFORMAT: u32 = 1810u32;
pub const EN_CORRECTTEXT: u32 = 1797u32;
pub const EN_DRAGDROPDONE: u32 = 1804u32;
pub const EN_DROPFILES: u32 = 1795u32;
pub const EN_ENDCOMPOSITION: u32 = 1812u32;
pub const EN_IMECHANGE: u32 = 1799u32;
pub const EN_LINK: u32 = 1803u32;
pub const EN_LOWFIRTF: u32 = 1807u32;
pub const EN_MSGFILTER: u32 = 1792u32;
pub const EN_OBJECTPOSITIONS: u32 = 1802u32;
pub const EN_OLEOPFAILED: u32 = 1801u32;
pub const EN_PAGECHANGE: u32 = 1806u32;
pub const EN_PARAGRAPHEXPANDED: u32 = 1805u32;
pub const EN_PROTECTED: u32 = 1796u32;
pub const EN_REQUESTRESIZE: u32 = 1793u32;
pub const EN_SAVECLIPBOARD: u32 = 1800u32;
pub const EN_SELCHANGE: u32 = 1794u32;
pub const EN_STARTCOMPOSITION: u32 = 1811u32;
pub const EN_STOPNOUNDO: u32 = 1798u32;
pub const EPR_0: u32 = 0u32;
pub const EPR_180: u32 = 2u32;
pub const EPR_270: u32 = 1u32;
pub const EPR_90: u32 = 3u32;
pub const EPR_SE: u32 = 5u32;
pub const ES_DISABLENOSCROLL: u32 = 8192u32;
pub const ES_EX_NOCALLOLEINIT: u32 = 0u32;
pub const ES_NOIME: u32 = 524288u32;
pub const ES_NOOLEDRAGDROP: u32 = 8u32;
pub const ES_SAVESEL: u32 = 32768u32;
pub const ES_SELECTIONBAR: u32 = 16777216u32;
pub const ES_SELFIME: u32 = 262144u32;
pub const ES_SUNKEN: u32 = 16384u32;
pub const ES_VERTICAL: u32 = 4194304u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FINDTEXTA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PCSTR,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct FINDTEXTA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FINDTEXTEXA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PCSTR,
    pub chrgText: CHARRANGE,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct FINDTEXTEXA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PCSTR,
    pub chrgText: CHARRANGE,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FINDTEXTEXW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PCWSTR,
    pub chrgText: CHARRANGE,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct FINDTEXTEXW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PCWSTR,
    pub chrgText: CHARRANGE,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FINDTEXTW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PCWSTR,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct FINDTEXTW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FORMATRANGE {
    pub hdc: super::super::super::Graphics::Gdi::HDC,
    pub hdcTarget: super::super::super::Graphics::Gdi::HDC,
    pub rc: super::super::super::Foundation::RECT,
    pub rcPage: super::super::super::Foundation::RECT,
    pub chrg: CHARRANGE,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Default)]
pub struct FORMATRANGE {
    pub hdc: super::super::super::Graphics::Gdi::HDC,
    pub hdcTarget: super::super::super::Graphics::Gdi::HDC,
    pub rc: super::super::super::Foundation::RECT,
    pub rcPage: super::super::super::Foundation::RECT,
    pub chrg: CHARRANGE,
}
pub const GCMF_GRIPPER: u32 = 1u32;
pub const GCMF_MOUSEMENU: u32 = 8192u32;
pub const GCMF_SPELLING: u32 = 2u32;
pub const GCMF_TOUCHMENU: u32 = 16384u32;
pub const GCM_MOUSEMENU: u32 = 8192u32;
pub const GCM_RIGHTMOUSEDROP: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(32768u16);
pub const GCM_TOUCHMENU: u32 = 16384u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GETCONTEXTMENUEX {
    pub chrg: CHARRANGE,
    pub dwFlags: u32,
    pub pt: super::super::super::Foundation::POINT,
    pub pvReserved: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl Default for GETCONTEXTMENUEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct GETCONTEXTMENUEX {
    pub chrg: CHARRANGE,
    pub dwFlags: u32,
    pub pt: super::super::super::Foundation::POINT,
    pub pvReserved: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for GETCONTEXTMENUEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GETTEXTEX {
    pub cb: u32,
    pub flags: GETTEXTEX_FLAGS,
    pub codepage: u32,
    pub lpDefaultChar: windows_core::PCSTR,
    pub lpUsedDefChar: *mut windows_core::BOOL,
}
#[cfg(target_arch = "x86")]
impl Default for GETTEXTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct GETTEXTEX {
    pub cb: u32,
    pub flags: GETTEXTEX_FLAGS,
    pub codepage: u32,
    pub lpDefaultChar: windows_core::PCSTR,
    pub lpUsedDefChar: *mut windows_core::BOOL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for GETTEXTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GETTEXTEX_FLAGS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GETTEXTLENGTHEX {
    pub flags: GETTEXTLENGTHEX_FLAGS,
    pub codepage: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GETTEXTLENGTHEX_FLAGS(pub u32);
impl GETTEXTLENGTHEX_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GETTEXTLENGTHEX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GETTEXTLENGTHEX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GETTEXTLENGTHEX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GETTEXTLENGTHEX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GETTEXTLENGTHEX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct GROUPTYPINGCHANGE {
    pub nmhdr: super::NMHDR,
    pub fGroupTyping: windows_core::BOOL,
}
pub const GTL_CLOSE: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(4u32);
pub const GTL_DEFAULT: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(0u32);
pub const GTL_NUMBYTES: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(16u32);
pub const GTL_NUMCHARS: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(8u32);
pub const GTL_PRECISE: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(2u32);
pub const GTL_USECRLF: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(1u32);
pub const GT_DEFAULT: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(0u32);
pub const GT_NOHIDDENTEXT: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(8u32);
pub const GT_RAWTEXT: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(4u32);
pub const GT_SELECTION: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(2u32);
pub const GT_USECRLF: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(1u32);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HYPHENATEINFO {
    pub cbSize: i16,
    pub dxHyphenateZone: i16,
    pub pfnHyphenate: isize,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct HYPHENATEINFO {
    pub cbSize: i16,
    pub dxHyphenateZone: i16,
    pub pfnHyphenate: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HYPHRESULT {
    pub khyph: KHYPH,
    pub ichHyph: i32,
    pub chHyph: u16,
}
pub const ICM_CTF: u32 = 5u32;
pub const ICM_LEVEL2: u32 = 2u32;
pub const ICM_LEVEL2_5: u32 = 3u32;
pub const ICM_LEVEL2_SUI: u32 = 4u32;
pub const ICM_LEVEL3: u32 = 1u32;
pub const ICM_NOTOPEN: u32 = 0u32;
pub const ICT_RESULTREADSTR: IMECOMPTEXT_FLAGS = IMECOMPTEXT_FLAGS(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMECOMPTEXT {
    pub cb: i32,
    pub flags: IMECOMPTEXT_FLAGS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMECOMPTEXT_FLAGS(pub u32);
pub const IMF_AUTOFONT: u32 = 2u32;
pub const IMF_AUTOFONTSIZEADJUST: u32 = 16u32;
pub const IMF_AUTOKEYBOARD: u32 = 1u32;
pub const IMF_CLOSESTATUSWINDOW: u32 = 8u32;
pub const IMF_DUALFONT: u32 = 128u32;
pub const IMF_FORCEACTIVE: u32 = 64u32;
pub const IMF_FORCEDISABLE: u32 = 4u32;
pub const IMF_FORCEENABLE: u32 = 2u32;
pub const IMF_FORCEINACTIVE: u32 = 128u32;
pub const IMF_FORCENONE: u32 = 1u32;
pub const IMF_FORCEREMEMBER: u32 = 256u32;
pub const IMF_IMEALWAYSSENDNOTIFY: u32 = 8u32;
pub const IMF_IMECANCELCOMPLETE: u32 = 4u32;
pub const IMF_IMEUIINTEGRATION: u32 = 8192u32;
pub const IMF_MULTIPLEEDIT: u32 = 1024u32;
pub const IMF_NOIMPLICITLANG: u32 = 64u32;
pub const IMF_NOKBDLIDFIXUP: u32 = 512u32;
pub const IMF_NORTFFONTSUBSTITUTE: u32 = 1024u32;
pub const IMF_SMODE_NONE: u32 = 2u32;
pub const IMF_SMODE_PLAURALCLAUSE: u32 = 1u32;
pub const IMF_SPELLCHECKING: u32 = 2048u32;
pub const IMF_TKBPREDICTION: u32 = 4096u32;
pub const IMF_UIFONTS: u32 = 32u32;
pub const IMF_VERTICAL: u32 = 32u32;
windows_core::imp::define_interface!(IRichEditOle, IRichEditOle_Vtbl, 0x00020d00_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRichEditOle, windows_core::IUnknown);
impl IRichEditOle {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetClientSite(&self) -> windows_core::Result<super::super::super::System::Ole::IOleClientSite> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClientSite)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetObjectCount(&self) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).GetObjectCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLinkCount(&self) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).GetLinkCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn GetObject(&self, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), iob, core::mem::transmute(lpreobject), dwflags).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn InsertObject(&self, lpreobject: *mut REOBJECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InsertObject)(windows_core::Interface::as_raw(self), core::mem::transmute(lpreobject)).ok() }
    }
    pub unsafe fn ConvertObject<P2>(&self, iob: i32, rclsidnew: *const windows_core::GUID, lpstrusertypenew: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertObject)(windows_core::Interface::as_raw(self), iob, rclsidnew, lpstrusertypenew.param().abi()).ok() }
    }
    pub unsafe fn ActivateAs(&self, rclsid: *const windows_core::GUID, rclsidas: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ActivateAs)(windows_core::Interface::as_raw(self), rclsid, rclsidas).ok() }
    }
    pub unsafe fn SetHostNames<P0, P1>(&self, lpstrcontainerapp: P0, lpstrcontainerobj: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHostNames)(windows_core::Interface::as_raw(self), lpstrcontainerapp.param().abi(), lpstrcontainerobj.param().abi()).ok() }
    }
    pub unsafe fn SetLinkAvailable(&self, iob: i32, favailable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLinkAvailable)(windows_core::Interface::as_raw(self), iob, favailable.into()).ok() }
    }
    pub unsafe fn SetDvaspect(&self, iob: i32, dvaspect: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDvaspect)(windows_core::Interface::as_raw(self), iob, dvaspect).ok() }
    }
    pub unsafe fn HandsOffStorage(&self, iob: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HandsOffStorage)(windows_core::Interface::as_raw(self), iob).ok() }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SaveCompleted<P1>(&self, iob: i32, lpstg: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::super::System::Com::StructuredStorage::IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveCompleted)(windows_core::Interface::as_raw(self), iob, lpstg.param().abi()).ok() }
    }
    pub unsafe fn InPlaceDeactivate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InPlaceDeactivate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ContextSensitiveHelp(&self, fentermode: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ContextSensitiveHelp)(windows_core::Interface::as_raw(self), fentermode.into()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut Option<super::super::super::System::Com::IDataObject>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClipboardData)(windows_core::Interface::as_raw(self), lpchrg as _, reco, core::mem::transmute(lplpdataobj)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportDataObject<P0>(&self, lpdataobj: P0, cf: u16, hmetapict: super::super::super::Foundation::HGLOBAL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).ImportDataObject)(windows_core::Interface::as_raw(self), lpdataobj.param().abi(), cf, hmetapict).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditOle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetClientSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetClientSite: usize,
    pub GetObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
    pub GetLinkCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut REOBJECT, RICH_EDIT_GET_OBJECT_FLAGS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole")))]
    GetObject: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub InsertObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut REOBJECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole")))]
    InsertObject: usize,
    pub ConvertObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, windows_core::PCSTR) -> windows_core::HRESULT,
    pub ActivateAs: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetHostNames: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR) -> windows_core::HRESULT,
    pub SetLinkAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetDvaspect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
    pub HandsOffStorage: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SaveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SaveCompleted: usize,
    pub InPlaceDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContextSensitiveHelp: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClipboardData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CHARRANGE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClipboardData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ImportDataObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u16, super::super::super::Foundation::HGLOBAL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImportDataObject: usize,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait IRichEditOle_Impl: windows_core::IUnknownImpl {
    fn GetClientSite(&self) -> windows_core::Result<super::super::super::System::Ole::IOleClientSite>;
    fn GetObjectCount(&self) -> i32;
    fn GetLinkCount(&self) -> i32;
    fn GetObject(&self, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> windows_core::Result<()>;
    fn InsertObject(&self, lpreobject: *mut REOBJECT) -> windows_core::Result<()>;
    fn ConvertObject(&self, iob: i32, rclsidnew: *const windows_core::GUID, lpstrusertypenew: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn ActivateAs(&self, rclsid: *const windows_core::GUID, rclsidas: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetHostNames(&self, lpstrcontainerapp: &windows_core::PCSTR, lpstrcontainerobj: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetLinkAvailable(&self, iob: i32, favailable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetDvaspect(&self, iob: i32, dvaspect: u32) -> windows_core::Result<()>;
    fn HandsOffStorage(&self, iob: i32) -> windows_core::Result<()>;
    fn SaveCompleted(&self, iob: i32, lpstg: windows_core::Ref<'_, super::super::super::System::Com::StructuredStorage::IStorage>) -> windows_core::Result<()>;
    fn InPlaceDeactivate(&self) -> windows_core::Result<()>;
    fn ContextSensitiveHelp(&self, fentermode: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: windows_core::OutRef<'_, super::super::super::System::Com::IDataObject>) -> windows_core::Result<()>;
    fn ImportDataObject(&self, lpdataobj: windows_core::Ref<'_, super::super::super::System::Com::IDataObject>, cf: u16, hmetapict: super::super::super::Foundation::HGLOBAL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl IRichEditOle_Vtbl {
    pub const fn new<Identity: IRichEditOle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientSite<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lplpolesite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRichEditOle_Impl::GetClientSite(this) {
                    Ok(ok__) => {
                        lplpolesite.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectCount<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::GetObjectCount(this)
            }
        }
        unsafe extern "system" fn GetLinkCount<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::GetLinkCount(this)
            }
        }
        unsafe extern "system" fn GetObject<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::GetObject(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&lpreobject), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn InsertObject<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpreobject: *mut REOBJECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::InsertObject(this, core::mem::transmute_copy(&lpreobject)).into()
            }
        }
        unsafe extern "system" fn ConvertObject<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, rclsidnew: *const windows_core::GUID, lpstrusertypenew: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::ConvertObject(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&rclsidnew), core::mem::transmute(&lpstrusertypenew)).into()
            }
        }
        unsafe extern "system" fn ActivateAs<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rclsidas: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::ActivateAs(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rclsidas)).into()
            }
        }
        unsafe extern "system" fn SetHostNames<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpstrcontainerapp: windows_core::PCSTR, lpstrcontainerobj: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::SetHostNames(this, core::mem::transmute(&lpstrcontainerapp), core::mem::transmute(&lpstrcontainerobj)).into()
            }
        }
        unsafe extern "system" fn SetLinkAvailable<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, favailable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::SetLinkAvailable(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&favailable)).into()
            }
        }
        unsafe extern "system" fn SetDvaspect<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, dvaspect: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::SetDvaspect(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&dvaspect)).into()
            }
        }
        unsafe extern "system" fn HandsOffStorage<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::HandsOffStorage(this, core::mem::transmute_copy(&iob)).into()
            }
        }
        unsafe extern "system" fn SaveCompleted<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iob: i32, lpstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::SaveCompleted(this, core::mem::transmute_copy(&iob), core::mem::transmute_copy(&lpstg)).into()
            }
        }
        unsafe extern "system" fn InPlaceDeactivate<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::InPlaceDeactivate(this).into()
            }
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fentermode: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::ContextSensitiveHelp(this, core::mem::transmute_copy(&fentermode)).into()
            }
        }
        unsafe extern "system" fn GetClipboardData<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::GetClipboardData(this, core::mem::transmute_copy(&lpchrg), core::mem::transmute_copy(&reco), core::mem::transmute_copy(&lplpdataobj)).into()
            }
        }
        unsafe extern "system" fn ImportDataObject<Identity: IRichEditOle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobj: *mut core::ffi::c_void, cf: u16, hmetapict: super::super::super::Foundation::HGLOBAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOle_Impl::ImportDataObject(this, core::mem::transmute_copy(&lpdataobj), core::mem::transmute_copy(&cf), core::mem::transmute_copy(&hmetapict)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientSite: GetClientSite::<Identity, OFFSET>,
            GetObjectCount: GetObjectCount::<Identity, OFFSET>,
            GetLinkCount: GetLinkCount::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            InsertObject: InsertObject::<Identity, OFFSET>,
            ConvertObject: ConvertObject::<Identity, OFFSET>,
            ActivateAs: ActivateAs::<Identity, OFFSET>,
            SetHostNames: SetHostNames::<Identity, OFFSET>,
            SetLinkAvailable: SetLinkAvailable::<Identity, OFFSET>,
            SetDvaspect: SetDvaspect::<Identity, OFFSET>,
            HandsOffStorage: HandsOffStorage::<Identity, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, OFFSET>,
            InPlaceDeactivate: InPlaceDeactivate::<Identity, OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Identity, OFFSET>,
            GetClipboardData: GetClipboardData::<Identity, OFFSET>,
            ImportDataObject: ImportDataObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRichEditOle as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IRichEditOle {}
windows_core::imp::define_interface!(IRichEditOleCallback, IRichEditOleCallback_Vtbl, 0x00020d03_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRichEditOleCallback, windows_core::IUnknown);
impl IRichEditOleCallback {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetNewStorage(&self) -> windows_core::Result<super::super::super::System::Com::StructuredStorage::IStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNewStorage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetInPlaceContext(&self, lplpframe: *mut Option<super::super::super::System::Ole::IOleInPlaceFrame>, lplpdoc: *mut Option<super::super::super::System::Ole::IOleInPlaceUIWindow>, lpframeinfo: *mut super::super::super::System::Ole::OLEINPLACEFRAMEINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInPlaceContext)(windows_core::Interface::as_raw(self), core::mem::transmute(lplpframe), core::mem::transmute(lplpdoc), lpframeinfo as _).ok() }
    }
    pub unsafe fn ShowContainerUI(&self, fshow: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShowContainerUI)(windows_core::Interface::as_raw(self), fshow.into()).ok() }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn QueryInsertObject<P1>(&self, lpclsid: *mut windows_core::GUID, lpstg: P1, cp: i32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::super::System::Com::StructuredStorage::IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryInsertObject)(windows_core::Interface::as_raw(self), lpclsid as _, lpstg.param().abi(), cp).ok() }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn DeleteObject<P0>(&self, lpoleobj: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Ole::IOleObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteObject)(windows_core::Interface::as_raw(self), lpoleobj.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub unsafe fn QueryAcceptData<P0>(&self, lpdataobj: P0, lpcfformat: *mut u16, reco: super::super::super::System::SystemServices::RECO_FLAGS, freally: bool, hmetapict: super::super::super::Foundation::HGLOBAL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryAcceptData)(windows_core::Interface::as_raw(self), lpdataobj.param().abi(), lpcfformat as _, reco, freally.into(), hmetapict).ok() }
    }
    pub unsafe fn ContextSensitiveHelp(&self, fentermode: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ContextSensitiveHelp)(windows_core::Interface::as_raw(self), fentermode.into()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut Option<super::super::super::System::Com::IDataObject>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClipboardData)(windows_core::Interface::as_raw(self), lpchrg as _, reco, core::mem::transmute(lplpdataobj)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetDragDropEffect(&self, fdrag: bool, grfkeystate: super::super::super::System::SystemServices::MODIFIERKEYS_FLAGS, pdweffect: *mut super::super::super::System::Ole::DROPEFFECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDragDropEffect)(windows_core::Interface::as_raw(self), fdrag.into(), grfkeystate, pdweffect as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetContextMenu<P1>(&self, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: P1, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::super::System::Ole::IOleObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetContextMenu)(windows_core::Interface::as_raw(self), seltype, lpoleobj.param().abi(), lpchrg as _, lphmenu as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditOleCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetNewStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetNewStorage: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetInPlaceContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::super::super::System::Ole::OLEINPLACEFRAMEINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging")))]
    GetInPlaceContext: usize,
    pub ShowContainerUI: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub QueryInsertObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    QueryInsertObject: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub DeleteObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    DeleteObject: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub QueryAcceptData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u16, super::super::super::System::SystemServices::RECO_FLAGS, windows_core::BOOL, super::super::super::Foundation::HGLOBAL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices")))]
    QueryAcceptData: usize,
    pub ContextSensitiveHelp: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClipboardData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CHARRANGE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClipboardData: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_SystemServices"))]
    pub GetDragDropEffect: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, super::super::super::System::SystemServices::MODIFIERKEYS_FLAGS, *mut super::super::super::System::Ole::DROPEFFECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_SystemServices")))]
    GetDragDropEffect: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, *mut core::ffi::c_void, *mut CHARRANGE, *mut super::super::WindowsAndMessaging::HMENU) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging")))]
    GetContextMenu: usize,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_SystemServices", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IRichEditOleCallback_Impl: windows_core::IUnknownImpl {
    fn GetNewStorage(&self) -> windows_core::Result<super::super::super::System::Com::StructuredStorage::IStorage>;
    fn GetInPlaceContext(&self, lplpframe: windows_core::OutRef<'_, super::super::super::System::Ole::IOleInPlaceFrame>, lplpdoc: windows_core::OutRef<'_, super::super::super::System::Ole::IOleInPlaceUIWindow>, lpframeinfo: *mut super::super::super::System::Ole::OLEINPLACEFRAMEINFO) -> windows_core::Result<()>;
    fn ShowContainerUI(&self, fshow: windows_core::BOOL) -> windows_core::Result<()>;
    fn QueryInsertObject(&self, lpclsid: *mut windows_core::GUID, lpstg: windows_core::Ref<'_, super::super::super::System::Com::StructuredStorage::IStorage>, cp: i32) -> windows_core::Result<()>;
    fn DeleteObject(&self, lpoleobj: windows_core::Ref<'_, super::super::super::System::Ole::IOleObject>) -> windows_core::Result<()>;
    fn QueryAcceptData(&self, lpdataobj: windows_core::Ref<'_, super::super::super::System::Com::IDataObject>, lpcfformat: *mut u16, reco: super::super::super::System::SystemServices::RECO_FLAGS, freally: windows_core::BOOL, hmetapict: super::super::super::Foundation::HGLOBAL) -> windows_core::Result<()>;
    fn ContextSensitiveHelp(&self, fentermode: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: windows_core::OutRef<'_, super::super::super::System::Com::IDataObject>) -> windows_core::Result<()>;
    fn GetDragDropEffect(&self, fdrag: windows_core::BOOL, grfkeystate: super::super::super::System::SystemServices::MODIFIERKEYS_FLAGS, pdweffect: *mut super::super::super::System::Ole::DROPEFFECT) -> windows_core::Result<()>;
    fn GetContextMenu(&self, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: windows_core::Ref<'_, super::super::super::System::Ole::IOleObject>, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_SystemServices", feature = "Win32_UI_WindowsAndMessaging"))]
impl IRichEditOleCallback_Vtbl {
    pub const fn new<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNewStorage<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lplpstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRichEditOleCallback_Impl::GetNewStorage(this) {
                    Ok(ok__) => {
                        lplpstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInPlaceContext<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lplpframe: *mut *mut core::ffi::c_void, lplpdoc: *mut *mut core::ffi::c_void, lpframeinfo: *mut super::super::super::System::Ole::OLEINPLACEFRAMEINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::GetInPlaceContext(this, core::mem::transmute_copy(&lplpframe), core::mem::transmute_copy(&lplpdoc), core::mem::transmute_copy(&lpframeinfo)).into()
            }
        }
        unsafe extern "system" fn ShowContainerUI<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::ShowContainerUI(this, core::mem::transmute_copy(&fshow)).into()
            }
        }
        unsafe extern "system" fn QueryInsertObject<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpclsid: *mut windows_core::GUID, lpstg: *mut core::ffi::c_void, cp: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::QueryInsertObject(this, core::mem::transmute_copy(&lpclsid), core::mem::transmute_copy(&lpstg), core::mem::transmute_copy(&cp)).into()
            }
        }
        unsafe extern "system" fn DeleteObject<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpoleobj: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::DeleteObject(this, core::mem::transmute_copy(&lpoleobj)).into()
            }
        }
        unsafe extern "system" fn QueryAcceptData<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdataobj: *mut core::ffi::c_void, lpcfformat: *mut u16, reco: super::super::super::System::SystemServices::RECO_FLAGS, freally: windows_core::BOOL, hmetapict: super::super::super::Foundation::HGLOBAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::QueryAcceptData(this, core::mem::transmute_copy(&lpdataobj), core::mem::transmute_copy(&lpcfformat), core::mem::transmute_copy(&reco), core::mem::transmute_copy(&freally), core::mem::transmute_copy(&hmetapict)).into()
            }
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fentermode: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::ContextSensitiveHelp(this, core::mem::transmute_copy(&fentermode)).into()
            }
        }
        unsafe extern "system" fn GetClipboardData<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::GetClipboardData(this, core::mem::transmute_copy(&lpchrg), core::mem::transmute_copy(&reco), core::mem::transmute_copy(&lplpdataobj)).into()
            }
        }
        unsafe extern "system" fn GetDragDropEffect<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdrag: windows_core::BOOL, grfkeystate: super::super::super::System::SystemServices::MODIFIERKEYS_FLAGS, pdweffect: *mut super::super::super::System::Ole::DROPEFFECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::GetDragDropEffect(this, core::mem::transmute_copy(&fdrag), core::mem::transmute_copy(&grfkeystate), core::mem::transmute_copy(&pdweffect)).into()
            }
        }
        unsafe extern "system" fn GetContextMenu<Identity: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: *mut core::ffi::c_void, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditOleCallback_Impl::GetContextMenu(this, core::mem::transmute_copy(&seltype), core::mem::transmute_copy(&lpoleobj), core::mem::transmute_copy(&lpchrg), core::mem::transmute_copy(&lphmenu)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNewStorage: GetNewStorage::<Identity, OFFSET>,
            GetInPlaceContext: GetInPlaceContext::<Identity, OFFSET>,
            ShowContainerUI: ShowContainerUI::<Identity, OFFSET>,
            QueryInsertObject: QueryInsertObject::<Identity, OFFSET>,
            DeleteObject: DeleteObject::<Identity, OFFSET>,
            QueryAcceptData: QueryAcceptData::<Identity, OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Identity, OFFSET>,
            GetClipboardData: GetClipboardData::<Identity, OFFSET>,
            GetDragDropEffect: GetDragDropEffect::<Identity, OFFSET>,
            GetContextMenu: GetContextMenu::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRichEditOleCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_SystemServices", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IRichEditOleCallback {}
windows_core::imp::define_interface!(IRicheditUiaOverrides, IRicheditUiaOverrides_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IRicheditUiaOverrides, windows_core::IUnknown);
impl IRicheditUiaOverrides {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetPropertyOverrideValue(&self, propertyid: i32, pretvalue: *mut super::super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyOverrideValue)(windows_core::Interface::as_raw(self), propertyid, core::mem::transmute(pretvalue)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRicheditUiaOverrides_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetPropertyOverrideValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetPropertyOverrideValue: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRicheditUiaOverrides_Impl: windows_core::IUnknownImpl {
    fn GetPropertyOverrideValue(&self, propertyid: i32, pretvalue: *mut super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRicheditUiaOverrides_Vtbl {
    pub const fn new<Identity: IRicheditUiaOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyOverrideValue<Identity: IRicheditUiaOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: i32, pretvalue: *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRicheditUiaOverrides_Impl::GetPropertyOverrideValue(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&pretvalue)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPropertyOverrideValue: GetPropertyOverrideValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRicheditUiaOverrides as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRicheditUiaOverrides {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextDisplays, ITextDisplays_Vtbl, 0xc241f5f2_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextDisplays {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextDisplays, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextDisplays_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextDisplays_Impl: super::super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextDisplays_Vtbl {
    pub const fn new<Identity: ITextDisplays_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDisplays as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextDisplays {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextDocument, ITextDocument_Vtbl, 0x8cc497c0_a1df_11ce_8098_00aa0047be5d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextDocument {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextDocument, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITextDocument {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSelection(&self) -> windows_core::Result<ITextSelection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStoryCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStoryCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStoryRanges(&self) -> windows_core::Result<ITextStoryRanges> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStoryRanges)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSaved(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSaved)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSaved(&self, value: tomConstants) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSaved)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetDefaultTabStop(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultTabStop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefaultTabStop(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultTabStop)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn New(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).New)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Open(&self, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), core::mem::transmute(pvar), flags, codepage).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Save(&self, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), core::mem::transmute(pvar), flags, codepage).ok() }
    }
    pub unsafe fn Freeze(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Freeze)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unfreeze(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Unfreeze)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginEditCollection(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginEditCollection)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EndEditCollection(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndEditCollection)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Undo(&self, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Undo)(windows_core::Interface::as_raw(self), count, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Redo(&self, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Redo)(windows_core::Interface::as_raw(self), count, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Range(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Range)(windows_core::Interface::as_raw(self), cpactive, cpanchor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RangeFromPoint(&self, x: i32, y: i32) -> windows_core::Result<ITextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeFromPoint)(windows_core::Interface::as_raw(self), x, y, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStoryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetStoryRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSaved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSaved: unsafe extern "system" fn(*mut core::ffi::c_void, tomConstants) -> windows_core::HRESULT,
    pub GetDefaultTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDefaultTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub New: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::System::Variant::VARIANT, tomConstants, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Open: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::System::Variant::VARIANT, tomConstants, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Save: usize,
    pub Freeze: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Unfreeze: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BeginEditCollection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndEditCollection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Undo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Redo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Range: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RangeFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextDocument_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSelection(&self) -> windows_core::Result<ITextSelection>;
    fn GetStoryCount(&self) -> windows_core::Result<i32>;
    fn GetStoryRanges(&self) -> windows_core::Result<ITextStoryRanges>;
    fn GetSaved(&self) -> windows_core::Result<i32>;
    fn SetSaved(&self, value: tomConstants) -> windows_core::Result<()>;
    fn GetDefaultTabStop(&self) -> windows_core::Result<f32>;
    fn SetDefaultTabStop(&self, value: f32) -> windows_core::Result<()>;
    fn New(&self) -> windows_core::Result<()>;
    fn Open(&self, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> windows_core::Result<()>;
    fn Save(&self, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> windows_core::Result<()>;
    fn Freeze(&self) -> windows_core::Result<i32>;
    fn Unfreeze(&self) -> windows_core::Result<i32>;
    fn BeginEditCollection(&self) -> windows_core::Result<()>;
    fn EndEditCollection(&self) -> windows_core::Result<()>;
    fn Undo(&self, count: i32) -> windows_core::Result<i32>;
    fn Redo(&self, count: i32) -> windows_core::Result<i32>;
    fn Range(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange>;
    fn RangeFromPoint(&self, x: i32, y: i32) -> windows_core::Result<ITextRange>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextDocument_Vtbl {
    pub const fn new<Identity: ITextDocument_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetName(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetSelection(this) {
                    Ok(ok__) => {
                        ppsel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStoryCount<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetStoryCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStoryRanges<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstories: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetStoryRanges(this) {
                    Ok(ok__) => {
                        ppstories.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSaved<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetSaved(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSaved<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: tomConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::SetSaved(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetDefaultTabStop<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetDefaultTabStop(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultTabStop<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::SetDefaultTabStop(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn New<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::New(this).into()
            }
        }
        unsafe extern "system" fn Open<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::Open(this, core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&codepage)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::Save(this, core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&codepage)).into()
            }
        }
        unsafe extern "system" fn Freeze<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::Freeze(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unfreeze<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::Unfreeze(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginEditCollection<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::BeginEditCollection(this).into()
            }
        }
        unsafe extern "system" fn EndEditCollection<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::EndEditCollection(this).into()
            }
        }
        unsafe extern "system" fn Undo<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::Undo(this, core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Redo<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::Redo(this, core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Range<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::Range(this, core::mem::transmute_copy(&cpactive), core::mem::transmute_copy(&cpanchor)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RangeFromPoint<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::RangeFromPoint(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            GetStoryCount: GetStoryCount::<Identity, OFFSET>,
            GetStoryRanges: GetStoryRanges::<Identity, OFFSET>,
            GetSaved: GetSaved::<Identity, OFFSET>,
            SetSaved: SetSaved::<Identity, OFFSET>,
            GetDefaultTabStop: GetDefaultTabStop::<Identity, OFFSET>,
            SetDefaultTabStop: SetDefaultTabStop::<Identity, OFFSET>,
            New: New::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            Freeze: Freeze::<Identity, OFFSET>,
            Unfreeze: Unfreeze::<Identity, OFFSET>,
            BeginEditCollection: BeginEditCollection::<Identity, OFFSET>,
            EndEditCollection: EndEditCollection::<Identity, OFFSET>,
            Undo: Undo::<Identity, OFFSET>,
            Redo: Redo::<Identity, OFFSET>,
            Range: Range::<Identity, OFFSET>,
            RangeFromPoint: RangeFromPoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDocument as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextDocument {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextDocument2, ITextDocument2_Vtbl, 0xc241f5e0_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextDocument2 {
    type Target = ITextDocument;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextDocument2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextDocument);
#[cfg(feature = "Win32_System_Com")]
impl ITextDocument2 {
    pub unsafe fn GetCaretType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaretType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCaretType(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCaretType)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetDisplays(&self) -> windows_core::Result<ITextDisplays> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplays)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDocumentFont(&self) -> windows_core::Result<ITextFont2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentFont)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDocumentFont<P0>(&self, pfont: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextFont2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDocumentFont)(windows_core::Interface::as_raw(self), pfont.param().abi()).ok() }
    }
    pub unsafe fn GetDocumentPara(&self) -> windows_core::Result<ITextPara2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentPara)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDocumentPara<P0>(&self, ppara: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextPara2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDocumentPara)(windows_core::Interface::as_raw(self), ppara.param().abi()).ok() }
    }
    pub unsafe fn GetEastAsianFlags(&self) -> windows_core::Result<tomConstants> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEastAsianFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGenerator(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGenerator)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetIMEInProgress(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIMEInProgress)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetNotificationMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNotificationMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNotificationMode(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNotificationMode)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetSelection2(&self) -> windows_core::Result<ITextSelection2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStoryRanges2(&self) -> windows_core::Result<ITextStoryRanges2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStoryRanges2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTypographyOptions(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypographyOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetWindow(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AttachMsgFilter<P0>(&self, pfilter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AttachMsgFilter)(windows_core::Interface::as_raw(self), pfilter.param().abi()).ok() }
    }
    pub unsafe fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckTextLimit)(windows_core::Interface::as_raw(self), cch, pcch).ok() }
    }
    pub unsafe fn GetCallManager(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCallManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetClientRect(&self, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClientRect)(windows_core::Interface::as_raw(self), r#type, pleft as _, ptop as _, pright as _, pbottom as _).ok() }
    }
    pub unsafe fn GetEffectColor(&self, index: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEffectColor)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetImmContext(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImmContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPreferredFont(&self, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut windows_core::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPreferredFont)(windows_core::Interface::as_raw(self), cp, charrep, options, curcharrep, curfontsize, core::mem::transmute(pbstr), ppitchandfamily as _, pnewfontsize as _).ok() }
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStrings(&self) -> windows_core::Result<ITextStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Notify(&self, notify: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), notify).ok() }
    }
    pub unsafe fn Range2(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Range2)(windows_core::Interface::as_raw(self), cpactive, cpanchor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RangeFromPoint2(&self, x: i32, y: i32, r#type: i32) -> windows_core::Result<ITextRange2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeFromPoint2)(windows_core::Interface::as_raw(self), x, y, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReleaseCallManager<P0>(&self, pvoid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReleaseCallManager)(windows_core::Interface::as_raw(self), pvoid.param().abi()).ok() }
    }
    pub unsafe fn ReleaseImmContext(&self, context: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseImmContext)(windows_core::Interface::as_raw(self), context).ok() }
    }
    pub unsafe fn SetEffectColor(&self, index: i32, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEffectColor)(windows_core::Interface::as_raw(self), index, value).ok() }
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), r#type, value).ok() }
    }
    pub unsafe fn SetTypographyOptions(&self, options: i32, mask: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTypographyOptions)(windows_core::Interface::as_raw(self), options, mask).ok() }
    }
    pub unsafe fn SysBeep(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SysBeep)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Update(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn UpdateWindow(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateWindow)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetMathProperties(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMathProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMathProperties(&self, options: i32, mask: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMathProperties)(windows_core::Interface::as_raw(self), options, mask).ok() }
    }
    pub unsafe fn GetActiveStory(&self) -> windows_core::Result<ITextStory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveStory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetActiveStory<P0>(&self, pstory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextStory>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActiveStory)(windows_core::Interface::as_raw(self), pstory.param().abi()).ok() }
    }
    pub unsafe fn GetMainStory(&self) -> windows_core::Result<ITextStory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMainStory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNewStory(&self) -> windows_core::Result<ITextStory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNewStory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStory(&self, index: i32) -> windows_core::Result<ITextStory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStory)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument2_Vtbl {
    pub base__: ITextDocument_Vtbl,
    pub GetCaretType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCaretType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDisplays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDocumentFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentPara: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDocumentPara: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEastAsianFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut tomConstants) -> windows_core::HRESULT,
    pub GetGenerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIMEInProgress: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSelection2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStoryRanges2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTypographyOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub AttachMsgFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckTextLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const i32) -> windows_core::HRESULT,
    pub GetCallManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClientRect: unsafe extern "system" fn(*mut core::ffi::c_void, tomConstants, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetEffectColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetImmContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub GetPreferredFont: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, i32, *mut *mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetStrings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Range2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RangeFromPoint2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseCallManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseImmContext: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SetEffectColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetTypographyOptions: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SysBeep: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub UpdateWindow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMathProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMathProperties: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetActiveStory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActiveStory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMainStory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNewStory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStory: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextDocument2_Impl: ITextDocument_Impl {
    fn GetCaretType(&self) -> windows_core::Result<i32>;
    fn SetCaretType(&self, value: i32) -> windows_core::Result<()>;
    fn GetDisplays(&self) -> windows_core::Result<ITextDisplays>;
    fn GetDocumentFont(&self) -> windows_core::Result<ITextFont2>;
    fn SetDocumentFont(&self, pfont: windows_core::Ref<'_, ITextFont2>) -> windows_core::Result<()>;
    fn GetDocumentPara(&self) -> windows_core::Result<ITextPara2>;
    fn SetDocumentPara(&self, ppara: windows_core::Ref<'_, ITextPara2>) -> windows_core::Result<()>;
    fn GetEastAsianFlags(&self) -> windows_core::Result<tomConstants>;
    fn GetGenerator(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetIMEInProgress(&self, value: i32) -> windows_core::Result<()>;
    fn GetNotificationMode(&self) -> windows_core::Result<i32>;
    fn SetNotificationMode(&self, value: i32) -> windows_core::Result<()>;
    fn GetSelection2(&self) -> windows_core::Result<ITextSelection2>;
    fn GetStoryRanges2(&self) -> windows_core::Result<ITextStoryRanges2>;
    fn GetTypographyOptions(&self) -> windows_core::Result<i32>;
    fn GetVersion(&self) -> windows_core::Result<i32>;
    fn GetWindow(&self) -> windows_core::Result<i64>;
    fn AttachMsgFilter(&self, pfilter: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> windows_core::Result<()>;
    fn GetCallManager(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetClientRect(&self, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::Result<()>;
    fn GetEffectColor(&self, index: i32) -> windows_core::Result<i32>;
    fn GetImmContext(&self) -> windows_core::Result<i64>;
    fn GetPreferredFont(&self, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut windows_core::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn GetStrings(&self) -> windows_core::Result<ITextStrings>;
    fn Notify(&self, notify: i32) -> windows_core::Result<()>;
    fn Range2(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange2>;
    fn RangeFromPoint2(&self, x: i32, y: i32, r#type: i32) -> windows_core::Result<ITextRange2>;
    fn ReleaseCallManager(&self, pvoid: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ReleaseImmContext(&self, context: i64) -> windows_core::Result<()>;
    fn SetEffectColor(&self, index: i32, value: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
    fn SetTypographyOptions(&self, options: i32, mask: i32) -> windows_core::Result<()>;
    fn SysBeep(&self) -> windows_core::Result<()>;
    fn Update(&self, value: i32) -> windows_core::Result<()>;
    fn UpdateWindow(&self) -> windows_core::Result<()>;
    fn GetMathProperties(&self) -> windows_core::Result<i32>;
    fn SetMathProperties(&self, options: i32, mask: i32) -> windows_core::Result<()>;
    fn GetActiveStory(&self) -> windows_core::Result<ITextStory>;
    fn SetActiveStory(&self, pstory: windows_core::Ref<'_, ITextStory>) -> windows_core::Result<()>;
    fn GetMainStory(&self) -> windows_core::Result<ITextStory>;
    fn GetNewStory(&self) -> windows_core::Result<ITextStory>;
    fn GetStory(&self, index: i32) -> windows_core::Result<ITextStory>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextDocument2_Vtbl {
    pub const fn new<Identity: ITextDocument2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCaretType<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetCaretType(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCaretType<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetCaretType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetDisplays<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisplays: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetDisplays(this) {
                    Ok(ok__) => {
                        ppdisplays.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentFont<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetDocumentFont(this) {
                    Ok(ok__) => {
                        ppfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDocumentFont<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetDocumentFont(this, core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn GetDocumentPara<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetDocumentPara(this) {
                    Ok(ok__) => {
                        pppara.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDocumentPara<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetDocumentPara(this, core::mem::transmute_copy(&ppara)).into()
            }
        }
        unsafe extern "system" fn GetEastAsianFlags<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut tomConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetEastAsianFlags(this) {
                    Ok(ok__) => {
                        pflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGenerator<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetGenerator(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIMEInProgress<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetIMEInProgress(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetNotificationMode<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetNotificationMode(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotificationMode<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetNotificationMode(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSelection2<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetSelection2(this) {
                    Ok(ok__) => {
                        ppsel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStoryRanges2<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstories: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetStoryRanges2(this) {
                    Ok(ok__) => {
                        ppstories.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypographyOptions<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poptions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetTypographyOptions(this) {
                    Ok(ok__) => {
                        poptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVersion<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetVersion(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWindow<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetWindow(this) {
                    Ok(ok__) => {
                        phwnd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AttachMsgFilter<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::AttachMsgFilter(this, core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn CheckTextLimit<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: i32, pcch: *const i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::CheckTextLimit(this, core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pcch)).into()
            }
        }
        unsafe extern "system" fn GetCallManager<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvoid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetCallManager(this) {
                    Ok(ok__) => {
                        ppvoid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClientRect<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::GetClientRect(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pright), core::mem::transmute_copy(&pbottom)).into()
            }
        }
        unsafe extern "system" fn GetEffectColor<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetEffectColor(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetImmContext<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetImmContext(this) {
                    Ok(ok__) => {
                        pcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreferredFont<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut *mut core::ffi::c_void, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::GetPreferredFont(this, core::mem::transmute_copy(&cp), core::mem::transmute_copy(&charrep), core::mem::transmute_copy(&options), core::mem::transmute_copy(&curcharrep), core::mem::transmute_copy(&curfontsize), core::mem::transmute_copy(&pbstr), core::mem::transmute_copy(&ppitchandfamily), core::mem::transmute_copy(&pnewfontsize)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStrings<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetStrings(this) {
                    Ok(ok__) => {
                        ppstrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Notify<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notify: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::Notify(this, core::mem::transmute_copy(&notify)).into()
            }
        }
        unsafe extern "system" fn Range2<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::Range2(this, core::mem::transmute_copy(&cpactive), core::mem::transmute_copy(&cpanchor)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RangeFromPoint2<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, r#type: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::RangeFromPoint2(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseCallManager<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvoid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::ReleaseCallManager(this, core::mem::transmute_copy(&pvoid)).into()
            }
        }
        unsafe extern "system" fn ReleaseImmContext<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::ReleaseImmContext(this, core::mem::transmute_copy(&context)).into()
            }
        }
        unsafe extern "system" fn SetEffectColor<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetEffectColor(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetTypographyOptions<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: i32, mask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetTypographyOptions(this, core::mem::transmute_copy(&options), core::mem::transmute_copy(&mask)).into()
            }
        }
        unsafe extern "system" fn SysBeep<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SysBeep(this).into()
            }
        }
        unsafe extern "system" fn Update<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::Update(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn UpdateWindow<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::UpdateWindow(this).into()
            }
        }
        unsafe extern "system" fn GetMathProperties<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poptions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetMathProperties(this) {
                    Ok(ok__) => {
                        poptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMathProperties<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: i32, mask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetMathProperties(this, core::mem::transmute_copy(&options), core::mem::transmute_copy(&mask)).into()
            }
        }
        unsafe extern "system" fn GetActiveStory<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetActiveStory(this) {
                    Ok(ok__) => {
                        ppstory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetActiveStory<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetActiveStory(this, core::mem::transmute_copy(&pstory)).into()
            }
        }
        unsafe extern "system" fn GetMainStory<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetMainStory(this) {
                    Ok(ok__) => {
                        ppstory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNewStory<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetNewStory(this) {
                    Ok(ok__) => {
                        ppstory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStory<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, ppstory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetStory(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppstory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ITextDocument_Vtbl::new::<Identity, OFFSET>(),
            GetCaretType: GetCaretType::<Identity, OFFSET>,
            SetCaretType: SetCaretType::<Identity, OFFSET>,
            GetDisplays: GetDisplays::<Identity, OFFSET>,
            GetDocumentFont: GetDocumentFont::<Identity, OFFSET>,
            SetDocumentFont: SetDocumentFont::<Identity, OFFSET>,
            GetDocumentPara: GetDocumentPara::<Identity, OFFSET>,
            SetDocumentPara: SetDocumentPara::<Identity, OFFSET>,
            GetEastAsianFlags: GetEastAsianFlags::<Identity, OFFSET>,
            GetGenerator: GetGenerator::<Identity, OFFSET>,
            SetIMEInProgress: SetIMEInProgress::<Identity, OFFSET>,
            GetNotificationMode: GetNotificationMode::<Identity, OFFSET>,
            SetNotificationMode: SetNotificationMode::<Identity, OFFSET>,
            GetSelection2: GetSelection2::<Identity, OFFSET>,
            GetStoryRanges2: GetStoryRanges2::<Identity, OFFSET>,
            GetTypographyOptions: GetTypographyOptions::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetWindow: GetWindow::<Identity, OFFSET>,
            AttachMsgFilter: AttachMsgFilter::<Identity, OFFSET>,
            CheckTextLimit: CheckTextLimit::<Identity, OFFSET>,
            GetCallManager: GetCallManager::<Identity, OFFSET>,
            GetClientRect: GetClientRect::<Identity, OFFSET>,
            GetEffectColor: GetEffectColor::<Identity, OFFSET>,
            GetImmContext: GetImmContext::<Identity, OFFSET>,
            GetPreferredFont: GetPreferredFont::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetStrings: GetStrings::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
            Range2: Range2::<Identity, OFFSET>,
            RangeFromPoint2: RangeFromPoint2::<Identity, OFFSET>,
            ReleaseCallManager: ReleaseCallManager::<Identity, OFFSET>,
            ReleaseImmContext: ReleaseImmContext::<Identity, OFFSET>,
            SetEffectColor: SetEffectColor::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            SetTypographyOptions: SetTypographyOptions::<Identity, OFFSET>,
            SysBeep: SysBeep::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            UpdateWindow: UpdateWindow::<Identity, OFFSET>,
            GetMathProperties: GetMathProperties::<Identity, OFFSET>,
            SetMathProperties: SetMathProperties::<Identity, OFFSET>,
            GetActiveStory: GetActiveStory::<Identity, OFFSET>,
            SetActiveStory: SetActiveStory::<Identity, OFFSET>,
            GetMainStory: GetMainStory::<Identity, OFFSET>,
            GetNewStory: GetNewStory::<Identity, OFFSET>,
            GetStory: GetStory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDocument2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextDocument as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextDocument2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextDocument2Old, ITextDocument2Old_Vtbl, 0x01c25500_4268_11d1_883a_3c8b00c10000);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextDocument2Old {
    type Target = ITextDocument;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextDocument2Old, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextDocument);
#[cfg(feature = "Win32_System_Com")]
impl ITextDocument2Old {
    pub unsafe fn AttachMsgFilter<P0>(&self, pfilter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AttachMsgFilter)(windows_core::Interface::as_raw(self), pfilter.param().abi()).ok() }
    }
    pub unsafe fn SetEffectColor(&self, index: i32, cr: super::super::super::Foundation::COLORREF) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEffectColor)(windows_core::Interface::as_raw(self), index, cr).ok() }
    }
    pub unsafe fn GetEffectColor(&self, index: i32) -> windows_core::Result<super::super::super::Foundation::COLORREF> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEffectColor)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCaretType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaretType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCaretType(&self, carettype: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCaretType)(windows_core::Interface::as_raw(self), carettype).ok() }
    }
    pub unsafe fn GetImmContext(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImmContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReleaseImmContext(&self, context: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseImmContext)(windows_core::Interface::as_raw(self), context).ok() }
    }
    pub unsafe fn GetPreferredFont(&self, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut windows_core::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPreferredFont)(windows_core::Interface::as_raw(self), cp, charrep, option, charrepcur, curfontsize, core::mem::transmute(pbstr), ppitchandfamily as _, pnewfontsize as _).ok() }
    }
    pub unsafe fn GetNotificationMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNotificationMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNotificationMode(&self, mode: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNotificationMode)(windows_core::Interface::as_raw(self), mode).ok() }
    }
    pub unsafe fn GetClientRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClientRect)(windows_core::Interface::as_raw(self), r#type, pleft as _, ptop as _, pright as _, pbottom as _).ok() }
    }
    pub unsafe fn GetSelection2(&self) -> windows_core::Result<ITextSelection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWindow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFEFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFEFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UpdateWindow(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateWindow)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckTextLimit)(windows_core::Interface::as_raw(self), cch, pcch).ok() }
    }
    pub unsafe fn IMEInProgress(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IMEInProgress)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn SysBeep(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SysBeep)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Update(&self, mode: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), mode).ok() }
    }
    pub unsafe fn Notify(&self, notify: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), notify).ok() }
    }
    pub unsafe fn GetDocumentFont(&self) -> windows_core::Result<ITextFont> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentFont)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDocumentPara(&self) -> windows_core::Result<ITextPara> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentPara)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCallManager(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCallManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReleaseCallManager<P0>(&self, pvoid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReleaseCallManager)(windows_core::Interface::as_raw(self), pvoid.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument2Old_Vtbl {
    pub base__: ITextDocument_Vtbl,
    pub AttachMsgFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEffectColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::super::Foundation::COLORREF) -> windows_core::HRESULT,
    pub GetEffectColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::super::Foundation::COLORREF) -> windows_core::HRESULT,
    pub GetCaretType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCaretType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetImmContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub ReleaseImmContext: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GetPreferredFont: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, i32, *mut *mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetClientRect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetSelection2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetFEFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UpdateWindow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckTextLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const i32) -> windows_core::HRESULT,
    pub IMEInProgress: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SysBeep: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDocumentFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentPara: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCallManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseCallManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextDocument2Old_Impl: ITextDocument_Impl {
    fn AttachMsgFilter(&self, pfilter: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetEffectColor(&self, index: i32, cr: super::super::super::Foundation::COLORREF) -> windows_core::Result<()>;
    fn GetEffectColor(&self, index: i32) -> windows_core::Result<super::super::super::Foundation::COLORREF>;
    fn GetCaretType(&self) -> windows_core::Result<i32>;
    fn SetCaretType(&self, carettype: i32) -> windows_core::Result<()>;
    fn GetImmContext(&self) -> windows_core::Result<i64>;
    fn ReleaseImmContext(&self, context: i64) -> windows_core::Result<()>;
    fn GetPreferredFont(&self, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut windows_core::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::Result<()>;
    fn GetNotificationMode(&self) -> windows_core::Result<i32>;
    fn SetNotificationMode(&self, mode: i32) -> windows_core::Result<()>;
    fn GetClientRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::Result<()>;
    fn GetSelection2(&self) -> windows_core::Result<ITextSelection>;
    fn GetWindow(&self) -> windows_core::Result<i32>;
    fn GetFEFlags(&self) -> windows_core::Result<i32>;
    fn UpdateWindow(&self) -> windows_core::Result<()>;
    fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> windows_core::Result<()>;
    fn IMEInProgress(&self, value: i32) -> windows_core::Result<()>;
    fn SysBeep(&self) -> windows_core::Result<()>;
    fn Update(&self, mode: i32) -> windows_core::Result<()>;
    fn Notify(&self, notify: i32) -> windows_core::Result<()>;
    fn GetDocumentFont(&self) -> windows_core::Result<ITextFont>;
    fn GetDocumentPara(&self) -> windows_core::Result<ITextPara>;
    fn GetCallManager(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn ReleaseCallManager(&self, pvoid: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextDocument2Old_Vtbl {
    pub const fn new<Identity: ITextDocument2Old_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AttachMsgFilter<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::AttachMsgFilter(this, core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn SetEffectColor<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, cr: super::super::super::Foundation::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::SetEffectColor(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&cr)).into()
            }
        }
        unsafe extern "system" fn GetEffectColor<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pcr: *mut super::super::super::Foundation::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetEffectColor(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pcr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCaretType<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcarettype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetCaretType(this) {
                    Ok(ok__) => {
                        pcarettype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCaretType<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, carettype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::SetCaretType(this, core::mem::transmute_copy(&carettype)).into()
            }
        }
        unsafe extern "system" fn GetImmContext<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetImmContext(this) {
                    Ok(ok__) => {
                        pcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseImmContext<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::ReleaseImmContext(this, core::mem::transmute_copy(&context)).into()
            }
        }
        unsafe extern "system" fn GetPreferredFont<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut *mut core::ffi::c_void, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::GetPreferredFont(this, core::mem::transmute_copy(&cp), core::mem::transmute_copy(&charrep), core::mem::transmute_copy(&option), core::mem::transmute_copy(&charrepcur), core::mem::transmute_copy(&curfontsize), core::mem::transmute_copy(&pbstr), core::mem::transmute_copy(&ppitchandfamily), core::mem::transmute_copy(&pnewfontsize)).into()
            }
        }
        unsafe extern "system" fn GetNotificationMode<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetNotificationMode(this) {
                    Ok(ok__) => {
                        pmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotificationMode<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::SetNotificationMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn GetClientRect<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::GetClientRect(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pright), core::mem::transmute_copy(&pbottom)).into()
            }
        }
        unsafe extern "system" fn GetSelection2<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetSelection2(this) {
                    Ok(ok__) => {
                        ppsel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWindow<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetWindow(this) {
                    Ok(ok__) => {
                        phwnd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFEFlags<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetFEFlags(this) {
                    Ok(ok__) => {
                        pflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateWindow<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::UpdateWindow(this).into()
            }
        }
        unsafe extern "system" fn CheckTextLimit<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: i32, pcch: *const i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::CheckTextLimit(this, core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pcch)).into()
            }
        }
        unsafe extern "system" fn IMEInProgress<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::IMEInProgress(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SysBeep<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::SysBeep(this).into()
            }
        }
        unsafe extern "system" fn Update<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::Update(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn Notify<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notify: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::Notify(this, core::mem::transmute_copy(&notify)).into()
            }
        }
        unsafe extern "system" fn GetDocumentFont<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitextfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetDocumentFont(this) {
                    Ok(ok__) => {
                        ppitextfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentPara<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitextpara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetDocumentPara(this) {
                    Ok(ok__) => {
                        ppitextpara.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCallManager<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvoid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetCallManager(this) {
                    Ok(ok__) => {
                        ppvoid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseCallManager<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvoid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::ReleaseCallManager(this, core::mem::transmute_copy(&pvoid)).into()
            }
        }
        Self {
            base__: ITextDocument_Vtbl::new::<Identity, OFFSET>(),
            AttachMsgFilter: AttachMsgFilter::<Identity, OFFSET>,
            SetEffectColor: SetEffectColor::<Identity, OFFSET>,
            GetEffectColor: GetEffectColor::<Identity, OFFSET>,
            GetCaretType: GetCaretType::<Identity, OFFSET>,
            SetCaretType: SetCaretType::<Identity, OFFSET>,
            GetImmContext: GetImmContext::<Identity, OFFSET>,
            ReleaseImmContext: ReleaseImmContext::<Identity, OFFSET>,
            GetPreferredFont: GetPreferredFont::<Identity, OFFSET>,
            GetNotificationMode: GetNotificationMode::<Identity, OFFSET>,
            SetNotificationMode: SetNotificationMode::<Identity, OFFSET>,
            GetClientRect: GetClientRect::<Identity, OFFSET>,
            GetSelection2: GetSelection2::<Identity, OFFSET>,
            GetWindow: GetWindow::<Identity, OFFSET>,
            GetFEFlags: GetFEFlags::<Identity, OFFSET>,
            UpdateWindow: UpdateWindow::<Identity, OFFSET>,
            CheckTextLimit: CheckTextLimit::<Identity, OFFSET>,
            IMEInProgress: IMEInProgress::<Identity, OFFSET>,
            SysBeep: SysBeep::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
            GetDocumentFont: GetDocumentFont::<Identity, OFFSET>,
            GetDocumentPara: GetDocumentPara::<Identity, OFFSET>,
            GetCallManager: GetCallManager::<Identity, OFFSET>,
            ReleaseCallManager: ReleaseCallManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDocument2Old as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextDocument as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextDocument2Old {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextFont, ITextFont_Vtbl, 0x8cc497c3_a1df_11ce_8098_00aa0047be5d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextFont {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextFont, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITextFont {
    pub unsafe fn GetDuplicate(&self) -> windows_core::Result<ITextFont> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuplicate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDuplicate<P0>(&self, pfont: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextFont>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDuplicate)(windows_core::Interface::as_raw(self), pfont.param().abi()).ok() }
    }
    pub unsafe fn CanChange(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEqual<P0>(&self, pfont: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ITextFont>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), pfont.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self, value: tomConstants) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetStyle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStyle(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStyle)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetAllCaps(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllCaps(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllCaps)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetAnimation(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAnimation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAnimation(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAnimation)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetBackColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBackColor(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBackColor)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetBold(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBold)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBold(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBold)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetEmboss(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEmboss)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEmboss(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEmboss)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetForeColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetForeColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetForeColor(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetForeColor)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetHidden(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHidden)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHidden(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHidden)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetEngrave(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEngrave)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEngrave(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEngrave)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetItalic(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItalic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetItalic(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetItalic)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetKerning(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKerning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKerning(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKerning)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetLanguageID(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguageID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLanguageID(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLanguageID)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)).ok() }
    }
    pub unsafe fn GetOutline(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOutline(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutline)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetPosition(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPosition(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetProtected(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProtected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProtected(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProtected)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetShadow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetShadow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShadow(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShadow)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetSize(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSize(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetSmallCaps(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSmallCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSmallCaps(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSmallCaps)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetSpacing(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpacing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSpacing(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSpacing)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetStrikeThrough(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrikeThrough)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStrikeThrough(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStrikeThrough)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetSubscript(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubscript)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSubscript(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSubscript)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetSuperscript(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSuperscript)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSuperscript(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSuperscript)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetUnderline(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnderline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUnderline(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUnderline)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetWeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWeight(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWeight)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextFont_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub GetDuplicate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuplicate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, tomConstants) -> windows_core::HRESULT,
    pub GetStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAllCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAllCaps: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAnimation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAnimation: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetBackColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBackColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetBold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBold: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetEmboss: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEmboss: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetForeColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetForeColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHidden: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetEngrave: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEngrave: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetItalic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetItalic: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetKerning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetKerning: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetLanguageID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLanguageID: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOutline: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetProtected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProtected: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetShadow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetShadow: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetSmallCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSmallCaps: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetStrikeThrough: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStrikeThrough: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSubscript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSubscript: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSuperscript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSuperscript: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetWeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextFont_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn GetDuplicate(&self) -> windows_core::Result<ITextFont>;
    fn SetDuplicate(&self, pfont: windows_core::Ref<'_, ITextFont>) -> windows_core::Result<()>;
    fn CanChange(&self) -> windows_core::Result<i32>;
    fn IsEqual(&self, pfont: windows_core::Ref<'_, ITextFont>) -> windows_core::Result<i32>;
    fn Reset(&self, value: tomConstants) -> windows_core::Result<()>;
    fn GetStyle(&self) -> windows_core::Result<i32>;
    fn SetStyle(&self, value: i32) -> windows_core::Result<()>;
    fn GetAllCaps(&self) -> windows_core::Result<i32>;
    fn SetAllCaps(&self, value: i32) -> windows_core::Result<()>;
    fn GetAnimation(&self) -> windows_core::Result<i32>;
    fn SetAnimation(&self, value: i32) -> windows_core::Result<()>;
    fn GetBackColor(&self) -> windows_core::Result<i32>;
    fn SetBackColor(&self, value: i32) -> windows_core::Result<()>;
    fn GetBold(&self) -> windows_core::Result<i32>;
    fn SetBold(&self, value: i32) -> windows_core::Result<()>;
    fn GetEmboss(&self) -> windows_core::Result<i32>;
    fn SetEmboss(&self, value: i32) -> windows_core::Result<()>;
    fn GetForeColor(&self) -> windows_core::Result<i32>;
    fn SetForeColor(&self, value: i32) -> windows_core::Result<()>;
    fn GetHidden(&self) -> windows_core::Result<i32>;
    fn SetHidden(&self, value: i32) -> windows_core::Result<()>;
    fn GetEngrave(&self) -> windows_core::Result<i32>;
    fn SetEngrave(&self, value: i32) -> windows_core::Result<()>;
    fn GetItalic(&self) -> windows_core::Result<i32>;
    fn SetItalic(&self, value: i32) -> windows_core::Result<()>;
    fn GetKerning(&self) -> windows_core::Result<f32>;
    fn SetKerning(&self, value: f32) -> windows_core::Result<()>;
    fn GetLanguageID(&self) -> windows_core::Result<i32>;
    fn SetLanguageID(&self, value: i32) -> windows_core::Result<()>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetOutline(&self) -> windows_core::Result<i32>;
    fn SetOutline(&self, value: i32) -> windows_core::Result<()>;
    fn GetPosition(&self) -> windows_core::Result<f32>;
    fn SetPosition(&self, value: f32) -> windows_core::Result<()>;
    fn GetProtected(&self) -> windows_core::Result<i32>;
    fn SetProtected(&self, value: i32) -> windows_core::Result<()>;
    fn GetShadow(&self) -> windows_core::Result<i32>;
    fn SetShadow(&self, value: i32) -> windows_core::Result<()>;
    fn GetSize(&self) -> windows_core::Result<f32>;
    fn SetSize(&self, value: f32) -> windows_core::Result<()>;
    fn GetSmallCaps(&self) -> windows_core::Result<i32>;
    fn SetSmallCaps(&self, value: i32) -> windows_core::Result<()>;
    fn GetSpacing(&self) -> windows_core::Result<f32>;
    fn SetSpacing(&self, value: f32) -> windows_core::Result<()>;
    fn GetStrikeThrough(&self) -> windows_core::Result<i32>;
    fn SetStrikeThrough(&self, value: i32) -> windows_core::Result<()>;
    fn GetSubscript(&self) -> windows_core::Result<i32>;
    fn SetSubscript(&self, value: i32) -> windows_core::Result<()>;
    fn GetSuperscript(&self) -> windows_core::Result<i32>;
    fn SetSuperscript(&self, value: i32) -> windows_core::Result<()>;
    fn GetUnderline(&self) -> windows_core::Result<i32>;
    fn SetUnderline(&self, value: i32) -> windows_core::Result<()>;
    fn GetWeight(&self) -> windows_core::Result<i32>;
    fn SetWeight(&self, value: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextFont_Vtbl {
    pub const fn new<Identity: ITextFont_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDuplicate<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetDuplicate(this) {
                    Ok(ok__) => {
                        ppfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDuplicate<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetDuplicate(this, core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn CanChange<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::CanChange(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::IsEqual(this, core::mem::transmute_copy(&pfont)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: tomConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::Reset(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetStyle<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetStyle(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStyle<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetStyle(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetAllCaps<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetAllCaps(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllCaps<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetAllCaps(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetAnimation<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetAnimation(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAnimation<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetAnimation(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetBackColor<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetBackColor(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackColor<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetBackColor(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetBold<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetBold(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBold<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetBold(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetEmboss<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetEmboss(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEmboss<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetEmboss(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetForeColor<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetForeColor(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetForeColor<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetForeColor(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetHidden<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetHidden(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHidden<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetHidden(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetEngrave<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetEngrave(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEngrave<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetEngrave(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetItalic<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetItalic(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetItalic<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetItalic(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetKerning<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetKerning(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKerning<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetKerning(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetLanguageID<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetLanguageID(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLanguageID<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetLanguageID(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetName(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetName(this, core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn GetOutline<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetOutline(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutline<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetOutline(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetPosition<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetPosition(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPosition<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetPosition(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetProtected<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetProtected(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProtected<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetProtected(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetShadow<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetShadow(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShadow<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetShadow(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSize<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetSize(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSize<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetSize(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSmallCaps<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetSmallCaps(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSmallCaps<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetSmallCaps(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSpacing<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetSpacing(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpacing<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetSpacing(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetStrikeThrough<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetStrikeThrough(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrikeThrough<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetStrikeThrough(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSubscript<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetSubscript(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubscript<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetSubscript(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSuperscript<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetSuperscript(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSuperscript<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetSuperscript(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetUnderline<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetUnderline(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnderline<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetUnderline(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetWeight<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetWeight(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWeight<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetWeight(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetDuplicate: GetDuplicate::<Identity, OFFSET>,
            SetDuplicate: SetDuplicate::<Identity, OFFSET>,
            CanChange: CanChange::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            GetStyle: GetStyle::<Identity, OFFSET>,
            SetStyle: SetStyle::<Identity, OFFSET>,
            GetAllCaps: GetAllCaps::<Identity, OFFSET>,
            SetAllCaps: SetAllCaps::<Identity, OFFSET>,
            GetAnimation: GetAnimation::<Identity, OFFSET>,
            SetAnimation: SetAnimation::<Identity, OFFSET>,
            GetBackColor: GetBackColor::<Identity, OFFSET>,
            SetBackColor: SetBackColor::<Identity, OFFSET>,
            GetBold: GetBold::<Identity, OFFSET>,
            SetBold: SetBold::<Identity, OFFSET>,
            GetEmboss: GetEmboss::<Identity, OFFSET>,
            SetEmboss: SetEmboss::<Identity, OFFSET>,
            GetForeColor: GetForeColor::<Identity, OFFSET>,
            SetForeColor: SetForeColor::<Identity, OFFSET>,
            GetHidden: GetHidden::<Identity, OFFSET>,
            SetHidden: SetHidden::<Identity, OFFSET>,
            GetEngrave: GetEngrave::<Identity, OFFSET>,
            SetEngrave: SetEngrave::<Identity, OFFSET>,
            GetItalic: GetItalic::<Identity, OFFSET>,
            SetItalic: SetItalic::<Identity, OFFSET>,
            GetKerning: GetKerning::<Identity, OFFSET>,
            SetKerning: SetKerning::<Identity, OFFSET>,
            GetLanguageID: GetLanguageID::<Identity, OFFSET>,
            SetLanguageID: SetLanguageID::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            GetOutline: GetOutline::<Identity, OFFSET>,
            SetOutline: SetOutline::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
            SetPosition: SetPosition::<Identity, OFFSET>,
            GetProtected: GetProtected::<Identity, OFFSET>,
            SetProtected: SetProtected::<Identity, OFFSET>,
            GetShadow: GetShadow::<Identity, OFFSET>,
            SetShadow: SetShadow::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            GetSmallCaps: GetSmallCaps::<Identity, OFFSET>,
            SetSmallCaps: SetSmallCaps::<Identity, OFFSET>,
            GetSpacing: GetSpacing::<Identity, OFFSET>,
            SetSpacing: SetSpacing::<Identity, OFFSET>,
            GetStrikeThrough: GetStrikeThrough::<Identity, OFFSET>,
            SetStrikeThrough: SetStrikeThrough::<Identity, OFFSET>,
            GetSubscript: GetSubscript::<Identity, OFFSET>,
            SetSubscript: SetSubscript::<Identity, OFFSET>,
            GetSuperscript: GetSuperscript::<Identity, OFFSET>,
            SetSuperscript: SetSuperscript::<Identity, OFFSET>,
            GetUnderline: GetUnderline::<Identity, OFFSET>,
            SetUnderline: SetUnderline::<Identity, OFFSET>,
            GetWeight: GetWeight::<Identity, OFFSET>,
            SetWeight: SetWeight::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextFont as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextFont {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextFont2, ITextFont2_Vtbl, 0xc241f5e3_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextFont2 {
    type Target = ITextFont;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextFont2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextFont);
#[cfg(feature = "Win32_System_Com")]
impl ITextFont2 {
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAutoLigatures(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAutoLigatures)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutoLigatures(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAutoLigatures)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetAutospaceAlpha(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAutospaceAlpha)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutospaceAlpha(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAutospaceAlpha)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetAutospaceNumeric(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAutospaceNumeric)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutospaceNumeric(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAutospaceNumeric)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetAutospaceParens(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAutospaceParens)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutospaceParens(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAutospaceParens)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCharRep(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCharRep)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCharRep(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCharRep)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCompressionMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCompressionMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCompressionMode(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompressionMode)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCookie(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCookie(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCookie)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetDoubleStrike(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDoubleStrike)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDoubleStrike(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDoubleStrike)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetDuplicate2(&self) -> windows_core::Result<ITextFont2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuplicate2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDuplicate2<P0>(&self, pfont: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextFont2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDuplicate2)(windows_core::Interface::as_raw(self), pfont.param().abi()).ok() }
    }
    pub unsafe fn GetLinkType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLinkType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMathZone(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMathZone)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMathZone(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMathZone)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetModWidthPairs(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModWidthPairs)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetModWidthPairs(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetModWidthPairs)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetModWidthSpace(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModWidthSpace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetModWidthSpace(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetModWidthSpace)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetOldNumbers(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOldNumbers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOldNumbers(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOldNumbers)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetOverlapping(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOverlapping)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOverlapping(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOverlapping)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetPositionSubSuper(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPositionSubSuper)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPositionSubSuper(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPositionSubSuper)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetScaling(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScaling)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScaling(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScaling)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetSpaceExtension(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpaceExtension)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSpaceExtension(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSpaceExtension)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetUnderlinePositionMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnderlinePositionMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUnderlinePositionMode(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUnderlinePositionMode)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEffects)(windows_core::Interface::as_raw(self), pvalue as _, pmask as _).ok() }
    }
    pub unsafe fn GetEffects2(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEffects2)(windows_core::Interface::as_raw(self), pvalue as _, pmask as _).ok() }
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPropertyInfo(&self, index: i32, ptype: *mut i32, pvalue: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyInfo)(windows_core::Interface::as_raw(self), index, ptype as _, pvalue as _).ok() }
    }
    pub unsafe fn IsEqual2<P0>(&self, pfont: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ITextFont2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual2)(windows_core::Interface::as_raw(self), pfont.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEffects(&self, value: i32, mask: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEffects)(windows_core::Interface::as_raw(self), value, mask).ok() }
    }
    pub unsafe fn SetEffects2(&self, value: i32, mask: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEffects2)(windows_core::Interface::as_raw(self), value, mask).ok() }
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), r#type, value).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextFont2_Vtbl {
    pub base__: ITextFont_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetAutoLigatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutoLigatures: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAutospaceAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutospaceAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAutospaceNumeric: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutospaceNumeric: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAutospaceParens: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutospaceParens: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCharRep: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCharRep: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCompressionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCompressionMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCookie: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDoubleStrike: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDoubleStrike: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDuplicate2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuplicate2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLinkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetMathZone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMathZone: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetModWidthPairs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetModWidthPairs: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetModWidthSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetModWidthSpace: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetOldNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOldNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetOverlapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOverlapping: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPositionSubSuper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPositionSubSuper: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetScaling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetScaling: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSpaceExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpaceExtension: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetUnderlinePositionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetUnderlinePositionMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetEffects2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetPropertyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub IsEqual2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetEffects2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextFont2_Impl: ITextFont_Impl {
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetAutoLigatures(&self) -> windows_core::Result<i32>;
    fn SetAutoLigatures(&self, value: i32) -> windows_core::Result<()>;
    fn GetAutospaceAlpha(&self) -> windows_core::Result<i32>;
    fn SetAutospaceAlpha(&self, value: i32) -> windows_core::Result<()>;
    fn GetAutospaceNumeric(&self) -> windows_core::Result<i32>;
    fn SetAutospaceNumeric(&self, value: i32) -> windows_core::Result<()>;
    fn GetAutospaceParens(&self) -> windows_core::Result<i32>;
    fn SetAutospaceParens(&self, value: i32) -> windows_core::Result<()>;
    fn GetCharRep(&self) -> windows_core::Result<i32>;
    fn SetCharRep(&self, value: i32) -> windows_core::Result<()>;
    fn GetCompressionMode(&self) -> windows_core::Result<i32>;
    fn SetCompressionMode(&self, value: i32) -> windows_core::Result<()>;
    fn GetCookie(&self) -> windows_core::Result<i32>;
    fn SetCookie(&self, value: i32) -> windows_core::Result<()>;
    fn GetDoubleStrike(&self) -> windows_core::Result<i32>;
    fn SetDoubleStrike(&self, value: i32) -> windows_core::Result<()>;
    fn GetDuplicate2(&self) -> windows_core::Result<ITextFont2>;
    fn SetDuplicate2(&self, pfont: windows_core::Ref<'_, ITextFont2>) -> windows_core::Result<()>;
    fn GetLinkType(&self) -> windows_core::Result<i32>;
    fn GetMathZone(&self) -> windows_core::Result<i32>;
    fn SetMathZone(&self, value: i32) -> windows_core::Result<()>;
    fn GetModWidthPairs(&self) -> windows_core::Result<i32>;
    fn SetModWidthPairs(&self, value: i32) -> windows_core::Result<()>;
    fn GetModWidthSpace(&self) -> windows_core::Result<i32>;
    fn SetModWidthSpace(&self, value: i32) -> windows_core::Result<()>;
    fn GetOldNumbers(&self) -> windows_core::Result<i32>;
    fn SetOldNumbers(&self, value: i32) -> windows_core::Result<()>;
    fn GetOverlapping(&self) -> windows_core::Result<i32>;
    fn SetOverlapping(&self, value: i32) -> windows_core::Result<()>;
    fn GetPositionSubSuper(&self) -> windows_core::Result<i32>;
    fn SetPositionSubSuper(&self, value: i32) -> windows_core::Result<()>;
    fn GetScaling(&self) -> windows_core::Result<i32>;
    fn SetScaling(&self, value: i32) -> windows_core::Result<()>;
    fn GetSpaceExtension(&self) -> windows_core::Result<f32>;
    fn SetSpaceExtension(&self, value: f32) -> windows_core::Result<()>;
    fn GetUnderlinePositionMode(&self) -> windows_core::Result<i32>;
    fn SetUnderlinePositionMode(&self, value: i32) -> windows_core::Result<()>;
    fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::Result<()>;
    fn GetEffects2(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn GetPropertyInfo(&self, index: i32, ptype: *mut i32, pvalue: *mut i32) -> windows_core::Result<()>;
    fn IsEqual2(&self, pfont: windows_core::Ref<'_, ITextFont2>) -> windows_core::Result<i32>;
    fn SetEffects(&self, value: i32, mask: i32) -> windows_core::Result<()>;
    fn SetEffects2(&self, value: i32, mask: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextFont2_Vtbl {
    pub const fn new<Identity: ITextFont2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAutoLigatures<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetAutoLigatures(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutoLigatures<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetAutoLigatures(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetAutospaceAlpha<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetAutospaceAlpha(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutospaceAlpha<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetAutospaceAlpha(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetAutospaceNumeric<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetAutospaceNumeric(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutospaceNumeric<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetAutospaceNumeric(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetAutospaceParens<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetAutospaceParens(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutospaceParens<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetAutospaceParens(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCharRep<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetCharRep(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCharRep<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetCharRep(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCompressionMode<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetCompressionMode(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCompressionMode<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetCompressionMode(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCookie<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetCookie(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCookie<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetCookie(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetDoubleStrike<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetDoubleStrike(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDoubleStrike<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetDoubleStrike(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetDuplicate2<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetDuplicate2(this) {
                    Ok(ok__) => {
                        ppfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDuplicate2<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetDuplicate2(this, core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn GetLinkType<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetLinkType(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMathZone<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetMathZone(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMathZone<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetMathZone(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetModWidthPairs<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetModWidthPairs(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModWidthPairs<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetModWidthPairs(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetModWidthSpace<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetModWidthSpace(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModWidthSpace<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetModWidthSpace(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetOldNumbers<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetOldNumbers(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOldNumbers<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetOldNumbers(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetOverlapping<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetOverlapping(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOverlapping<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetOverlapping(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetPositionSubSuper<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetPositionSubSuper(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPositionSubSuper<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetPositionSubSuper(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetScaling<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetScaling(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScaling<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetScaling(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSpaceExtension<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetSpaceExtension(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpaceExtension<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetSpaceExtension(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetUnderlinePositionMode<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetUnderlinePositionMode(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnderlinePositionMode<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetUnderlinePositionMode(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetEffects<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::GetEffects(this, core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pmask)).into()
            }
        }
        unsafe extern "system" fn GetEffects2<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::GetEffects2(this, core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pmask)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, ptype: *mut i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn IsEqual2<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void, pb: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::IsEqual2(this, core::mem::transmute_copy(&pfont)) {
                    Ok(ok__) => {
                        pb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEffects<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32, mask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetEffects(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&mask)).into()
            }
        }
        unsafe extern "system" fn SetEffects2<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32, mask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetEffects2(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&mask)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: ITextFont_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAutoLigatures: GetAutoLigatures::<Identity, OFFSET>,
            SetAutoLigatures: SetAutoLigatures::<Identity, OFFSET>,
            GetAutospaceAlpha: GetAutospaceAlpha::<Identity, OFFSET>,
            SetAutospaceAlpha: SetAutospaceAlpha::<Identity, OFFSET>,
            GetAutospaceNumeric: GetAutospaceNumeric::<Identity, OFFSET>,
            SetAutospaceNumeric: SetAutospaceNumeric::<Identity, OFFSET>,
            GetAutospaceParens: GetAutospaceParens::<Identity, OFFSET>,
            SetAutospaceParens: SetAutospaceParens::<Identity, OFFSET>,
            GetCharRep: GetCharRep::<Identity, OFFSET>,
            SetCharRep: SetCharRep::<Identity, OFFSET>,
            GetCompressionMode: GetCompressionMode::<Identity, OFFSET>,
            SetCompressionMode: SetCompressionMode::<Identity, OFFSET>,
            GetCookie: GetCookie::<Identity, OFFSET>,
            SetCookie: SetCookie::<Identity, OFFSET>,
            GetDoubleStrike: GetDoubleStrike::<Identity, OFFSET>,
            SetDoubleStrike: SetDoubleStrike::<Identity, OFFSET>,
            GetDuplicate2: GetDuplicate2::<Identity, OFFSET>,
            SetDuplicate2: SetDuplicate2::<Identity, OFFSET>,
            GetLinkType: GetLinkType::<Identity, OFFSET>,
            GetMathZone: GetMathZone::<Identity, OFFSET>,
            SetMathZone: SetMathZone::<Identity, OFFSET>,
            GetModWidthPairs: GetModWidthPairs::<Identity, OFFSET>,
            SetModWidthPairs: SetModWidthPairs::<Identity, OFFSET>,
            GetModWidthSpace: GetModWidthSpace::<Identity, OFFSET>,
            SetModWidthSpace: SetModWidthSpace::<Identity, OFFSET>,
            GetOldNumbers: GetOldNumbers::<Identity, OFFSET>,
            SetOldNumbers: SetOldNumbers::<Identity, OFFSET>,
            GetOverlapping: GetOverlapping::<Identity, OFFSET>,
            SetOverlapping: SetOverlapping::<Identity, OFFSET>,
            GetPositionSubSuper: GetPositionSubSuper::<Identity, OFFSET>,
            SetPositionSubSuper: SetPositionSubSuper::<Identity, OFFSET>,
            GetScaling: GetScaling::<Identity, OFFSET>,
            SetScaling: SetScaling::<Identity, OFFSET>,
            GetSpaceExtension: GetSpaceExtension::<Identity, OFFSET>,
            SetSpaceExtension: SetSpaceExtension::<Identity, OFFSET>,
            GetUnderlinePositionMode: GetUnderlinePositionMode::<Identity, OFFSET>,
            SetUnderlinePositionMode: SetUnderlinePositionMode::<Identity, OFFSET>,
            GetEffects: GetEffects::<Identity, OFFSET>,
            GetEffects2: GetEffects2::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, OFFSET>,
            IsEqual2: IsEqual2::<Identity, OFFSET>,
            SetEffects: SetEffects::<Identity, OFFSET>,
            SetEffects2: SetEffects2::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextFont2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextFont as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextFont2 {}
windows_core::imp::define_interface!(ITextHost, ITextHost_Vtbl, 0);
windows_core::imp::interface_hierarchy!(ITextHost, windows_core::IUnknown);
impl ITextHost {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxGetDC(&self) -> super::super::super::Graphics::Gdi::HDC {
        unsafe { (windows_core::Interface::vtable(self).TxGetDC)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxReleaseDC(&self, hdc: super::super::super::Graphics::Gdi::HDC) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).TxReleaseDC)(windows_core::Interface::as_raw(self), hdc) }
    }
    pub unsafe fn TxShowScrollBar(&self, fnbar: i32, fshow: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxShowScrollBar)(windows_core::Interface::as_raw(self), fnbar, fshow.into()) }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn TxEnableScrollBar(&self, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: i32) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxEnableScrollBar)(windows_core::Interface::as_raw(self), fusbflags, fuarrowflags) }
    }
    pub unsafe fn TxSetScrollRange(&self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxSetScrollRange)(windows_core::Interface::as_raw(self), fnbar, nminpos, nmaxpos, fredraw.into()) }
    }
    pub unsafe fn TxSetScrollPos(&self, fnbar: i32, npos: i32, fredraw: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxSetScrollPos)(windows_core::Interface::as_raw(self), fnbar, npos, fredraw.into()) }
    }
    pub unsafe fn TxInvalidateRect(&self, prc: *mut super::super::super::Foundation::RECT, fmode: bool) {
        unsafe { (windows_core::Interface::vtable(self).TxInvalidateRect)(windows_core::Interface::as_raw(self), prc as _, fmode.into()) }
    }
    pub unsafe fn TxViewChange(&self, fupdate: bool) {
        unsafe { (windows_core::Interface::vtable(self).TxViewChange)(windows_core::Interface::as_raw(self), fupdate.into()) }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxCreateCaret(&self, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxCreateCaret)(windows_core::Interface::as_raw(self), hbmp, xwidth, yheight) }
    }
    pub unsafe fn TxShowCaret(&self, fshow: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxShowCaret)(windows_core::Interface::as_raw(self), fshow.into()) }
    }
    pub unsafe fn TxSetCaretPos(&self, x: i32, y: i32) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxSetCaretPos)(windows_core::Interface::as_raw(self), x, y) }
    }
    pub unsafe fn TxSetTimer(&self, idtimer: u32, utimeout: u32) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxSetTimer)(windows_core::Interface::as_raw(self), idtimer, utimeout) }
    }
    pub unsafe fn TxKillTimer(&self, idtimer: u32) {
        unsafe { (windows_core::Interface::vtable(self).TxKillTimer)(windows_core::Interface::as_raw(self), idtimer) }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxScrollWindowEx(&self, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SCROLL_WINDOW_FLAGS) {
        unsafe { (windows_core::Interface::vtable(self).TxScrollWindowEx)(windows_core::Interface::as_raw(self), dx, dy, lprcscroll as _, lprcclip as _, hrgnupdate, lprcupdate as _, fuscroll) }
    }
    pub unsafe fn TxSetCapture(&self, fcapture: bool) {
        unsafe { (windows_core::Interface::vtable(self).TxSetCapture)(windows_core::Interface::as_raw(self), fcapture.into()) }
    }
    pub unsafe fn TxSetFocus(&self) {
        unsafe { (windows_core::Interface::vtable(self).TxSetFocus)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn TxSetCursor(&self, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: bool) {
        unsafe { (windows_core::Interface::vtable(self).TxSetCursor)(windows_core::Interface::as_raw(self), hcur, ftext.into()) }
    }
    pub unsafe fn TxScreenToClient(&self, lppt: *mut super::super::super::Foundation::POINT) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxScreenToClient)(windows_core::Interface::as_raw(self), lppt as _) }
    }
    pub unsafe fn TxClientToScreen(&self, lppt: *mut super::super::super::Foundation::POINT) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxClientToScreen)(windows_core::Interface::as_raw(self), lppt as _) }
    }
    pub unsafe fn TxActivate(&self, ploldstate: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxActivate)(windows_core::Interface::as_raw(self), ploldstate as _).ok() }
    }
    pub unsafe fn TxDeactivate(&self, lnewstate: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxDeactivate)(windows_core::Interface::as_raw(self), lnewstate).ok() }
    }
    pub unsafe fn TxGetClientRect(&self, prc: *mut super::super::super::Foundation::RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetClientRect)(windows_core::Interface::as_raw(self), prc as _).ok() }
    }
    pub unsafe fn TxGetViewInset(&self, prc: *mut super::super::super::Foundation::RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetViewInset)(windows_core::Interface::as_raw(self), prc as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxGetCharFormat(&self, ppcf: *const *const CHARFORMATW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetCharFormat)(windows_core::Interface::as_raw(self), ppcf).ok() }
    }
    pub unsafe fn TxGetParaFormat(&self, pppf: *const *const PARAFORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetParaFormat)(windows_core::Interface::as_raw(self), pppf).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxGetSysColor(&self, nindex: super::super::super::Graphics::Gdi::SYS_COLOR_INDEX) -> super::super::super::Foundation::COLORREF {
        unsafe { (windows_core::Interface::vtable(self).TxGetSysColor)(windows_core::Interface::as_raw(self), nindex) }
    }
    pub unsafe fn TxGetBackStyle(&self, pstyle: *mut TXTBACKSTYLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetBackStyle)(windows_core::Interface::as_raw(self), pstyle as _).ok() }
    }
    pub unsafe fn TxGetMaxLength(&self, plength: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetMaxLength)(windows_core::Interface::as_raw(self), plength as _).ok() }
    }
    pub unsafe fn TxGetScrollBars(&self, pdwscrollbar: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetScrollBars)(windows_core::Interface::as_raw(self), pdwscrollbar as _).ok() }
    }
    pub unsafe fn TxGetPasswordChar(&self) -> windows_core::Result<i8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetPasswordChar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TxGetAcceleratorPos(&self, pcp: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetAcceleratorPos)(windows_core::Interface::as_raw(self), pcp as _).ok() }
    }
    pub unsafe fn TxGetExtent(&self, lpextent: *mut super::super::super::Foundation::SIZE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetExtent)(windows_core::Interface::as_raw(self), lpextent as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn OnTxCharFormatChange(&self, pcf: *const CHARFORMATW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTxCharFormatChange)(windows_core::Interface::as_raw(self), pcf).ok() }
    }
    pub unsafe fn OnTxParaFormatChange(&self, ppf: *const PARAFORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTxParaFormatChange)(windows_core::Interface::as_raw(self), ppf).ok() }
    }
    pub unsafe fn TxGetPropertyBits(&self, dwmask: u32, pdwbits: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetPropertyBits)(windows_core::Interface::as_raw(self), dwmask, pdwbits as _).ok() }
    }
    pub unsafe fn TxNotify(&self, inotify: u32, pv: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxNotify)(windows_core::Interface::as_raw(self), inotify, pv as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_Ime")]
    pub unsafe fn TxImmGetContext(&self) -> super::super::Input::Ime::HIMC {
        unsafe { (windows_core::Interface::vtable(self).TxImmGetContext)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_UI_Input_Ime")]
    pub unsafe fn TxImmReleaseContext(&self, himc: super::super::Input::Ime::HIMC) {
        unsafe { (windows_core::Interface::vtable(self).TxImmReleaseContext)(windows_core::Interface::as_raw(self), himc) }
    }
    pub unsafe fn TxGetSelectionBarWidth(&self, lselbarwidth: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetSelectionBarWidth)(windows_core::Interface::as_raw(self), lselbarwidth as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub TxGetDC: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::super::Graphics::Gdi::HDC,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    TxGetDC: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub TxReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Graphics::Gdi::HDC) -> i32,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    TxReleaseDC: usize,
    pub TxShowScrollBar: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::BOOL) -> windows_core::BOOL,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub TxEnableScrollBar: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, i32) -> windows_core::BOOL,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    TxEnableScrollBar: usize,
    pub TxSetScrollRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, windows_core::BOOL) -> windows_core::BOOL,
    pub TxSetScrollPos: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::BOOL) -> windows_core::BOOL,
    pub TxInvalidateRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::RECT, windows_core::BOOL),
    pub TxViewChange: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL),
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub TxCreateCaret: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Graphics::Gdi::HBITMAP, i32, i32) -> windows_core::BOOL,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    TxCreateCaret: usize,
    pub TxShowCaret: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::BOOL,
    pub TxSetCaretPos: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::BOOL,
    pub TxSetTimer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::BOOL,
    pub TxKillTimer: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TxScrollWindowEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut super::super::super::Foundation::RECT, *mut super::super::super::Foundation::RECT, super::super::super::Graphics::Gdi::HRGN, *mut super::super::super::Foundation::RECT, super::super::WindowsAndMessaging::SCROLL_WINDOW_FLAGS),
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    TxScrollWindowEx: usize,
    pub TxSetCapture: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL),
    pub TxSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub TxSetCursor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::WindowsAndMessaging::HCURSOR, windows_core::BOOL),
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    TxSetCursor: usize,
    pub TxScreenToClient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::POINT) -> windows_core::BOOL,
    pub TxClientToScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::POINT) -> windows_core::BOOL,
    pub TxActivate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TxDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub TxGetClientRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub TxGetViewInset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::RECT) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub TxGetCharFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const CHARFORMATW) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    TxGetCharFormat: usize,
    pub TxGetParaFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const PARAFORMAT) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub TxGetSysColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Graphics::Gdi::SYS_COLOR_INDEX) -> super::super::super::Foundation::COLORREF,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    TxGetSysColor: usize,
    pub TxGetBackStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TXTBACKSTYLE) -> windows_core::HRESULT,
    pub TxGetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TxGetScrollBars: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TxGetPasswordChar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i8) -> windows_core::HRESULT,
    pub TxGetAcceleratorPos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TxGetExtent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::SIZE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub OnTxCharFormatChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const CHARFORMATW) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    OnTxCharFormatChange: usize,
    pub OnTxParaFormatChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const PARAFORMAT) -> windows_core::HRESULT,
    pub TxGetPropertyBits: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub TxNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_Ime")]
    pub TxImmGetContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Input::Ime::HIMC,
    #[cfg(not(feature = "Win32_UI_Input_Ime"))]
    TxImmGetContext: usize,
    #[cfg(feature = "Win32_UI_Input_Ime")]
    pub TxImmReleaseContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Input::Ime::HIMC),
    #[cfg(not(feature = "Win32_UI_Input_Ime"))]
    TxImmReleaseContext: usize,
    pub TxGetSelectionBarWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Input_Ime", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITextHost_Impl: windows_core::IUnknownImpl {
    fn TxGetDC(&self) -> super::super::super::Graphics::Gdi::HDC;
    fn TxReleaseDC(&self, hdc: super::super::super::Graphics::Gdi::HDC) -> i32;
    fn TxShowScrollBar(&self, fnbar: i32, fshow: windows_core::BOOL) -> windows_core::BOOL;
    fn TxEnableScrollBar(&self, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: i32) -> windows_core::BOOL;
    fn TxSetScrollRange(&self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: windows_core::BOOL) -> windows_core::BOOL;
    fn TxSetScrollPos(&self, fnbar: i32, npos: i32, fredraw: windows_core::BOOL) -> windows_core::BOOL;
    fn TxInvalidateRect(&self, prc: *mut super::super::super::Foundation::RECT, fmode: windows_core::BOOL);
    fn TxViewChange(&self, fupdate: windows_core::BOOL);
    fn TxCreateCaret(&self, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> windows_core::BOOL;
    fn TxShowCaret(&self, fshow: windows_core::BOOL) -> windows_core::BOOL;
    fn TxSetCaretPos(&self, x: i32, y: i32) -> windows_core::BOOL;
    fn TxSetTimer(&self, idtimer: u32, utimeout: u32) -> windows_core::BOOL;
    fn TxKillTimer(&self, idtimer: u32);
    fn TxScrollWindowEx(&self, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SCROLL_WINDOW_FLAGS);
    fn TxSetCapture(&self, fcapture: windows_core::BOOL);
    fn TxSetFocus(&self);
    fn TxSetCursor(&self, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: windows_core::BOOL);
    fn TxScreenToClient(&self, lppt: *mut super::super::super::Foundation::POINT) -> windows_core::BOOL;
    fn TxClientToScreen(&self, lppt: *mut super::super::super::Foundation::POINT) -> windows_core::BOOL;
    fn TxActivate(&self, ploldstate: *mut i32) -> windows_core::Result<()>;
    fn TxDeactivate(&self, lnewstate: i32) -> windows_core::Result<()>;
    fn TxGetClientRect(&self, prc: *mut super::super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn TxGetViewInset(&self, prc: *mut super::super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn TxGetCharFormat(&self, ppcf: *const *const CHARFORMATW) -> windows_core::Result<()>;
    fn TxGetParaFormat(&self, pppf: *const *const PARAFORMAT) -> windows_core::Result<()>;
    fn TxGetSysColor(&self, nindex: super::super::super::Graphics::Gdi::SYS_COLOR_INDEX) -> super::super::super::Foundation::COLORREF;
    fn TxGetBackStyle(&self, pstyle: *mut TXTBACKSTYLE) -> windows_core::Result<()>;
    fn TxGetMaxLength(&self, plength: *mut u32) -> windows_core::Result<()>;
    fn TxGetScrollBars(&self, pdwscrollbar: *mut u32) -> windows_core::Result<()>;
    fn TxGetPasswordChar(&self) -> windows_core::Result<i8>;
    fn TxGetAcceleratorPos(&self, pcp: *mut i32) -> windows_core::Result<()>;
    fn TxGetExtent(&self, lpextent: *mut super::super::super::Foundation::SIZE) -> windows_core::Result<()>;
    fn OnTxCharFormatChange(&self, pcf: *const CHARFORMATW) -> windows_core::Result<()>;
    fn OnTxParaFormatChange(&self, ppf: *const PARAFORMAT) -> windows_core::Result<()>;
    fn TxGetPropertyBits(&self, dwmask: u32, pdwbits: *mut u32) -> windows_core::Result<()>;
    fn TxNotify(&self, inotify: u32, pv: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn TxImmGetContext(&self) -> super::super::Input::Ime::HIMC;
    fn TxImmReleaseContext(&self, himc: super::super::Input::Ime::HIMC);
    fn TxGetSelectionBarWidth(&self, lselbarwidth: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Input_Ime", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITextHost_Vtbl {
    pub const fn new<Identity: ITextHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TxGetDC<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::super::Graphics::Gdi::HDC {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetDC(this)
            }
        }
        unsafe extern "system" fn TxReleaseDC<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::super::super::Graphics::Gdi::HDC) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxReleaseDC(this, core::mem::transmute_copy(&hdc))
            }
        }
        unsafe extern "system" fn TxShowScrollBar<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnbar: i32, fshow: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxShowScrollBar(this, core::mem::transmute_copy(&fnbar), core::mem::transmute_copy(&fshow))
            }
        }
        unsafe extern "system" fn TxEnableScrollBar<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: i32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxEnableScrollBar(this, core::mem::transmute_copy(&fusbflags), core::mem::transmute_copy(&fuarrowflags))
            }
        }
        unsafe extern "system" fn TxSetScrollRange<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetScrollRange(this, core::mem::transmute_copy(&fnbar), core::mem::transmute_copy(&nminpos), core::mem::transmute_copy(&nmaxpos), core::mem::transmute_copy(&fredraw))
            }
        }
        unsafe extern "system" fn TxSetScrollPos<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnbar: i32, npos: i32, fredraw: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetScrollPos(this, core::mem::transmute_copy(&fnbar), core::mem::transmute_copy(&npos), core::mem::transmute_copy(&fredraw))
            }
        }
        unsafe extern "system" fn TxInvalidateRect<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT, fmode: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxInvalidateRect(this, core::mem::transmute_copy(&prc), core::mem::transmute_copy(&fmode))
            }
        }
        unsafe extern "system" fn TxViewChange<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fupdate: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxViewChange(this, core::mem::transmute_copy(&fupdate))
            }
        }
        unsafe extern "system" fn TxCreateCaret<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxCreateCaret(this, core::mem::transmute_copy(&hbmp), core::mem::transmute_copy(&xwidth), core::mem::transmute_copy(&yheight))
            }
        }
        unsafe extern "system" fn TxShowCaret<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxShowCaret(this, core::mem::transmute_copy(&fshow))
            }
        }
        unsafe extern "system" fn TxSetCaretPos<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetCaretPos(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y))
            }
        }
        unsafe extern "system" fn TxSetTimer<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idtimer: u32, utimeout: u32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetTimer(this, core::mem::transmute_copy(&idtimer), core::mem::transmute_copy(&utimeout))
            }
        }
        unsafe extern "system" fn TxKillTimer<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idtimer: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxKillTimer(this, core::mem::transmute_copy(&idtimer))
            }
        }
        unsafe extern "system" fn TxScrollWindowEx<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SCROLL_WINDOW_FLAGS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxScrollWindowEx(this, core::mem::transmute_copy(&dx), core::mem::transmute_copy(&dy), core::mem::transmute_copy(&lprcscroll), core::mem::transmute_copy(&lprcclip), core::mem::transmute_copy(&hrgnupdate), core::mem::transmute_copy(&lprcupdate), core::mem::transmute_copy(&fuscroll))
            }
        }
        unsafe extern "system" fn TxSetCapture<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcapture: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetCapture(this, core::mem::transmute_copy(&fcapture))
            }
        }
        unsafe extern "system" fn TxSetFocus<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetFocus(this)
            }
        }
        unsafe extern "system" fn TxSetCursor<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetCursor(this, core::mem::transmute_copy(&hcur), core::mem::transmute_copy(&ftext))
            }
        }
        unsafe extern "system" fn TxScreenToClient<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxScreenToClient(this, core::mem::transmute_copy(&lppt))
            }
        }
        unsafe extern "system" fn TxClientToScreen<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxClientToScreen(this, core::mem::transmute_copy(&lppt))
            }
        }
        unsafe extern "system" fn TxActivate<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploldstate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxActivate(this, core::mem::transmute_copy(&ploldstate)).into()
            }
        }
        unsafe extern "system" fn TxDeactivate<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnewstate: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxDeactivate(this, core::mem::transmute_copy(&lnewstate)).into()
            }
        }
        unsafe extern "system" fn TxGetClientRect<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetClientRect(this, core::mem::transmute_copy(&prc)).into()
            }
        }
        unsafe extern "system" fn TxGetViewInset<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetViewInset(this, core::mem::transmute_copy(&prc)).into()
            }
        }
        unsafe extern "system" fn TxGetCharFormat<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcf: *const *const CHARFORMATW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetCharFormat(this, core::mem::transmute_copy(&ppcf)).into()
            }
        }
        unsafe extern "system" fn TxGetParaFormat<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppf: *const *const PARAFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetParaFormat(this, core::mem::transmute_copy(&pppf)).into()
            }
        }
        unsafe extern "system" fn TxGetSysColor<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: super::super::super::Graphics::Gdi::SYS_COLOR_INDEX) -> super::super::super::Foundation::COLORREF {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetSysColor(this, core::mem::transmute_copy(&nindex))
            }
        }
        unsafe extern "system" fn TxGetBackStyle<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstyle: *mut TXTBACKSTYLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetBackStyle(this, core::mem::transmute_copy(&pstyle)).into()
            }
        }
        unsafe extern "system" fn TxGetMaxLength<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetMaxLength(this, core::mem::transmute_copy(&plength)).into()
            }
        }
        unsafe extern "system" fn TxGetScrollBars<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwscrollbar: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetScrollBars(this, core::mem::transmute_copy(&pdwscrollbar)).into()
            }
        }
        unsafe extern "system" fn TxGetPasswordChar<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pch: *mut i8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxGetPasswordChar(this) {
                    Ok(ok__) => {
                        pch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxGetAcceleratorPos<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcp: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetAcceleratorPos(this, core::mem::transmute_copy(&pcp)).into()
            }
        }
        unsafe extern "system" fn TxGetExtent<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpextent: *mut super::super::super::Foundation::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetExtent(this, core::mem::transmute_copy(&lpextent)).into()
            }
        }
        unsafe extern "system" fn OnTxCharFormatChange<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcf: *const CHARFORMATW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::OnTxCharFormatChange(this, core::mem::transmute_copy(&pcf)).into()
            }
        }
        unsafe extern "system" fn OnTxParaFormatChange<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppf: *const PARAFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::OnTxParaFormatChange(this, core::mem::transmute_copy(&ppf)).into()
            }
        }
        unsafe extern "system" fn TxGetPropertyBits<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmask: u32, pdwbits: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetPropertyBits(this, core::mem::transmute_copy(&dwmask), core::mem::transmute_copy(&pdwbits)).into()
            }
        }
        unsafe extern "system" fn TxNotify<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inotify: u32, pv: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxNotify(this, core::mem::transmute_copy(&inotify), core::mem::transmute_copy(&pv)).into()
            }
        }
        unsafe extern "system" fn TxImmGetContext<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Input::Ime::HIMC {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxImmGetContext(this)
            }
        }
        unsafe extern "system" fn TxImmReleaseContext<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: super::super::Input::Ime::HIMC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxImmReleaseContext(this, core::mem::transmute_copy(&himc))
            }
        }
        unsafe extern "system" fn TxGetSelectionBarWidth<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lselbarwidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetSelectionBarWidth(this, core::mem::transmute_copy(&lselbarwidth)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TxGetDC: TxGetDC::<Identity, OFFSET>,
            TxReleaseDC: TxReleaseDC::<Identity, OFFSET>,
            TxShowScrollBar: TxShowScrollBar::<Identity, OFFSET>,
            TxEnableScrollBar: TxEnableScrollBar::<Identity, OFFSET>,
            TxSetScrollRange: TxSetScrollRange::<Identity, OFFSET>,
            TxSetScrollPos: TxSetScrollPos::<Identity, OFFSET>,
            TxInvalidateRect: TxInvalidateRect::<Identity, OFFSET>,
            TxViewChange: TxViewChange::<Identity, OFFSET>,
            TxCreateCaret: TxCreateCaret::<Identity, OFFSET>,
            TxShowCaret: TxShowCaret::<Identity, OFFSET>,
            TxSetCaretPos: TxSetCaretPos::<Identity, OFFSET>,
            TxSetTimer: TxSetTimer::<Identity, OFFSET>,
            TxKillTimer: TxKillTimer::<Identity, OFFSET>,
            TxScrollWindowEx: TxScrollWindowEx::<Identity, OFFSET>,
            TxSetCapture: TxSetCapture::<Identity, OFFSET>,
            TxSetFocus: TxSetFocus::<Identity, OFFSET>,
            TxSetCursor: TxSetCursor::<Identity, OFFSET>,
            TxScreenToClient: TxScreenToClient::<Identity, OFFSET>,
            TxClientToScreen: TxClientToScreen::<Identity, OFFSET>,
            TxActivate: TxActivate::<Identity, OFFSET>,
            TxDeactivate: TxDeactivate::<Identity, OFFSET>,
            TxGetClientRect: TxGetClientRect::<Identity, OFFSET>,
            TxGetViewInset: TxGetViewInset::<Identity, OFFSET>,
            TxGetCharFormat: TxGetCharFormat::<Identity, OFFSET>,
            TxGetParaFormat: TxGetParaFormat::<Identity, OFFSET>,
            TxGetSysColor: TxGetSysColor::<Identity, OFFSET>,
            TxGetBackStyle: TxGetBackStyle::<Identity, OFFSET>,
            TxGetMaxLength: TxGetMaxLength::<Identity, OFFSET>,
            TxGetScrollBars: TxGetScrollBars::<Identity, OFFSET>,
            TxGetPasswordChar: TxGetPasswordChar::<Identity, OFFSET>,
            TxGetAcceleratorPos: TxGetAcceleratorPos::<Identity, OFFSET>,
            TxGetExtent: TxGetExtent::<Identity, OFFSET>,
            OnTxCharFormatChange: OnTxCharFormatChange::<Identity, OFFSET>,
            OnTxParaFormatChange: OnTxParaFormatChange::<Identity, OFFSET>,
            TxGetPropertyBits: TxGetPropertyBits::<Identity, OFFSET>,
            TxNotify: TxNotify::<Identity, OFFSET>,
            TxImmGetContext: TxImmGetContext::<Identity, OFFSET>,
            TxImmReleaseContext: TxImmReleaseContext::<Identity, OFFSET>,
            TxGetSelectionBarWidth: TxGetSelectionBarWidth::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextHost as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Input_Ime", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for ITextHost {}
windows_core::imp::define_interface!(ITextHost2, ITextHost2_Vtbl, 0);
impl core::ops::Deref for ITextHost2 {
    type Target = ITextHost;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextHost2, windows_core::IUnknown, ITextHost);
impl ITextHost2 {
    pub unsafe fn TxIsDoubleClickPending(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxIsDoubleClickPending)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TxGetWindow(&self, phwnd: *mut super::super::super::Foundation::HWND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetWindow)(windows_core::Interface::as_raw(self), phwnd as _).ok() }
    }
    pub unsafe fn TxSetForegroundWindow(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxSetForegroundWindow)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxGetPalette(&self) -> super::super::super::Graphics::Gdi::HPALETTE {
        unsafe { (windows_core::Interface::vtable(self).TxGetPalette)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TxGetEastAsianFlags(&self, pflags: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetEastAsianFlags)(windows_core::Interface::as_raw(self), pflags as _).ok() }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn TxSetCursor2(&self, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: bool) -> super::super::WindowsAndMessaging::HCURSOR {
        unsafe { (windows_core::Interface::vtable(self).TxSetCursor2)(windows_core::Interface::as_raw(self), hcur, btext.into()) }
    }
    pub unsafe fn TxFreeTextServicesNotification(&self) {
        unsafe { (windows_core::Interface::vtable(self).TxFreeTextServicesNotification)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TxGetEditStyle(&self, dwitem: u32, pdwdata: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetEditStyle)(windows_core::Interface::as_raw(self), dwitem, pdwdata as _).ok() }
    }
    pub unsafe fn TxGetWindowStyles(&self, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetWindowStyles)(windows_core::Interface::as_raw(self), pdwstyle as _, pdwexstyle as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxShowDropCaret(&self, fshow: bool, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxShowDropCaret)(windows_core::Interface::as_raw(self), fshow.into(), hdc, prc as _).ok() }
    }
    pub unsafe fn TxDestroyCaret(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxDestroyCaret)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn TxGetHorzExtent(&self, plhorzextent: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetHorzExtent)(windows_core::Interface::as_raw(self), plhorzextent as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHost2_Vtbl {
    pub base__: ITextHost_Vtbl,
    pub TxIsDoubleClickPending: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub TxGetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub TxSetForegroundWindow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub TxGetPalette: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::super::Graphics::Gdi::HPALETTE,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    TxGetPalette: usize,
    pub TxGetEastAsianFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub TxSetCursor2: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::WindowsAndMessaging::HCURSOR, windows_core::BOOL) -> super::super::WindowsAndMessaging::HCURSOR,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    TxSetCursor2: usize,
    pub TxFreeTextServicesNotification: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub TxGetEditStyle: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub TxGetWindowStyles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub TxShowDropCaret: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, super::super::super::Graphics::Gdi::HDC, *mut super::super::super::Foundation::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    TxShowDropCaret: usize,
    pub TxDestroyCaret: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TxGetHorzExtent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Input_Ime", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITextHost2_Impl: ITextHost_Impl {
    fn TxIsDoubleClickPending(&self) -> windows_core::BOOL;
    fn TxGetWindow(&self, phwnd: *mut super::super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn TxSetForegroundWindow(&self) -> windows_core::Result<()>;
    fn TxGetPalette(&self) -> super::super::super::Graphics::Gdi::HPALETTE;
    fn TxGetEastAsianFlags(&self, pflags: *mut i32) -> windows_core::Result<()>;
    fn TxSetCursor2(&self, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: windows_core::BOOL) -> super::super::WindowsAndMessaging::HCURSOR;
    fn TxFreeTextServicesNotification(&self);
    fn TxGetEditStyle(&self, dwitem: u32, pdwdata: *mut u32) -> windows_core::Result<()>;
    fn TxGetWindowStyles(&self, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> windows_core::Result<()>;
    fn TxShowDropCaret(&self, fshow: windows_core::BOOL, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn TxDestroyCaret(&self) -> windows_core::Result<()>;
    fn TxGetHorzExtent(&self, plhorzextent: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Input_Ime", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITextHost2_Vtbl {
    pub const fn new<Identity: ITextHost2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TxIsDoubleClickPending<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxIsDoubleClickPending(this)
            }
        }
        unsafe extern "system" fn TxGetWindow<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxGetWindow(this, core::mem::transmute_copy(&phwnd)).into()
            }
        }
        unsafe extern "system" fn TxSetForegroundWindow<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxSetForegroundWindow(this).into()
            }
        }
        unsafe extern "system" fn TxGetPalette<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::super::Graphics::Gdi::HPALETTE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxGetPalette(this)
            }
        }
        unsafe extern "system" fn TxGetEastAsianFlags<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxGetEastAsianFlags(this, core::mem::transmute_copy(&pflags)).into()
            }
        }
        unsafe extern "system" fn TxSetCursor2<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: windows_core::BOOL) -> super::super::WindowsAndMessaging::HCURSOR {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxSetCursor2(this, core::mem::transmute_copy(&hcur), core::mem::transmute_copy(&btext))
            }
        }
        unsafe extern "system" fn TxFreeTextServicesNotification<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxFreeTextServicesNotification(this)
            }
        }
        unsafe extern "system" fn TxGetEditStyle<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwitem: u32, pdwdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxGetEditStyle(this, core::mem::transmute_copy(&dwitem), core::mem::transmute_copy(&pdwdata)).into()
            }
        }
        unsafe extern "system" fn TxGetWindowStyles<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxGetWindowStyles(this, core::mem::transmute_copy(&pdwstyle), core::mem::transmute_copy(&pdwexstyle)).into()
            }
        }
        unsafe extern "system" fn TxShowDropCaret<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: windows_core::BOOL, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxShowDropCaret(this, core::mem::transmute_copy(&fshow), core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&prc)).into()
            }
        }
        unsafe extern "system" fn TxDestroyCaret<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxDestroyCaret(this).into()
            }
        }
        unsafe extern "system" fn TxGetHorzExtent<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhorzextent: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxGetHorzExtent(this, core::mem::transmute_copy(&plhorzextent)).into()
            }
        }
        Self {
            base__: ITextHost_Vtbl::new::<Identity, OFFSET>(),
            TxIsDoubleClickPending: TxIsDoubleClickPending::<Identity, OFFSET>,
            TxGetWindow: TxGetWindow::<Identity, OFFSET>,
            TxSetForegroundWindow: TxSetForegroundWindow::<Identity, OFFSET>,
            TxGetPalette: TxGetPalette::<Identity, OFFSET>,
            TxGetEastAsianFlags: TxGetEastAsianFlags::<Identity, OFFSET>,
            TxSetCursor2: TxSetCursor2::<Identity, OFFSET>,
            TxFreeTextServicesNotification: TxFreeTextServicesNotification::<Identity, OFFSET>,
            TxGetEditStyle: TxGetEditStyle::<Identity, OFFSET>,
            TxGetWindowStyles: TxGetWindowStyles::<Identity, OFFSET>,
            TxShowDropCaret: TxShowDropCaret::<Identity, OFFSET>,
            TxDestroyCaret: TxDestroyCaret::<Identity, OFFSET>,
            TxGetHorzExtent: TxGetHorzExtent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextHost2 as windows_core::Interface>::IID || iid == &<ITextHost as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Input_Ime", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for ITextHost2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextPara, ITextPara_Vtbl, 0x8cc497c4_a1df_11ce_8098_00aa0047be5d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextPara {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextPara, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITextPara {
    pub unsafe fn GetDuplicate(&self) -> windows_core::Result<ITextPara> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuplicate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDuplicate<P0>(&self, ppara: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextPara>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDuplicate)(windows_core::Interface::as_raw(self), ppara.param().abi()).ok() }
    }
    pub unsafe fn CanChange(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEqual<P0>(&self, ppara: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ITextPara>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), ppara.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetStyle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStyle(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStyle)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetAlignment(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAlignment(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAlignment)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetHyphenation(&self) -> windows_core::Result<tomConstants> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHyphenation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHyphenation(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHyphenation)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetFirstLineIndent(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstLineIndent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetKeepTogether(&self) -> windows_core::Result<tomConstants> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeepTogether)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKeepTogether(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKeepTogether)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetKeepWithNext(&self) -> windows_core::Result<tomConstants> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeepWithNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKeepWithNext)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetLeftIndent(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLeftIndent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLineSpacing(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLineSpacing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLineSpacingRule(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLineSpacingRule)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetListAlignment(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetListAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetListAlignment(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetListAlignment)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetListLevelIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetListLevelIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetListLevelIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetListLevelIndex)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetListStart(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetListStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetListStart(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetListStart)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetListTab(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetListTab)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetListTab(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetListTab)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetListType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetListType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetListType(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetListType)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetNoLineNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNoLineNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNoLineNumber(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNoLineNumber)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetPageBreakBefore(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPageBreakBefore)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPageBreakBefore(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPageBreakBefore)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetRightIndent(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRightIndent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRightIndent(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRightIndent)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn SetIndents(&self, first: f32, left: f32, right: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIndents)(windows_core::Interface::as_raw(self), first, left, right).ok() }
    }
    pub unsafe fn SetLineSpacing(&self, rule: i32, spacing: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLineSpacing)(windows_core::Interface::as_raw(self), rule, spacing).ok() }
    }
    pub unsafe fn GetSpaceAfter(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpaceAfter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSpaceAfter(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSpaceAfter)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetSpaceBefore(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpaceBefore)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSpaceBefore(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSpaceBefore)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetWidowControl(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWidowControl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWidowControl(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWidowControl)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetTabCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTabCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddTab(&self, tbpos: f32, tbalign: i32, tbleader: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddTab)(windows_core::Interface::as_raw(self), tbpos, tbalign, tbleader).ok() }
    }
    pub unsafe fn ClearAllTabs(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearAllTabs)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DeleteTab(&self, tbpos: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteTab)(windows_core::Interface::as_raw(self), tbpos).ok() }
    }
    pub unsafe fn GetTab(&self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTab)(windows_core::Interface::as_raw(self), itab, ptbpos as _, ptbalign as _, ptbleader as _).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextPara_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub GetDuplicate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuplicate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetHyphenation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut tomConstants) -> windows_core::HRESULT,
    pub SetHyphenation: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetFirstLineIndent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetKeepTogether: unsafe extern "system" fn(*mut core::ffi::c_void, *mut tomConstants) -> windows_core::HRESULT,
    pub SetKeepTogether: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetKeepWithNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut tomConstants) -> windows_core::HRESULT,
    pub SetKeepWithNext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetLeftIndent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetLineSpacingRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetListAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetListAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetListLevelIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetListLevelIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetListStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetListStart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetListTab: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetListTab: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetListType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetListType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetNoLineNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetNoLineNumber: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPageBreakBefore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPageBreakBefore: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetRightIndent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetRightIndent: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetIndents: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32) -> windows_core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, i32, f32) -> windows_core::HRESULT,
    pub GetSpaceAfter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpaceAfter: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetSpaceBefore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpaceBefore: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetWidowControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWidowControl: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetTabCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AddTab: unsafe extern "system" fn(*mut core::ffi::c_void, f32, i32, i32) -> windows_core::HRESULT,
    pub ClearAllTabs: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteTab: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetTab: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut f32, *mut i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextPara_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn GetDuplicate(&self) -> windows_core::Result<ITextPara>;
    fn SetDuplicate(&self, ppara: windows_core::Ref<'_, ITextPara>) -> windows_core::Result<()>;
    fn CanChange(&self) -> windows_core::Result<i32>;
    fn IsEqual(&self, ppara: windows_core::Ref<'_, ITextPara>) -> windows_core::Result<i32>;
    fn Reset(&self, value: i32) -> windows_core::Result<()>;
    fn GetStyle(&self) -> windows_core::Result<i32>;
    fn SetStyle(&self, value: i32) -> windows_core::Result<()>;
    fn GetAlignment(&self) -> windows_core::Result<i32>;
    fn SetAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetHyphenation(&self) -> windows_core::Result<tomConstants>;
    fn SetHyphenation(&self, value: i32) -> windows_core::Result<()>;
    fn GetFirstLineIndent(&self) -> windows_core::Result<f32>;
    fn GetKeepTogether(&self) -> windows_core::Result<tomConstants>;
    fn SetKeepTogether(&self, value: i32) -> windows_core::Result<()>;
    fn GetKeepWithNext(&self) -> windows_core::Result<tomConstants>;
    fn SetKeepWithNext(&self, value: i32) -> windows_core::Result<()>;
    fn GetLeftIndent(&self) -> windows_core::Result<f32>;
    fn GetLineSpacing(&self) -> windows_core::Result<f32>;
    fn GetLineSpacingRule(&self) -> windows_core::Result<i32>;
    fn GetListAlignment(&self) -> windows_core::Result<i32>;
    fn SetListAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetListLevelIndex(&self) -> windows_core::Result<i32>;
    fn SetListLevelIndex(&self, value: i32) -> windows_core::Result<()>;
    fn GetListStart(&self) -> windows_core::Result<i32>;
    fn SetListStart(&self, value: i32) -> windows_core::Result<()>;
    fn GetListTab(&self) -> windows_core::Result<f32>;
    fn SetListTab(&self, value: f32) -> windows_core::Result<()>;
    fn GetListType(&self) -> windows_core::Result<i32>;
    fn SetListType(&self, value: i32) -> windows_core::Result<()>;
    fn GetNoLineNumber(&self) -> windows_core::Result<i32>;
    fn SetNoLineNumber(&self, value: i32) -> windows_core::Result<()>;
    fn GetPageBreakBefore(&self) -> windows_core::Result<i32>;
    fn SetPageBreakBefore(&self, value: i32) -> windows_core::Result<()>;
    fn GetRightIndent(&self) -> windows_core::Result<f32>;
    fn SetRightIndent(&self, value: f32) -> windows_core::Result<()>;
    fn SetIndents(&self, first: f32, left: f32, right: f32) -> windows_core::Result<()>;
    fn SetLineSpacing(&self, rule: i32, spacing: f32) -> windows_core::Result<()>;
    fn GetSpaceAfter(&self) -> windows_core::Result<f32>;
    fn SetSpaceAfter(&self, value: f32) -> windows_core::Result<()>;
    fn GetSpaceBefore(&self) -> windows_core::Result<f32>;
    fn SetSpaceBefore(&self, value: f32) -> windows_core::Result<()>;
    fn GetWidowControl(&self) -> windows_core::Result<i32>;
    fn SetWidowControl(&self, value: i32) -> windows_core::Result<()>;
    fn GetTabCount(&self) -> windows_core::Result<i32>;
    fn AddTab(&self, tbpos: f32, tbalign: i32, tbleader: i32) -> windows_core::Result<()>;
    fn ClearAllTabs(&self) -> windows_core::Result<()>;
    fn DeleteTab(&self, tbpos: f32) -> windows_core::Result<()>;
    fn GetTab(&self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextPara_Vtbl {
    pub const fn new<Identity: ITextPara_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDuplicate<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetDuplicate(this) {
                    Ok(ok__) => {
                        pppara.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDuplicate<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetDuplicate(this, core::mem::transmute_copy(&ppara)).into()
            }
        }
        unsafe extern "system" fn CanChange<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::CanChange(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::IsEqual(this, core::mem::transmute_copy(&ppara)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::Reset(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetStyle<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetStyle(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStyle<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetStyle(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetAlignment<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetAlignment(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetAlignment(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetHyphenation<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut tomConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetHyphenation(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHyphenation<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetHyphenation(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetFirstLineIndent<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetFirstLineIndent(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetKeepTogether<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut tomConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetKeepTogether(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeepTogether<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetKeepTogether(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetKeepWithNext<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut tomConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetKeepWithNext(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetKeepWithNext(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetLeftIndent<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetLeftIndent(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLineSpacing<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetLineSpacing(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLineSpacingRule<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetLineSpacingRule(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetListAlignment<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetListAlignment(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListAlignment<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetListAlignment(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetListLevelIndex<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetListLevelIndex(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListLevelIndex<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetListLevelIndex(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetListStart<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetListStart(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListStart<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetListStart(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetListTab<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetListTab(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListTab<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetListTab(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetListType<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetListType(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListType<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetListType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetNoLineNumber<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetNoLineNumber(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNoLineNumber<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetNoLineNumber(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetPageBreakBefore<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetPageBreakBefore(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPageBreakBefore<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetPageBreakBefore(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetRightIndent<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetRightIndent(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRightIndent<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetRightIndent(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetIndents<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, first: f32, left: f32, right: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetIndents(this, core::mem::transmute_copy(&first), core::mem::transmute_copy(&left), core::mem::transmute_copy(&right)).into()
            }
        }
        unsafe extern "system" fn SetLineSpacing<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rule: i32, spacing: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetLineSpacing(this, core::mem::transmute_copy(&rule), core::mem::transmute_copy(&spacing)).into()
            }
        }
        unsafe extern "system" fn GetSpaceAfter<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetSpaceAfter(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpaceAfter<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetSpaceAfter(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSpaceBefore<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetSpaceBefore(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpaceBefore<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetSpaceBefore(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetWidowControl<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetWidowControl(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWidowControl<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetWidowControl(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetTabCount<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetTabCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTab<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tbpos: f32, tbalign: i32, tbleader: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::AddTab(this, core::mem::transmute_copy(&tbpos), core::mem::transmute_copy(&tbalign), core::mem::transmute_copy(&tbleader)).into()
            }
        }
        unsafe extern "system" fn ClearAllTabs<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::ClearAllTabs(this).into()
            }
        }
        unsafe extern "system" fn DeleteTab<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tbpos: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::DeleteTab(this, core::mem::transmute_copy(&tbpos)).into()
            }
        }
        unsafe extern "system" fn GetTab<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::GetTab(this, core::mem::transmute_copy(&itab), core::mem::transmute_copy(&ptbpos), core::mem::transmute_copy(&ptbalign), core::mem::transmute_copy(&ptbleader)).into()
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetDuplicate: GetDuplicate::<Identity, OFFSET>,
            SetDuplicate: SetDuplicate::<Identity, OFFSET>,
            CanChange: CanChange::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            GetStyle: GetStyle::<Identity, OFFSET>,
            SetStyle: SetStyle::<Identity, OFFSET>,
            GetAlignment: GetAlignment::<Identity, OFFSET>,
            SetAlignment: SetAlignment::<Identity, OFFSET>,
            GetHyphenation: GetHyphenation::<Identity, OFFSET>,
            SetHyphenation: SetHyphenation::<Identity, OFFSET>,
            GetFirstLineIndent: GetFirstLineIndent::<Identity, OFFSET>,
            GetKeepTogether: GetKeepTogether::<Identity, OFFSET>,
            SetKeepTogether: SetKeepTogether::<Identity, OFFSET>,
            GetKeepWithNext: GetKeepWithNext::<Identity, OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Identity, OFFSET>,
            GetLeftIndent: GetLeftIndent::<Identity, OFFSET>,
            GetLineSpacing: GetLineSpacing::<Identity, OFFSET>,
            GetLineSpacingRule: GetLineSpacingRule::<Identity, OFFSET>,
            GetListAlignment: GetListAlignment::<Identity, OFFSET>,
            SetListAlignment: SetListAlignment::<Identity, OFFSET>,
            GetListLevelIndex: GetListLevelIndex::<Identity, OFFSET>,
            SetListLevelIndex: SetListLevelIndex::<Identity, OFFSET>,
            GetListStart: GetListStart::<Identity, OFFSET>,
            SetListStart: SetListStart::<Identity, OFFSET>,
            GetListTab: GetListTab::<Identity, OFFSET>,
            SetListTab: SetListTab::<Identity, OFFSET>,
            GetListType: GetListType::<Identity, OFFSET>,
            SetListType: SetListType::<Identity, OFFSET>,
            GetNoLineNumber: GetNoLineNumber::<Identity, OFFSET>,
            SetNoLineNumber: SetNoLineNumber::<Identity, OFFSET>,
            GetPageBreakBefore: GetPageBreakBefore::<Identity, OFFSET>,
            SetPageBreakBefore: SetPageBreakBefore::<Identity, OFFSET>,
            GetRightIndent: GetRightIndent::<Identity, OFFSET>,
            SetRightIndent: SetRightIndent::<Identity, OFFSET>,
            SetIndents: SetIndents::<Identity, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
            GetSpaceAfter: GetSpaceAfter::<Identity, OFFSET>,
            SetSpaceAfter: SetSpaceAfter::<Identity, OFFSET>,
            GetSpaceBefore: GetSpaceBefore::<Identity, OFFSET>,
            SetSpaceBefore: SetSpaceBefore::<Identity, OFFSET>,
            GetWidowControl: GetWidowControl::<Identity, OFFSET>,
            SetWidowControl: SetWidowControl::<Identity, OFFSET>,
            GetTabCount: GetTabCount::<Identity, OFFSET>,
            AddTab: AddTab::<Identity, OFFSET>,
            ClearAllTabs: ClearAllTabs::<Identity, OFFSET>,
            DeleteTab: DeleteTab::<Identity, OFFSET>,
            GetTab: GetTab::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextPara as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextPara {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextPara2, ITextPara2_Vtbl, 0xc241f5e4_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextPara2 {
    type Target = ITextPara;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextPara2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextPara);
#[cfg(feature = "Win32_System_Com")]
impl ITextPara2 {
    pub unsafe fn GetBorders(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBorders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDuplicate2(&self) -> windows_core::Result<ITextPara2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuplicate2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDuplicate2<P0>(&self, ppara: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextPara2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDuplicate2)(windows_core::Interface::as_raw(self), ppara.param().abi()).ok() }
    }
    pub unsafe fn GetFontAlignment(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFontAlignment(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFontAlignment)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetHangingPunctuation(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHangingPunctuation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHangingPunctuation(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHangingPunctuation)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetSnapToGrid(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSnapToGrid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSnapToGrid(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSnapToGrid)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetTrimPunctuationAtStart(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTrimPunctuationAtStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTrimPunctuationAtStart(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTrimPunctuationAtStart)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEffects)(windows_core::Interface::as_raw(self), pvalue as _, pmask as _).ok() }
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEqual2<P0>(&self, ppara: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ITextPara2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual2)(windows_core::Interface::as_raw(self), ppara.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEffects(&self, value: i32, mask: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEffects)(windows_core::Interface::as_raw(self), value, mask).ok() }
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), r#type, value).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextPara2_Vtbl {
    pub base__: ITextPara_Vtbl,
    pub GetBorders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDuplicate2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuplicate2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFontAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetHangingPunctuation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHangingPunctuation: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSnapToGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSnapToGrid: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetTrimPunctuationAtStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTrimPunctuationAtStart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub IsEqual2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextPara2_Impl: ITextPara_Impl {
    fn GetBorders(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetDuplicate2(&self) -> windows_core::Result<ITextPara2>;
    fn SetDuplicate2(&self, ppara: windows_core::Ref<'_, ITextPara2>) -> windows_core::Result<()>;
    fn GetFontAlignment(&self) -> windows_core::Result<i32>;
    fn SetFontAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetHangingPunctuation(&self) -> windows_core::Result<i32>;
    fn SetHangingPunctuation(&self, value: i32) -> windows_core::Result<()>;
    fn GetSnapToGrid(&self) -> windows_core::Result<i32>;
    fn SetSnapToGrid(&self, value: i32) -> windows_core::Result<()>;
    fn GetTrimPunctuationAtStart(&self) -> windows_core::Result<i32>;
    fn SetTrimPunctuationAtStart(&self, value: i32) -> windows_core::Result<()>;
    fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn IsEqual2(&self, ppara: windows_core::Ref<'_, ITextPara2>) -> windows_core::Result<i32>;
    fn SetEffects(&self, value: i32, mask: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextPara2_Vtbl {
    pub const fn new<Identity: ITextPara2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBorders<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppborders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetBorders(this) {
                    Ok(ok__) => {
                        ppborders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDuplicate2<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetDuplicate2(this) {
                    Ok(ok__) => {
                        pppara.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDuplicate2<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetDuplicate2(this, core::mem::transmute_copy(&ppara)).into()
            }
        }
        unsafe extern "system" fn GetFontAlignment<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetFontAlignment(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFontAlignment<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetFontAlignment(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetHangingPunctuation<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetHangingPunctuation(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHangingPunctuation<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetHangingPunctuation(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSnapToGrid<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetSnapToGrid(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSnapToGrid<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetSnapToGrid(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetTrimPunctuationAtStart<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetTrimPunctuationAtStart(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTrimPunctuationAtStart<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetTrimPunctuationAtStart(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetEffects<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::GetEffects(this, core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pmask)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual2<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void, pb: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::IsEqual2(this, core::mem::transmute_copy(&ppara)) {
                    Ok(ok__) => {
                        pb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEffects<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32, mask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetEffects(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&mask)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: ITextPara_Vtbl::new::<Identity, OFFSET>(),
            GetBorders: GetBorders::<Identity, OFFSET>,
            GetDuplicate2: GetDuplicate2::<Identity, OFFSET>,
            SetDuplicate2: SetDuplicate2::<Identity, OFFSET>,
            GetFontAlignment: GetFontAlignment::<Identity, OFFSET>,
            SetFontAlignment: SetFontAlignment::<Identity, OFFSET>,
            GetHangingPunctuation: GetHangingPunctuation::<Identity, OFFSET>,
            SetHangingPunctuation: SetHangingPunctuation::<Identity, OFFSET>,
            GetSnapToGrid: GetSnapToGrid::<Identity, OFFSET>,
            SetSnapToGrid: SetSnapToGrid::<Identity, OFFSET>,
            GetTrimPunctuationAtStart: GetTrimPunctuationAtStart::<Identity, OFFSET>,
            SetTrimPunctuationAtStart: SetTrimPunctuationAtStart::<Identity, OFFSET>,
            GetEffects: GetEffects::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            IsEqual2: IsEqual2::<Identity, OFFSET>,
            SetEffects: SetEffects::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextPara2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextPara as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextPara2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextRange, ITextRange_Vtbl, 0x8cc497c2_a1df_11ce_8098_00aa0047be5d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextRange {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextRange, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITextRange {
    pub unsafe fn GetText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetText(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)).ok() }
    }
    pub unsafe fn GetChar(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetChar(&self, char: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetChar)(windows_core::Interface::as_raw(self), char).ok() }
    }
    pub unsafe fn GetDuplicate(&self) -> windows_core::Result<ITextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuplicate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFormattedText(&self) -> windows_core::Result<ITextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormattedText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFormattedText<P0>(&self, prange: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextRange>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFormattedText)(windows_core::Interface::as_raw(self), prange.param().abi()).ok() }
    }
    pub unsafe fn GetStart(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStart(&self, cpfirst: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStart)(windows_core::Interface::as_raw(self), cpfirst).ok() }
    }
    pub unsafe fn GetEnd(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnd(&self, cplim: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnd)(windows_core::Interface::as_raw(self), cplim).ok() }
    }
    pub unsafe fn GetFont(&self) -> windows_core::Result<ITextFont> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFont)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFont<P0>(&self, pfont: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextFont>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFont)(windows_core::Interface::as_raw(self), pfont.param().abi()).ok() }
    }
    pub unsafe fn GetPara(&self) -> windows_core::Result<ITextPara> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPara)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPara<P0>(&self, ppara: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextPara>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPara)(windows_core::Interface::as_raw(self), ppara.param().abi()).ok() }
    }
    pub unsafe fn GetStoryLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStoryLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStoryType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStoryType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Collapse(&self, bstart: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Collapse)(windows_core::Interface::as_raw(self), bstart).ok() }
    }
    pub unsafe fn Expand(&self, unit: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self), unit, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIndex(&self, unit: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndex)(windows_core::Interface::as_raw(self), unit, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIndex)(windows_core::Interface::as_raw(self), unit, index, extend).ok() }
    }
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRange)(windows_core::Interface::as_raw(self), cpanchor, cpactive).ok() }
    }
    pub unsafe fn InRange<P0>(&self, prange: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ITextRange>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InRange)(windows_core::Interface::as_raw(self), prange.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InStory<P0>(&self, prange: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ITextRange>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InStory)(windows_core::Interface::as_raw(self), prange.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEqual<P0>(&self, prange: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ITextRange>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), prange.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Select(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartOf)(windows_core::Interface::as_raw(self), unit, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndOf)(windows_core::Interface::as_raw(self), unit, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Move(&self, unit: i32, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), unit, count, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveStart)(windows_core::Interface::as_raw(self), unit, count, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveEnd)(windows_core::Interface::as_raw(self), unit, count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveWhile)(windows_core::Interface::as_raw(self), core::mem::transmute(cset), count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveStartWhile)(windows_core::Interface::as_raw(self), core::mem::transmute(cset), count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveEndWhile)(windows_core::Interface::as_raw(self), core::mem::transmute(cset), count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveUntil)(windows_core::Interface::as_raw(self), core::mem::transmute(cset), count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveStartUntil)(windows_core::Interface::as_raw(self), core::mem::transmute(cset), count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveEndUntil)(windows_core::Interface::as_raw(self), core::mem::transmute(cset), count, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FindText(&self, bstr: &windows_core::BSTR, count: i32, flags: tomConstants) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr), count, flags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FindTextStart(&self, bstr: &windows_core::BSTR, count: i32, flags: tomConstants) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindTextStart)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr), count, flags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FindTextEnd(&self, bstr: &windows_core::BSTR, count: i32, flags: tomConstants) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindTextEnd)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr), count, flags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), unit, count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Cut(&self) -> windows_core::Result<super::super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cut)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Copy(&self) -> windows_core::Result<super::super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Paste)(windows_core::Interface::as_raw(self), core::mem::transmute(pvar), format).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanPaste)(windows_core::Interface::as_raw(self), core::mem::transmute(pvar), format, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanEdit(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanEdit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ChangeCase(&self, r#type: tomConstants) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ChangeCase)(windows_core::Interface::as_raw(self), r#type).ok() }
    }
    pub unsafe fn GetPoint(&self, r#type: tomConstants, px: *mut i32, py: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPoint)(windows_core::Interface::as_raw(self), r#type, px as _, py as _).ok() }
    }
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: tomConstants, extend: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPoint)(windows_core::Interface::as_raw(self), x, y, r#type, extend).ok() }
    }
    pub unsafe fn ScrollIntoView(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ScrollIntoView)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetEmbeddedObject(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEmbeddedObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextRange_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetChar: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDuplicate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEnd: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPara: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPara: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStoryLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetStoryType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Collapse: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub SetRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub InRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub InStory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartOf: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub EndOf: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveStart: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveEnd: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MoveWhile: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::System::Variant::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MoveWhile: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MoveStartWhile: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::System::Variant::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MoveStartWhile: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MoveEndWhile: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::System::Variant::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MoveEndWhile: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MoveUntil: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::System::Variant::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MoveUntil: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MoveStartUntil: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::System::Variant::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MoveStartUntil: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MoveEndUntil: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::System::Variant::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MoveEndUntil: usize,
    pub FindText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, tomConstants, *mut i32) -> windows_core::HRESULT,
    pub FindTextStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, tomConstants, *mut i32) -> windows_core::HRESULT,
    pub FindTextEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, tomConstants, *mut i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Cut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Cut: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Copy: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Paste: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::System::Variant::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Paste: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CanPaste: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::System::Variant::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CanPaste: usize,
    pub CanEdit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ChangeCase: unsafe extern "system" fn(*mut core::ffi::c_void, tomConstants) -> windows_core::HRESULT,
    pub GetPoint: unsafe extern "system" fn(*mut core::ffi::c_void, tomConstants, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetPoint: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, tomConstants, i32) -> windows_core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetEmbeddedObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextRange_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn GetText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetText(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetChar(&self) -> windows_core::Result<i32>;
    fn SetChar(&self, char: i32) -> windows_core::Result<()>;
    fn GetDuplicate(&self) -> windows_core::Result<ITextRange>;
    fn GetFormattedText(&self) -> windows_core::Result<ITextRange>;
    fn SetFormattedText(&self, prange: windows_core::Ref<'_, ITextRange>) -> windows_core::Result<()>;
    fn GetStart(&self) -> windows_core::Result<i32>;
    fn SetStart(&self, cpfirst: i32) -> windows_core::Result<()>;
    fn GetEnd(&self) -> windows_core::Result<i32>;
    fn SetEnd(&self, cplim: i32) -> windows_core::Result<()>;
    fn GetFont(&self) -> windows_core::Result<ITextFont>;
    fn SetFont(&self, pfont: windows_core::Ref<'_, ITextFont>) -> windows_core::Result<()>;
    fn GetPara(&self) -> windows_core::Result<ITextPara>;
    fn SetPara(&self, ppara: windows_core::Ref<'_, ITextPara>) -> windows_core::Result<()>;
    fn GetStoryLength(&self) -> windows_core::Result<i32>;
    fn GetStoryType(&self) -> windows_core::Result<i32>;
    fn Collapse(&self, bstart: i32) -> windows_core::Result<()>;
    fn Expand(&self, unit: i32) -> windows_core::Result<i32>;
    fn GetIndex(&self, unit: i32) -> windows_core::Result<i32>;
    fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> windows_core::Result<()>;
    fn SetRange(&self, cpanchor: i32, cpactive: i32) -> windows_core::Result<()>;
    fn InRange(&self, prange: windows_core::Ref<'_, ITextRange>) -> windows_core::Result<i32>;
    fn InStory(&self, prange: windows_core::Ref<'_, ITextRange>) -> windows_core::Result<i32>;
    fn IsEqual(&self, prange: windows_core::Ref<'_, ITextRange>) -> windows_core::Result<i32>;
    fn Select(&self) -> windows_core::Result<()>;
    fn StartOf(&self, unit: i32, extend: i32) -> windows_core::Result<i32>;
    fn EndOf(&self, unit: i32, extend: i32) -> windows_core::Result<i32>;
    fn Move(&self, unit: i32, count: i32) -> windows_core::Result<i32>;
    fn MoveStart(&self, unit: i32, count: i32) -> windows_core::Result<i32>;
    fn MoveEnd(&self, unit: i32, count: i32) -> windows_core::Result<i32>;
    fn MoveWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveStartWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveEndWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveStartUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveEndUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn FindText(&self, bstr: &windows_core::BSTR, count: i32, flags: tomConstants) -> windows_core::Result<i32>;
    fn FindTextStart(&self, bstr: &windows_core::BSTR, count: i32, flags: tomConstants) -> windows_core::Result<i32>;
    fn FindTextEnd(&self, bstr: &windows_core::BSTR, count: i32, flags: tomConstants) -> windows_core::Result<i32>;
    fn Delete(&self, unit: i32, count: i32) -> windows_core::Result<i32>;
    fn Cut(&self) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn Copy(&self) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn Paste(&self, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> windows_core::Result<()>;
    fn CanPaste(&self, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> windows_core::Result<i32>;
    fn CanEdit(&self) -> windows_core::Result<i32>;
    fn ChangeCase(&self, r#type: tomConstants) -> windows_core::Result<()>;
    fn GetPoint(&self, r#type: tomConstants, px: *mut i32, py: *mut i32) -> windows_core::Result<()>;
    fn SetPoint(&self, x: i32, y: i32, r#type: tomConstants, extend: i32) -> windows_core::Result<()>;
    fn ScrollIntoView(&self, value: i32) -> windows_core::Result<()>;
    fn GetEmbeddedObject(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextRange_Vtbl {
    pub const fn new<Identity: ITextRange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetText(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetText(this, core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn GetChar<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchar: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetChar(this) {
                    Ok(ok__) => {
                        pchar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChar<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, char: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetChar(this, core::mem::transmute_copy(&char)).into()
            }
        }
        unsafe extern "system" fn GetDuplicate<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetDuplicate(this) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetFormattedText(this) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetFormattedText(this, core::mem::transmute_copy(&prange)).into()
            }
        }
        unsafe extern "system" fn GetStart<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpfirst: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetStart(this) {
                    Ok(ok__) => {
                        pcpfirst.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStart<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpfirst: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetStart(this, core::mem::transmute_copy(&cpfirst)).into()
            }
        }
        unsafe extern "system" fn GetEnd<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcplim: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetEnd(this) {
                    Ok(ok__) => {
                        pcplim.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnd<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cplim: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetEnd(this, core::mem::transmute_copy(&cplim)).into()
            }
        }
        unsafe extern "system" fn GetFont<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetFont(this) {
                    Ok(ok__) => {
                        ppfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFont<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetFont(this, core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn GetPara<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetPara(this) {
                    Ok(ok__) => {
                        pppara.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPara<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetPara(this, core::mem::transmute_copy(&ppara)).into()
            }
        }
        unsafe extern "system" fn GetStoryLength<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetStoryLength(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStoryType<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetStoryType(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Collapse<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstart: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::Collapse(this, core::mem::transmute_copy(&bstart)).into()
            }
        }
        unsafe extern "system" fn Expand<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Expand(this, core::mem::transmute_copy(&unit)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIndex<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, pindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetIndex(this, core::mem::transmute_copy(&unit)) {
                    Ok(ok__) => {
                        pindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIndex<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, index: i32, extend: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetIndex(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&index), core::mem::transmute_copy(&extend)).into()
            }
        }
        unsafe extern "system" fn SetRange<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpanchor: i32, cpactive: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetRange(this, core::mem::transmute_copy(&cpanchor), core::mem::transmute_copy(&cpactive)).into()
            }
        }
        unsafe extern "system" fn InRange<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::InRange(this, core::mem::transmute_copy(&prange)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InStory<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::InStory(this, core::mem::transmute_copy(&prange)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::IsEqual(this, core::mem::transmute_copy(&prange)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Select<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::Select(this).into()
            }
        }
        unsafe extern "system" fn StartOf<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::StartOf(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndOf<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::EndOf(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Move<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Move(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveStart<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveStart(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveEnd<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveEnd(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveWhile<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const super::super::super::System::Variant::VARIANT, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveWhile(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveStartWhile<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const super::super::super::System::Variant::VARIANT, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveStartWhile(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveEndWhile<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const super::super::super::System::Variant::VARIANT, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveEndWhile(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveUntil<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const super::super::super::System::Variant::VARIANT, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveUntil(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveStartUntil<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const super::super::super::System::Variant::VARIANT, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveStartUntil(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveEndUntil<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const super::super::super::System::Variant::VARIANT, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveEndUntil(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void, count: i32, flags: tomConstants, plength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::FindText(this, core::mem::transmute(&bstr), core::mem::transmute_copy(&count), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        plength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindTextStart<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void, count: i32, flags: tomConstants, plength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::FindTextStart(this, core::mem::transmute(&bstr), core::mem::transmute_copy(&count), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        plength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindTextEnd<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void, count: i32, flags: tomConstants, plength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::FindTextEnd(this, core::mem::transmute(&bstr), core::mem::transmute_copy(&count), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        plength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Delete(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cut<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Cut(this) {
                    Ok(ok__) => {
                        pvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Copy<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut super::super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Copy(this) {
                    Ok(ok__) => {
                        pvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Paste<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::Paste(this, core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn CanPaste<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const super::super::super::System::Variant::VARIANT, format: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::CanPaste(this, core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanEdit<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::CanEdit(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChangeCase<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: tomConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::ChangeCase(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn GetPoint<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: tomConstants, px: *mut i32, py: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::GetPoint(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&px), core::mem::transmute_copy(&py)).into()
            }
        }
        unsafe extern "system" fn SetPoint<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, r#type: tomConstants, extend: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetPoint(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&extend)).into()
            }
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::ScrollIntoView(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetEmbeddedObject<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetEmbeddedObject(this) {
                    Ok(ok__) => {
                        ppobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetChar: GetChar::<Identity, OFFSET>,
            SetChar: SetChar::<Identity, OFFSET>,
            GetDuplicate: GetDuplicate::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, OFFSET>,
            GetStart: GetStart::<Identity, OFFSET>,
            SetStart: SetStart::<Identity, OFFSET>,
            GetEnd: GetEnd::<Identity, OFFSET>,
            SetEnd: SetEnd::<Identity, OFFSET>,
            GetFont: GetFont::<Identity, OFFSET>,
            SetFont: SetFont::<Identity, OFFSET>,
            GetPara: GetPara::<Identity, OFFSET>,
            SetPara: SetPara::<Identity, OFFSET>,
            GetStoryLength: GetStoryLength::<Identity, OFFSET>,
            GetStoryType: GetStoryType::<Identity, OFFSET>,
            Collapse: Collapse::<Identity, OFFSET>,
            Expand: Expand::<Identity, OFFSET>,
            GetIndex: GetIndex::<Identity, OFFSET>,
            SetIndex: SetIndex::<Identity, OFFSET>,
            SetRange: SetRange::<Identity, OFFSET>,
            InRange: InRange::<Identity, OFFSET>,
            InStory: InStory::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Select: Select::<Identity, OFFSET>,
            StartOf: StartOf::<Identity, OFFSET>,
            EndOf: EndOf::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            MoveStart: MoveStart::<Identity, OFFSET>,
            MoveEnd: MoveEnd::<Identity, OFFSET>,
            MoveWhile: MoveWhile::<Identity, OFFSET>,
            MoveStartWhile: MoveStartWhile::<Identity, OFFSET>,
            MoveEndWhile: MoveEndWhile::<Identity, OFFSET>,
            MoveUntil: MoveUntil::<Identity, OFFSET>,
            MoveStartUntil: MoveStartUntil::<Identity, OFFSET>,
            MoveEndUntil: MoveEndUntil::<Identity, OFFSET>,
            FindText: FindText::<Identity, OFFSET>,
            FindTextStart: FindTextStart::<Identity, OFFSET>,
            FindTextEnd: FindTextEnd::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Cut: Cut::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Paste: Paste::<Identity, OFFSET>,
            CanPaste: CanPaste::<Identity, OFFSET>,
            CanEdit: CanEdit::<Identity, OFFSET>,
            ChangeCase: ChangeCase::<Identity, OFFSET>,
            GetPoint: GetPoint::<Identity, OFFSET>,
            SetPoint: SetPoint::<Identity, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, OFFSET>,
            GetEmbeddedObject: GetEmbeddedObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextRange as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextRange {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextRange2, ITextRange2_Vtbl, 0xc241f5e2_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextRange2 {
    type Target = ITextSelection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextRange2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextRange, ITextSelection);
#[cfg(feature = "Win32_System_Com")]
impl ITextRange2 {
    pub unsafe fn GetCch(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCch)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCells(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCells)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetColumn(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumn)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDuplicate2(&self) -> windows_core::Result<ITextRange2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuplicate2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFont2(&self) -> windows_core::Result<ITextFont2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFont2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFont2<P0>(&self, pfont: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextFont2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFont2)(windows_core::Interface::as_raw(self), pfont.param().abi()).ok() }
    }
    pub unsafe fn GetFormattedText2(&self) -> windows_core::Result<ITextRange2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormattedText2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFormattedText2<P0>(&self, prange: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextRange2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFormattedText2)(windows_core::Interface::as_raw(self), prange.param().abi()).ok() }
    }
    pub unsafe fn GetGravity(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGravity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGravity(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGravity)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetPara2(&self) -> windows_core::Result<ITextPara2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPara2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPara2<P0>(&self, ppara: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextPara2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPara2)(windows_core::Interface::as_raw(self), ppara.param().abi()).ok() }
    }
    pub unsafe fn GetRow(&self) -> windows_core::Result<ITextRow> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRow)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStartPara(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStartPara)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTable(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTable)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetURL(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetURL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)).ok() }
    }
    pub unsafe fn AddSubrange(&self, cp1: i32, cp2: i32, activate: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddSubrange)(windows_core::Interface::as_raw(self), cp1, cp2, activate).ok() }
    }
    pub unsafe fn BuildUpMath(&self, flags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BuildUpMath)(windows_core::Interface::as_raw(self), flags).ok() }
    }
    pub unsafe fn DeleteSubrange(&self, cpfirst: i32, cplim: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteSubrange)(windows_core::Interface::as_raw(self), cpfirst, cplim).ok() }
    }
    pub unsafe fn Find<P0>(&self, prange: P0, count: i32, flags: i32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ITextRange2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Find)(windows_core::Interface::as_raw(self), prange.param().abi(), count, flags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChar2(&self, pchar: *mut i32, offset: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetChar2)(windows_core::Interface::as_raw(self), pchar as _, offset).ok() }
    }
    pub unsafe fn GetDropCap(&self, pcline: *mut i32, pposition: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDropCap)(windows_core::Interface::as_raw(self), pcline as _, pposition as _).ok() }
    }
    pub unsafe fn GetInlineObject(&self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInlineObject)(windows_core::Interface::as_raw(self), ptype as _, palign as _, pchar as _, pchar1 as _, pchar2 as _, pcount as _, ptexstyle as _, pccol as _, plevel as _).ok() }
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRect)(windows_core::Interface::as_raw(self), r#type, pleft as _, ptop as _, pright as _, pbottom as _, phit as _).ok() }
    }
    pub unsafe fn GetSubrange(&self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSubrange)(windows_core::Interface::as_raw(self), isubrange, pcpfirst as _, pcplim as _).ok() }
    }
    pub unsafe fn GetText2(&self, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText2)(windows_core::Interface::as_raw(self), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn HexToUnicode(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HexToUnicode)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn InsertTable(&self, ccol: i32, crow: i32, autofit: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InsertTable)(windows_core::Interface::as_raw(self), ccol, crow, autofit).ok() }
    }
    pub unsafe fn Linearize(&self, flags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Linearize)(windows_core::Interface::as_raw(self), flags).ok() }
    }
    pub unsafe fn SetActiveSubrange(&self, cpanchor: i32, cpactive: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetActiveSubrange)(windows_core::Interface::as_raw(self), cpanchor, cpactive).ok() }
    }
    pub unsafe fn SetDropCap(&self, cline: i32, position: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDropCap)(windows_core::Interface::as_raw(self), cline, position).ok() }
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), r#type, value).ok() }
    }
    pub unsafe fn SetText2(&self, flags: i32, bstr: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetText2)(windows_core::Interface::as_raw(self), flags, core::mem::transmute_copy(bstr)).ok() }
    }
    pub unsafe fn UnicodeToHex(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnicodeToHex)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetInlineObject(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInlineObject)(windows_core::Interface::as_raw(self), r#type, align, char, char1, char2, count, texstyle, ccol).ok() }
    }
    pub unsafe fn GetMathFunctionType(&self, bstr: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMathFunctionType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InsertImage<P5>(&self, width: i32, height: i32, ascent: i32, r#type: i32, bstralttext: &windows_core::BSTR, pstream: P5) -> windows_core::Result<()>
    where
        P5: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertImage)(windows_core::Interface::as_raw(self), width, height, ascent, r#type, core::mem::transmute_copy(bstralttext), pstream.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextRange2_Vtbl {
    pub base__: ITextSelection_Vtbl,
    pub GetCch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetCells: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetDuplicate2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFont2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFont2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFormattedText2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFormattedText2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGravity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetGravity: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPara2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPara2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStartPara: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddSubrange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub BuildUpMath: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DeleteSubrange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub Find: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub GetChar2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, i32) -> windows_core::HRESULT,
    pub GetDropCap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetInlineObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetRect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetSubrange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetText2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HexToUnicode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertTable: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub Linearize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetActiveSubrange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetDropCap: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetText2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnicodeToHex: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInlineObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub GetMathFunctionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub InsertImage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextRange2_Impl: ITextSelection_Impl {
    fn GetCch(&self) -> windows_core::Result<i32>;
    fn GetCells(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetColumn(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetDuplicate2(&self) -> windows_core::Result<ITextRange2>;
    fn GetFont2(&self) -> windows_core::Result<ITextFont2>;
    fn SetFont2(&self, pfont: windows_core::Ref<'_, ITextFont2>) -> windows_core::Result<()>;
    fn GetFormattedText2(&self) -> windows_core::Result<ITextRange2>;
    fn SetFormattedText2(&self, prange: windows_core::Ref<'_, ITextRange2>) -> windows_core::Result<()>;
    fn GetGravity(&self) -> windows_core::Result<i32>;
    fn SetGravity(&self, value: i32) -> windows_core::Result<()>;
    fn GetPara2(&self) -> windows_core::Result<ITextPara2>;
    fn SetPara2(&self, ppara: windows_core::Ref<'_, ITextPara2>) -> windows_core::Result<()>;
    fn GetRow(&self) -> windows_core::Result<ITextRow>;
    fn GetStartPara(&self) -> windows_core::Result<i32>;
    fn GetTable(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetURL(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddSubrange(&self, cp1: i32, cp2: i32, activate: i32) -> windows_core::Result<()>;
    fn BuildUpMath(&self, flags: i32) -> windows_core::Result<()>;
    fn DeleteSubrange(&self, cpfirst: i32, cplim: i32) -> windows_core::Result<()>;
    fn Find(&self, prange: windows_core::Ref<'_, ITextRange2>, count: i32, flags: i32) -> windows_core::Result<i32>;
    fn GetChar2(&self, pchar: *mut i32, offset: i32) -> windows_core::Result<()>;
    fn GetDropCap(&self, pcline: *mut i32, pposition: *mut i32) -> windows_core::Result<()>;
    fn GetInlineObject(&self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn GetRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> windows_core::Result<()>;
    fn GetSubrange(&self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> windows_core::Result<()>;
    fn GetText2(&self, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn HexToUnicode(&self) -> windows_core::Result<()>;
    fn InsertTable(&self, ccol: i32, crow: i32, autofit: i32) -> windows_core::Result<()>;
    fn Linearize(&self, flags: i32) -> windows_core::Result<()>;
    fn SetActiveSubrange(&self, cpanchor: i32, cpactive: i32) -> windows_core::Result<()>;
    fn SetDropCap(&self, cline: i32, position: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
    fn SetText2(&self, flags: i32, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UnicodeToHex(&self) -> windows_core::Result<()>;
    fn SetInlineObject(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> windows_core::Result<()>;
    fn GetMathFunctionType(&self, bstr: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn InsertImage(&self, width: i32, height: i32, ascent: i32, r#type: i32, bstralttext: &windows_core::BSTR, pstream: windows_core::Ref<'_, super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextRange2_Vtbl {
    pub const fn new<Identity: ITextRange2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCch<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcch: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetCch(this) {
                    Ok(ok__) => {
                        pcch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCells<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcells: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetCells(this) {
                    Ok(ok__) => {
                        ppcells.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColumn<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolumn: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetColumn(this) {
                    Ok(ok__) => {
                        ppcolumn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDuplicate2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetDuplicate2(this) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFont2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetFont2(this) {
                    Ok(ok__) => {
                        ppfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFont2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetFont2(this, core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn GetFormattedText2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetFormattedText2(this) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormattedText2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetFormattedText2(this, core::mem::transmute_copy(&prange)).into()
            }
        }
        unsafe extern "system" fn GetGravity<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetGravity(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGravity<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetGravity(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetPara2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetPara2(this) {
                    Ok(ok__) => {
                        pppara.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPara2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetPara2(this, core::mem::transmute_copy(&ppara)).into()
            }
        }
        unsafe extern "system" fn GetRow<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprow: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetRow(this) {
                    Ok(ok__) => {
                        pprow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStartPara<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetStartPara(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTable<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetTable(this) {
                    Ok(ok__) => {
                        pptable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetURL<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetURL(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetURL<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetURL(this, core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn AddSubrange<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cp1: i32, cp2: i32, activate: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::AddSubrange(this, core::mem::transmute_copy(&cp1), core::mem::transmute_copy(&cp2), core::mem::transmute_copy(&activate)).into()
            }
        }
        unsafe extern "system" fn BuildUpMath<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::BuildUpMath(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn DeleteSubrange<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpfirst: i32, cplim: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::DeleteSubrange(this, core::mem::transmute_copy(&cpfirst), core::mem::transmute_copy(&cplim)).into()
            }
        }
        unsafe extern "system" fn Find<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, count: i32, flags: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::Find(this, core::mem::transmute_copy(&prange), core::mem::transmute_copy(&count), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChar2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchar: *mut i32, offset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::GetChar2(this, core::mem::transmute_copy(&pchar), core::mem::transmute_copy(&offset)).into()
            }
        }
        unsafe extern "system" fn GetDropCap<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcline: *mut i32, pposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::GetDropCap(this, core::mem::transmute_copy(&pcline), core::mem::transmute_copy(&pposition)).into()
            }
        }
        unsafe extern "system" fn GetInlineObject<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::GetInlineObject(this, core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&palign), core::mem::transmute_copy(&pchar), core::mem::transmute_copy(&pchar1), core::mem::transmute_copy(&pchar2), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&ptexstyle), core::mem::transmute_copy(&pccol), core::mem::transmute_copy(&plevel)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRect<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::GetRect(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pright), core::mem::transmute_copy(&pbottom), core::mem::transmute_copy(&phit)).into()
            }
        }
        unsafe extern "system" fn GetSubrange<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::GetSubrange(this, core::mem::transmute_copy(&isubrange), core::mem::transmute_copy(&pcpfirst), core::mem::transmute_copy(&pcplim)).into()
            }
        }
        unsafe extern "system" fn GetText2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetText2(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HexToUnicode<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::HexToUnicode(this).into()
            }
        }
        unsafe extern "system" fn InsertTable<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccol: i32, crow: i32, autofit: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::InsertTable(this, core::mem::transmute_copy(&ccol), core::mem::transmute_copy(&crow), core::mem::transmute_copy(&autofit)).into()
            }
        }
        unsafe extern "system" fn Linearize<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::Linearize(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn SetActiveSubrange<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpanchor: i32, cpactive: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetActiveSubrange(this, core::mem::transmute_copy(&cpanchor), core::mem::transmute_copy(&cpactive)).into()
            }
        }
        unsafe extern "system" fn SetDropCap<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cline: i32, position: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetDropCap(this, core::mem::transmute_copy(&cline), core::mem::transmute_copy(&position)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetText2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetText2(this, core::mem::transmute_copy(&flags), core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn UnicodeToHex<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::UnicodeToHex(this).into()
            }
        }
        unsafe extern "system" fn SetInlineObject<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetInlineObject(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&align), core::mem::transmute_copy(&char), core::mem::transmute_copy(&char1), core::mem::transmute_copy(&char2), core::mem::transmute_copy(&count), core::mem::transmute_copy(&texstyle), core::mem::transmute_copy(&ccol)).into()
            }
        }
        unsafe extern "system" fn GetMathFunctionType<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetMathFunctionType(this, core::mem::transmute(&bstr)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertImage<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: i32, height: i32, ascent: i32, r#type: i32, bstralttext: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::InsertImage(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&ascent), core::mem::transmute_copy(&r#type), core::mem::transmute(&bstralttext), core::mem::transmute_copy(&pstream)).into()
            }
        }
        Self {
            base__: ITextSelection_Vtbl::new::<Identity, OFFSET>(),
            GetCch: GetCch::<Identity, OFFSET>,
            GetCells: GetCells::<Identity, OFFSET>,
            GetColumn: GetColumn::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetDuplicate2: GetDuplicate2::<Identity, OFFSET>,
            GetFont2: GetFont2::<Identity, OFFSET>,
            SetFont2: SetFont2::<Identity, OFFSET>,
            GetFormattedText2: GetFormattedText2::<Identity, OFFSET>,
            SetFormattedText2: SetFormattedText2::<Identity, OFFSET>,
            GetGravity: GetGravity::<Identity, OFFSET>,
            SetGravity: SetGravity::<Identity, OFFSET>,
            GetPara2: GetPara2::<Identity, OFFSET>,
            SetPara2: SetPara2::<Identity, OFFSET>,
            GetRow: GetRow::<Identity, OFFSET>,
            GetStartPara: GetStartPara::<Identity, OFFSET>,
            GetTable: GetTable::<Identity, OFFSET>,
            GetURL: GetURL::<Identity, OFFSET>,
            SetURL: SetURL::<Identity, OFFSET>,
            AddSubrange: AddSubrange::<Identity, OFFSET>,
            BuildUpMath: BuildUpMath::<Identity, OFFSET>,
            DeleteSubrange: DeleteSubrange::<Identity, OFFSET>,
            Find: Find::<Identity, OFFSET>,
            GetChar2: GetChar2::<Identity, OFFSET>,
            GetDropCap: GetDropCap::<Identity, OFFSET>,
            GetInlineObject: GetInlineObject::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetRect: GetRect::<Identity, OFFSET>,
            GetSubrange: GetSubrange::<Identity, OFFSET>,
            GetText2: GetText2::<Identity, OFFSET>,
            HexToUnicode: HexToUnicode::<Identity, OFFSET>,
            InsertTable: InsertTable::<Identity, OFFSET>,
            Linearize: Linearize::<Identity, OFFSET>,
            SetActiveSubrange: SetActiveSubrange::<Identity, OFFSET>,
            SetDropCap: SetDropCap::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            SetText2: SetText2::<Identity, OFFSET>,
            UnicodeToHex: UnicodeToHex::<Identity, OFFSET>,
            SetInlineObject: SetInlineObject::<Identity, OFFSET>,
            GetMathFunctionType: GetMathFunctionType::<Identity, OFFSET>,
            InsertImage: InsertImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextRange2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextRange as windows_core::Interface>::IID || iid == &<ITextSelection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextRange2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextRow, ITextRow_Vtbl, 0xc241f5ef_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextRow {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextRow, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITextRow {
    pub unsafe fn GetAlignment(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAlignment(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAlignment)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCellCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellCount(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCellCount)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCellCountCache(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellCountCache)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellCountCache(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCellCountCache)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCellIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCellIndex)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCellMargin(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellMargin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellMargin(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCellMargin)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHeight(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHeight)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetIndent(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIndent(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIndent)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetKeepTogether(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeepTogether)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKeepTogether(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKeepTogether)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetKeepWithNext(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeepWithNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKeepWithNext)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetNestLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNestLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRTL(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRTL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRTL(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRTL)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCellAlignment(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellAlignment(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCellAlignment)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCellColorBack(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellColorBack)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellColorBack(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCellColorBack)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCellColorFore(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellColorFore)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellColorFore(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCellColorFore)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCellMergeFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellMergeFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellMergeFlags(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCellMergeFlags)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCellShading(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellShading)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellShading(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCellShading)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCellVerticalText(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellVerticalText)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellVerticalText(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCellVerticalText)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCellWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellWidth(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCellWidth)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetCellBorderColors(&self, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCellBorderColors)(windows_core::Interface::as_raw(self), pcrleft as _, pcrtop as _, pcrright as _, pcrbottom as _).ok() }
    }
    pub unsafe fn GetCellBorderWidths(&self, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCellBorderWidths)(windows_core::Interface::as_raw(self), pduleft as _, pdutop as _, pduright as _, pdubottom as _).ok() }
    }
    pub unsafe fn SetCellBorderColors(&self, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCellBorderColors)(windows_core::Interface::as_raw(self), crleft, crtop, crright, crbottom).ok() }
    }
    pub unsafe fn SetCellBorderWidths(&self, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCellBorderWidths)(windows_core::Interface::as_raw(self), duleft, dutop, duright, dubottom).ok() }
    }
    pub unsafe fn Apply(&self, crow: i32, flags: tomConstants) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self), crow, flags).ok() }
    }
    pub unsafe fn CanChange(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Insert(&self, crow: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), crow).ok() }
    }
    pub unsafe fn IsEqual<P0>(&self, prow: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ITextRow>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), prow.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), r#type, value).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextRow_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub GetAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellCountCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellCountCache: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellMargin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellMargin: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetIndent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIndent: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetKeepTogether: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetKeepTogether: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetKeepWithNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetKeepWithNext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetNestLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetRTL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRTL: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellColorBack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellColorBack: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellColorFore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellColorFore: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellMergeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellMergeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellShading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellShading: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellVerticalText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellVerticalText: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellBorderColors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetCellBorderWidths: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetCellBorderColors: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub SetCellBorderWidths: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void, i32, tomConstants) -> windows_core::HRESULT,
    pub CanChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextRow_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn GetAlignment(&self) -> windows_core::Result<i32>;
    fn SetAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellCount(&self) -> windows_core::Result<i32>;
    fn SetCellCount(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellCountCache(&self) -> windows_core::Result<i32>;
    fn SetCellCountCache(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellIndex(&self) -> windows_core::Result<i32>;
    fn SetCellIndex(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellMargin(&self) -> windows_core::Result<i32>;
    fn SetCellMargin(&self, value: i32) -> windows_core::Result<()>;
    fn GetHeight(&self) -> windows_core::Result<i32>;
    fn SetHeight(&self, value: i32) -> windows_core::Result<()>;
    fn GetIndent(&self) -> windows_core::Result<i32>;
    fn SetIndent(&self, value: i32) -> windows_core::Result<()>;
    fn GetKeepTogether(&self) -> windows_core::Result<i32>;
    fn SetKeepTogether(&self, value: i32) -> windows_core::Result<()>;
    fn GetKeepWithNext(&self) -> windows_core::Result<i32>;
    fn SetKeepWithNext(&self, value: i32) -> windows_core::Result<()>;
    fn GetNestLevel(&self) -> windows_core::Result<i32>;
    fn GetRTL(&self) -> windows_core::Result<i32>;
    fn SetRTL(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellAlignment(&self) -> windows_core::Result<i32>;
    fn SetCellAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellColorBack(&self) -> windows_core::Result<i32>;
    fn SetCellColorBack(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellColorFore(&self) -> windows_core::Result<i32>;
    fn SetCellColorFore(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellMergeFlags(&self) -> windows_core::Result<i32>;
    fn SetCellMergeFlags(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellShading(&self) -> windows_core::Result<i32>;
    fn SetCellShading(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellVerticalText(&self) -> windows_core::Result<i32>;
    fn SetCellVerticalText(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellWidth(&self) -> windows_core::Result<i32>;
    fn SetCellWidth(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellBorderColors(&self, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> windows_core::Result<()>;
    fn GetCellBorderWidths(&self, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> windows_core::Result<()>;
    fn SetCellBorderColors(&self, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> windows_core::Result<()>;
    fn SetCellBorderWidths(&self, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> windows_core::Result<()>;
    fn Apply(&self, crow: i32, flags: tomConstants) -> windows_core::Result<()>;
    fn CanChange(&self) -> windows_core::Result<i32>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn Insert(&self, crow: i32) -> windows_core::Result<()>;
    fn IsEqual(&self, prow: windows_core::Ref<'_, ITextRow>) -> windows_core::Result<i32>;
    fn Reset(&self, value: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextRow_Vtbl {
    pub const fn new<Identity: ITextRow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAlignment<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetAlignment(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetAlignment(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellCount<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellCount(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellCount<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellCount(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellCountCache<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellCountCache(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellCountCache<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellCountCache(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellIndex<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellIndex(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellIndex<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellIndex(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellMargin<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellMargin(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellMargin<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellMargin(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetHeight<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetHeight(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHeight<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetHeight(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetIndent<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetIndent(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIndent<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetIndent(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetKeepTogether<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetKeepTogether(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeepTogether<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetKeepTogether(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetKeepWithNext<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetKeepWithNext(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetKeepWithNext(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetNestLevel<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetNestLevel(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRTL<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetRTL(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRTL<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetRTL(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellAlignment<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellAlignment(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellAlignment<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellAlignment(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellColorBack<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellColorBack(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellColorBack<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellColorBack(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellColorFore<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellColorFore(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellColorFore<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellColorFore(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellMergeFlags<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellMergeFlags(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellMergeFlags<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellMergeFlags(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellShading<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellShading(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellShading<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellShading(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellVerticalText<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellVerticalText(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellVerticalText<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellVerticalText(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellWidth<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellWidth(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellWidth<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellWidth(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellBorderColors<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::GetCellBorderColors(this, core::mem::transmute_copy(&pcrleft), core::mem::transmute_copy(&pcrtop), core::mem::transmute_copy(&pcrright), core::mem::transmute_copy(&pcrbottom)).into()
            }
        }
        unsafe extern "system" fn GetCellBorderWidths<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::GetCellBorderWidths(this, core::mem::transmute_copy(&pduleft), core::mem::transmute_copy(&pdutop), core::mem::transmute_copy(&pduright), core::mem::transmute_copy(&pdubottom)).into()
            }
        }
        unsafe extern "system" fn SetCellBorderColors<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellBorderColors(this, core::mem::transmute_copy(&crleft), core::mem::transmute_copy(&crtop), core::mem::transmute_copy(&crright), core::mem::transmute_copy(&crbottom)).into()
            }
        }
        unsafe extern "system" fn SetCellBorderWidths<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellBorderWidths(this, core::mem::transmute_copy(&duleft), core::mem::transmute_copy(&dutop), core::mem::transmute_copy(&duright), core::mem::transmute_copy(&dubottom)).into()
            }
        }
        unsafe extern "system" fn Apply<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crow: i32, flags: tomConstants) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::Apply(this, core::mem::transmute_copy(&crow), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn CanChange<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::CanChange(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Insert<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crow: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::Insert(this, core::mem::transmute_copy(&crow)).into()
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prow: *mut core::ffi::c_void, pb: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::IsEqual(this, core::mem::transmute_copy(&prow)) {
                    Ok(ok__) => {
                        pb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::Reset(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetAlignment: GetAlignment::<Identity, OFFSET>,
            SetAlignment: SetAlignment::<Identity, OFFSET>,
            GetCellCount: GetCellCount::<Identity, OFFSET>,
            SetCellCount: SetCellCount::<Identity, OFFSET>,
            GetCellCountCache: GetCellCountCache::<Identity, OFFSET>,
            SetCellCountCache: SetCellCountCache::<Identity, OFFSET>,
            GetCellIndex: GetCellIndex::<Identity, OFFSET>,
            SetCellIndex: SetCellIndex::<Identity, OFFSET>,
            GetCellMargin: GetCellMargin::<Identity, OFFSET>,
            SetCellMargin: SetCellMargin::<Identity, OFFSET>,
            GetHeight: GetHeight::<Identity, OFFSET>,
            SetHeight: SetHeight::<Identity, OFFSET>,
            GetIndent: GetIndent::<Identity, OFFSET>,
            SetIndent: SetIndent::<Identity, OFFSET>,
            GetKeepTogether: GetKeepTogether::<Identity, OFFSET>,
            SetKeepTogether: SetKeepTogether::<Identity, OFFSET>,
            GetKeepWithNext: GetKeepWithNext::<Identity, OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Identity, OFFSET>,
            GetNestLevel: GetNestLevel::<Identity, OFFSET>,
            GetRTL: GetRTL::<Identity, OFFSET>,
            SetRTL: SetRTL::<Identity, OFFSET>,
            GetCellAlignment: GetCellAlignment::<Identity, OFFSET>,
            SetCellAlignment: SetCellAlignment::<Identity, OFFSET>,
            GetCellColorBack: GetCellColorBack::<Identity, OFFSET>,
            SetCellColorBack: SetCellColorBack::<Identity, OFFSET>,
            GetCellColorFore: GetCellColorFore::<Identity, OFFSET>,
            SetCellColorFore: SetCellColorFore::<Identity, OFFSET>,
            GetCellMergeFlags: GetCellMergeFlags::<Identity, OFFSET>,
            SetCellMergeFlags: SetCellMergeFlags::<Identity, OFFSET>,
            GetCellShading: GetCellShading::<Identity, OFFSET>,
            SetCellShading: SetCellShading::<Identity, OFFSET>,
            GetCellVerticalText: GetCellVerticalText::<Identity, OFFSET>,
            SetCellVerticalText: SetCellVerticalText::<Identity, OFFSET>,
            GetCellWidth: GetCellWidth::<Identity, OFFSET>,
            SetCellWidth: SetCellWidth::<Identity, OFFSET>,
            GetCellBorderColors: GetCellBorderColors::<Identity, OFFSET>,
            GetCellBorderWidths: GetCellBorderWidths::<Identity, OFFSET>,
            SetCellBorderColors: SetCellBorderColors::<Identity, OFFSET>,
            SetCellBorderWidths: SetCellBorderWidths::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
            CanChange: CanChange::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextRow as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextRow {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextSelection, ITextSelection_Vtbl, 0x8cc497c1_a1df_11ce_8098_00aa0047be5d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextSelection {
    type Target = ITextRange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextSelection, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextRange);
#[cfg(feature = "Win32_System_Com")]
impl ITextSelection {
    pub unsafe fn GetFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFlags(&self, flags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), flags).ok() }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveLeft)(windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveRight)(windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveUp)(windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveDown)(windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HomeKey(&self, unit: tomConstants, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HomeKey)(windows_core::Interface::as_raw(self), unit, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndKey(&self, unit: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndKey)(windows_core::Interface::as_raw(self), unit, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TypeText(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TypeText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelection_Vtbl {
    pub base__: ITextRange_Vtbl,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MoveLeft: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveRight: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveUp: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveDown: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub HomeKey: unsafe extern "system" fn(*mut core::ffi::c_void, tomConstants, i32, *mut i32) -> windows_core::HRESULT,
    pub EndKey: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub TypeText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextSelection_Impl: ITextRange_Impl {
    fn GetFlags(&self) -> windows_core::Result<i32>;
    fn SetFlags(&self, flags: i32) -> windows_core::Result<()>;
    fn GetType(&self) -> windows_core::Result<i32>;
    fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32>;
    fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32>;
    fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32>;
    fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32>;
    fn HomeKey(&self, unit: tomConstants, extend: i32) -> windows_core::Result<i32>;
    fn EndKey(&self, unit: i32, extend: i32) -> windows_core::Result<i32>;
    fn TypeText(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextSelection_Vtbl {
    pub const fn new<Identity: ITextSelection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFlags<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        pflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFlags<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextSelection_Impl::SetFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::GetType(this) {
                    Ok(ok__) => {
                        ptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveLeft<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::MoveLeft(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveRight<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::MoveRight(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveUp<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::MoveUp(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveDown<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::MoveDown(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HomeKey<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: tomConstants, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::HomeKey(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndKey<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::EndKey(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TypeText<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextSelection_Impl::TypeText(this, core::mem::transmute(&bstr)).into()
            }
        }
        Self {
            base__: ITextRange_Vtbl::new::<Identity, OFFSET>(),
            GetFlags: GetFlags::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            MoveLeft: MoveLeft::<Identity, OFFSET>,
            MoveRight: MoveRight::<Identity, OFFSET>,
            MoveUp: MoveUp::<Identity, OFFSET>,
            MoveDown: MoveDown::<Identity, OFFSET>,
            HomeKey: HomeKey::<Identity, OFFSET>,
            EndKey: EndKey::<Identity, OFFSET>,
            TypeText: TypeText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextSelection as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextRange as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextSelection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextSelection2, ITextSelection2_Vtbl, 0xc241f5e1_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextSelection2 {
    type Target = ITextRange2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextSelection2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextRange, ITextSelection, ITextRange2);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelection2_Vtbl {
    pub base__: ITextRange2_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextSelection2_Impl: ITextRange2_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextSelection2_Vtbl {
    pub const fn new<Identity: ITextSelection2_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ITextRange2_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextSelection2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextRange as windows_core::Interface>::IID || iid == &<ITextSelection as windows_core::Interface>::IID || iid == &<ITextRange2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextSelection2 {}
windows_core::imp::define_interface!(ITextServices, ITextServices_Vtbl, 0);
windows_core::imp::interface_hierarchy!(ITextServices, windows_core::IUnknown);
impl ITextServices {
    pub unsafe fn TxSendMessage(&self, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxSendMessage)(windows_core::Interface::as_raw(self), msg, wparam, lparam, plresult as _).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxDraw(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxDraw)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect as _, ptd as _, hdcdraw, hictargetdev, lprcbounds as _, lprcwbounds as _, lprcupdate as _, pfncontinue, dwcontinue, lviewid).ok() }
    }
    pub unsafe fn TxGetHScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetHScroll)(windows_core::Interface::as_raw(self), plmin as _, plmax as _, plpos as _, plpage as _, pfenabled as _).ok() }
    }
    pub unsafe fn TxGetVScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetVScroll)(windows_core::Interface::as_raw(self), plmin as _, plmax as _, plpos as _, plpage as _, pfenabled as _).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn OnTxSetCursor(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTxSetCursor)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect as _, ptd as _, hdcdraw, hictargetdev, lprcclient as _, x, y).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxQueryHitPoint(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxQueryHitPoint)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect as _, ptd as _, hdcdraw, hictargetdev, lprcclient as _, x, y, phitresult as _).ok() }
    }
    pub unsafe fn OnTxInPlaceActivate(&self, prcclient: *mut super::super::super::Foundation::RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTxInPlaceActivate)(windows_core::Interface::as_raw(self), prcclient as _).ok() }
    }
    pub unsafe fn OnTxInPlaceDeactivate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTxInPlaceDeactivate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnTxUIActivate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTxUIActivate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnTxUIDeactivate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTxUIDeactivate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn TxGetText(&self, pbstrtext: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetText)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrtext)).ok() }
    }
    pub unsafe fn TxSetText<P0>(&self, psztext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).TxSetText)(windows_core::Interface::as_raw(self), psztext.param().abi()).ok() }
    }
    pub unsafe fn TxGetCurTargetX(&self, param0: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetCurTargetX)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn TxGetBaseLinePos(&self, param0: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetBaseLinePos)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxGetNaturalSize(&self, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetNaturalSize)(windows_core::Interface::as_raw(self), dwaspect, hdcdraw, hictargetdev, ptd as _, dwmode, psizelextent, pwidth as _, pheight as _).ok() }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn TxGetDropTarget(&self) -> windows_core::Result<super::super::super::System::Ole::IDropTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetDropTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OnTxPropertyBitsChange(&self, dwmask: u32, dwbits: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTxPropertyBitsChange)(windows_core::Interface::as_raw(self), dwmask, dwbits).ok() }
    }
    pub unsafe fn TxGetCachedSize(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetCachedSize)(windows_core::Interface::as_raw(self), pdwwidth as _, pdwheight as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TxSendMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM, *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub TxDraw: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::System::Com::DVASPECT, i32, *mut core::ffi::c_void, *mut super::super::super::System::Com::DVTARGETDEVICE, super::super::super::Graphics::Gdi::HDC, super::super::super::Graphics::Gdi::HDC, *mut super::super::super::Foundation::RECTL, *mut super::super::super::Foundation::RECTL, *mut super::super::super::Foundation::RECT, isize, u32, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    TxDraw: usize,
    pub TxGetHScroll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub TxGetVScroll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub OnTxSetCursor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::System::Com::DVASPECT, i32, *mut core::ffi::c_void, *mut super::super::super::System::Com::DVTARGETDEVICE, super::super::super::Graphics::Gdi::HDC, super::super::super::Graphics::Gdi::HDC, *mut super::super::super::Foundation::RECT, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    OnTxSetCursor: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub TxQueryHitPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::System::Com::DVASPECT, i32, *mut core::ffi::c_void, *mut super::super::super::System::Com::DVTARGETDEVICE, super::super::super::Graphics::Gdi::HDC, super::super::super::Graphics::Gdi::HDC, *mut super::super::super::Foundation::RECT, i32, i32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    TxQueryHitPoint: usize,
    pub OnTxInPlaceActivate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub OnTxInPlaceDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnTxUIActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnTxUIDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TxGetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TxSetText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub TxGetCurTargetX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TxGetBaseLinePos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub TxGetNaturalSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::super::Graphics::Gdi::HDC, super::super::super::Graphics::Gdi::HDC, *mut super::super::super::System::Com::DVTARGETDEVICE, u32, *const super::super::super::Foundation::SIZE, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    TxGetNaturalSize: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub TxGetDropTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    TxGetDropTarget: usize,
    pub OnTxPropertyBitsChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub TxGetCachedSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextServices_Impl: windows_core::IUnknownImpl {
    fn TxSendMessage(&self, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>;
    fn TxDraw(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> windows_core::Result<()>;
    fn TxGetHScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn TxGetVScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn OnTxSetCursor(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> windows_core::Result<()>;
    fn TxQueryHitPoint(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> windows_core::Result<()>;
    fn OnTxInPlaceActivate(&self, prcclient: *mut super::super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn OnTxInPlaceDeactivate(&self) -> windows_core::Result<()>;
    fn OnTxUIActivate(&self) -> windows_core::Result<()>;
    fn OnTxUIDeactivate(&self) -> windows_core::Result<()>;
    fn TxGetText(&self, pbstrtext: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn TxSetText(&self, psztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn TxGetCurTargetX(&self, param0: *mut i32) -> windows_core::Result<()>;
    fn TxGetBaseLinePos(&self, param0: *mut i32) -> windows_core::Result<()>;
    fn TxGetNaturalSize(&self, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn TxGetDropTarget(&self) -> windows_core::Result<super::super::super::System::Ole::IDropTarget>;
    fn OnTxPropertyBitsChange(&self, dwmask: u32, dwbits: u32) -> windows_core::Result<()>;
    fn TxGetCachedSize(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextServices_Vtbl {
    pub const fn new<Identity: ITextServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TxSendMessage<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxSendMessage(this, core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam), core::mem::transmute_copy(&plresult)).into()
            }
        }
        unsafe extern "system" fn TxDraw<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxDraw(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&lprcbounds), core::mem::transmute_copy(&lprcwbounds), core::mem::transmute_copy(&lprcupdate), core::mem::transmute_copy(&pfncontinue), core::mem::transmute_copy(&dwcontinue), core::mem::transmute_copy(&lviewid)).into()
            }
        }
        unsafe extern "system" fn TxGetHScroll<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxGetHScroll(this, core::mem::transmute_copy(&plmin), core::mem::transmute_copy(&plmax), core::mem::transmute_copy(&plpos), core::mem::transmute_copy(&plpage), core::mem::transmute_copy(&pfenabled)).into()
            }
        }
        unsafe extern "system" fn TxGetVScroll<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxGetVScroll(this, core::mem::transmute_copy(&plmin), core::mem::transmute_copy(&plmax), core::mem::transmute_copy(&plpos), core::mem::transmute_copy(&plpage), core::mem::transmute_copy(&pfenabled)).into()
            }
        }
        unsafe extern "system" fn OnTxSetCursor<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::OnTxSetCursor(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&lprcclient), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn TxQueryHitPoint<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxQueryHitPoint(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&lprcclient), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&phitresult)).into()
            }
        }
        unsafe extern "system" fn OnTxInPlaceActivate<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcclient: *mut super::super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::OnTxInPlaceActivate(this, core::mem::transmute_copy(&prcclient)).into()
            }
        }
        unsafe extern "system" fn OnTxInPlaceDeactivate<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::OnTxInPlaceDeactivate(this).into()
            }
        }
        unsafe extern "system" fn OnTxUIActivate<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::OnTxUIActivate(this).into()
            }
        }
        unsafe extern "system" fn OnTxUIDeactivate<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::OnTxUIDeactivate(this).into()
            }
        }
        unsafe extern "system" fn TxGetText<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxGetText(this, core::mem::transmute_copy(&pbstrtext)).into()
            }
        }
        unsafe extern "system" fn TxSetText<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxSetText(this, core::mem::transmute(&psztext)).into()
            }
        }
        unsafe extern "system" fn TxGetCurTargetX<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxGetCurTargetX(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn TxGetBaseLinePos<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxGetBaseLinePos(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn TxGetNaturalSize<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxGetNaturalSize(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&psizelextent), core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn TxGetDropTarget<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdroptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextServices_Impl::TxGetDropTarget(this) {
                    Ok(ok__) => {
                        ppdroptarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnTxPropertyBitsChange<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmask: u32, dwbits: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::OnTxPropertyBitsChange(this, core::mem::transmute_copy(&dwmask), core::mem::transmute_copy(&dwbits)).into()
            }
        }
        unsafe extern "system" fn TxGetCachedSize<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxGetCachedSize(this, core::mem::transmute_copy(&pdwwidth), core::mem::transmute_copy(&pdwheight)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TxSendMessage: TxSendMessage::<Identity, OFFSET>,
            TxDraw: TxDraw::<Identity, OFFSET>,
            TxGetHScroll: TxGetHScroll::<Identity, OFFSET>,
            TxGetVScroll: TxGetVScroll::<Identity, OFFSET>,
            OnTxSetCursor: OnTxSetCursor::<Identity, OFFSET>,
            TxQueryHitPoint: TxQueryHitPoint::<Identity, OFFSET>,
            OnTxInPlaceActivate: OnTxInPlaceActivate::<Identity, OFFSET>,
            OnTxInPlaceDeactivate: OnTxInPlaceDeactivate::<Identity, OFFSET>,
            OnTxUIActivate: OnTxUIActivate::<Identity, OFFSET>,
            OnTxUIDeactivate: OnTxUIDeactivate::<Identity, OFFSET>,
            TxGetText: TxGetText::<Identity, OFFSET>,
            TxSetText: TxSetText::<Identity, OFFSET>,
            TxGetCurTargetX: TxGetCurTargetX::<Identity, OFFSET>,
            TxGetBaseLinePos: TxGetBaseLinePos::<Identity, OFFSET>,
            TxGetNaturalSize: TxGetNaturalSize::<Identity, OFFSET>,
            TxGetDropTarget: TxGetDropTarget::<Identity, OFFSET>,
            OnTxPropertyBitsChange: OnTxPropertyBitsChange::<Identity, OFFSET>,
            TxGetCachedSize: TxGetCachedSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextServices as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for ITextServices {}
windows_core::imp::define_interface!(ITextServices2, ITextServices2_Vtbl, 0);
impl core::ops::Deref for ITextServices2 {
    type Target = ITextServices;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextServices2, windows_core::IUnknown, ITextServices);
impl ITextServices2 {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxGetNaturalSize2(&self, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TxGetNaturalSize2)(windows_core::Interface::as_raw(self), dwaspect, hdcdraw, hictargetdev, ptd as _, dwmode, psizelextent, pwidth as _, pheight as _, pascent as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn TxDrawD2D<P0>(&self, prendertarget: P0, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct2D::ID2D1RenderTarget>,
    {
        unsafe { (windows_core::Interface::vtable(self).TxDrawD2D)(windows_core::Interface::as_raw(self), prendertarget.param().abi(), lprcbounds as _, lprcupdate as _, lviewid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextServices2_Vtbl {
    pub base__: ITextServices_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub TxGetNaturalSize2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::super::Graphics::Gdi::HDC, super::super::super::Graphics::Gdi::HDC, *mut super::super::super::System::Com::DVTARGETDEVICE, u32, *const super::super::super::Foundation::SIZE, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    TxGetNaturalSize2: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub TxDrawD2D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::RECTL, *mut super::super::super::Foundation::RECT, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    TxDrawD2D: usize,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextServices2_Impl: ITextServices_Impl {
    fn TxGetNaturalSize2(&self, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> windows_core::Result<()>;
    fn TxDrawD2D(&self, prendertarget: windows_core::Ref<'_, super::super::super::Graphics::Direct2D::ID2D1RenderTarget>, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextServices2_Vtbl {
    pub const fn new<Identity: ITextServices2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TxGetNaturalSize2<Identity: ITextServices2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices2_Impl::TxGetNaturalSize2(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&psizelextent), core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight), core::mem::transmute_copy(&pascent)).into()
            }
        }
        unsafe extern "system" fn TxDrawD2D<Identity: ITextServices2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertarget: *mut core::ffi::c_void, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices2_Impl::TxDrawD2D(this, core::mem::transmute_copy(&prendertarget), core::mem::transmute_copy(&lprcbounds), core::mem::transmute_copy(&lprcupdate), core::mem::transmute_copy(&lviewid)).into()
            }
        }
        Self {
            base__: ITextServices_Vtbl::new::<Identity, OFFSET>(),
            TxGetNaturalSize2: TxGetNaturalSize2::<Identity, OFFSET>,
            TxDrawD2D: TxDrawD2D::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextServices2 as windows_core::Interface>::IID || iid == &<ITextServices as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for ITextServices2 {}
windows_core::imp::define_interface!(ITextStory, ITextStory_Vtbl, 0xc241f5f3_7206_11d8_a2c7_00a0d1d6c6b3);
windows_core::imp::interface_hierarchy!(ITextStory, windows_core::IUnknown);
impl ITextStory {
    pub unsafe fn GetActive(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetActive(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetActive)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetDisplay(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplay)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetType(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRange(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRange)(windows_core::Interface::as_raw(self), cpactive, cpanchor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetText(&self, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFormattedText<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFormattedText)(windows_core::Interface::as_raw(self), punk.param().abi()).ok() }
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), r#type, value).ok() }
    }
    pub unsafe fn SetText(&self, flags: i32, bstr: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), flags, core::mem::transmute_copy(bstr)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetActive: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRange: usize,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStory_Impl: windows_core::IUnknownImpl {
    fn GetActive(&self) -> windows_core::Result<i32>;
    fn SetActive(&self, value: i32) -> windows_core::Result<()>;
    fn GetDisplay(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetIndex(&self) -> windows_core::Result<i32>;
    fn GetType(&self) -> windows_core::Result<i32>;
    fn SetType(&self, value: i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn GetRange(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange2>;
    fn GetText(&self, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetFormattedText(&self, punk: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
    fn SetText(&self, flags: i32, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITextStory_Vtbl {
    pub const fn new<Identity: ITextStory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetActive<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetActive(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetActive<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStory_Impl::SetActive(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetDisplay<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisplay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetDisplay(this) {
                    Ok(ok__) => {
                        ppdisplay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIndex<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetIndex(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetType(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetType<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStory_Impl::SetType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRange<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetRange(this, core::mem::transmute_copy(&cpactive), core::mem::transmute_copy(&cpanchor)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetText<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetText(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStory_Impl::SetFormattedText(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStory_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetText<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStory_Impl::SetText(this, core::mem::transmute_copy(&flags), core::mem::transmute(&bstr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetActive: GetActive::<Identity, OFFSET>,
            SetActive: SetActive::<Identity, OFFSET>,
            GetDisplay: GetDisplay::<Identity, OFFSET>,
            GetIndex: GetIndex::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetRange: GetRange::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextStory {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextStoryRanges, ITextStoryRanges_Vtbl, 0x8cc497c5_a1df_11ce_8098_00aa0047be5d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextStoryRanges {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextStoryRanges, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITextStoryRanges {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ITextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoryRanges_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextStoryRanges_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<ITextRange>;
    fn GetCount(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextStoryRanges_Vtbl {
    pub const fn new<Identity: ITextStoryRanges_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: ITextStoryRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoryRanges_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunkenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ITextStoryRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoryRanges_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ITextStoryRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoryRanges_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoryRanges as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextStoryRanges {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextStoryRanges2, ITextStoryRanges2_Vtbl, 0xc241f5e5_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextStoryRanges2 {
    type Target = ITextStoryRanges;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextStoryRanges2, windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextStoryRanges);
#[cfg(feature = "Win32_System_Com")]
impl ITextStoryRanges2 {
    pub unsafe fn Item2(&self, index: i32) -> windows_core::Result<ITextRange2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item2)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoryRanges2_Vtbl {
    pub base__: ITextStoryRanges_Vtbl,
    pub Item2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextStoryRanges2_Impl: ITextStoryRanges_Impl {
    fn Item2(&self, index: i32) -> windows_core::Result<ITextRange2>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextStoryRanges2_Vtbl {
    pub const fn new<Identity: ITextStoryRanges2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item2<Identity: ITextStoryRanges2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoryRanges2_Impl::Item2(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ITextStoryRanges_Vtbl::new::<Identity, OFFSET>(), Item2: Item2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoryRanges2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITextStoryRanges as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextStoryRanges2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITextStrings, ITextStrings_Vtbl, 0xc241f5e7_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITextStrings {
    type Target = super::super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITextStrings, windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITextStrings {
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ITextRange2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)).ok() }
    }
    pub unsafe fn Append<P0>(&self, prange: P0, istring: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextRange2>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), prange.param().abi(), istring).ok() }
    }
    pub unsafe fn Cat2(&self, istring: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cat2)(windows_core::Interface::as_raw(self), istring).ok() }
    }
    pub unsafe fn CatTop2(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CatTop2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)).ok() }
    }
    pub unsafe fn DeleteRange<P0>(&self, prange: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextRange2>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteRange)(windows_core::Interface::as_raw(self), prange.param().abi()).ok() }
    }
    pub unsafe fn EncodeFunction<P8>(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: P8) -> windows_core::Result<()>
    where
        P8: windows_core::Param<ITextRange2>,
    {
        unsafe { (windows_core::Interface::vtable(self).EncodeFunction)(windows_core::Interface::as_raw(self), r#type, align, char, char1, char2, count, texstyle, ccol, prange.param().abi()).ok() }
    }
    pub unsafe fn GetCch(&self, istring: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCch)(windows_core::Interface::as_raw(self), istring, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InsertNullStr(&self, istring: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InsertNullStr)(windows_core::Interface::as_raw(self), istring).ok() }
    }
    pub unsafe fn MoveBoundary(&self, istring: i32, cch: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MoveBoundary)(windows_core::Interface::as_raw(self), istring, cch).ok() }
    }
    pub unsafe fn PrefixTop(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PrefixTop)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)).ok() }
    }
    pub unsafe fn Remove(&self, istring: i32, cstring: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), istring, cstring).ok() }
    }
    pub unsafe fn SetFormattedText<P0, P1>(&self, pranged: P0, pranges: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextRange2>,
        P1: windows_core::Param<ITextRange2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFormattedText)(windows_core::Interface::as_raw(self), pranged.param().abi(), pranges.param().abi()).ok() }
    }
    pub unsafe fn SetOpCp(&self, istring: i32, cp: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOpCp)(windows_core::Interface::as_raw(self), istring, cp).ok() }
    }
    pub unsafe fn SuffixTop<P1>(&self, bstr: &windows_core::BSTR, prange: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITextRange2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SuffixTop)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr), prange.param().abi()).ok() }
    }
    pub unsafe fn Swap(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Swap)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextStrings_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Cat2: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CatTop2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EncodeFunction: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, i32, i32, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCch: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub InsertNullStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MoveBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub PrefixTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOpCp: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SuffixTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Swap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextStrings_Impl: super::super::super::System::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> windows_core::Result<ITextRange2>;
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn Add(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Append(&self, prange: windows_core::Ref<'_, ITextRange2>, istring: i32) -> windows_core::Result<()>;
    fn Cat2(&self, istring: i32) -> windows_core::Result<()>;
    fn CatTop2(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DeleteRange(&self, prange: windows_core::Ref<'_, ITextRange2>) -> windows_core::Result<()>;
    fn EncodeFunction(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: windows_core::Ref<'_, ITextRange2>) -> windows_core::Result<()>;
    fn GetCch(&self, istring: i32) -> windows_core::Result<i32>;
    fn InsertNullStr(&self, istring: i32) -> windows_core::Result<()>;
    fn MoveBoundary(&self, istring: i32, cch: i32) -> windows_core::Result<()>;
    fn PrefixTop(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Remove(&self, istring: i32, cstring: i32) -> windows_core::Result<()>;
    fn SetFormattedText(&self, pranged: windows_core::Ref<'_, ITextRange2>, pranges: windows_core::Ref<'_, ITextRange2>) -> windows_core::Result<()>;
    fn SetOpCp(&self, istring: i32, cp: i32) -> windows_core::Result<()>;
    fn SuffixTop(&self, bstr: &windows_core::BSTR, prange: windows_core::Ref<'_, ITextRange2>) -> windows_core::Result<()>;
    fn Swap(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextStrings_Vtbl {
    pub const fn new<Identity: ITextStrings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStrings_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStrings_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::Add(this, core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, istring: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::Append(this, core::mem::transmute_copy(&prange), core::mem::transmute_copy(&istring)).into()
            }
        }
        unsafe extern "system" fn Cat2<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::Cat2(this, core::mem::transmute_copy(&istring)).into()
            }
        }
        unsafe extern "system" fn CatTop2<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::CatTop2(this, core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn DeleteRange<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::DeleteRange(this, core::mem::transmute_copy(&prange)).into()
            }
        }
        unsafe extern "system" fn EncodeFunction<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::EncodeFunction(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&align), core::mem::transmute_copy(&char), core::mem::transmute_copy(&char1), core::mem::transmute_copy(&char2), core::mem::transmute_copy(&count), core::mem::transmute_copy(&texstyle), core::mem::transmute_copy(&ccol), core::mem::transmute_copy(&prange)).into()
            }
        }
        unsafe extern "system" fn GetCch<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32, pcch: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStrings_Impl::GetCch(this, core::mem::transmute_copy(&istring)) {
                    Ok(ok__) => {
                        pcch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertNullStr<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::InsertNullStr(this, core::mem::transmute_copy(&istring)).into()
            }
        }
        unsafe extern "system" fn MoveBoundary<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32, cch: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::MoveBoundary(this, core::mem::transmute_copy(&istring), core::mem::transmute_copy(&cch)).into()
            }
        }
        unsafe extern "system" fn PrefixTop<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::PrefixTop(this, core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32, cstring: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::Remove(this, core::mem::transmute_copy(&istring), core::mem::transmute_copy(&cstring)).into()
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pranged: *mut core::ffi::c_void, pranges: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::SetFormattedText(this, core::mem::transmute_copy(&pranged), core::mem::transmute_copy(&pranges)).into()
            }
        }
        unsafe extern "system" fn SetOpCp<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32, cp: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::SetOpCp(this, core::mem::transmute_copy(&istring), core::mem::transmute_copy(&cp)).into()
            }
        }
        unsafe extern "system" fn SuffixTop<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::SuffixTop(this, core::mem::transmute(&bstr), core::mem::transmute_copy(&prange)).into()
            }
        }
        unsafe extern "system" fn Swap<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::Swap(this).into()
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Item: Item::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            Cat2: Cat2::<Identity, OFFSET>,
            CatTop2: CatTop2::<Identity, OFFSET>,
            DeleteRange: DeleteRange::<Identity, OFFSET>,
            EncodeFunction: EncodeFunction::<Identity, OFFSET>,
            GetCch: GetCch::<Identity, OFFSET>,
            InsertNullStr: InsertNullStr::<Identity, OFFSET>,
            MoveBoundary: MoveBoundary::<Identity, OFFSET>,
            PrefixTop: PrefixTop::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, OFFSET>,
            SetOpCp: SetOpCp::<Identity, OFFSET>,
            SuffixTop: SuffixTop::<Identity, OFFSET>,
            Swap: Swap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStrings as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextStrings {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KHYPH(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MANCODE(pub i32);
pub const MAX_TABLE_CELLS: u32 = 63u32;
pub const MAX_TAB_STOPS: u32 = 32u32;
pub const MBOLD: MANCODE = MANCODE(16i32);
pub const MFRAK: MANCODE = MANCODE(2i32);
pub const MGREEK: MANCODE = MANCODE(64i32);
pub const MINIT: MANCODE = MANCODE(8i32);
pub const MISOL: MANCODE = MANCODE(7i32);
pub const MITAL: MANCODE = MANCODE(32i32);
pub const MLOOP: MANCODE = MANCODE(11i32);
pub const MMATH: MANCODE = MANCODE(6i32);
pub const MMONO: MANCODE = MANCODE(5i32);
pub const MOPEN: MANCODE = MANCODE(3i32);
pub const MOPENA: MANCODE = MANCODE(12i32);
pub const MROMN: MANCODE = MANCODE(0i32);
pub const MSANS: MANCODE = MANCODE(4i32);
pub const MSCRP: MANCODE = MANCODE(1i32);
pub const MSFTEDIT_CLASS: windows_core::PCWSTR = windows_core::w!("RICHEDIT50W");
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSGFILTER {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct MSGFILTER {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
}
pub const MSTRCH: MANCODE = MANCODE(10i32);
pub const MTAIL: MANCODE = MANCODE(9i32);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OBJECTPOSITIONS {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub pcpPositions: *mut i32,
}
#[cfg(target_arch = "x86")]
impl Default for OBJECTPOSITIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct OBJECTPOSITIONS {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub pcpPositions: *mut i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OBJECTPOSITIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OBJECTTYPE(pub i32);
pub const OLEOP_DOVERB: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PARAFORMAT {
    pub cbSize: u32,
    pub dwMask: PARAFORMAT_MASK,
    pub wNumbering: PARAFORMAT_NUMBERING,
    pub Anonymous: PARAFORMAT_0,
    pub dxStartIndent: i32,
    pub dxRightIndent: i32,
    pub dxOffset: i32,
    pub wAlignment: PARAFORMAT_ALIGNMENT,
    pub cTabCount: i16,
    pub rgxTabs: [u32; 32],
}
impl Default for PARAFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PARAFORMAT_0 {
    pub wReserved: u16,
    pub wEffects: u16,
}
impl Default for PARAFORMAT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PARAFORMAT2 {
    pub Base: PARAFORMAT,
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
impl Default for PARAFORMAT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PARAFORMAT_ALIGNMENT(pub u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PARAFORMAT_BORDERS(pub u16);
impl PARAFORMAT_BORDERS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PARAFORMAT_BORDERS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PARAFORMAT_BORDERS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PARAFORMAT_BORDERS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PARAFORMAT_BORDERS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PARAFORMAT_BORDERS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PARAFORMAT_BORDERS_AUTOCOLOR: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(64u16);
pub const PARAFORMAT_BORDERS_BOTTOM: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(8u16);
pub const PARAFORMAT_BORDERS_INSIDE: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(16u16);
pub const PARAFORMAT_BORDERS_LEFT: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(1u16);
pub const PARAFORMAT_BORDERS_OUTSIDE: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(32u16);
pub const PARAFORMAT_BORDERS_RIGHT: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(2u16);
pub const PARAFORMAT_BORDERS_TOP: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(4u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PARAFORMAT_MASK(pub u32);
impl PARAFORMAT_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PARAFORMAT_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PARAFORMAT_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PARAFORMAT_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PARAFORMAT_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PARAFORMAT_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PARAFORMAT_NUMBERING(pub u16);
impl PARAFORMAT_NUMBERING {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PARAFORMAT_NUMBERING {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PARAFORMAT_NUMBERING {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PARAFORMAT_NUMBERING {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PARAFORMAT_NUMBERING {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PARAFORMAT_NUMBERING {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PARAFORMAT_NUMBERING_STYLE(pub u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PARAFORMAT_SHADING_STYLE(pub u16);
pub const PARAFORMAT_SHADING_STYLE_DARK_DOWN_DIAG: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(3u16);
pub const PARAFORMAT_SHADING_STYLE_DARK_GRID: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(5u16);
pub const PARAFORMAT_SHADING_STYLE_DARK_HORIZ: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(1u16);
pub const PARAFORMAT_SHADING_STYLE_DARK_TRELLIS: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(6u16);
pub const PARAFORMAT_SHADING_STYLE_DARK_UP_DIAG: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(4u16);
pub const PARAFORMAT_SHADING_STYLE_DARK_VERT: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(2u16);
pub const PARAFORMAT_SHADING_STYLE_LIGHT_DOWN_DIAG: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(9u16);
pub const PARAFORMAT_SHADING_STYLE_LIGHT_GRID: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(11u16);
pub const PARAFORMAT_SHADING_STYLE_LIGHT_HORZ: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(7u16);
pub const PARAFORMAT_SHADING_STYLE_LIGHT_TRELLIS: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(12u16);
pub const PARAFORMAT_SHADING_STYLE_LIGHT_UP_DIAG: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(10u16);
pub const PARAFORMAT_SHADING_STYLE_LIGHT_VERT: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(8u16);
pub const PARAFORMAT_SHADING_STYLE_NONE: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(0u16);
pub const PC_DELIMITER: u32 = 4u32;
pub const PC_FOLLOWING: u32 = 1u32;
pub const PC_LEADING: u32 = 2u32;
pub const PC_OVERFLOW: u32 = 3u32;
pub type PCreateTextServices = Option<unsafe extern "system" fn(punkouter: windows_core::Ref<'_, windows_core::IUnknown>, pitexthost: windows_core::Ref<'_, ITextHost>, ppunk: windows_core::OutRef<'_, windows_core::IUnknown>) -> windows_core::HRESULT>;
pub const PFA_CENTER: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(3u16);
pub const PFA_FULL_GLYPHS: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(8u16);
pub const PFA_FULL_INTERLETTER: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(6u16);
pub const PFA_FULL_INTERWORD: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(4u16);
pub const PFA_FULL_NEWSPAPER: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(5u16);
pub const PFA_FULL_SCALED: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(7u16);
pub const PFA_JUSTIFY: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(4u16);
pub const PFA_LEFT: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(1u16);
pub const PFA_RIGHT: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(2u16);
pub const PFM_ALIGNMENT: PARAFORMAT_MASK = PARAFORMAT_MASK(8u32);
pub const PFM_ALL: PARAFORMAT_MASK = PARAFORMAT_MASK(2147549247u32);
pub const PFM_ALL2: PARAFORMAT_MASK = PARAFORMAT_MASK(3506437631u32);
pub const PFM_BORDER: PARAFORMAT_MASK = PARAFORMAT_MASK(2048u32);
pub const PFM_BOX: PARAFORMAT_MASK = PARAFORMAT_MASK(67108864u32);
pub const PFM_COLLAPSED: PARAFORMAT_MASK = PARAFORMAT_MASK(16777216u32);
pub const PFM_DONOTHYPHEN: PARAFORMAT_MASK = PARAFORMAT_MASK(4194304u32);
pub const PFM_EFFECTS: PARAFORMAT_MASK = PARAFORMAT_MASK(1358888960u32);
pub const PFM_KEEP: PARAFORMAT_MASK = PARAFORMAT_MASK(131072u32);
pub const PFM_KEEPNEXT: PARAFORMAT_MASK = PARAFORMAT_MASK(262144u32);
pub const PFM_LINESPACING: PARAFORMAT_MASK = PARAFORMAT_MASK(256u32);
pub const PFM_NOLINENUMBER: PARAFORMAT_MASK = PARAFORMAT_MASK(1048576u32);
pub const PFM_NOWIDOWCONTROL: PARAFORMAT_MASK = PARAFORMAT_MASK(2097152u32);
pub const PFM_NUMBERING: PARAFORMAT_MASK = PARAFORMAT_MASK(32u32);
pub const PFM_NUMBERINGSTART: PARAFORMAT_MASK = PARAFORMAT_MASK(32768u32);
pub const PFM_NUMBERINGSTYLE: PARAFORMAT_MASK = PARAFORMAT_MASK(8192u32);
pub const PFM_NUMBERINGTAB: PARAFORMAT_MASK = PARAFORMAT_MASK(16384u32);
pub const PFM_OFFSET: PARAFORMAT_MASK = PARAFORMAT_MASK(4u32);
pub const PFM_OFFSETINDENT: PARAFORMAT_MASK = PARAFORMAT_MASK(2147483648u32);
pub const PFM_OUTLINELEVEL: PARAFORMAT_MASK = PARAFORMAT_MASK(33554432u32);
pub const PFM_PAGEBREAKBEFORE: PARAFORMAT_MASK = PARAFORMAT_MASK(524288u32);
pub const PFM_RESERVED2: PARAFORMAT_MASK = PARAFORMAT_MASK(134217728u32);
pub const PFM_RIGHTINDENT: PARAFORMAT_MASK = PARAFORMAT_MASK(2u32);
pub const PFM_RTLPARA: PARAFORMAT_MASK = PARAFORMAT_MASK(65536u32);
pub const PFM_SHADING: PARAFORMAT_MASK = PARAFORMAT_MASK(4096u32);
pub const PFM_SIDEBYSIDE: PARAFORMAT_MASK = PARAFORMAT_MASK(8388608u32);
pub const PFM_SPACEAFTER: PARAFORMAT_MASK = PARAFORMAT_MASK(128u32);
pub const PFM_SPACEBEFORE: PARAFORMAT_MASK = PARAFORMAT_MASK(64u32);
pub const PFM_STARTINDENT: PARAFORMAT_MASK = PARAFORMAT_MASK(1u32);
pub const PFM_STYLE: PARAFORMAT_MASK = PARAFORMAT_MASK(1024u32);
pub const PFM_TABLE: PARAFORMAT_MASK = PARAFORMAT_MASK(1073741824u32);
pub const PFM_TABLEROWDELIMITER: PARAFORMAT_MASK = PARAFORMAT_MASK(268435456u32);
pub const PFM_TABSTOPS: PARAFORMAT_MASK = PARAFORMAT_MASK(16u32);
pub const PFM_TEXTWRAPPINGBREAK: PARAFORMAT_MASK = PARAFORMAT_MASK(536870912u32);
pub const PFNS_NEWNUMBER: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(32768u16);
pub const PFNS_NONUMBER: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(1024u16);
pub const PFNS_PAREN: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(0u16);
pub const PFNS_PARENS: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(256u16);
pub const PFNS_PERIOD: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(512u16);
pub const PFNS_PLAIN: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(768u16);
pub const PFN_ARABIC: PARAFORMAT_NUMBERING = PARAFORMAT_NUMBERING(2u16);
pub const PFN_BULLET: PARAFORMAT_NUMBERING = PARAFORMAT_NUMBERING(1u16);
pub const PFN_LCLETTER: PARAFORMAT_NUMBERING = PARAFORMAT_NUMBERING(3u16);
pub const PFN_LCROMAN: PARAFORMAT_NUMBERING = PARAFORMAT_NUMBERING(5u16);
pub const PFN_UCLETTER: PARAFORMAT_NUMBERING = PARAFORMAT_NUMBERING(4u16);
pub const PFN_UCROMAN: PARAFORMAT_NUMBERING = PARAFORMAT_NUMBERING(6u16);
pub type PShutdownTextServices = Option<unsafe extern "system" fn(ptextservices: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::HRESULT>;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PUNCTUATION {
    pub iSize: u32,
    pub szPunctuation: windows_core::PSTR,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct PUNCTUATION {
    pub iSize: u32,
    pub szPunctuation: windows_core::PSTR,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct REOBJECT {
    pub cbStruct: u32,
    pub cp: i32,
    pub clsid: windows_core::GUID,
    pub poleobj: core::mem::ManuallyDrop<Option<super::super::super::System::Ole::IOleObject>>,
    pub pstg: core::mem::ManuallyDrop<Option<super::super::super::System::Com::StructuredStorage::IStorage>>,
    pub polesite: core::mem::ManuallyDrop<Option<super::super::super::System::Ole::IOleClientSite>>,
    pub sizel: super::super::super::Foundation::SIZE,
    pub dvaspect: u32,
    pub dwFlags: REOBJECT_FLAGS,
    pub dwUser: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REOBJECT_FLAGS(pub u32);
impl REOBJECT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for REOBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for REOBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for REOBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for REOBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for REOBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const REO_ALIGNTORIGHT: REOBJECT_FLAGS = REOBJECT_FLAGS(256u32);
pub const REO_BELOWBASELINE: REOBJECT_FLAGS = REOBJECT_FLAGS(2u32);
pub const REO_BLANK: REOBJECT_FLAGS = REOBJECT_FLAGS(16u32);
pub const REO_CANROTATE: REOBJECT_FLAGS = REOBJECT_FLAGS(128u32);
pub const REO_DONTNEEDPALETTE: REOBJECT_FLAGS = REOBJECT_FLAGS(32u32);
pub const REO_DYNAMICSIZE: REOBJECT_FLAGS = REOBJECT_FLAGS(8u32);
pub const REO_GETMETAFILE: REOBJECT_FLAGS = REOBJECT_FLAGS(4194304u32);
pub const REO_GETOBJ_ALL_INTERFACES: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(7u32);
pub const REO_GETOBJ_NO_INTERFACES: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(0u32);
pub const REO_GETOBJ_POLEOBJ: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(1u32);
pub const REO_GETOBJ_POLESITE: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(4u32);
pub const REO_GETOBJ_PSTG: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(2u32);
pub const REO_HILITED: REOBJECT_FLAGS = REOBJECT_FLAGS(16777216u32);
pub const REO_INPLACEACTIVE: REOBJECT_FLAGS = REOBJECT_FLAGS(33554432u32);
pub const REO_INVERTEDSELECT: REOBJECT_FLAGS = REOBJECT_FLAGS(4u32);
pub const REO_LINK: REOBJECT_FLAGS = REOBJECT_FLAGS(2147483648u32);
pub const REO_LINKAVAILABLE: REOBJECT_FLAGS = REOBJECT_FLAGS(8388608u32);
pub const REO_NULL: i32 = 0i32;
pub const REO_OPEN: REOBJECT_FLAGS = REOBJECT_FLAGS(67108864u32);
pub const REO_OWNERDRAWSELECT: REOBJECT_FLAGS = REOBJECT_FLAGS(64u32);
pub const REO_READWRITEMASK: i32 = 2047i32;
pub const REO_RESIZABLE: REOBJECT_FLAGS = REOBJECT_FLAGS(1u32);
pub const REO_SELECTED: REOBJECT_FLAGS = REOBJECT_FLAGS(134217728u32);
pub const REO_STATIC: REOBJECT_FLAGS = REOBJECT_FLAGS(1073741824u32);
pub const REO_USEASBACKGROUND: REOBJECT_FLAGS = REOBJECT_FLAGS(1024u32);
pub const REO_WRAPTEXTAROUND: REOBJECT_FLAGS = REOBJECT_FLAGS(512u32);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPASTESPECIAL {
    pub dwAspect: super::super::super::System::Com::DVASPECT,
    pub dwParam: usize,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Default)]
pub struct REPASTESPECIAL {
    pub dwAspect: super::super::super::System::Com::DVASPECT,
    pub dwParam: usize,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REQRESIZE {
    pub nmhdr: super::NMHDR,
    pub rc: super::super::super::Foundation::RECT,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct REQRESIZE {
    pub nmhdr: super::NMHDR,
    pub rc: super::super::super::Foundation::RECT,
}
pub const RICHEDIT60_CLASS: windows_core::PCWSTR = windows_core::w!("RICHEDIT60W");
pub const RICHEDIT_CLASS: windows_core::PCWSTR = windows_core::w!("RichEdit20W");
pub const RICHEDIT_CLASS10A: windows_core::PCSTR = windows_core::s!("RICHEDIT");
pub const RICHEDIT_CLASSA: windows_core::PCSTR = windows_core::s!("RichEdit20A");
pub const RICHEDIT_CLASSW: windows_core::PCWSTR = windows_core::w!("RichEdit20W");
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RICHEDIT_IMAGE_PARAMETERS {
    pub xWidth: i32,
    pub yHeight: i32,
    pub Ascent: i32,
    pub Type: i32,
    pub pwszAlternateText: windows_core::PCWSTR,
    pub pIStream: core::mem::ManuallyDrop<Option<super::super::super::System::Com::IStream>>,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Default)]
pub struct RICHEDIT_IMAGE_PARAMETERS {
    pub xWidth: i32,
    pub yHeight: i32,
    pub Ascent: i32,
    pub Type: i32,
    pub pwszAlternateText: windows_core::PCWSTR,
    pub pIStream: core::mem::ManuallyDrop<Option<super::super::super::System::Com::IStream>>,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(pub u16);
impl RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RICH_EDIT_GET_OBJECT_FLAGS(pub u32);
impl RICH_EDIT_GET_OBJECT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RICH_EDIT_GET_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RICH_EDIT_GET_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RICH_EDIT_GET_OBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RICH_EDIT_GET_OBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RICH_EDIT_GET_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RTO_DISABLEHANDLES: u32 = 2u32;
pub const RTO_READINGMODE: u32 = 3u32;
pub const RTO_SHOWHANDLES: u32 = 1u32;
pub const SCF_ALL: u32 = 4u32;
pub const SCF_ASSOCIATEFONT: u32 = 16u32;
pub const SCF_ASSOCIATEFONT2: u32 = 64u32;
pub const SCF_CHARREPFROMLCID: u32 = 256u32;
pub const SCF_DEFAULT: u32 = 0u32;
pub const SCF_NOKBUPDATE: u32 = 32u32;
pub const SCF_SELECTION: u32 = 1u32;
pub const SCF_SMARTFONT: u32 = 128u32;
pub const SCF_USEUIRULES: u32 = 8u32;
pub const SCF_WORD: u32 = 2u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SELCHANGE {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct SELCHANGE {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
pub const SEL_EMPTY: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(0u16);
pub const SEL_MULTICHAR: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(4u16);
pub const SEL_MULTIOBJECT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(8u16);
pub const SEL_OBJECT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(2u16);
pub const SEL_TEXT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(1u16);
pub const SES_ALLOWBEEPS: u32 = 256u32;
pub const SES_BEEPONMAXTEXT: u32 = 2u32;
pub const SES_BIDI: u32 = 4096u32;
pub const SES_CTFALLOWEMBED: u32 = 2097152u32;
pub const SES_CTFALLOWPROOFING: u32 = 8388608u32;
pub const SES_CTFALLOWSMARTTAG: u32 = 4194304u32;
pub const SES_CTFNOLOCK: u32 = 268435456u32;
pub const SES_CUSTOMLOOK: u32 = 524288u32;
pub const SES_DEFAULTLATINLIGA: u32 = 16u32;
pub const SES_DRAFTMODE: u32 = 32768u32;
pub const SES_EMULATE10: u32 = 16u32;
pub const SES_EMULATESYSEDIT: u32 = 1u32;
pub const SES_EXTENDBACKCOLOR: u32 = 4u32;
pub const SES_EX_HANDLEFRIENDLYURL: u32 = 256u32;
pub const SES_EX_HIDETEMPFORMAT: u32 = 268435456u32;
pub const SES_EX_MULTITOUCH: u32 = 134217728u32;
pub const SES_EX_NOACETATESELECTION: u32 = 1048576u32;
pub const SES_EX_NOMATH: u32 = 64u32;
pub const SES_EX_NOTABLE: u32 = 4u32;
pub const SES_EX_NOTHEMING: u32 = 524288u32;
pub const SES_EX_USEMOUSEWPARAM: u32 = 536870912u32;
pub const SES_EX_USESINGLELINE: u32 = 2097152u32;
pub const SES_HIDEGRIDLINES: u32 = 131072u32;
pub const SES_HYPERLINKTOOLTIPS: u32 = 8u32;
pub const SES_LBSCROLLNOTIFY: u32 = 1048576u32;
pub const SES_LOGICALCARET: u32 = 16777216u32;
pub const SES_LOWERCASE: u32 = 1024u32;
pub const SES_MAPCPS: u32 = 8u32;
pub const SES_MAX: u32 = 536870912u32;
pub const SES_MULTISELECT: u32 = 134217728u32;
pub const SES_NOEALINEHEIGHTADJUST: u32 = 536870912u32;
pub const SES_NOFOCUSLINKNOTIFY: u32 = 32u32;
pub const SES_NOIME: u32 = 128u32;
pub const SES_NOINPUTSEQUENCECHK: u32 = 2048u32;
pub const SES_SCROLLONKILLFOCUS: u32 = 8192u32;
pub const SES_SMARTDRAGDROP: u32 = 67108864u32;
pub const SES_UPPERCASE: u32 = 512u32;
pub const SES_USEAIMM: u32 = 64u32;
pub const SES_USEATFONT: u32 = 262144u32;
pub const SES_USECRLF: u32 = 32u32;
pub const SES_USECTF: u32 = 65536u32;
pub const SES_WORDDRAGDROP: u32 = 33554432u32;
pub const SES_XLTCRCRLFTOCR: u32 = 16384u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SETTEXTEX {
    pub flags: u32,
    pub codepage: u32,
}
pub const SFF_KEEPDOCINFO: u32 = 4096u32;
pub const SFF_PERSISTVIEWSCALE: u32 = 8192u32;
pub const SFF_PLAINRTF: u32 = 16384u32;
pub const SFF_PWD: u32 = 2048u32;
pub const SFF_SELECTION: u32 = 32768u32;
pub const SFF_WRITEXTRAPAR: u32 = 128u32;
pub const SF_NCRFORNONASCII: u32 = 64u32;
pub const SF_RTF: u32 = 2u32;
pub const SF_RTFNOOBJS: u32 = 3u32;
pub const SF_RTFVAL: u32 = 1792u32;
pub const SF_TEXT: u32 = 1u32;
pub const SF_TEXTIZED: u32 = 4u32;
pub const SF_UNICODE: u32 = 16u32;
pub const SF_USECODEPAGE: u32 = 32u32;
pub const SPF_DONTSETDEFAULT: u32 = 2u32;
pub const SPF_SETDEFAULT: u32 = 4u32;
pub const ST_DEFAULT: u32 = 0u32;
pub const ST_KEEPUNDO: u32 = 1u32;
pub const ST_NEWCHARS: u32 = 4u32;
pub const ST_SELECTION: u32 = 2u32;
pub const ST_UNICODE: u32 = 8u32;
pub const S_MSG_KEY_IGNORED: windows_core::HRESULT = windows_core::HRESULT(0x40201_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TABLECELLPARMS {
    pub dxWidth: i32,
    pub _bitfield: u16,
    pub wShading: u16,
    pub dxBrdrLeft: i16,
    pub dyBrdrTop: i16,
    pub dxBrdrRight: i16,
    pub dyBrdrBottom: i16,
    pub crBrdrLeft: super::super::super::Foundation::COLORREF,
    pub crBrdrTop: super::super::super::Foundation::COLORREF,
    pub crBrdrRight: super::super::super::Foundation::COLORREF,
    pub crBrdrBottom: super::super::super::Foundation::COLORREF,
    pub crBackPat: super::super::super::Foundation::COLORREF,
    pub crForePat: super::super::super::Foundation::COLORREF,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TEXTMODE(pub i32);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TEXTRANGEA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PSTR,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct TEXTRANGEA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PSTR,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TEXTRANGEW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PWSTR,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct TEXTRANGEW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PWSTR,
}
pub const TM_MULTICODEPAGE: TEXTMODE = TEXTMODE(32i32);
pub const TM_MULTILEVELUNDO: TEXTMODE = TEXTMODE(8i32);
pub const TM_PLAINTEXT: TEXTMODE = TEXTMODE(1i32);
pub const TM_RICHTEXT: TEXTMODE = TEXTMODE(2i32);
pub const TM_SINGLECODEPAGE: TEXTMODE = TEXTMODE(16i32);
pub const TM_SINGLELEVELUNDO: TEXTMODE = TEXTMODE(4i32);
pub const TO_ADVANCEDLAYOUT: u32 = 8u32;
pub const TO_ADVANCEDTYPOGRAPHY: u32 = 1u32;
pub const TO_DISABLECUSTOMTEXTOUT: u32 = 4u32;
pub const TO_SIMPLELINEBREAK: u32 = 2u32;
pub const TXES_ISDIALOG: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXTBACKSTYLE(pub i32);
pub const TXTBACK_OPAQUE: TXTBACKSTYLE = TXTBACKSTYLE(1i32);
pub const TXTBACK_TRANSPARENT: TXTBACKSTYLE = TXTBACKSTYLE(0i32);
pub const TXTBIT_ADVANCEDINPUT: u32 = 536870912u32;
pub const TXTBIT_ALLOWBEEP: u32 = 2048u32;
pub const TXTBIT_AUTOWORDSEL: u32 = 128u32;
pub const TXTBIT_BACKSTYLECHANGE: u32 = 16384u32;
pub const TXTBIT_CHARFORMATCHANGE: u32 = 131072u32;
pub const TXTBIT_CLIENTRECTCHANGE: u32 = 1048576u32;
pub const TXTBIT_D2DDWRITE: u32 = 16777216u32;
pub const TXTBIT_D2DPIXELSNAPPED: u32 = 67108864u32;
pub const TXTBIT_D2DSIMPLETYPOGRAPHY: u32 = 33554432u32;
pub const TXTBIT_D2DSUBPIXELLINES: u32 = 134217728u32;
pub const TXTBIT_DISABLEDRAG: u32 = 4096u32;
pub const TXTBIT_EXTENTCHANGE: u32 = 524288u32;
pub const TXTBIT_FLASHLASTPASSWORDCHAR: u32 = 268435456u32;
pub const TXTBIT_HIDESELECTION: u32 = 32u32;
pub const TXTBIT_MAXLENGTHCHANGE: u32 = 32768u32;
pub const TXTBIT_MULTILINE: u32 = 2u32;
pub const TXTBIT_NOTHREADREFCOUNT: u32 = 4194304u32;
pub const TXTBIT_PARAFORMATCHANGE: u32 = 262144u32;
pub const TXTBIT_READONLY: u32 = 4u32;
pub const TXTBIT_RICHTEXT: u32 = 1u32;
pub const TXTBIT_SAVESELECTION: u32 = 64u32;
pub const TXTBIT_SCROLLBARCHANGE: u32 = 65536u32;
pub const TXTBIT_SELBARCHANGE: u32 = 512u32;
pub const TXTBIT_SHOWACCELERATOR: u32 = 8u32;
pub const TXTBIT_SHOWPASSWORD: u32 = 8388608u32;
pub const TXTBIT_USECURRENTBKG: u32 = 2097152u32;
pub const TXTBIT_USEPASSWORD: u32 = 16u32;
pub const TXTBIT_VERTICAL: u32 = 256u32;
pub const TXTBIT_VIEWINSETCHANGE: u32 = 8192u32;
pub const TXTBIT_WORDWRAP: u32 = 1024u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXTHITRESULT(pub i32);
pub const TXTHITRESULT_CLOSE: TXTHITRESULT = TXTHITRESULT(2i32);
pub const TXTHITRESULT_HIT: TXTHITRESULT = TXTHITRESULT(3i32);
pub const TXTHITRESULT_NOHIT: TXTHITRESULT = TXTHITRESULT(0i32);
pub const TXTHITRESULT_TRANSPARENT: TXTHITRESULT = TXTHITRESULT(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXTNATURALSIZE(pub i32);
pub const TXTNS_EMU: TXTNATURALSIZE = TXTNATURALSIZE(-2147483648i32);
pub const TXTNS_FITTOCONTENT: TXTNATURALSIZE = TXTNATURALSIZE(1i32);
pub const TXTNS_FITTOCONTENT2: TXTNATURALSIZE = TXTNATURALSIZE(0i32);
pub const TXTNS_FITTOCONTENT3: TXTNATURALSIZE = TXTNATURALSIZE(3i32);
pub const TXTNS_FITTOCONTENTWSP: TXTNATURALSIZE = TXTNATURALSIZE(4i32);
pub const TXTNS_INCLUDELASTLINE: TXTNATURALSIZE = TXTNATURALSIZE(1073741824i32);
pub const TXTNS_ROUNDTOLINE: TXTNATURALSIZE = TXTNATURALSIZE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TXTVIEW(pub i32);
pub const TXTVIEW_ACTIVE: TXTVIEW = TXTVIEW(0i32);
pub const TXTVIEW_INACTIVE: TXTVIEW = TXTVIEW(-1i32);
pub const UID_AUTOTABLE: UNDONAMEID = UNDONAMEID(6i32);
pub const UID_CUT: UNDONAMEID = UNDONAMEID(4i32);
pub const UID_DELETE: UNDONAMEID = UNDONAMEID(2i32);
pub const UID_DRAGDROP: UNDONAMEID = UNDONAMEID(3i32);
pub const UID_PASTE: UNDONAMEID = UNDONAMEID(5i32);
pub const UID_TYPING: UNDONAMEID = UNDONAMEID(1i32);
pub const UID_UNKNOWN: UNDONAMEID = UNDONAMEID(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNDONAMEID(pub i32);
pub const VM_NORMAL: u32 = 4u32;
pub const VM_OUTLINE: u32 = 2u32;
pub const VM_PAGE: u32 = 9u32;
pub const WBF_CUSTOM: u32 = 512u32;
pub const WBF_LEVEL1: u32 = 128u32;
pub const WBF_LEVEL2: u32 = 256u32;
pub const WBF_OVERFLOW: u32 = 64u32;
pub const WBF_WORDBREAK: u32 = 32u32;
pub const WBF_WORDWRAP: u32 = 16u32;
pub const WB_MOVEWORDNEXT: u32 = 5u32;
pub const WB_MOVEWORDPREV: u32 = 4u32;
pub const WB_NEXTBREAK: u32 = 7u32;
pub const WB_PREVBREAK: u32 = 6u32;
pub const cchTextLimitDefault: u32 = 32767u32;
pub const khyphAddBefore: KHYPH = KHYPH(2i32);
pub const khyphChangeAfter: KHYPH = KHYPH(5i32);
pub const khyphChangeBefore: KHYPH = KHYPH(3i32);
pub const khyphDelAndChange: KHYPH = KHYPH(6i32);
pub const khyphDeleteBefore: KHYPH = KHYPH(4i32);
pub const khyphNil: KHYPH = KHYPH(0i32);
pub const khyphNormal: KHYPH = KHYPH(1i32);
pub const lDefaultTab: u32 = 720u32;
pub const tomAboriginal: tomConstants = tomConstants(39i32);
pub const tomAccent: OBJECTTYPE = OBJECTTYPE(10i32);
pub const tomAdjustCRLF: tomConstants = tomConstants(1i32);
pub const tomAlignBar: tomConstants = tomConstants(4i32);
pub const tomAlignCenter: tomConstants = tomConstants(1i32);
pub const tomAlignDecimal: tomConstants = tomConstants(3i32);
pub const tomAlignDefault: tomConstants = tomConstants(0i32);
pub const tomAlignInterLetter: tomConstants = tomConstants(5i32);
pub const tomAlignInterWord: tomConstants = tomConstants(3i32);
pub const tomAlignJustify: tomConstants = tomConstants(3i32);
pub const tomAlignLeft: tomConstants = tomConstants(0i32);
pub const tomAlignMatchAscentDescent: tomConstants = tomConstants(2i32);
pub const tomAlignNewspaper: tomConstants = tomConstants(4i32);
pub const tomAlignRight: tomConstants = tomConstants(2i32);
pub const tomAlignScaled: tomConstants = tomConstants(6i32);
pub const tomAllCaps: tomConstants = tomConstants(-2147483520i32);
pub const tomAllowFinalEOP: tomConstants = tomConstants(8i32);
pub const tomAllowMathBold: tomConstants = tomConstants(2048i32);
pub const tomAllowOffClient: tomConstants = tomConstants(512i32);
pub const tomAnimationMax: tomConstants = tomConstants(8i32);
pub const tomAnsi: tomConstants = tomConstants(0i32);
pub const tomApplyLater: tomConstants = tomConstants(1i32);
pub const tomApplyNow: tomConstants = tomConstants(0i32);
pub const tomApplyRtfDocProps: tomConstants = tomConstants(16384i32);
pub const tomApplyTmp: tomConstants = tomConstants(4i32);
pub const tomArabic: tomConstants = tomConstants(6i32);
pub const tomArmenian: tomConstants = tomConstants(19i32);
pub const tomAtEnd: tomConstants = tomConstants(4096i32);
pub const tomAutoBackColor: tomConstants = tomConstants(-2080374784i32);
pub const tomAutoColor: tomConstants = tomConstants(-9999997i32);
pub const tomAutoLinkEmail: tomConstants = tomConstants(5i32);
pub const tomAutoLinkPath: tomConstants = tomConstants(7i32);
pub const tomAutoLinkPhone: tomConstants = tomConstants(6i32);
pub const tomAutoLinkURL: tomConstants = tomConstants(4i32);
pub const tomAutoSpaceAlpha: tomConstants = tomConstants(4i32);
pub const tomAutoSpaceNumeric: tomConstants = tomConstants(8i32);
pub const tomAutoSpaceParens: tomConstants = tomConstants(16i32);
pub const tomAutoTextColor: tomConstants = tomConstants(-1073741824i32);
pub const tomBIG5: tomConstants = tomConstants(15i32);
pub const tomBackward: tomConstants = tomConstants(-1073741823i32);
pub const tomBaltic: tomConstants = tomConstants(7i32);
pub const tomBengali: tomConstants = tomConstants(23i32);
pub const tomBlinkingBackground: tomConstants = tomConstants(2i32);
pub const tomBold: tomConstants = tomConstants(-2147483647i32);
pub const tomBox: OBJECTTYPE = OBJECTTYPE(11i32);
pub const tomBoxAlignCenter: tomConstants = tomConstants(1i32);
pub const tomBoxHideBottom: tomConstants = tomConstants(2i32);
pub const tomBoxHideLeft: tomConstants = tomConstants(4i32);
pub const tomBoxHideRight: tomConstants = tomConstants(8i32);
pub const tomBoxHideTop: tomConstants = tomConstants(1i32);
pub const tomBoxStrikeBLTR: tomConstants = tomConstants(128i32);
pub const tomBoxStrikeH: tomConstants = tomConstants(16i32);
pub const tomBoxStrikeTLBR: tomConstants = tomConstants(64i32);
pub const tomBoxStrikeV: tomConstants = tomConstants(32i32);
pub const tomBoxedFormula: OBJECTTYPE = OBJECTTYPE(12i32);
pub const tomBrackets: OBJECTTYPE = OBJECTTYPE(13i32);
pub const tomBracketsWithSeps: OBJECTTYPE = OBJECTTYPE(14i32);
pub const tomBraille: tomConstants = tomConstants(44i32);
pub const tomCacheParms: tomConstants = tomConstants(3i32);
pub const tomCanCopy: tomConstants = tomConstants(137i32);
pub const tomCanRedo: tomConstants = tomConstants(138i32);
pub const tomCanUndo: tomConstants = tomConstants(139i32);
pub const tomCell: tomConstants = tomConstants(12i32);
pub const tomCellStructureChangeOnly: tomConstants = tomConstants(1i32);
pub const tomCharFormat: tomConstants = tomConstants(13i32);
pub const tomCharRepFromLcid: tomConstants = tomConstants(1073741824i32);
pub const tomCharRepMax: tomConstants = tomConstants(63i32);
pub const tomCharacter: tomConstants = tomConstants(1i32);
pub const tomCharset: tomConstants = tomConstants(-2147483648i32);
pub const tomCheckTextLimit: tomConstants = tomConstants(32i32);
pub const tomCherokee: tomConstants = tomConstants(38i32);
pub const tomClientCoord: tomConstants = tomConstants(256i32);
pub const tomClientLink: tomConstants = tomConstants(1i32);
pub const tomCluster: tomConstants = tomConstants(19i32);
pub const tomCollapseEnd: tomConstants = tomConstants(0i32);
pub const tomCollapseStart: tomConstants = tomConstants(1i32);
pub const tomColumn: tomConstants = tomConstants(9i32);
pub const tomCommentsStory: tomConstants = tomConstants(4i32);
pub const tomCompressMax: tomConstants = tomConstants(2i32);
pub const tomCompressNone: tomConstants = tomConstants(0i32);
pub const tomCompressPunctuation: tomConstants = tomConstants(1i32);
pub const tomCompressPunctuationAndKana: tomConstants = tomConstants(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct tomConstants(pub i32);
pub const tomConvertMathChar: tomConstants = tomConstants(512i32);
pub const tomConvertRTF: tomConstants = tomConstants(8192i32);
pub const tomCreateAlways: tomConstants = tomConstants(32i32);
pub const tomCreateNew: tomConstants = tomConstants(16i32);
pub const tomCyrillic: tomConstants = tomConstants(2i32);
pub const tomDash: tomConstants = tomConstants(5i32);
pub const tomDashDot: tomConstants = tomConstants(6i32);
pub const tomDashDotDot: tomConstants = tomConstants(7i32);
pub const tomDashes: tomConstants = tomConstants(2i32);
pub const tomDecDecSize: tomConstants = tomConstants(254i32);
pub const tomDecSize: tomConstants = tomConstants(255i32);
pub const tomDefault: tomConstants = tomConstants(-9999996i32);
pub const tomDefaultCharRep: tomConstants = tomConstants(9i32);
pub const tomDefaultTab: tomConstants = tomConstants(5i32);
pub const tomDeseret: tomConstants = tomConstants(61i32);
pub const tomDevanagari: tomConstants = tomConstants(22i32);
pub const tomDisableSmartFont: tomConstants = tomConstants(8i32);
pub const tomDisabled: tomConstants = tomConstants(-2147475456i32);
pub const tomDocAutoLink: tomConstants = tomConstants(141i32);
pub const tomDocMathBuild: tomConstants = tomConstants(128i32);
pub const tomDontGrowWithContent: tomConstants = tomConstants(64i32);
pub const tomDots: tomConstants = tomConstants(1i32);
pub const tomDotted: tomConstants = tomConstants(4i32);
pub const tomDouble: tomConstants = tomConstants(3i32);
pub const tomDoubleWave: tomConstants = tomConstants(11i32);
pub const tomDoublestrike: tomConstants = tomConstants(64i32);
pub const tomEastEurope: tomConstants = tomConstants(1i32);
pub const tomEllipsisEnd: tomConstants = tomConstants(1i32);
pub const tomEllipsisMode: tomConstants = tomConstants(142i32);
pub const tomEllipsisNone: tomConstants = tomConstants(0i32);
pub const tomEllipsisPresent: tomConstants = tomConstants(1i32);
pub const tomEllipsisState: tomConstants = tomConstants(143i32);
pub const tomEllipsisWord: tomConstants = tomConstants(3i32);
pub const tomEmbeddedFont: tomConstants = tomConstants(32i32);
pub const tomEmboss: tomConstants = tomConstants(-2147481600i32);
pub const tomEmoji: tomConstants = tomConstants(53i32);
pub const tomEnableSmartFont: tomConstants = tomConstants(9i32);
pub const tomEnd: tomConstants = tomConstants(0i32);
pub const tomEndnotesStory: tomConstants = tomConstants(3i32);
pub const tomEq: OBJECTTYPE = OBJECTTYPE(9i32);
pub const tomEqArrayAlignBottomRow: tomConstants = tomConstants(12i32);
pub const tomEqArrayAlignCenter: tomConstants = tomConstants(0i32);
pub const tomEqArrayAlignMask: tomConstants = tomConstants(12i32);
pub const tomEqArrayAlignTopRow: tomConstants = tomConstants(4i32);
pub const tomEqArrayLayoutWidth: tomConstants = tomConstants(1i32);
pub const tomEquals: tomConstants = tomConstants(5i32);
pub const tomEquationArray: OBJECTTYPE = OBJECTTYPE(15i32);
pub const tomEthiopic: tomConstants = tomConstants(37i32);
pub const tomEvenPagesFooterStory: tomConstants = tomConstants(8i32);
pub const tomEvenPagesHeaderStory: tomConstants = tomConstants(6i32);
pub const tomExtend: tomConstants = tomConstants(1i32);
pub const tomExtendedChar: tomConstants = tomConstants(-2113929216i32);
pub const tomFalse: tomConstants = tomConstants(0i32);
pub const tomFindStory: tomConstants = tomConstants(128i32);
pub const tomFirstPageFooterStory: tomConstants = tomConstants(11i32);
pub const tomFirstPageHeaderStory: tomConstants = tomConstants(10i32);
pub const tomFoldMathAlpha: tomConstants = tomConstants(16i32);
pub const tomFontAlignmentAuto: tomConstants = tomConstants(0i32);
pub const tomFontAlignmentBaseline: tomConstants = tomConstants(2i32);
pub const tomFontAlignmentBottom: tomConstants = tomConstants(3i32);
pub const tomFontAlignmentCenter: tomConstants = tomConstants(4i32);
pub const tomFontAlignmentMax: tomConstants = tomConstants(4i32);
pub const tomFontAlignmentTop: tomConstants = tomConstants(1i32);
pub const tomFontBound: tomConstants = tomConstants(-2146435072i32);
pub const tomFontPropAlign: tomConstants = tomConstants(829i32);
pub const tomFontPropTeXStyle: tomConstants = tomConstants(828i32);
pub const tomFontStretch: tomConstants = tomConstants(830i32);
pub const tomFontStretchCondensed: tomConstants = tomConstants(3i32);
pub const tomFontStretchDefault: tomConstants = tomConstants(0i32);
pub const tomFontStretchExpanded: tomConstants = tomConstants(7i32);
pub const tomFontStretchExtraCondensed: tomConstants = tomConstants(2i32);
pub const tomFontStretchExtraExpanded: tomConstants = tomConstants(8i32);
pub const tomFontStretchNormal: tomConstants = tomConstants(5i32);
pub const tomFontStretchSemiCondensed: tomConstants = tomConstants(4i32);
pub const tomFontStretchSemiExpanded: tomConstants = tomConstants(6i32);
pub const tomFontStretchUltraCondensed: tomConstants = tomConstants(1i32);
pub const tomFontStretchUltraExpanded: tomConstants = tomConstants(9i32);
pub const tomFontStyle: tomConstants = tomConstants(831i32);
pub const tomFontStyleItalic: tomConstants = tomConstants(2i32);
pub const tomFontStyleOblique: tomConstants = tomConstants(1i32);
pub const tomFontStyleUpright: tomConstants = tomConstants(0i32);
pub const tomFontWeightBlack: tomConstants = tomConstants(900i32);
pub const tomFontWeightBold: tomConstants = tomConstants(700i32);
pub const tomFontWeightDefault: tomConstants = tomConstants(0i32);
pub const tomFontWeightExtraBlack: tomConstants = tomConstants(950i32);
pub const tomFontWeightExtraBold: tomConstants = tomConstants(800i32);
pub const tomFontWeightExtraLight: tomConstants = tomConstants(200i32);
pub const tomFontWeightHeavy: tomConstants = tomConstants(900i32);
pub const tomFontWeightLight: tomConstants = tomConstants(300i32);
pub const tomFontWeightMedium: tomConstants = tomConstants(500i32);
pub const tomFontWeightNormal: tomConstants = tomConstants(400i32);
pub const tomFontWeightRegular: tomConstants = tomConstants(400i32);
pub const tomFontWeightSemiBold: tomConstants = tomConstants(600i32);
pub const tomFontWeightThin: tomConstants = tomConstants(100i32);
pub const tomFootnotesStory: tomConstants = tomConstants(2i32);
pub const tomForward: tomConstants = tomConstants(1073741823i32);
pub const tomFraction: OBJECTTYPE = OBJECTTYPE(16i32);
pub const tomFriendlyLinkAddress: tomConstants = tomConstants(3i32);
pub const tomFriendlyLinkName: tomConstants = tomConstants(2i32);
pub const tomFunctionApply: OBJECTTYPE = OBJECTTYPE(17i32);
pub const tomFunctionTypeIsLim: tomConstants = tomConstants(4i32);
pub const tomFunctionTypeNone: tomConstants = tomConstants(0i32);
pub const tomFunctionTypeTakesArg: tomConstants = tomConstants(1i32);
pub const tomFunctionTypeTakesLim: tomConstants = tomConstants(2i32);
pub const tomFunctionTypeTakesLim2: tomConstants = tomConstants(3i32);
pub const tomGB2312: tomConstants = tomConstants(13i32);
pub const tomGeorgian: tomConstants = tomConstants(35i32);
pub const tomGetHeightOnly: tomConstants = tomConstants(8i32);
pub const tomGlagolitic: tomConstants = tomConstants(54i32);
pub const tomGothic: tomConstants = tomConstants(60i32);
pub const tomGravityBack: tomConstants = tomConstants(1i32);
pub const tomGravityBackward: tomConstants = tomConstants(536870912i32);
pub const tomGravityFore: tomConstants = tomConstants(2i32);
pub const tomGravityForward: tomConstants = tomConstants(1073741824i32);
pub const tomGravityIn: tomConstants = tomConstants(3i32);
pub const tomGravityOut: tomConstants = tomConstants(4i32);
pub const tomGravityUI: tomConstants = tomConstants(0i32);
pub const tomGreek: tomConstants = tomConstants(3i32);
pub const tomGrowWithContent: tomConstants = tomConstants(128i32);
pub const tomGujarati: tomConstants = tomConstants(25i32);
pub const tomGurmukhi: tomConstants = tomConstants(24i32);
pub const tomHContCell: tomConstants = tomConstants(8i32);
pub const tomHStartCell: tomConstants = tomConstants(4i32);
pub const tomHTML: tomConstants = tomConstants(3i32);
pub const tomHair: tomConstants = tomConstants(10i32);
pub const tomHangul: tomConstants = tomConstants(14i32);
pub const tomHardParagraph: tomConstants = tomConstants(18i32);
pub const tomHeavyWave: tomConstants = tomConstants(12i32);
pub const tomHebrew: tomConstants = tomConstants(5i32);
pub const tomHidden: tomConstants = tomConstants(-2147483392i32);
pub const tomHorzVert: OBJECTTYPE = OBJECTTYPE(2i32);
pub const tomHstring: tomConstants = tomConstants(596i32);
pub const tomIgnoreCurrentFont: tomConstants = tomConstants(0i32);
pub const tomIgnoreNumberStyle: tomConstants = tomConstants(16777216i32);
pub const tomImprint: tomConstants = tomConstants(-2147479552i32);
pub const tomIncIncSize: tomConstants = tomConstants(66i32);
pub const tomIncSize: tomConstants = tomConstants(65i32);
pub const tomIncludeInset: tomConstants = tomConstants(1i32);
pub const tomIncludeNumbering: tomConstants = tomConstants(64i32);
pub const tomInlineObject: tomConstants = tomConstants(20i32);
pub const tomInlineObjectArg: tomConstants = tomConstants(21i32);
pub const tomInlineObjectStart: tomConstants = tomConstants(-2130706432i32);
pub const tomItalic: tomConstants = tomConstants(-2147483646i32);
pub const tomJamo: tomConstants = tomConstants(36i32);
pub const tomKannada: tomConstants = tomConstants(29i32);
pub const tomKayahli: tomConstants = tomConstants(51i32);
pub const tomKharoshthi: tomConstants = tomConstants(50i32);
pub const tomKhmer: tomConstants = tomConstants(42i32);
pub const tomKoreanBlockCaret: tomConstants = tomConstants(1i32);
pub const tomLanguageTag: tomConstants = tomConstants(4096i32);
pub const tomLao: tomConstants = tomConstants(32i32);
pub const tomLasVegasLights: tomConstants = tomConstants(1i32);
pub const tomLayoutColumn: tomConstants = tomConstants(23i32);
pub const tomLeafLine: tomConstants = tomConstants(22i32);
pub const tomLeftSubSup: OBJECTTYPE = OBJECTTYPE(18i32);
pub const tomLimbu: tomConstants = tomConstants(46i32);
pub const tomLimitAlignCenter: tomConstants = tomConstants(0i32);
pub const tomLimitAlignLeft: tomConstants = tomConstants(1i32);
pub const tomLimitAlignMask: tomConstants = tomConstants(3i32);
pub const tomLimitAlignRight: tomConstants = tomConstants(2i32);
pub const tomLimitsDefault: tomConstants = tomConstants(0i32);
pub const tomLimitsOpposite: tomConstants = tomConstants(4i32);
pub const tomLimitsSubSup: tomConstants = tomConstants(2i32);
pub const tomLimitsUnderOver: tomConstants = tomConstants(1i32);
pub const tomLine: tomConstants = tomConstants(5i32);
pub const tomLineSpace1pt5: tomConstants = tomConstants(1i32);
pub const tomLineSpaceAtLeast: tomConstants = tomConstants(3i32);
pub const tomLineSpaceDouble: tomConstants = tomConstants(2i32);
pub const tomLineSpaceExactly: tomConstants = tomConstants(4i32);
pub const tomLineSpaceMultiple: tomConstants = tomConstants(5i32);
pub const tomLineSpacePercent: tomConstants = tomConstants(6i32);
pub const tomLineSpaceSingle: tomConstants = tomConstants(0i32);
pub const tomLines: tomConstants = tomConstants(3i32);
pub const tomLink: tomConstants = tomConstants(-2147483616i32);
pub const tomLinkProtected: tomConstants = tomConstants(-2139095040i32);
pub const tomListBullet: tomConstants = tomConstants(1i32);
pub const tomListMinus: tomConstants = tomConstants(524288i32);
pub const tomListNoNumber: tomConstants = tomConstants(262144i32);
pub const tomListNone: tomConstants = tomConstants(0i32);
pub const tomListNumberAsArabic: tomConstants = tomConstants(2i32);
pub const tomListNumberAsLCLetter: tomConstants = tomConstants(3i32);
pub const tomListNumberAsLCRoman: tomConstants = tomConstants(5i32);
pub const tomListNumberAsSequence: tomConstants = tomConstants(7i32);
pub const tomListNumberAsUCLetter: tomConstants = tomConstants(4i32);
pub const tomListNumberAsUCRoman: tomConstants = tomConstants(6i32);
pub const tomListNumberedArabic1: tomConstants = tomConstants(16i32);
pub const tomListNumberedArabic2: tomConstants = tomConstants(17i32);
pub const tomListNumberedArabicWide: tomConstants = tomConstants(11i32);
pub const tomListNumberedBlackCircleWingding: tomConstants = tomConstants(9i32);
pub const tomListNumberedChS: tomConstants = tomConstants(12i32);
pub const tomListNumberedChT: tomConstants = tomConstants(13i32);
pub const tomListNumberedCircle: tomConstants = tomConstants(8i32);
pub const tomListNumberedHebrew: tomConstants = tomConstants(18i32);
pub const tomListNumberedHindiAlpha: tomConstants = tomConstants(21i32);
pub const tomListNumberedHindiAlpha1: tomConstants = tomConstants(22i32);
pub const tomListNumberedHindiNum: tomConstants = tomConstants(23i32);
pub const tomListNumberedJpnChS: tomConstants = tomConstants(14i32);
pub const tomListNumberedJpnKor: tomConstants = tomConstants(15i32);
pub const tomListNumberedThaiAlpha: tomConstants = tomConstants(19i32);
pub const tomListNumberedThaiNum: tomConstants = tomConstants(20i32);
pub const tomListNumberedWhiteCircleWingding: tomConstants = tomConstants(10i32);
pub const tomListParentheses: tomConstants = tomConstants(65536i32);
pub const tomListPeriod: tomConstants = tomConstants(131072i32);
pub const tomListPlain: tomConstants = tomConstants(196608i32);
pub const tomLisu: tomConstants = tomConstants(55i32);
pub const tomLongDash: tomConstants = tomConstants(13i32);
pub const tomLowerCase: tomConstants = tomConstants(0i32);
pub const tomLowerLimit: OBJECTTYPE = OBJECTTYPE(19i32);
pub const tomMac: tomConstants = tomConstants(18i32);
pub const tomMainTextStory: tomConstants = tomConstants(1i32);
pub const tomMalayalam: tomConstants = tomConstants(30i32);
pub const tomMarchingBlackAnts: tomConstants = tomConstants(4i32);
pub const tomMarchingRedAnts: tomConstants = tomConstants(5i32);
pub const tomMatchAscii: tomConstants = tomConstants(4i32);
pub const tomMatchCase: tomConstants = tomConstants(4i32);
pub const tomMatchCharRep: tomConstants = tomConstants(1i32);
pub const tomMatchFontSignature: tomConstants = tomConstants(2i32);
pub const tomMatchMathFont: tomConstants = tomConstants(16i32);
pub const tomMatchPattern: tomConstants = tomConstants(8i32);
pub const tomMatchWord: tomConstants = tomConstants(2i32);
pub const tomMath: OBJECTTYPE = OBJECTTYPE(10i32);
pub const tomMathArgShadingEnd: tomConstants = tomConstants(594i32);
pub const tomMathArgShadingStart: tomConstants = tomConstants(593i32);
pub const tomMathBreakCenter: tomConstants = tomConstants(126i32);
pub const tomMathBreakLeft: tomConstants = tomConstants(125i32);
pub const tomMathBreakRight: tomConstants = tomConstants(127i32);
pub const tomMathBrkBinAfter: tomConstants = tomConstants(65536i32);
pub const tomMathBrkBinBefore: tomConstants = tomConstants(0i32);
pub const tomMathBrkBinDup: tomConstants = tomConstants(131072i32);
pub const tomMathBrkBinMask: tomConstants = tomConstants(196608i32);
pub const tomMathBrkBinSubMM: tomConstants = tomConstants(0i32);
pub const tomMathBrkBinSubMP: tomConstants = tomConstants(524288i32);
pub const tomMathBrkBinSubMask: tomConstants = tomConstants(786432i32);
pub const tomMathBrkBinSubPM: tomConstants = tomConstants(262144i32);
pub const tomMathCFCheck: tomConstants = tomConstants(4i32);
pub const tomMathDispAlignCenter: tomConstants = tomConstants(1i32);
pub const tomMathDispAlignCenterGroup: tomConstants = tomConstants(0i32);
pub const tomMathDispAlignLeft: tomConstants = tomConstants(2i32);
pub const tomMathDispAlignMask: tomConstants = tomConstants(3i32);
pub const tomMathDispAlignRight: tomConstants = tomConstants(3i32);
pub const tomMathDispDef: tomConstants = tomConstants(2048i32);
pub const tomMathDispFracTeX: tomConstants = tomConstants(8i32);
pub const tomMathDispIntUnderOver: tomConstants = tomConstants(4i32);
pub const tomMathDispNaryGrow: tomConstants = tomConstants(16i32);
pub const tomMathDispNarySubSup: tomConstants = tomConstants(1024i32);
pub const tomMathDocDiffDefault: tomConstants = tomConstants(0i32);
pub const tomMathDocDiffItalic: tomConstants = tomConstants(512i32);
pub const tomMathDocDiffMask: tomConstants = tomConstants(768i32);
pub const tomMathDocDiffOpenItalic: tomConstants = tomConstants(768i32);
pub const tomMathDocDiffUpright: tomConstants = tomConstants(256i32);
pub const tomMathDocEmptyArgAlways: tomConstants = tomConstants(32i32);
pub const tomMathDocEmptyArgAuto: tomConstants = tomConstants(0i32);
pub const tomMathDocEmptyArgMask: tomConstants = tomConstants(96i32);
pub const tomMathDocEmptyArgNever: tomConstants = tomConstants(64i32);
pub const tomMathDocSbSpOpUnchanged: tomConstants = tomConstants(128i32);
pub const tomMathEnableRtl: tomConstants = tomConstants(4096i32);
pub const tomMathEqAlign: tomConstants = tomConstants(128i32);
pub const tomMathInterSpace: tomConstants = tomConstants(135i32);
pub const tomMathIntraSpace: tomConstants = tomConstants(136i32);
pub const tomMathLMargin: tomConstants = tomConstants(129i32);
pub const tomMathManualBreakMask: tomConstants = tomConstants(127i32);
pub const tomMathObjShadingEnd: tomConstants = tomConstants(596i32);
pub const tomMathObjShadingStart: tomConstants = tomConstants(595i32);
pub const tomMathParaAlignCenter: tomConstants = tomConstants(2i32);
pub const tomMathParaAlignCenterGroup: tomConstants = tomConstants(1i32);
pub const tomMathParaAlignDefault: tomConstants = tomConstants(0i32);
pub const tomMathParaAlignLeft: tomConstants = tomConstants(3i32);
pub const tomMathParaAlignRight: tomConstants = tomConstants(4i32);
pub const tomMathPostSpace: tomConstants = tomConstants(134i32);
pub const tomMathPreSpace: tomConstants = tomConstants(133i32);
pub const tomMathRMargin: tomConstants = tomConstants(130i32);
pub const tomMathRelSize: tomConstants = tomConstants(64i32);
pub const tomMathVariant: tomConstants = tomConstants(32i32);
pub const tomMathWrapIndent: tomConstants = tomConstants(131i32);
pub const tomMathWrapRight: tomConstants = tomConstants(132i32);
pub const tomMathZone: tomConstants = tomConstants(-1879048192i32);
pub const tomMathZoneDisplay: tomConstants = tomConstants(262144i32);
pub const tomMathZoneNoBuildUp: tomConstants = tomConstants(-2013265920i32);
pub const tomMathZoneOrdinary: tomConstants = tomConstants(-1610612736i32);
pub const tomMatrix: OBJECTTYPE = OBJECTTYPE(20i32);
pub const tomMatrixAlignBottomRow: tomConstants = tomConstants(3i32);
pub const tomMatrixAlignCenter: tomConstants = tomConstants(0i32);
pub const tomMatrixAlignMask: tomConstants = tomConstants(3i32);
pub const tomMatrixAlignTopRow: tomConstants = tomConstants(1i32);
pub const tomModWidthPairs: tomConstants = tomConstants(1i32);
pub const tomModWidthSpace: tomConstants = tomConstants(2i32);
pub const tomMongolian: tomConstants = tomConstants(43i32);
pub const tomMove: tomConstants = tomConstants(0i32);
pub const tomMyanmar: tomConstants = tomConstants(34i32);
pub const tomNKo: tomConstants = tomConstants(57i32);
pub const tomNary: OBJECTTYPE = OBJECTTYPE(21i32);
pub const tomNewTaiLue: tomConstants = tomConstants(48i32);
pub const tomNoAnimation: tomConstants = tomConstants(0i32);
pub const tomNoBreak: tomConstants = tomConstants(128i32);
pub const tomNoHidden: tomConstants = tomConstants(32i32);
pub const tomNoIME: tomConstants = tomConstants(524288i32);
pub const tomNoLink: tomConstants = tomConstants(0i32);
pub const tomNoMathZoneBrackets: tomConstants = tomConstants(256i32);
pub const tomNoSelection: tomConstants = tomConstants(0i32);
pub const tomNoUCGreekItalic: tomConstants = tomConstants(1024i32);
pub const tomNoUpScroll: tomConstants = tomConstants(65536i32);
pub const tomNoVpScroll: tomConstants = tomConstants(262144i32);
pub const tomNone: tomConstants = tomConstants(0i32);
pub const tomNormalCaret: tomConstants = tomConstants(0i32);
pub const tomNullCaret: tomConstants = tomConstants(2i32);
pub const tomOEM: tomConstants = tomConstants(17i32);
pub const tomObject: tomConstants = tomConstants(16i32);
pub const tomObjectArg: tomConstants = tomConstants(2048i32);
pub const tomObjectMax: OBJECTTYPE = OBJECTTYPE(33i32);
pub const tomOgham: tomConstants = tomConstants(40i32);
pub const tomOpChar: OBJECTTYPE = OBJECTTYPE(22i32);
pub const tomOpenAlways: tomConstants = tomConstants(64i32);
pub const tomOpenExisting: tomConstants = tomConstants(48i32);
pub const tomOriya: tomConstants = tomConstants(26i32);
pub const tomOsmanya: tomConstants = tomConstants(58i32);
pub const tomOutline: tomConstants = tomConstants(-2147483136i32);
pub const tomOverbar: OBJECTTYPE = OBJECTTYPE(23i32);
pub const tomOverlapping: tomConstants = tomConstants(128i32);
pub const tomPC437: tomConstants = tomConstants(16i32);
pub const tomPage: tomConstants = tomConstants(17i32);
pub const tomParaEffectBox: tomConstants = tomConstants(1024i32);
pub const tomParaEffectCollapsed: tomConstants = tomConstants(256i32);
pub const tomParaEffectDoNotHyphen: tomConstants = tomConstants(64i32);
pub const tomParaEffectKeep: tomConstants = tomConstants(2i32);
pub const tomParaEffectKeepNext: tomConstants = tomConstants(4i32);
pub const tomParaEffectNoLineNumber: tomConstants = tomConstants(16i32);
pub const tomParaEffectNoWidowControl: tomConstants = tomConstants(32i32);
pub const tomParaEffectOutlineLevel: tomConstants = tomConstants(512i32);
pub const tomParaEffectPageBreakBefore: tomConstants = tomConstants(8i32);
pub const tomParaEffectRTL: tomConstants = tomConstants(1i32);
pub const tomParaEffectSideBySide: tomConstants = tomConstants(128i32);
pub const tomParaEffectTable: tomConstants = tomConstants(16384i32);
pub const tomParaEffectTableRowDelimiter: tomConstants = tomConstants(4096i32);
pub const tomParaFormat: tomConstants = tomConstants(14i32);
pub const tomParaPropMathAlign: tomConstants = tomConstants(1079i32);
pub const tomParaStyleHeading1: tomConstants = tomConstants(-2i32);
pub const tomParaStyleHeading2: tomConstants = tomConstants(-3i32);
pub const tomParaStyleHeading3: tomConstants = tomConstants(-4i32);
pub const tomParaStyleHeading4: tomConstants = tomConstants(-5i32);
pub const tomParaStyleHeading5: tomConstants = tomConstants(-6i32);
pub const tomParaStyleHeading6: tomConstants = tomConstants(-7i32);
pub const tomParaStyleHeading7: tomConstants = tomConstants(-8i32);
pub const tomParaStyleHeading8: tomConstants = tomConstants(-9i32);
pub const tomParaStyleHeading9: tomConstants = tomConstants(-10i32);
pub const tomParaStyleNormal: tomConstants = tomConstants(-1i32);
pub const tomParagraph: tomConstants = tomConstants(4i32);
pub const tomPasteFile: tomConstants = tomConstants(4096i32);
pub const tomPhagsPa: tomConstants = tomConstants(59i32);
pub const tomPhantom: OBJECTTYPE = OBJECTTYPE(24i32);
pub const tomPhantomASmash: tomConstants = tomConstants(5i32);
pub const tomPhantomDSmash: tomConstants = tomConstants(9i32);
pub const tomPhantomHSmash: tomConstants = tomConstants(3i32);
pub const tomPhantomHorz: tomConstants = tomConstants(12i32);
pub const tomPhantomShow: tomConstants = tomConstants(1i32);
pub const tomPhantomSmash: tomConstants = tomConstants(13i32);
pub const tomPhantomTransparent: tomConstants = tomConstants(16i32);
pub const tomPhantomVert: tomConstants = tomConstants(2i32);
pub const tomPhantomZeroAscent: tomConstants = tomConstants(4i32);
pub const tomPhantomZeroDescent: tomConstants = tomConstants(8i32);
pub const tomPhantomZeroWidth: tomConstants = tomConstants(2i32);
pub const tomPrimaryFooterStory: tomConstants = tomConstants(9i32);
pub const tomPrimaryHeaderStory: tomConstants = tomConstants(7i32);
pub const tomProcessId: tomConstants = tomConstants(1073741825i32);
pub const tomProtected: tomConstants = tomConstants(-2147483632i32);
pub const tomRE10Mode: tomConstants = tomConstants(1i32);
pub const tomRTF: tomConstants = tomConstants(1i32);
pub const tomRadical: OBJECTTYPE = OBJECTTYPE(25i32);
pub const tomReadOnly: tomConstants = tomConstants(256i32);
pub const tomReplaceStory: tomConstants = tomConstants(129i32);
pub const tomResume: tomConstants = tomConstants(-9999994i32);
pub const tomRevised: tomConstants = tomConstants(-2147467264i32);
pub const tomRow: tomConstants = tomConstants(10i32);
pub const tomRowApplyDefault: tomConstants = tomConstants(0i32);
pub const tomRowHeightActual: tomConstants = tomConstants(2059i32);
pub const tomRowUpdate: tomConstants = tomConstants(1i32);
pub const tomRuby: OBJECTTYPE = OBJECTTYPE(1i32);
pub const tomRubyAlign010: tomConstants = tomConstants(1i32);
pub const tomRubyAlign121: tomConstants = tomConstants(2i32);
pub const tomRubyAlignCenter: tomConstants = tomConstants(0i32);
pub const tomRubyAlignLeft: tomConstants = tomConstants(3i32);
pub const tomRubyAlignRight: tomConstants = tomConstants(4i32);
pub const tomRubyBelow: tomConstants = tomConstants(128i32);
pub const tomRunic: tomConstants = tomConstants(41i32);
pub const tomScratchStory: tomConstants = tomConstants(127i32);
pub const tomScreen: tomConstants = tomConstants(7i32);
pub const tomSection: tomConstants = tomConstants(8i32);
pub const tomSelActive: tomConstants = tomConstants(8i32);
pub const tomSelAtEOL: tomConstants = tomConstants(2i32);
pub const tomSelOvertype: tomConstants = tomConstants(4i32);
pub const tomSelRange: tomConstants = tomConstants(597i32);
pub const tomSelReplace: tomConstants = tomConstants(16i32);
pub const tomSelStartActive: tomConstants = tomConstants(1i32);
pub const tomSelectionBlock: tomConstants = tomConstants(6i32);
pub const tomSelectionColumn: tomConstants = tomConstants(4i32);
pub const tomSelectionFrame: tomConstants = tomConstants(3i32);
pub const tomSelectionIP: tomConstants = tomConstants(1i32);
pub const tomSelectionInlineShape: tomConstants = tomConstants(7i32);
pub const tomSelectionNormal: tomConstants = tomConstants(2i32);
pub const tomSelectionRow: tomConstants = tomConstants(5i32);
pub const tomSelectionShape: tomConstants = tomConstants(8i32);
pub const tomSelfIME: tomConstants = tomConstants(262144i32);
pub const tomSentence: tomConstants = tomConstants(3i32);
pub const tomSentenceCase: tomConstants = tomConstants(4i32);
pub const tomShadow: tomConstants = tomConstants(-2147482624i32);
pub const tomShareDenyRead: tomConstants = tomConstants(512i32);
pub const tomShareDenyWrite: tomConstants = tomConstants(1024i32);
pub const tomShiftJIS: tomConstants = tomConstants(12i32);
pub const tomShimmer: tomConstants = tomConstants(6i32);
pub const tomShowDegPlaceHldr: tomConstants = tomConstants(8i32);
pub const tomShowLLimPlaceHldr: tomConstants = tomConstants(8i32);
pub const tomShowMatPlaceHldr: tomConstants = tomConstants(8i32);
pub const tomShowULimPlaceHldr: tomConstants = tomConstants(16i32);
pub const tomSimpleText: OBJECTTYPE = OBJECTTYPE(0i32);
pub const tomSingle: tomConstants = tomConstants(1i32);
pub const tomSinhala: tomConstants = tomConstants(31i32);
pub const tomSizeScript: tomConstants = tomConstants(64i32);
pub const tomSizeScriptScript: tomConstants = tomConstants(96i32);
pub const tomSizeText: tomConstants = tomConstants(32i32);
pub const tomSlashedFraction: OBJECTTYPE = OBJECTTYPE(26i32);
pub const tomSmallCaps: tomConstants = tomConstants(-2147483584i32);
pub const tomSpaceBinary: tomConstants = tomConstants(8i32);
pub const tomSpaceDefault: tomConstants = tomConstants(0i32);
pub const tomSpaceDifferential: tomConstants = tomConstants(24i32);
pub const tomSpaceMask: tomConstants = tomConstants(28i32);
pub const tomSpaceOrd: tomConstants = tomConstants(20i32);
pub const tomSpaceRelational: tomConstants = tomConstants(12i32);
pub const tomSpaceSkip: tomConstants = tomConstants(16i32);
pub const tomSpaceUnary: tomConstants = tomConstants(4i32);
pub const tomSpaces: tomConstants = tomConstants(0i32);
pub const tomSparkleText: tomConstants = tomConstants(3i32);
pub const tomStack: OBJECTTYPE = OBJECTTYPE(27i32);
pub const tomStart: tomConstants = tomConstants(32i32);
pub const tomStory: tomConstants = tomConstants(6i32);
pub const tomStoryActiveDisplay: tomConstants = tomConstants(1i32);
pub const tomStoryActiveDisplayUI: tomConstants = tomConstants(3i32);
pub const tomStoryActiveUI: tomConstants = tomConstants(2i32);
pub const tomStoryInactive: tomConstants = tomConstants(0i32);
pub const tomStretchBaseAbove: tomConstants = tomConstants(3i32);
pub const tomStretchBaseBelow: tomConstants = tomConstants(2i32);
pub const tomStretchCharAbove: tomConstants = tomConstants(1i32);
pub const tomStretchCharBelow: tomConstants = tomConstants(0i32);
pub const tomStretchStack: OBJECTTYPE = OBJECTTYPE(28i32);
pub const tomStrikeout: tomConstants = tomConstants(-2147483640i32);
pub const tomStyleDefault: tomConstants = tomConstants(0i32);
pub const tomStyleDisplay: tomConstants = tomConstants(8i32);
pub const tomStyleDisplayCramped: tomConstants = tomConstants(7i32);
pub const tomStyleScript: tomConstants = tomConstants(4i32);
pub const tomStyleScriptCramped: tomConstants = tomConstants(3i32);
pub const tomStyleScriptScript: tomConstants = tomConstants(2i32);
pub const tomStyleScriptScriptCramped: tomConstants = tomConstants(1i32);
pub const tomStyleText: tomConstants = tomConstants(6i32);
pub const tomStyleTextCramped: tomConstants = tomConstants(5i32);
pub const tomSubSup: OBJECTTYPE = OBJECTTYPE(30i32);
pub const tomSubSupAlign: tomConstants = tomConstants(1i32);
pub const tomSubscript: OBJECTTYPE = OBJECTTYPE(29i32);
pub const tomSubscriptCF: tomConstants = tomConstants(-2147418112i32);
pub const tomSuperscript: OBJECTTYPE = OBJECTTYPE(31i32);
pub const tomSuperscriptCF: tomConstants = tomConstants(-2147352576i32);
pub const tomSuspend: tomConstants = tomConstants(-9999995i32);
pub const tomSylotiNagri: tomConstants = tomConstants(49i32);
pub const tomSymbol: tomConstants = tomConstants(10i32);
pub const tomSyriac: tomConstants = tomConstants(20i32);
pub const tomTabBack: tomConstants = tomConstants(-3i32);
pub const tomTabHere: tomConstants = tomConstants(-1i32);
pub const tomTabNext: tomConstants = tomConstants(-2i32);
pub const tomTable: tomConstants = tomConstants(15i32);
pub const tomTableColumn: tomConstants = tomConstants(9i32);
pub const tomTaiLe: tomConstants = tomConstants(47i32);
pub const tomTamil: tomConstants = tomConstants(27i32);
pub const tomTelugu: tomConstants = tomConstants(28i32);
pub const tomText: tomConstants = tomConstants(2i32);
pub const tomTextFlowES: tomConstants = tomConstants(0i32);
pub const tomTextFlowMask: tomConstants = tomConstants(12i32);
pub const tomTextFlowNE: tomConstants = tomConstants(12i32);
pub const tomTextFlowSW: tomConstants = tomConstants(4i32);
pub const tomTextFlowWN: tomConstants = tomConstants(8i32);
pub const tomTextFrameStory: tomConstants = tomConstants(5i32);
pub const tomTextize: tomConstants = tomConstants(4i32);
pub const tomThaana: tomConstants = tomConstants(21i32);
pub const tomThai: tomConstants = tomConstants(11i32);
pub const tomThick: tomConstants = tomConstants(9i32);
pub const tomThickDash: tomConstants = tomConstants(14i32);
pub const tomThickDashDot: tomConstants = tomConstants(15i32);
pub const tomThickDashDotDot: tomConstants = tomConstants(16i32);
pub const tomThickDotted: tomConstants = tomConstants(17i32);
pub const tomThickLines: tomConstants = tomConstants(4i32);
pub const tomThickLongDash: tomConstants = tomConstants(18i32);
pub const tomTibetan: tomConstants = tomConstants(33i32);
pub const tomTifinagh: tomConstants = tomConstants(62i32);
pub const tomTitleCase: tomConstants = tomConstants(2i32);
pub const tomToggle: tomConstants = tomConstants(-9999998i32);
pub const tomToggleCase: tomConstants = tomConstants(5i32);
pub const tomTrackParms: tomConstants = tomConstants(2i32);
pub const tomTransform: tomConstants = tomConstants(1024i32);
pub const tomTranslateTableCell: tomConstants = tomConstants(128i32);
pub const tomTransparentForPositioning: tomConstants = tomConstants(256i32);
pub const tomTransparentForSpacing: tomConstants = tomConstants(512i32);
pub const tomTrue: tomConstants = tomConstants(-1i32);
pub const tomTruncateExisting: tomConstants = tomConstants(80i32);
pub const tomTurkish: tomConstants = tomConstants(4i32);
pub const tomUndefined: tomConstants = tomConstants(-9999999i32);
pub const tomUnderbar: OBJECTTYPE = OBJECTTYPE(32i32);
pub const tomUnderline: tomConstants = tomConstants(-2147483644i32);
pub const tomUnderlinePositionAbove: tomConstants = tomConstants(2i32);
pub const tomUnderlinePositionAuto: tomConstants = tomConstants(0i32);
pub const tomUnderlinePositionBelow: tomConstants = tomConstants(1i32);
pub const tomUnderlinePositionMax: tomConstants = tomConstants(2i32);
pub const tomUndoLimit: tomConstants = tomConstants(140i32);
pub const tomUnhide: tomConstants = tomConstants(16i32);
pub const tomUnicodeBiDi: tomConstants = tomConstants(1i32);
pub const tomUnknownStory: tomConstants = tomConstants(0i32);
pub const tomUnlink: tomConstants = tomConstants(8i32);
pub const tomUpperCase: tomConstants = tomConstants(1i32);
pub const tomUpperLimit: OBJECTTYPE = OBJECTTYPE(33i32);
pub const tomUpperLimitAsSuperScript: tomConstants = tomConstants(3i32);
pub const tomUseAtFont: tomConstants = tomConstants(2i32);
pub const tomUseCRLF: tomConstants = tomConstants(2i32);
pub const tomUsePoints: tomConstants = tomConstants(10i32);
pub const tomUseTwips: tomConstants = tomConstants(11i32);
pub const tomUsymbol: tomConstants = tomConstants(52i32);
pub const tomVLowCell: tomConstants = tomConstants(2i32);
pub const tomVTopCell: tomConstants = tomConstants(1i32);
pub const tomVai: tomConstants = tomConstants(56i32);
pub const tomVietnamese: tomConstants = tomConstants(8i32);
pub const tomWarichu: OBJECTTYPE = OBJECTTYPE(3i32);
pub const tomWave: tomConstants = tomConstants(8i32);
pub const tomWindow: tomConstants = tomConstants(11i32);
pub const tomWipeDown: tomConstants = tomConstants(7i32);
pub const tomWipeRight: tomConstants = tomConstants(8i32);
pub const tomWord: tomConstants = tomConstants(2i32);
pub const tomWordDocument: tomConstants = tomConstants(4i32);
pub const tomWords: tomConstants = tomConstants(2i32);
pub const tomYi: tomConstants = tomConstants(45i32);
pub const yHeightCharPtsMost: u32 = 1638u32;
