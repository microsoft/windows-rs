pub const ATP_CHANGE: u32 = 1;
pub const ATP_NOCHANGE: u32 = 0;
pub const ATP_NODELIMITER: u32 = 2;
pub const ATP_REPLACEALLTEXT: u32 = 4;
pub const AURL_DISABLEMIXEDLGC: u32 = 32;
pub const AURL_ENABLEDRIVELETTERS: u32 = 16;
pub const AURL_ENABLEEA: u32 = 1;
pub const AURL_ENABLEEAURLS: u32 = 8;
pub const AURL_ENABLEEMAILADDR: u32 = 2;
pub const AURL_ENABLETELNO: u32 = 4;
pub const AURL_ENABLEURL: u32 = 1;
#[cfg(feature = "winnt")]
pub type AutoCorrectProc = Option<unsafe extern "system" fn(langid: super::LANGID, pszbefore: *const u16, pszafter: *mut u16, cchafter: i32, pcchreplaced: *mut i32) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BIDIOPTIONS {
    pub cbSize: u32,
    pub wMask: u16,
    pub wEffects: u16,
}
pub const BOE_CONTEXTALIGNMENT: u32 = 16;
pub const BOE_CONTEXTREADING: u32 = 8;
pub const BOE_FORCERECALC: u32 = 32;
pub const BOE_LEGACYBIDICLASS: u32 = 64;
pub const BOE_NEUTRALOVERRIDE: u32 = 4;
pub const BOE_UNICODEBIDI: u32 = 128;
pub const BOM_CONTEXTALIGNMENT: u32 = 16;
pub const BOM_CONTEXTREADING: u32 = 8;
pub const BOM_LEGACYBIDICLASS: u32 = 64;
pub const BOM_NEUTRALOVERRIDE: u32 = 4;
pub const BOM_UNICODEBIDI: u32 = 128;
pub const CERICHEDIT_CLASSA: windows_sys::core::PCSTR = windows_sys::core::s!("RichEditCEA");
pub const CERICHEDIT_CLASSW: windows_sys::core::PCWSTR = windows_sys::core::w!("RichEditCEW");
pub const CFE_ALLCAPS: u32 = 128;
pub const CFE_AUTOBACKCOLOR: u32 = 67108864;
pub const CFE_AUTOCOLOR: u32 = 1073741824;
pub const CFE_BOLD: u32 = 1;
pub const CFE_DISABLED: u32 = 8192;
pub const CFE_EMBOSS: u32 = 2048;
pub const CFE_EXTENDED: u32 = 33554432;
pub const CFE_FONTBOUND: u32 = 1048576;
pub const CFE_HIDDEN: u32 = 256;
pub const CFE_IMPRINT: u32 = 4096;
pub const CFE_ITALIC: u32 = 2;
pub const CFE_LINK: u32 = 32;
pub const CFE_LINKPROTECTED: u32 = 8388608;
pub const CFE_MATH: u32 = 268435456;
pub const CFE_MATHNOBUILDUP: u32 = 134217728;
pub const CFE_MATHORDINARY: u32 = 536870912;
pub const CFE_OUTLINE: u32 = 512;
pub const CFE_PROTECTED: u32 = 16;
pub const CFE_REVISED: u32 = 16384;
pub const CFE_SHADOW: u32 = 1024;
pub const CFE_SMALLCAPS: u32 = 64;
pub const CFE_STRIKEOUT: u32 = 8;
pub const CFE_SUBSCRIPT: u32 = 65536;
pub const CFE_SUPERSCRIPT: u32 = 131072;
pub const CFE_UNDERLINE: u32 = 4;
pub const CFM_ALL: i32 = -134217665;
pub const CFM_ALL2: i32 = -1;
pub const CFM_ALLCAPS: u32 = 128;
pub const CFM_ALLEFFECTS: u32 = 2115207167;
pub const CFM_ANIMATION: u32 = 262144;
pub const CFM_BACKCOLOR: u32 = 67108864;
pub const CFM_BOLD: u32 = 1;
pub const CFM_CHARSET: u32 = 134217728;
pub const CFM_COLOR: u32 = 1073741824;
pub const CFM_COOKIE: u32 = 16777216;
pub const CFM_DISABLED: u32 = 8192;
pub const CFM_EFFECTS: u32 = 1073741887;
pub const CFM_EFFECTS2: u32 = 1141080063;
pub const CFM_EMBOSS: u32 = 2048;
pub const CFM_EXTENDED: u32 = 33554432;
pub const CFM_FACE: u32 = 536870912;
pub const CFM_FONTBOUND: u32 = 1048576;
pub const CFM_HIDDEN: u32 = 256;
pub const CFM_IMPRINT: u32 = 4096;
pub const CFM_ITALIC: u32 = 2;
pub const CFM_KERNING: u32 = 1048576;
pub const CFM_LCID: u32 = 33554432;
pub const CFM_LINK: u32 = 32;
pub const CFM_LINKPROTECTED: u32 = 8388608;
pub const CFM_MATH: u32 = 268435456;
pub const CFM_MATHNOBUILDUP: u32 = 134217728;
pub const CFM_MATHORDINARY: u32 = 536870912;
pub const CFM_OFFSET: u32 = 268435456;
pub const CFM_OUTLINE: u32 = 512;
pub const CFM_PROTECTED: u32 = 16;
pub const CFM_REVAUTHOR: u32 = 32768;
pub const CFM_REVISED: u32 = 16384;
pub const CFM_SHADOW: u32 = 1024;
pub const CFM_SIZE: u32 = 2147483648;
pub const CFM_SMALLCAPS: u32 = 64;
pub const CFM_SPACING: u32 = 2097152;
pub const CFM_STRIKEOUT: u32 = 8;
pub const CFM_STYLE: u32 = 524288;
pub const CFM_SUBSCRIPT: u32 = 196608;
pub const CFM_SUPERSCRIPT: u32 = 196608;
pub const CFM_UNDERLINE: u32 = 4;
pub const CFM_UNDERLINETYPE: u32 = 8388608;
pub const CFM_WEIGHT: u32 = 4194304;
pub const CFU_CF1UNDERLINE: u32 = 255;
pub const CFU_INVERT: u32 = 254;
pub const CFU_UNDERLINE: u32 = 1;
pub const CFU_UNDERLINEDASH: u32 = 5;
pub const CFU_UNDERLINEDASHDOT: u32 = 6;
pub const CFU_UNDERLINEDASHDOTDOT: u32 = 7;
pub const CFU_UNDERLINEDOTTED: u32 = 4;
pub const CFU_UNDERLINEDOUBLE: u32 = 3;
pub const CFU_UNDERLINEDOUBLEWAVE: u32 = 11;
pub const CFU_UNDERLINEHAIRLINE: u32 = 10;
pub const CFU_UNDERLINEHEAVYWAVE: u32 = 12;
pub const CFU_UNDERLINELONGDASH: u32 = 13;
pub const CFU_UNDERLINENONE: u32 = 0;
pub const CFU_UNDERLINETHICK: u32 = 9;
pub const CFU_UNDERLINETHICKDASH: u32 = 14;
pub const CFU_UNDERLINETHICKDASHDOT: u32 = 15;
pub const CFU_UNDERLINETHICKDASHDOTDOT: u32 = 16;
pub const CFU_UNDERLINETHICKDOTTED: u32 = 17;
pub const CFU_UNDERLINETHICKLONGDASH: u32 = 18;
pub const CFU_UNDERLINEWAVE: u32 = 8;
pub const CFU_UNDERLINEWORD: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CHARFORMAT2A {
    pub Base: CHARFORMATA,
    pub wWeight: u16,
    pub sSpacing: i16,
    pub crBackColor: super::COLORREF,
    pub lcid: super::LCID,
    pub Anonymous: CHARFORMAT2A_0,
    pub sStyle: i16,
    pub wKerning: u16,
    pub bUnderlineType: u8,
    pub bAnimation: u8,
    pub bRevAuthor: u8,
    pub bUnderlineColor: u8,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for CHARFORMAT2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union CHARFORMAT2A_0 {
    pub dwReserved: u32,
    pub dwCookie: u32,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for CHARFORMAT2A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CHARFORMAT2W {
    pub Base: CHARFORMATW,
    pub wWeight: u16,
    pub sSpacing: i16,
    pub crBackColor: super::COLORREF,
    pub lcid: super::LCID,
    pub Anonymous: CHARFORMAT2W_0,
    pub sStyle: i16,
    pub wKerning: u16,
    pub bUnderlineType: u8,
    pub bAnimation: u8,
    pub bRevAuthor: u8,
    pub bUnderlineColor: u8,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for CHARFORMAT2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union CHARFORMAT2W_0 {
    pub dwReserved: u32,
    pub dwCookie: u32,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for CHARFORMAT2W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct CHARFORMATA {
    pub cbSize: u32,
    pub dwMask: u32,
    pub dwEffects: u32,
    pub yHeight: i32,
    pub yOffset: i32,
    pub crTextColor: super::COLORREF,
    pub bCharSet: u8,
    pub bPitchAndFamily: u8,
    pub szFaceName: [i8; 32],
}
#[cfg(feature = "windef")]
impl Default for CHARFORMATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct CHARFORMATW {
    pub cbSize: u32,
    pub dwMask: u32,
    pub dwEffects: u32,
    pub yHeight: i32,
    pub yOffset: i32,
    pub crTextColor: super::COLORREF,
    pub bCharSet: u8,
    pub bPitchAndFamily: u8,
    pub szFaceName: [u16; 32],
}
#[cfg(feature = "windef")]
impl Default for CHARFORMATW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CHARRANGE {
    pub cpMin: i32,
    pub cpMax: i32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winuser", feature = "wtypes"))]
#[derive(Clone, Copy, Default)]
pub struct CLIPBOARDFORMAT {
    pub nmhdr: super::NMHDR,
    pub cf: super::CLIPFORMAT,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winuser", feature = "wtypes"))]
#[derive(Clone, Copy, Default)]
pub struct CLIPBOARDFORMAT {
    pub nmhdr: super::NMHDR,
    pub cf: super::CLIPFORMAT,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct COMPCOLOR {
    pub crText: super::COLORREF,
    pub crBackground: super::COLORREF,
    pub dwEffects: u32,
}
pub const CTFMODEBIAS_CONVERSATION: u32 = 5;
pub const CTFMODEBIAS_DATETIME: u32 = 4;
pub const CTFMODEBIAS_DEFAULT: u32 = 0;
pub const CTFMODEBIAS_FILENAME: u32 = 1;
pub const CTFMODEBIAS_FULLWIDTHALPHANUMERIC: u32 = 11;
pub const CTFMODEBIAS_HALFWIDTHALPHANUMERIC: u32 = 12;
pub const CTFMODEBIAS_HALFWIDTHKATAKANA: u32 = 10;
pub const CTFMODEBIAS_HANGUL: u32 = 9;
pub const CTFMODEBIAS_HIRAGANA: u32 = 7;
pub const CTFMODEBIAS_KATAKANA: u32 = 8;
pub const CTFMODEBIAS_NAME: u32 = 2;
pub const CTFMODEBIAS_NUMERIC: u32 = 6;
pub const CTFMODEBIAS_READING: u32 = 3;
pub const ECN_ENDCOMPOSITION: u32 = 1;
pub const ECN_NEWTEXT: u32 = 2;
pub const ECOOP_AND: u32 = 3;
pub const ECOOP_OR: u32 = 2;
pub const ECOOP_SET: u32 = 1;
pub const ECOOP_XOR: u32 = 4;
pub const ECO_AUTOHSCROLL: u32 = 128;
pub const ECO_AUTOVSCROLL: u32 = 64;
pub const ECO_AUTOWORDSELECTION: u32 = 1;
pub const ECO_NOHIDESEL: u32 = 256;
pub const ECO_READONLY: u32 = 2048;
pub const ECO_SAVESEL: u32 = 32768;
pub const ECO_SELECTIONBAR: u32 = 16777216;
pub const ECO_VERTICAL: u32 = 4194304;
pub const ECO_WANTRETURN: u32 = 4096;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
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
pub type EDITWORDBREAKPROCEX = Option<unsafe extern "system" fn(pchtext: *mut i8, cchtext: i32, bcharset: u8, action: i32) -> i32>;
pub const ELLIPSIS_END: u32 = 1;
pub const ELLIPSIS_MASK: u32 = 3;
pub const ELLIPSIS_NONE: u32 = 0;
pub const ELLIPSIS_WORD: u32 = 3;
pub const EMO_ENTER: u32 = 1;
pub const EMO_EXIT: u32 = 0;
pub const EMO_EXPAND: u32 = 3;
pub const EMO_EXPANDDOCUMENT: u32 = 1;
pub const EMO_EXPANDSELECTION: u32 = 0;
pub const EMO_GETVIEWMODE: u32 = 5;
pub const EMO_MOVESELECTION: u32 = 4;
pub const EMO_PROMOTE: u32 = 2;
pub const EM_AUTOURLDETECT: u32 = 1115;
pub const EM_CALLAUTOCORRECTPROC: u32 = 1279;
pub const EM_CANPASTE: u32 = 1074;
pub const EM_CANREDO: u32 = 1109;
pub const EM_CONVPOSITION: u32 = 1132;
pub const EM_DISPLAYBAND: u32 = 1075;
pub const EM_EXGETSEL: u32 = 1076;
pub const EM_EXLIMITTEXT: u32 = 1077;
pub const EM_EXLINEFROMCHAR: u32 = 1078;
pub const EM_EXSETSEL: u32 = 1079;
pub const EM_FINDTEXT: u32 = 1080;
pub const EM_FINDTEXTEX: u32 = 1103;
pub const EM_FINDTEXTEXW: u32 = 1148;
pub const EM_FINDTEXTW: u32 = 1147;
pub const EM_FINDWORDBREAK: u32 = 1100;
pub const EM_FORMATRANGE: u32 = 1081;
pub const EM_GETAUTOCORRECTPROC: u32 = 1257;
pub const EM_GETAUTOURLDETECT: u32 = 1116;
pub const EM_GETBIDIOPTIONS: u32 = 1225;
pub const EM_GETCHARFORMAT: u32 = 1082;
pub const EM_GETCTFMODEBIAS: u32 = 1261;
pub const EM_GETCTFOPENSTATUS: u32 = 1264;
pub const EM_GETEDITSTYLE: u32 = 1229;
pub const EM_GETEDITSTYLEEX: u32 = 1300;
pub const EM_GETELLIPSISMODE: u32 = 1329;
pub const EM_GETELLIPSISSTATE: u32 = 1346;
pub const EM_GETEVENTMASK: u32 = 1083;
pub const EM_GETHYPHENATEINFO: u32 = 1254;
pub const EM_GETIMECOLOR: u32 = 1129;
pub const EM_GETIMECOMPMODE: u32 = 1146;
pub const EM_GETIMECOMPTEXT: u32 = 1266;
pub const EM_GETIMEMODEBIAS: u32 = 1151;
pub const EM_GETIMEOPTIONS: u32 = 1131;
pub const EM_GETIMEPROPERTY: u32 = 1268;
pub const EM_GETLANGOPTIONS: u32 = 1145;
pub const EM_GETOLEINTERFACE: u32 = 1084;
pub const EM_GETOPTIONS: u32 = 1102;
pub const EM_GETPAGE: u32 = 1252;
pub const EM_GETPAGEROTATE: u32 = 1259;
pub const EM_GETPARAFORMAT: u32 = 1085;
pub const EM_GETPUNCTUATION: u32 = 1125;
pub const EM_GETQUERYRTFOBJ: u32 = 1293;
pub const EM_GETREDONAME: u32 = 1111;
pub const EM_GETSCROLLPOS: u32 = 1245;
pub const EM_GETSELTEXT: u32 = 1086;
pub const EM_GETSTORYTYPE: u32 = 1314;
pub const EM_GETTABLEPARMS: u32 = 1289;
pub const EM_GETTEXTEX: u32 = 1118;
pub const EM_GETTEXTLENGTHEX: u32 = 1119;
pub const EM_GETTEXTMODE: u32 = 1114;
pub const EM_GETTEXTRANGE: u32 = 1099;
pub const EM_GETTOUCHOPTIONS: u32 = 1334;
pub const EM_GETTYPOGRAPHYOPTIONS: u32 = 1227;
pub const EM_GETUNDONAME: u32 = 1110;
pub const EM_GETVIEWKIND: u32 = 1250;
pub const EM_GETWORDBREAKPROCEX: u32 = 1104;
pub const EM_GETWORDWRAPMODE: u32 = 1127;
pub const EM_HIDESELECTION: u32 = 1087;
pub const EM_INSERTIMAGE: u32 = 1338;
pub const EM_INSERTTABLE: u32 = 1256;
pub const EM_ISIME: u32 = 1267;
pub const EM_OUTLINE: u32 = 1244;
pub const EM_PASTESPECIAL: u32 = 1088;
pub const EM_RECONVERSION: u32 = 1149;
pub const EM_REDO: u32 = 1108;
pub const EM_REQUESTRESIZE: u32 = 1089;
pub const EM_SELECTIONTYPE: u32 = 1090;
pub const EM_SETAUTOCORRECTPROC: u32 = 1258;
pub const EM_SETBIDIOPTIONS: u32 = 1224;
pub const EM_SETBKGNDCOLOR: u32 = 1091;
pub const EM_SETCHARFORMAT: u32 = 1092;
pub const EM_SETCTFMODEBIAS: u32 = 1262;
pub const EM_SETCTFOPENSTATUS: u32 = 1265;
pub const EM_SETDISABLEOLELINKCONVERSION: u32 = 1428;
pub const EM_SETEDITSTYLE: u32 = 1228;
pub const EM_SETEDITSTYLEEX: u32 = 1299;
pub const EM_SETELLIPSISMODE: u32 = 1330;
pub const EM_SETEVENTMASK: u32 = 1093;
pub const EM_SETFONTSIZE: u32 = 1247;
pub const EM_SETHYPHENATEINFO: u32 = 1255;
pub const EM_SETIMECOLOR: u32 = 1128;
pub const EM_SETIMEMODEBIAS: u32 = 1150;
pub const EM_SETIMEOPTIONS: u32 = 1130;
pub const EM_SETLANGOPTIONS: u32 = 1144;
pub const EM_SETOLECALLBACK: u32 = 1094;
pub const EM_SETOPTIONS: u32 = 1101;
pub const EM_SETPAGE: u32 = 1253;
pub const EM_SETPAGEROTATE: u32 = 1260;
pub const EM_SETPALETTE: u32 = 1117;
pub const EM_SETPARAFORMAT: u32 = 1095;
pub const EM_SETPUNCTUATION: u32 = 1124;
pub const EM_SETQUERYCONVERTOLELINKCALLBACK: u32 = 1427;
pub const EM_SETQUERYRTFOBJ: u32 = 1294;
pub const EM_SETSCROLLPOS: u32 = 1246;
pub const EM_SETSTORYTYPE: u32 = 1315;
pub const EM_SETTABLEPARMS: u32 = 1331;
pub const EM_SETTARGETDEVICE: u32 = 1096;
pub const EM_SETTEXTEX: u32 = 1121;
pub const EM_SETTEXTMODE: u32 = 1113;
pub const EM_SETTOUCHOPTIONS: u32 = 1335;
pub const EM_SETTYPOGRAPHYOPTIONS: u32 = 1226;
pub const EM_SETUIANAME: u32 = 1344;
pub const EM_SETUNDOLIMIT: u32 = 1106;
pub const EM_SETVIEWKIND: u32 = 1251;
pub const EM_SETWORDBREAKPROCEX: u32 = 1105;
pub const EM_SETWORDWRAPMODE: u32 = 1126;
pub const EM_SHOWSCROLLBAR: u32 = 1120;
pub const EM_STOPGROUPTYPING: u32 = 1112;
pub const EM_STREAMIN: u32 = 1097;
pub const EM_STREAMOUT: u32 = 1098;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENCORRECTTEXT {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: u16,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENCORRECTTEXT {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: u16,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENDCOMPOSITIONNOTIFY {
    pub nmhdr: super::NMHDR,
    pub dwCode: u32,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENDCOMPOSITIONNOTIFY {
    pub nmhdr: super::NMHDR,
    pub dwCode: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct ENDROPFILES {
    pub nmhdr: super::NMHDR,
    pub hDrop: super::HANDLE,
    pub cp: i32,
    pub fProtected: windows_sys::core::BOOL,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for ENDROPFILES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct ENDROPFILES {
    pub nmhdr: super::NMHDR,
    pub hDrop: super::HANDLE,
    pub cp: i32,
    pub fProtected: windows_sys::core::BOOL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for ENDROPFILES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENLINK {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
    pub chrg: CHARRANGE,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENLINK {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
    pub chrg: CHARRANGE,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct ENLOWFIRTF {
    pub nmhdr: super::NMHDR,
    pub szControl: *mut i8,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for ENLOWFIRTF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct ENLOWFIRTF {
    pub nmhdr: super::NMHDR,
    pub szControl: *mut i8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for ENLOWFIRTF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENM_CHANGE: u32 = 1;
pub const ENM_CLIPFORMAT: u32 = 128;
pub const ENM_CORRECTTEXT: u32 = 4194304;
pub const ENM_DRAGDROPDONE: u32 = 16;
pub const ENM_DROPFILES: u32 = 1048576;
pub const ENM_ENDCOMPOSITION: u32 = 536870912;
pub const ENM_GROUPTYPINGCHANGE: u32 = 1073741824;
pub const ENM_HIDELINKTOOLTIP: u32 = 2147483648;
pub const ENM_IMECHANGE: u32 = 8388608;
pub const ENM_KEYEVENTS: u32 = 65536;
pub const ENM_LANGCHANGE: u32 = 16777216;
pub const ENM_LINK: u32 = 67108864;
pub const ENM_LOWFIRTF: u32 = 134217728;
pub const ENM_MOUSEEVENTS: u32 = 131072;
pub const ENM_NONE: u32 = 0;
pub const ENM_OBJECTPOSITIONS: u32 = 33554432;
pub const ENM_PAGECHANGE: u32 = 64;
pub const ENM_PARAGRAPHEXPANDED: u32 = 32;
pub const ENM_PROTECTED: u32 = 2097152;
pub const ENM_REQUESTRESIZE: u32 = 262144;
pub const ENM_SCROLL: u32 = 4;
pub const ENM_SCROLLEVENTS: u32 = 8;
pub const ENM_SELCHANGE: u32 = 524288;
pub const ENM_STARTCOMPOSITION: u32 = 268435456;
pub const ENM_UPDATE: u32 = 2;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENOLEOPFAILED {
    pub nmhdr: super::NMHDR,
    pub iob: i32,
    pub lOper: i32,
    pub hr: windows_sys::core::HRESULT,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENOLEOPFAILED {
    pub nmhdr: super::NMHDR,
    pub iob: i32,
    pub lOper: i32,
    pub hr: windows_sys::core::HRESULT,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENPROTECTED {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
    pub chrg: CHARRANGE,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENPROTECTED {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
    pub chrg: CHARRANGE,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENSAVECLIPBOARD {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub cch: i32,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENSAVECLIPBOARD {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub cch: i32,
}
pub const EN_ALIGNLTR: u32 = 1808;
pub const EN_ALIGNRTL: u32 = 1809;
pub const EN_CLIPFORMAT: u32 = 1810;
pub const EN_CORRECTTEXT: u32 = 1797;
pub const EN_DRAGDROPDONE: u32 = 1804;
pub const EN_DROPFILES: u32 = 1795;
pub const EN_ENDCOMPOSITION: u32 = 1812;
pub const EN_IMECHANGE: u32 = 1799;
pub const EN_LINK: u32 = 1803;
pub const EN_LOWFIRTF: u32 = 1807;
pub const EN_MSGFILTER: u32 = 1792;
pub const EN_OBJECTPOSITIONS: u32 = 1802;
pub const EN_OLEOPFAILED: u32 = 1801;
pub const EN_PAGECHANGE: u32 = 1806;
pub const EN_PARAGRAPHEXPANDED: u32 = 1805;
pub const EN_PROTECTED: u32 = 1796;
pub const EN_REQUESTRESIZE: u32 = 1793;
pub const EN_SAVECLIPBOARD: u32 = 1800;
pub const EN_SELCHANGE: u32 = 1794;
pub const EN_STARTCOMPOSITION: u32 = 1811;
pub const EN_STOPNOUNDO: u32 = 1798;
pub const EPR_0: u32 = 0;
pub const EPR_180: u32 = 2;
pub const EPR_270: u32 = 1;
pub const EPR_90: u32 = 3;
pub const EPR_SE: u32 = 5;
pub const ES_DISABLENOSCROLL: u32 = 8192;
pub const ES_EX_NOCALLOLEINIT: u32 = 0;
pub const ES_NOIME: u32 = 524288;
pub const ES_NOOLEDRAGDROP: u32 = 8;
pub const ES_SAVESEL: u32 = 32768;
pub const ES_SELECTIONBAR: u32 = 16777216;
pub const ES_SELFIME: u32 = 262144;
pub const ES_SUNKEN: u32 = 16384;
pub const ES_VERTICAL: u32 = 4194304;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FINDTEXTA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_sys::core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl Default for FINDTEXTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct FINDTEXTA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_sys::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FINDTEXTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FINDTEXTEXA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_sys::core::PCSTR,
    pub chrgText: CHARRANGE,
}
#[cfg(target_arch = "x86")]
impl Default for FINDTEXTEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct FINDTEXTEXA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_sys::core::PCSTR,
    pub chrgText: CHARRANGE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FINDTEXTEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FINDTEXTEXW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_sys::core::PCWSTR,
    pub chrgText: CHARRANGE,
}
#[cfg(target_arch = "x86")]
impl Default for FINDTEXTEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct FINDTEXTEXW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_sys::core::PCWSTR,
    pub chrgText: CHARRANGE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FINDTEXTEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FINDTEXTW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for FINDTEXTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct FINDTEXTW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FINDTEXTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct FORMATRANGE {
    pub hdc: super::HDC,
    pub hdcTarget: super::HDC,
    pub rc: super::RECT,
    pub rcPage: super::RECT,
    pub chrg: CHARRANGE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
impl Default for FORMATRANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct FORMATRANGE {
    pub hdc: super::HDC,
    pub hdcTarget: super::HDC,
    pub rc: super::RECT,
    pub rcPage: super::RECT,
    pub chrg: CHARRANGE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
impl Default for FORMATRANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GCMF_GRIPPER: u32 = 1;
pub const GCMF_MOUSEMENU: u32 = 8192;
pub const GCMF_SPELLING: u32 = 2;
pub const GCMF_TOUCHMENU: u32 = 16384;
pub const GCM_MOUSEMENU: u32 = 8192;
pub const GCM_RIGHTMOUSEDROP: u32 = 32768;
pub const GCM_TOUCHMENU: u32 = 16384;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct GETCONTEXTMENUEX {
    pub chrg: CHARRANGE,
    pub dwFlags: u32,
    pub pt: super::POINT,
    pub pvReserved: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
impl Default for GETCONTEXTMENUEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct GETCONTEXTMENUEX {
    pub chrg: CHARRANGE,
    pub dwFlags: u32,
    pub pt: super::POINT,
    pub pvReserved: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
impl Default for GETCONTEXTMENUEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct GETTEXTEX {
    pub cb: u32,
    pub flags: u32,
    pub codepage: u32,
    pub lpDefaultChar: windows_sys::core::PCSTR,
    pub lpUsedDefChar: super::LPBOOL,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
impl Default for GETTEXTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct GETTEXTEX {
    pub cb: u32,
    pub flags: u32,
    pub codepage: u32,
    pub lpDefaultChar: windows_sys::core::PCSTR,
    pub lpUsedDefChar: super::LPBOOL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
impl Default for GETTEXTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GETTEXTLENGTHEX {
    pub flags: u32,
    pub codepage: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct GROUPTYPINGCHANGE {
    pub nmhdr: super::NMHDR,
    pub fGroupTyping: windows_sys::core::BOOL,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct GROUPTYPINGCHANGE {
    pub nmhdr: super::NMHDR,
    pub fGroupTyping: windows_sys::core::BOOL,
}
pub const GTL_CLOSE: u32 = 4;
pub const GTL_DEFAULT: u32 = 0;
pub const GTL_NUMBYTES: u32 = 16;
pub const GTL_NUMCHARS: u32 = 8;
pub const GTL_PRECISE: u32 = 2;
pub const GTL_USECRLF: u32 = 1;
pub const GT_DEFAULT: u32 = 0;
pub const GT_NOHIDDENTEXT: u32 = 8;
pub const GT_RAWTEXT: u32 = 4;
pub const GT_SELECTION: u32 = 2;
pub const GT_USECRLF: u32 = 1;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct HYPHENATEINFO {
    pub cbSize: i16,
    pub dxHyphenateZone: i16,
    pub pfnHyphenate: *mut u8,
}
#[cfg(target_arch = "x86")]
impl Default for HYPHENATEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct HYPHENATEINFO {
    pub cbSize: i16,
    pub dxHyphenateZone: i16,
    pub pfnHyphenate: *mut u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for HYPHENATEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HYPHRESULT {
    pub khyph: KHYPH,
    pub ichHyph: i32,
    pub chHyph: u16,
}
pub const ICM_CTF: u32 = 5;
pub const ICM_LEVEL2: u32 = 2;
pub const ICM_LEVEL2_5: u32 = 3;
pub const ICM_LEVEL2_SUI: u32 = 4;
pub const ICM_LEVEL3: u32 = 1;
pub const ICM_NOTOPEN: u32 = 0;
pub const ICT_RESULTREADSTR: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IMECOMPTEXT {
    pub cb: i32,
    pub flags: u32,
}
pub const IMF_AUTOFONT: u32 = 2;
pub const IMF_AUTOFONTSIZEADJUST: u32 = 16;
pub const IMF_AUTOKEYBOARD: u32 = 1;
pub const IMF_CLOSESTATUSWINDOW: u32 = 8;
pub const IMF_DUALFONT: u32 = 128;
pub const IMF_FORCEACTIVE: u32 = 64;
pub const IMF_FORCEDISABLE: u32 = 4;
pub const IMF_FORCEENABLE: u32 = 2;
pub const IMF_FORCEINACTIVE: u32 = 128;
pub const IMF_FORCENONE: u32 = 1;
pub const IMF_FORCEREMEMBER: u32 = 256;
pub const IMF_IMEALWAYSSENDNOTIFY: u32 = 8;
pub const IMF_IMECANCELCOMPLETE: u32 = 4;
pub const IMF_IMEUIINTEGRATION: u32 = 8192;
pub const IMF_MULTIPLEEDIT: u32 = 1024;
pub const IMF_NOIMPLICITLANG: u32 = 64;
pub const IMF_NOKBDLIDFIXUP: u32 = 512;
pub const IMF_NORTFFONTSUBSTITUTE: u32 = 1024;
pub const IMF_SMODE_NONE: u32 = 2;
pub const IMF_SMODE_PLAURALCLAUSE: u32 = 1;
pub const IMF_SPELLCHECKING: u32 = 2048;
pub const IMF_TKBPREDICTION: u32 = 4096;
pub const IMF_UIFONTS: u32 = 32;
pub const IMF_VERTICAL: u32 = 32;
pub type KHYPH = i32;
pub const MAX_TABLE_CELLS: u32 = 63;
pub const MAX_TAB_STOPS: u32 = 32;
pub const MSFTEDIT_CLASS: windows_sys::core::PCWSTR = windows_sys::core::w!("RICHEDIT50W");
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct MSGFILTER {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct MSGFILTER {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct OBJECTPOSITIONS {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub pcpPositions: *mut i32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for OBJECTPOSITIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct OBJECTPOSITIONS {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub pcpPositions: *mut i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winuser"))]
impl Default for OBJECTPOSITIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OLEOP_DOVERB: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PARAFORMAT {
    pub cbSize: u32,
    pub dwMask: u32,
    pub wNumbering: u16,
    pub Anonymous: PARAFORMAT_0,
    pub dxStartIndent: i32,
    pub dxRightIndent: i32,
    pub dxOffset: i32,
    pub wAlignment: u16,
    pub cTabCount: i16,
    pub rgxTabs: [i32; 32],
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
    pub wShadingStyle: u16,
    pub wNumberingStart: u16,
    pub wNumberingStyle: u16,
    pub wNumberingTab: u16,
    pub wBorderSpace: u16,
    pub wBorderWidth: u16,
    pub wBorders: u16,
}
impl Default for PARAFORMAT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PC_DELIMITER: u32 = 4;
pub const PC_FOLLOWING: u32 = 1;
pub const PC_LEADING: u32 = 2;
pub const PC_OVERFLOW: u32 = 3;
pub const PFA_CENTER: u32 = 3;
pub const PFA_FULL_GLYPHS: u32 = 8;
pub const PFA_FULL_INTERLETTER: u32 = 6;
pub const PFA_FULL_INTERWORD: u32 = 4;
pub const PFA_FULL_NEWSPAPER: u32 = 5;
pub const PFA_FULL_SCALED: u32 = 7;
pub const PFA_JUSTIFY: u32 = 4;
pub const PFA_LEFT: u32 = 1;
pub const PFA_RIGHT: u32 = 2;
pub const PFE_BOX: u32 = 1024;
pub const PFE_COLLAPSED: u32 = 256;
pub const PFE_DONOTHYPHEN: u32 = 64;
pub const PFE_KEEP: u32 = 2;
pub const PFE_KEEPNEXT: u32 = 4;
pub const PFE_NOLINENUMBER: u32 = 16;
pub const PFE_NOWIDOWCONTROL: u32 = 32;
pub const PFE_PAGEBREAKBEFORE: u32 = 8;
pub const PFE_RTLPARA: u32 = 1;
pub const PFE_SIDEBYSIDE: u32 = 128;
pub const PFE_TABLE: u32 = 16384;
pub const PFE_TABLEROWDELIMITER: u32 = 4096;
pub const PFE_TEXTWRAPPINGBREAK: u32 = 8192;
pub const PFM_ALIGNMENT: u32 = 8;
pub const PFM_ALL: i32 = -2147418049;
pub const PFM_ALL2: i32 = -788529665;
pub const PFM_BORDER: u32 = 2048;
pub const PFM_BOX: u32 = 67108864;
pub const PFM_COLLAPSED: u32 = 16777216;
pub const PFM_DONOTHYPHEN: u32 = 4194304;
pub const PFM_EFFECTS: u32 = 1358888960;
pub const PFM_KEEP: u32 = 131072;
pub const PFM_KEEPNEXT: u32 = 262144;
pub const PFM_LINESPACING: u32 = 256;
pub const PFM_NOLINENUMBER: u32 = 1048576;
pub const PFM_NOWIDOWCONTROL: u32 = 2097152;
pub const PFM_NUMBERING: u32 = 32;
pub const PFM_NUMBERINGSTART: u32 = 32768;
pub const PFM_NUMBERINGSTYLE: u32 = 8192;
pub const PFM_NUMBERINGTAB: u32 = 16384;
pub const PFM_OFFSET: u32 = 4;
pub const PFM_OFFSETINDENT: u32 = 2147483648;
pub const PFM_OUTLINELEVEL: u32 = 33554432;
pub const PFM_PAGEBREAKBEFORE: u32 = 524288;
pub const PFM_RESERVED2: u32 = 134217728;
pub const PFM_RIGHTINDENT: u32 = 2;
pub const PFM_RTLPARA: u32 = 65536;
pub const PFM_SHADING: u32 = 4096;
pub const PFM_SIDEBYSIDE: u32 = 8388608;
pub const PFM_SPACEAFTER: u32 = 128;
pub const PFM_SPACEBEFORE: u32 = 64;
pub const PFM_STARTINDENT: u32 = 1;
pub const PFM_STYLE: u32 = 1024;
pub const PFM_TABLE: u32 = 1073741824;
pub const PFM_TABLEROWDELIMITER: u32 = 268435456;
pub const PFM_TABSTOPS: u32 = 16;
pub const PFM_TEXTWRAPPINGBREAK: u32 = 536870912;
pub const PFNS_NEWNUMBER: u32 = 32768;
pub const PFNS_NONUMBER: u32 = 1024;
pub const PFNS_PAREN: u32 = 0;
pub const PFNS_PARENS: u32 = 256;
pub const PFNS_PERIOD: u32 = 512;
pub const PFNS_PLAIN: u32 = 768;
pub const PFN_ARABIC: u32 = 2;
pub const PFN_BULLET: u32 = 1;
pub const PFN_LCLETTER: u32 = 3;
pub const PFN_LCROMAN: u32 = 5;
pub const PFN_UCLETTER: u32 = 4;
pub const PFN_UCROMAN: u32 = 6;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct PUNCTUATION {
    pub iSize: u32,
    pub szPunctuation: windows_sys::core::PSTR,
}
#[cfg(target_arch = "x86")]
impl Default for PUNCTUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct PUNCTUATION {
    pub iSize: u32,
    pub szPunctuation: windows_sys::core::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PUNCTUATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct REPASTESPECIAL {
    pub dwAspect: u32,
    pub dwParam: usize,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct REPASTESPECIAL {
    pub dwAspect: u32,
    pub dwParam: usize,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct REQRESIZE {
    pub nmhdr: super::NMHDR,
    pub rc: super::RECT,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct REQRESIZE {
    pub nmhdr: super::NMHDR,
    pub rc: super::RECT,
}
pub const RICHEDIT60_CLASS: windows_sys::core::PCWSTR = windows_sys::core::w!("RICHEDIT60W");
pub const RICHEDIT_CLASS10A: windows_sys::core::PCSTR = windows_sys::core::s!("RICHEDIT");
pub const RICHEDIT_CLASSA: windows_sys::core::PCSTR = windows_sys::core::s!("RichEdit20A");
pub const RICHEDIT_CLASSW: windows_sys::core::PCWSTR = windows_sys::core::w!("RichEdit20W");
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Copy)]
pub struct RICHEDIT_IMAGE_PARAMETERS {
    pub xWidth: i32,
    pub yHeight: i32,
    pub Ascent: i32,
    pub Type: i32,
    pub pwszAlternateText: windows_sys::core::PCWSTR,
    pub pIStream: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "objidlbase")]
impl Default for RICHEDIT_IMAGE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Copy)]
pub struct RICHEDIT_IMAGE_PARAMETERS {
    pub xWidth: i32,
    pub yHeight: i32,
    pub Ascent: i32,
    pub Type: i32,
    pub pwszAlternateText: windows_sys::core::PCWSTR,
    pub pIStream: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "objidlbase")]
impl Default for RICHEDIT_IMAGE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RTO_DISABLEHANDLES: u32 = 2;
pub const RTO_READINGMODE: u32 = 3;
pub const RTO_SHOWHANDLES: u32 = 1;
pub const SCF_ALL: u32 = 4;
pub const SCF_ASSOCIATEFONT: u32 = 16;
pub const SCF_ASSOCIATEFONT2: u32 = 64;
pub const SCF_CHARREPFROMLCID: u32 = 256;
pub const SCF_DEFAULT: u32 = 0;
pub const SCF_NOKBUPDATE: u32 = 32;
pub const SCF_SELECTION: u32 = 1;
pub const SCF_SMARTFONT: u32 = 128;
pub const SCF_USEUIRULES: u32 = 8;
pub const SCF_WORD: u32 = 2;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct SELCHANGE {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: u16,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct SELCHANGE {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: u16,
}
pub const SEL_EMPTY: u32 = 0;
pub const SEL_MULTICHAR: u32 = 4;
pub const SEL_MULTIOBJECT: u32 = 8;
pub const SEL_OBJECT: u32 = 2;
pub const SEL_TEXT: u32 = 1;
pub const SES_ALLOWBEEPS: u32 = 256;
pub const SES_BEEPONMAXTEXT: u32 = 2;
pub const SES_BIDI: u32 = 4096;
pub const SES_CTFALLOWEMBED: u32 = 2097152;
pub const SES_CTFALLOWPROOFING: u32 = 8388608;
pub const SES_CTFALLOWSMARTTAG: u32 = 4194304;
pub const SES_CTFNOLOCK: u32 = 268435456;
pub const SES_CUSTOMLOOK: u32 = 524288;
pub const SES_DEFAULTLATINLIGA: u32 = 16;
pub const SES_DRAFTMODE: u32 = 32768;
pub const SES_EMULATE10: u32 = 16;
pub const SES_EMULATESYSEDIT: u32 = 1;
pub const SES_EXTENDBACKCOLOR: u32 = 4;
pub const SES_EX_HANDLEFRIENDLYURL: u32 = 256;
pub const SES_EX_HIDETEMPFORMAT: u32 = 268435456;
pub const SES_EX_MULTITOUCH: u32 = 134217728;
pub const SES_EX_NOACETATESELECTION: u32 = 1048576;
pub const SES_EX_NOMATH: u32 = 64;
pub const SES_EX_NOTABLE: u32 = 4;
pub const SES_EX_NOTHEMING: u32 = 524288;
pub const SES_EX_USEMOUSEWPARAM: u32 = 536870912;
pub const SES_EX_USESINGLELINE: u32 = 2097152;
pub const SES_HIDEGRIDLINES: u32 = 131072;
pub const SES_HYPERLINKTOOLTIPS: u32 = 8;
pub const SES_LBSCROLLNOTIFY: u32 = 1048576;
pub const SES_LOGICALCARET: u32 = 16777216;
pub const SES_LOWERCASE: u32 = 1024;
pub const SES_MAPCPS: u32 = 8;
pub const SES_MAX: u32 = 536870912;
pub const SES_MULTISELECT: u32 = 134217728;
pub const SES_NOEALINEHEIGHTADJUST: u32 = 536870912;
pub const SES_NOFOCUSLINKNOTIFY: u32 = 32;
pub const SES_NOIME: u32 = 128;
pub const SES_NOINPUTSEQUENCECHK: u32 = 2048;
pub const SES_SCROLLONKILLFOCUS: u32 = 8192;
pub const SES_SMARTDRAGDROP: u32 = 67108864;
pub const SES_UPPERCASE: u32 = 512;
pub const SES_USEAIMM: u32 = 64;
pub const SES_USEATFONT: u32 = 262144;
pub const SES_USECRLF: u32 = 32;
pub const SES_USECTF: u32 = 65536;
pub const SES_WORDDRAGDROP: u32 = 33554432;
pub const SES_XLTCRCRLFTOCR: u32 = 16384;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SETTEXTEX {
    pub flags: u32,
    pub codepage: u32,
}
pub const SFF_KEEPDOCINFO: u32 = 4096;
pub const SFF_PERSISTVIEWSCALE: u32 = 8192;
pub const SFF_PLAINRTF: u32 = 16384;
pub const SFF_PWD: u32 = 2048;
pub const SFF_SELECTION: u32 = 32768;
pub const SFF_WRITEXTRAPAR: u32 = 128;
pub const SF_NCRFORNONASCII: u32 = 64;
pub const SF_RTF: u32 = 2;
pub const SF_RTFNOOBJS: u32 = 3;
pub const SF_RTFVAL: u32 = 1792;
pub const SF_TEXT: u32 = 1;
pub const SF_TEXTIZED: u32 = 4;
pub const SF_UNICODE: u32 = 16;
pub const SF_USECODEPAGE: u32 = 32;
pub const SPF_DONTSETDEFAULT: u32 = 2;
pub const SPF_SETDEFAULT: u32 = 4;
pub const ST_DEFAULT: u32 = 0;
pub const ST_KEEPUNDO: u32 = 1;
pub const ST_NEWCHARS: u32 = 4;
pub const ST_SELECTION: u32 = 2;
pub const ST_UNICODE: u32 = 8;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct TABLECELLPARMS {
    pub dxWidth: i32,
    pub _bitfield: u16,
    pub wShading: u16,
    pub dxBrdrLeft: i16,
    pub dyBrdrTop: i16,
    pub dxBrdrRight: i16,
    pub dyBrdrBottom: i16,
    pub crBrdrLeft: super::COLORREF,
    pub crBrdrTop: super::COLORREF,
    pub crBrdrRight: super::COLORREF,
    pub crBrdrBottom: super::COLORREF,
    pub crBackPat: super::COLORREF,
    pub crForePat: super::COLORREF,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub type TEXTMODE = i32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct TEXTRANGEA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_sys::core::PSTR,
}
#[cfg(target_arch = "x86")]
impl Default for TEXTRANGEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct TEXTRANGEA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_sys::core::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for TEXTRANGEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct TEXTRANGEW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_sys::core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for TEXTRANGEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct TEXTRANGEW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_sys::core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for TEXTRANGEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TM_MULTICODEPAGE: TEXTMODE = 32;
pub const TM_MULTILEVELUNDO: TEXTMODE = 8;
pub const TM_PLAINTEXT: TEXTMODE = 1;
pub const TM_RICHTEXT: TEXTMODE = 2;
pub const TM_SINGLECODEPAGE: TEXTMODE = 16;
pub const TM_SINGLELEVELUNDO: TEXTMODE = 4;
pub const TO_ADVANCEDLAYOUT: u32 = 8;
pub const TO_ADVANCEDTYPOGRAPHY: u32 = 1;
pub const TO_DISABLECUSTOMTEXTOUT: u32 = 4;
pub const TO_SIMPLELINEBREAK: u32 = 2;
pub const UID_AUTOTABLE: UNDONAMEID = 6;
pub const UID_CUT: UNDONAMEID = 4;
pub const UID_DELETE: UNDONAMEID = 2;
pub const UID_DRAGDROP: UNDONAMEID = 3;
pub const UID_PASTE: UNDONAMEID = 5;
pub const UID_TYPING: UNDONAMEID = 1;
pub const UID_UNKNOWN: UNDONAMEID = 0;
pub type UNDONAMEID = i32;
pub const VM_NORMAL: u32 = 4;
pub const VM_OUTLINE: u32 = 2;
pub const VM_PAGE: u32 = 9;
pub const WBF_BREAKAFTER: u8 = 64;
pub const WBF_BREAKLINE: u8 = 32;
pub const WBF_CLASS: u8 = 15;
pub const WBF_CUSTOM: u32 = 512;
pub const WBF_ISWHITE: u8 = 16;
pub const WBF_LEVEL1: u32 = 128;
pub const WBF_LEVEL2: u32 = 256;
pub const WBF_OVERFLOW: u32 = 64;
pub const WBF_WORDBREAK: u32 = 32;
pub const WBF_WORDWRAP: u32 = 16;
pub const WB_CLASSIFY: u32 = 3;
pub const WB_LEFTBREAK: u32 = 6;
pub const WB_MOVEWORDLEFT: u32 = 4;
pub const WB_MOVEWORDNEXT: u32 = 5;
pub const WB_MOVEWORDPREV: u32 = 4;
pub const WB_MOVEWORDRIGHT: u32 = 5;
pub const WB_NEXTBREAK: u32 = 7;
pub const WB_PREVBREAK: u32 = 6;
pub const WB_RIGHTBREAK: u32 = 7;
pub const WCH_EMBEDDING: u16 = 65532;
pub const cchTextLimitDefault: u32 = 32767;
pub const khyphAddBefore: KHYPH = 2;
pub const khyphChangeAfter: KHYPH = 5;
pub const khyphChangeBefore: KHYPH = 3;
pub const khyphDelAndChange: KHYPH = 6;
pub const khyphDeleteBefore: KHYPH = 4;
pub const khyphNil: KHYPH = 0;
pub const khyphNormal: KHYPH = 1;
pub const lDefaultTab: u32 = 720;
pub const yHeightCharPtsMost: u32 = 1638;
