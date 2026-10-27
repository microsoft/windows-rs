pub const ATP_CHANGE: i32 = 1;
pub const ATP_NOCHANGE: i32 = 0;
pub const ATP_NODELIMITER: i32 = 2;
pub const ATP_REPLACEALLTEXT: i32 = 4;
pub const AURL_DISABLEMIXEDLGC: i32 = 32;
pub const AURL_ENABLEDRIVELETTERS: i32 = 16;
pub const AURL_ENABLEEA: i32 = 1;
pub const AURL_ENABLEEAURLS: i32 = 8;
pub const AURL_ENABLEEMAILADDR: i32 = 2;
pub const AURL_ENABLETELNO: i32 = 4;
pub const AURL_ENABLEURL: i32 = 1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type AutoCorrectProc = Option<unsafe extern "system" fn(langid: super::LANGID, pszbefore: *const super::WCHAR, pszafter: *mut super::WCHAR, cchafter: super::LONG, pcchreplaced: *mut super::LONG) -> i32>;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BIDIOPTIONS {
    pub cbSize: super::UINT,
    pub wMask: super::WORD,
    pub wEffects: super::WORD,
}
pub const BOE_CONTEXTALIGNMENT: i32 = 16;
pub const BOE_CONTEXTREADING: i32 = 8;
pub const BOE_FORCERECALC: i32 = 32;
pub const BOE_LEGACYBIDICLASS: i32 = 64;
pub const BOE_NEUTRALOVERRIDE: i32 = 4;
pub const BOE_UNICODEBIDI: i32 = 128;
pub const BOM_CONTEXTALIGNMENT: i32 = 16;
pub const BOM_CONTEXTREADING: i32 = 8;
pub const BOM_LEGACYBIDICLASS: i32 = 64;
pub const BOM_NEUTRALOVERRIDE: i32 = 4;
pub const BOM_UNICODEBIDI: i32 = 128;
pub const CERICHEDIT_CLASSA: windows_core::PCSTR = windows_core::s!("RichEditCEA");
pub const CERICHEDIT_CLASSW: windows_core::PCWSTR = windows_core::w!("RichEditCEW");
pub const CFE_ALLCAPS: i32 = 128;
pub const CFE_AUTOBACKCOLOR: i32 = 67108864;
pub const CFE_AUTOCOLOR: i32 = 1073741824;
pub const CFE_BOLD: i32 = 1;
pub const CFE_DISABLED: i32 = 8192;
pub const CFE_EMBOSS: i32 = 2048;
pub const CFE_EXTENDED: i32 = 33554432;
pub const CFE_FONTBOUND: i32 = 1048576;
pub const CFE_HIDDEN: i32 = 256;
pub const CFE_IMPRINT: i32 = 4096;
pub const CFE_ITALIC: i32 = 2;
pub const CFE_LINK: i32 = 32;
pub const CFE_LINKPROTECTED: i32 = 8388608;
pub const CFE_MATH: i32 = 268435456;
pub const CFE_MATHNOBUILDUP: i32 = 134217728;
pub const CFE_MATHORDINARY: i32 = 536870912;
pub const CFE_OUTLINE: i32 = 512;
pub const CFE_PROTECTED: i32 = 16;
pub const CFE_REVISED: i32 = 16384;
pub const CFE_SHADOW: i32 = 1024;
pub const CFE_SMALLCAPS: i32 = 64;
pub const CFE_STRIKEOUT: i32 = 8;
pub const CFE_SUBSCRIPT: i32 = 65536;
pub const CFE_SUPERSCRIPT: i32 = 131072;
pub const CFE_UNDERLINE: i32 = 4;
pub const CFM_ALL: u32 = 4160749631;
pub const CFM_ALL2: u32 = 4294967295;
pub const CFM_ALLCAPS: i32 = 128;
pub const CFM_ALLEFFECTS: i32 = 2115207167;
pub const CFM_ANIMATION: i32 = 262144;
pub const CFM_BACKCOLOR: i32 = 67108864;
pub const CFM_BOLD: i32 = 1;
pub const CFM_CHARSET: i32 = 134217728;
pub const CFM_COLOR: i32 = 1073741824;
pub const CFM_COOKIE: i32 = 16777216;
pub const CFM_DISABLED: i32 = 8192;
pub const CFM_EFFECTS: i32 = 1073741887;
pub const CFM_EFFECTS2: i32 = 1141080063;
pub const CFM_EMBOSS: i32 = 2048;
pub const CFM_EXTENDED: i32 = 33554432;
pub const CFM_FACE: i32 = 536870912;
pub const CFM_FONTBOUND: i32 = 1048576;
pub const CFM_HIDDEN: i32 = 256;
pub const CFM_IMPRINT: i32 = 4096;
pub const CFM_ITALIC: i32 = 2;
pub const CFM_KERNING: i32 = 1048576;
pub const CFM_LCID: i32 = 33554432;
pub const CFM_LINK: i32 = 32;
pub const CFM_LINKPROTECTED: i32 = 8388608;
pub const CFM_MATH: i32 = 268435456;
pub const CFM_MATHNOBUILDUP: i32 = 134217728;
pub const CFM_MATHORDINARY: i32 = 536870912;
pub const CFM_OFFSET: i32 = 268435456;
pub const CFM_OUTLINE: i32 = 512;
pub const CFM_PROTECTED: i32 = 16;
pub const CFM_REVAUTHOR: i32 = 32768;
pub const CFM_REVISED: i32 = 16384;
pub const CFM_SHADOW: i32 = 1024;
pub const CFM_SIZE: u32 = 2147483648;
pub const CFM_SMALLCAPS: i32 = 64;
pub const CFM_SPACING: i32 = 2097152;
pub const CFM_STRIKEOUT: i32 = 8;
pub const CFM_STYLE: i32 = 524288;
pub const CFM_SUBSCRIPT: i32 = 196608;
pub const CFM_SUPERSCRIPT: i32 = 196608;
pub const CFM_UNDERLINE: i32 = 4;
pub const CFM_UNDERLINETYPE: i32 = 8388608;
pub const CFM_WEIGHT: i32 = 4194304;
pub const CFU_CF1UNDERLINE: i32 = 255;
pub const CFU_INVERT: i32 = 254;
pub const CFU_UNDERLINE: i32 = 1;
pub const CFU_UNDERLINEDASH: i32 = 5;
pub const CFU_UNDERLINEDASHDOT: i32 = 6;
pub const CFU_UNDERLINEDASHDOTDOT: i32 = 7;
pub const CFU_UNDERLINEDOTTED: i32 = 4;
pub const CFU_UNDERLINEDOUBLE: i32 = 3;
pub const CFU_UNDERLINEDOUBLEWAVE: i32 = 11;
pub const CFU_UNDERLINEHAIRLINE: i32 = 10;
pub const CFU_UNDERLINEHEAVYWAVE: i32 = 12;
pub const CFU_UNDERLINELONGDASH: i32 = 13;
pub const CFU_UNDERLINENONE: i32 = 0;
pub const CFU_UNDERLINETHICK: i32 = 9;
pub const CFU_UNDERLINETHICKDASH: i32 = 14;
pub const CFU_UNDERLINETHICKDASHDOT: i32 = 15;
pub const CFU_UNDERLINETHICKDASHDOTDOT: i32 = 16;
pub const CFU_UNDERLINETHICKDOTTED: i32 = 17;
pub const CFU_UNDERLINETHICKLONGDASH: i32 = 18;
pub const CFU_UNDERLINEWAVE: i32 = 8;
pub const CFU_UNDERLINEWORD: i32 = 2;
pub const CF_RETEXTOBJ: windows_core::PCSTR = windows_core::s!("RichEdit Text and Objects");
pub const CF_RTF: windows_core::PCSTR = windows_core::s!("Rich Text Format");
pub const CF_RTFNOOBJS: windows_core::PCSTR = windows_core::s!("Rich Text Format Without Objects");
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct CHARFORMAT2A {
    pub base: CHARFORMATA,
    pub wWeight: super::WORD,
    pub sSpacing: super::SHORT,
    pub crBackColor: super::COLORREF,
    pub lcid: super::LCID,
    pub Anonymous: CHARFORMAT2A_0,
    pub sStyle: super::SHORT,
    pub wKerning: super::WORD,
    pub bUnderlineType: super::BYTE,
    pub bAnimation: super::BYTE,
    pub bRevAuthor: super::BYTE,
    pub bUnderlineColor: super::BYTE,
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for CHARFORMAT2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub union CHARFORMAT2A_0 {
    pub dwReserved: super::DWORD,
    pub dwCookie: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for CHARFORMAT2A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CHARFORMAT2W {
    pub base: CHARFORMATW,
    pub wWeight: super::WORD,
    pub sSpacing: super::SHORT,
    pub crBackColor: super::COLORREF,
    pub lcid: super::LCID,
    pub Anonymous: CHARFORMAT2W_0,
    pub sStyle: super::SHORT,
    pub wKerning: super::WORD,
    pub bUnderlineType: super::BYTE,
    pub bAnimation: super::BYTE,
    pub bRevAuthor: super::BYTE,
    pub bUnderlineColor: super::BYTE,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for CHARFORMAT2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union CHARFORMAT2W_0 {
    pub dwReserved: super::DWORD,
    pub dwCookie: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for CHARFORMAT2W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct CHARFORMATA {
    pub cbSize: super::UINT,
    pub dwMask: super::DWORD,
    pub dwEffects: super::DWORD,
    pub yHeight: super::LONG,
    pub yOffset: super::LONG,
    pub crTextColor: super::COLORREF,
    pub bCharSet: super::BYTE,
    pub bPitchAndFamily: super::BYTE,
    pub szFaceName: [i8; 32],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for CHARFORMATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const CHARFORMATDELTA: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CHARFORMATDELTA: u64 = 24;
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CHARFORMATW {
    pub cbSize: super::UINT,
    pub dwMask: super::DWORD,
    pub dwEffects: super::DWORD,
    pub yHeight: super::LONG,
    pub yOffset: super::LONG,
    pub crTextColor: super::COLORREF,
    pub bCharSet: super::BYTE,
    pub bPitchAndFamily: super::BYTE,
    pub szFaceName: [super::WCHAR; 32],
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for CHARFORMATW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CHARRANGE {
    pub cpMin: super::LONG,
    pub cpMax: super::LONG,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winuser", feature = "wtypes"))]
#[derive(Clone, Copy, Default)]
pub struct CLIPBOARDFORMAT {
    pub nmhdr: super::NMHDR,
    pub cf: super::CLIPFORMAT,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct COMPCOLOR {
    pub crText: super::COLORREF,
    pub crBackground: super::COLORREF,
    pub dwEffects: super::DWORD,
}
pub const CTFMODEBIAS_CONVERSATION: i32 = 5;
pub const CTFMODEBIAS_DATETIME: i32 = 4;
pub const CTFMODEBIAS_DEFAULT: i32 = 0;
pub const CTFMODEBIAS_FILENAME: i32 = 1;
pub const CTFMODEBIAS_FULLWIDTHALPHANUMERIC: i32 = 11;
pub const CTFMODEBIAS_HALFWIDTHALPHANUMERIC: i32 = 12;
pub const CTFMODEBIAS_HALFWIDTHKATAKANA: i32 = 10;
pub const CTFMODEBIAS_HANGUL: i32 = 9;
pub const CTFMODEBIAS_HIRAGANA: i32 = 7;
pub const CTFMODEBIAS_KATAKANA: i32 = 8;
pub const CTFMODEBIAS_NAME: i32 = 2;
pub const CTFMODEBIAS_NUMERIC: i32 = 6;
pub const CTFMODEBIAS_READING: i32 = 3;
pub const ECN_ENDCOMPOSITION: i32 = 1;
pub const ECN_NEWTEXT: i32 = 2;
pub const ECOOP_AND: i32 = 3;
pub const ECOOP_OR: i32 = 2;
pub const ECOOP_SET: i32 = 1;
pub const ECOOP_XOR: i32 = 4;
pub const ECO_AUTOHSCROLL: i32 = 128;
pub const ECO_AUTOVSCROLL: i32 = 64;
pub const ECO_AUTOWORDSELECTION: i32 = 1;
pub const ECO_NOHIDESEL: i32 = 256;
pub const ECO_READONLY: i32 = 2048;
pub const ECO_SAVESEL: i32 = 32768;
pub const ECO_SELECTIONBAR: i32 = 16777216;
pub const ECO_VERTICAL: i32 = 4194304;
pub const ECO_WANTRETURN: i32 = 4096;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EDITSTREAM {
    pub dwCookie: super::DWORD_PTR,
    pub dwError: super::DWORD,
    pub pfnCallback: EDITSTREAMCALLBACK,
}
#[cfg(feature = "shared")]
pub type EDITSTREAMCALLBACK = Option<unsafe extern "system" fn(dwcookie: super::DWORD_PTR, pbbuff: super::LPBYTE, cb: super::LONG, pcb: *mut super::LONG) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type EDITWORDBREAKPROCEX = Option<unsafe extern "C" fn(pchtext: *mut i8, cchtext: super::LONG, bcharset: super::BYTE, action: super::INT) -> super::LONG>;
pub const ELLIPSIS_END: i32 = 1;
pub const ELLIPSIS_MASK: i32 = 3;
pub const ELLIPSIS_NONE: i32 = 0;
pub const ELLIPSIS_WORD: i32 = 3;
pub const EMO_ENTER: i32 = 1;
pub const EMO_EXIT: i32 = 0;
pub const EMO_EXPAND: i32 = 3;
pub const EMO_EXPANDDOCUMENT: i32 = 1;
pub const EMO_EXPANDSELECTION: i32 = 0;
pub const EMO_GETVIEWMODE: i32 = 5;
pub const EMO_MOVESELECTION: i32 = 4;
pub const EMO_PROMOTE: i32 = 2;
pub const EM_AUTOURLDETECT: i32 = 1115;
pub const EM_CALLAUTOCORRECTPROC: i32 = 1279;
pub const EM_CANPASTE: i32 = 1074;
pub const EM_CANREDO: i32 = 1109;
pub const EM_CONVPOSITION: i32 = 1132;
pub const EM_DISPLAYBAND: i32 = 1075;
pub const EM_EXGETSEL: i32 = 1076;
pub const EM_EXLIMITTEXT: i32 = 1077;
pub const EM_EXLINEFROMCHAR: i32 = 1078;
pub const EM_EXSETSEL: i32 = 1079;
pub const EM_FINDTEXT: i32 = 1080;
pub const EM_FINDTEXTEX: i32 = 1103;
pub const EM_FINDTEXTEXW: i32 = 1148;
pub const EM_FINDTEXTW: i32 = 1147;
pub const EM_FINDWORDBREAK: i32 = 1100;
pub const EM_FORMATRANGE: i32 = 1081;
pub const EM_GETAUTOCORRECTPROC: i32 = 1257;
pub const EM_GETAUTOURLDETECT: i32 = 1116;
pub const EM_GETBIDIOPTIONS: i32 = 1225;
pub const EM_GETCHARFORMAT: i32 = 1082;
pub const EM_GETCTFMODEBIAS: i32 = 1261;
pub const EM_GETCTFOPENSTATUS: i32 = 1264;
pub const EM_GETEDITSTYLE: i32 = 1229;
pub const EM_GETEDITSTYLEEX: i32 = 1300;
pub const EM_GETELLIPSISMODE: i32 = 1329;
pub const EM_GETELLIPSISSTATE: i32 = 1346;
pub const EM_GETEVENTMASK: i32 = 1083;
pub const EM_GETHYPHENATEINFO: i32 = 1254;
pub const EM_GETIMECOLOR: i32 = 1129;
pub const EM_GETIMECOMPMODE: i32 = 1146;
pub const EM_GETIMECOMPTEXT: i32 = 1266;
pub const EM_GETIMEMODEBIAS: i32 = 1151;
pub const EM_GETIMEOPTIONS: i32 = 1131;
pub const EM_GETIMEPROPERTY: i32 = 1268;
pub const EM_GETLANGOPTIONS: i32 = 1145;
pub const EM_GETOLEINTERFACE: i32 = 1084;
pub const EM_GETOPTIONS: i32 = 1102;
pub const EM_GETPAGE: i32 = 1252;
pub const EM_GETPAGEROTATE: i32 = 1259;
pub const EM_GETPARAFORMAT: i32 = 1085;
pub const EM_GETPUNCTUATION: i32 = 1125;
pub const EM_GETQUERYRTFOBJ: i32 = 1293;
pub const EM_GETREDONAME: i32 = 1111;
pub const EM_GETSCROLLPOS: i32 = 1245;
pub const EM_GETSELTEXT: i32 = 1086;
pub const EM_GETSTORYTYPE: i32 = 1314;
pub const EM_GETTABLEPARMS: i32 = 1289;
pub const EM_GETTEXTEX: i32 = 1118;
pub const EM_GETTEXTLENGTHEX: i32 = 1119;
pub const EM_GETTEXTMODE: i32 = 1114;
pub const EM_GETTEXTRANGE: i32 = 1099;
pub const EM_GETTOUCHOPTIONS: i32 = 1334;
pub const EM_GETTYPOGRAPHYOPTIONS: i32 = 1227;
pub const EM_GETUNDONAME: i32 = 1110;
pub const EM_GETVIEWKIND: i32 = 1250;
pub const EM_GETWORDBREAKPROCEX: i32 = 1104;
pub const EM_GETWORDWRAPMODE: i32 = 1127;
pub const EM_GETZOOM: i32 = 1248;
pub const EM_HIDESELECTION: i32 = 1087;
pub const EM_INSERTIMAGE: i32 = 1338;
pub const EM_INSERTTABLE: i32 = 1256;
pub const EM_ISIME: i32 = 1267;
pub const EM_OUTLINE: i32 = 1244;
pub const EM_PASTESPECIAL: i32 = 1088;
pub const EM_RECONVERSION: i32 = 1149;
pub const EM_REDO: i32 = 1108;
pub const EM_REQUESTRESIZE: i32 = 1089;
pub const EM_SELECTIONTYPE: i32 = 1090;
pub const EM_SETAUTOCORRECTPROC: i32 = 1258;
pub const EM_SETBIDIOPTIONS: i32 = 1224;
pub const EM_SETBKGNDCOLOR: i32 = 1091;
pub const EM_SETCHARFORMAT: i32 = 1092;
pub const EM_SETCTFMODEBIAS: i32 = 1262;
pub const EM_SETCTFOPENSTATUS: i32 = 1265;
pub const EM_SETDISABLEOLELINKCONVERSION: i32 = 1428;
pub const EM_SETEDITSTYLE: i32 = 1228;
pub const EM_SETEDITSTYLEEX: i32 = 1299;
pub const EM_SETELLIPSISMODE: i32 = 1330;
pub const EM_SETEVENTMASK: i32 = 1093;
pub const EM_SETFONTSIZE: i32 = 1247;
pub const EM_SETHYPHENATEINFO: i32 = 1255;
pub const EM_SETIMECOLOR: i32 = 1128;
pub const EM_SETIMEMODEBIAS: i32 = 1150;
pub const EM_SETIMEOPTIONS: i32 = 1130;
pub const EM_SETLANGOPTIONS: i32 = 1144;
pub const EM_SETOLECALLBACK: i32 = 1094;
pub const EM_SETOPTIONS: i32 = 1101;
pub const EM_SETPAGE: i32 = 1253;
pub const EM_SETPAGEROTATE: i32 = 1260;
pub const EM_SETPALETTE: i32 = 1117;
pub const EM_SETPARAFORMAT: i32 = 1095;
pub const EM_SETPUNCTUATION: i32 = 1124;
pub const EM_SETQUERYCONVERTOLELINKCALLBACK: i32 = 1427;
pub const EM_SETQUERYRTFOBJ: i32 = 1294;
pub const EM_SETSCROLLPOS: i32 = 1246;
pub const EM_SETSTORYTYPE: i32 = 1315;
pub const EM_SETTABLEPARMS: i32 = 1331;
pub const EM_SETTARGETDEVICE: i32 = 1096;
pub const EM_SETTEXTEX: i32 = 1121;
pub const EM_SETTEXTMODE: i32 = 1113;
pub const EM_SETTOUCHOPTIONS: i32 = 1335;
pub const EM_SETTYPOGRAPHYOPTIONS: i32 = 1226;
pub const EM_SETUIANAME: i32 = 1344;
pub const EM_SETUNDOLIMIT: i32 = 1106;
pub const EM_SETVIEWKIND: i32 = 1251;
pub const EM_SETWORDBREAKPROCEX: i32 = 1105;
pub const EM_SETWORDWRAPMODE: i32 = 1126;
pub const EM_SETZOOM: i32 = 1249;
pub const EM_SHOWSCROLLBAR: i32 = 1120;
pub const EM_STOPGROUPTYPING: i32 = 1112;
pub const EM_STREAMIN: i32 = 1097;
pub const EM_STREAMOUT: i32 = 1098;
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENCORRECTTEXT {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: super::WORD,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENDCOMPOSITIONNOTIFY {
    pub nmhdr: super::NMHDR,
    pub dwCode: super::DWORD,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENDROPFILES {
    pub nmhdr: super::NMHDR,
    pub hDrop: super::HANDLE,
    pub cp: super::LONG,
    pub fProtected: windows_core::BOOL,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENLINK {
    pub nmhdr: super::NMHDR,
    pub msg: super::UINT,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
    pub chrg: CHARRANGE,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENLOWFIRTF {
    pub nmhdr: super::NMHDR,
    pub szControl: *mut i8,
}
pub const ENM_CHANGE: i32 = 1;
pub const ENM_CLIPFORMAT: i32 = 128;
pub const ENM_CORRECTTEXT: i32 = 4194304;
pub const ENM_DRAGDROPDONE: i32 = 16;
pub const ENM_DROPFILES: i32 = 1048576;
pub const ENM_ENDCOMPOSITION: i32 = 536870912;
pub const ENM_GROUPTYPINGCHANGE: i32 = 1073741824;
pub const ENM_HIDELINKTOOLTIP: u32 = 2147483648;
pub const ENM_IMECHANGE: i32 = 8388608;
pub const ENM_KEYEVENTS: i32 = 65536;
pub const ENM_LANGCHANGE: i32 = 16777216;
pub const ENM_LINK: i32 = 67108864;
pub const ENM_LOWFIRTF: i32 = 134217728;
pub const ENM_MOUSEEVENTS: i32 = 131072;
pub const ENM_NONE: i32 = 0;
pub const ENM_OBJECTPOSITIONS: i32 = 33554432;
pub const ENM_PAGECHANGE: i32 = 64;
pub const ENM_PARAGRAPHEXPANDED: i32 = 32;
pub const ENM_PROTECTED: i32 = 2097152;
pub const ENM_REQUESTRESIZE: i32 = 262144;
pub const ENM_SCROLL: i32 = 4;
pub const ENM_SCROLLEVENTS: i32 = 8;
pub const ENM_SELCHANGE: i32 = 524288;
pub const ENM_STARTCOMPOSITION: i32 = 268435456;
pub const ENM_UPDATE: i32 = 2;
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENOLEOPFAILED {
    pub nmhdr: super::NMHDR,
    pub iob: super::LONG,
    pub lOper: super::LONG,
    pub hr: windows_core::HRESULT,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENPROTECTED {
    pub nmhdr: super::NMHDR,
    pub msg: super::UINT,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
    pub chrg: CHARRANGE,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct ENSAVECLIPBOARD {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: super::LONG,
    pub cch: super::LONG,
}
pub const EN_ALIGNLTR: i32 = 1808;
pub const EN_ALIGNRTL: i32 = 1809;
pub const EN_CLIPFORMAT: i32 = 1810;
pub const EN_CORRECTTEXT: i32 = 1797;
pub const EN_DRAGDROPDONE: i32 = 1804;
pub const EN_DROPFILES: i32 = 1795;
pub const EN_ENDCOMPOSITION: i32 = 1812;
pub const EN_IMECHANGE: i32 = 1799;
pub const EN_LINK: i32 = 1803;
pub const EN_LOWFIRTF: i32 = 1807;
pub const EN_MSGFILTER: i32 = 1792;
pub const EN_OBJECTPOSITIONS: i32 = 1802;
pub const EN_OLEOPFAILED: i32 = 1801;
pub const EN_PAGECHANGE: i32 = 1806;
pub const EN_PARAGRAPHEXPANDED: i32 = 1805;
pub const EN_PROTECTED: i32 = 1796;
pub const EN_REQUESTRESIZE: i32 = 1793;
pub const EN_SAVECLIPBOARD: i32 = 1800;
pub const EN_SELCHANGE: i32 = 1794;
pub const EN_STARTCOMPOSITION: i32 = 1811;
pub const EN_STOPNOUNDO: i32 = 1798;
pub const EPR_0: i32 = 0;
pub const EPR_180: i32 = 2;
pub const EPR_270: i32 = 1;
pub const EPR_90: i32 = 3;
pub const EPR_SE: i32 = 5;
pub const ES_DISABLENOSCROLL: i32 = 8192;
pub const ES_EX_NOCALLOLEINIT: i32 = 0;
pub const ES_NOIME: i32 = 524288;
pub const ES_NOOLEDRAGDROP: i32 = 8;
pub const ES_SAVESEL: i32 = 32768;
pub const ES_SELECTIONBAR: i32 = 16777216;
pub const ES_SELFIME: i32 = 262144;
pub const ES_SUNKEN: i32 = 16384;
pub const ES_VERTICAL: i32 = 4194304;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FINDTEXTA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PCSTR,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FINDTEXTEXA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PCSTR,
    pub chrgText: CHARRANGE,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FINDTEXTEXW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PCWSTR,
    pub chrgText: CHARRANGE,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FINDTEXTW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PCWSTR,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct FORMATRANGE {
    pub hdc: super::HDC,
    pub hdcTarget: super::HDC,
    pub rc: super::RECT,
    pub rcPage: super::RECT,
    pub chrg: CHARRANGE,
}
pub const GCMF_GRIPPER: i32 = 1;
pub const GCMF_MOUSEMENU: i32 = 8192;
pub const GCMF_SPELLING: i32 = 2;
pub const GCMF_TOUCHMENU: i32 = 16384;
pub const GCM_MOUSEMENU: i32 = 8192;
pub const GCM_RIGHTMOUSEDROP: i32 = 32768;
pub const GCM_TOUCHMENU: i32 = 16384;
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct GETCONTEXTMENUEX {
    pub chrg: CHARRANGE,
    pub dwFlags: super::DWORD,
    pub pt: super::POINT,
    pub pvReserved: *mut core::ffi::c_void,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct GETTEXTEX {
    pub cb: super::DWORD,
    pub flags: super::DWORD,
    pub codepage: super::UINT,
    pub lpDefaultChar: windows_core::PCSTR,
    pub lpUsedDefChar: super::LPBOOL,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct GETTEXTLENGTHEX {
    pub flags: super::DWORD,
    pub codepage: super::UINT,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct GROUPTYPINGCHANGE {
    pub nmhdr: super::NMHDR,
    pub fGroupTyping: windows_core::BOOL,
}
pub const GTL_CLOSE: i32 = 4;
pub const GTL_DEFAULT: i32 = 0;
pub const GTL_NUMBYTES: i32 = 16;
pub const GTL_NUMCHARS: i32 = 8;
pub const GTL_PRECISE: i32 = 2;
pub const GTL_USECRLF: i32 = 1;
pub const GT_DEFAULT: i32 = 0;
pub const GT_NOHIDDENTEXT: i32 = 8;
pub const GT_RAWTEXT: i32 = 4;
pub const GT_SELECTION: i32 = 2;
pub const GT_USECRLF: i32 = 1;
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HYPHENATEINFO {
    pub cbSize: super::SHORT,
    pub dxHyphenateZone: super::SHORT,
    pub pfnHyphenate: Option<unsafe extern "system" fn(*mut super::WCHAR, super::LANGID, i32, *mut HYPHRESULT)>,
}
#[repr(C, packed(4))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct HYPHRESULT {
    pub khyph: KHYPH,
    pub ichHyph: i32,
    pub chHyph: super::WCHAR,
}
pub const ICM_CTF: i32 = 5;
pub const ICM_LEVEL2: i32 = 2;
pub const ICM_LEVEL2_5: i32 = 3;
pub const ICM_LEVEL2_SUI: i32 = 4;
pub const ICM_LEVEL3: i32 = 1;
pub const ICM_NOTOPEN: i32 = 0;
pub const ICT_RESULTREADSTR: i32 = 1;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IMECOMPTEXT {
    pub cb: super::LONG,
    pub flags: super::DWORD,
}
pub const IMF_AUTOFONT: i32 = 2;
pub const IMF_AUTOFONTSIZEADJUST: i32 = 16;
pub const IMF_AUTOKEYBOARD: i32 = 1;
pub const IMF_CLOSESTATUSWINDOW: i32 = 8;
pub const IMF_DUALFONT: i32 = 128;
pub const IMF_FORCEACTIVE: i32 = 64;
pub const IMF_FORCEDISABLE: i32 = 4;
pub const IMF_FORCEENABLE: i32 = 2;
pub const IMF_FORCEINACTIVE: i32 = 128;
pub const IMF_FORCENONE: i32 = 1;
pub const IMF_FORCEREMEMBER: i32 = 256;
pub const IMF_IMEALWAYSSENDNOTIFY: i32 = 8;
pub const IMF_IMECANCELCOMPLETE: i32 = 4;
pub const IMF_IMEUIINTEGRATION: i32 = 8192;
pub const IMF_MULTIPLEEDIT: i32 = 1024;
pub const IMF_NOIMPLICITLANG: i32 = 64;
pub const IMF_NOKBDLIDFIXUP: i32 = 512;
pub const IMF_NORTFFONTSUBSTITUTE: i32 = 1024;
pub const IMF_SMODE_NONE: i32 = 2;
pub const IMF_SMODE_PLAURALCLAUSE: i32 = 1;
pub const IMF_SPELLCHECKING: i32 = 2048;
pub const IMF_TKBPREDICTION: i32 = 4096;
pub const IMF_UIFONTS: i32 = 32;
pub const IMF_VERTICAL: i32 = 32;
pub type KHYPH = i32;
pub const MAX_TABLE_CELLS: i32 = 63;
pub const MAX_TAB_STOPS: i32 = 32;
pub const MSFTEDIT_CLASS: windows_core::PCWSTR = windows_core::w!("RICHEDIT50W");
#[repr(C, packed(4))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct MSGFILTER {
    pub nmhdr: super::NMHDR,
    pub msg: super::UINT,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct OBJECTPOSITIONS {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: super::LONG,
    pub pcpPositions: *mut super::LONG,
}
pub const OLEOP_DOVERB: i32 = 1;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PARAFORMAT {
    pub cbSize: super::UINT,
    pub dwMask: super::DWORD,
    pub wNumbering: super::WORD,
    pub Anonymous: PARAFORMAT_0,
    pub dxStartIndent: super::LONG,
    pub dxRightIndent: super::LONG,
    pub dxOffset: super::LONG,
    pub wAlignment: super::WORD,
    pub cTabCount: super::SHORT,
    pub rgxTabs: [super::LONG; 32],
}
#[cfg(feature = "shared")]
impl Default for PARAFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PARAFORMAT_0 {
    pub wReserved: super::WORD,
    pub wEffects: super::WORD,
}
#[cfg(feature = "shared")]
impl Default for PARAFORMAT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PARAFORMAT2 {
    pub base: PARAFORMAT,
    pub dySpaceBefore: super::LONG,
    pub dySpaceAfter: super::LONG,
    pub dyLineSpacing: super::LONG,
    pub sStyle: super::SHORT,
    pub bLineSpacingRule: super::BYTE,
    pub bOutlineLevel: super::BYTE,
    pub wShadingWeight: super::WORD,
    pub wShadingStyle: super::WORD,
    pub wNumberingStart: super::WORD,
    pub wNumberingStyle: super::WORD,
    pub wNumberingTab: super::WORD,
    pub wBorderSpace: super::WORD,
    pub wBorderWidth: super::WORD,
    pub wBorders: super::WORD,
}
#[cfg(feature = "shared")]
impl Default for PARAFORMAT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PC_DELIMITER: i32 = 4;
pub const PC_FOLLOWING: i32 = 1;
pub const PC_LEADING: i32 = 2;
pub const PC_OVERFLOW: i32 = 3;
pub const PFA_CENTER: i32 = 3;
pub const PFA_FULL_GLYPHS: i32 = 8;
pub const PFA_FULL_INTERLETTER: i32 = 6;
pub const PFA_FULL_INTERWORD: i32 = 4;
pub const PFA_FULL_NEWSPAPER: i32 = 5;
pub const PFA_FULL_SCALED: i32 = 7;
pub const PFA_JUSTIFY: i32 = 4;
pub const PFA_LEFT: i32 = 1;
pub const PFA_RIGHT: i32 = 2;
pub const PFE_BOX: i32 = 1024;
pub const PFE_COLLAPSED: i32 = 256;
pub const PFE_DONOTHYPHEN: i32 = 64;
pub const PFE_KEEP: i32 = 2;
pub const PFE_KEEPNEXT: i32 = 4;
pub const PFE_NOLINENUMBER: i32 = 16;
pub const PFE_NOWIDOWCONTROL: i32 = 32;
pub const PFE_PAGEBREAKBEFORE: i32 = 8;
pub const PFE_RTLPARA: i32 = 1;
pub const PFE_SIDEBYSIDE: i32 = 128;
pub const PFE_TABLE: i32 = 16384;
pub const PFE_TABLEROWDELIMITER: i32 = 4096;
pub const PFE_TEXTWRAPPINGBREAK: i32 = 8192;
pub const PFM_ALIGNMENT: i32 = 8;
pub const PFM_ALL: u32 = 2147549247;
pub const PFM_ALL2: u32 = 3506437631;
pub const PFM_BORDER: i32 = 2048;
pub const PFM_BOX: i32 = 67108864;
pub const PFM_COLLAPSED: i32 = 16777216;
pub const PFM_DONOTHYPHEN: i32 = 4194304;
pub const PFM_EFFECTS: i32 = 1358888960;
pub const PFM_KEEP: i32 = 131072;
pub const PFM_KEEPNEXT: i32 = 262144;
pub const PFM_LINESPACING: i32 = 256;
pub const PFM_NOLINENUMBER: i32 = 1048576;
pub const PFM_NOWIDOWCONTROL: i32 = 2097152;
pub const PFM_NUMBERING: i32 = 32;
pub const PFM_NUMBERINGSTART: i32 = 32768;
pub const PFM_NUMBERINGSTYLE: i32 = 8192;
pub const PFM_NUMBERINGTAB: i32 = 16384;
pub const PFM_OFFSET: i32 = 4;
pub const PFM_OFFSETINDENT: u32 = 2147483648;
pub const PFM_OUTLINELEVEL: i32 = 33554432;
pub const PFM_PAGEBREAKBEFORE: i32 = 524288;
pub const PFM_RESERVED2: i32 = 134217728;
pub const PFM_RIGHTINDENT: i32 = 2;
pub const PFM_RTLPARA: i32 = 65536;
pub const PFM_SHADING: i32 = 4096;
pub const PFM_SIDEBYSIDE: i32 = 8388608;
pub const PFM_SPACEAFTER: i32 = 128;
pub const PFM_SPACEBEFORE: i32 = 64;
pub const PFM_STARTINDENT: i32 = 1;
pub const PFM_STYLE: i32 = 1024;
pub const PFM_TABLE: i32 = 1073741824;
pub const PFM_TABLEROWDELIMITER: i32 = 268435456;
pub const PFM_TABSTOPS: i32 = 16;
pub const PFM_TEXTWRAPPINGBREAK: i32 = 536870912;
pub const PFNS_NEWNUMBER: i32 = 32768;
pub const PFNS_NONUMBER: i32 = 1024;
pub const PFNS_PAREN: i32 = 0;
pub const PFNS_PARENS: i32 = 256;
pub const PFNS_PERIOD: i32 = 512;
pub const PFNS_PLAIN: i32 = 768;
pub const PFN_ARABIC: i32 = 2;
pub const PFN_BULLET: i32 = 1;
pub const PFN_LCLETTER: i32 = 3;
pub const PFN_LCROMAN: i32 = 5;
pub const PFN_UCLETTER: i32 = 4;
pub const PFN_UCROMAN: i32 = 6;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PUNCTUATION {
    pub iSize: super::UINT,
    pub szPunctuation: windows_core::PSTR,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct REPASTESPECIAL {
    pub dwAspect: super::DWORD,
    pub dwParam: super::DWORD_PTR,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct REQRESIZE {
    pub nmhdr: super::NMHDR,
    pub rc: super::RECT,
}
pub const RICHEDIT60_CLASS: windows_core::PCWSTR = windows_core::w!("RICHEDIT60W");
pub const RICHEDIT_CLASS: windows_core::PCSTR = windows_core::s!("RichEdit20A");
pub const RICHEDIT_CLASS10A: windows_core::PCSTR = windows_core::s!("RICHEDIT");
pub const RICHEDIT_CLASSA: windows_core::PCSTR = windows_core::s!("RichEdit20A");
pub const RICHEDIT_CLASSW: windows_core::PCWSTR = windows_core::w!("RichEdit20W");
#[repr(C, packed(4))]
#[cfg(all(feature = "objidlbase", feature = "shared"))]
#[derive(Default)]
pub struct RICHEDIT_IMAGE_PARAMETERS {
    pub xWidth: super::LONG,
    pub yHeight: super::LONG,
    pub Ascent: super::LONG,
    pub Type: super::LONG,
    pub pwszAlternateText: windows_core::PCWSTR,
    pub pIStream: core::mem::ManuallyDrop<Option<super::IStream>>,
}
pub const RTO_DISABLEHANDLES: i32 = 2;
pub const RTO_READINGMODE: i32 = 3;
pub const RTO_SHOWHANDLES: i32 = 1;
pub const SCF_ALL: i32 = 4;
pub const SCF_ASSOCIATEFONT: i32 = 16;
pub const SCF_ASSOCIATEFONT2: i32 = 64;
pub const SCF_CHARREPFROMLCID: i32 = 256;
pub const SCF_DEFAULT: i32 = 0;
pub const SCF_NOKBUPDATE: i32 = 32;
pub const SCF_SELECTION: i32 = 1;
pub const SCF_SMARTFONT: i32 = 128;
pub const SCF_USEUIRULES: i32 = 8;
pub const SCF_WORD: i32 = 2;
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct SELCHANGE {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: super::WORD,
}
pub const SEL_EMPTY: i32 = 0;
pub const SEL_MULTICHAR: i32 = 4;
pub const SEL_MULTIOBJECT: i32 = 8;
pub const SEL_OBJECT: i32 = 2;
pub const SEL_TEXT: i32 = 1;
pub const SES_ALLOWBEEPS: i32 = 256;
pub const SES_BEEPONMAXTEXT: i32 = 2;
pub const SES_BIDI: i32 = 4096;
pub const SES_CTFALLOWEMBED: i32 = 2097152;
pub const SES_CTFALLOWPROOFING: i32 = 8388608;
pub const SES_CTFALLOWSMARTTAG: i32 = 4194304;
pub const SES_CTFNOLOCK: i32 = 268435456;
pub const SES_CUSTOMLOOK: i32 = 524288;
pub const SES_DEFAULTLATINLIGA: i32 = 16;
pub const SES_DRAFTMODE: i32 = 32768;
pub const SES_EMULATE10: i32 = 16;
pub const SES_EMULATESYSEDIT: i32 = 1;
pub const SES_EXTENDBACKCOLOR: i32 = 4;
pub const SES_EX_HANDLEFRIENDLYURL: i32 = 256;
pub const SES_EX_HIDETEMPFORMAT: i32 = 268435456;
pub const SES_EX_MULTITOUCH: i32 = 134217728;
pub const SES_EX_NOACETATESELECTION: i32 = 1048576;
pub const SES_EX_NOMATH: i32 = 64;
pub const SES_EX_NOTABLE: i32 = 4;
pub const SES_EX_NOTHEMING: i32 = 524288;
pub const SES_EX_USEMOUSEWPARAM: i32 = 536870912;
pub const SES_EX_USESINGLELINE: i32 = 2097152;
pub const SES_HIDEGRIDLINES: i32 = 131072;
pub const SES_HYPERLINKTOOLTIPS: i32 = 8;
pub const SES_LBSCROLLNOTIFY: i32 = 1048576;
pub const SES_LOGICALCARET: i32 = 16777216;
pub const SES_LOWERCASE: i32 = 1024;
pub const SES_MAPCPS: i32 = 8;
pub const SES_MAX: i32 = 536870912;
pub const SES_MULTISELECT: i32 = 134217728;
pub const SES_NOEALINEHEIGHTADJUST: i32 = 536870912;
pub const SES_NOFOCUSLINKNOTIFY: i32 = 32;
pub const SES_NOIME: i32 = 128;
pub const SES_NOINPUTSEQUENCECHK: i32 = 2048;
pub const SES_SCROLLONKILLFOCUS: i32 = 8192;
pub const SES_SMARTDRAGDROP: i32 = 67108864;
pub const SES_UPPERCASE: i32 = 512;
pub const SES_USEAIMM: i32 = 64;
pub const SES_USEATFONT: i32 = 262144;
pub const SES_USECRLF: i32 = 32;
pub const SES_USECTF: i32 = 65536;
pub const SES_WORDDRAGDROP: i32 = 33554432;
pub const SES_XLTCRCRLFTOCR: i32 = 16384;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SETTEXTEX {
    pub flags: super::DWORD,
    pub codepage: super::UINT,
}
pub const SFF_KEEPDOCINFO: i32 = 4096;
pub const SFF_PERSISTVIEWSCALE: i32 = 8192;
pub const SFF_PLAINRTF: i32 = 16384;
pub const SFF_PWD: i32 = 2048;
pub const SFF_SELECTION: i32 = 32768;
pub const SFF_WRITEXTRAPAR: i32 = 128;
pub const SF_NCRFORNONASCII: i32 = 64;
pub const SF_RTF: i32 = 2;
pub const SF_RTFNOOBJS: i32 = 3;
pub const SF_RTFVAL: i32 = 1792;
pub const SF_TEXT: i32 = 1;
pub const SF_TEXTIZED: i32 = 4;
pub const SF_UNICODE: i32 = 16;
pub const SF_USECODEPAGE: i32 = 32;
pub const SPF_DONTSETDEFAULT: i32 = 2;
pub const SPF_SETDEFAULT: i32 = 4;
pub const ST_DEFAULT: i32 = 0;
pub const ST_KEEPUNDO: i32 = 1;
pub const ST_NEWCHARS: i32 = 4;
pub const ST_SELECTION: i32 = 2;
pub const ST_UNICODE: i32 = 8;
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct TABLECELLPARMS {
    pub dxWidth: super::LONG,
    pub _bitfield: u16,
    pub wShading: super::WORD,
    pub dxBrdrLeft: super::SHORT,
    pub dyBrdrTop: super::SHORT,
    pub dxBrdrRight: super::SHORT,
    pub dyBrdrBottom: super::SHORT,
    pub crBrdrLeft: super::COLORREF,
    pub crBrdrTop: super::COLORREF,
    pub crBrdrRight: super::COLORREF,
    pub crBrdrBottom: super::COLORREF,
    pub crBackPat: super::COLORREF,
    pub crForePat: super::COLORREF,
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl TABLECELLPARMS {
    pub fn nVertAlign(&self) -> super::WORD {
        (self._bitfield) & 3
    }
    pub fn set_nVertAlign(&mut self, value: super::WORD) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn fMergeTop(&self) -> super::WORD {
        (self._bitfield >> 2) & 1
    }
    pub fn set_fMergeTop(&mut self, value: super::WORD) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn fMergePrev(&self) -> super::WORD {
        (self._bitfield >> 3) & 1
    }
    pub fn set_fMergePrev(&mut self, value: super::WORD) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn fVertical(&self) -> super::WORD {
        (self._bitfield >> 4) & 1
    }
    pub fn set_fVertical(&mut self, value: super::WORD) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn fMergeStart(&self) -> super::WORD {
        (self._bitfield >> 5) & 1
    }
    pub fn set_fMergeStart(&mut self, value: super::WORD) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn fMergeCont(&self) -> super::WORD {
        (self._bitfield >> 6) & 1
    }
    pub fn set_fMergeCont(&mut self, value: super::WORD) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TABLEROWPARMS {
    pub cbRow: super::BYTE,
    pub cbCell: super::BYTE,
    pub cCell: super::BYTE,
    pub cRow: super::BYTE,
    pub dxCellMargin: super::LONG,
    pub dxIndent: super::LONG,
    pub dyHeight: super::LONG,
    pub _bitfield: u32,
    pub cpStartRow: super::LONG,
    pub bTableLevel: super::BYTE,
    pub iCell: super::BYTE,
}
#[cfg(feature = "shared")]
impl TABLEROWPARMS {
    pub fn nAlignment(&self) -> super::DWORD {
        (self._bitfield) & 7
    }
    pub fn set_nAlignment(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn fRTL(&self) -> super::DWORD {
        (self._bitfield >> 3) & 1
    }
    pub fn set_fRTL(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn fKeep(&self) -> super::DWORD {
        (self._bitfield >> 4) & 1
    }
    pub fn set_fKeep(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn fKeepFollow(&self) -> super::DWORD {
        (self._bitfield >> 5) & 1
    }
    pub fn set_fKeepFollow(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn fWrap(&self) -> super::DWORD {
        (self._bitfield >> 6) & 1
    }
    pub fn set_fWrap(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn fIdentCells(&self) -> super::DWORD {
        (self._bitfield >> 7) & 1
    }
    pub fn set_fIdentCells(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
pub type TEXTMODE = i32;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TEXTRANGEA {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PSTR,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TEXTRANGEW {
    pub chrg: CHARRANGE,
    pub lpstrText: windows_core::PWSTR,
}
pub const TM_MULTICODEPAGE: TEXTMODE = 32;
pub const TM_MULTILEVELUNDO: TEXTMODE = 8;
pub const TM_PLAINTEXT: TEXTMODE = 1;
pub const TM_RICHTEXT: TEXTMODE = 2;
pub const TM_SINGLECODEPAGE: TEXTMODE = 16;
pub const TM_SINGLELEVELUNDO: TEXTMODE = 4;
pub const TO_ADVANCEDLAYOUT: i32 = 8;
pub const TO_ADVANCEDTYPOGRAPHY: i32 = 1;
pub const TO_DISABLECUSTOMTEXTOUT: i32 = 4;
pub const TO_SIMPLELINEBREAK: i32 = 2;
pub const UID_AUTOTABLE: UNDONAMEID = 6;
pub const UID_CUT: UNDONAMEID = 4;
pub const UID_DELETE: UNDONAMEID = 2;
pub const UID_DRAGDROP: UNDONAMEID = 3;
pub const UID_PASTE: UNDONAMEID = 5;
pub const UID_TYPING: UNDONAMEID = 1;
pub const UID_UNKNOWN: UNDONAMEID = 0;
pub type UNDONAMEID = i32;
pub const VM_NORMAL: i32 = 4;
pub const VM_OUTLINE: i32 = 2;
pub const VM_PAGE: i32 = 9;
#[cfg(feature = "shared")]
pub const WBF_BREAKAFTER: super::BYTE = 64;
#[cfg(feature = "shared")]
pub const WBF_BREAKLINE: super::BYTE = 32;
#[cfg(feature = "shared")]
pub const WBF_CLASS: super::BYTE = 15;
pub const WBF_CUSTOM: i32 = 512;
#[cfg(feature = "shared")]
pub const WBF_ISWHITE: super::BYTE = 16;
pub const WBF_LEVEL1: i32 = 128;
pub const WBF_LEVEL2: i32 = 256;
pub const WBF_OVERFLOW: i32 = 64;
pub const WBF_WORDBREAK: i32 = 32;
pub const WBF_WORDWRAP: i32 = 16;
pub const WB_CLASSIFY: i32 = 3;
pub const WB_LEFTBREAK: i32 = 6;
pub const WB_MOVEWORDLEFT: i32 = 4;
pub const WB_MOVEWORDNEXT: i32 = 5;
pub const WB_MOVEWORDPREV: i32 = 4;
pub const WB_MOVEWORDRIGHT: i32 = 5;
pub const WB_NEXTBREAK: i32 = 7;
pub const WB_PREVBREAK: i32 = 6;
pub const WB_RIGHTBREAK: i32 = 7;
#[cfg(feature = "winnt")]
pub const WCH_EMBEDDING: super::WCHAR = 65532;
pub const _RICHEDIT_VER: i32 = 2064;
pub const cchTextLimitDefault: i32 = 32767;
pub const khyphAddBefore: KHYPH = 2;
pub const khyphChangeAfter: KHYPH = 5;
pub const khyphChangeBefore: KHYPH = 3;
pub const khyphDelAndChange: KHYPH = 6;
pub const khyphDeleteBefore: KHYPH = 4;
pub const khyphNil: KHYPH = 0;
pub const khyphNormal: KHYPH = 1;
pub const lDefaultTab: i32 = 720;
pub const yHeightCharPtsMost: i32 = 1638;
