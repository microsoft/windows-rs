#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdjustWindowRect(lprect: *mut super::super::Foundation::RECT, dwstyle: WINDOW_STYLE, bmenu: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdjustWindowRectEx(lprect: *mut super::super::Foundation::RECT, dwstyle: WINDOW_STYLE, bmenu: super::super::Foundation::BOOL, dwexstyle: WINDOW_EX_STYLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllowSetForegroundWindow(dwprocessid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AnimateWindow(hwnd: super::super::Foundation::HWND, dwtime: u32, dwflags: ANIMATE_WINDOW_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AnyPopup() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppendMenuA(hmenu: HMENU, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppendMenuW(hmenu: HMENU, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ArrangeIconicWindows(hwnd: super::super::Foundation::HWND) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn BeginDeferWindowPos(nnumwindows: i32) -> isize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BringWindowToTop(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CalculatePopupWindowPosition(anchorpoint: *const super::super::Foundation::POINT, windowsize: *const super::super::Foundation::SIZE, flags: u32, excluderect: *const super::super::Foundation::RECT, popupwindowposition: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallMsgFilterA(lpmsg: *const MSG, ncode: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallMsgFilterW(lpmsg: *const MSG, ncode: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallNextHookEx(hhk: HHOOK, ncode: i32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallWindowProcA(lpprevwndfunc: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallWindowProcW(lpprevwndfunc: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelShutdown() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CascadeWindows(hwndparent: super::super::Foundation::HWND, whow: CASCADE_WINDOWS_HOW, lprect: *const super::super::Foundation::RECT, ckids: u32, lpkids: *const super::super::Foundation::HWND) -> u16;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeMenuA(hmenu: HMENU, cmd: u32, lpsznewitem: super::super::Foundation::PSTR, cmdinsert: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeMenuW(hmenu: HMENU, cmd: u32, lpsznewitem: super::super::Foundation::PWSTR, cmdinsert: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeWindowMessageFilter(message: u32, dwflag: CHANGE_WINDOW_MESSAGE_FILTER_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeWindowMessageFilterEx(hwnd: super::super::Foundation::HWND, message: u32, action: WINDOW_MESSAGE_FILTER_ACTION, pchangefilterstruct: *mut CHANGEFILTERSTRUCT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharLowerA(lpsz: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharLowerBuffA(lpsz: super::super::Foundation::PSTR, cchlength: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharLowerBuffW(lpsz: super::super::Foundation::PWSTR, cchlength: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharLowerW(lpsz: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharNextA(lpsz: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharNextExA(codepage: u16, lpcurrentchar: super::super::Foundation::PSTR, dwflags: u32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharNextW(lpsz: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharPrevA(lpszstart: super::super::Foundation::PSTR, lpszcurrent: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharPrevExA(codepage: u16, lpstart: super::super::Foundation::PSTR, lpcurrentchar: super::super::Foundation::PSTR, dwflags: u32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharPrevW(lpszstart: super::super::Foundation::PWSTR, lpszcurrent: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharToOemA(psrc: super::super::Foundation::PSTR, pdst: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharToOemBuffA(lpszsrc: super::super::Foundation::PSTR, lpszdst: super::super::Foundation::PSTR, cchdstlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharToOemBuffW(lpszsrc: super::super::Foundation::PWSTR, lpszdst: super::super::Foundation::PSTR, cchdstlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharToOemW(psrc: super::super::Foundation::PWSTR, pdst: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharUpperA(lpsz: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharUpperBuffA(lpsz: super::super::Foundation::PSTR, cchlength: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharUpperBuffW(lpsz: super::super::Foundation::PWSTR, cchlength: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharUpperW(lpsz: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CheckMenuItem(hmenu: HMENU, uidcheckitem: u32, ucheck: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckMenuRadioItem(hmenu: HMENU, first: u32, last: u32, check: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChildWindowFromPoint(hwndparent: super::super::Foundation::HWND, point: super::super::Foundation::POINT) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChildWindowFromPointEx(hwnd: super::super::Foundation::HWND, pt: super::super::Foundation::POINT, flags: CWP_FLAGS) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClipCursor(lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CopyAcceleratorTableA(haccelsrc: HACCEL, lpacceldst: *mut ACCEL, caccelentries: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CopyAcceleratorTableW(haccelsrc: HACCEL, lpacceldst: *mut ACCEL, caccelentries: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CopyIcon(hicon: HICON) -> HICON;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyImage(h: super::super::Foundation::HANDLE, r#type: GDI_IMAGE_TYPE, cx: i32, cy: i32, flags: IMAGE_FLAGS) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CreateAcceleratorTableA(paccel: *const ACCEL, caccel: i32) -> HACCEL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CreateAcceleratorTableW(paccel: *const ACCEL, caccel: i32) -> HACCEL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateCaret(hwnd: super::super::Foundation::HWND, hbitmap: super::super::Graphics::Gdi::HBITMAP, nwidth: i32, nheight: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateCursor(hinst: super::super::Foundation::HINSTANCE, xhotspot: i32, yhotspot: i32, nwidth: i32, nheight: i32, pvandplane: *const ::core::ffi::c_void, pvxorplane: *const ::core::ffi::c_void) -> HCURSOR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDialogIndirectParamA(hinstance: super::super::Foundation::HINSTANCE, lptemplate: *const DLGTEMPLATE, hwndparent: super::super::Foundation::HWND, lpdialogfunc: ::windows::runtime::RawPtr, dwinitparam: super::super::Foundation::LPARAM) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDialogIndirectParamW(hinstance: super::super::Foundation::HINSTANCE, lptemplate: *const DLGTEMPLATE, hwndparent: super::super::Foundation::HWND, lpdialogfunc: ::windows::runtime::RawPtr, dwinitparam: super::super::Foundation::LPARAM) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDialogParamA(hinstance: super::super::Foundation::HINSTANCE, lptemplatename: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, lpdialogfunc: ::windows::runtime::RawPtr, dwinitparam: super::super::Foundation::LPARAM) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDialogParamW(hinstance: super::super::Foundation::HINSTANCE, lptemplatename: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, lpdialogfunc: ::windows::runtime::RawPtr, dwinitparam: super::super::Foundation::LPARAM) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateIcon(hinstance: super::super::Foundation::HINSTANCE, nwidth: i32, nheight: i32, cplanes: u8, cbitspixel: u8, lpbandbits: *const u8, lpbxorbits: *const u8) -> HICON;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateIconFromResource(presbits: *const u8, dwressize: u32, ficon: super::super::Foundation::BOOL, dwver: u32) -> HICON;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateIconFromResourceEx(presbits: *const u8, dwressize: u32, ficon: super::super::Foundation::BOOL, dwver: u32, cxdesired: i32, cydesired: i32, flags: IMAGE_FLAGS) -> HICON;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateIconIndirect(piconinfo: *const ICONINFO) -> HICON;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMDIWindowA(lpclassname: super::super::Foundation::PSTR, lpwindowname: super::super::Foundation::PSTR, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMDIWindowW(lpclassname: super::super::Foundation::PWSTR, lpwindowname: super::super::Foundation::PWSTR, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CreateMenu() -> HMENU;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CreatePopupMenu() -> HMENU;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateResourceIndexer(projectroot: super::super::Foundation::PWSTR, extensiondllpath: super::super::Foundation::PWSTR, ppresourceindexer: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateWindowExA(dwexstyle: WINDOW_EX_STYLE, lpclassname: super::super::Foundation::PSTR, lpwindowname: super::super::Foundation::PSTR, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: super::super::Foundation::HWND, hmenu: HMENU, hinstance: super::super::Foundation::HINSTANCE, lpparam: *const ::core::ffi::c_void) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateWindowExW(dwexstyle: WINDOW_EX_STYLE, lpclassname: super::super::Foundation::PWSTR, lpwindowname: super::super::Foundation::PWSTR, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: super::super::Foundation::HWND, hmenu: HMENU, hinstance: super::super::Foundation::HINSTANCE, lpparam: *const ::core::ffi::c_void) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefDlgProcA(hdlg: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefDlgProcW(hdlg: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefFrameProcA(hwnd: super::super::Foundation::HWND, hwndmdiclient: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefFrameProcW(hwnd: super::super::Foundation::HWND, hwndmdiclient: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefMDIChildProcA(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefMDIChildProcW(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefWindowProcA(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefWindowProcW(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeferWindowPos(hwinposinfo: isize, hwnd: super::super::Foundation::HWND, hwndinsertafter: super::super::Foundation::HWND, x: i32, y: i32, cx: i32, cy: i32, uflags: SET_WINDOW_POS_FLAGS) -> isize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteMenu(hmenu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeregisterShellHookWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyAcceleratorTable(haccel: HACCEL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyCaret() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyCursor(hcursor: HCURSOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyIcon(hicon: HICON) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyIndexedResults(resourceuri: super::super::Foundation::PWSTR, qualifiercount: u32, qualifiers: *const IndexedResourceQualifier);
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyMenu(hmenu: HMENU) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn DestroyResourceIndexer(resourceindexer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DialogBoxIndirectParamA(hinstance: super::super::Foundation::HINSTANCE, hdialogtemplate: *const DLGTEMPLATE, hwndparent: super::super::Foundation::HWND, lpdialogfunc: ::windows::runtime::RawPtr, dwinitparam: super::super::Foundation::LPARAM) -> isize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DialogBoxIndirectParamW(hinstance: super::super::Foundation::HINSTANCE, hdialogtemplate: *const DLGTEMPLATE, hwndparent: super::super::Foundation::HWND, lpdialogfunc: ::windows::runtime::RawPtr, dwinitparam: super::super::Foundation::LPARAM) -> isize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DialogBoxParamA(hinstance: super::super::Foundation::HINSTANCE, lptemplatename: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, lpdialogfunc: ::windows::runtime::RawPtr, dwinitparam: super::super::Foundation::LPARAM) -> isize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DialogBoxParamW(hinstance: super::super::Foundation::HINSTANCE, lptemplatename: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, lpdialogfunc: ::windows::runtime::RawPtr, dwinitparam: super::super::Foundation::LPARAM) -> isize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn DisableProcessWindowsGhosting();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DispatchMessageA(lpmsg: *const MSG) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DispatchMessageW(lpmsg: *const MSG) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragObject(hwndparent: super::super::Foundation::HWND, hwndfrom: super::super::Foundation::HWND, fmt: u32, data: usize, hcur: HCURSOR) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawIcon(hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, hicon: HICON) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawIconEx(hdc: super::super::Graphics::Gdi::HDC, xleft: i32, ytop: i32, hicon: HICON, cxwidth: i32, cywidth: i32, istepifanicur: u32, hbrflickerfreedraw: super::super::Graphics::Gdi::HBRUSH, diflags: DI_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawMenuBar(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableMenuItem(hmenu: HMENU, uidenableitem: u32, uenable: MENU_ITEM_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndDeferWindowPos(hwinposinfo: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndDialog(hdlg: super::super::Foundation::HWND, nresult: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndMenu() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumChildWindows(hwndparent: super::super::Foundation::HWND, lpenumfunc: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPropsA(hwnd: super::super::Foundation::HWND, lpenumfunc: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPropsExA(hwnd: super::super::Foundation::HWND, lpenumfunc: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPropsExW(hwnd: super::super::Foundation::HWND, lpenumfunc: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPropsW(hwnd: super::super::Foundation::HWND, lpenumfunc: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumThreadWindows(dwthreadid: u32, lpfn: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumWindows(lpenumfunc: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindWindowA(lpclassname: super::super::Foundation::PSTR, lpwindowname: super::super::Foundation::PSTR) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindWindowExA(hwndparent: super::super::Foundation::HWND, hwndchildafter: super::super::Foundation::HWND, lpszclass: super::super::Foundation::PSTR, lpszwindow: super::super::Foundation::PSTR) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindWindowExW(hwndparent: super::super::Foundation::HWND, hwndchildafter: super::super::Foundation::HWND, lpszclass: super::super::Foundation::PWSTR, lpszwindow: super::super::Foundation::PWSTR) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindWindowW(lpclassname: super::super::Foundation::PWSTR, lpwindowname: super::super::Foundation::PWSTR) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlashWindow(hwnd: super::super::Foundation::HWND, binvert: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlashWindowEx(pfwi: *const FLASHWINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAltTabInfoA(hwnd: super::super::Foundation::HWND, iitem: i32, pati: *mut ALTTABINFO, pszitemtext: super::super::Foundation::PSTR, cchitemtext: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAltTabInfoW(hwnd: super::super::Foundation::HWND, iitem: i32, pati: *mut ALTTABINFO, pszitemtext: super::super::Foundation::PWSTR, cchitemtext: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAncestor(hwnd: super::super::Foundation::HWND, gaflags: GET_ANCESTOR_FLAGS) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetCaretBlinkTime() -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCaretPos(lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetClassInfoA(hinstance: super::super::Foundation::HINSTANCE, lpclassname: super::super::Foundation::PSTR, lpwndclass: *mut ::core::mem::ManuallyDrop<WNDCLASSA>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetClassInfoExA(hinstance: super::super::Foundation::HINSTANCE, lpszclass: super::super::Foundation::PSTR, lpwcx: *mut ::core::mem::ManuallyDrop<WNDCLASSEXA>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetClassInfoExW(hinstance: super::super::Foundation::HINSTANCE, lpszclass: super::super::Foundation::PWSTR, lpwcx: *mut ::core::mem::ManuallyDrop<WNDCLASSEXW>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetClassInfoW(hinstance: super::super::Foundation::HINSTANCE, lpclassname: super::super::Foundation::PWSTR, lpwndclass: *mut ::core::mem::ManuallyDrop<WNDCLASSW>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassLongA(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassLongPtrA(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX) -> usize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassLongPtrW(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX) -> usize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassLongW(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassNameA(hwnd: super::super::Foundation::HWND, lpclassname: super::super::Foundation::PSTR, nmaxcount: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassNameW(hwnd: super::super::Foundation::HWND, lpclassname: super::super::Foundation::PWSTR, nmaxcount: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassWord(hwnd: super::super::Foundation::HWND, nindex: i32) -> u16;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClientRect(hwnd: super::super::Foundation::HWND, lprect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipCursor(lprect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetCursor() -> HCURSOR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCursorInfo(pci: *mut CURSORINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCursorPos(lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDesktopWindow() -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetDialogBaseUnits() -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDlgCtrlID(hwnd: super::super::Foundation::HWND) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDlgItem(hdlg: super::super::Foundation::HWND, niddlgitem: i32) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDlgItemInt(hdlg: super::super::Foundation::HWND, niddlgitem: i32, lptranslated: *mut super::super::Foundation::BOOL, bsigned: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDlgItemTextA(hdlg: super::super::Foundation::HWND, niddlgitem: i32, lpstring: super::super::Foundation::PSTR, cchmax: i32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDlgItemTextW(hdlg: super::super::Foundation::HWND, niddlgitem: i32, lpstring: super::super::Foundation::PWSTR, cchmax: i32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetForegroundWindow() -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGUIThreadInfo(idthread: u32, pgui: *mut GUITHREADINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetIconInfo(hicon: HICON, piconinfo: *mut ICONINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetIconInfoExA(hicon: HICON, piconinfo: *mut ICONINFOEXA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetIconInfoExW(hicon: HICON, piconinfo: *mut ICONINFOEXW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInputState() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLastActivePopup(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLayeredWindowAttributes(hwnd: super::super::Foundation::HWND, pcrkey: *mut u32, pbalpha: *mut u8, pdwflags: *mut LAYERED_WINDOW_ATTRIBUTES_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMenu(hwnd: super::super::Foundation::HWND) -> HMENU;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMenuBarInfo(hwnd: super::super::Foundation::HWND, idobject: OBJECT_IDENTIFIER, iditem: i32, pmbi: *mut MENUBARINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMenuCheckMarkDimensions() -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMenuDefaultItem(hmenu: HMENU, fbypos: u32, gmdiflags: GET_MENU_DEFAULT_ITEM_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetMenuInfo(param0: HMENU, param1: *mut MENUINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMenuItemCount(hmenu: HMENU) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMenuItemID(hmenu: HMENU, npos: i32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetMenuItemInfoA(hmenu: HMENU, item: u32, fbyposition: super::super::Foundation::BOOL, lpmii: *mut MENUITEMINFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetMenuItemInfoW(hmenu: HMENU, item: u32, fbyposition: super::super::Foundation::BOOL, lpmii: *mut MENUITEMINFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMenuItemRect(hwnd: super::super::Foundation::HWND, hmenu: HMENU, uitem: u32, lprcitem: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMenuState(hmenu: HMENU, uid: u32, uflags: MENU_ITEM_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMenuStringA(hmenu: HMENU, uiditem: u32, lpstring: super::super::Foundation::PSTR, cchmax: i32, flags: MENU_ITEM_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMenuStringW(hmenu: HMENU, uiditem: u32, lpstring: super::super::Foundation::PWSTR, cchmax: i32, flags: MENU_ITEM_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMessageA(lpmsg: *mut MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMessageExtraInfo() -> super::super::Foundation::LPARAM;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMessagePos() -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMessageTime() -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMessageW(lpmsg: *mut MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNextDlgGroupItem(hdlg: super::super::Foundation::HWND, hctl: super::super::Foundation::HWND, bprevious: super::super::Foundation::BOOL) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNextDlgTabItem(hdlg: super::super::Foundation::HWND, hctl: super::super::Foundation::HWND, bprevious: super::super::Foundation::BOOL) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetParent(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPhysicalCursorPos(lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessDefaultLayout(pdwdefaultlayout: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPropA(hwnd: super::super::Foundation::HWND, lpstring: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPropW(hwnd: super::super::Foundation::HWND, lpstring: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetQueueStatus(flags: QUEUE_STATUS_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetScrollBarInfo(hwnd: super::super::Foundation::HWND, idobject: OBJECT_IDENTIFIER, psbi: *mut SCROLLBARINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetScrollInfo(hwnd: super::super::Foundation::HWND, nbar: SCROLLBAR_CONSTANTS, lpsi: *mut SCROLLINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetScrollPos(hwnd: super::super::Foundation::HWND, nbar: SCROLLBAR_CONSTANTS) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetScrollRange(hwnd: super::super::Foundation::HWND, nbar: SCROLLBAR_CONSTANTS, lpminpos: *mut i32, lpmaxpos: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetShellWindow() -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetSubMenu(hmenu: HMENU, npos: i32) -> HMENU;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetSysColor(nindex: SYS_COLOR_INDEX) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemMenu(hwnd: super::super::Foundation::HWND, brevert: super::super::Foundation::BOOL) -> HMENU;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetSystemMetrics(nindex: SYSTEM_METRICS_INDEX) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTitleBarInfo(hwnd: super::super::Foundation::HWND, pti: *mut TITLEBARINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTopWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindow(hwnd: super::super::Foundation::HWND, ucmd: GET_WINDOW_CMD) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowDisplayAffinity(hwnd: super::super::Foundation::HWND, pdwaffinity: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowInfo(hwnd: super::super::Foundation::HWND, pwi: *mut WINDOWINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowLongA(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowLongPtrA(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX) -> isize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowLongPtrW(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX) -> isize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowLongW(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowModuleFileNameA(hwnd: super::super::Foundation::HWND, pszfilename: super::super::Foundation::PSTR, cchfilenamemax: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowModuleFileNameW(hwnd: super::super::Foundation::HWND, pszfilename: super::super::Foundation::PWSTR, cchfilenamemax: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowPlacement(hwnd: super::super::Foundation::HWND, lpwndpl: *mut WINDOWPLACEMENT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowRect(hwnd: super::super::Foundation::HWND, lprect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowTextA(hwnd: super::super::Foundation::HWND, lpstring: super::super::Foundation::PSTR, nmaxcount: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowTextLengthA(hwnd: super::super::Foundation::HWND) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowTextLengthW(hwnd: super::super::Foundation::HWND) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowTextW(hwnd: super::super::Foundation::HWND, lpstring: super::super::Foundation::PWSTR, nmaxcount: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowThreadProcessId(hwnd: super::super::Foundation::HWND, lpdwprocessid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowWord(hwnd: super::super::Foundation::HWND, nindex: i32) -> u16;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HideCaret(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HiliteMenuItem(hwnd: super::super::Foundation::HWND, hmenu: HMENU, uidhiliteitem: u32, uhilite: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InSendMessage() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn InSendMessageEx(lpreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IndexFilePath(resourceindexer: *const ::core::ffi::c_void, filepath: super::super::Foundation::PWSTR, ppresourceuri: *mut super::super::Foundation::PWSTR, pqualifiercount: *mut u32, ppqualifiers: *mut *mut IndexedResourceQualifier) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InheritWindowMonitor(hwnd: super::super::Foundation::HWND, hwndinherit: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InsertMenuA(hmenu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn InsertMenuItemA(hmenu: HMENU, item: u32, fbyposition: super::super::Foundation::BOOL, lpmi: *const MENUITEMINFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn InsertMenuItemW(hmenu: HMENU, item: u32, fbyposition: super::super::Foundation::BOOL, lpmi: *const MENUITEMINFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InsertMenuW(hmenu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternalGetWindowText(hwnd: super::super::Foundation::HWND, pstring: super::super::Foundation::PWSTR, cchmaxcount: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharAlphaA(ch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharAlphaNumericA(ch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharAlphaNumericW(ch: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharAlphaW(ch: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharLowerA(ch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharUpperA(ch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharUpperW(ch: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsChild(hwndparent: super::super::Foundation::HWND, hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDialogMessageA(hdlg: super::super::Foundation::HWND, lpmsg: *const MSG) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDialogMessageW(hdlg: super::super::Foundation::HWND, lpmsg: *const MSG) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsGUIThread(bconvert: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsHungAppWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsIconic(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsMenu(hmenu: HMENU) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessDPIAware() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWindowUnicode(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWindowVisible(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWow64Message() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsZoomed(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KillTimer(hwnd: super::super::Foundation::HWND, uidevent: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadAcceleratorsA(hinstance: super::super::Foundation::HINSTANCE, lptablename: super::super::Foundation::PSTR) -> HACCEL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadAcceleratorsW(hinstance: super::super::Foundation::HINSTANCE, lptablename: super::super::Foundation::PWSTR) -> HACCEL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadCursorA(hinstance: super::super::Foundation::HINSTANCE, lpcursorname: super::super::Foundation::PSTR) -> HCURSOR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadCursorFromFileA(lpfilename: super::super::Foundation::PSTR) -> HCURSOR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadCursorFromFileW(lpfilename: super::super::Foundation::PWSTR) -> HCURSOR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadCursorW(hinstance: super::super::Foundation::HINSTANCE, lpcursorname: super::super::Foundation::PWSTR) -> HCURSOR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadIconA(hinstance: super::super::Foundation::HINSTANCE, lpiconname: super::super::Foundation::PSTR) -> HICON;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadIconW(hinstance: super::super::Foundation::HINSTANCE, lpiconname: super::super::Foundation::PWSTR) -> HICON;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadImageA(hinst: super::super::Foundation::HINSTANCE, name: super::super::Foundation::PSTR, r#type: GDI_IMAGE_TYPE, cx: i32, cy: i32, fuload: IMAGE_FLAGS) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadImageW(hinst: super::super::Foundation::HINSTANCE, name: super::super::Foundation::PWSTR, r#type: GDI_IMAGE_TYPE, cx: i32, cy: i32, fuload: IMAGE_FLAGS) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadMenuA(hinstance: super::super::Foundation::HINSTANCE, lpmenuname: super::super::Foundation::PSTR) -> HMENU;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn LoadMenuIndirectA(lpmenutemplate: *const ::core::ffi::c_void) -> HMENU;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn LoadMenuIndirectW(lpmenutemplate: *const ::core::ffi::c_void) -> HMENU;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadMenuW(hinstance: super::super::Foundation::HINSTANCE, lpmenuname: super::super::Foundation::PWSTR) -> HMENU;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadStringA(hinstance: super::super::Foundation::HINSTANCE, uid: u32, lpbuffer: super::super::Foundation::PSTR, cchbuffermax: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadStringW(hinstance: super::super::Foundation::HINSTANCE, uid: u32, lpbuffer: super::super::Foundation::PWSTR, cchbuffermax: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LockSetForegroundWindow(ulockcode: FOREGROUND_WINDOW_LOCK_CODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogicalToPhysicalPoint(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupIconIdFromDirectory(presbits: *const u8, ficon: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupIconIdFromDirectoryEx(presbits: *const u8, ficon: super::super::Foundation::BOOL, cxdesired: i32, cydesired: i32, flags: IMAGE_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapDialogRect(hdlg: super::super::Foundation::HWND, lprect: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MenuItemFromPoint(hwnd: super::super::Foundation::HWND, hmenu: HMENU, ptscreen: super::super::Foundation::POINT) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MessageBoxA(hwnd: super::super::Foundation::HWND, lptext: super::super::Foundation::PSTR, lpcaption: super::super::Foundation::PSTR, utype: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MessageBoxExA(hwnd: super::super::Foundation::HWND, lptext: super::super::Foundation::PSTR, lpcaption: super::super::Foundation::PSTR, utype: MESSAGEBOX_STYLE, wlanguageid: u16) -> MESSAGEBOX_RESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MessageBoxExW(hwnd: super::super::Foundation::HWND, lptext: super::super::Foundation::PWSTR, lpcaption: super::super::Foundation::PWSTR, utype: MESSAGEBOX_STYLE, wlanguageid: u16) -> MESSAGEBOX_RESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_UI_Shell`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub fn MessageBoxIndirectA(lpmbp: *const ::core::mem::ManuallyDrop<MSGBOXPARAMSA>) -> MESSAGEBOX_RESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_UI_Shell`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub fn MessageBoxIndirectW(lpmbp: *const ::core::mem::ManuallyDrop<MSGBOXPARAMSW>) -> MESSAGEBOX_RESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MessageBoxW(hwnd: super::super::Foundation::HWND, lptext: super::super::Foundation::PWSTR, lpcaption: super::super::Foundation::PWSTR, utype: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ModifyMenuA(hmnu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ModifyMenuW(hmnu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS, uidnewitem: usize, lpnewitem: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveWindow(hwnd: super::super::Foundation::HWND, x: i32, y: i32, nwidth: i32, nheight: i32, brepaint: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateConfig(platformversion: MrmPlatformVersion, defaultqualifiers: super::super::Foundation::PWSTR, outputxmlfile: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateConfigInMemory(platformversion: MrmPlatformVersion, defaultqualifiers: super::super::Foundation::PWSTR, outputxmldata: *mut *mut u8, outputxmlsize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceFile(indexer: MrmResourceIndexerHandle, packagingmode: MrmPackagingMode, packagingoptions: MrmPackagingOptions, outputdirectory: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn MrmCreateResourceFileInMemory(indexer: MrmResourceIndexerHandle, packagingmode: MrmPackagingMode, packagingoptions: MrmPackagingOptions, outputpridata: *mut *mut u8, outputprisize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceFileWithChecksum(indexer: MrmResourceIndexerHandle, packagingmode: MrmPackagingMode, packagingoptions: MrmPackagingOptions, checksum: u32, outputdirectory: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceIndexer(packagefamilyname: super::super::Foundation::PWSTR, projectroot: super::super::Foundation::PWSTR, platformversion: MrmPlatformVersion, defaultqualifiers: super::super::Foundation::PWSTR, indexer: *mut MrmResourceIndexerHandle) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceIndexerFromPreviousPriData(projectroot: super::super::Foundation::PWSTR, platformversion: MrmPlatformVersion, defaultqualifiers: super::super::Foundation::PWSTR, pridata: *const u8, prisize: u32, indexer: *mut MrmResourceIndexerHandle) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceIndexerFromPreviousPriFile(projectroot: super::super::Foundation::PWSTR, platformversion: MrmPlatformVersion, defaultqualifiers: super::super::Foundation::PWSTR, prifile: super::super::Foundation::PWSTR, indexer: *mut MrmResourceIndexerHandle) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceIndexerFromPreviousSchemaData(projectroot: super::super::Foundation::PWSTR, platformversion: MrmPlatformVersion, defaultqualifiers: super::super::Foundation::PWSTR, schemaxmldata: *const u8, schemaxmlsize: u32, indexer: *mut MrmResourceIndexerHandle) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceIndexerFromPreviousSchemaFile(projectroot: super::super::Foundation::PWSTR, platformversion: MrmPlatformVersion, defaultqualifiers: super::super::Foundation::PWSTR, schemafile: super::super::Foundation::PWSTR, indexer: *mut MrmResourceIndexerHandle) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceIndexerWithFlags(packagefamilyname: super::super::Foundation::PWSTR, projectroot: super::super::Foundation::PWSTR, platformversion: MrmPlatformVersion, defaultqualifiers: super::super::Foundation::PWSTR, flags: MrmIndexerFlags, indexer: *mut MrmResourceIndexerHandle) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn MrmDestroyIndexerAndMessages(indexer: MrmResourceIndexerHandle) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn MrmDumpPriDataInMemory(inputpridata: *const u8, inputprisize: u32, schemapridata: *const u8, schemaprisize: u32, dumptype: MrmDumpType, outputxmldata: *mut *mut u8, outputxmlsize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmDumpPriFile(indexfilename: super::super::Foundation::PWSTR, schemaprifile: super::super::Foundation::PWSTR, dumptype: MrmDumpType, outputxmlfile: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmDumpPriFileInMemory(indexfilename: super::super::Foundation::PWSTR, schemaprifile: super::super::Foundation::PWSTR, dumptype: MrmDumpType, outputxmldata: *mut *mut u8, outputxmlsize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn MrmFreeMemory(data: *const u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmGetPriFileContentChecksum(prifile: super::super::Foundation::PWSTR, checksum: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmIndexEmbeddedData(indexer: MrmResourceIndexerHandle, resourceuri: super::super::Foundation::PWSTR, embeddeddata: *const u8, embeddeddatasize: u32, qualifiers: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmIndexFile(indexer: MrmResourceIndexerHandle, resourceuri: super::super::Foundation::PWSTR, filepath: super::super::Foundation::PWSTR, qualifiers: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmIndexFileAutoQualifiers(indexer: MrmResourceIndexerHandle, filepath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmIndexResourceContainerAutoQualifiers(indexer: MrmResourceIndexerHandle, containerpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmIndexString(indexer: MrmResourceIndexerHandle, resourceuri: super::super::Foundation::PWSTR, resourcestring: super::super::Foundation::PWSTR, qualifiers: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmPeekResourceIndexerMessages(handle: MrmResourceIndexerHandle, messages: *mut *mut MrmResourceIndexerMessage, nummsgs: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsgWaitForMultipleObjects(ncount: u32, phandles: *const super::super::Foundation::HANDLE, fwaitall: super::super::Foundation::BOOL, dwmilliseconds: u32, dwwakemask: QUEUE_STATUS_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsgWaitForMultipleObjectsEx(ncount: u32, phandles: *const super::super::Foundation::HANDLE, dwmilliseconds: u32, dwwakemask: QUEUE_STATUS_FLAGS, dwflags: MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OemToCharA(psrc: super::super::Foundation::PSTR, pdst: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OemToCharBuffA(lpszsrc: super::super::Foundation::PSTR, lpszdst: super::super::Foundation::PSTR, cchdstlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OemToCharBuffW(lpszsrc: super::super::Foundation::PSTR, lpszdst: super::super::Foundation::PWSTR, cchdstlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OemToCharW(psrc: super::super::Foundation::PSTR, pdst: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenIcon(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeekMessageA(lpmsg: *mut MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeekMessageW(lpmsg: *mut MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PhysicalToLogicalPoint(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PostMessageA(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PostMessageW(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn PostQuitMessage(nexitcode: i32);
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PostThreadMessageA(idthread: u32, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PostThreadMessageW(idthread: u32, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivateExtractIconsA(szfilename: super::super::Foundation::PSTR, niconindex: i32, cxicon: i32, cyicon: i32, phicon: *mut HICON, piconid: *mut u32, nicons: u32, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivateExtractIconsW(szfilename: super::super::Foundation::PWSTR, niconindex: i32, cxicon: i32, cyicon: i32, phicon: *mut HICON, piconid: *mut u32, nicons: u32, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RealChildWindowFromPoint(hwndparent: super::super::Foundation::HWND, ptparentclientcoords: super::super::Foundation::POINT) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RealGetWindowClassA(hwnd: super::super::Foundation::HWND, ptszclassname: super::super::Foundation::PSTR, cchclassnamemax: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RealGetWindowClassW(hwnd: super::super::Foundation::HWND, ptszclassname: super::super::Foundation::PWSTR, cchclassnamemax: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn RegisterClassA(lpwndclass: *const ::core::mem::ManuallyDrop<WNDCLASSA>) -> u16;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn RegisterClassExA(param0: *const ::core::mem::ManuallyDrop<WNDCLASSEXA>) -> u16;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn RegisterClassExW(param0: *const ::core::mem::ManuallyDrop<WNDCLASSEXW>) -> u16;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn RegisterClassW(lpwndclass: *const ::core::mem::ManuallyDrop<WNDCLASSW>) -> u16;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_System_Power`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Power"))]
    pub fn RegisterDeviceNotificationA(hrecipient: super::super::Foundation::HANDLE, notificationfilter: *const ::core::ffi::c_void, flags: super::super::System::Power::POWER_SETTING_REGISTER_NOTIFICATION_FLAGS) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_System_Power`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Power"))]
    pub fn RegisterDeviceNotificationW(hrecipient: super::super::Foundation::HANDLE, notificationfilter: *const ::core::ffi::c_void, flags: super::super::System::Power::POWER_SETTING_REGISTER_NOTIFICATION_FLAGS) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterShellHookWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterWindowMessageA(lpstring: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterWindowMessageW(lpstring: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveMenu(hmenu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePropA(hwnd: super::super::Foundation::HWND, lpstring: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePropW(hwnd: super::super::Foundation::HWND, lpstring: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplyMessage(lresult: super::super::Foundation::LRESULT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScrollDC(hdc: super::super::Graphics::Gdi::HDC, dx: i32, dy: i32, lprcscroll: *const super::super::Foundation::RECT, lprcclip: *const super::super::Foundation::RECT, hrgnupdate: super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScrollWindow(hwnd: super::super::Foundation::HWND, xamount: i32, yamount: i32, lprect: *const super::super::Foundation::RECT, lpcliprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScrollWindowEx(hwnd: super::super::Foundation::HWND, dx: i32, dy: i32, prcscroll: *const super::super::Foundation::RECT, prcclip: *const super::super::Foundation::RECT, hrgnupdate: super::super::Graphics::Gdi::HRGN, prcupdate: *mut super::super::Foundation::RECT, flags: SHOW_WINDOW_CMD) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendDlgItemMessageA(hdlg: super::super::Foundation::HWND, niddlgitem: i32, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendDlgItemMessageW(hdlg: super::super::Foundation::HWND, niddlgitem: i32, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendMessageA(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendMessageCallbackA(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lpresultcallback: ::windows::runtime::RawPtr, dwdata: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendMessageCallbackW(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lpresultcallback: ::windows::runtime::RawPtr, dwdata: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendMessageTimeoutA(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, fuflags: SEND_MESSAGE_TIMEOUT_FLAGS, utimeout: u32, lpdwresult: *mut usize) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendMessageTimeoutW(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, fuflags: SEND_MESSAGE_TIMEOUT_FLAGS, utimeout: u32, lpdwresult: *mut usize) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendMessageW(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendNotifyMessageA(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendNotifyMessageW(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCaretBlinkTime(umseconds: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCaretPos(x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClassLongA(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX, dwnewlong: i32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClassLongPtrA(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX, dwnewlong: isize) -> usize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClassLongPtrW(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX, dwnewlong: isize) -> usize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClassLongW(hwnd: super::super::Foundation::HWND, nindex: GET_CLASS_LONG_INDEX, dwnewlong: i32) -> u32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClassWord(hwnd: super::super::Foundation::HWND, nindex: i32, wnewword: u16) -> u16;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCoalescableTimer(hwnd: super::super::Foundation::HWND, nidevent: usize, uelapse: u32, lptimerfunc: ::windows::runtime::RawPtr, utolerancedelay: u32) -> usize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn SetCursor(hcursor: HCURSOR) -> HCURSOR;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCursorPos(x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn SetDebugErrorLevel(dwlevel: u32);
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDlgItemInt(hdlg: super::super::Foundation::HWND, niddlgitem: i32, uvalue: u32, bsigned: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDlgItemTextA(hdlg: super::super::Foundation::HWND, niddlgitem: i32, lpstring: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDlgItemTextW(hdlg: super::super::Foundation::HWND, niddlgitem: i32, lpstring: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetForegroundWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLayeredWindowAttributes(hwnd: super::super::Foundation::HWND, crkey: u32, balpha: u8, dwflags: LAYERED_WINDOW_ATTRIBUTES_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMenu(hwnd: super::super::Foundation::HWND, hmenu: HMENU) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMenuDefaultItem(hmenu: HMENU, uitem: u32, fbypos: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetMenuInfo(param0: HMENU, param1: *const MENUINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetMenuItemBitmaps(hmenu: HMENU, uposition: u32, uflags: MENU_ITEM_FLAGS, hbitmapunchecked: super::super::Graphics::Gdi::HBITMAP, hbitmapchecked: super::super::Graphics::Gdi::HBITMAP) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetMenuItemInfoA(hmenu: HMENU, item: u32, fbypositon: super::super::Foundation::BOOL, lpmii: *const MENUITEMINFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetMenuItemInfoW(hmenu: HMENU, item: u32, fbypositon: super::super::Foundation::BOOL, lpmii: *const MENUITEMINFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMessageExtraInfo(lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LPARAM;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMessageQueue(cmessagesmax: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetParent(hwndchild: super::super::Foundation::HWND, hwndnewparent: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPhysicalCursorPos(x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDPIAware() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDefaultLayout(dwdefaultlayout: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPropA(hwnd: super::super::Foundation::HWND, lpstring: super::super::Foundation::PSTR, hdata: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPropW(hwnd: super::super::Foundation::HWND, lpstring: super::super::Foundation::PWSTR, hdata: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSysColors(celements: i32, lpaelements: *const i32, lpargbvalues: *const u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemCursor(hcur: HCURSOR, id: SYSTEM_CURSOR_ID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTimer(hwnd: super::super::Foundation::HWND, nidevent: usize, uelapse: u32, lptimerfunc: ::windows::runtime::RawPtr) -> usize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowDisplayAffinity(hwnd: super::super::Foundation::HWND, dwaffinity: WINDOW_DISPLAY_AFFINITY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowLongA(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowLongPtrA(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: isize) -> isize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowLongPtrW(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: isize) -> isize;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowLongW(hwnd: super::super::Foundation::HWND, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowPlacement(hwnd: super::super::Foundation::HWND, lpwndpl: *const WINDOWPLACEMENT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowPos(hwnd: super::super::Foundation::HWND, hwndinsertafter: super::super::Foundation::HWND, x: i32, y: i32, cx: i32, cy: i32, uflags: SET_WINDOW_POS_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowTextA(hwnd: super::super::Foundation::HWND, lpstring: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowTextW(hwnd: super::super::Foundation::HWND, lpstring: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowWord(hwnd: super::super::Foundation::HWND, nindex: i32, wnewword: u16) -> u16;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowsHookA(nfiltertype: i32, pfnfilterproc: ::windows::runtime::RawPtr) -> HHOOK;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowsHookExA(idhook: WINDOWS_HOOK_ID, lpfn: ::windows::runtime::RawPtr, hmod: super::super::Foundation::HINSTANCE, dwthreadid: u32) -> HHOOK;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowsHookExW(idhook: WINDOWS_HOOK_ID, lpfn: ::windows::runtime::RawPtr, hmod: super::super::Foundation::HINSTANCE, dwthreadid: u32) -> HHOOK;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowsHookW(nfiltertype: i32, pfnfilterproc: ::windows::runtime::RawPtr) -> HHOOK;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowCaret(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowCursor(bshow: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowOwnedPopups(hwnd: super::super::Foundation::HWND, fshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowWindow(hwnd: super::super::Foundation::HWND, ncmdshow: SHOW_WINDOW_CMD) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowWindowAsync(hwnd: super::super::Foundation::HWND, ncmdshow: SHOW_WINDOW_CMD) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SoundSentry() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwitchToThisWindow(hwnd: super::super::Foundation::HWND, funknown: super::super::Foundation::BOOL);
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemParametersInfoA(uiaction: SYSTEM_PARAMETERS_INFO_ACTION, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemParametersInfoW(uiaction: SYSTEM_PARAMETERS_INFO_ACTION, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TileWindows(hwndparent: super::super::Foundation::HWND, whow: TILE_WINDOWS_HOW, lprect: *const super::super::Foundation::RECT, ckids: u32, lpkids: *const super::super::Foundation::HWND) -> u16;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TrackPopupMenu(hmenu: HMENU, uflags: TRACK_POPUP_MENU_FLAGS, x: i32, y: i32, nreserved: i32, hwnd: super::super::Foundation::HWND, prcrect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TrackPopupMenuEx(hmenu: HMENU, uflags: u32, x: i32, y: i32, hwnd: super::super::Foundation::HWND, lptpm: *const TPMPARAMS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateAcceleratorA(hwnd: super::super::Foundation::HWND, hacctable: HACCEL, lpmsg: *const MSG) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateAcceleratorW(hwnd: super::super::Foundation::HWND, hacctable: HACCEL, lpmsg: *const MSG) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateMDISysAccel(hwndclient: super::super::Foundation::HWND, lpmsg: *const MSG) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateMessage(lpmsg: *const MSG) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnhookWindowsHook(ncode: i32, pfnfilterproc: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnhookWindowsHookEx(hhk: HHOOK) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterClassA(lpclassname: super::super::Foundation::PSTR, hinstance: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterClassW(lpclassname: super::super::Foundation::PWSTR, hinstance: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn UpdateLayeredWindow(hwnd: super::super::Foundation::HWND, hdcdst: super::super::Graphics::Gdi::HDC, pptdst: *const super::super::Foundation::POINT, psize: *const super::super::Foundation::SIZE, hdcsrc: super::super::Graphics::Gdi::HDC, pptsrc: *const super::super::Foundation::POINT, crkey: u32, pblend: *const super::super::Graphics::Gdi::BLENDFUNCTION, dwflags: UPDATE_LAYERED_WINDOW_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn UpdateLayeredWindowIndirect(hwnd: super::super::Foundation::HWND, pulwinfo: *const UPDATELAYEREDWINDOWINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitMessage() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowFromPhysicalPoint(point: super::super::Foundation::POINT) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowFromPoint(point: super::super::Foundation::POINT) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wsprintfA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wsprintfW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wvsprintfA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, arglist: *const i8) -> i32;
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wvsprintfW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, arglist: *const i8) -> i32;
}
