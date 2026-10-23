#[cfg(feature = "windef")]
windows_link::link!("imm32.dll" "system" fn ImmAssociateContext(param0 : super::HWND, param1 : HIMC) -> HIMC);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmAssociateContextEx(param0 : super::HWND, param1 : HIMC, param2 : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmConfigureIMEA(param0 : super::HKL, param1 : super::HWND, param2 : super::DWORD, param3 : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmConfigureIMEW(param0 : super::HKL, param1 : super::HWND, param2 : super::DWORD, param3 : super::LPVOID) -> windows_sys::core::BOOL);
windows_link::link!("imm32.dll" "system" fn ImmCreateContext() -> HIMC);
windows_link::link!("imm32.dll" "system" fn ImmDestroyContext(param0 : HIMC) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("imm32.dll" "system" fn ImmDisableIME(param0 : super::DWORD) -> windows_sys::core::BOOL);
windows_link::link!("imm32.dll" "system" fn ImmDisableLegacyIME() -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("imm32.dll" "system" fn ImmDisableTextFrameService(idthread : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmEnumInputContext(idthread : super::DWORD, lpfn : IMCENUMPROC, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmEnumRegisterWordA(param0 : super::HKL, param1 : REGISTERWORDENUMPROCA, lpszreading : windows_sys::core::PCSTR, param3 : super::DWORD, lpszregister : windows_sys::core::PCSTR, param5 : super::LPVOID) -> super::UINT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmEnumRegisterWordW(param0 : super::HKL, param1 : REGISTERWORDENUMPROCW, lpszreading : windows_sys::core::PCWSTR, param3 : super::DWORD, lpszregister : windows_sys::core::PCWSTR, param5 : super::LPVOID) -> super::UINT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmEscapeA(param0 : super::HKL, param1 : HIMC, param2 : super::UINT, param3 : super::LPVOID) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmEscapeW(param0 : super::HKL, param1 : HIMC, param2 : super::UINT, param3 : super::LPVOID) -> super::LRESULT);
#[cfg(feature = "shared")]
windows_link::link!("imm32.dll" "system" fn ImmGetCandidateListA(param0 : HIMC, deindex : super::DWORD, lpcandlist : LPCANDIDATELIST, dwbuflen : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmGetCandidateListCountA(param0 : HIMC, lpdwlistcount : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmGetCandidateListCountW(param0 : HIMC, lpdwlistcount : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("imm32.dll" "system" fn ImmGetCandidateListW(param0 : HIMC, deindex : super::DWORD, lpcandlist : LPCANDIDATELIST, dwbuflen : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmGetCandidateWindow(param0 : HIMC, param1 : super::DWORD, lpcandidate : LPCANDIDATEFORM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("imm32.dll" "system" fn ImmGetCompositionFontA(param0 : HIMC, lplf : super::LPLOGFONTA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wingdi", feature = "winnt"))]
windows_link::link!("imm32.dll" "system" fn ImmGetCompositionFontW(param0 : HIMC, lplf : super::LPLOGFONTW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmGetCompositionStringA(param0 : HIMC, param1 : super::DWORD, lpbuf : super::LPVOID, dwbuflen : super::DWORD) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmGetCompositionStringW(param0 : HIMC, param1 : super::DWORD, lpbuf : super::LPVOID, dwbuflen : super::DWORD) -> super::LONG);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmGetCompositionWindow(param0 : HIMC, lpcompform : LPCOMPOSITIONFORM) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("imm32.dll" "system" fn ImmGetContext(param0 : super::HWND) -> HIMC);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmGetConversionListA(param0 : super::HKL, param1 : HIMC, lpsrc : windows_sys::core::PCSTR, lpdst : LPCANDIDATELIST, dwbuflen : super::DWORD, uflag : super::UINT) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmGetConversionListW(param0 : super::HKL, param1 : HIMC, lpsrc : windows_sys::core::PCWSTR, lpdst : LPCANDIDATELIST, dwbuflen : super::DWORD, uflag : super::UINT) -> super::DWORD);
#[cfg(feature = "minwindef")]
windows_link::link!("imm32.dll" "system" fn ImmGetConversionStatus(param0 : HIMC, lpfdwconversion : super::LPDWORD, lpfdwsentence : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("imm32.dll" "system" fn ImmGetDefaultIMEWnd(param0 : super::HWND) -> super::HWND);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmGetDescriptionA(param0 : super::HKL, lpszdescription : windows_sys::core::PSTR, ubuflen : super::UINT) -> super::UINT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmGetDescriptionW(param0 : super::HKL, lpszdescription : windows_sys::core::PWSTR, ubuflen : super::UINT) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("imm32.dll" "system" fn ImmGetGuideLineA(param0 : HIMC, dwindex : super::DWORD, lpbuf : windows_sys::core::PSTR, dwbuflen : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("imm32.dll" "system" fn ImmGetGuideLineW(param0 : HIMC, dwindex : super::DWORD, lpbuf : windows_sys::core::PWSTR, dwbuflen : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmGetIMEFileNameA(param0 : super::HKL, lpszfilename : windows_sys::core::PSTR, ubuflen : super::UINT) -> super::UINT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmGetIMEFileNameW(param0 : super::HKL, lpszfilename : windows_sys::core::PWSTR, ubuflen : super::UINT) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmGetImeMenuItemsA(param0 : HIMC, param1 : super::DWORD, param2 : super::DWORD, lpimeparentmenu : LPIMEMENUITEMINFOA, lpimemenu : LPIMEMENUITEMINFOA, dwsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("imm32.dll" "system" fn ImmGetImeMenuItemsW(param0 : HIMC, param1 : super::DWORD, param2 : super::DWORD, lpimeparentmenu : LPIMEMENUITEMINFOW, lpimemenu : LPIMEMENUITEMINFOW, dwsize : super::DWORD) -> super::DWORD);
windows_link::link!("imm32.dll" "system" fn ImmGetOpenStatus(param0 : HIMC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmGetProperty(param0 : super::HKL, param1 : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmGetRegisterWordStyleA(param0 : super::HKL, nitem : super::UINT, lpstylebuf : LPSTYLEBUFA) -> super::UINT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("imm32.dll" "system" fn ImmGetRegisterWordStyleW(param0 : super::HKL, nitem : super::UINT, lpstylebuf : LPSTYLEBUFW) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmGetStatusWindowPos(param0 : HIMC, lpptpos : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmGetVirtualKey(param0 : super::HWND) -> super::UINT);
#[cfg(feature = "minwindef")]
windows_link::link!("imm32.dll" "system" fn ImmInstallIMEA(lpszimefilename : windows_sys::core::PCSTR, lpszlayouttext : windows_sys::core::PCSTR) -> super::HKL);
#[cfg(feature = "minwindef")]
windows_link::link!("imm32.dll" "system" fn ImmInstallIMEW(lpszimefilename : windows_sys::core::PCWSTR, lpszlayouttext : windows_sys::core::PCWSTR) -> super::HKL);
#[cfg(feature = "minwindef")]
windows_link::link!("imm32.dll" "system" fn ImmIsIME(param0 : super::HKL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmIsUIMessageA(param0 : super::HWND, param1 : super::UINT, param2 : super::WPARAM, param3 : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmIsUIMessageW(param0 : super::HWND, param1 : super::UINT, param2 : super::WPARAM, param3 : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("imm32.dll" "system" fn ImmNotifyIME(param0 : HIMC, dwaction : super::DWORD, dwindex : super::DWORD, dwvalue : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmRegisterWordA(param0 : super::HKL, lpszreading : windows_sys::core::PCSTR, param2 : super::DWORD, lpszregister : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmRegisterWordW(param0 : super::HKL, lpszreading : windows_sys::core::PCWSTR, param2 : super::DWORD, lpszregister : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("imm32.dll" "system" fn ImmReleaseContext(param0 : super::HWND, param1 : HIMC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmSetCandidateWindow(param0 : HIMC, lpcandidate : LPCANDIDATEFORM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("imm32.dll" "system" fn ImmSetCompositionFontA(param0 : HIMC, lplf : super::LPLOGFONTA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wingdi", feature = "winnt"))]
windows_link::link!("imm32.dll" "system" fn ImmSetCompositionFontW(param0 : HIMC, lplf : super::LPLOGFONTW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmSetCompositionStringA(param0 : HIMC, dwindex : super::DWORD, lpcomp : super::LPVOID, dwcomplen : super::DWORD, lpread : super::LPVOID, dwreadlen : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmSetCompositionStringW(param0 : HIMC, dwindex : super::DWORD, lpcomp : super::LPVOID, dwcomplen : super::DWORD, lpread : super::LPVOID, dwreadlen : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmSetCompositionWindow(param0 : HIMC, lpcompform : LPCOMPOSITIONFORM) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("imm32.dll" "system" fn ImmSetConversionStatus(param0 : HIMC, param1 : super::DWORD, param2 : super::DWORD) -> windows_sys::core::BOOL);
windows_link::link!("imm32.dll" "system" fn ImmSetOpenStatus(param0 : HIMC, param1 : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmSetStatusWindowPos(param0 : HIMC, lpptpos : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("imm32.dll" "system" fn ImmSimulateHotKey(param0 : super::HWND, param1 : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmUnregisterWordA(param0 : super::HKL, lpszreading : windows_sys::core::PCSTR, param2 : super::DWORD, lpszunregister : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("imm32.dll" "system" fn ImmUnregisterWordW(param0 : super::HKL, lpszreading : windows_sys::core::PCWSTR, param2 : super::DWORD, lpszunregister : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
pub const ATTR_CONVERTED: i32 = 2;
pub const ATTR_FIXEDCONVERTED: i32 = 5;
pub const ATTR_INPUT: i32 = 0;
pub const ATTR_INPUT_ERROR: i32 = 4;
pub const ATTR_TARGET_CONVERTED: i32 = 1;
pub const ATTR_TARGET_NOTCONVERTED: i32 = 3;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct CANDIDATEFORM {
    pub dwIndex: super::DWORD,
    pub dwStyle: super::DWORD,
    pub ptCurrentPos: super::POINT,
    pub rcArea: super::RECT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct CANDIDATELIST {
    pub dwSize: super::DWORD,
    pub dwStyle: super::DWORD,
    pub dwCount: super::DWORD,
    pub dwSelection: super::DWORD,
    pub dwPageStart: super::DWORD,
    pub dwPageSize: super::DWORD,
    pub dwOffset: [super::DWORD; 1],
}
#[cfg(feature = "shared")]
impl Default for CANDIDATELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CFS_CANDIDATEPOS: i32 = 64;
pub const CFS_DEFAULT: i32 = 0;
pub const CFS_EXCLUDE: i32 = 128;
pub const CFS_FORCE_POSITION: i32 = 32;
pub const CFS_POINT: i32 = 2;
pub const CFS_RECT: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct COMPOSITIONFORM {
    pub dwStyle: super::DWORD,
    pub ptCurrentPos: super::POINT,
    pub rcArea: super::RECT,
}
pub const CPS_CANCEL: i32 = 4;
pub const CPS_COMPLETE: i32 = 1;
pub const CPS_CONVERT: i32 = 2;
pub const CPS_REVERT: i32 = 3;
pub const CS_INSERTCHAR: i32 = 8192;
pub const CS_NOMOVECARET: i32 = 16384;
pub const GCL_CONVERSION: i32 = 1;
pub const GCL_REVERSECONVERSION: i32 = 2;
pub const GCL_REVERSE_LENGTH: i32 = 3;
pub const GCS_COMPATTR: i32 = 16;
pub const GCS_COMPCLAUSE: i32 = 32;
pub const GCS_COMPREADATTR: i32 = 2;
pub const GCS_COMPREADCLAUSE: i32 = 4;
pub const GCS_COMPREADSTR: i32 = 1;
pub const GCS_COMPSTR: i32 = 8;
pub const GCS_CURSORPOS: i32 = 128;
pub const GCS_DELTASTART: i32 = 256;
pub const GCS_RESULTCLAUSE: i32 = 4096;
pub const GCS_RESULTREADCLAUSE: i32 = 1024;
pub const GCS_RESULTREADSTR: i32 = 512;
pub const GCS_RESULTSTR: i32 = 2048;
pub const GGL_INDEX: i32 = 2;
pub const GGL_LEVEL: i32 = 1;
pub const GGL_PRIVATE: i32 = 4;
pub const GGL_STRING: i32 = 3;
pub const GL_ID_CANNOTSAVE: i32 = 17;
pub const GL_ID_CHOOSECANDIDATE: i32 = 40;
pub const GL_ID_INPUTCODE: i32 = 38;
pub const GL_ID_INPUTRADICAL: i32 = 37;
pub const GL_ID_INPUTREADING: i32 = 36;
pub const GL_ID_INPUTSYMBOL: i32 = 39;
pub const GL_ID_NOCONVERT: i32 = 32;
pub const GL_ID_NODICTIONARY: i32 = 16;
pub const GL_ID_NOMODULE: i32 = 1;
pub const GL_ID_PRIVATE_FIRST: i32 = 32768;
pub const GL_ID_PRIVATE_LAST: i32 = 65535;
pub const GL_ID_READINGCONFLICT: i32 = 35;
pub const GL_ID_REVERSECONVERSION: i32 = 41;
pub const GL_ID_TOOMANYSTROKE: i32 = 34;
pub const GL_ID_TYPINGERROR: i32 = 33;
pub const GL_ID_UNKNOWN: i32 = 0;
pub const GL_LEVEL_ERROR: i32 = 2;
pub const GL_LEVEL_FATAL: i32 = 1;
pub const GL_LEVEL_INFORMATION: i32 = 4;
pub const GL_LEVEL_NOGUIDELINE: i32 = 0;
pub const GL_LEVEL_WARNING: i32 = 3;
pub type HIMC = *mut HIMC__;
pub type HIMCC = *mut HIMCC__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HIMCC__ {
    pub unused: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HIMC__ {
    pub unused: i32,
}
pub const IACE_CHILDREN: i32 = 1;
pub const IACE_DEFAULT: i32 = 16;
pub const IACE_IGNORENOCONTEXT: i32 = 32;
pub const IGIMIF_RIGHTMENU: i32 = 1;
pub const IGIMII_CMODE: i32 = 1;
pub const IGIMII_CONFIGURE: i32 = 4;
pub const IGIMII_HELP: i32 = 16;
pub const IGIMII_INPUTTOOLS: i32 = 64;
pub const IGIMII_OTHER: i32 = 32;
pub const IGIMII_SMODE: i32 = 2;
pub const IGIMII_TOOLS: i32 = 8;
pub const IGP_CONVERSION: i32 = 8;
#[cfg(feature = "shared")]
pub const IGP_GETIMEVERSION: super::DWORD = 4294967292;
pub const IGP_PROPERTY: i32 = 4;
pub const IGP_SELECT: i32 = 24;
pub const IGP_SENTENCE: i32 = 12;
pub const IGP_SETCOMPSTR: i32 = 20;
pub const IGP_UI: i32 = 16;
#[cfg(feature = "minwindef")]
pub type IMCENUMPROC = Option<unsafe extern "system" fn(param0: HIMC, param1: super::LPARAM) -> windows_sys::core::BOOL>;
pub const IMC_CLOSESTATUSWINDOW: i32 = 33;
pub const IMC_GETCANDIDATEPOS: i32 = 7;
pub const IMC_GETCOMPOSITIONFONT: i32 = 9;
pub const IMC_GETCOMPOSITIONWINDOW: i32 = 11;
pub const IMC_GETSTATUSWINDOWPOS: i32 = 15;
pub const IMC_OPENSTATUSWINDOW: i32 = 34;
pub const IMC_SETCANDIDATEPOS: i32 = 8;
pub const IMC_SETCOMPOSITIONFONT: i32 = 10;
pub const IMC_SETCOMPOSITIONWINDOW: i32 = 12;
pub const IMC_SETSTATUSWINDOWPOS: i32 = 16;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct IMECHARPOSITION {
    pub dwSize: super::DWORD,
    pub dwCharPos: super::DWORD,
    pub pt: super::POINT,
    pub cLineHeight: super::UINT,
    pub rcDocument: super::RECT,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub type IMEMENUITEMINFO = IMEMENUITEMINFOA;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct IMEMENUITEMINFOA {
    pub cbSize: super::UINT,
    pub fType: super::UINT,
    pub fState: super::UINT,
    pub wID: super::UINT,
    pub hbmpChecked: super::HBITMAP,
    pub hbmpUnchecked: super::HBITMAP,
    pub dwItemData: super::DWORD,
    pub szString: [super::CHAR; 80],
    pub hbmpItem: super::HBITMAP,
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for IMEMENUITEMINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IMEMENUITEMINFOW {
    pub cbSize: super::UINT,
    pub fType: super::UINT,
    pub fState: super::UINT,
    pub wID: super::UINT,
    pub hbmpChecked: super::HBITMAP,
    pub hbmpUnchecked: super::HBITMAP,
    pub dwItemData: super::DWORD,
    pub szString: [super::WCHAR; 80],
    pub hbmpItem: super::HBITMAP,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for IMEMENUITEMINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMEMENUITEM_STRING_SIZE: i32 = 80;
pub const IMEVER_0310: i32 = 196618;
pub const IMEVER_0400: i32 = 262144;
pub const IME_CAND_CODE: i32 = 2;
pub const IME_CAND_MEANING: i32 = 3;
pub const IME_CAND_RADICAL: i32 = 4;
pub const IME_CAND_READ: i32 = 1;
pub const IME_CAND_STROKE: i32 = 5;
pub const IME_CAND_UNKNOWN: i32 = 0;
pub const IME_CHOTKEY_IME_NONIME_TOGGLE: i32 = 16;
pub const IME_CHOTKEY_SHAPE_TOGGLE: i32 = 17;
pub const IME_CHOTKEY_SYMBOL_TOGGLE: i32 = 18;
pub const IME_CMODE_EUDC: i32 = 512;
pub const IME_CMODE_FIXED: i32 = 2048;
pub const IME_CMODE_HANGEUL: i32 = 1;
pub const IME_CMODE_NOCONVERSION: i32 = 256;
pub const IME_CMODE_RESERVED: u32 = 4026531840;
pub const IME_CMODE_SOFTKBD: i32 = 128;
pub const IME_CMODE_SYMBOL: i32 = 1024;
pub const IME_CONFIG_GENERAL: i32 = 1;
pub const IME_CONFIG_REGISTERWORD: i32 = 2;
pub const IME_CONFIG_SELECTDICTIONARY: i32 = 3;
pub const IME_ESC_AUTOMATA: i32 = 4105;
pub const IME_ESC_GETHELPFILENAME: i32 = 4107;
pub const IME_ESC_GET_EUDC_DICTIONARY: i32 = 4099;
pub const IME_ESC_HANJA_MODE: i32 = 4104;
pub const IME_ESC_IME_NAME: i32 = 4102;
pub const IME_ESC_MAX_KEY: i32 = 4101;
pub const IME_ESC_PRIVATE_FIRST: i32 = 2048;
pub const IME_ESC_PRIVATE_HOTKEY: i32 = 4106;
pub const IME_ESC_PRIVATE_LAST: i32 = 4095;
pub const IME_ESC_QUERY_SUPPORT: i32 = 3;
pub const IME_ESC_RESERVED_FIRST: i32 = 4;
pub const IME_ESC_RESERVED_LAST: i32 = 2047;
pub const IME_ESC_SEQUENCE_TO_INTERNAL: i32 = 4097;
pub const IME_ESC_SET_EUDC_DICTIONARY: i32 = 4100;
pub const IME_ESC_SYNC_HOTKEY: i32 = 4103;
pub const IME_HOTKEY_DSWITCH_FIRST: i32 = 256;
pub const IME_HOTKEY_DSWITCH_LAST: i32 = 287;
pub const IME_HOTKEY_PRIVATE_FIRST: i32 = 512;
pub const IME_HOTKEY_PRIVATE_LAST: i32 = 543;
pub const IME_ITHOTKEY_PREVIOUS_COMPOSITION: i32 = 513;
pub const IME_ITHOTKEY_RECONVERTSTRING: i32 = 515;
pub const IME_ITHOTKEY_RESEND_RESULTSTR: i32 = 512;
pub const IME_ITHOTKEY_UISTYLE_TOGGLE: i32 = 514;
pub const IME_JHOTKEY_CLOSE_OPEN: i32 = 48;
pub const IME_KHOTKEY_ENGLISH: i32 = 82;
pub const IME_KHOTKEY_HANJACONVERT: i32 = 81;
pub const IME_KHOTKEY_SHAPE_TOGGLE: i32 = 80;
pub const IME_PROP_AT_CARET: i32 = 65536;
pub const IME_PROP_CANDLIST_START_FROM_1: i32 = 262144;
pub const IME_PROP_COMPLETE_ON_UNSELECT: i32 = 1048576;
pub const IME_PROP_SPECIAL_UI: i32 = 131072;
pub const IME_PROP_UNICODE: i32 = 524288;
pub const IME_REGWORD_STYLE_EUDC: i32 = 1;
pub const IME_REGWORD_STYLE_USER_FIRST: u32 = 2147483648;
pub const IME_REGWORD_STYLE_USER_LAST: u32 = 4294967295;
pub const IME_SMODE_AUTOMATIC: i32 = 4;
pub const IME_SMODE_CONVERSATION: i32 = 16;
pub const IME_SMODE_NONE: i32 = 0;
pub const IME_SMODE_PHRASEPREDICT: i32 = 8;
pub const IME_SMODE_PLAURALCLAUSE: i32 = 1;
pub const IME_SMODE_RESERVED: i32 = 61440;
pub const IME_SMODE_SINGLECONVERT: i32 = 2;
pub const IME_THOTKEY_IME_NONIME_TOGGLE: i32 = 112;
pub const IME_THOTKEY_SHAPE_TOGGLE: i32 = 113;
pub const IME_THOTKEY_SYMBOL_TOGGLE: i32 = 114;
pub const IMFS_CHECKED: i32 = 8;
pub const IMFS_DEFAULT: i32 = 4096;
pub const IMFS_DISABLED: i32 = 3;
pub const IMFS_ENABLED: i32 = 0;
pub const IMFS_GRAYED: i32 = 3;
pub const IMFS_HILITE: i32 = 128;
pub const IMFS_UNCHECKED: i32 = 0;
pub const IMFS_UNHILITE: i32 = 0;
pub const IMFT_RADIOCHECK: i32 = 1;
pub const IMFT_SEPARATOR: i32 = 2;
pub const IMFT_SUBMENU: i32 = 4;
pub const IMM_ERROR_GENERAL: i32 = -2;
pub const IMM_ERROR_NODATA: i32 = -1;
pub const IMN_CHANGECANDIDATE: i32 = 3;
pub const IMN_CLOSECANDIDATE: i32 = 4;
pub const IMN_CLOSESTATUSWINDOW: i32 = 1;
pub const IMN_GUIDELINE: i32 = 13;
pub const IMN_OPENCANDIDATE: i32 = 5;
pub const IMN_OPENSTATUSWINDOW: i32 = 2;
pub const IMN_PRIVATE: i32 = 14;
pub const IMN_SETCANDIDATEPOS: i32 = 9;
pub const IMN_SETCOMPOSITIONFONT: i32 = 10;
pub const IMN_SETCOMPOSITIONWINDOW: i32 = 11;
pub const IMN_SETCONVERSIONMODE: i32 = 6;
pub const IMN_SETOPENSTATUS: i32 = 8;
pub const IMN_SETSENTENCEMODE: i32 = 7;
pub const IMN_SETSTATUSWINDOWPOS: i32 = 12;
pub const IMR_CANDIDATEWINDOW: i32 = 2;
pub const IMR_COMPOSITIONFONT: i32 = 3;
pub const IMR_COMPOSITIONWINDOW: i32 = 1;
pub const IMR_CONFIRMRECONVERTSTRING: i32 = 5;
pub const IMR_DOCUMENTFEED: i32 = 7;
pub const IMR_QUERYCHARPOSITION: i32 = 6;
pub const IMR_RECONVERTSTRING: i32 = 4;
pub const ISC_SHOWUIALL: u32 = 3221225487;
pub const ISC_SHOWUIALLCANDIDATEWINDOW: i32 = 15;
pub const ISC_SHOWUICANDIDATEWINDOW: i32 = 1;
pub const ISC_SHOWUICOMPOSITIONWINDOW: u32 = 2147483648;
pub const ISC_SHOWUIGUIDELINE: i32 = 1073741824;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPCANDIDATEFORM = *mut CANDIDATEFORM;
#[cfg(feature = "shared")]
pub type LPCANDIDATELIST = *mut CANDIDATELIST;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPCOMPOSITIONFORM = *mut COMPOSITIONFORM;
#[cfg(feature = "minwindef")]
pub type LPHKL = *mut *mut super::HKL__;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPIMECHARPOSITION = *mut IMECHARPOSITION;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPIMEMENUITEMINFO = *mut IMEMENUITEMINFOA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPIMEMENUITEMINFOA = *mut IMEMENUITEMINFOA;
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPIMEMENUITEMINFOW = *mut IMEMENUITEMINFOW;
#[cfg(feature = "shared")]
pub type LPRECONVERTSTRING = *mut RECONVERTSTRING;
pub type LPREGISTERWORD = *mut REGISTERWORDA;
pub type LPREGISTERWORDA = *mut REGISTERWORDA;
pub type LPREGISTERWORDW = *mut REGISTERWORDW;
#[cfg(feature = "shared")]
pub type LPSTYLEBUF = *mut STYLEBUFA;
#[cfg(feature = "shared")]
pub type LPSTYLEBUFA = *mut STYLEBUFA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSTYLEBUFW = *mut STYLEBUFW;
pub const MOD_ALT: i32 = 1;
pub const MOD_CONTROL: i32 = 2;
pub const MOD_IGNORE_ALL_MODIFIER: i32 = 1024;
pub const MOD_LEFT: i32 = 32768;
pub const MOD_ON_KEYUP: i32 = 2048;
pub const MOD_RIGHT: i32 = 16384;
pub const MOD_SHIFT: i32 = 4;
pub const NI_CHANGECANDIDATELIST: i32 = 19;
pub const NI_CLOSECANDIDATE: i32 = 17;
pub const NI_COMPOSITIONSTR: i32 = 21;
pub const NI_FINALIZECONVERSIONRESULT: i32 = 20;
pub const NI_IMEMENUSELECTED: i32 = 24;
pub const NI_OPENCANDIDATE: i32 = 16;
pub const NI_SELECTCANDIDATESTR: i32 = 18;
pub const NI_SETCANDIDATE_PAGESIZE: i32 = 23;
pub const NI_SETCANDIDATE_PAGESTART: i32 = 22;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPCANDIDATEFORM = *mut CANDIDATEFORM;
#[cfg(feature = "shared")]
pub type NPCANDIDATELIST = *mut CANDIDATELIST;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPCOMPOSITIONFORM = *mut COMPOSITIONFORM;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPIMECHARPOSITION = *mut IMECHARPOSITION;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPIMEMENUITEMINFO = *mut IMEMENUITEMINFOA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPIMEMENUITEMINFOA = *mut IMEMENUITEMINFOA;
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub type NPIMEMENUITEMINFOW = *mut IMEMENUITEMINFOW;
#[cfg(feature = "shared")]
pub type NPRECONVERTSTRING = *mut RECONVERTSTRING;
pub type NPREGISTERWORD = *mut REGISTERWORDA;
pub type NPREGISTERWORDA = *mut REGISTERWORDA;
pub type NPREGISTERWORDW = *mut REGISTERWORDW;
#[cfg(feature = "shared")]
pub type NPSTYLEBUF = *mut STYLEBUFA;
#[cfg(feature = "shared")]
pub type NPSTYLEBUFA = *mut STYLEBUFA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type NPSTYLEBUFW = *mut STYLEBUFW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PCANDIDATEFORM = *mut CANDIDATEFORM;
#[cfg(feature = "shared")]
pub type PCANDIDATELIST = *mut CANDIDATELIST;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PCOMPOSITIONFORM = *mut COMPOSITIONFORM;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PIMECHARPOSITION = *mut IMECHARPOSITION;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PIMEMENUITEMINFO = *mut IMEMENUITEMINFOA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PIMEMENUITEMINFOA = *mut IMEMENUITEMINFOA;
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub type PIMEMENUITEMINFOW = *mut IMEMENUITEMINFOW;
#[cfg(feature = "shared")]
pub type PRECONVERTSTRING = *mut RECONVERTSTRING;
pub type PREGISTERWORD = *mut REGISTERWORDA;
pub type PREGISTERWORDA = *mut REGISTERWORDA;
pub type PREGISTERWORDW = *mut REGISTERWORDW;
#[cfg(feature = "shared")]
pub type PSTYLEBUF = *mut STYLEBUFA;
#[cfg(feature = "shared")]
pub type PSTYLEBUFA = *mut STYLEBUFA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTYLEBUFW = *mut STYLEBUFW;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct RECONVERTSTRING {
    pub dwSize: super::DWORD,
    pub dwVersion: super::DWORD,
    pub dwStrLen: super::DWORD,
    pub dwStrOffset: super::DWORD,
    pub dwCompStrLen: super::DWORD,
    pub dwCompStrOffset: super::DWORD,
    pub dwTargetStrLen: super::DWORD,
    pub dwTargetStrOffset: super::DWORD,
}
pub type REGISTERWORD = REGISTERWORDA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REGISTERWORDA {
    pub lpReading: windows_sys::core::PSTR,
    pub lpWord: windows_sys::core::PSTR,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type REGISTERWORDENUMPROCA = Option<unsafe extern "system" fn(lpszreading: windows_sys::core::PCSTR, param1: super::DWORD, lpszstring: windows_sys::core::PCSTR, param3: super::LPVOID) -> i32>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type REGISTERWORDENUMPROCW = Option<unsafe extern "system" fn(lpszreading: windows_sys::core::PCWSTR, param1: super::DWORD, lpszstring: windows_sys::core::PCWSTR, param3: super::LPVOID) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REGISTERWORDW {
    pub lpReading: windows_sys::core::PWSTR,
    pub lpWord: windows_sys::core::PWSTR,
}
pub const SCS_CAP_COMPSTR: i32 = 1;
pub const SCS_CAP_MAKEREAD: i32 = 2;
pub const SCS_CAP_SETRECONVERTSTRING: i32 = 4;
pub const SCS_CHANGEATTR: i32 = 18;
pub const SCS_CHANGECLAUSE: i32 = 36;
pub const SCS_QUERYRECONVERTSTRING: i32 = 131072;
pub const SCS_SETRECONVERTSTRING: i32 = 65536;
pub const SCS_SETSTR: i32 = 9;
pub const SELECT_CAP_CONVERSION: i32 = 1;
pub const SELECT_CAP_SENTENCE: i32 = 2;
pub const SOFTKEYBOARD_TYPE_C1: i32 = 2;
pub const SOFTKEYBOARD_TYPE_T1: i32 = 1;
#[cfg(feature = "shared")]
pub type STYLEBUF = STYLEBUFA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct STYLEBUFA {
    pub dwStyle: super::DWORD,
    pub szDescription: [super::CHAR; 32],
}
#[cfg(feature = "shared")]
impl Default for STYLEBUFA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STYLEBUFW {
    pub dwStyle: super::DWORD,
    pub szDescription: [super::WCHAR; 32],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STYLEBUFW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STYLE_DESCRIPTION_SIZE: i32 = 32;
pub const UI_CAP_2700: i32 = 1;
pub const UI_CAP_ROT90: i32 = 2;
pub const UI_CAP_ROTANY: i32 = 4;
