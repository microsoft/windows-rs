#[cfg(feature = "Win32_windef")]
windows_link::link!("t2embed.dll" "system" fn TTCharToUnicode(hdc : super::windef::HDC, puccharcodes : *const u8, ulcharcodesize : u32, pusshortcodes : *mut u16, ulshortcodesize : u32, ulflags : u32) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("t2embed.dll" "system" fn TTDeleteEmbeddedFont(hfontreference : super::winnt::HANDLE, ulflags : u32, pulstatus : *mut u32) -> i32);
#[cfg(feature = "Win32_windef")]
windows_link::link!("t2embed.dll" "system" fn TTEmbedFont(hdc : super::windef::HDC, ulflags : u32, ulcharset : u32, pulprivstatus : *mut u32, pulstatus : *mut u32, lpfnwritetostream : WRITEEMBEDPROC, lpvwritestream : *const core::ffi::c_void, puscharcodeset : *const u16, uscharcodecount : u16, uslanguage : u16, pttembedinfo : *const TTEMBEDINFO) -> i32);
#[cfg(feature = "Win32_windef")]
windows_link::link!("t2embed.dll" "system" fn TTEmbedFontEx(hdc : super::windef::HDC, ulflags : u32, ulcharset : u32, pulprivstatus : *mut u32, pulstatus : *mut u32, lpfnwritetostream : WRITEEMBEDPROC, lpvwritestream : *const core::ffi::c_void, pulcharcodeset : *const u32, uscharcodecount : u16, uslanguage : u16, pttembedinfo : *const TTEMBEDINFO) -> i32);
#[cfg(feature = "Win32_windef")]
windows_link::link!("t2embed.dll" "system" fn TTEmbedFontFromFileA(hdc : super::windef::HDC, szfontfilename : windows_sys::core::PCSTR, usttcindex : u16, ulflags : u32, ulcharset : u32, pulprivstatus : *mut u32, pulstatus : *mut u32, lpfnwritetostream : WRITEEMBEDPROC, lpvwritestream : *const core::ffi::c_void, puscharcodeset : *const u16, uscharcodecount : u16, uslanguage : u16, pttembedinfo : *const TTEMBEDINFO) -> i32);
windows_link::link!("t2embed.dll" "system" fn TTEnableEmbeddingForFacename(lpszfacename : windows_sys::core::PCSTR, benable : windows_sys::core::BOOL) -> i32);
windows_link::link!("t2embed.dll" "system" fn TTGetEmbeddedFontInfo(ulflags : u32, pulprivstatus : *mut u32, ulprivs : u32, pulstatus : *mut u32, lpfnreadfromstream : READEMBEDPROC, lpvreadstream : *const core::ffi::c_void, pttloadinfo : *const TTLOADINFO) -> i32);
#[cfg(feature = "Win32_windef")]
windows_link::link!("t2embed.dll" "system" fn TTGetEmbeddingType(hdc : super::windef::HDC, pulembedtype : *mut u32) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("t2embed.dll" "system" fn TTGetNewFontName(phfontreference : *const super::winnt::HANDLE, wzwinfamilyname : windows_sys::core::PWSTR, cchmaxwinname : i32, szmacfamilyname : windows_sys::core::PSTR, cchmaxmacname : i32) -> i32);
#[cfg(feature = "Win32_windef")]
windows_link::link!("t2embed.dll" "system" fn TTIsEmbeddingEnabled(hdc : super::windef::HDC, pbenabled : *mut windows_sys::core::BOOL) -> i32);
windows_link::link!("t2embed.dll" "system" fn TTIsEmbeddingEnabledForFacename(lpszfacename : windows_sys::core::PCSTR, pbenabled : *mut windows_sys::core::BOOL) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("t2embed.dll" "system" fn TTLoadEmbeddedFont(phfontreference : *mut super::winnt::HANDLE, ulflags : u32, pulprivstatus : *mut u32, ulprivs : u32, pulstatus : *mut u32, lpfnreadfromstream : READEMBEDPROC, lpvreadstream : *const core::ffi::c_void, szwinfamilyname : windows_sys::core::PCWSTR, szmacfamilyname : windows_sys::core::PCSTR, pttloadinfo : *const TTLOADINFO) -> i32);
#[cfg(feature = "Win32_windef")]
windows_link::link!("t2embed.dll" "system" fn TTRunValidationTests(hdc : super::windef::HDC, ptestparam : *const TTVALIDATIONTESTSPARAMS) -> i32);
#[cfg(feature = "Win32_windef")]
windows_link::link!("t2embed.dll" "system" fn TTRunValidationTestsEx(hdc : super::windef::HDC, ptestparam : *const TTVALIDATIONTESTSPARAMSEX) -> i32);
pub const CHARSET_DEFAULT: u32 = 1;
pub const CHARSET_GLYPHIDX: u32 = 3;
pub const CHARSET_SYMBOL: u32 = 2;
pub const CHARSET_UNICODE: u32 = 1;
pub const EMBED_EDITABLE: u32 = 2;
pub const EMBED_INSTALLABLE: u32 = 3;
pub const EMBED_NOEMBEDDING: u32 = 4;
pub const EMBED_PREVIEWPRINT: u32 = 1;
pub const E_ADDFONTFAILED: u32 = 512;
pub const E_API_NOTIMPL: u32 = 1;
pub const E_CHARCODECOUNTINVALID: u32 = 2;
pub const E_CHARCODESETINVALID: u32 = 3;
pub const E_CHARSETINVALID: u32 = 21;
pub const E_COULDNTCREATETEMPFILE: u32 = 513;
pub const E_DEVICETRUETYPEFONT: u32 = 4;
pub const E_ERRORACCESSINGEXCLUDELIST: u32 = 274;
pub const E_ERRORACCESSINGFACENAME: u32 = 13;
pub const E_ERRORACCESSINGFONTDATA: u32 = 12;
pub const E_ERRORCOMPRESSINGFONTDATA: u32 = 256;
pub const E_ERRORCONVERTINGCHARS: u32 = 18;
pub const E_ERRORCREATINGFONTFILE: u32 = 269;
pub const E_ERRORDECOMPRESSINGFONTDATA: u32 = 273;
pub const E_ERROREXPANDINGFONTDATA: u32 = 519;
pub const E_ERRORGETTINGDC: u32 = 520;
pub const E_ERRORREADINGFONTDATA: u32 = 267;
pub const E_ERRORUNICODECONVERSION: u32 = 17;
pub const E_EXCEPTION: u32 = 19;
pub const E_EXCEPTIONINCOMPRESSION: u32 = 522;
pub const E_EXCEPTIONINDECOMPRESSION: u32 = 521;
pub const E_FACENAMEINVALID: u32 = 275;
pub const E_FILE_NOT_FOUND: u32 = 23;
pub const E_FLAGSINVALID: u32 = 268;
pub const E_FONTALREADYEXISTS: u32 = 270;
pub const E_FONTDATAINVALID: u32 = 258;
pub const E_FONTFAMILYNAMENOTINFULL: u32 = 285;
pub const E_FONTFILECREATEFAILED: u32 = 515;
pub const E_FONTFILENOTFOUND: u32 = 517;
pub const E_FONTINSTALLFAILED: u32 = 272;
pub const E_FONTNAMEALREADYEXISTS: u32 = 271;
pub const E_FONTNOTEMBEDDABLE: u32 = 260;
pub const E_FONTREFERENCEINVALID: u32 = 8;
pub const E_FONTVARIATIONSIMULATED: u32 = 283;
pub const E_HDCINVALID: u32 = 6;
pub const E_INPUTPARAMINVALID: u32 = 25;
pub const E_NAMECHANGEFAILED: u32 = 259;
pub const E_NOFREEMEMORY: u32 = 7;
pub const E_NONE: u32 = 0;
pub const E_NOOS2: u32 = 265;
pub const E_NOTATRUETYPEFONT: u32 = 10;
pub const E_PBENABLEDINVALID: u32 = 280;
pub const E_PERMISSIONSINVALID: u32 = 279;
pub const E_PRIVSINVALID: u32 = 261;
pub const E_PRIVSTATUSINVALID: u32 = 278;
pub const E_READFROMSTREAMFAILED: u32 = 263;
pub const E_RESERVEDPARAMNOTNULL: u32 = 20;
pub const E_RESOURCEFILECREATEFAILED: u32 = 518;
pub const E_SAVETOSTREAMFAILED: u32 = 264;
pub const E_STATUSINVALID: u32 = 277;
pub const E_STREAMINVALID: u32 = 276;
pub const E_SUBSETTINGEXCEPTION: u32 = 281;
pub const E_SUBSETTINGFAILED: u32 = 262;
pub const E_SUBSTRING_TEST_FAIL: u32 = 282;
pub const E_T2NOFREEMEMORY: u32 = 266;
pub const E_TTC_INDEX_OUT_OF_RANGE: u32 = 24;
pub const E_WINDOWSAPI: u32 = 516;
pub const LICENSE_DEFAULT: u32 = 0;
pub const LICENSE_EDITABLE: u32 = 8;
pub const LICENSE_INSTALLABLE: u32 = 0;
pub const LICENSE_NOEMBEDDING: u32 = 2;
pub const LICENSE_PREVIEWPRINT: u32 = 4;
pub type READEMBEDPROC = Option<unsafe extern "C" fn(param0: *mut core::ffi::c_void, param1: *mut core::ffi::c_void, param2: u32) -> u32>;
pub const TTDELETE_DONTREMOVEFONT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TTEMBEDINFO {
    pub usStructSize: u16,
    pub usRootStrSize: u16,
    pub pusRootStr: *mut u16,
}
impl Default for TTEMBEDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TTEMBED_EMBEDEUDC: u32 = 32;
pub const TTEMBED_EUDCEMBEDDED: u32 = 2;
pub const TTEMBED_FAILIFVARIATIONSIMULATED: u32 = 16;
pub const TTEMBED_RAW: u32 = 0;
pub const TTEMBED_SUBSET: u32 = 1;
pub const TTEMBED_SUBSETCANCEL: u32 = 4;
pub const TTEMBED_TTCOMPRESSED: u32 = 4;
pub const TTEMBED_VARIATIONSIMULATED: u32 = 1;
pub const TTEMBED_WEBOBJECT: u32 = 128;
pub const TTEMBED_XORENCRYPTDATA: u32 = 268435456;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TTLOADINFO {
    pub usStructSize: u16,
    pub usRefStrSize: u16,
    pub pusRefStr: *mut u16,
}
impl Default for TTLOADINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TTLOAD_EUDC_OVERWRITE: u32 = 2;
pub const TTLOAD_EUDC_SET: u32 = 4;
pub const TTLOAD_FONT_IN_SYSSTARTUP: u32 = 2;
pub const TTLOAD_FONT_SUBSETTED: u32 = 1;
pub const TTLOAD_PRIVATE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TTVALIDATIONTESTSPARAMS {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pusCharCodeSet: *mut u16,
}
impl Default for TTVALIDATIONTESTSPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TTVALIDATIONTESTSPARAMSEX {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pulCharCodeSet: *mut u32,
}
impl Default for TTVALIDATIONTESTSPARAMSEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WRITEEMBEDPROC = Option<unsafe extern "C" fn(param0: *mut core::ffi::c_void, param1: *const core::ffi::c_void, param2: u32) -> u32>;
