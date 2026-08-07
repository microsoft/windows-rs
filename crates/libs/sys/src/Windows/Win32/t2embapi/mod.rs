#[cfg(feature = "windef")]
windows_link::link!("t2embed.dll" "system" fn TTCharToUnicode(hdc : super::HDC, puccharcodes : *const u8, ulcharcodesize : u32, pusshortcodes : *mut u16, ulshortcodesize : u32, ulflags : u32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("t2embed.dll" "system" fn TTDeleteEmbeddedFont(hfontreference : super::HANDLE, ulflags : u32, pulstatus : *mut u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("t2embed.dll" "system" fn TTEmbedFont(hdc : super::HDC, ulflags : u32, ulcharset : u32, pulprivstatus : *mut u32, pulstatus : *mut u32, lpfnwritetostream : WRITEEMBEDPROC, lpvwritestream : *const core::ffi::c_void, puscharcodeset : *const u16, uscharcodecount : u16, uslanguage : u16, pttembedinfo : *const TTEMBEDINFO) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("t2embed.dll" "system" fn TTEmbedFontEx(hdc : super::HDC, ulflags : u32, ulcharset : u32, pulprivstatus : *mut u32, pulstatus : *mut u32, lpfnwritetostream : WRITEEMBEDPROC, lpvwritestream : *const core::ffi::c_void, pulcharcodeset : *const u32, uscharcodecount : u16, uslanguage : u16, pttembedinfo : *const TTEMBEDINFO) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("t2embed.dll" "system" fn TTEmbedFontFromFileA(hdc : super::HDC, szfontfilename : windows_sys::core::PCSTR, usttcindex : u16, ulflags : u32, ulcharset : u32, pulprivstatus : *mut u32, pulstatus : *mut u32, lpfnwritetostream : WRITEEMBEDPROC, lpvwritestream : *const core::ffi::c_void, puscharcodeset : *const u16, uscharcodecount : u16, uslanguage : u16, pttembedinfo : *const TTEMBEDINFO) -> i32);
windows_link::link!("t2embed.dll" "system" fn TTEnableEmbeddingForFacename(lpszfacename : windows_sys::core::PCSTR, benable : windows_sys::core::BOOL) -> i32);
windows_link::link!("t2embed.dll" "system" fn TTGetEmbeddedFontInfo(ulflags : u32, pulprivstatus : *mut u32, ulprivs : u32, pulstatus : *mut u32, lpfnreadfromstream : READEMBEDPROC, lpvreadstream : *const core::ffi::c_void, pttloadinfo : *const TTLOADINFO) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("t2embed.dll" "system" fn TTGetEmbeddingType(hdc : super::HDC, pulembedtype : *mut u32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("t2embed.dll" "system" fn TTGetNewFontName(phfontreference : *const super::HANDLE, wzwinfamilyname : windows_sys::core::PWSTR, cchmaxwinname : i32, szmacfamilyname : windows_sys::core::PSTR, cchmaxmacname : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("t2embed.dll" "system" fn TTIsEmbeddingEnabled(hdc : super::HDC, pbenabled : *mut windows_sys::core::BOOL) -> i32);
windows_link::link!("t2embed.dll" "system" fn TTIsEmbeddingEnabledForFacename(lpszfacename : windows_sys::core::PCSTR, pbenabled : *mut windows_sys::core::BOOL) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("t2embed.dll" "system" fn TTLoadEmbeddedFont(phfontreference : *mut super::HANDLE, ulflags : u32, pulprivstatus : *mut u32, ulprivs : u32, pulstatus : *mut u32, lpfnreadfromstream : READEMBEDPROC, lpvreadstream : *const core::ffi::c_void, szwinfamilyname : windows_sys::core::PCWSTR, szmacfamilyname : windows_sys::core::PCSTR, pttloadinfo : *const TTLOADINFO) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("t2embed.dll" "system" fn TTRunValidationTests(hdc : super::HDC, ptestparam : *const TTVALIDATIONTESTSPARAMS) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("t2embed.dll" "system" fn TTRunValidationTestsEx(hdc : super::HDC, ptestparam : *const TTVALIDATIONTESTSPARAMSEX) -> i32);
pub const CHARSET_DEFAULT: i32 = 1;
pub const CHARSET_GLYPHIDX: i32 = 3;
pub const CHARSET_SYMBOL: i32 = 2;
pub const CHARSET_UNICODE: i32 = 1;
pub const EMBED_EDITABLE: i32 = 2;
pub const EMBED_INSTALLABLE: i32 = 3;
pub const EMBED_NOEMBEDDING: i32 = 4;
pub const EMBED_PREVIEWPRINT: i32 = 1;
pub const E_ADDFONTFAILED: i32 = 512;
pub const E_API_NOTIMPL: i32 = 1;
pub const E_CHARCODECOUNTINVALID: i32 = 2;
pub const E_CHARCODESETINVALID: i32 = 3;
pub const E_CHARSETINVALID: i32 = 21;
pub const E_COULDNTCREATETEMPFILE: i32 = 513;
pub const E_DEVICETRUETYPEFONT: i32 = 4;
pub const E_ERRORACCESSINGEXCLUDELIST: i32 = 274;
pub const E_ERRORACCESSINGFACENAME: i32 = 13;
pub const E_ERRORACCESSINGFONTDATA: i32 = 12;
pub const E_ERRORCOMPRESSINGFONTDATA: i32 = 256;
pub const E_ERRORCONVERTINGCHARS: i32 = 18;
pub const E_ERRORCREATINGFONTFILE: i32 = 269;
pub const E_ERRORDECOMPRESSINGFONTDATA: i32 = 273;
pub const E_ERROREXPANDINGFONTDATA: i32 = 519;
pub const E_ERRORGETTINGDC: i32 = 520;
pub const E_ERRORREADINGFONTDATA: i32 = 267;
pub const E_ERRORUNICODECONVERSION: i32 = 17;
pub const E_EXCEPTION: i32 = 19;
pub const E_EXCEPTIONINCOMPRESSION: i32 = 522;
pub const E_EXCEPTIONINDECOMPRESSION: i32 = 521;
pub const E_FACENAMEINVALID: i32 = 275;
pub const E_FILE_NOT_FOUND: i32 = 23;
pub const E_FLAGSINVALID: i32 = 268;
pub const E_FONTALREADYEXISTS: i32 = 270;
pub const E_FONTDATAINVALID: i32 = 258;
pub const E_FONTFAMILYNAMENOTINFULL: i32 = 285;
pub const E_FONTFILECREATEFAILED: i32 = 515;
pub const E_FONTFILENOTFOUND: i32 = 517;
pub const E_FONTINSTALLFAILED: i32 = 272;
pub const E_FONTNAMEALREADYEXISTS: i32 = 271;
pub const E_FONTNOTEMBEDDABLE: i32 = 260;
pub const E_FONTREFERENCEINVALID: i32 = 8;
pub const E_FONTVARIATIONSIMULATED: i32 = 283;
pub const E_HDCINVALID: i32 = 6;
pub const E_INPUTPARAMINVALID: i32 = 25;
pub const E_NAMECHANGEFAILED: i32 = 259;
pub const E_NOFREEMEMORY: i32 = 7;
pub const E_NONE: i32 = 0;
pub const E_NOOS2: i32 = 265;
pub const E_NOTATRUETYPEFONT: i32 = 10;
pub const E_PBENABLEDINVALID: i32 = 280;
pub const E_PERMISSIONSINVALID: i32 = 279;
pub const E_PRIVSINVALID: i32 = 261;
pub const E_PRIVSTATUSINVALID: i32 = 278;
pub const E_READFROMSTREAMFAILED: i32 = 263;
pub const E_RESERVEDPARAMNOTNULL: i32 = 20;
pub const E_RESOURCEFILECREATEFAILED: i32 = 518;
pub const E_SAVETOSTREAMFAILED: i32 = 264;
pub const E_STATUSINVALID: i32 = 277;
pub const E_STREAMINVALID: i32 = 276;
pub const E_SUBSETTINGEXCEPTION: i32 = 281;
pub const E_SUBSETTINGFAILED: i32 = 262;
pub const E_SUBSTRING_TEST_FAIL: i32 = 282;
pub const E_T2NOFREEMEMORY: i32 = 266;
pub const E_TTC_INDEX_OUT_OF_RANGE: i32 = 24;
pub const E_WINDOWSAPI: i32 = 516;
pub const LICENSE_DEFAULT: i32 = 0;
pub const LICENSE_EDITABLE: i32 = 8;
pub const LICENSE_INSTALLABLE: i32 = 0;
pub const LICENSE_NOEMBEDDING: i32 = 2;
pub const LICENSE_PREVIEWPRINT: i32 = 4;
pub type READEMBEDPROC = Option<unsafe extern "C" fn(param0: *mut core::ffi::c_void, param1: *mut core::ffi::c_void, param2: u32) -> u32>;
pub const TTDELETE_DONTREMOVEFONT: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TTEMBEDINFO {
    pub usStructSize: u16,
    pub usRootStrSize: u16,
    pub pusRootStr: *mut u16,
}
pub const TTEMBED_EMBEDEUDC: i32 = 32;
pub const TTEMBED_EUDCEMBEDDED: i32 = 2;
pub const TTEMBED_FAILIFVARIATIONSIMULATED: i32 = 16;
pub const TTEMBED_RAW: i32 = 0;
pub const TTEMBED_SUBSET: i32 = 1;
pub const TTEMBED_SUBSETCANCEL: i32 = 4;
pub const TTEMBED_TTCOMPRESSED: i32 = 4;
pub const TTEMBED_VARIATIONSIMULATED: i32 = 1;
pub const TTEMBED_WEBOBJECT: i32 = 128;
pub const TTEMBED_XORENCRYPTDATA: i32 = 268435456;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TTLOADINFO {
    pub usStructSize: u16,
    pub usRefStrSize: u16,
    pub pusRefStr: *mut u16,
}
pub const TTLOAD_EUDC_OVERWRITE: i32 = 2;
pub const TTLOAD_EUDC_SET: i32 = 4;
pub const TTLOAD_FONT_IN_SYSSTARTUP: i32 = 2;
pub const TTLOAD_FONT_SUBSETTED: i32 = 1;
pub const TTLOAD_PRIVATE: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TTVALIDATIONTESTSPARAMS {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pusCharCodeSet: *mut u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TTVALIDATIONTESTSPARAMSEX {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pulCharCodeSet: *mut u32,
}
pub type WRITEEMBEDPROC = Option<unsafe extern "C" fn(param0: *mut core::ffi::c_void, param1: *const core::ffi::c_void, param2: u32) -> u32>;
