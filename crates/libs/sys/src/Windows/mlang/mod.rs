pub const CMLangConvertCharset: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd66d6f99_cdaa_11d0_b822_00c04fc9b31f);
pub const CMLangString: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc04d65cf_b70d_11d0_b188_00aa0038c969);
pub const CMultiLanguage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x275c23e2_3747_11d0_9fea_00aa003f8646);
pub const CPIOD_FORCE_PROMPT: u32 = 2147483648;
pub const CPIOD_PEEK: u32 = 1073741824;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DetectEncodingInfo {
    pub nLangID: u32,
    pub nCodePage: u32,
    pub nDocPercent: i32,
    pub nConfidence: i32,
}
pub const MAX_LOCALE_NAME: u32 = 32;
pub const MAX_MIMECP_NAME: u32 = 64;
pub const MAX_MIMECSET_NAME: u32 = 50;
pub const MAX_MIMEFACE_NAME: u32 = 32;
pub const MAX_RFC1766_NAME: u32 = 6;
pub const MAX_SCRIPT_NAME: u32 = 48;
pub type MIMECONTF = i32;
pub const MIMECONTF_BROWSER: MIMECONTF = 2;
pub const MIMECONTF_EXPORT: MIMECONTF = 1024;
pub const MIMECONTF_IMPORT: MIMECONTF = 8;
pub const MIMECONTF_MAILNEWS: MIMECONTF = 1;
pub const MIMECONTF_MIME_IE4: MIMECONTF = 268435456;
pub const MIMECONTF_MIME_LATEST: MIMECONTF = 536870912;
pub const MIMECONTF_MIME_REGISTRY: MIMECONTF = 1073741824;
pub const MIMECONTF_MINIMAL: MIMECONTF = 4;
pub const MIMECONTF_PRIVCONVERTER: MIMECONTF = 65536;
pub const MIMECONTF_SAVABLE_BROWSER: MIMECONTF = 512;
pub const MIMECONTF_SAVABLE_MAILNEWS: MIMECONTF = 256;
pub const MIMECONTF_VALID: MIMECONTF = 131072;
pub const MIMECONTF_VALID_NLS: MIMECONTF = 262144;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIMECPINFO {
    pub dwFlags: u32,
    pub uiCodePage: u32,
    pub uiFamilyCodePage: u32,
    pub wszDescription: [u16; 64],
    pub wszWebCharset: [u16; 50],
    pub wszHeaderCharset: [u16; 50],
    pub wszBodyCharset: [u16; 50],
    pub wszFixedWidthFont: [u16; 32],
    pub wszProportionalFont: [u16; 32],
    pub bGDICharset: u8,
}
impl Default for MIMECPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIMECSETINFO {
    pub uiCodePage: u32,
    pub uiInternetEncoding: u32,
    pub wszCharset: [u16; 50],
}
impl Default for MIMECSETINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MLCONVCHAR = i32;
pub const MLCONVCHARF_AUTODETECT: MLCONVCHAR = 1;
pub const MLCONVCHARF_DETECTJPN: MLCONVCHAR = 32;
pub const MLCONVCHARF_ENTITIZE: MLCONVCHAR = 2;
pub const MLCONVCHARF_NAME_ENTITIZE: MLCONVCHAR = 4;
pub const MLCONVCHARF_NCR_ENTITIZE: MLCONVCHAR = 2;
pub const MLCONVCHARF_NOBESTFITCHARS: MLCONVCHAR = 16;
pub const MLCONVCHARF_USEDEFCHAR: MLCONVCHAR = 8;
pub type MLCP = i32;
pub type MLDETECTCP = i32;
pub const MLDETECTCP_7BIT: MLDETECTCP = 1;
pub const MLDETECTCP_8BIT: MLDETECTCP = 2;
pub const MLDETECTCP_DBCS: MLDETECTCP = 4;
pub const MLDETECTCP_HTML: MLDETECTCP = 8;
pub const MLDETECTCP_NONE: MLDETECTCP = 0;
pub const MLDETECTCP_NUMBER: MLDETECTCP = 16;
pub const MLDETECTF_BROWSER: MLCP = 2;
pub const MLDETECTF_EURO_UTF8: MLCP = 128;
pub const MLDETECTF_FILTER_SPECIALCHAR: MLCP = 64;
pub const MLDETECTF_MAILNEWS: MLCP = 1;
pub const MLDETECTF_PREFERRED_ONLY: MLCP = 32;
pub const MLDETECTF_PRESERVE_ORDER: MLCP = 16;
pub const MLDETECTF_VALID: MLCP = 4;
pub const MLDETECTF_VALID_NLS: MLCP = 8;
pub type MLSTR_FLAGS = i32;
pub const MLSTR_READ: MLSTR_FLAGS = 1;
pub const MLSTR_WRITE: MLSTR_FLAGS = 2;
pub type PMIMECPINFO = *mut MIMECPINFO;
pub type PMIMECSETINFO = *mut MIMECSETINFO;
#[cfg(feature = "winnt")]
pub type PRFC1766INFO = *mut RFC1766INFO;
pub type PSCRIPTFONTINFO = *mut SCRIPTFONTINFO;
pub type PSCRIPTINFO = *mut SCRIPTINFO;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct RFC1766INFO {
    pub lcid: super::winnt::LCID,
    pub wszRfc1766: [u16; 6],
    pub wszLocaleName: [u16; 32],
}
#[cfg(feature = "winnt")]
impl Default for RFC1766INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SCRIPTCONTF = i32;
pub const SCRIPTCONTF_FIXED_FONT: SCRIPTFONTCONTF = 1;
pub const SCRIPTCONTF_PROPORTIONAL_FONT: SCRIPTFONTCONTF = 2;
pub const SCRIPTCONTF_SCRIPT_HIDE: SCRIPTFONTCONTF = 131072;
pub const SCRIPTCONTF_SCRIPT_SYSTEM: SCRIPTFONTCONTF = 262144;
pub const SCRIPTCONTF_SCRIPT_USER: SCRIPTFONTCONTF = 65536;
pub type SCRIPTFONTCONTF = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCRIPTFONTINFO {
    pub scripts: SCRIPT_IDS,
    pub wszFont: [u16; 32],
}
impl Default for SCRIPTFONTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCRIPTINFO {
    pub ScriptId: SCRIPT_ID,
    pub uiCodePage: u32,
    pub wszDescription: [u16; 48],
    pub wszFixedWidthFont: [u16; 32],
    pub wszProportionalFont: [u16; 32],
}
impl Default for SCRIPTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SCRIPT_ID = u8;
pub type SCRIPT_IDS = i64;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct UNICODERANGE {
    pub wcFrom: u16,
    pub wcTo: u16,
}
pub type pDetectEncodingInfo = *mut DetectEncodingInfo;
pub const sidArabic: SCRIPTCONTF = 9;
pub const sidArmenian: SCRIPTCONTF = 7;
pub const sidAsciiLatin: SCRIPTCONTF = 3;
pub const sidAsciiSym: SCRIPTCONTF = 2;
pub const sidBengali: SCRIPTCONTF = 11;
pub const sidBopomofo: SCRIPTCONTF = 25;
pub const sidBraille: SCRIPTCONTF = 31;
pub const sidBurmese: SCRIPTCONTF = 36;
pub const sidCanSyllabic: SCRIPTCONTF = 28;
pub const sidCherokee: SCRIPTCONTF = 29;
pub const sidCyrillic: SCRIPTCONTF = 6;
pub const sidDefault: SCRIPTCONTF = 0;
pub const sidDevanagari: SCRIPTCONTF = 10;
pub const sidEthiopic: SCRIPTCONTF = 27;
pub const sidFEFirst: SCRIPTCONTF = 23;
pub const sidFELast: SCRIPTCONTF = 26;
pub const sidGeorgian: SCRIPTCONTF = 22;
pub const sidGreek: SCRIPTCONTF = 5;
pub const sidGujarati: SCRIPTCONTF = 13;
pub const sidGurmukhi: SCRIPTCONTF = 12;
pub const sidHan: SCRIPTCONTF = 26;
pub const sidHangul: SCRIPTCONTF = 23;
pub const sidHebrew: SCRIPTCONTF = 8;
pub const sidKana: SCRIPTCONTF = 24;
pub const sidKannada: SCRIPTCONTF = 17;
pub const sidKhmer: SCRIPTCONTF = 37;
pub const sidLao: SCRIPTCONTF = 20;
pub const sidLatin: SCRIPTCONTF = 4;
pub const sidLim: SCRIPTCONTF = 41;
pub const sidMalayalam: SCRIPTCONTF = 18;
pub const sidMerge: SCRIPTCONTF = 1;
pub const sidMongolian: SCRIPTCONTF = 39;
pub const sidOgham: SCRIPTCONTF = 33;
pub const sidOriya: SCRIPTCONTF = 14;
pub const sidRunic: SCRIPTCONTF = 32;
pub const sidSinhala: SCRIPTCONTF = 34;
pub const sidSyriac: SCRIPTCONTF = 35;
pub const sidTamil: SCRIPTCONTF = 15;
pub const sidTelugu: SCRIPTCONTF = 16;
pub const sidThaana: SCRIPTCONTF = 38;
pub const sidThai: SCRIPTCONTF = 19;
pub const sidTibetan: SCRIPTCONTF = 21;
pub const sidUserDefined: SCRIPTCONTF = 40;
pub const sidYi: SCRIPTCONTF = 30;
