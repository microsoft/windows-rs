#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmAssociateContext(param0: super::super::super::Foundation::HWND, param1: super::super::super::Globalization::HIMC) -> super::super::super::Globalization::HIMC;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmAssociateContextEx(param0: super::super::super::Foundation::HWND, param1: super::super::super::Globalization::HIMC, param2: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmConfigureIMEA(param0: super::super::TextServices::HKL, param1: super::super::super::Foundation::HWND, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmConfigureIMEW(param0: super::super::TextServices::HKL, param1: super::super::super::Foundation::HWND, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmCreateContext() -> super::super::super::Globalization::HIMC;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmCreateIMCC(param0: u32) -> super::super::super::Globalization::HIMCC;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmCreateSoftKeyboard(param0: u32, param1: super::super::super::Foundation::HWND, param2: i32, param3: i32) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmDestroyContext(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmDestroyIMCC(param0: super::super::super::Globalization::HIMCC) -> super::super::super::Globalization::HIMCC;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmDestroySoftKeyboard(param0: super::super::super::Foundation::HWND) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmDisableIME(param0: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmDisableLegacyIME() -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmDisableTextFrameService(idthread: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmEnumInputContext(idthread: u32, lpfn: ::windows::runtime::RawPtr, lparam: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmEnumRegisterWordA(param0: super::super::TextServices::HKL, param1: ::windows::runtime::RawPtr, lpszreading: super::super::super::Foundation::PSTR, param3: u32, lpszregister: super::super::super::Foundation::PSTR, param5: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmEnumRegisterWordW(param0: super::super::TextServices::HKL, param1: ::windows::runtime::RawPtr, lpszreading: super::super::super::Foundation::PWSTR, param3: u32, lpszregister: super::super::super::Foundation::PWSTR, param5: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub fn ImmEscapeA(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub fn ImmEscapeW(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGenerateMessage(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCandidateListA(param0: super::super::super::Globalization::HIMC, deindex: u32, lpcandlist: *mut CANDIDATELIST, dwbuflen: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCandidateListCountA(param0: super::super::super::Globalization::HIMC, lpdwlistcount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCandidateListCountW(param0: super::super::super::Globalization::HIMC, lpdwlistcount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCandidateListW(param0: super::super::super::Globalization::HIMC, deindex: u32, lpcandlist: *mut CANDIDATELIST, dwbuflen: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetCandidateWindow(param0: super::super::super::Globalization::HIMC, param1: u32, lpcandidate: *mut CANDIDATEFORM) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmGetCompositionFontA(param0: super::super::super::Globalization::HIMC, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmGetCompositionFontW(param0: super::super::super::Globalization::HIMC, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCompositionStringA(param0: super::super::super::Globalization::HIMC, param1: u32, lpbuf: *mut ::core::ffi::c_void, dwbuflen: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCompositionStringW(param0: super::super::super::Globalization::HIMC, param1: u32, lpbuf: *mut ::core::ffi::c_void, dwbuflen: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetCompositionWindow(param0: super::super::super::Globalization::HIMC, lpcompform: *mut COMPOSITIONFORM) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetContext(param0: super::super::super::Foundation::HWND) -> super::super::super::Globalization::HIMC;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetConversionListA(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, lpsrc: super::super::super::Foundation::PSTR, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetConversionListW(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, lpsrc: super::super::super::Foundation::PWSTR, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetConversionStatus(param0: super::super::super::Globalization::HIMC, lpfdwconversion: *mut u32, lpfdwsentence: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmGetDefaultIMEWnd(param0: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetDescriptionA(param0: super::super::TextServices::HKL, lpszdescription: super::super::super::Foundation::PSTR, ubuflen: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetDescriptionW(param0: super::super::TextServices::HKL, lpszdescription: super::super::super::Foundation::PWSTR, ubuflen: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetGuideLineA(param0: super::super::super::Globalization::HIMC, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: super::super::super::Foundation::PSTR, dwbuflen: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetGuideLineW(param0: super::super::super::Globalization::HIMC, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: super::super::super::Foundation::PWSTR, dwbuflen: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmGetHotKey(param0: u32, lpumodifiers: *mut u32, lpuvkey: *mut u32, phkl: *mut isize) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetIMCCLockCount(param0: super::super::super::Globalization::HIMCC) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetIMCCSize(param0: super::super::super::Globalization::HIMCC) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetIMCLockCount(param0: super::super::super::Globalization::HIMC) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetIMEFileNameA(param0: super::super::TextServices::HKL, lpszfilename: super::super::super::Foundation::PSTR, ubuflen: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetIMEFileNameW(param0: super::super::TextServices::HKL, lpszfilename: super::super::super::Foundation::PWSTR, ubuflen: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmGetImeMenuItemsA(param0: super::super::super::Globalization::HIMC, param1: u32, param2: u32, lpimeparentmenu: *mut IMEMENUITEMINFOA, lpimemenu: *mut IMEMENUITEMINFOA, dwsize: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmGetImeMenuItemsW(param0: super::super::super::Globalization::HIMC, param1: u32, param2: u32, lpimeparentmenu: *mut IMEMENUITEMINFOW, lpimemenu: *mut IMEMENUITEMINFOW, dwsize: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetOpenStatus(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ImmGetProperty(param0: super::super::TextServices::HKL, param1: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetRegisterWordStyleA(param0: super::super::TextServices::HKL, nitem: u32, lpstylebuf: *mut STYLEBUFA) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ImmGetRegisterWordStyleW(param0: super::super::TextServices::HKL, nitem: u32, lpstylebuf: *mut STYLEBUFW) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetStatusWindowPos(param0: super::super::super::Globalization::HIMC, lpptpos: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmGetVirtualKey(param0: super::super::super::Foundation::HWND) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmInstallIMEA(lpszimefilename: super::super::super::Foundation::PSTR, lpszlayouttext: super::super::super::Foundation::PSTR) -> super::super::TextServices::HKL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmInstallIMEW(lpszimefilename: super::super::super::Foundation::PWSTR, lpszlayouttext: super::super::super::Foundation::PWSTR) -> super::super::TextServices::HKL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmIsIME(param0: super::super::TextServices::HKL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmIsUIMessageA(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmIsUIMessageW(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmLockIMC(param0: super::super::super::Globalization::HIMC) -> *mut INPUTCONTEXT;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmLockIMCC(param0: super::super::super::Globalization::HIMCC) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmNotifyIME(param0: super::super::super::Globalization::HIMC, dwaction: NOTIFY_IME_ACTION, dwindex: NOTIFY_IME_INDEX, dwvalue: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmReSizeIMCC(param0: super::super::super::Globalization::HIMCC, param1: u32) -> super::super::super::Globalization::HIMCC;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmRegisterWordA(param0: super::super::TextServices::HKL, lpszreading: super::super::super::Foundation::PSTR, param2: u32, lpszregister: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmRegisterWordW(param0: super::super::TextServices::HKL, lpszreading: super::super::super::Foundation::PWSTR, param2: u32, lpszregister: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmReleaseContext(param0: super::super::super::Foundation::HWND, param1: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmRequestMessageA(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::WPARAM, param2: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmRequestMessageW(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::WPARAM, param2: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetCandidateWindow(param0: super::super::super::Globalization::HIMC, lpcandidate: *const CANDIDATEFORM) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmSetCompositionFontA(param0: super::super::super::Globalization::HIMC, lplf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmSetCompositionFontW(param0: super::super::super::Globalization::HIMC, lplf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetCompositionStringA(param0: super::super::super::Globalization::HIMC, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: *const ::core::ffi::c_void, dwcomplen: u32, lpread: *const ::core::ffi::c_void, dwreadlen: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetCompositionStringW(param0: super::super::super::Globalization::HIMC, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: *const ::core::ffi::c_void, dwcomplen: u32, lpread: *const ::core::ffi::c_void, dwreadlen: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetCompositionWindow(param0: super::super::super::Globalization::HIMC, lpcompform: *const COMPOSITIONFORM) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetConversionStatus(param0: super::super::super::Globalization::HIMC, param1: u32, param2: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmSetHotKey(param0: u32, param1: u32, param2: u32, param3: super::super::TextServices::HKL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetOpenStatus(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetStatusWindowPos(param0: super::super::super::Globalization::HIMC, lpptpos: *const super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmShowSoftKeyboard(param0: super::super::super::Foundation::HWND, param1: i32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmSimulateHotKey(param0: super::super::super::Foundation::HWND, param1: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmUnlockIMC(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmUnlockIMCC(param0: super::super::super::Globalization::HIMCC) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmUnregisterWordA(param0: super::super::TextServices::HKL, lpszreading: super::super::super::Foundation::PSTR, param2: u32, lpszunregister: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmUnregisterWordW(param0: super::super::TextServices::HKL, lpszreading: super::super::super::Foundation::PWSTR, param2: u32, lpszunregister: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
}
