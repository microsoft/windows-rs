#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
pub const CARET_NONE: i32 = 0i32;
pub const CARET_CUSTOM: i32 = 1i32;
pub const CARET_RTL: i32 = 2i32;
pub const CARET_ITALIC: i32 = 32i32;
pub const CARET_NULL: i32 = 64i32;
pub const CARET_ROTATE90: i32 = 128i32;
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
pub const CFE_ALLCAPS: u32 = 128u32;
pub const CFE_AUTOBACKCOLOR: u32 = 67108864u32;
pub const CFE_DISABLED: u32 = 8192u32;
pub const CFE_EMBOSS: u32 = 2048u32;
pub const CFE_HIDDEN: u32 = 256u32;
pub const CFE_IMPRINT: u32 = 4096u32;
pub const CFE_OUTLINE: u32 = 512u32;
pub const CFE_REVISED: u32 = 16384u32;
pub const CFE_SHADOW: u32 = 1024u32;
pub const CFE_SMALLCAPS: u32 = 64u32;
pub const CFE_AUTOCOLOR: u32 = 1073741824u32;
pub const CFE_BOLD: u32 = 1u32;
pub const CFE_ITALIC: u32 = 2u32;
pub const CFE_STRIKEOUT: u32 = 8u32;
pub const CFE_UNDERLINE: u32 = 4u32;
pub const CFE_PROTECTED: u32 = 16u32;
pub const CFE_LINK: u32 = 32u32;
pub const CFE_SUBSCRIPT: u32 = 65536u32;
pub const CFE_SUPERSCRIPT: u32 = 131072u32;
pub const CFE_FONTBOUND: u32 = 1048576u32;
pub const CFE_LINKPROTECTED: u32 = 8388608u32;
pub const CFE_EXTENDED: u32 = 33554432u32;
pub const CFE_MATHNOBUILDUP: u32 = 134217728u32;
pub const CFE_MATH: u32 = 268435456u32;
pub const CFE_MATHORDINARY: u32 = 536870912u32;
pub const CFM_SUBSCRIPT: u32 = 196608u32;
pub const CFM_SUPERSCRIPT: u32 = 196608u32;
pub const CFM_EFFECTS: u32 = 1073741887u32;
pub const CFM_ALL: u32 = 4160749631u32;
pub const CFM_BOLD: u32 = 1u32;
pub const CFM_CHARSET: u32 = 134217728u32;
pub const CFM_COLOR: u32 = 1073741824u32;
pub const CFM_FACE: u32 = 536870912u32;
pub const CFM_ITALIC: u32 = 2u32;
pub const CFM_OFFSET: u32 = 268435456u32;
pub const CFM_PROTECTED: u32 = 16u32;
pub const CFM_SIZE: u32 = 2147483648u32;
pub const CFM_STRIKEOUT: u32 = 8u32;
pub const CFM_UNDERLINE: u32 = 4u32;
pub const CFM_LINK: u32 = 32u32;
pub const CFM_SMALLCAPS: u32 = 64u32;
pub const CFM_ALLCAPS: u32 = 128u32;
pub const CFM_HIDDEN: u32 = 256u32;
pub const CFM_OUTLINE: u32 = 512u32;
pub const CFM_SHADOW: u32 = 1024u32;
pub const CFM_EMBOSS: u32 = 2048u32;
pub const CFM_IMPRINT: u32 = 4096u32;
pub const CFM_DISABLED: u32 = 8192u32;
pub const CFM_REVISED: u32 = 16384u32;
pub const CFM_REVAUTHOR: u32 = 32768u32;
pub const CFM_ANIMATION: u32 = 262144u32;
pub const CFM_STYLE: u32 = 524288u32;
pub const CFM_KERNING: u32 = 1048576u32;
pub const CFM_SPACING: u32 = 2097152u32;
pub const CFM_WEIGHT: u32 = 4194304u32;
pub const CFM_UNDERLINETYPE: u32 = 8388608u32;
pub const CFM_COOKIE: u32 = 16777216u32;
pub const CFM_LCID: u32 = 33554432u32;
pub const CFM_BACKCOLOR: u32 = 67108864u32;
pub const CFM_EFFECTS2: u32 = 1141080063u32;
pub const CFM_ALL2: u32 = 4294967295u32;
pub const CFM_FONTBOUND: u32 = 1048576u32;
pub const CFM_LINKPROTECTED: u32 = 8388608u32;
pub const CFM_EXTENDED: u32 = 33554432u32;
pub const CFM_MATHNOBUILDUP: u32 = 134217728u32;
pub const CFM_MATH: u32 = 268435456u32;
pub const CFM_MATHORDINARY: u32 = 536870912u32;
pub const CFM_ALLEFFECTS: u32 = 2115207167u32;
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
pub const CN_GENERIC: i32 = 0i32;
pub const CN_TEXTCHANGED: i32 = 1i32;
pub const CN_NEWUNDO: i32 = 2i32;
pub const CN_NEWREDO: i32 = 4i32;
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
    pub pfnCallback: EDITSTREAMCALLBACK,
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
pub const ECN_ENDCOMPOSITION: u32 = 1u32;
pub const ECN_NEWTEXT: u32 = 2u32;
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
pub const GT_DEFAULT: u32 = 0u32;
pub const GT_NOHIDDENTEXT: u32 = 8u32;
pub const GT_RAWTEXT: u32 = 4u32;
pub const GT_SELECTION: u32 = 2u32;
pub const GT_USECRLF: u32 = 1u32;
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
pub const GTL_DEFAULT: u32 = 0u32;
pub const GTL_USECRLF: u32 = 1u32;
pub const GTL_PRECISE: u32 = 2u32;
pub const GTL_CLOSE: u32 = 4u32;
pub const GTL_NUMCHARS: u32 = 8u32;
pub const GTL_NUMBYTES: u32 = 16u32;
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
pub const ICT_RESULTREADSTR: u32 = 1u32;
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
impl ::core::marker::Copy for IRichEditOle {}
impl ::core::clone::Clone for IRichEditOle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditOleCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditOleCallback {}
impl ::core::clone::Clone for IRichEditOleCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRicheditUiaOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRicheditUiaOverrides {}
impl ::core::clone::Clone for IRicheditUiaOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextDisplays(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextDisplays {}
impl ::core::clone::Clone for ITextDisplays {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextDocument(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextDocument {}
impl ::core::clone::Clone for ITextDocument {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextDocument2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextDocument2 {}
impl ::core::clone::Clone for ITextDocument2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextDocument2Old(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextDocument2Old {}
impl ::core::clone::Clone for ITextDocument2Old {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextFont(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextFont {}
impl ::core::clone::Clone for ITextFont {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextFont2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextFont2 {}
impl ::core::clone::Clone for ITextFont2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextHost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextHost {}
impl ::core::clone::Clone for ITextHost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextHost2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextHost2 {}
impl ::core::clone::Clone for ITextHost2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextPara(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextPara {}
impl ::core::clone::Clone for ITextPara {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextPara2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextPara2 {}
impl ::core::clone::Clone for ITextPara2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextRange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextRange {}
impl ::core::clone::Clone for ITextRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextRange2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextRange2 {}
impl ::core::clone::Clone for ITextRange2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextRow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextRow {}
impl ::core::clone::Clone for ITextRow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextSelection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextSelection {}
impl ::core::clone::Clone for ITextSelection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextSelection2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextSelection2 {}
impl ::core::clone::Clone for ITextSelection2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextServices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextServices {}
impl ::core::clone::Clone for ITextServices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextServices2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextServices2 {}
impl ::core::clone::Clone for ITextServices2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextStory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextStory {}
impl ::core::clone::Clone for ITextStory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextStoryRanges(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextStoryRanges {}
impl ::core::clone::Clone for ITextStoryRanges {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextStoryRanges2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextStoryRanges2 {}
impl ::core::clone::Clone for ITextStoryRanges2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextStrings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextStrings {}
impl ::core::clone::Clone for ITextStrings {
    fn clone(&self) -> Self {
        *self
    }
}
pub const khyphNil: i32 = 0i32;
pub const khyphNormal: i32 = 1i32;
pub const khyphAddBefore: i32 = 2i32;
pub const khyphChangeBefore: i32 = 3i32;
pub const khyphDeleteBefore: i32 = 4i32;
pub const khyphChangeAfter: i32 = 5i32;
pub const khyphDelAndChange: i32 = 6i32;
pub const MBOLD: i32 = 16i32;
pub const MITAL: i32 = 32i32;
pub const MGREEK: i32 = 64i32;
pub const MROMN: i32 = 0i32;
pub const MSCRP: i32 = 1i32;
pub const MFRAK: i32 = 2i32;
pub const MOPEN: i32 = 3i32;
pub const MSANS: i32 = 4i32;
pub const MMONO: i32 = 5i32;
pub const MMATH: i32 = 6i32;
pub const MISOL: i32 = 7i32;
pub const MINIT: i32 = 8i32;
pub const MTAIL: i32 = 9i32;
pub const MSTRCH: i32 = 10i32;
pub const MLOOP: i32 = 11i32;
pub const MOPENA: i32 = 12i32;
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
pub const tomSimpleText: i32 = 0i32;
pub const tomRuby: i32 = 1i32;
pub const tomHorzVert: i32 = 2i32;
pub const tomWarichu: i32 = 3i32;
pub const tomEq: i32 = 9i32;
pub const tomMath: i32 = 10i32;
pub const tomAccent: i32 = 10i32;
pub const tomBox: i32 = 11i32;
pub const tomBoxedFormula: i32 = 12i32;
pub const tomBrackets: i32 = 13i32;
pub const tomBracketsWithSeps: i32 = 14i32;
pub const tomEquationArray: i32 = 15i32;
pub const tomFraction: i32 = 16i32;
pub const tomFunctionApply: i32 = 17i32;
pub const tomLeftSubSup: i32 = 18i32;
pub const tomLowerLimit: i32 = 19i32;
pub const tomMatrix: i32 = 20i32;
pub const tomNary: i32 = 21i32;
pub const tomOpChar: i32 = 22i32;
pub const tomOverbar: i32 = 23i32;
pub const tomPhantom: i32 = 24i32;
pub const tomRadical: i32 = 25i32;
pub const tomSlashedFraction: i32 = 26i32;
pub const tomStack: i32 = 27i32;
pub const tomStretchStack: i32 = 28i32;
pub const tomSubscript: i32 = 29i32;
pub const tomSubSup: i32 = 30i32;
pub const tomSuperscript: i32 = 31i32;
pub const tomUnderbar: i32 = 32i32;
pub const tomUpperLimit: i32 = 33i32;
pub const tomObjectMax: i32 = 33i32;
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
pub const PFA_CENTER: u16 = 3u16;
pub const PFA_LEFT: u16 = 1u16;
pub const PFA_RIGHT: u16 = 2u16;
pub const PARAFORMAT_BORDERS_LEFT: u16 = 1u16;
pub const PARAFORMAT_BORDERS_RIGHT: u16 = 2u16;
pub const PARAFORMAT_BORDERS_TOP: u16 = 4u16;
pub const PARAFORMAT_BORDERS_BOTTOM: u16 = 8u16;
pub const PARAFORMAT_BORDERS_INSIDE: u16 = 16u16;
pub const PARAFORMAT_BORDERS_OUTSIDE: u16 = 32u16;
pub const PARAFORMAT_BORDERS_AUTOCOLOR: u16 = 64u16;
pub const PFM_ALIGNMENT: u32 = 8u32;
pub const PFM_NUMBERING: u32 = 32u32;
pub const PFM_OFFSET: u32 = 4u32;
pub const PFM_OFFSETINDENT: u32 = 2147483648u32;
pub const PFM_RIGHTINDENT: u32 = 2u32;
pub const PFM_RTLPARA: u32 = 65536u32;
pub const PFM_STARTINDENT: u32 = 1u32;
pub const PFM_TABSTOPS: u32 = 16u32;
pub const PFNS_PAREN: u16 = 0u16;
pub const PFNS_PARENS: u16 = 256u16;
pub const PFNS_PERIOD: u16 = 512u16;
pub const PFNS_PLAIN: u16 = 768u16;
pub const PFNS_NONUMBER: u16 = 1024u16;
pub const PFNS_NEWNUMBER: u16 = 32768u16;
pub const PARAFORMAT_SHADING_STYLE_NONE: u16 = 0u16;
pub const PARAFORMAT_SHADING_STYLE_DARK_HORIZ: u16 = 1u16;
pub const PARAFORMAT_SHADING_STYLE_DARK_VERT: u16 = 2u16;
pub const PARAFORMAT_SHADING_STYLE_DARK_DOWN_DIAG: u16 = 3u16;
pub const PARAFORMAT_SHADING_STYLE_DARK_UP_DIAG: u16 = 4u16;
pub const PARAFORMAT_SHADING_STYLE_DARK_GRID: u16 = 5u16;
pub const PARAFORMAT_SHADING_STYLE_DARK_TRELLIS: u16 = 6u16;
pub const PARAFORMAT_SHADING_STYLE_LIGHT_HORZ: u16 = 7u16;
pub const PARAFORMAT_SHADING_STYLE_LIGHT_VERT: u16 = 8u16;
pub const PARAFORMAT_SHADING_STYLE_LIGHT_DOWN_DIAG: u16 = 9u16;
pub const PARAFORMAT_SHADING_STYLE_LIGHT_UP_DIAG: u16 = 10u16;
pub const PARAFORMAT_SHADING_STYLE_LIGHT_GRID: u16 = 11u16;
pub const PARAFORMAT_SHADING_STYLE_LIGHT_TRELLIS: u16 = 12u16;
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
    pub poleobj: super::super::super::System::Ole::IOleObject,
    pub pstg: super::super::super::System::Com::StructuredStorage::IStorage,
    pub polesite: super::super::super::System::Ole::IOleClientSite,
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
pub const REO_ALIGNTORIGHT: u32 = 256u32;
pub const REO_BELOWBASELINE: u32 = 2u32;
pub const REO_BLANK: u32 = 16u32;
pub const REO_CANROTATE: u32 = 128u32;
pub const REO_DONTNEEDPALETTE: u32 = 32u32;
pub const REO_DYNAMICSIZE: u32 = 8u32;
pub const REO_GETMETAFILE: u32 = 4194304u32;
pub const REO_HILITED: u32 = 16777216u32;
pub const REO_INPLACEACTIVE: u32 = 33554432u32;
pub const REO_INVERTEDSELECT: u32 = 4u32;
pub const REO_LINK: u32 = 2147483648u32;
pub const REO_LINKAVAILABLE: u32 = 8388608u32;
pub const REO_OPEN: u32 = 67108864u32;
pub const REO_OWNERDRAWSELECT: u32 = 64u32;
pub const REO_RESIZABLE: u32 = 1u32;
pub const REO_SELECTED: u32 = 134217728u32;
pub const REO_STATIC: u32 = 1073741824u32;
pub const REO_USEASBACKGROUND: u32 = 1024u32;
pub const REO_WRAPTEXTAROUND: u32 = 512u32;
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
    pub pIStream: super::super::super::System::Com::IStream,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for RICHEDIT_IMAGE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for RICHEDIT_IMAGE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SEL_EMPTY: u16 = 0u16;
pub const SEL_TEXT: u16 = 1u16;
pub const SEL_OBJECT: u16 = 2u16;
pub const SEL_MULTICHAR: u16 = 4u16;
pub const SEL_MULTIOBJECT: u16 = 8u16;
pub const GCM_RIGHTMOUSEDROP: u16 = 32768u16;
pub const REO_GETOBJ_POLEOBJ: u32 = 1u32;
pub const REO_GETOBJ_PSTG: u32 = 2u32;
pub const REO_GETOBJ_POLESITE: u32 = 4u32;
pub const REO_GETOBJ_NO_INTERFACES: u32 = 0u32;
pub const REO_GETOBJ_ALL_INTERFACES: u32 = 7u32;
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
pub const TM_PLAINTEXT: i32 = 1i32;
pub const TM_RICHTEXT: i32 = 2i32;
pub const TM_SINGLELEVELUNDO: i32 = 4i32;
pub const TM_MULTILEVELUNDO: i32 = 8i32;
pub const TM_SINGLECODEPAGE: i32 = 16i32;
pub const TM_MULTICODEPAGE: i32 = 32i32;
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
pub const TXTBACK_TRANSPARENT: i32 = 0i32;
pub const TXTBACK_OPAQUE: i32 = 1i32;
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
pub const TXTHITRESULT_NOHIT: i32 = 0i32;
pub const TXTHITRESULT_TRANSPARENT: i32 = 1i32;
pub const TXTHITRESULT_CLOSE: i32 = 2i32;
pub const TXTHITRESULT_HIT: i32 = 3i32;
pub const TXTNS_FITTOCONTENT2: i32 = 0i32;
pub const TXTNS_FITTOCONTENT: i32 = 1i32;
pub const TXTNS_ROUNDTOLINE: i32 = 2i32;
pub const TXTNS_FITTOCONTENT3: i32 = 3i32;
pub const TXTNS_FITTOCONTENTWSP: i32 = 4i32;
pub const TXTNS_INCLUDELASTLINE: i32 = 1073741824i32;
pub const TXTNS_EMU: i32 = -2147483648i32;
pub const TXTVIEW_ACTIVE: i32 = 0i32;
pub const TXTVIEW_INACTIVE: i32 = -1i32;
pub const UID_UNKNOWN: i32 = 0i32;
pub const UID_TYPING: i32 = 1i32;
pub const UID_DELETE: i32 = 2i32;
pub const UID_DRAGDROP: i32 = 3i32;
pub const UID_CUT: i32 = 4i32;
pub const UID_PASTE: i32 = 5i32;
pub const UID_AUTOTABLE: i32 = 6i32;
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
pub const tomFalse: i32 = 0i32;
pub const tomTrue: i32 = -1i32;
pub const tomUndefined: i32 = -9999999i32;
pub const tomToggle: i32 = -9999998i32;
pub const tomAutoColor: i32 = -9999997i32;
pub const tomDefault: i32 = -9999996i32;
pub const tomSuspend: i32 = -9999995i32;
pub const tomResume: i32 = -9999994i32;
pub const tomApplyNow: i32 = 0i32;
pub const tomApplyLater: i32 = 1i32;
pub const tomTrackParms: i32 = 2i32;
pub const tomCacheParms: i32 = 3i32;
pub const tomApplyTmp: i32 = 4i32;
pub const tomDisableSmartFont: i32 = 8i32;
pub const tomEnableSmartFont: i32 = 9i32;
pub const tomUsePoints: i32 = 10i32;
pub const tomUseTwips: i32 = 11i32;
pub const tomBackward: i32 = -1073741823i32;
pub const tomForward: i32 = 1073741823i32;
pub const tomMove: i32 = 0i32;
pub const tomExtend: i32 = 1i32;
pub const tomNoSelection: i32 = 0i32;
pub const tomSelectionIP: i32 = 1i32;
pub const tomSelectionNormal: i32 = 2i32;
pub const tomSelectionFrame: i32 = 3i32;
pub const tomSelectionColumn: i32 = 4i32;
pub const tomSelectionRow: i32 = 5i32;
pub const tomSelectionBlock: i32 = 6i32;
pub const tomSelectionInlineShape: i32 = 7i32;
pub const tomSelectionShape: i32 = 8i32;
pub const tomSelStartActive: i32 = 1i32;
pub const tomSelAtEOL: i32 = 2i32;
pub const tomSelOvertype: i32 = 4i32;
pub const tomSelActive: i32 = 8i32;
pub const tomSelReplace: i32 = 16i32;
pub const tomEnd: i32 = 0i32;
pub const tomStart: i32 = 32i32;
pub const tomCollapseEnd: i32 = 0i32;
pub const tomCollapseStart: i32 = 1i32;
pub const tomClientCoord: i32 = 256i32;
pub const tomAllowOffClient: i32 = 512i32;
pub const tomTransform: i32 = 1024i32;
pub const tomObjectArg: i32 = 2048i32;
pub const tomAtEnd: i32 = 4096i32;
pub const tomNone: i32 = 0i32;
pub const tomSingle: i32 = 1i32;
pub const tomWords: i32 = 2i32;
pub const tomDouble: i32 = 3i32;
pub const tomDotted: i32 = 4i32;
pub const tomDash: i32 = 5i32;
pub const tomDashDot: i32 = 6i32;
pub const tomDashDotDot: i32 = 7i32;
pub const tomWave: i32 = 8i32;
pub const tomThick: i32 = 9i32;
pub const tomHair: i32 = 10i32;
pub const tomDoubleWave: i32 = 11i32;
pub const tomHeavyWave: i32 = 12i32;
pub const tomLongDash: i32 = 13i32;
pub const tomThickDash: i32 = 14i32;
pub const tomThickDashDot: i32 = 15i32;
pub const tomThickDashDotDot: i32 = 16i32;
pub const tomThickDotted: i32 = 17i32;
pub const tomThickLongDash: i32 = 18i32;
pub const tomLineSpaceSingle: i32 = 0i32;
pub const tomLineSpace1pt5: i32 = 1i32;
pub const tomLineSpaceDouble: i32 = 2i32;
pub const tomLineSpaceAtLeast: i32 = 3i32;
pub const tomLineSpaceExactly: i32 = 4i32;
pub const tomLineSpaceMultiple: i32 = 5i32;
pub const tomLineSpacePercent: i32 = 6i32;
pub const tomAlignLeft: i32 = 0i32;
pub const tomAlignCenter: i32 = 1i32;
pub const tomAlignRight: i32 = 2i32;
pub const tomAlignJustify: i32 = 3i32;
pub const tomAlignDecimal: i32 = 3i32;
pub const tomAlignBar: i32 = 4i32;
pub const tomDefaultTab: i32 = 5i32;
pub const tomAlignInterWord: i32 = 3i32;
pub const tomAlignNewspaper: i32 = 4i32;
pub const tomAlignInterLetter: i32 = 5i32;
pub const tomAlignScaled: i32 = 6i32;
pub const tomSpaces: i32 = 0i32;
pub const tomDots: i32 = 1i32;
pub const tomDashes: i32 = 2i32;
pub const tomLines: i32 = 3i32;
pub const tomThickLines: i32 = 4i32;
pub const tomEquals: i32 = 5i32;
pub const tomTabBack: i32 = -3i32;
pub const tomTabNext: i32 = -2i32;
pub const tomTabHere: i32 = -1i32;
pub const tomListNone: i32 = 0i32;
pub const tomListBullet: i32 = 1i32;
pub const tomListNumberAsArabic: i32 = 2i32;
pub const tomListNumberAsLCLetter: i32 = 3i32;
pub const tomListNumberAsUCLetter: i32 = 4i32;
pub const tomListNumberAsLCRoman: i32 = 5i32;
pub const tomListNumberAsUCRoman: i32 = 6i32;
pub const tomListNumberAsSequence: i32 = 7i32;
pub const tomListNumberedCircle: i32 = 8i32;
pub const tomListNumberedBlackCircleWingding: i32 = 9i32;
pub const tomListNumberedWhiteCircleWingding: i32 = 10i32;
pub const tomListNumberedArabicWide: i32 = 11i32;
pub const tomListNumberedChS: i32 = 12i32;
pub const tomListNumberedChT: i32 = 13i32;
pub const tomListNumberedJpnChS: i32 = 14i32;
pub const tomListNumberedJpnKor: i32 = 15i32;
pub const tomListNumberedArabic1: i32 = 16i32;
pub const tomListNumberedArabic2: i32 = 17i32;
pub const tomListNumberedHebrew: i32 = 18i32;
pub const tomListNumberedThaiAlpha: i32 = 19i32;
pub const tomListNumberedThaiNum: i32 = 20i32;
pub const tomListNumberedHindiAlpha: i32 = 21i32;
pub const tomListNumberedHindiAlpha1: i32 = 22i32;
pub const tomListNumberedHindiNum: i32 = 23i32;
pub const tomListParentheses: i32 = 65536i32;
pub const tomListPeriod: i32 = 131072i32;
pub const tomListPlain: i32 = 196608i32;
pub const tomListNoNumber: i32 = 262144i32;
pub const tomListMinus: i32 = 524288i32;
pub const tomIgnoreNumberStyle: i32 = 16777216i32;
pub const tomParaStyleNormal: i32 = -1i32;
pub const tomParaStyleHeading1: i32 = -2i32;
pub const tomParaStyleHeading2: i32 = -3i32;
pub const tomParaStyleHeading3: i32 = -4i32;
pub const tomParaStyleHeading4: i32 = -5i32;
pub const tomParaStyleHeading5: i32 = -6i32;
pub const tomParaStyleHeading6: i32 = -7i32;
pub const tomParaStyleHeading7: i32 = -8i32;
pub const tomParaStyleHeading8: i32 = -9i32;
pub const tomParaStyleHeading9: i32 = -10i32;
pub const tomCharacter: i32 = 1i32;
pub const tomWord: i32 = 2i32;
pub const tomSentence: i32 = 3i32;
pub const tomParagraph: i32 = 4i32;
pub const tomLine: i32 = 5i32;
pub const tomStory: i32 = 6i32;
pub const tomScreen: i32 = 7i32;
pub const tomSection: i32 = 8i32;
pub const tomTableColumn: i32 = 9i32;
pub const tomColumn: i32 = 9i32;
pub const tomRow: i32 = 10i32;
pub const tomWindow: i32 = 11i32;
pub const tomCell: i32 = 12i32;
pub const tomCharFormat: i32 = 13i32;
pub const tomParaFormat: i32 = 14i32;
pub const tomTable: i32 = 15i32;
pub const tomObject: i32 = 16i32;
pub const tomPage: i32 = 17i32;
pub const tomHardParagraph: i32 = 18i32;
pub const tomCluster: i32 = 19i32;
pub const tomInlineObject: i32 = 20i32;
pub const tomInlineObjectArg: i32 = 21i32;
pub const tomLeafLine: i32 = 22i32;
pub const tomLayoutColumn: i32 = 23i32;
pub const tomProcessId: i32 = 1073741825i32;
pub const tomMatchWord: i32 = 2i32;
pub const tomMatchCase: i32 = 4i32;
pub const tomMatchPattern: i32 = 8i32;
pub const tomUnknownStory: i32 = 0i32;
pub const tomMainTextStory: i32 = 1i32;
pub const tomFootnotesStory: i32 = 2i32;
pub const tomEndnotesStory: i32 = 3i32;
pub const tomCommentsStory: i32 = 4i32;
pub const tomTextFrameStory: i32 = 5i32;
pub const tomEvenPagesHeaderStory: i32 = 6i32;
pub const tomPrimaryHeaderStory: i32 = 7i32;
pub const tomEvenPagesFooterStory: i32 = 8i32;
pub const tomPrimaryFooterStory: i32 = 9i32;
pub const tomFirstPageHeaderStory: i32 = 10i32;
pub const tomFirstPageFooterStory: i32 = 11i32;
pub const tomScratchStory: i32 = 127i32;
pub const tomFindStory: i32 = 128i32;
pub const tomReplaceStory: i32 = 129i32;
pub const tomStoryInactive: i32 = 0i32;
pub const tomStoryActiveDisplay: i32 = 1i32;
pub const tomStoryActiveUI: i32 = 2i32;
pub const tomStoryActiveDisplayUI: i32 = 3i32;
pub const tomNoAnimation: i32 = 0i32;
pub const tomLasVegasLights: i32 = 1i32;
pub const tomBlinkingBackground: i32 = 2i32;
pub const tomSparkleText: i32 = 3i32;
pub const tomMarchingBlackAnts: i32 = 4i32;
pub const tomMarchingRedAnts: i32 = 5i32;
pub const tomShimmer: i32 = 6i32;
pub const tomWipeDown: i32 = 7i32;
pub const tomWipeRight: i32 = 8i32;
pub const tomAnimationMax: i32 = 8i32;
pub const tomLowerCase: i32 = 0i32;
pub const tomUpperCase: i32 = 1i32;
pub const tomTitleCase: i32 = 2i32;
pub const tomSentenceCase: i32 = 4i32;
pub const tomToggleCase: i32 = 5i32;
pub const tomReadOnly: i32 = 256i32;
pub const tomShareDenyRead: i32 = 512i32;
pub const tomShareDenyWrite: i32 = 1024i32;
pub const tomPasteFile: i32 = 4096i32;
pub const tomCreateNew: i32 = 16i32;
pub const tomCreateAlways: i32 = 32i32;
pub const tomOpenExisting: i32 = 48i32;
pub const tomOpenAlways: i32 = 64i32;
pub const tomTruncateExisting: i32 = 80i32;
pub const tomRTF: i32 = 1i32;
pub const tomText: i32 = 2i32;
pub const tomHTML: i32 = 3i32;
pub const tomWordDocument: i32 = 4i32;
pub const tomBold: i32 = -2147483647i32;
pub const tomItalic: i32 = -2147483646i32;
pub const tomUnderline: i32 = -2147483644i32;
pub const tomStrikeout: i32 = -2147483640i32;
pub const tomProtected: i32 = -2147483632i32;
pub const tomLink: i32 = -2147483616i32;
pub const tomSmallCaps: i32 = -2147483584i32;
pub const tomAllCaps: i32 = -2147483520i32;
pub const tomHidden: i32 = -2147483392i32;
pub const tomOutline: i32 = -2147483136i32;
pub const tomShadow: i32 = -2147482624i32;
pub const tomEmboss: i32 = -2147481600i32;
pub const tomImprint: i32 = -2147479552i32;
pub const tomDisabled: i32 = -2147475456i32;
pub const tomRevised: i32 = -2147467264i32;
pub const tomSubscriptCF: i32 = -2147418112i32;
pub const tomSuperscriptCF: i32 = -2147352576i32;
pub const tomFontBound: i32 = -2146435072i32;
pub const tomLinkProtected: i32 = -2139095040i32;
pub const tomInlineObjectStart: i32 = -2130706432i32;
pub const tomExtendedChar: i32 = -2113929216i32;
pub const tomAutoBackColor: i32 = -2080374784i32;
pub const tomMathZoneNoBuildUp: i32 = -2013265920i32;
pub const tomMathZone: i32 = -1879048192i32;
pub const tomMathZoneOrdinary: i32 = -1610612736i32;
pub const tomAutoTextColor: i32 = -1073741824i32;
pub const tomMathZoneDisplay: i32 = 262144i32;
pub const tomParaEffectRTL: i32 = 1i32;
pub const tomParaEffectKeep: i32 = 2i32;
pub const tomParaEffectKeepNext: i32 = 4i32;
pub const tomParaEffectPageBreakBefore: i32 = 8i32;
pub const tomParaEffectNoLineNumber: i32 = 16i32;
pub const tomParaEffectNoWidowControl: i32 = 32i32;
pub const tomParaEffectDoNotHyphen: i32 = 64i32;
pub const tomParaEffectSideBySide: i32 = 128i32;
pub const tomParaEffectCollapsed: i32 = 256i32;
pub const tomParaEffectOutlineLevel: i32 = 512i32;
pub const tomParaEffectBox: i32 = 1024i32;
pub const tomParaEffectTableRowDelimiter: i32 = 4096i32;
pub const tomParaEffectTable: i32 = 16384i32;
pub const tomModWidthPairs: i32 = 1i32;
pub const tomModWidthSpace: i32 = 2i32;
pub const tomAutoSpaceAlpha: i32 = 4i32;
pub const tomAutoSpaceNumeric: i32 = 8i32;
pub const tomAutoSpaceParens: i32 = 16i32;
pub const tomEmbeddedFont: i32 = 32i32;
pub const tomDoublestrike: i32 = 64i32;
pub const tomOverlapping: i32 = 128i32;
pub const tomNormalCaret: i32 = 0i32;
pub const tomKoreanBlockCaret: i32 = 1i32;
pub const tomNullCaret: i32 = 2i32;
pub const tomIncludeInset: i32 = 1i32;
pub const tomUnicodeBiDi: i32 = 1i32;
pub const tomMathCFCheck: i32 = 4i32;
pub const tomUnlink: i32 = 8i32;
pub const tomUnhide: i32 = 16i32;
pub const tomCheckTextLimit: i32 = 32i32;
pub const tomIgnoreCurrentFont: i32 = 0i32;
pub const tomMatchCharRep: i32 = 1i32;
pub const tomMatchFontSignature: i32 = 2i32;
pub const tomMatchAscii: i32 = 4i32;
pub const tomGetHeightOnly: i32 = 8i32;
pub const tomMatchMathFont: i32 = 16i32;
pub const tomCharset: i32 = -2147483648i32;
pub const tomCharRepFromLcid: i32 = 1073741824i32;
pub const tomAnsi: i32 = 0i32;
pub const tomEastEurope: i32 = 1i32;
pub const tomCyrillic: i32 = 2i32;
pub const tomGreek: i32 = 3i32;
pub const tomTurkish: i32 = 4i32;
pub const tomHebrew: i32 = 5i32;
pub const tomArabic: i32 = 6i32;
pub const tomBaltic: i32 = 7i32;
pub const tomVietnamese: i32 = 8i32;
pub const tomDefaultCharRep: i32 = 9i32;
pub const tomSymbol: i32 = 10i32;
pub const tomThai: i32 = 11i32;
pub const tomShiftJIS: i32 = 12i32;
pub const tomGB2312: i32 = 13i32;
pub const tomHangul: i32 = 14i32;
pub const tomBIG5: i32 = 15i32;
pub const tomPC437: i32 = 16i32;
pub const tomOEM: i32 = 17i32;
pub const tomMac: i32 = 18i32;
pub const tomArmenian: i32 = 19i32;
pub const tomSyriac: i32 = 20i32;
pub const tomThaana: i32 = 21i32;
pub const tomDevanagari: i32 = 22i32;
pub const tomBengali: i32 = 23i32;
pub const tomGurmukhi: i32 = 24i32;
pub const tomGujarati: i32 = 25i32;
pub const tomOriya: i32 = 26i32;
pub const tomTamil: i32 = 27i32;
pub const tomTelugu: i32 = 28i32;
pub const tomKannada: i32 = 29i32;
pub const tomMalayalam: i32 = 30i32;
pub const tomSinhala: i32 = 31i32;
pub const tomLao: i32 = 32i32;
pub const tomTibetan: i32 = 33i32;
pub const tomMyanmar: i32 = 34i32;
pub const tomGeorgian: i32 = 35i32;
pub const tomJamo: i32 = 36i32;
pub const tomEthiopic: i32 = 37i32;
pub const tomCherokee: i32 = 38i32;
pub const tomAboriginal: i32 = 39i32;
pub const tomOgham: i32 = 40i32;
pub const tomRunic: i32 = 41i32;
pub const tomKhmer: i32 = 42i32;
pub const tomMongolian: i32 = 43i32;
pub const tomBraille: i32 = 44i32;
pub const tomYi: i32 = 45i32;
pub const tomLimbu: i32 = 46i32;
pub const tomTaiLe: i32 = 47i32;
pub const tomNewTaiLue: i32 = 48i32;
pub const tomSylotiNagri: i32 = 49i32;
pub const tomKharoshthi: i32 = 50i32;
pub const tomKayahli: i32 = 51i32;
pub const tomUsymbol: i32 = 52i32;
pub const tomEmoji: i32 = 53i32;
pub const tomGlagolitic: i32 = 54i32;
pub const tomLisu: i32 = 55i32;
pub const tomVai: i32 = 56i32;
pub const tomNKo: i32 = 57i32;
pub const tomOsmanya: i32 = 58i32;
pub const tomPhagsPa: i32 = 59i32;
pub const tomGothic: i32 = 60i32;
pub const tomDeseret: i32 = 61i32;
pub const tomTifinagh: i32 = 62i32;
pub const tomCharRepMax: i32 = 63i32;
pub const tomRE10Mode: i32 = 1i32;
pub const tomUseAtFont: i32 = 2i32;
pub const tomTextFlowMask: i32 = 12i32;
pub const tomTextFlowES: i32 = 0i32;
pub const tomTextFlowSW: i32 = 4i32;
pub const tomTextFlowWN: i32 = 8i32;
pub const tomTextFlowNE: i32 = 12i32;
pub const tomNoIME: i32 = 524288i32;
pub const tomSelfIME: i32 = 262144i32;
pub const tomNoUpScroll: i32 = 65536i32;
pub const tomNoVpScroll: i32 = 262144i32;
pub const tomNoLink: i32 = 0i32;
pub const tomClientLink: i32 = 1i32;
pub const tomFriendlyLinkName: i32 = 2i32;
pub const tomFriendlyLinkAddress: i32 = 3i32;
pub const tomAutoLinkURL: i32 = 4i32;
pub const tomAutoLinkEmail: i32 = 5i32;
pub const tomAutoLinkPhone: i32 = 6i32;
pub const tomAutoLinkPath: i32 = 7i32;
pub const tomCompressNone: i32 = 0i32;
pub const tomCompressPunctuation: i32 = 1i32;
pub const tomCompressPunctuationAndKana: i32 = 2i32;
pub const tomCompressMax: i32 = 2i32;
pub const tomUnderlinePositionAuto: i32 = 0i32;
pub const tomUnderlinePositionBelow: i32 = 1i32;
pub const tomUnderlinePositionAbove: i32 = 2i32;
pub const tomUnderlinePositionMax: i32 = 2i32;
pub const tomFontAlignmentAuto: i32 = 0i32;
pub const tomFontAlignmentTop: i32 = 1i32;
pub const tomFontAlignmentBaseline: i32 = 2i32;
pub const tomFontAlignmentBottom: i32 = 3i32;
pub const tomFontAlignmentCenter: i32 = 4i32;
pub const tomFontAlignmentMax: i32 = 4i32;
pub const tomRubyBelow: i32 = 128i32;
pub const tomRubyAlignCenter: i32 = 0i32;
pub const tomRubyAlign010: i32 = 1i32;
pub const tomRubyAlign121: i32 = 2i32;
pub const tomRubyAlignLeft: i32 = 3i32;
pub const tomRubyAlignRight: i32 = 4i32;
pub const tomLimitsDefault: i32 = 0i32;
pub const tomLimitsUnderOver: i32 = 1i32;
pub const tomLimitsSubSup: i32 = 2i32;
pub const tomUpperLimitAsSuperScript: i32 = 3i32;
pub const tomLimitsOpposite: i32 = 4i32;
pub const tomShowLLimPlaceHldr: i32 = 8i32;
pub const tomShowULimPlaceHldr: i32 = 16i32;
pub const tomDontGrowWithContent: i32 = 64i32;
pub const tomGrowWithContent: i32 = 128i32;
pub const tomSubSupAlign: i32 = 1i32;
pub const tomLimitAlignMask: i32 = 3i32;
pub const tomLimitAlignCenter: i32 = 0i32;
pub const tomLimitAlignLeft: i32 = 1i32;
pub const tomLimitAlignRight: i32 = 2i32;
pub const tomShowDegPlaceHldr: i32 = 8i32;
pub const tomAlignDefault: i32 = 0i32;
pub const tomAlignMatchAscentDescent: i32 = 2i32;
pub const tomMathVariant: i32 = 32i32;
pub const tomStyleDefault: i32 = 0i32;
pub const tomStyleScriptScriptCramped: i32 = 1i32;
pub const tomStyleScriptScript: i32 = 2i32;
pub const tomStyleScriptCramped: i32 = 3i32;
pub const tomStyleScript: i32 = 4i32;
pub const tomStyleTextCramped: i32 = 5i32;
pub const tomStyleText: i32 = 6i32;
pub const tomStyleDisplayCramped: i32 = 7i32;
pub const tomStyleDisplay: i32 = 8i32;
pub const tomMathRelSize: i32 = 64i32;
pub const tomDecDecSize: i32 = 254i32;
pub const tomDecSize: i32 = 255i32;
pub const tomIncSize: i32 = 65i32;
pub const tomIncIncSize: i32 = 66i32;
pub const tomGravityUI: i32 = 0i32;
pub const tomGravityBack: i32 = 1i32;
pub const tomGravityFore: i32 = 2i32;
pub const tomGravityIn: i32 = 3i32;
pub const tomGravityOut: i32 = 4i32;
pub const tomGravityBackward: i32 = 536870912i32;
pub const tomGravityForward: i32 = 1073741824i32;
pub const tomAdjustCRLF: i32 = 1i32;
pub const tomUseCRLF: i32 = 2i32;
pub const tomTextize: i32 = 4i32;
pub const tomAllowFinalEOP: i32 = 8i32;
pub const tomFoldMathAlpha: i32 = 16i32;
pub const tomNoHidden: i32 = 32i32;
pub const tomIncludeNumbering: i32 = 64i32;
pub const tomTranslateTableCell: i32 = 128i32;
pub const tomNoMathZoneBrackets: i32 = 256i32;
pub const tomConvertMathChar: i32 = 512i32;
pub const tomNoUCGreekItalic: i32 = 1024i32;
pub const tomAllowMathBold: i32 = 2048i32;
pub const tomLanguageTag: i32 = 4096i32;
pub const tomConvertRTF: i32 = 8192i32;
pub const tomApplyRtfDocProps: i32 = 16384i32;
pub const tomPhantomShow: i32 = 1i32;
pub const tomPhantomZeroWidth: i32 = 2i32;
pub const tomPhantomZeroAscent: i32 = 4i32;
pub const tomPhantomZeroDescent: i32 = 8i32;
pub const tomPhantomTransparent: i32 = 16i32;
pub const tomPhantomASmash: i32 = 5i32;
pub const tomPhantomDSmash: i32 = 9i32;
pub const tomPhantomHSmash: i32 = 3i32;
pub const tomPhantomSmash: i32 = 13i32;
pub const tomPhantomHorz: i32 = 12i32;
pub const tomPhantomVert: i32 = 2i32;
pub const tomBoxHideTop: i32 = 1i32;
pub const tomBoxHideBottom: i32 = 2i32;
pub const tomBoxHideLeft: i32 = 4i32;
pub const tomBoxHideRight: i32 = 8i32;
pub const tomBoxStrikeH: i32 = 16i32;
pub const tomBoxStrikeV: i32 = 32i32;
pub const tomBoxStrikeTLBR: i32 = 64i32;
pub const tomBoxStrikeBLTR: i32 = 128i32;
pub const tomBoxAlignCenter: i32 = 1i32;
pub const tomSpaceMask: i32 = 28i32;
pub const tomSpaceDefault: i32 = 0i32;
pub const tomSpaceUnary: i32 = 4i32;
pub const tomSpaceBinary: i32 = 8i32;
pub const tomSpaceRelational: i32 = 12i32;
pub const tomSpaceSkip: i32 = 16i32;
pub const tomSpaceOrd: i32 = 20i32;
pub const tomSpaceDifferential: i32 = 24i32;
pub const tomSizeText: i32 = 32i32;
pub const tomSizeScript: i32 = 64i32;
pub const tomSizeScriptScript: i32 = 96i32;
pub const tomNoBreak: i32 = 128i32;
pub const tomTransparentForPositioning: i32 = 256i32;
pub const tomTransparentForSpacing: i32 = 512i32;
pub const tomStretchCharBelow: i32 = 0i32;
pub const tomStretchCharAbove: i32 = 1i32;
pub const tomStretchBaseBelow: i32 = 2i32;
pub const tomStretchBaseAbove: i32 = 3i32;
pub const tomMatrixAlignMask: i32 = 3i32;
pub const tomMatrixAlignCenter: i32 = 0i32;
pub const tomMatrixAlignTopRow: i32 = 1i32;
pub const tomMatrixAlignBottomRow: i32 = 3i32;
pub const tomShowMatPlaceHldr: i32 = 8i32;
pub const tomEqArrayLayoutWidth: i32 = 1i32;
pub const tomEqArrayAlignMask: i32 = 12i32;
pub const tomEqArrayAlignCenter: i32 = 0i32;
pub const tomEqArrayAlignTopRow: i32 = 4i32;
pub const tomEqArrayAlignBottomRow: i32 = 12i32;
pub const tomMathManualBreakMask: i32 = 127i32;
pub const tomMathBreakLeft: i32 = 125i32;
pub const tomMathBreakCenter: i32 = 126i32;
pub const tomMathBreakRight: i32 = 127i32;
pub const tomMathEqAlign: i32 = 128i32;
pub const tomMathArgShadingStart: i32 = 593i32;
pub const tomMathArgShadingEnd: i32 = 594i32;
pub const tomMathObjShadingStart: i32 = 595i32;
pub const tomMathObjShadingEnd: i32 = 596i32;
pub const tomFunctionTypeNone: i32 = 0i32;
pub const tomFunctionTypeTakesArg: i32 = 1i32;
pub const tomFunctionTypeTakesLim: i32 = 2i32;
pub const tomFunctionTypeTakesLim2: i32 = 3i32;
pub const tomFunctionTypeIsLim: i32 = 4i32;
pub const tomMathParaAlignDefault: i32 = 0i32;
pub const tomMathParaAlignCenterGroup: i32 = 1i32;
pub const tomMathParaAlignCenter: i32 = 2i32;
pub const tomMathParaAlignLeft: i32 = 3i32;
pub const tomMathParaAlignRight: i32 = 4i32;
pub const tomMathDispAlignMask: i32 = 3i32;
pub const tomMathDispAlignCenterGroup: i32 = 0i32;
pub const tomMathDispAlignCenter: i32 = 1i32;
pub const tomMathDispAlignLeft: i32 = 2i32;
pub const tomMathDispAlignRight: i32 = 3i32;
pub const tomMathDispIntUnderOver: i32 = 4i32;
pub const tomMathDispFracTeX: i32 = 8i32;
pub const tomMathDispNaryGrow: i32 = 16i32;
pub const tomMathDocEmptyArgMask: i32 = 96i32;
pub const tomMathDocEmptyArgAuto: i32 = 0i32;
pub const tomMathDocEmptyArgAlways: i32 = 32i32;
pub const tomMathDocEmptyArgNever: i32 = 64i32;
pub const tomMathDocSbSpOpUnchanged: i32 = 128i32;
pub const tomMathDocDiffMask: i32 = 768i32;
pub const tomMathDocDiffDefault: i32 = 0i32;
pub const tomMathDocDiffUpright: i32 = 256i32;
pub const tomMathDocDiffItalic: i32 = 512i32;
pub const tomMathDocDiffOpenItalic: i32 = 768i32;
pub const tomMathDispNarySubSup: i32 = 1024i32;
pub const tomMathDispDef: i32 = 2048i32;
pub const tomMathEnableRtl: i32 = 4096i32;
pub const tomMathBrkBinMask: i32 = 196608i32;
pub const tomMathBrkBinBefore: i32 = 0i32;
pub const tomMathBrkBinAfter: i32 = 65536i32;
pub const tomMathBrkBinDup: i32 = 131072i32;
pub const tomMathBrkBinSubMask: i32 = 786432i32;
pub const tomMathBrkBinSubMM: i32 = 0i32;
pub const tomMathBrkBinSubPM: i32 = 262144i32;
pub const tomMathBrkBinSubMP: i32 = 524288i32;
pub const tomSelRange: i32 = 597i32;
pub const tomHstring: i32 = 596i32;
pub const tomFontPropTeXStyle: i32 = 828i32;
pub const tomFontPropAlign: i32 = 829i32;
pub const tomFontStretch: i32 = 830i32;
pub const tomFontStyle: i32 = 831i32;
pub const tomFontStyleUpright: i32 = 0i32;
pub const tomFontStyleOblique: i32 = 1i32;
pub const tomFontStyleItalic: i32 = 2i32;
pub const tomFontStretchDefault: i32 = 0i32;
pub const tomFontStretchUltraCondensed: i32 = 1i32;
pub const tomFontStretchExtraCondensed: i32 = 2i32;
pub const tomFontStretchCondensed: i32 = 3i32;
pub const tomFontStretchSemiCondensed: i32 = 4i32;
pub const tomFontStretchNormal: i32 = 5i32;
pub const tomFontStretchSemiExpanded: i32 = 6i32;
pub const tomFontStretchExpanded: i32 = 7i32;
pub const tomFontStretchExtraExpanded: i32 = 8i32;
pub const tomFontStretchUltraExpanded: i32 = 9i32;
pub const tomFontWeightDefault: i32 = 0i32;
pub const tomFontWeightThin: i32 = 100i32;
pub const tomFontWeightExtraLight: i32 = 200i32;
pub const tomFontWeightLight: i32 = 300i32;
pub const tomFontWeightNormal: i32 = 400i32;
pub const tomFontWeightRegular: i32 = 400i32;
pub const tomFontWeightMedium: i32 = 500i32;
pub const tomFontWeightSemiBold: i32 = 600i32;
pub const tomFontWeightBold: i32 = 700i32;
pub const tomFontWeightExtraBold: i32 = 800i32;
pub const tomFontWeightBlack: i32 = 900i32;
pub const tomFontWeightHeavy: i32 = 900i32;
pub const tomFontWeightExtraBlack: i32 = 950i32;
pub const tomParaPropMathAlign: i32 = 1079i32;
pub const tomDocMathBuild: i32 = 128i32;
pub const tomMathLMargin: i32 = 129i32;
pub const tomMathRMargin: i32 = 130i32;
pub const tomMathWrapIndent: i32 = 131i32;
pub const tomMathWrapRight: i32 = 132i32;
pub const tomMathPostSpace: i32 = 134i32;
pub const tomMathPreSpace: i32 = 133i32;
pub const tomMathInterSpace: i32 = 135i32;
pub const tomMathIntraSpace: i32 = 136i32;
pub const tomCanCopy: i32 = 137i32;
pub const tomCanRedo: i32 = 138i32;
pub const tomCanUndo: i32 = 139i32;
pub const tomUndoLimit: i32 = 140i32;
pub const tomDocAutoLink: i32 = 141i32;
pub const tomEllipsisMode: i32 = 142i32;
pub const tomEllipsisState: i32 = 143i32;
pub const tomEllipsisNone: i32 = 0i32;
pub const tomEllipsisEnd: i32 = 1i32;
pub const tomEllipsisWord: i32 = 3i32;
pub const tomEllipsisPresent: i32 = 1i32;
pub const tomVTopCell: i32 = 1i32;
pub const tomVLowCell: i32 = 2i32;
pub const tomHStartCell: i32 = 4i32;
pub const tomHContCell: i32 = 8i32;
pub const tomRowUpdate: i32 = 1i32;
pub const tomRowApplyDefault: i32 = 0i32;
pub const tomCellStructureChangeOnly: i32 = 1i32;
pub const tomRowHeightActual: i32 = 2059i32;
