#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
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
#[cfg(feature = "Win32_Foundation")]
pub type AutoCorrectProc = unsafe extern "system" fn(langid: u16, pszbefore: super::super::super::Foundation::PWSTR, pszafter: super::super::super::Foundation::PWSTR, cchafter: i32, pcchreplaced: *mut i32) -> i32;
#[repr(C)]
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
#[repr(transparent)]
pub struct CARET_FLAGS(pub i32);
pub const CARET_NONE: CARET_FLAGS = CARET_FLAGS(0i32);
pub const CARET_CUSTOM: CARET_FLAGS = CARET_FLAGS(1i32);
pub const CARET_RTL: CARET_FLAGS = CARET_FLAGS(2i32);
pub const CARET_ITALIC: CARET_FLAGS = CARET_FLAGS(32i32);
pub const CARET_NULL: CARET_FLAGS = CARET_FLAGS(64i32);
pub const CARET_ROTATE90: CARET_FLAGS = CARET_FLAGS(128i32);
impl ::core::marker::Copy for CARET_FLAGS {}
impl ::core::clone::Clone for CARET_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for CFE_EFFECTS {}
impl ::core::clone::Clone for CFE_EFFECTS {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for CFM_MASK {}
impl ::core::clone::Clone for CFM_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(transparent)]
pub struct CHANGETYPE(pub i32);
pub const CN_GENERIC: CHANGETYPE = CHANGETYPE(0i32);
pub const CN_TEXTCHANGED: CHANGETYPE = CHANGETYPE(1i32);
pub const CN_NEWUNDO: CHANGETYPE = CHANGETYPE(2i32);
pub const CN_NEWREDO: CHANGETYPE = CHANGETYPE(4i32);
impl ::core::marker::Copy for CHANGETYPE {}
impl ::core::clone::Clone for CHANGETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C, packed(4))]
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
#[repr(C)]
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
#[repr(C, packed(4))]
pub struct EDITSTREAM {
    pub dwCookie: usize,
    pub dwError: u32,
    pub pfnCallback: ::core::option::Option<EDITSTREAMCALLBACK>,
}
impl ::core::marker::Copy for EDITSTREAM {}
impl ::core::clone::Clone for EDITSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EDITSTREAMCALLBACK = unsafe extern "system" fn(dwcookie: usize, pbbuff: *mut u8, cb: i32, pcb: *mut i32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type EDITWORDBREAKPROCEX = unsafe extern "system" fn(pchtext: super::super::super::Foundation::PSTR, cchtext: i32, bcharset: u8, action: i32) -> i32;
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
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
#[repr(transparent)]
pub struct ENDCOMPOSITIONNOTIFY_CODE(pub u32);
pub const ECN_ENDCOMPOSITION: ENDCOMPOSITIONNOTIFY_CODE = ENDCOMPOSITIONNOTIFY_CODE(1u32);
pub const ECN_NEWTEXT: ENDCOMPOSITIONNOTIFY_CODE = ENDCOMPOSITIONNOTIFY_CODE(2u32);
impl ::core::marker::Copy for ENDCOMPOSITIONNOTIFY_CODE {}
impl ::core::clone::Clone for ENDCOMPOSITIONNOTIFY_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
pub struct ENOLEOPFAILED {
    pub nmhdr: super::NMHDR,
    pub iob: i32,
    pub lOper: i32,
    pub hr: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENOLEOPFAILED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENOLEOPFAILED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
pub const FR_MATCHALEFHAMZA: u32 = 2147483648u32;
pub const FR_MATCHDIAC: u32 = 536870912u32;
pub const FR_MATCHKASHIDA: u32 = 1073741824u32;
pub const GCMF_GRIPPER: u32 = 1u32;
pub const GCMF_MOUSEMENU: u32 = 8192u32;
pub const GCMF_SPELLING: u32 = 2u32;
pub const GCMF_TOUCHMENU: u32 = 16384u32;
pub const GCM_MOUSEMENU: u32 = 8192u32;
pub const GCM_TOUCHMENU: u32 = 16384u32;
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
#[repr(transparent)]
pub struct GETTEXTEX_FLAGS(pub u32);
pub const GT_DEFAULT: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(0u32);
pub const GT_NOHIDDENTEXT: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(8u32);
pub const GT_RAWTEXT: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(4u32);
pub const GT_SELECTION: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(2u32);
pub const GT_USECRLF: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(1u32);
impl ::core::marker::Copy for GETTEXTEX_FLAGS {}
impl ::core::clone::Clone for GETTEXTEX_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[repr(transparent)]
pub struct GETTEXTLENGTHEX_FLAGS(pub u32);
pub const GTL_DEFAULT: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(0u32);
pub const GTL_USECRLF: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(1u32);
pub const GTL_PRECISE: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(2u32);
pub const GTL_CLOSE: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(4u32);
pub const GTL_NUMCHARS: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(8u32);
pub const GTL_NUMBYTES: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(16u32);
impl ::core::marker::Copy for GETTEXTLENGTHEX_FLAGS {}
impl ::core::clone::Clone for GETTEXTLENGTHEX_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
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
pub const ICM_CTF: u32 = 5u32;
pub const ICM_LEVEL2: u32 = 2u32;
pub const ICM_LEVEL2_5: u32 = 3u32;
pub const ICM_LEVEL2_SUI: u32 = 4u32;
pub const ICM_LEVEL3: u32 = 1u32;
pub const ICM_NOTOPEN: u32 = 0u32;
#[repr(C)]
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
#[repr(transparent)]
pub struct IMECOMPTEXT_FLAGS(pub u32);
pub const ICT_RESULTREADSTR: IMECOMPTEXT_FLAGS = IMECOMPTEXT_FLAGS(1u32);
impl ::core::marker::Copy for IMECOMPTEXT_FLAGS {}
impl ::core::clone::Clone for IMECOMPTEXT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct IRichEditOle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditOleCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRicheditUiaOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextDisplays(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextDocument2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextDocument2Old(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextFont(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextFont2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextHost2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextPara(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextPara2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextRange2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextRow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextSelection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextSelection2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextServices2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextStory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextStoryRanges(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextStoryRanges2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextStrings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KHYPH(pub i32);
pub const khyphNil: KHYPH = KHYPH(0i32);
pub const khyphNormal: KHYPH = KHYPH(1i32);
pub const khyphAddBefore: KHYPH = KHYPH(2i32);
pub const khyphChangeBefore: KHYPH = KHYPH(3i32);
pub const khyphDeleteBefore: KHYPH = KHYPH(4i32);
pub const khyphChangeAfter: KHYPH = KHYPH(5i32);
pub const khyphDelAndChange: KHYPH = KHYPH(6i32);
impl ::core::marker::Copy for KHYPH {}
impl ::core::clone::Clone for KHYPH {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for MANCODE {}
impl ::core::clone::Clone for MANCODE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MAX_TABLE_CELLS: u32 = 63u32;
pub const MAX_TAB_STOPS: u32 = 32u32;
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
impl ::core::marker::Copy for OBJECTTYPE {}
impl ::core::clone::Clone for OBJECTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const OLEOP_DOVERB: u32 = 1u32;
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(transparent)]
pub struct PARAFORMAT_ALIGNMENT(pub u16);
pub const PFA_CENTER: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(3u16);
pub const PFA_LEFT: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(1u16);
pub const PFA_RIGHT: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(2u16);
impl ::core::marker::Copy for PARAFORMAT_ALIGNMENT {}
impl ::core::clone::Clone for PARAFORMAT_ALIGNMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PARAFORMAT_BORDERS(pub u16);
pub const PARAFORMAT_BORDERS_LEFT: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(1u16);
pub const PARAFORMAT_BORDERS_RIGHT: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(2u16);
pub const PARAFORMAT_BORDERS_TOP: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(4u16);
pub const PARAFORMAT_BORDERS_BOTTOM: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(8u16);
pub const PARAFORMAT_BORDERS_INSIDE: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(16u16);
pub const PARAFORMAT_BORDERS_OUTSIDE: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(32u16);
pub const PARAFORMAT_BORDERS_AUTOCOLOR: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(64u16);
impl ::core::marker::Copy for PARAFORMAT_BORDERS {}
impl ::core::clone::Clone for PARAFORMAT_BORDERS {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PARAFORMAT_MASK {}
impl ::core::clone::Clone for PARAFORMAT_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PARAFORMAT_NUMBERING_STYLE(pub u16);
pub const PFNS_PAREN: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(0u16);
pub const PFNS_PARENS: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(256u16);
pub const PFNS_PERIOD: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(512u16);
pub const PFNS_PLAIN: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(768u16);
pub const PFNS_NONUMBER: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(1024u16);
pub const PFNS_NEWNUMBER: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(32768u16);
impl ::core::marker::Copy for PARAFORMAT_NUMBERING_STYLE {}
impl ::core::clone::Clone for PARAFORMAT_NUMBERING_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PARAFORMAT_SHADING_STYLE {}
impl ::core::clone::Clone for PARAFORMAT_SHADING_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PC_DELIMITER: u32 = 4u32;
pub const PC_FOLLOWING: u32 = 1u32;
pub const PC_LEADING: u32 = 2u32;
pub const PC_OVERFLOW: u32 = 3u32;
pub type PCreateTextServices = unsafe extern "system" fn(punkouter: ::windows_sys::core::IUnknown, pitexthost: ITextHost, ppunk: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
pub const PFA_FULL_GLYPHS: u32 = 8u32;
pub const PFA_FULL_INTERLETTER: u32 = 6u32;
pub const PFA_FULL_INTERWORD: u32 = 4u32;
pub const PFA_FULL_NEWSPAPER: u32 = 5u32;
pub const PFA_FULL_SCALED: u32 = 7u32;
pub const PFA_JUSTIFY: u32 = 4u32;
pub const PFM_BORDER: u32 = 2048u32;
pub const PFM_BOX: u32 = 67108864u32;
pub const PFM_COLLAPSED: u32 = 16777216u32;
pub const PFM_DONOTHYPHEN: u32 = 4194304u32;
pub const PFM_KEEP: u32 = 131072u32;
pub const PFM_KEEPNEXT: u32 = 262144u32;
pub const PFM_LINESPACING: u32 = 256u32;
pub const PFM_NOLINENUMBER: u32 = 1048576u32;
pub const PFM_NOWIDOWCONTROL: u32 = 2097152u32;
pub const PFM_NUMBERINGSTART: u32 = 32768u32;
pub const PFM_NUMBERINGSTYLE: u32 = 8192u32;
pub const PFM_NUMBERINGTAB: u32 = 16384u32;
pub const PFM_OUTLINELEVEL: u32 = 33554432u32;
pub const PFM_PAGEBREAKBEFORE: u32 = 524288u32;
pub const PFM_RESERVED2: u32 = 134217728u32;
pub const PFM_SHADING: u32 = 4096u32;
pub const PFM_SIDEBYSIDE: u32 = 8388608u32;
pub const PFM_SPACEAFTER: u32 = 128u32;
pub const PFM_SPACEBEFORE: u32 = 64u32;
pub const PFM_STYLE: u32 = 1024u32;
pub const PFM_TABLE: u32 = 1073741824u32;
pub const PFM_TABLEROWDELIMITER: u32 = 268435456u32;
pub const PFM_TEXTWRAPPINGBREAK: u32 = 536870912u32;
pub const PFN_ARABIC: u32 = 2u32;
pub const PFN_BULLET: u32 = 1u32;
pub const PFN_LCLETTER: u32 = 3u32;
pub const PFN_LCROMAN: u32 = 5u32;
pub const PFN_UCLETTER: u32 = 4u32;
pub const PFN_UCROMAN: u32 = 6u32;
pub type PShutdownTextServices = unsafe extern "system" fn(ptextservices: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
#[repr(C, packed(4))]
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
pub const RECO_COPY: i32 = 2i32;
pub const RECO_CUT: i32 = 3i32;
pub const RECO_DRAG: i32 = 4i32;
pub const RECO_DROP: i32 = 1i32;
pub const RECO_PASTE: i32 = 0i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub struct REOBJECT {
    pub cbStruct: u32,
    pub cp: i32,
    pub clsid: ::windows_sys::core::GUID,
    pub poleobj: ::core::option::Option<super::super::super::System::Ole::IOleObject>,
    pub pstg: ::core::option::Option<super::super::super::System::Com::StructuredStorage::IStorage>,
    pub polesite: ::core::option::Option<super::super::super::System::Ole::IOleClientSite>,
    pub sizel: super::super::super::Foundation::SIZE,
    pub dvaspect: u32,
    pub dwFlags: REOBJECT_FLAGS,
    pub dwUser: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for REOBJECT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for REOBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for REOBJECT_FLAGS {}
impl ::core::clone::Clone for REOBJECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const REO_NULL: i32 = 0i32;
pub const REO_READWRITEMASK: i32 = 2047i32;
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
impl ::core::marker::Copy for RICHEDIT_IMAGE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for RICHEDIT_IMAGE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(pub u16);
pub const SEL_EMPTY: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(0u16);
pub const SEL_TEXT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(1u16);
pub const SEL_OBJECT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(2u16);
pub const SEL_MULTICHAR: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(4u16);
pub const SEL_MULTIOBJECT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(8u16);
pub const GCM_RIGHTMOUSEDROP: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(32768u16);
impl ::core::marker::Copy for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {}
impl ::core::clone::Clone for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RICH_EDIT_GET_OBJECT_FLAGS(pub u32);
pub const REO_GETOBJ_POLEOBJ: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(1u32);
pub const REO_GETOBJ_PSTG: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(2u32);
pub const REO_GETOBJ_POLESITE: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(4u32);
pub const REO_GETOBJ_NO_INTERFACES: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(0u32);
pub const REO_GETOBJ_ALL_INTERFACES: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(7u32);
impl ::core::marker::Copy for RICH_EDIT_GET_OBJECT_FLAGS {}
impl ::core::clone::Clone for RICH_EDIT_GET_OBJECT_FLAGS {
    fn clone(&self) -> Self {
        *self
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
#[repr(C, packed(4))]
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
pub const S_MSG_KEY_IGNORED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262657i32 as _);
#[repr(C)]
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
#[repr(C)]
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
#[repr(transparent)]
pub struct TEXTMODE(pub i32);
pub const TM_PLAINTEXT: TEXTMODE = TEXTMODE(1i32);
pub const TM_RICHTEXT: TEXTMODE = TEXTMODE(2i32);
pub const TM_SINGLELEVELUNDO: TEXTMODE = TEXTMODE(4i32);
pub const TM_MULTILEVELUNDO: TEXTMODE = TEXTMODE(8i32);
pub const TM_SINGLECODEPAGE: TEXTMODE = TEXTMODE(16i32);
pub const TM_MULTICODEPAGE: TEXTMODE = TEXTMODE(32i32);
impl ::core::marker::Copy for TEXTMODE {}
impl ::core::clone::Clone for TEXTMODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
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
#[repr(C, packed(4))]
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
pub const TO_ADVANCEDLAYOUT: u32 = 8u32;
pub const TO_ADVANCEDTYPOGRAPHY: u32 = 1u32;
pub const TO_DISABLECUSTOMTEXTOUT: u32 = 4u32;
pub const TO_SIMPLELINEBREAK: u32 = 2u32;
pub const TXES_ISDIALOG: u32 = 1u32;
#[repr(transparent)]
pub struct TXTBACKSTYLE(pub i32);
pub const TXTBACK_TRANSPARENT: TXTBACKSTYLE = TXTBACKSTYLE(0i32);
pub const TXTBACK_OPAQUE: TXTBACKSTYLE = TXTBACKSTYLE(1i32);
impl ::core::marker::Copy for TXTBACKSTYLE {}
impl ::core::clone::Clone for TXTBACKSTYLE {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub struct TXTHITRESULT(pub i32);
pub const TXTHITRESULT_NOHIT: TXTHITRESULT = TXTHITRESULT(0i32);
pub const TXTHITRESULT_TRANSPARENT: TXTHITRESULT = TXTHITRESULT(1i32);
pub const TXTHITRESULT_CLOSE: TXTHITRESULT = TXTHITRESULT(2i32);
pub const TXTHITRESULT_HIT: TXTHITRESULT = TXTHITRESULT(3i32);
impl ::core::marker::Copy for TXTHITRESULT {}
impl ::core::clone::Clone for TXTHITRESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TXTNATURALSIZE(pub i32);
pub const TXTNS_FITTOCONTENT2: TXTNATURALSIZE = TXTNATURALSIZE(0i32);
pub const TXTNS_FITTOCONTENT: TXTNATURALSIZE = TXTNATURALSIZE(1i32);
pub const TXTNS_ROUNDTOLINE: TXTNATURALSIZE = TXTNATURALSIZE(2i32);
pub const TXTNS_FITTOCONTENT3: TXTNATURALSIZE = TXTNATURALSIZE(3i32);
pub const TXTNS_FITTOCONTENTWSP: TXTNATURALSIZE = TXTNATURALSIZE(4i32);
pub const TXTNS_INCLUDELASTLINE: TXTNATURALSIZE = TXTNATURALSIZE(1073741824i32);
pub const TXTNS_EMU: TXTNATURALSIZE = TXTNATURALSIZE(-2147483648i32);
impl ::core::marker::Copy for TXTNATURALSIZE {}
impl ::core::clone::Clone for TXTNATURALSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TXTVIEW(pub i32);
pub const TXTVIEW_ACTIVE: TXTVIEW = TXTVIEW(0i32);
pub const TXTVIEW_INACTIVE: TXTVIEW = TXTVIEW(-1i32);
impl ::core::marker::Copy for TXTVIEW {}
impl ::core::clone::Clone for TXTVIEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UNDONAMEID(pub i32);
pub const UID_UNKNOWN: UNDONAMEID = UNDONAMEID(0i32);
pub const UID_TYPING: UNDONAMEID = UNDONAMEID(1i32);
pub const UID_DELETE: UNDONAMEID = UNDONAMEID(2i32);
pub const UID_DRAGDROP: UNDONAMEID = UNDONAMEID(3i32);
pub const UID_CUT: UNDONAMEID = UNDONAMEID(4i32);
pub const UID_PASTE: UNDONAMEID = UNDONAMEID(5i32);
pub const UID_AUTOTABLE: UNDONAMEID = UNDONAMEID(6i32);
impl ::core::marker::Copy for UNDONAMEID {}
impl ::core::clone::Clone for UNDONAMEID {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub const WM_CONTEXTMENU: u32 = 123u32;
pub const WM_NOTIFY: u32 = 78u32;
pub const WM_PRINTCLIENT: u32 = 792u32;
pub const WM_UNICHAR: u32 = 265u32;
#[repr(C, packed(4))]
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
#[repr(C)]
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
impl ::core::marker::Copy for tomConstants {}
impl ::core::clone::Clone for tomConstants {
    fn clone(&self) -> Self {
        *self
    }
}
