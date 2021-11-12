#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmAssociateContext(param0: super::super::super::Foundation::HWND, param1: super::super::super::Globalization::HIMC) -> super::super::super::Globalization::HIMC;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmAssociateContextEx(param0: super::super::super::Foundation::HWND, param1: super::super::super::Globalization::HIMC, param2: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmConfigureIMEA(param0: super::super::TextServices::HKL, param1: super::super::super::Foundation::HWND, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmConfigureIMEW(param0: super::super::TextServices::HKL, param1: super::super::super::Foundation::HWND, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmCreateContext() -> super::super::super::Globalization::HIMC;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmCreateIMCC(param0: u32) -> super::super::super::Globalization::HIMCC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmCreateSoftKeyboard(param0: u32, param1: super::super::super::Foundation::HWND, param2: i32, param3: i32) -> super::super::super::Foundation::HWND;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmDestroyContext(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmDestroyIMCC(param0: super::super::super::Globalization::HIMCC) -> super::super::super::Globalization::HIMCC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmDestroySoftKeyboard(param0: super::super::super::Foundation::HWND) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmDisableIME(param0: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmDisableLegacyIME() -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmDisableTextFrameService(idthread: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmEnumInputContext(idthread: u32, lpfn: IMCENUMPROC, lparam: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmEnumRegisterWordA(param0: super::super::TextServices::HKL, param1: REGISTERWORDENUMPROCA, lpszreading: super::super::super::Foundation::PSTR, param3: u32, lpszregister: super::super::super::Foundation::PSTR, param5: *mut ::core::ffi::c_void) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmEnumRegisterWordW(param0: super::super::TextServices::HKL, param1: REGISTERWORDENUMPROCW, lpszreading: super::super::super::Foundation::PWSTR, param3: u32, lpszregister: super::super::super::Foundation::PWSTR, param5: *mut ::core::ffi::c_void) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub fn ImmEscapeA(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub fn ImmEscapeW(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGenerateMessage(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCandidateListA(param0: super::super::super::Globalization::HIMC, deindex: u32, lpcandlist: *mut CANDIDATELIST, dwbuflen: u32) -> u32;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCandidateListCountA(param0: super::super::super::Globalization::HIMC, lpdwlistcount: *mut u32) -> u32;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCandidateListCountW(param0: super::super::super::Globalization::HIMC, lpdwlistcount: *mut u32) -> u32;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCandidateListW(param0: super::super::super::Globalization::HIMC, deindex: u32, lpcandlist: *mut CANDIDATELIST, dwbuflen: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetCandidateWindow(param0: super::super::super::Globalization::HIMC, param1: u32, lpcandidate: *mut CANDIDATEFORM) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmGetCompositionFontA(param0: super::super::super::Globalization::HIMC, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmGetCompositionFontW(param0: super::super::super::Globalization::HIMC, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCompositionStringA(param0: super::super::super::Globalization::HIMC, param1: u32, lpbuf: *mut ::core::ffi::c_void, dwbuflen: u32) -> i32;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCompositionStringW(param0: super::super::super::Globalization::HIMC, param1: u32, lpbuf: *mut ::core::ffi::c_void, dwbuflen: u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetCompositionWindow(param0: super::super::super::Globalization::HIMC, lpcompform: *mut COMPOSITIONFORM) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetContext(param0: super::super::super::Foundation::HWND) -> super::super::super::Globalization::HIMC;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetConversionListA(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, lpsrc: super::super::super::Foundation::PSTR, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetConversionListW(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, lpsrc: super::super::super::Foundation::PWSTR, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetConversionStatus(param0: super::super::super::Globalization::HIMC, lpfdwconversion: *mut u32, lpfdwsentence: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmGetDefaultIMEWnd(param0: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetDescriptionA(param0: super::super::TextServices::HKL, lpszdescription: super::super::super::Foundation::PSTR, ubuflen: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetDescriptionW(param0: super::super::TextServices::HKL, lpszdescription: super::super::super::Foundation::PWSTR, ubuflen: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetGuideLineA(param0: super::super::super::Globalization::HIMC, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: super::super::super::Foundation::PSTR, dwbuflen: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetGuideLineW(param0: super::super::super::Globalization::HIMC, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: super::super::super::Foundation::PWSTR, dwbuflen: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmGetHotKey(param0: u32, lpumodifiers: *mut u32, lpuvkey: *mut u32, phkl: *mut isize) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetIMCCLockCount(param0: super::super::super::Globalization::HIMCC) -> u32;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetIMCCSize(param0: super::super::super::Globalization::HIMCC) -> u32;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetIMCLockCount(param0: super::super::super::Globalization::HIMC) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetIMEFileNameA(param0: super::super::TextServices::HKL, lpszfilename: super::super::super::Foundation::PSTR, ubuflen: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetIMEFileNameW(param0: super::super::TextServices::HKL, lpszfilename: super::super::super::Foundation::PWSTR, ubuflen: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmGetImeMenuItemsA(param0: super::super::super::Globalization::HIMC, param1: u32, param2: u32, lpimeparentmenu: *mut IMEMENUITEMINFOA, lpimemenu: *mut IMEMENUITEMINFOA, dwsize: u32) -> u32;
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmGetImeMenuItemsW(param0: super::super::super::Globalization::HIMC, param1: u32, param2: u32, lpimeparentmenu: *mut IMEMENUITEMINFOW, lpimemenu: *mut IMEMENUITEMINFOW, dwsize: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetOpenStatus(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ImmGetProperty(param0: super::super::TextServices::HKL, param1: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetRegisterWordStyleA(param0: super::super::TextServices::HKL, nitem: u32, lpstylebuf: *mut STYLEBUFA) -> u32;
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ImmGetRegisterWordStyleW(param0: super::super::TextServices::HKL, nitem: u32, lpstylebuf: *mut STYLEBUFW) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetStatusWindowPos(param0: super::super::super::Globalization::HIMC, lpptpos: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmGetVirtualKey(param0: super::super::super::Foundation::HWND) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmInstallIMEA(lpszimefilename: super::super::super::Foundation::PSTR, lpszlayouttext: super::super::super::Foundation::PSTR) -> super::super::TextServices::HKL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmInstallIMEW(lpszimefilename: super::super::super::Foundation::PWSTR, lpszlayouttext: super::super::super::Foundation::PWSTR) -> super::super::TextServices::HKL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmIsIME(param0: super::super::TextServices::HKL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmIsUIMessageA(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmIsUIMessageW(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmLockIMC(param0: super::super::super::Globalization::HIMC) -> *mut INPUTCONTEXT;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmLockIMCC(param0: super::super::super::Globalization::HIMCC) -> *mut ::core::ffi::c_void;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmNotifyIME(param0: super::super::super::Globalization::HIMC, dwaction: NOTIFY_IME_ACTION, dwindex: NOTIFY_IME_INDEX, dwvalue: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmReSizeIMCC(param0: super::super::super::Globalization::HIMCC, param1: u32) -> super::super::super::Globalization::HIMCC;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmRegisterWordA(param0: super::super::TextServices::HKL, lpszreading: super::super::super::Foundation::PSTR, param2: u32, lpszregister: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmRegisterWordW(param0: super::super::TextServices::HKL, lpszreading: super::super::super::Foundation::PWSTR, param2: u32, lpszregister: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmReleaseContext(param0: super::super::super::Foundation::HWND, param1: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmRequestMessageA(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::WPARAM, param2: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::LRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmRequestMessageW(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::WPARAM, param2: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::LRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetCandidateWindow(param0: super::super::super::Globalization::HIMC, lpcandidate: *const CANDIDATEFORM) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmSetCompositionFontA(param0: super::super::super::Globalization::HIMC, lplf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmSetCompositionFontW(param0: super::super::super::Globalization::HIMC, lplf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetCompositionStringA(param0: super::super::super::Globalization::HIMC, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: *const ::core::ffi::c_void, dwcomplen: u32, lpread: *const ::core::ffi::c_void, dwreadlen: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetCompositionStringW(param0: super::super::super::Globalization::HIMC, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: *const ::core::ffi::c_void, dwcomplen: u32, lpread: *const ::core::ffi::c_void, dwreadlen: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetCompositionWindow(param0: super::super::super::Globalization::HIMC, lpcompform: *const COMPOSITIONFORM) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetConversionStatus(param0: super::super::super::Globalization::HIMC, param1: u32, param2: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmSetHotKey(param0: u32, param1: u32, param2: u32, param3: super::super::TextServices::HKL) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetOpenStatus(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetStatusWindowPos(param0: super::super::super::Globalization::HIMC, lpptpos: *const super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmShowSoftKeyboard(param0: super::super::super::Foundation::HWND, param1: i32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmSimulateHotKey(param0: super::super::super::Foundation::HWND, param1: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmUnlockIMC(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmUnlockIMCC(param0: super::super::super::Globalization::HIMCC) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmUnregisterWordA(param0: super::super::TextServices::HKL, lpszreading: super::super::super::Foundation::PSTR, param2: u32, lpszunregister: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmUnregisterWordW(param0: super::super::TextServices::HKL, lpszreading: super::super::super::Foundation::PWSTR, param2: u32, lpszunregister: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
}
pub struct APPLETIDLIST(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct APPLYCANDEXPARAM(i32);
pub const ATTR_CONVERTED: u32 = 2u32;
pub const ATTR_FIXEDCONVERTED: u32 = 5u32;
pub const ATTR_INPUT: u32 = 0u32;
pub const ATTR_INPUT_ERROR: u32 = 4u32;
pub const ATTR_TARGET_CONVERTED: u32 = 1u32;
pub const ATTR_TARGET_NOTCONVERTED: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CANDIDATEFORM(i32);
pub struct CANDIDATEINFO(i32);
pub struct CANDIDATELIST(i32);
pub const CATID_MSIME_IImePadApplet: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1969670865,
    data2: 20169,
    data3: 17528,
    data4: [159, 233, 142, 215, 102, 97, 158, 223],
};
pub const CATID_MSIME_IImePadApplet1000: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3766608342,
    data2: 9097,
    data3: 17355,
    data4: [182, 111, 96, 159, 130, 61, 159, 156],
};
pub const CATID_MSIME_IImePadApplet1200: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2759833084,
    data2: 32021,
    data3: 16931,
    data4: [167, 137, 183, 129, 191, 154, 230, 103],
};
pub const CATID_MSIME_IImePadApplet900: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4205728191,
    data2: 24155,
    data3: 18973,
    data4: [141, 225, 23, 193, 217, 225, 114, 141],
};
pub const CATID_MSIME_IImePadApplet_VER7: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1242533425, data2: 50158, data3: 4561, data4: [175, 239, 0, 128, 95, 12, 139, 109] };
pub const CATID_MSIME_IImePadApplet_VER80: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1459070866, data2: 65265, data3: 4563, data4: [132, 99, 0, 192, 79, 122, 6, 229] };
pub const CATID_MSIME_IImePadApplet_VER81: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1701126320, data2: 48008, data3: 4564, data4: [132, 192, 0, 192, 79, 122, 6, 229] };
pub struct CActiveIMM(i32);
pub const CFS_CANDIDATEPOS: u32 = 64u32;
pub const CFS_DEFAULT: u32 = 0u32;
pub const CFS_EXCLUDE: u32 = 128u32;
pub const CFS_FORCE_POSITION: u32 = 32u32;
pub const CFS_POINT: u32 = 2u32;
pub const CFS_RECT: u32 = 1u32;
pub const CHARINFO_APPLETID_MASK: u32 = 4278190080u32;
pub const CHARINFO_CHARID_MASK: u32 = 65535u32;
pub const CHARINFO_FEID_MASK: u32 = 15728640u32;
pub const CLSID_ImePlugInDictDictionaryList_CHS: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2079330971,
    data2: 23535,
    data3: 19940,
    data4: [155, 11, 94, 219, 102, 172, 47, 166],
};
pub const CLSID_ImePlugInDictDictionaryList_JPN: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1340241771,
    data2: 45305,
    data3: 17302,
    data4: [181, 252, 233, 212, 207, 30, 193, 149],
};
pub const CLSID_VERSION_DEPENDENT_MSIME_JAPANESE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1787888286,
    data2: 43593,
    data3: 18203,
    data4: [174, 231, 125, 51, 39, 133, 102, 13],
};
#[cfg(feature = "Win32_Foundation")]
pub struct COMPOSITIONFORM(i32);
pub struct COMPOSITIONSTRING(i32);
pub const CS_INSERTCHAR: u32 = 8192u32;
pub const CS_NOMOVECARET: u32 = 16384u32;
pub const E_LARGEINPUT: u32 = 51u32;
pub const E_NOCAND: u32 = 48u32;
pub const E_NOTENOUGH_BUFFER: u32 = 49u32;
pub const E_NOTENOUGH_WDD: u32 = 50u32;
pub const FEID_CHINESE_HONGKONG: u32 = 3u32;
pub const FEID_CHINESE_SIMPLIFIED: u32 = 2u32;
pub const FEID_CHINESE_SINGAPORE: u32 = 4u32;
pub const FEID_CHINESE_TRADITIONAL: u32 = 1u32;
pub const FEID_JAPANESE: u32 = 5u32;
pub const FEID_KOREAN: u32 = 6u32;
pub const FEID_KOREAN_JOHAB: u32 = 7u32;
pub const FEID_NONE: u32 = 0u32;
pub const FELANG_CLMN_FIXD: u32 = 32u32;
pub const FELANG_CLMN_FIXR: u32 = 16u32;
pub const FELANG_CLMN_NOPBREAK: u32 = 8u32;
pub const FELANG_CLMN_NOWBREAK: u32 = 2u32;
pub const FELANG_CLMN_PBREAK: u32 = 4u32;
pub const FELANG_CLMN_WBREAK: u32 = 1u32;
pub const FELANG_CMODE_AUTOMATIC: u32 = 134217728u32;
pub const FELANG_CMODE_BESTFIRST: u32 = 16384u32;
pub const FELANG_CMODE_BOPOMOFO: u32 = 64u32;
pub const FELANG_CMODE_CONVERSATION: u32 = 536870912u32;
pub const FELANG_CMODE_FULLWIDTHOUT: u32 = 32u32;
pub const FELANG_CMODE_HALFWIDTHOUT: u32 = 16u32;
pub const FELANG_CMODE_HANGUL: u32 = 128u32;
pub const FELANG_CMODE_HIRAGANAOUT: u32 = 0u32;
pub const FELANG_CMODE_KATAKANAOUT: u32 = 8u32;
pub const FELANG_CMODE_MERGECAND: u32 = 4096u32;
pub const FELANG_CMODE_MONORUBY: u32 = 2u32;
pub const FELANG_CMODE_NAME: u32 = 268435456u32;
pub const FELANG_CMODE_NOINVISIBLECHAR: u32 = 1073741824u32;
pub const FELANG_CMODE_NONE: u32 = 16777216u32;
pub const FELANG_CMODE_NOPRUNING: u32 = 4u32;
pub const FELANG_CMODE_PHRASEPREDICT: u32 = 268435456u32;
pub const FELANG_CMODE_PINYIN: u32 = 256u32;
pub const FELANG_CMODE_PLAURALCLAUSE: u32 = 33554432u32;
pub const FELANG_CMODE_PRECONV: u32 = 512u32;
pub const FELANG_CMODE_RADICAL: u32 = 1024u32;
pub const FELANG_CMODE_ROMAN: u32 = 8192u32;
pub const FELANG_CMODE_SINGLECONVERT: u32 = 67108864u32;
pub const FELANG_CMODE_UNKNOWNREADING: u32 = 2048u32;
pub const FELANG_CMODE_USENOREVWORDS: u32 = 32768u32;
pub const FELANG_INVALD_PO: u32 = 65535u32;
pub const FELANG_REQ_CONV: u32 = 65536u32;
pub const FELANG_REQ_RECONV: u32 = 131072u32;
pub const FELANG_REQ_REV: u32 = 196608u32;
pub const FID_MSIME_KMS_DEL_KEYLIST: u32 = 4u32;
pub const FID_MSIME_KMS_FUNCDESC: u32 = 9u32;
pub const FID_MSIME_KMS_GETMAP: u32 = 6u32;
pub const FID_MSIME_KMS_GETMAPFAST: u32 = 11u32;
pub const FID_MSIME_KMS_GETMAPSEAMLESS: u32 = 10u32;
pub const FID_MSIME_KMS_INIT: u32 = 2u32;
pub const FID_MSIME_KMS_INVOKE: u32 = 7u32;
pub const FID_MSIME_KMS_NOTIFY: u32 = 5u32;
pub const FID_MSIME_KMS_SETMAP: u32 = 8u32;
pub const FID_MSIME_KMS_TERM: u32 = 3u32;
pub const FID_MSIME_KMS_VERSION: u32 = 1u32;
pub const FID_MSIME_VERSION: u32 = 0u32;
pub const FID_RECONVERT_VERSION: u32 = 268435456u32;
pub const GCSEX_CANCELRECONVERT: u32 = 268435456u32;
pub const GCS_COMPATTR: u32 = 16u32;
pub const GCS_COMPCLAUSE: u32 = 32u32;
pub const GCS_COMPREADATTR: u32 = 2u32;
pub const GCS_COMPREADCLAUSE: u32 = 4u32;
pub const GCS_COMPREADSTR: u32 = 1u32;
pub const GCS_COMPSTR: u32 = 8u32;
pub const GCS_CURSORPOS: u32 = 128u32;
pub const GCS_DELTASTART: u32 = 256u32;
pub const GCS_RESULTCLAUSE: u32 = 4096u32;
pub const GCS_RESULTREADCLAUSE: u32 = 1024u32;
pub const GCS_RESULTREADSTR: u32 = 512u32;
pub const GCS_RESULTSTR: u32 = 2048u32;
pub struct GET_CONVERSION_LIST_FLAG(i32);
pub struct GET_GUIDE_LINE_TYPE(i32);
pub const GL_ID_CANNOTSAVE: u32 = 17u32;
pub const GL_ID_CHOOSECANDIDATE: u32 = 40u32;
pub const GL_ID_INPUTCODE: u32 = 38u32;
pub const GL_ID_INPUTRADICAL: u32 = 37u32;
pub const GL_ID_INPUTREADING: u32 = 36u32;
pub const GL_ID_INPUTSYMBOL: u32 = 39u32;
pub const GL_ID_NOCONVERT: u32 = 32u32;
pub const GL_ID_NODICTIONARY: u32 = 16u32;
pub const GL_ID_NOMODULE: u32 = 1u32;
pub const GL_ID_PRIVATE_FIRST: u32 = 32768u32;
pub const GL_ID_PRIVATE_LAST: u32 = 65535u32;
pub const GL_ID_READINGCONFLICT: u32 = 35u32;
pub const GL_ID_REVERSECONVERSION: u32 = 41u32;
pub const GL_ID_TOOMANYSTROKE: u32 = 34u32;
pub const GL_ID_TYPINGERROR: u32 = 33u32;
pub const GL_ID_UNKNOWN: u32 = 0u32;
pub const GL_LEVEL_ERROR: u32 = 2u32;
pub const GL_LEVEL_FATAL: u32 = 1u32;
pub const GL_LEVEL_INFORMATION: u32 = 4u32;
pub const GL_LEVEL_NOGUIDELINE: u32 = 0u32;
pub const GL_LEVEL_WARNING: u32 = 3u32;
pub struct GUIDELINE(i32);
pub const IACE_CHILDREN: u32 = 1u32;
pub const IACE_DEFAULT: u32 = 16u32;
pub const IACE_IGNORENOCONTEXT: u32 = 32u32;
#[repr(transparent)]
pub struct IActiveIME(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveIME2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveIMMApp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveIMMIME(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveIMMMessagePumpOwner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveIMMRegistrar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumInputContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumRegisterWordA(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumRegisterWordW(pub *mut ::core::ffi::c_void);
pub const IFEC_S_ALREADY_DEFAULT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(291840i32 as _);
#[repr(transparent)]
pub struct IFEClassFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFECommon(pub *mut ::core::ffi::c_void);
pub const IFED_E_INVALID_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147192063i32 as _);
pub const IFED_E_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147192064i32 as _);
pub const IFED_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147192057i32 as _);
pub const IFED_E_NOT_USER_DIC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147192058i32 as _);
pub const IFED_E_NO_ENTRY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147192060i32 as _);
pub const IFED_E_OPEN_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147192062i32 as _);
pub const IFED_E_REGISTER_DISCONNECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147192053i32 as _);
pub const IFED_E_REGISTER_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147192059i32 as _);
pub const IFED_E_REGISTER_ILLEGAL_POS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147192055i32 as _);
pub const IFED_E_REGISTER_IMPROPER_WORD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147192054i32 as _);
pub const IFED_E_USER_COMMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147192056i32 as _);
pub const IFED_E_WRITE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147192061i32 as _);
pub const IFED_POS_ADJECTIVE: u32 = 4u32;
pub const IFED_POS_ADJECTIVE_VERB: u32 = 8u32;
pub const IFED_POS_ADNOUN: u32 = 32u32;
pub const IFED_POS_ADVERB: u32 = 16u32;
pub const IFED_POS_AFFIX: u32 = 1536u32;
pub const IFED_POS_ALL: u32 = 131071u32;
pub const IFED_POS_AUXILIARY_VERB: u32 = 32768u32;
pub const IFED_POS_CONJUNCTION: u32 = 64u32;
pub const IFED_POS_DEPENDENT: u32 = 114688u32;
pub const IFED_POS_IDIOMS: u32 = 4096u32;
pub const IFED_POS_INDEPENDENT: u32 = 255u32;
pub const IFED_POS_INFLECTIONALSUFFIX: u32 = 256u32;
pub const IFED_POS_INTERJECTION: u32 = 128u32;
pub const IFED_POS_NONE: u32 = 0u32;
pub const IFED_POS_NOUN: u32 = 1u32;
pub const IFED_POS_PARTICLE: u32 = 16384u32;
pub const IFED_POS_PREFIX: u32 = 512u32;
pub const IFED_POS_SUB_VERB: u32 = 65536u32;
pub const IFED_POS_SUFFIX: u32 = 1024u32;
pub const IFED_POS_SYMBOLS: u32 = 8192u32;
pub const IFED_POS_TANKANJI: u32 = 2048u32;
pub const IFED_POS_VERB: u32 = 2u32;
pub const IFED_REG_ALL: u32 = 7u32;
pub const IFED_REG_AUTO: u32 = 2u32;
pub const IFED_REG_GRAMMAR: u32 = 4u32;
pub const IFED_REG_NONE: u32 = 0u32;
pub const IFED_REG_USER: u32 = 1u32;
pub const IFED_SELECT_ALL: u32 = 15u32;
pub const IFED_SELECT_COMMENT: u32 = 8u32;
pub const IFED_SELECT_DISPLAY: u32 = 2u32;
pub const IFED_SELECT_NONE: u32 = 0u32;
pub const IFED_SELECT_POS: u32 = 4u32;
pub const IFED_SELECT_READING: u32 = 1u32;
pub const IFED_S_COMMENT_CHANGED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(291331i32 as _);
pub const IFED_S_EMPTY_DICTIONARY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(291329i32 as _);
pub const IFED_S_MORE_ENTRIES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(291328i32 as _);
pub const IFED_S_WORD_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(291330i32 as _);
pub const IFED_TYPE_ALL: u32 = 31u32;
pub const IFED_TYPE_ENGLISH: u32 = 16u32;
pub const IFED_TYPE_GENERAL: u32 = 1u32;
pub const IFED_TYPE_NAMEPLACE: u32 = 2u32;
pub const IFED_TYPE_NONE: u32 = 0u32;
pub const IFED_TYPE_REVERSE: u32 = 8u32;
pub const IFED_TYPE_SPEECH: u32 = 4u32;
#[repr(transparent)]
pub struct IFEDictionary(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFELanguage(pub *mut ::core::ffi::c_void);
pub const IGIMIF_RIGHTMENU: u32 = 1u32;
pub const IGIMII_CMODE: u32 = 1u32;
pub const IGIMII_CONFIGURE: u32 = 4u32;
pub const IGIMII_HELP: u32 = 16u32;
pub const IGIMII_INPUTTOOLS: u32 = 64u32;
pub const IGIMII_OTHER: u32 = 32u32;
pub const IGIMII_SMODE: u32 = 2u32;
pub const IGIMII_TOOLS: u32 = 8u32;
#[repr(transparent)]
pub struct IImePad(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImePadApplet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImePlugInDictDictionaryList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImeSpecifyApplets(pub *mut ::core::ffi::c_void);
pub struct IMCENUMPROC(i32);
pub const IMC_CLOSESTATUSWINDOW: u32 = 33u32;
pub const IMC_GETCANDIDATEPOS: u32 = 7u32;
pub const IMC_GETCOMPOSITIONFONT: u32 = 9u32;
pub const IMC_GETCOMPOSITIONWINDOW: u32 = 11u32;
pub const IMC_GETSOFTKBDFONT: u32 = 17u32;
pub const IMC_GETSOFTKBDPOS: u32 = 19u32;
pub const IMC_GETSOFTKBDSUBTYPE: u32 = 21u32;
pub const IMC_GETSTATUSWINDOWPOS: u32 = 15u32;
pub const IMC_OPENSTATUSWINDOW: u32 = 34u32;
pub const IMC_SETCANDIDATEPOS: u32 = 8u32;
pub const IMC_SETCOMPOSITIONFONT: u32 = 10u32;
pub const IMC_SETCOMPOSITIONWINDOW: u32 = 12u32;
pub const IMC_SETCONVERSIONMODE: u32 = 2u32;
pub const IMC_SETOPENSTATUS: u32 = 6u32;
pub const IMC_SETSENTENCEMODE: u32 = 4u32;
pub const IMC_SETSOFTKBDDATA: u32 = 24u32;
pub const IMC_SETSOFTKBDFONT: u32 = 18u32;
pub const IMC_SETSOFTKBDPOS: u32 = 20u32;
pub const IMC_SETSOFTKBDSUBTYPE: u32 = 22u32;
pub const IMC_SETSTATUSWINDOWPOS: u32 = 16u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct IMEAPPLETCFG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct IMEAPPLETUI(i32);
pub struct IMECHARINFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct IMECHARPOSITION(i32);
pub struct IMECOMPOSITIONSTRINGINFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct IMEDLG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct IMEDP(i32);
pub const IMEFAREASTINFO_TYPE_COMMENT: u32 = 2u32;
pub const IMEFAREASTINFO_TYPE_COSTTIME: u32 = 3u32;
pub const IMEFAREASTINFO_TYPE_DEFAULT: u32 = 0u32;
pub const IMEFAREASTINFO_TYPE_READING: u32 = 1u32;
pub struct IMEFMT(i32);
pub struct IMEINFO(i32);
pub struct IMEITEM(i32);
pub struct IMEITEMCANDIDATE(i32);
pub const IMEKEYCTRLMASK_ALT: u32 = 1u32;
pub const IMEKEYCTRLMASK_CTRL: u32 = 2u32;
pub const IMEKEYCTRLMASK_SHIFT: u32 = 4u32;
pub const IMEKEYCTRL_DOWN: u32 = 0u32;
pub const IMEKEYCTRL_UP: u32 = 1u32;
#[cfg(feature = "Win32_Globalization")]
pub struct IMEKMS(i32);
pub struct IMEKMSFUNCDESC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct IMEKMSINIT(i32);
#[cfg(feature = "Win32_Globalization")]
pub struct IMEKMSINVK(i32);
pub struct IMEKMSKEY(i32);
#[cfg(feature = "Win32_Globalization")]
pub struct IMEKMSKMP(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
pub struct IMEKMSNTFY(i32);
pub const IMEKMS_2NDLEVEL: u32 = 4u32;
pub const IMEKMS_CANDIDATE: u32 = 6u32;
pub const IMEKMS_COMPOSITION: u32 = 1u32;
pub const IMEKMS_IMEOFF: u32 = 3u32;
pub const IMEKMS_INPTGL: u32 = 5u32;
pub const IMEKMS_NOCOMPOSITION: u32 = 0u32;
pub const IMEKMS_SELECTION: u32 = 2u32;
pub const IMEKMS_TYPECAND: u32 = 7u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct IMEMENUITEMINFOA(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct IMEMENUITEMINFOW(i32);
pub const IMEMENUITEM_STRING_SIZE: u32 = 80u32;
pub const IMEMOUSERET_NOTHANDLED: i32 = -1i32;
pub const IMEMOUSE_LDOWN: u32 = 1u32;
pub const IMEMOUSE_MDOWN: u32 = 4u32;
pub const IMEMOUSE_NONE: u32 = 0u32;
pub const IMEMOUSE_RDOWN: u32 = 2u32;
pub const IMEMOUSE_VERSION: u32 = 255u32;
pub const IMEMOUSE_WDOWN: u32 = 32u32;
pub const IMEMOUSE_WUP: u32 = 16u32;
pub const IMEPADCTRL_CARETBACKSPACE: u32 = 10u32;
pub const IMEPADCTRL_CARETBOTTOM: u32 = 9u32;
pub const IMEPADCTRL_CARETDELETE: u32 = 11u32;
pub const IMEPADCTRL_CARETLEFT: u32 = 6u32;
pub const IMEPADCTRL_CARETRIGHT: u32 = 7u32;
pub const IMEPADCTRL_CARETSET: u32 = 5u32;
pub const IMEPADCTRL_CARETTOP: u32 = 8u32;
pub const IMEPADCTRL_CLEARALL: u32 = 4u32;
pub const IMEPADCTRL_CONVERTALL: u32 = 1u32;
pub const IMEPADCTRL_DETERMINALL: u32 = 2u32;
pub const IMEPADCTRL_DETERMINCHAR: u32 = 3u32;
pub const IMEPADCTRL_INSERTFULLSPACE: u32 = 14u32;
pub const IMEPADCTRL_INSERTHALFSPACE: u32 = 15u32;
pub const IMEPADCTRL_INSERTSPACE: u32 = 13u32;
pub const IMEPADCTRL_OFFIME: u32 = 17u32;
pub const IMEPADCTRL_OFFPRECONVERSION: u32 = 19u32;
pub const IMEPADCTRL_ONIME: u32 = 16u32;
pub const IMEPADCTRL_ONPRECONVERSION: u32 = 18u32;
pub const IMEPADCTRL_PHONETICCANDIDATE: u32 = 20u32;
pub const IMEPADCTRL_PHRASEDELETE: u32 = 12u32;
pub const IMEPADREQ_CHANGESTRINGCANDIDATEINFO: u32 = 4111u32;
pub const IMEPADREQ_CHANGESTRINGINFO: u32 = 4115u32;
pub const IMEPADREQ_FIRST: u32 = 4096u32;
pub const IMEPADREQ_GETAPPLETDATA: u32 = 4106u32;
pub const IMEPADREQ_GETCOMPOSITIONSTRINGID: u32 = 4109u32;
pub const IMEPADREQ_GETCURRENTUILANGID: u32 = 4120u32;
pub const IMEPADREQ_GETSELECTEDSTRING: u32 = 4103u32;
pub const IMEPADREQ_INSERTITEMCANDIDATE: u32 = 4099u32;
pub const IMEPADREQ_INSERTSTRINGCANDIDATE: u32 = 4098u32;
pub const IMEPADREQ_INSERTSTRINGCANDIDATEINFO: u32 = 4110u32;
pub const IMEPADREQ_INSERTSTRINGINFO: u32 = 4114u32;
pub const IMEPADREQ_SENDKEYCONTROL: u32 = 4101u32;
pub const IMEPADREQ_SETAPPLETDATA: u32 = 4105u32;
pub const IMEPADREQ_SETTITLEFONT: u32 = 4107u32;
pub const IMEPN_ACTIVATE: u32 = 257u32;
pub const IMEPN_APPLYCAND: u32 = 267u32;
pub const IMEPN_APPLYCANDEX: u32 = 268u32;
pub const IMEPN_CONFIG: u32 = 264u32;
pub const IMEPN_FIRST: u32 = 256u32;
pub const IMEPN_HELP: u32 = 265u32;
pub const IMEPN_HIDE: u32 = 261u32;
pub const IMEPN_INACTIVATE: u32 = 258u32;
pub const IMEPN_QUERYCAND: u32 = 266u32;
pub const IMEPN_SETTINGCHANGED: u32 = 269u32;
pub const IMEPN_SHOW: u32 = 260u32;
pub const IMEPN_SIZECHANGED: u32 = 263u32;
pub const IMEPN_SIZECHANGING: u32 = 262u32;
pub const IMEPN_USER: u32 = 356u32;
pub struct IMEREG(i32);
pub struct IMEREL(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct IMESHF(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct IMESTRINGCANDIDATE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct IMESTRINGCANDIDATEINFO(i32);
pub struct IMEUCT(i32);
pub const IMEVER_0310: u32 = 196618u32;
pub const IMEVER_0400: u32 = 262144u32;
#[cfg(feature = "Win32_Foundation")]
pub struct IMEWRD(i32);
pub const IME_CAND_CODE: u32 = 2u32;
pub const IME_CAND_MEANING: u32 = 3u32;
pub const IME_CAND_RADICAL: u32 = 4u32;
pub const IME_CAND_READ: u32 = 1u32;
pub const IME_CAND_STROKE: u32 = 5u32;
pub const IME_CAND_UNKNOWN: u32 = 0u32;
pub const IME_CHOTKEY_IME_NONIME_TOGGLE: u32 = 16u32;
pub const IME_CHOTKEY_SHAPE_TOGGLE: u32 = 17u32;
pub const IME_CHOTKEY_SYMBOL_TOGGLE: u32 = 18u32;
pub const IME_CMODE_EUDC: u32 = 512u32;
pub const IME_CMODE_FIXED: u32 = 2048u32;
pub const IME_CMODE_NOCONVERSION: u32 = 256u32;
pub const IME_CMODE_RESERVED: u32 = 4026531840u32;
pub const IME_CMODE_SOFTKBD: u32 = 128u32;
pub const IME_CMODE_SYMBOL: u32 = 1024u32;
pub const IME_CONFIG_GENERAL: u32 = 1u32;
pub const IME_CONFIG_REGISTERWORD: u32 = 2u32;
pub const IME_CONFIG_SELECTDICTIONARY: u32 = 3u32;
pub const IME_ESC_AUTOMATA: u32 = 4105u32;
pub const IME_ESC_GETHELPFILENAME: u32 = 4107u32;
pub const IME_ESC_GET_EUDC_DICTIONARY: u32 = 4099u32;
pub const IME_ESC_HANJA_MODE: u32 = 4104u32;
pub const IME_ESC_IME_NAME: u32 = 4102u32;
pub const IME_ESC_MAX_KEY: u32 = 4101u32;
pub const IME_ESC_PRIVATE_FIRST: u32 = 2048u32;
pub const IME_ESC_PRIVATE_HOTKEY: u32 = 4106u32;
pub const IME_ESC_PRIVATE_LAST: u32 = 4095u32;
pub const IME_ESC_QUERY_SUPPORT: u32 = 3u32;
pub const IME_ESC_RESERVED_FIRST: u32 = 4u32;
pub const IME_ESC_RESERVED_LAST: u32 = 2047u32;
pub const IME_ESC_SEQUENCE_TO_INTERNAL: u32 = 4097u32;
pub const IME_ESC_SET_EUDC_DICTIONARY: u32 = 4100u32;
pub const IME_ESC_STRING_BUFFER_SIZE: u32 = 80u32;
pub const IME_ESC_SYNC_HOTKEY: u32 = 4103u32;
pub const IME_HOTKEY_DSWITCH_FIRST: u32 = 256u32;
pub const IME_HOTKEY_DSWITCH_LAST: u32 = 287u32;
pub const IME_HOTKEY_PRIVATE_FIRST: u32 = 512u32;
pub const IME_HOTKEY_PRIVATE_LAST: u32 = 543u32;
pub const IME_ITHOTKEY_PREVIOUS_COMPOSITION: u32 = 513u32;
pub const IME_ITHOTKEY_RECONVERTSTRING: u32 = 515u32;
pub const IME_ITHOTKEY_RESEND_RESULTSTR: u32 = 512u32;
pub const IME_ITHOTKEY_UISTYLE_TOGGLE: u32 = 514u32;
pub const IME_JHOTKEY_CLOSE_OPEN: u32 = 48u32;
pub const IME_KHOTKEY_ENGLISH: u32 = 82u32;
pub const IME_KHOTKEY_HANJACONVERT: u32 = 81u32;
pub const IME_KHOTKEY_SHAPE_TOGGLE: u32 = 80u32;
pub struct IME_PAD_REQUEST_FLAGS(i32);
pub const IME_PROP_ACCEPT_WIDE_VKEY: u32 = 32u32;
pub const IME_PROP_AT_CARET: u32 = 65536u32;
pub const IME_PROP_CANDLIST_START_FROM_1: u32 = 262144u32;
pub const IME_PROP_COMPLETE_ON_UNSELECT: u32 = 1048576u32;
pub const IME_PROP_END_UNLOAD: u32 = 1u32;
pub const IME_PROP_IGNORE_UPKEYS: u32 = 4u32;
pub const IME_PROP_KBD_CHAR_FIRST: u32 = 2u32;
pub const IME_PROP_NEED_ALTKEY: u32 = 8u32;
pub const IME_PROP_NO_KEYS_ON_CLOSE: u32 = 16u32;
pub const IME_PROP_SPECIAL_UI: u32 = 131072u32;
pub const IME_PROP_UNICODE: u32 = 524288u32;
pub const IME_REGWORD_STYLE_EUDC: u32 = 1u32;
pub const IME_REGWORD_STYLE_USER_FIRST: u32 = 2147483648u32;
pub const IME_REGWORD_STYLE_USER_LAST: u32 = 4294967295u32;
pub const IME_SMODE_AUTOMATIC: u32 = 4u32;
pub const IME_SMODE_CONVERSATION: u32 = 16u32;
pub const IME_SMODE_NONE: u32 = 0u32;
pub const IME_SMODE_PHRASEPREDICT: u32 = 8u32;
pub const IME_SMODE_PLAURALCLAUSE: u32 = 1u32;
pub const IME_SMODE_RESERVED: u32 = 61440u32;
pub const IME_SMODE_SINGLECONVERT: u32 = 2u32;
pub const IME_SYSINFO_WINLOGON: u32 = 1u32;
pub const IME_SYSINFO_WOW16: u32 = 2u32;
pub const IME_THOTKEY_IME_NONIME_TOGGLE: u32 = 112u32;
pub const IME_THOTKEY_SHAPE_TOGGLE: u32 = 113u32;
pub const IME_THOTKEY_SYMBOL_TOGGLE: u32 = 114u32;
pub const IME_UI_CLASS_NAME_SIZE: u32 = 16u32;
pub const IMFT_RADIOCHECK: u32 = 1u32;
pub const IMFT_SEPARATOR: u32 = 2u32;
pub const IMFT_SUBMENU: u32 = 4u32;
pub const IMMGWLP_IMC: u32 = 0u32;
pub const IMMGWL_IMC: u32 = 0u32;
pub const IMM_ERROR_GENERAL: i32 = -2i32;
pub const IMM_ERROR_NODATA: i32 = -1i32;
pub const IMN_CHANGECANDIDATE: u32 = 3u32;
pub const IMN_CLOSECANDIDATE: u32 = 4u32;
pub const IMN_CLOSESTATUSWINDOW: u32 = 1u32;
pub const IMN_GUIDELINE: u32 = 13u32;
pub const IMN_OPENCANDIDATE: u32 = 5u32;
pub const IMN_OPENSTATUSWINDOW: u32 = 2u32;
pub const IMN_PRIVATE: u32 = 14u32;
pub const IMN_SETCANDIDATEPOS: u32 = 9u32;
pub const IMN_SETCOMPOSITIONFONT: u32 = 10u32;
pub const IMN_SETCOMPOSITIONWINDOW: u32 = 11u32;
pub const IMN_SETCONVERSIONMODE: u32 = 6u32;
pub const IMN_SETOPENSTATUS: u32 = 8u32;
pub const IMN_SETSENTENCEMODE: u32 = 7u32;
pub const IMN_SETSTATUSWINDOWPOS: u32 = 12u32;
pub const IMN_SOFTKBDDESTROYED: u32 = 17u32;
pub const IMR_CANDIDATEWINDOW: u32 = 2u32;
pub const IMR_COMPOSITIONFONT: u32 = 3u32;
pub const IMR_COMPOSITIONWINDOW: u32 = 1u32;
pub const IMR_CONFIRMRECONVERTSTRING: u32 = 5u32;
pub const IMR_DOCUMENTFEED: u32 = 7u32;
pub const IMR_QUERYCHARPOSITION: u32 = 6u32;
pub const IMR_RECONVERTSTRING: u32 = 4u32;
pub const INFOMASK_APPLY_CAND: u32 = 2u32;
pub const INFOMASK_APPLY_CAND_EX: u32 = 4u32;
pub const INFOMASK_BLOCK_CAND: u32 = 262144u32;
pub const INFOMASK_HIDE_CAND: u32 = 131072u32;
pub const INFOMASK_NONE: u32 = 0u32;
pub const INFOMASK_QUERY_CAND: u32 = 1u32;
pub const INFOMASK_STRING_FIX: u32 = 65536u32;
pub const INIT_COMPFORM: u32 = 16u32;
pub const INIT_CONVERSION: u32 = 2u32;
pub const INIT_LOGFONT: u32 = 8u32;
pub const INIT_SENTENCE: u32 = 4u32;
pub const INIT_SOFTKBDPOS: u32 = 32u32;
pub const INIT_STATUSWNDPOS: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
pub struct INPUTCONTEXT(i32);
pub const IPACFG_CATEGORY: i32 = 262144i32;
pub const IPACFG_HELP: i32 = 2i32;
pub const IPACFG_LANG: i32 = 16i32;
pub const IPACFG_NONE: i32 = 0i32;
pub const IPACFG_PROPERTY: i32 = 1i32;
pub const IPACFG_TITLE: i32 = 65536i32;
pub const IPACFG_TITLEFONTFACE: i32 = 131072i32;
pub const IPACID_CHARLIST: u32 = 9u32;
pub const IPACID_EPWING: u32 = 7u32;
pub const IPACID_HANDWRITING: u32 = 2u32;
pub const IPACID_NONE: u32 = 0u32;
pub const IPACID_OCR: u32 = 8u32;
pub const IPACID_RADICALSEARCH: u32 = 4u32;
pub const IPACID_SOFTKEY: u32 = 1u32;
pub const IPACID_STROKESEARCH: u32 = 3u32;
pub const IPACID_SYMBOLSEARCH: u32 = 5u32;
pub const IPACID_USER: u32 = 256u32;
pub const IPACID_VOICE: u32 = 6u32;
pub const IPAWS_ENABLED: i32 = 1i32;
pub const IPAWS_HORIZONTALFIXED: i32 = 512i32;
pub const IPAWS_MAXHEIGHTFIXED: i32 = 8192i32;
pub const IPAWS_MAXSIZEFIXED: i32 = 12288i32;
pub const IPAWS_MAXWIDTHFIXED: i32 = 4096i32;
pub const IPAWS_MINHEIGHTFIXED: i32 = 131072i32;
pub const IPAWS_MINSIZEFIXED: i32 = 196608i32;
pub const IPAWS_MINWIDTHFIXED: i32 = 65536i32;
pub const IPAWS_SIZEFIXED: i32 = 768i32;
pub const IPAWS_SIZINGNOTIFY: i32 = 4i32;
pub const IPAWS_VERTICALFIXED: i32 = 256i32;
pub const ISC_SHOWUIALL: u32 = 3221225487u32;
pub const ISC_SHOWUIALLCANDIDATEWINDOW: u32 = 15u32;
pub const ISC_SHOWUICANDIDATEWINDOW: u32 = 1u32;
pub const ISC_SHOWUICOMPOSITIONWINDOW: u32 = 2147483648u32;
pub const ISC_SHOWUIGUIDELINE: u32 = 1073741824u32;
pub const JPOS_1DAN: u32 = 213u32;
pub const JPOS_4DAN_HA: u32 = 212u32;
pub const JPOS_5DAN_AWA: u32 = 200u32;
pub const JPOS_5DAN_AWAUON: u32 = 209u32;
pub const JPOS_5DAN_BA: u32 = 206u32;
pub const JPOS_5DAN_GA: u32 = 202u32;
pub const JPOS_5DAN_KA: u32 = 201u32;
pub const JPOS_5DAN_KASOKUON: u32 = 210u32;
pub const JPOS_5DAN_MA: u32 = 207u32;
pub const JPOS_5DAN_NA: u32 = 205u32;
pub const JPOS_5DAN_RA: u32 = 208u32;
pub const JPOS_5DAN_RAHEN: u32 = 211u32;
pub const JPOS_5DAN_SA: u32 = 203u32;
pub const JPOS_5DAN_TA: u32 = 204u32;
pub const JPOS_BUPPIN: u32 = 122u32;
pub const JPOS_CHIMEI: u32 = 109u32;
pub const JPOS_CHIMEI_EKI: u32 = 117u32;
pub const JPOS_CHIMEI_GUN: u32 = 112u32;
pub const JPOS_CHIMEI_KEN: u32 = 111u32;
pub const JPOS_CHIMEI_KU: u32 = 113u32;
pub const JPOS_CHIMEI_KUNI: u32 = 110u32;
pub const JPOS_CHIMEI_MACHI: u32 = 115u32;
pub const JPOS_CHIMEI_MURA: u32 = 116u32;
pub const JPOS_CHIMEI_SHI: u32 = 114u32;
pub const JPOS_CLOSEBRACE: u32 = 911u32;
pub const JPOS_DAIMEISHI: u32 = 123u32;
pub const JPOS_DAIMEISHI_NINSHOU: u32 = 124u32;
pub const JPOS_DAIMEISHI_SHIJI: u32 = 125u32;
pub const JPOS_DOKURITSUGO: u32 = 903u32;
pub const JPOS_EIJI: u32 = 906u32;
pub const JPOS_FUKUSHI: u32 = 500u32;
pub const JPOS_FUKUSHI_DA: u32 = 504u32;
pub const JPOS_FUKUSHI_NANO: u32 = 503u32;
pub const JPOS_FUKUSHI_NI: u32 = 502u32;
pub const JPOS_FUKUSHI_SAHEN: u32 = 501u32;
pub const JPOS_FUKUSHI_TO: u32 = 505u32;
pub const JPOS_FUKUSHI_TOSURU: u32 = 506u32;
pub const JPOS_FUTEIGO: u32 = 904u32;
pub const JPOS_HUKUSIMEISHI: u32 = 104u32;
pub const JPOS_JINMEI: u32 = 106u32;
pub const JPOS_JINMEI_MEI: u32 = 108u32;
pub const JPOS_JINMEI_SEI: u32 = 107u32;
pub const JPOS_KANDOUSHI: u32 = 670u32;
pub const JPOS_KANJI: u32 = 909u32;
pub const JPOS_KANYOUKU: u32 = 902u32;
pub const JPOS_KAZU: u32 = 126u32;
pub const JPOS_KAZU_SURYOU: u32 = 127u32;
pub const JPOS_KAZU_SUSHI: u32 = 128u32;
pub const JPOS_KEIDOU: u32 = 400u32;
pub const JPOS_KEIDOU_GARU: u32 = 403u32;
pub const JPOS_KEIDOU_NO: u32 = 401u32;
pub const JPOS_KEIDOU_TARU: u32 = 402u32;
pub const JPOS_KEIYOU: u32 = 300u32;
pub const JPOS_KEIYOU_GARU: u32 = 301u32;
pub const JPOS_KEIYOU_GE: u32 = 302u32;
pub const JPOS_KEIYOU_ME: u32 = 303u32;
pub const JPOS_KEIYOU_U: u32 = 305u32;
pub const JPOS_KEIYOU_YUU: u32 = 304u32;
pub const JPOS_KENCHIKU: u32 = 121u32;
pub const JPOS_KIGOU: u32 = 905u32;
pub const JPOS_KURU_KI: u32 = 219u32;
pub const JPOS_KURU_KITA: u32 = 220u32;
pub const JPOS_KURU_KITARA: u32 = 221u32;
pub const JPOS_KURU_KITARI: u32 = 222u32;
pub const JPOS_KURU_KITAROU: u32 = 223u32;
pub const JPOS_KURU_KITE: u32 = 224u32;
pub const JPOS_KURU_KO: u32 = 226u32;
pub const JPOS_KURU_KOI: u32 = 227u32;
pub const JPOS_KURU_KOYOU: u32 = 228u32;
pub const JPOS_KURU_KUREBA: u32 = 225u32;
pub const JPOS_KUTEN: u32 = 907u32;
pub const JPOS_MEISA_KEIDOU: u32 = 105u32;
pub const JPOS_MEISHI_FUTSU: u32 = 100u32;
pub const JPOS_MEISHI_KEIYOUDOUSHI: u32 = 103u32;
pub const JPOS_MEISHI_SAHEN: u32 = 101u32;
pub const JPOS_MEISHI_ZAHEN: u32 = 102u32;
pub const JPOS_OPENBRACE: u32 = 910u32;
pub const JPOS_RENTAISHI: u32 = 600u32;
pub const JPOS_RENTAISHI_SHIJI: u32 = 601u32;
pub const JPOS_RENYOU_SETSUBI: u32 = 826u32;
pub const JPOS_SETSUBI: u32 = 800u32;
pub const JPOS_SETSUBI_CHIMEI: u32 = 811u32;
pub const JPOS_SETSUBI_CHOU: u32 = 818u32;
pub const JPOS_SETSUBI_CHU: u32 = 804u32;
pub const JPOS_SETSUBI_DONO: u32 = 835u32;
pub const JPOS_SETSUBI_EKI: u32 = 821u32;
pub const JPOS_SETSUBI_FU: u32 = 805u32;
pub const JPOS_SETSUBI_FUKUSU: u32 = 836u32;
pub const JPOS_SETSUBI_GUN: u32 = 814u32;
pub const JPOS_SETSUBI_JIKAN: u32 = 829u32;
pub const JPOS_SETSUBI_JIKANPLUS: u32 = 830u32;
pub const JPOS_SETSUBI_JINMEI: u32 = 810u32;
pub const JPOS_SETSUBI_JOSUSHI: u32 = 827u32;
pub const JPOS_SETSUBI_JOSUSHIPLUS: u32 = 828u32;
pub const JPOS_SETSUBI_KA: u32 = 803u32;
pub const JPOS_SETSUBI_KATA: u32 = 808u32;
pub const JPOS_SETSUBI_KEN: u32 = 813u32;
pub const JPOS_SETSUBI_KENCHIKU: u32 = 825u32;
pub const JPOS_SETSUBI_KU: u32 = 815u32;
pub const JPOS_SETSUBI_KUN: u32 = 833u32;
pub const JPOS_SETSUBI_KUNI: u32 = 812u32;
pub const JPOS_SETSUBI_MACHI: u32 = 817u32;
pub const JPOS_SETSUBI_MEISHIRENDAKU: u32 = 809u32;
pub const JPOS_SETSUBI_MURA: u32 = 819u32;
pub const JPOS_SETSUBI_RA: u32 = 838u32;
pub const JPOS_SETSUBI_RYU: u32 = 806u32;
pub const JPOS_SETSUBI_SAMA: u32 = 834u32;
pub const JPOS_SETSUBI_SAN: u32 = 832u32;
pub const JPOS_SETSUBI_SEI: u32 = 802u32;
pub const JPOS_SETSUBI_SHAMEI: u32 = 823u32;
pub const JPOS_SETSUBI_SHI: u32 = 816u32;
pub const JPOS_SETSUBI_SON: u32 = 820u32;
pub const JPOS_SETSUBI_SONOTA: u32 = 822u32;
pub const JPOS_SETSUBI_SOSHIKI: u32 = 824u32;
pub const JPOS_SETSUBI_TACHI: u32 = 837u32;
pub const JPOS_SETSUBI_TEINEI: u32 = 831u32;
pub const JPOS_SETSUBI_TEKI: u32 = 801u32;
pub const JPOS_SETSUBI_YOU: u32 = 807u32;
pub const JPOS_SETSUZOKUSHI: u32 = 650u32;
pub const JPOS_SETTOU: u32 = 700u32;
pub const JPOS_SETTOU_CHIMEI: u32 = 710u32;
pub const JPOS_SETTOU_CHOUTAN: u32 = 707u32;
pub const JPOS_SETTOU_DAISHOU: u32 = 705u32;
pub const JPOS_SETTOU_FUKU: u32 = 703u32;
pub const JPOS_SETTOU_JINMEI: u32 = 709u32;
pub const JPOS_SETTOU_JOSUSHI: u32 = 712u32;
pub const JPOS_SETTOU_KAKU: u32 = 701u32;
pub const JPOS_SETTOU_KOUTEI: u32 = 706u32;
pub const JPOS_SETTOU_MI: u32 = 704u32;
pub const JPOS_SETTOU_SAI: u32 = 702u32;
pub const JPOS_SETTOU_SHINKYU: u32 = 708u32;
pub const JPOS_SETTOU_SONOTA: u32 = 711u32;
pub const JPOS_SETTOU_TEINEI_GO: u32 = 714u32;
pub const JPOS_SETTOU_TEINEI_O: u32 = 713u32;
pub const JPOS_SETTOU_TEINEI_ON: u32 = 715u32;
pub const JPOS_SHAMEI: u32 = 119u32;
pub const JPOS_SONOTA: u32 = 118u32;
pub const JPOS_SOSHIKI: u32 = 120u32;
pub const JPOS_SURU_SA: u32 = 229u32;
pub const JPOS_SURU_SE: u32 = 238u32;
pub const JPOS_SURU_SEYO: u32 = 239u32;
pub const JPOS_SURU_SI: u32 = 230u32;
pub const JPOS_SURU_SIATRI: u32 = 233u32;
pub const JPOS_SURU_SITA: u32 = 231u32;
pub const JPOS_SURU_SITARA: u32 = 232u32;
pub const JPOS_SURU_SITAROU: u32 = 234u32;
pub const JPOS_SURU_SITE: u32 = 235u32;
pub const JPOS_SURU_SIYOU: u32 = 236u32;
pub const JPOS_SURU_SUREBA: u32 = 237u32;
pub const JPOS_TANKANJI: u32 = 900u32;
pub const JPOS_TANKANJI_KAO: u32 = 901u32;
pub const JPOS_TANSHUKU: u32 = 913u32;
pub const JPOS_TOKUSHU_KAHEN: u32 = 214u32;
pub const JPOS_TOKUSHU_NAHEN: u32 = 218u32;
pub const JPOS_TOKUSHU_SAHEN: u32 = 216u32;
pub const JPOS_TOKUSHU_SAHENSURU: u32 = 215u32;
pub const JPOS_TOKUSHU_ZAHEN: u32 = 217u32;
pub const JPOS_TOUTEN: u32 = 908u32;
pub const JPOS_UNDEFINED: u32 = 0u32;
pub const JPOS_YOKUSEI: u32 = 912u32;
pub const MAX_APPLETTITLE: u32 = 64u32;
pub const MAX_FONTFACE: u32 = 32u32;
pub const MODEBIASMODE_DEFAULT: u32 = 0u32;
pub const MODEBIASMODE_DIGIT: u32 = 4u32;
pub const MODEBIASMODE_FILENAME: u32 = 1u32;
pub const MODEBIASMODE_READING: u32 = 2u32;
pub const MODEBIAS_GETVALUE: u32 = 2u32;
pub const MODEBIAS_GETVERSION: u32 = 0u32;
pub const MODEBIAS_SETVALUE: u32 = 1u32;
pub const MOD_IGNORE_ALL_MODIFIER: u32 = 1024u32;
pub const MOD_LEFT: u32 = 32768u32;
pub const MOD_ON_KEYUP: u32 = 2048u32;
pub const MOD_RIGHT: u32 = 16384u32;
#[cfg(feature = "Win32_Foundation")]
pub struct MORRSLT(i32);
pub const NI_CONTEXTUPDATED: u32 = 3u32;
pub const NI_FINALIZECONVERSIONRESULT: u32 = 20u32;
pub struct NOTIFY_IME_ACTION(i32);
pub struct NOTIFY_IME_INDEX(i32);
pub struct PFNLOG(i32);
pub struct POSTBL(i32);
pub const POS_UNDEFINED: u32 = 0u32;
pub struct RECONVERTSTRING(i32);
pub const RECONVOPT_NONE: u32 = 0u32;
pub const RECONVOPT_USECANCELNOTIFY: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct REGISTERWORDA(i32);
pub struct REGISTERWORDENUMPROCA(i32);
pub struct REGISTERWORDENUMPROCW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct REGISTERWORDW(i32);
pub const SCS_CAP_COMPSTR: u32 = 1u32;
pub const SCS_CAP_MAKEREAD: u32 = 2u32;
pub const SCS_CAP_SETRECONVERTSTRING: u32 = 4u32;
pub const SELECT_CAP_CONVERSION: u32 = 1u32;
pub const SELECT_CAP_SENTENCE: u32 = 2u32;
pub struct SET_COMPOSITION_STRING_TYPE(i32);
pub const SHOWIMEPAD_CATEGORY: u32 = 1u32;
pub const SHOWIMEPAD_DEFAULT: u32 = 0u32;
pub const SHOWIMEPAD_GUID: u32 = 2u32;
pub struct SOFTKBDDATA(i32);
pub const SOFTKEYBOARD_TYPE_C1: u32 = 2u32;
pub const SOFTKEYBOARD_TYPE_T1: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct STYLEBUFA(i32);
pub struct STYLEBUFW(i32);
pub const STYLE_DESCRIPTION_SIZE: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
pub struct TRANSMSG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct TRANSMSGLIST(i32);
pub const UI_CAP_2700: u32 = 1u32;
pub const UI_CAP_ROT90: u32 = 2u32;
pub const UI_CAP_ROTANY: u32 = 4u32;
pub const UI_CAP_SOFTKBD: u32 = 65536u32;
pub const VERSION_DOCUMENTFEED: u32 = 1u32;
pub const VERSION_ID_CHINESE_SIMPLIFIED: u32 = 134217728u32;
pub const VERSION_ID_CHINESE_TRADITIONAL: u32 = 67108864u32;
pub const VERSION_ID_JAPANESE: u32 = 16777216u32;
pub const VERSION_ID_KOREAN: u32 = 33554432u32;
pub const VERSION_MODEBIAS: u32 = 1u32;
pub const VERSION_MOUSE_OPERATION: u32 = 1u32;
pub const VERSION_QUERYPOSITION: u32 = 1u32;
pub const VERSION_RECONVERSION: u32 = 1u32;
pub struct WDD(i32);
pub struct fpCreateIFECommonInstanceType(i32);
pub struct fpCreateIFEDictionaryInstanceType(i32);
pub struct fpCreateIFELanguageInstanceType(i32);
pub struct tabIMEFAREASTINFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct tabIMESTRINGINFO(i32);
