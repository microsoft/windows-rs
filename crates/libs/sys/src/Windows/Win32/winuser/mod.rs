#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn ActivateKeyboardLayout(hkl : super::HKL, flags : u32) -> super::HKL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn AddClipboardFormatListener(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn AdjustWindowRect(lprect : *mut super::RECT, dwstyle : u32, bmenu : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn AdjustWindowRectEx(lprect : *mut super::RECT, dwstyle : u32, bmenu : windows_sys::core::BOOL, dwexstyle : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn AdjustWindowRectExForDpi(lprect : *mut super::RECT, dwstyle : u32, bmenu : windows_sys::core::BOOL, dwexstyle : u32, dpi : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn AllowSetForegroundWindow(dwprocessid : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn AnimateWindow(hwnd : super::HWND, dwtime : u32, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn AnyPopup() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn AppendMenuA(hmenu : super::HMENU, uflags : u32, uidnewitem : usize, lpnewitem : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn AppendMenuW(hmenu : super::HMENU, uflags : u32, uidnewitem : usize, lpnewitem : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ApplyWindowAction(hwnd : super::HWND, paction : *mut WINDOW_ACTION) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn AreDpiAwarenessContextsEqual(dpicontexta : super::DPI_AWARENESS_CONTEXT, dpicontextb : super::DPI_AWARENESS_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ArrangeIconicWindows(hwnd : super::HWND) -> u32);
windows_link::link!("user32.dll" "system" fn AttachThreadInput(idattach : u32, idattachto : u32, fattach : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn BeginDeferWindowPos(nnumwindows : i32) -> HDWP);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn BeginPaint(hwnd : super::HWND, lppaint : *mut PAINTSTRUCT) -> super::HDC);
windows_link::link!("user32.dll" "system" fn BlockInput(fblockit : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn BringWindowToTop(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn BroadcastSystemMessageA(flags : u32, lpinfo : *mut u32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn BroadcastSystemMessageExA(flags : u32, lpinfo : *mut u32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM, pbsminfo : *mut BSMINFO) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn BroadcastSystemMessageExW(flags : u32, lpinfo : *mut u32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM, pbsminfo : *mut BSMINFO) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn BroadcastSystemMessageW(flags : u32, lpinfo : *mut u32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CalculatePopupWindowPosition(anchorpoint : *const super::POINT, windowsize : *const super::SIZE, flags : u32, excluderect : *const super::RECT, popupwindowposition : *mut super::RECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CallMsgFilterA(lpmsg : *const MSG, ncode : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CallMsgFilterW(lpmsg : *const MSG, ncode : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CallNextHookEx(hhk : super::HHOOK, ncode : i32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CallWindowProcA(lpprevwndfunc : WNDPROC, hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CallWindowProcW(lpprevwndfunc : WNDPROC, hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
windows_link::link!("user32.dll" "system" fn CancelShutdown() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CascadeWindows(hwndparent : super::HWND, whow : u32, lprect : *const super::RECT, ckids : u32, lpkids : *const super::HWND) -> u16);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ChangeClipboardChain(hwndremove : super::HWND, hwndnewnext : super::HWND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("user32.dll" "system" fn ChangeDisplaySettingsA(lpdevmode : *const super::DEVMODEA, dwflags : u32) -> i32);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("user32.dll" "system" fn ChangeDisplaySettingsExA(lpszdevicename : windows_sys::core::PCSTR, lpdevmode : *const super::DEVMODEA, hwnd : super::HWND, dwflags : u32, lparam : *const core::ffi::c_void) -> i32);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("user32.dll" "system" fn ChangeDisplaySettingsExW(lpszdevicename : windows_sys::core::PCWSTR, lpdevmode : *const super::DEVMODEW, hwnd : super::HWND, dwflags : u32, lparam : *const core::ffi::c_void) -> i32);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("user32.dll" "system" fn ChangeDisplaySettingsW(lpdevmode : *const super::DEVMODEW, dwflags : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ChangeMenuA(hmenu : super::HMENU, cmd : u32, lpsznewitem : windows_sys::core::PCSTR, cmdinsert : u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ChangeMenuW(hmenu : super::HMENU, cmd : u32, lpsznewitem : windows_sys::core::PCWSTR, cmdinsert : u32, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn ChangeWindowMessageFilter(message : u32, dwflag : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ChangeWindowMessageFilterEx(hwnd : super::HWND, message : u32, action : u32, pchangefilterstruct : *mut CHANGEFILTERSTRUCT) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn CharLowerA(lpsz : windows_sys::core::PSTR) -> windows_sys::core::PSTR);
windows_link::link!("user32.dll" "system" fn CharLowerBuffA(lpsz : windows_sys::core::PSTR, cchlength : u32) -> u32);
windows_link::link!("user32.dll" "system" fn CharLowerBuffW(lpsz : windows_sys::core::PWSTR, cchlength : u32) -> u32);
windows_link::link!("user32.dll" "system" fn CharLowerW(lpsz : windows_sys::core::PWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("user32.dll" "system" fn CharNextA(lpsz : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("user32.dll" "system" fn CharNextExA(codepage : u16, lpcurrentchar : windows_sys::core::PCSTR, dwflags : u32) -> windows_sys::core::PSTR);
windows_link::link!("user32.dll" "system" fn CharNextW(lpsz : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("user32.dll" "system" fn CharPrevA(lpszstart : windows_sys::core::PCSTR, lpszcurrent : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("user32.dll" "system" fn CharPrevExA(codepage : u16, lpstart : windows_sys::core::PCSTR, lpcurrentchar : windows_sys::core::PCSTR, dwflags : u32) -> windows_sys::core::PSTR);
windows_link::link!("user32.dll" "system" fn CharPrevW(lpszstart : windows_sys::core::PCWSTR, lpszcurrent : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("user32.dll" "system" fn CharToOemA(psrc : windows_sys::core::PCSTR, pdst : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn CharToOemBuffA(lpszsrc : windows_sys::core::PCSTR, lpszdst : windows_sys::core::PSTR, cchdstlength : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn CharToOemBuffW(lpszsrc : windows_sys::core::PCWSTR, lpszdst : windows_sys::core::PSTR, cchdstlength : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn CharToOemW(psrc : windows_sys::core::PCWSTR, pdst : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn CharUpperA(lpsz : windows_sys::core::PSTR) -> windows_sys::core::PSTR);
windows_link::link!("user32.dll" "system" fn CharUpperBuffA(lpsz : windows_sys::core::PSTR, cchlength : u32) -> u32);
windows_link::link!("user32.dll" "system" fn CharUpperBuffW(lpsz : windows_sys::core::PWSTR, cchlength : u32) -> u32);
windows_link::link!("user32.dll" "system" fn CharUpperW(lpsz : windows_sys::core::PWSTR) -> windows_sys::core::PWSTR);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CheckDlgButton(hdlg : super::HWND, nidbutton : i32, ucheck : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CheckMenuItem(hmenu : super::HMENU, uidcheckitem : u32, ucheck : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CheckMenuRadioItem(hmenu : super::HMENU, first : u32, last : u32, check : u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CheckRadioButton(hdlg : super::HWND, nidfirstbutton : i32, nidlastbutton : i32, nidcheckbutton : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ChildWindowFromPoint(hwndparent : super::HWND, point : super::POINT) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ChildWindowFromPointEx(hwnd : super::HWND, pt : super::POINT, flags : u32) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ClientToScreen(hwnd : super::HWND, lppoint : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ClipCursor(lprect : *const super::RECT) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn CloseClipboard() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CloseDesktop(hdesktop : super::HDESK) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn CloseGestureInfoHandle(hgestureinfo : HGESTUREINFO) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn CloseTouchInputHandle(htouchinput : HTOUCHINPUT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CloseWindow(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn CloseWindowStation(hwinsta : super::HWINSTA) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn ConvertPrimaryPointerToMouseDrag() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ConvertToInterceptWindow(toplevelwindow : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CopyAcceleratorTableA(haccelsrc : super::HACCEL, lpacceldst : *mut ACCEL, caccelentries : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CopyAcceleratorTableW(haccelsrc : super::HACCEL, lpacceldst : *mut ACCEL, caccelentries : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CopyIcon(hicon : super::HICON) -> super::HICON);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn CopyImage(h : super::HANDLE, r#type : u32, cx : i32, cy : i32, flags : u32) -> super::HANDLE);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CopyRect(lprcdst : *mut super::RECT, lprcsrc : *const super::RECT) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn CountClipboardFormats() -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CreateAcceleratorTableA(paccel : *const ACCEL, caccel : i32) -> super::HACCEL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CreateAcceleratorTableW(paccel : *const ACCEL, caccel : i32) -> super::HACCEL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CreateCaret(hwnd : super::HWND, hbitmap : super::HBITMAP, nwidth : i32, nheight : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CreateCursor(hinst : super::HINSTANCE, xhotspot : i32, yhotspot : i32, nwidth : i32, nheight : i32, pvandplane : *const core::ffi::c_void, pvxorplane : *const core::ffi::c_void) -> super::HCURSOR);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn CreateDesktopA(lpszdesktop : windows_sys::core::PCSTR, lpszdevice : windows_sys::core::PCSTR, pdevmode : *const super::DEVMODEA, dwflags : u32, dwdesiredaccess : super::ACCESS_MASK, lpsa : *const super::SECURITY_ATTRIBUTES) -> super::HDESK);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn CreateDesktopExA(lpszdesktop : windows_sys::core::PCSTR, lpszdevice : windows_sys::core::PCSTR, pdevmode : *const super::DEVMODEA, dwflags : u32, dwdesiredaccess : super::ACCESS_MASK, lpsa : *const super::SECURITY_ATTRIBUTES, ulheapsize : u32, pvoid : *const core::ffi::c_void) -> super::HDESK);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn CreateDesktopExW(lpszdesktop : windows_sys::core::PCWSTR, lpszdevice : windows_sys::core::PCWSTR, pdevmode : *const super::DEVMODEW, dwflags : u32, dwdesiredaccess : super::ACCESS_MASK, lpsa : *const super::SECURITY_ATTRIBUTES, ulheapsize : u32, pvoid : *const core::ffi::c_void) -> super::HDESK);
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn CreateDesktopW(lpszdesktop : windows_sys::core::PCWSTR, lpszdevice : windows_sys::core::PCWSTR, pdevmode : *const super::DEVMODEW, dwflags : u32, dwdesiredaccess : super::ACCESS_MASK, lpsa : *const super::SECURITY_ATTRIBUTES) -> super::HDESK);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CreateDialogIndirectParamA(hinstance : super::HINSTANCE, lptemplate : *const DLGTEMPLATE, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> super::HWND);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CreateDialogIndirectParamW(hinstance : super::HINSTANCE, lptemplate : *const DLGTEMPLATE, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> super::HWND);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CreateDialogParamA(hinstance : super::HINSTANCE, lptemplatename : windows_sys::core::PCSTR, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> super::HWND);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CreateDialogParamW(hinstance : super::HINSTANCE, lptemplatename : windows_sys::core::PCWSTR, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> super::HWND);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CreateIcon(hinstance : super::HINSTANCE, nwidth : i32, nheight : i32, cplanes : u8, cbitspixel : u8, lpbandbits : *const u8, lpbxorbits : *const u8) -> super::HICON);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CreateIconFromResource(presbits : *const u8, dwressize : u32, ficon : windows_sys::core::BOOL, dwver : u32) -> super::HICON);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CreateIconFromResourceEx(presbits : *const u8, dwressize : u32, ficon : windows_sys::core::BOOL, dwver : u32, cxdesired : i32, cydesired : i32, flags : u32) -> super::HICON);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CreateIconIndirect(piconinfo : *const ICONINFO) -> super::HICON);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CreateMDIWindowA(lpclassname : windows_sys::core::PCSTR, lpwindowname : windows_sys::core::PCSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::HWND, hinstance : super::HINSTANCE, lparam : super::LPARAM) -> super::HWND);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CreateMDIWindowW(lpclassname : windows_sys::core::PCWSTR, lpwindowname : windows_sys::core::PCWSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::HWND, hinstance : super::HINSTANCE, lparam : super::LPARAM) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CreateMenu() -> super::HMENU);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn CreatePopupMenu() -> super::HMENU);
windows_link::link!("user32.dll" "system" fn CreateSyntheticPointerDevice(pointertype : POINTER_INPUT_TYPE, maxcount : u32, mode : POINTER_FEEDBACK_MODE) -> HSYNTHETICPOINTERDEVICE);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CreateWindowExA(dwexstyle : u32, lpclassname : windows_sys::core::PCSTR, lpwindowname : windows_sys::core::PCSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::HWND, hmenu : super::HMENU, hinstance : super::HINSTANCE, lpparam : *const core::ffi::c_void) -> super::HWND);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn CreateWindowExW(dwexstyle : u32, lpclassname : windows_sys::core::PCWSTR, lpwindowname : windows_sys::core::PCWSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::HWND, hmenu : super::HMENU, hinstance : super::HINSTANCE, lpparam : *const core::ffi::c_void) -> super::HWND);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn CreateWindowStationA(lpwinsta : windows_sys::core::PCSTR, dwflags : u32, dwdesiredaccess : super::ACCESS_MASK, lpsa : *const super::SECURITY_ATTRIBUTES) -> super::HWINSTA);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn CreateWindowStationW(lpwinsta : windows_sys::core::PCWSTR, dwflags : u32, dwdesiredaccess : super::ACCESS_MASK, lpsa : *const super::SECURITY_ATTRIBUTES) -> super::HWINSTA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DefDlgProcA(hdlg : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DefDlgProcW(hdlg : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DefFrameProcA(hwnd : super::HWND, hwndmdiclient : super::HWND, umsg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DefFrameProcW(hwnd : super::HWND, hwndmdiclient : super::HWND, umsg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DefMDIChildProcA(hwnd : super::HWND, umsg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DefMDIChildProcW(hwnd : super::HWND, umsg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn DefRawInputProc(parawinput : *const PRAWINPUT, ninput : i32, cbsizeheader : u32) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DefWindowProcA(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DefWindowProcW(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn DeferWindowPos(hwinposinfo : HDWP, hwnd : super::HWND, hwndinsertafter : super::HWND, x : i32, y : i32, cx : i32, cy : i32, uflags : u32) -> HDWP);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DeleteMenu(hmenu : super::HMENU, uposition : u32, uflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DeregisterShellHookWindow(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DestroyAcceleratorTable(haccel : super::HACCEL) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DestroyCaret() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DestroyCursor(hcursor : super::HCURSOR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DestroyIcon(hicon : super::HICON) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DestroyMenu(hmenu : super::HMENU) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DestroySyntheticPointerDevice(device : HSYNTHETICPOINTERDEVICE));
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DestroyWindow(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DialogBoxIndirectParamA(hinstance : super::HINSTANCE, hdialogtemplate : *const DLGTEMPLATE, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> isize);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DialogBoxIndirectParamW(hinstance : super::HINSTANCE, hdialogtemplate : *const DLGTEMPLATE, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> isize);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DialogBoxParamA(hinstance : super::HINSTANCE, lptemplatename : windows_sys::core::PCSTR, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> isize);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DialogBoxParamW(hinstance : super::HINSTANCE, lptemplatename : windows_sys::core::PCWSTR, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> isize);
windows_link::link!("user32.dll" "system" fn DisableProcessWindowsGhosting());
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DispatchMessageA(lpmsg : *const MSG) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DispatchMessageW(lpmsg : *const MSG) -> super::LRESULT);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn DisplayConfigGetDeviceInfo(requestpacket : *mut super::DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn DisplayConfigSetDeviceInfo(setpacket : *const super::DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DlgDirListA(hdlg : super::HWND, lppathspec : windows_sys::core::PSTR, nidlistbox : i32, nidstaticpath : i32, ufiletype : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DlgDirListComboBoxA(hdlg : super::HWND, lppathspec : windows_sys::core::PSTR, nidcombobox : i32, nidstaticpath : i32, ufiletype : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DlgDirListComboBoxW(hdlg : super::HWND, lppathspec : windows_sys::core::PWSTR, nidcombobox : i32, nidstaticpath : i32, ufiletype : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DlgDirListW(hdlg : super::HWND, lppathspec : windows_sys::core::PWSTR, nidlistbox : i32, nidstaticpath : i32, ufiletype : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DlgDirSelectComboBoxExA(hwnddlg : super::HWND, lpstring : windows_sys::core::PSTR, cchout : i32, idcombobox : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DlgDirSelectComboBoxExW(hwnddlg : super::HWND, lpstring : windows_sys::core::PWSTR, cchout : i32, idcombobox : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DlgDirSelectExA(hwnddlg : super::HWND, lpstring : windows_sys::core::PSTR, chcount : i32, idlistbox : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DlgDirSelectExW(hwnddlg : super::HWND, lpstring : windows_sys::core::PWSTR, chcount : i32, idlistbox : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DragDetect(hwnd : super::HWND, pt : super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DragObject(hwndparent : super::HWND, hwndfrom : super::HWND, fmt : u32, data : usize, hcur : super::HCURSOR) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DrawAnimatedRects(hwnd : super::HWND, idani : i32, lprcfrom : *const super::RECT, lprcto : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DrawCaption(hwnd : super::HWND, hdc : super::HDC, lprect : *const super::RECT, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DrawEdge(hdc : super::HDC, qrc : *mut super::RECT, edge : u32, grfflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DrawFocusRect(hdc : super::HDC, lprc : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DrawFrameControl(hdc : super::HDC, lprc : *mut super::RECT, utype : u32, ustate : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DrawIcon(hdc : super::HDC, x : i32, y : i32, hicon : super::HICON) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DrawIconEx(hdc : super::HDC, xleft : i32, ytop : i32, hicon : super::HICON, cxwidth : i32, cywidth : i32, istepifanicur : u32, hbrflickerfreedraw : super::HBRUSH, diflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DrawMenuBar(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DrawStateA(hdc : super::HDC, hbrfore : super::HBRUSH, qfncallback : DRAWSTATEPROC, ldata : super::LPARAM, wdata : super::WPARAM, x : i32, y : i32, cx : i32, cy : i32, uflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn DrawStateW(hdc : super::HDC, hbrfore : super::HBRUSH, qfncallback : DRAWSTATEPROC, ldata : super::LPARAM, wdata : super::WPARAM, x : i32, y : i32, cx : i32, cy : i32, uflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DrawTextA(hdc : super::HDC, lpchtext : windows_sys::core::PCSTR, cchtext : i32, lprc : *mut super::RECT, format : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DrawTextExA(hdc : super::HDC, lpchtext : windows_sys::core::PCSTR, cchtext : i32, lprc : *mut super::RECT, format : u32, lpdtp : *const DRAWTEXTPARAMS) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DrawTextExW(hdc : super::HDC, lpchtext : windows_sys::core::PCWSTR, cchtext : i32, lprc : *mut super::RECT, format : u32, lpdtp : *const DRAWTEXTPARAMS) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn DrawTextW(hdc : super::HDC, lpchtext : windows_sys::core::PCWSTR, cchtext : i32, lprc : *mut super::RECT, format : u32) -> i32);
windows_link::link!("user32.dll" "system" fn EmptyClipboard() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn EnableMenuItem(hmenu : super::HMENU, uidenableitem : u32, uenable : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn EnableMouseInPointer(fenable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn EnableNonClientDpiScaling(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn EnableScrollBar(hwnd : super::HWND, wsbflags : u32, warrows : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn EnableWindow(hwnd : super::HWND, benable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn EndDeferWindowPos(hwinposinfo : HDWP) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn EndDialog(hdlg : super::HWND, nresult : isize) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn EndMenu() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn EndPaint(hwnd : super::HWND, lppaint : *const PAINTSTRUCT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn EnterMoveSizeLoop(hwnd : super::HWND, ptcursor : super::POINT, movesizecode : MOVESIZE_OPERATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn EnumChildWindows(hwndparent : super::HWND, lpenumfunc : WNDENUMPROC, lparam : super::LPARAM) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn EnumClipboardFormats(format : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn EnumDesktopWindows(hdesktop : super::HDESK, lpfn : WNDENUMPROC, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn EnumDesktopsA(hwinsta : super::HWINSTA, lpenumfunc : DESKTOPENUMPROCA, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn EnumDesktopsW(hwinsta : super::HWINSTA, lpenumfunc : DESKTOPENUMPROCW, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "wingdi")]
windows_link::link!("user32.dll" "system" fn EnumDisplayDevicesA(lpdevice : windows_sys::core::PCSTR, idevnum : u32, lpdisplaydevice : *mut super::DISPLAY_DEVICEA, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "wingdi")]
windows_link::link!("user32.dll" "system" fn EnumDisplayDevicesW(lpdevice : windows_sys::core::PCWSTR, idevnum : u32, lpdisplaydevice : *mut super::DISPLAY_DEVICEW, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn EnumDisplayMonitors(hdc : super::HDC, lprcclip : *const super::RECT, lpfnenum : MONITORENUMPROC, dwdata : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("user32.dll" "system" fn EnumDisplaySettingsA(lpszdevicename : windows_sys::core::PCSTR, imodenum : u32, lpdevmode : *mut super::DEVMODEA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("user32.dll" "system" fn EnumDisplaySettingsExA(lpszdevicename : windows_sys::core::PCSTR, imodenum : u32, lpdevmode : *mut super::DEVMODEA, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("user32.dll" "system" fn EnumDisplaySettingsExW(lpszdevicename : windows_sys::core::PCWSTR, imodenum : u32, lpdevmode : *mut super::DEVMODEW, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("user32.dll" "system" fn EnumDisplaySettingsW(lpszdevicename : windows_sys::core::PCWSTR, imodenum : u32, lpdevmode : *mut super::DEVMODEW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn EnumPropsA(hwnd : super::HWND, lpenumfunc : PROPENUMPROCA) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn EnumPropsExA(hwnd : super::HWND, lpenumfunc : PROPENUMPROCEXA, lparam : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn EnumPropsExW(hwnd : super::HWND, lpenumfunc : PROPENUMPROCEXW, lparam : super::LPARAM) -> i32);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn EnumPropsW(hwnd : super::HWND, lpenumfunc : PROPENUMPROCW) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn EnumThreadWindows(dwthreadid : u32, lpfn : WNDENUMPROC, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn EnumWindowStationsA(lpenumfunc : WINSTAENUMPROCA, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn EnumWindowStationsW(lpenumfunc : WINSTAENUMPROCW, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn EnumWindows(lpenumfunc : WNDENUMPROC, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn EqualRect(lprc1 : *const super::RECT, lprc2 : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn EvaluateProximityToPolygon(numvertices : u32, controlpolygon : *const super::POINT, phittestinginput : *const TOUCH_HIT_TESTING_INPUT, pproximityeval : *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn EvaluateProximityToRect(controlboundingbox : *const super::RECT, phittestinginput : *const TOUCH_HIT_TESTING_INPUT, pproximityeval : *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ExcludeUpdateRgn(hdc : super::HDC, hwnd : super::HWND) -> i32);
windows_link::link!("user32.dll" "system" fn ExitWindowsEx(uflags : u32, dwreason : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn FillRect(hdc : super::HDC, lprc : *const super::RECT, hbr : super::HBRUSH) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn FindWindowA(lpclassname : windows_sys::core::PCSTR, lpwindowname : windows_sys::core::PCSTR) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn FindWindowExA(hwndparent : super::HWND, hwndchildafter : super::HWND, lpszclass : windows_sys::core::PCSTR, lpszwindow : windows_sys::core::PCSTR) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn FindWindowExW(hwndparent : super::HWND, hwndchildafter : super::HWND, lpszclass : windows_sys::core::PCWSTR, lpszwindow : windows_sys::core::PCWSTR) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn FindWindowW(lpclassname : windows_sys::core::PCWSTR, lpwindowname : windows_sys::core::PCWSTR) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn FlashWindow(hwnd : super::HWND, binvert : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn FlashWindowEx(pfwi : *const FLASHWINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn FrameRect(hdc : super::HDC, lprc : *const super::RECT, hbr : super::HBRUSH) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetActiveWindow() -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetAltTabInfoA(hwnd : super::HWND, iitem : i32, pati : *mut ALTTABINFO, pszitemtext : windows_sys::core::PSTR, cchitemtext : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetAltTabInfoW(hwnd : super::HWND, iitem : i32, pati : *mut ALTTABINFO, pszitemtext : windows_sys::core::PWSTR, cchitemtext : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetAncestor(hwnd : super::HWND, gaflags : u32) -> super::HWND);
windows_link::link!("user32.dll" "system" fn GetAsyncKeyState(vkey : i32) -> i16);
windows_link::link!("user32.dll" "system" fn GetAutoRotationState(pstate : *mut AR_STATE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetAwarenessFromDpiAwarenessContext(value : super::DPI_AWARENESS_CONTEXT) -> super::DPI_AWARENESS);
windows_link::link!("user32.dll" "system" fn GetCIMSSM(inputmessagesource : *mut INPUT_MESSAGE_SOURCE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetCapture() -> super::HWND);
windows_link::link!("user32.dll" "system" fn GetCaretBlinkTime() -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetCaretPos(lppoint : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn GetClassInfoA(hinstance : super::HINSTANCE, lpclassname : windows_sys::core::PCSTR, lpwndclass : *mut WNDCLASSA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn GetClassInfoExA(hinstance : super::HINSTANCE, lpszclass : windows_sys::core::PCSTR, lpwcx : *mut WNDCLASSEXA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn GetClassInfoExW(hinstance : super::HINSTANCE, lpszclass : windows_sys::core::PCWSTR, lpwcx : *mut WNDCLASSEXW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn GetClassInfoW(hinstance : super::HINSTANCE, lpclassname : windows_sys::core::PCWSTR, lpwndclass : *mut WNDCLASSW) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetClassLongA(hwnd : super::HWND, nindex : i32) -> u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetClassLongPtrA(hwnd : super::HWND, nindex : i32) -> usize);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetClassLongPtrW(hwnd : super::HWND, nindex : i32) -> usize);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetClassLongW(hwnd : super::HWND, nindex : i32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetClassNameA(hwnd : super::HWND, lpclassname : windows_sys::core::PSTR, nmaxcount : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetClassNameW(hwnd : super::HWND, lpclassname : windows_sys::core::PWSTR, nmaxcount : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetClassWord(hwnd : super::HWND, nindex : i32) -> u16);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetClientRect(hwnd : super::HWND, lprect : *mut super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetClipCursor(lprect : *mut super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn GetClipboardData(uformat : u32) -> super::HANDLE);
windows_link::link!("user32.dll" "system" fn GetClipboardFormatNameA(format : u32, lpszformatname : windows_sys::core::PSTR, cchmaxcount : i32) -> i32);
windows_link::link!("user32.dll" "system" fn GetClipboardFormatNameW(format : u32, lpszformatname : windows_sys::core::PWSTR, cchmaxcount : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetClipboardOwner() -> super::HWND);
windows_link::link!("user32.dll" "system" fn GetClipboardSequenceNumber() -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetClipboardViewer() -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetComboBoxInfo(hwndcombo : super::HWND, pcbi : *mut COMBOBOXINFO) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetCurrentInputMessageSource(inputmessagesource : *mut INPUT_MESSAGE_SOURCE) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetCurrentMonitorTopologyId() -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetCursor() -> super::HCURSOR);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetCursorInfo(pci : *mut CURSORINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetCursorPos(lppoint : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetDC(hwnd : super::HWND) -> super::HDC);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn GetDCEx(hwnd : super::HWND, hrgnclip : super::HRGN, flags : u32) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetDesktopWindow() -> super::HWND);
windows_link::link!("user32.dll" "system" fn GetDialogBaseUnits() -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetDialogControlDpiChangeBehavior(hwnd : super::HWND) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetDialogDpiChangeBehavior(hdlg : super::HWND) -> DIALOG_DPI_CHANGE_BEHAVIORS);
windows_link::link!("user32.dll" "system" fn GetDisplayAutoRotationPreferences(porientation : *mut ORIENTATION_PREFERENCE) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetDisplayConfigBufferSizes(flags : u32, numpatharrayelements : *mut u32, nummodeinfoarrayelements : *mut u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetDlgCtrlID(hwnd : super::HWND) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetDlgItem(hdlg : super::HWND, niddlgitem : i32) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetDlgItemInt(hdlg : super::HWND, niddlgitem : i32, lptranslated : *mut windows_sys::core::BOOL, bsigned : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetDlgItemTextA(hdlg : super::HWND, niddlgitem : i32, lpstring : windows_sys::core::PSTR, cchmax : i32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetDlgItemTextW(hdlg : super::HWND, niddlgitem : i32, lpstring : windows_sys::core::PWSTR, cchmax : i32) -> u32);
windows_link::link!("user32.dll" "system" fn GetDoubleClickTime() -> u32);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetDpiAwarenessContextForProcess(hprocess : super::HANDLE) -> super::DPI_AWARENESS_CONTEXT);
windows_link::link!("user32.dll" "system" fn GetDpiForSystem() -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetDpiForWindow(hwnd : super::HWND) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetDpiFromDpiAwarenessContext(value : super::DPI_AWARENESS_CONTEXT) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetFocus() -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetForegroundWindow() -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetGUIThreadInfo(idthread : u32, pgui : *mut GUITHREADINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetGestureConfig(hwnd : super::HWND, dwreserved : u32, dwflags : u32, pcids : *const u32, pgestureconfig : *mut GESTURECONFIG, cbsize : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetGestureExtraArgs(hgestureinfo : HGESTUREINFO, cbextraargs : u32, pextraargs : *mut u8) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetGestureInfo(hgestureinfo : HGESTUREINFO, pgestureinfo : *mut GESTUREINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn GetGuiResources(hprocess : super::HANDLE, uiflags : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetIconInfo(hicon : super::HICON, piconinfo : *mut ICONINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetIconInfoExA(hicon : super::HICON, piconinfo : *mut ICONINFOEXA) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetIconInfoExW(hicon : super::HICON, piconinfo : *mut ICONINFOEXW) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetInputState() -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetKBCodePage() -> u32);
windows_link::link!("user32.dll" "system" fn GetKeyNameTextA(lparam : i32, lpstring : windows_sys::core::PSTR, cchsize : i32) -> i32);
windows_link::link!("user32.dll" "system" fn GetKeyNameTextW(lparam : i32, lpstring : windows_sys::core::PWSTR, cchsize : i32) -> i32);
windows_link::link!("user32.dll" "system" fn GetKeyState(nvirtkey : i32) -> i16);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn GetKeyboardLayout(idthread : u32) -> super::HKL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn GetKeyboardLayoutList(nbuff : i32, lplist : *mut super::HKL) -> i32);
windows_link::link!("user32.dll" "system" fn GetKeyboardLayoutNameA(pwszklid : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetKeyboardLayoutNameW(pwszklid : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetKeyboardState(lpkeystate : *mut u8) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetKeyboardType(ntypeflag : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetLastActivePopup(hwnd : super::HWND) -> super::HWND);
windows_link::link!("user32.dll" "system" fn GetLastInputInfo(plii : *mut LASTINPUTINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetLayeredWindowAttributes(hwnd : super::HWND, pcrkey : *mut super::COLORREF, pbalpha : *mut u8, pdwflags : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetListBoxInfo(hwnd : super::HWND) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMenu(hwnd : super::HWND) -> super::HMENU);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMenuBarInfo(hwnd : super::HWND, idobject : i32, iditem : i32, pmbi : *mut MENUBARINFO) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetMenuCheckMarkDimensions() -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMenuContextHelpId(param0 : super::HMENU) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMenuDefaultItem(hmenu : super::HMENU, fbypos : u32, gmdiflags : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMenuInfo(param0 : super::HMENU, param1 : *mut MENUINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMenuItemCount(hmenu : super::HMENU) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMenuItemID(hmenu : super::HMENU, npos : i32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMenuItemInfoA(hmenu : super::HMENU, item : u32, fbyposition : windows_sys::core::BOOL, lpmii : *mut MENUITEMINFOA) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMenuItemInfoW(hmenu : super::HMENU, item : u32, fbyposition : windows_sys::core::BOOL, lpmii : *mut MENUITEMINFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMenuItemRect(hwnd : super::HWND, hmenu : super::HMENU, uitem : u32, lprcitem : *mut super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMenuState(hmenu : super::HMENU, uid : u32, uflags : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMenuStringA(hmenu : super::HMENU, uiditem : u32, lpstring : windows_sys::core::PSTR, cchmax : i32, flags : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMenuStringW(hmenu : super::HMENU, uiditem : u32, lpstring : windows_sys::core::PWSTR, cchmax : i32, flags : u32) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn GetMessageA(lpmsg : *mut MSG, hwnd : super::HWND, wmsgfiltermin : u32, wmsgfiltermax : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn GetMessageExtraInfo() -> super::LPARAM);
windows_link::link!("user32.dll" "system" fn GetMessagePos() -> u32);
windows_link::link!("user32.dll" "system" fn GetMessageTime() -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn GetMessageW(lpmsg : *mut MSG, hwnd : super::HWND, wmsgfiltermin : u32, wmsgfiltermax : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMonitorInfoA(hmonitor : super::HMONITOR, lpmi : *mut MONITORINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetMonitorInfoW(hmonitor : super::HMONITOR, lpmi : *mut MONITORINFO) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetMouseMovePointsEx(cbsize : u32, lppt : *const MOUSEMOVEPOINT, lpptbuf : *mut MOUSEMOVEPOINT, nbufpoints : i32, resolution : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetNextDlgGroupItem(hdlg : super::HWND, hctl : super::HWND, bprevious : windows_sys::core::BOOL) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetNextDlgTabItem(hdlg : super::HWND, hctl : super::HWND, bprevious : windows_sys::core::BOOL) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetOpenClipboardWindow() -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetParent(hwnd : super::HWND) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetPhysicalCursorPos(lppoint : *mut super::POINT) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetPointerCursorId(pointerid : u32, cursorid : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerDevice(device : super::HANDLE, pointerdevice : *mut POINTER_DEVICE_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn GetPointerDeviceCursors(device : super::HANDLE, cursorcount : *mut u32, devicecursors : *mut POINTER_DEVICE_CURSOR_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn GetPointerDeviceProperties(device : super::HANDLE, propertycount : *mut u32, pointerproperties : *mut POINTER_DEVICE_PROPERTY) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerDeviceRects(device : super::HANDLE, pointerdevicerect : *mut super::RECT, displayrect : *mut super::RECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerDevices(devicecount : *mut u32, pointerdevices : *mut POINTER_DEVICE_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerFrameInfo(pointerid : u32, pointercount : *mut u32, pointerinfo : *mut POINTER_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerFrameInfoHistory(pointerid : u32, entriescount : *mut u32, pointercount : *mut u32, pointerinfo : *mut POINTER_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerFramePenInfo(pointerid : u32, pointercount : *mut u32, peninfo : *mut POINTER_PEN_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerFramePenInfoHistory(pointerid : u32, entriescount : *mut u32, pointercount : *mut u32, peninfo : *mut POINTER_PEN_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerFrameTouchInfo(pointerid : u32, pointercount : *mut u32, touchinfo : *mut POINTER_TOUCH_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerFrameTouchInfoHistory(pointerid : u32, entriescount : *mut u32, pointercount : *mut u32, touchinfo : *mut POINTER_TOUCH_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerInfo(pointerid : u32, pointerinfo : *mut POINTER_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerInfoHistory(pointerid : u32, entriescount : *mut u32, pointerinfo : *mut POINTER_INFO) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetPointerInputTransform(pointerid : u32, historycount : u32, inputtransform : *mut INPUT_TRANSFORM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerPenInfo(pointerid : u32, peninfo : *mut POINTER_PEN_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerPenInfoHistory(pointerid : u32, entriescount : *mut u32, peninfo : *mut POINTER_PEN_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerTouchInfo(pointerid : u32, touchinfo : *mut POINTER_TOUCH_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPointerTouchInfoHistory(pointerid : u32, entriescount : *mut u32, touchinfo : *mut POINTER_TOUCH_INFO) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetPointerType(pointerid : u32, pointertype : *mut POINTER_INPUT_TYPE) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetPriorityClipboardFormat(paformatprioritylist : *const u32, cformats : i32) -> i32);
windows_link::link!("user32.dll" "system" fn GetProcessDefaultLayout(pdwdefaultlayout : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn GetProcessWindowStation() -> super::HWINSTA);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPropA(hwnd : super::HWND, lpstring : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetPropW(hwnd : super::HWND, lpstring : windows_sys::core::PCWSTR) -> super::HANDLE);
windows_link::link!("user32.dll" "system" fn GetQueueStatus(flags : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn GetRawInputBuffer(pdata : *mut RAWINPUT, pcbsize : *mut u32, cbsizeheader : u32) -> u32);
windows_link::link!("user32.dll" "system" fn GetRawInputData(hrawinput : HRAWINPUT, uicommand : u32, pdata : *mut core::ffi::c_void, pcbsize : *mut u32, cbsizeheader : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn GetRawInputDeviceInfoA(hdevice : super::HANDLE, uicommand : u32, pdata : *mut core::ffi::c_void, pcbsize : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn GetRawInputDeviceInfoW(hdevice : super::HANDLE, uicommand : u32, pdata : *mut core::ffi::c_void, pcbsize : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn GetRawInputDeviceList(prawinputdevicelist : *mut RAWINPUTDEVICELIST, puinumdevices : *mut u32, cbsize : u32) -> u32);
windows_link::link!("user32.dll" "system" fn GetRawPointerDeviceData(pointerid : u32, historycount : u32, propertiescount : u32, pproperties : *const POINTER_DEVICE_PROPERTY, pvalues : *mut i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetRegisteredRawInputDevices(prawinputdevices : *mut RAWINPUTDEVICE, puinumdevices : *mut u32, cbsize : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetScrollBarInfo(hwnd : super::HWND, idobject : i32, psbi : *mut SCROLLBARINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetScrollInfo(hwnd : super::HWND, nbar : i32, lpsi : *mut SCROLLINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetScrollPos(hwnd : super::HWND, nbar : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetScrollRange(hwnd : super::HWND, nbar : i32, lpminpos : *mut i32, lpmaxpos : *mut i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetShellWindow() -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetSubMenu(hmenu : super::HMENU, npos : i32) -> super::HMENU);
windows_link::link!("user32.dll" "system" fn GetSysColor(nindex : i32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetSysColorBrush(nindex : i32) -> super::HBRUSH);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn GetSystemDpiForProcess(hprocess : super::HANDLE) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetSystemMenu(hwnd : super::HWND, brevert : windows_sys::core::BOOL) -> super::HMENU);
windows_link::link!("user32.dll" "system" fn GetSystemMetrics(nindex : i32) -> i32);
windows_link::link!("user32.dll" "system" fn GetSystemMetricsForDpi(nindex : i32, dpi : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetTabbedTextExtentA(hdc : super::HDC, lpstring : windows_sys::core::PCSTR, chcount : i32, ntabpositions : i32, lpntabstoppositions : *const i32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetTabbedTextExtentW(hdc : super::HDC, lpstring : windows_sys::core::PCWSTR, chcount : i32, ntabpositions : i32, lpntabstoppositions : *const i32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetThreadDesktop(dwthreadid : u32) -> super::HDESK);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetThreadDpiAwarenessContext() -> super::DPI_AWARENESS_CONTEXT);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetThreadDpiHostingBehavior() -> super::DPI_HOSTING_BEHAVIOR);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetTitleBarInfo(hwnd : super::HWND, pti : *mut TITLEBARINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetTopWindow(hwnd : super::HWND) -> super::HWND);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn GetTouchInputInfo(htouchinput : HTOUCHINPUT, cinputs : u32, pinputs : *mut TOUCHINPUT, cbsize : i32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetUnpredictedMessagePos() -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetUpdateRect(hwnd : super::HWND, lprect : *mut super::RECT, berase : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn GetUpdateRgn(hwnd : super::HWND, hrgn : super::HRGN, berase : windows_sys::core::BOOL) -> i32);
windows_link::link!("user32.dll" "system" fn GetUpdatedClipboardFormats(lpuiformats : *mut u32, cformats : u32, pcformatsout : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn GetUserObjectInformationA(hobj : super::HANDLE, nindex : i32, pvinfo : *mut core::ffi::c_void, nlength : u32, lpnlengthneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn GetUserObjectInformationW(hobj : super::HANDLE, nindex : i32, pvinfo : *mut core::ffi::c_void, nlength : u32, lpnlengthneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn GetUserObjectSecurity(hobj : super::HANDLE, psirequested : *const u32, psid : super::PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindow(hwnd : super::HWND, ucmd : u32) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowContextHelpId(param0 : super::HWND) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowDC(hwnd : super::HWND) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowDisplayAffinity(hwnd : super::HWND, pdwaffinity : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowDpiAwarenessContext(hwnd : super::HWND) -> super::DPI_AWARENESS_CONTEXT);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowDpiHostingBehavior(hwnd : super::HWND) -> super::DPI_HOSTING_BEHAVIOR);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowFeedbackSetting(hwnd : super::HWND, feedback : FEEDBACK_TYPE, dwflags : u32, psize : *mut u32, config : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn GetWindowInfo(hwnd : super::HWND, pwi : *mut WINDOWINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowLongA(hwnd : super::HWND, nindex : i32) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowLongPtrA(hwnd : super::HWND, nindex : i32) -> isize);
#[cfg(target_pointer_width = "32")]
pub use GetWindowLongA as GetWindowLongPtrA;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowLongPtrW(hwnd : super::HWND, nindex : i32) -> isize);
#[cfg(target_pointer_width = "32")]
pub use GetWindowLongW as GetWindowLongPtrW;
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowLongW(hwnd : super::HWND, nindex : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowModuleFileNameA(hwnd : super::HWND, pszfilename : windows_sys::core::PSTR, cchfilenamemax : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowModuleFileNameW(hwnd : super::HWND, pszfilename : windows_sys::core::PWSTR, cchfilenamemax : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowPlacement(hwnd : super::HWND, lpwndpl : *mut WINDOWPLACEMENT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowRect(hwnd : super::HWND, lprect : *mut super::RECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn GetWindowRgn(hwnd : super::HWND, hrgn : super::HRGN) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowRgnBox(hwnd : super::HWND, lprc : *mut super::RECT) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowTextA(hwnd : super::HWND, lpstring : windows_sys::core::PSTR, nmaxcount : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowTextLengthA(hwnd : super::HWND) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowTextLengthW(hwnd : super::HWND) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowTextW(hwnd : super::HWND, lpstring : windows_sys::core::PWSTR, nmaxcount : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowThreadProcessId(hwnd : super::HWND, lpdwprocessid : *mut u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn GetWindowWord(hwnd : super::HWND, nindex : i32) -> u16);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn GrayStringA(hdc : super::HDC, hbrush : super::HBRUSH, lpoutputfunc : GRAYSTRINGPROC, lpdata : super::LPARAM, ncount : i32, x : i32, y : i32, nwidth : i32, nheight : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn GrayStringW(hdc : super::HDC, hbrush : super::HBRUSH, lpoutputfunc : GRAYSTRINGPROC, lpdata : super::LPARAM, ncount : i32, x : i32, y : i32, nwidth : i32, nheight : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn HideCaret(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn HiliteMenuItem(hwnd : super::HWND, hmenu : super::HMENU, uidhiliteitem : u32, uhilite : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn InSendMessage() -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn InSendMessageEx(lpreserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn InflateRect(lprc : *mut super::RECT, dx : i32, dy : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn InheritWindowMonitor(hwnd : super::HWND, hwndinherit : super::HWND) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn InitializeTouchInjection(maxcount : u32, dwmode : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn InjectSyntheticPointerInput(device : HSYNTHETICPOINTERDEVICE, pointerinfo : *const POINTER_TYPE_INFO, count : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn InjectTouchInput(count : u32, contacts : *const POINTER_TOUCH_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn InsertMenuA(hmenu : super::HMENU, uposition : u32, uflags : u32, uidnewitem : usize, lpnewitem : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn InsertMenuItemA(hmenu : super::HMENU, item : u32, fbyposition : windows_sys::core::BOOL, lpmi : *const MENUITEMINFOA) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn InsertMenuItemW(hmenu : super::HMENU, item : u32, fbyposition : windows_sys::core::BOOL, lpmi : *const MENUITEMINFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn InsertMenuW(hmenu : super::HMENU, uposition : u32, uflags : u32, uidnewitem : usize, lpnewitem : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn InternalGetWindowText(hwnd : super::HWND, pstring : windows_sys::core::PWSTR, cchmaxcount : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IntersectRect(lprcdst : *mut super::RECT, lprcsrc1 : *const super::RECT, lprcsrc2 : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn InvalidateRect(hwnd : super::HWND, lprect : *const super::RECT, berase : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn InvalidateRgn(hwnd : super::HWND, hrgn : super::HRGN, berase : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn InvertRect(hdc : super::HDC, lprc : *const super::RECT) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsCharAlphaA(ch : i8) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsCharAlphaNumericA(ch : i8) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsCharAlphaNumericW(ch : u16) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsCharAlphaW(ch : u16) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsCharLowerA(ch : i8) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsCharLowerW(ch : u16) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsCharUpperA(ch : i8) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsCharUpperW(ch : u16) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsChild(hwndparent : super::HWND, hwnd : super::HWND) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsClipboardFormatAvailable(format : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn IsDialogMessageA(hdlg : super::HWND, lpmsg : *const MSG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn IsDialogMessageW(hdlg : super::HWND, lpmsg : *const MSG) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsDlgButtonChecked(hdlg : super::HWND, nidbutton : i32) -> u32);
windows_link::link!("user32.dll" "system" fn IsGUIThread(bconvert : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsHungAppWindow(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsIconic(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn IsImmersiveProcess(hprocess : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsInterceptWindow(toplevelwindow : super::HWND, isintercept : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsMenu(hmenu : super::HMENU) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsMouseInPointerEnabled() -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsProcessDPIAware() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsRectEmpty(lprc : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsTouchWindow(hwnd : super::HWND, pulflags : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsValidDpiAwarenessContext(value : super::DPI_AWARENESS_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsWinEventHookInstalled(event : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsWindow(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsWindowArranged(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsWindowEnabled(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsWindowUnicode(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsWindowVisible(hwnd : super::HWND) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn IsWow64Message() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn IsZoomed(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn KillTimer(hwnd : super::HWND, uidevent : usize) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn LoadAcceleratorsA(hinstance : super::HINSTANCE, lptablename : windows_sys::core::PCSTR) -> super::HACCEL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn LoadAcceleratorsW(hinstance : super::HINSTANCE, lptablename : windows_sys::core::PCWSTR) -> super::HACCEL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn LoadBitmapA(hinstance : super::HINSTANCE, lpbitmapname : windows_sys::core::PCSTR) -> super::HBITMAP);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn LoadBitmapW(hinstance : super::HINSTANCE, lpbitmapname : windows_sys::core::PCWSTR) -> super::HBITMAP);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn LoadCursorA(hinstance : super::HINSTANCE, lpcursorname : windows_sys::core::PCSTR) -> super::HCURSOR);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn LoadCursorFromFileA(lpfilename : windows_sys::core::PCSTR) -> super::HCURSOR);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn LoadCursorFromFileW(lpfilename : windows_sys::core::PCWSTR) -> super::HCURSOR);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn LoadCursorW(hinstance : super::HINSTANCE, lpcursorname : windows_sys::core::PCWSTR) -> super::HCURSOR);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn LoadIconA(hinstance : super::HINSTANCE, lpiconname : windows_sys::core::PCSTR) -> super::HICON);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn LoadIconW(hinstance : super::HINSTANCE, lpiconname : windows_sys::core::PCWSTR) -> super::HICON);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn LoadImageA(hinst : super::HINSTANCE, name : windows_sys::core::PCSTR, r#type : u32, cx : i32, cy : i32, fuload : u32) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn LoadImageW(hinst : super::HINSTANCE, name : windows_sys::core::PCWSTR, r#type : u32, cx : i32, cy : i32, fuload : u32) -> super::HANDLE);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn LoadKeyboardLayoutA(pwszklid : windows_sys::core::PCSTR, flags : u32) -> super::HKL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn LoadKeyboardLayoutW(pwszklid : windows_sys::core::PCWSTR, flags : u32) -> super::HKL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn LoadMenuA(hinstance : super::HINSTANCE, lpmenuname : windows_sys::core::PCSTR) -> super::HMENU);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn LoadMenuIndirectA(lpmenutemplate : *const MENUTEMPLATEA) -> super::HMENU);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn LoadMenuIndirectW(lpmenutemplate : *const MENUTEMPLATEW) -> super::HMENU);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn LoadMenuW(hinstance : super::HINSTANCE, lpmenuname : windows_sys::core::PCWSTR) -> super::HMENU);
windows_link::link!("user32.dll" "system" fn LockSetForegroundWindow(ulockcode : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn LockWindowUpdate(hwndlock : super::HWND) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn LockWorkStation() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn LogicalToPhysicalPoint(hwnd : super::HWND, lppoint : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn LogicalToPhysicalPointForPerMonitorDPI(hwnd : super::HWND, lppoint : *mut super::POINT) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn LookupIconIdFromDirectory(presbits : *const u8, ficon : windows_sys::core::BOOL) -> i32);
windows_link::link!("user32.dll" "system" fn LookupIconIdFromDirectoryEx(presbits : *const u8, ficon : windows_sys::core::BOOL, cxdesired : i32, cydesired : i32, flags : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn MapDialogRect(hdlg : super::HWND, lprect : *mut super::RECT) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn MapVirtualKeyA(ucode : u32, umaptype : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn MapVirtualKeyExA(ucode : u32, umaptype : u32, dwhkl : super::HKL) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn MapVirtualKeyExW(ucode : u32, umaptype : u32, dwhkl : super::HKL) -> u32);
windows_link::link!("user32.dll" "system" fn MapVirtualKeyW(ucode : u32, umaptype : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn MapWindowPoints(hwndfrom : super::HWND, hwndto : super::HWND, lppoints : *mut super::POINT, cpoints : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn MenuItemFromPoint(hwnd : super::HWND, hmenu : super::HMENU, ptscreen : super::POINT) -> i32);
windows_link::link!("user32.dll" "system" fn MessageBeep(utype : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn MessageBoxA(hwnd : super::HWND, lptext : windows_sys::core::PCSTR, lpcaption : windows_sys::core::PCSTR, utype : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn MessageBoxExA(hwnd : super::HWND, lptext : windows_sys::core::PCSTR, lpcaption : windows_sys::core::PCSTR, utype : u32, wlanguageid : u16) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn MessageBoxExW(hwnd : super::HWND, lptext : windows_sys::core::PCWSTR, lpcaption : windows_sys::core::PCWSTR, utype : u32, wlanguageid : u16) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn MessageBoxIndirectA(lpmbp : *const MSGBOXPARAMSA) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn MessageBoxIndirectW(lpmbp : *const MSGBOXPARAMSW) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn MessageBoxW(hwnd : super::HWND, lptext : windows_sys::core::PCWSTR, lpcaption : windows_sys::core::PCWSTR, utype : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ModifyMenuA(hmnu : super::HMENU, uposition : u32, uflags : u32, uidnewitem : usize, lpnewitem : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ModifyMenuW(hmnu : super::HMENU, uposition : u32, uflags : u32, uidnewitem : usize, lpnewitem : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn MonitorFromPoint(pt : super::POINT, dwflags : u32) -> super::HMONITOR);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn MonitorFromRect(lprc : *const super::RECT, dwflags : u32) -> super::HMONITOR);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn MonitorFromWindow(hwnd : super::HWND, dwflags : u32) -> super::HMONITOR);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn MoveWindow(hwnd : super::HWND, x : i32, y : i32, nwidth : i32, nheight : i32, brepaint : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn MsgWaitForMultipleObjects(ncount : u32, phandles : *const super::HANDLE, fwaitall : windows_sys::core::BOOL, dwmilliseconds : u32, dwwakemask : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn MsgWaitForMultipleObjectsEx(ncount : u32, phandles : *const super::HANDLE, dwmilliseconds : u32, dwwakemask : u32, dwflags : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn NotifyWinEvent(event : u32, hwnd : super::HWND, idobject : i32, idchild : i32));
windows_link::link!("user32.dll" "system" fn OemKeyScan(woemchar : u16) -> u32);
windows_link::link!("user32.dll" "system" fn OemToCharA(psrc : windows_sys::core::PCSTR, pdst : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn OemToCharBuffA(lpszsrc : windows_sys::core::PCSTR, lpszdst : windows_sys::core::PSTR, cchdstlength : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn OemToCharBuffW(lpszsrc : windows_sys::core::PCSTR, lpszdst : windows_sys::core::PWSTR, cchdstlength : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn OemToCharW(psrc : windows_sys::core::PCSTR, pdst : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn OffsetRect(lprc : *mut super::RECT, dx : i32, dy : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn OpenClipboard(hwndnewowner : super::HWND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn OpenDesktopA(lpszdesktop : windows_sys::core::PCSTR, dwflags : u32, finherit : windows_sys::core::BOOL, dwdesiredaccess : super::ACCESS_MASK) -> super::HDESK);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn OpenDesktopW(lpszdesktop : windows_sys::core::PCWSTR, dwflags : u32, finherit : windows_sys::core::BOOL, dwdesiredaccess : super::ACCESS_MASK) -> super::HDESK);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn OpenIcon(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn OpenInputDesktop(dwflags : u32, finherit : windows_sys::core::BOOL, dwdesiredaccess : super::ACCESS_MASK) -> super::HDESK);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn OpenWindowStationA(lpszwinsta : windows_sys::core::PCSTR, finherit : windows_sys::core::BOOL, dwdesiredaccess : super::ACCESS_MASK) -> super::HWINSTA);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn OpenWindowStationW(lpszwinsta : windows_sys::core::PCWSTR, finherit : windows_sys::core::BOOL, dwdesiredaccess : super::ACCESS_MASK) -> super::HWINSTA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn PackTouchHitTestingProximityEvaluation(phittestinginput : *const TOUCH_HIT_TESTING_INPUT, pproximityeval : *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::LRESULT);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn PaintDesktop(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn PeekMessageA(lpmsg : *mut MSG, hwnd : super::HWND, wmsgfiltermin : u32, wmsgfiltermax : u32, wremovemsg : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn PeekMessageW(lpmsg : *mut MSG, hwnd : super::HWND, wmsgfiltermin : u32, wmsgfiltermax : u32, wremovemsg : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn PhysicalToLogicalPoint(hwnd : super::HWND, lppoint : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn PhysicalToLogicalPointForPerMonitorDPI(hwnd : super::HWND, lppoint : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn PostMessageA(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn PostMessageW(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn PostQuitMessage(nexitcode : i32));
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn PostThreadMessageA(idthread : u32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn PostThreadMessageW(idthread : u32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn PrintWindow(hwnd : super::HWND, hdcblt : super::HDC, nflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn PrivateExtractIconsA(szfilename : windows_sys::core::PCSTR, niconindex : i32, cxicon : i32, cyicon : i32, phicon : *mut super::HICON, piconid : *mut u32, nicons : u32, flags : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn PrivateExtractIconsW(szfilename : windows_sys::core::PCWSTR, niconindex : i32, cxicon : i32, cyicon : i32, phicon : *mut super::HICON, piconid : *mut u32, nicons : u32, flags : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn PtInRect(lprc : *const super::RECT, pt : super::POINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn QueryDisplayConfig(flags : u32, numpatharrayelements : *mut u32, patharray : *mut super::DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements : *mut u32, modeinfoarray : *mut super::DISPLAYCONFIG_MODE_INFO, currenttopologyid : *mut super::DISPLAYCONFIG_TOPOLOGY_ID) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RealChildWindowFromPoint(hwndparent : super::HWND, ptparentclientcoords : super::POINT) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RealGetWindowClassA(hwnd : super::HWND, ptszclassname : windows_sys::core::PSTR, cchclassnamemax : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RealGetWindowClassW(hwnd : super::HWND, ptszclassname : windows_sys::core::PWSTR, cchclassnamemax : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn RedrawWindow(hwnd : super::HWND, lprcupdate : *const super::RECT, hrgnupdate : super::HRGN, flags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn RegisterClassA(lpwndclass : *const WNDCLASSA) -> super::ATOM);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn RegisterClassExA(param0 : *const WNDCLASSEXA) -> super::ATOM);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn RegisterClassExW(param0 : *const WNDCLASSEXW) -> super::ATOM);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn RegisterClassW(lpwndclass : *const WNDCLASSW) -> super::ATOM);
windows_link::link!("user32.dll" "system" fn RegisterClipboardFormatA(lpszformat : windows_sys::core::PCSTR) -> u32);
windows_link::link!("user32.dll" "system" fn RegisterClipboardFormatW(lpszformat : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RegisterCloakedNotification(hwnd : super::HWND, fregister : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn RegisterDeviceNotificationA(hrecipient : super::HANDLE, notificationfilter : *const core::ffi::c_void, flags : u32) -> HDEVNOTIFY);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn RegisterDeviceNotificationW(hrecipient : super::HANDLE, notificationfilter : *const core::ffi::c_void, flags : u32) -> HDEVNOTIFY);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RegisterForTooltipDismissNotification(hwnd : super::HWND, tdflags : TOOLTIP_DISMISS_FLAGS) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RegisterHotKey(hwnd : super::HWND, id : i32, fsmodifiers : u32, vk : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RegisterPointerDeviceNotifications(window : super::HWND, notifyrange : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RegisterPointerInputTarget(hwnd : super::HWND, pointertype : POINTER_INPUT_TYPE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RegisterPointerInputTargetEx(hwnd : super::HWND, pointertype : POINTER_INPUT_TYPE, fobserve : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn RegisterPowerSettingNotification(hrecipient : super::HANDLE, powersettingguid : *const windows_sys::core::GUID, flags : u32) -> HPOWERNOTIFY);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RegisterRawInputDevices(prawinputdevices : *const RAWINPUTDEVICE, uinumdevices : u32, cbsize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RegisterShellHookWindow(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn RegisterSuspendResumeNotification(hrecipient : super::HANDLE, flags : u32) -> HPOWERNOTIFY);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RegisterTouchHitTestingWindow(hwnd : super::HWND, value : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RegisterTouchWindow(hwnd : super::HWND, ulflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn RegisterWindowMessageA(lpstring : windows_sys::core::PCSTR) -> u32);
windows_link::link!("user32.dll" "system" fn RegisterWindowMessageW(lpstring : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("user32.dll" "system" fn ReleaseCapture() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ReleaseDC(hwnd : super::HWND, hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RemoveClipboardFormatListener(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn RemoveMenu(hmenu : super::HMENU, uposition : u32, uflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn RemovePropA(hwnd : super::HWND, lpstring : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn RemovePropW(hwnd : super::HWND, lpstring : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn ReplyMessage(lresult : super::LRESULT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ScreenToClient(hwnd : super::HWND, lppoint : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn ScrollDC(hdc : super::HDC, dx : i32, dy : i32, lprcscroll : *const super::RECT, lprcclip : *const super::RECT, hrgnupdate : super::HRGN, lprcupdate : *mut super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ScrollWindow(hwnd : super::HWND, xamount : i32, yamount : i32, lprect : *const super::RECT, lpcliprect : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn ScrollWindowEx(hwnd : super::HWND, dx : i32, dy : i32, prcscroll : *const super::RECT, prcclip : *const super::RECT, hrgnupdate : super::HRGN, prcupdate : *mut super::RECT, flags : u32) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SendDlgItemMessageA(hdlg : super::HWND, niddlgitem : i32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SendDlgItemMessageW(hdlg : super::HWND, niddlgitem : i32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
windows_link::link!("user32.dll" "system" fn SendInput(cinputs : u32, pinputs : *const INPUT, cbsize : i32) -> u32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SendMessageA(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SendMessageCallbackA(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM, lpresultcallback : SENDASYNCPROC, dwdata : usize) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SendMessageCallbackW(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM, lpresultcallback : SENDASYNCPROC, dwdata : usize) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SendMessageTimeoutA(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM, fuflags : u32, utimeout : u32, lpdwresult : *mut usize) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SendMessageTimeoutW(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM, fuflags : u32, utimeout : u32, lpdwresult : *mut usize) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SendMessageW(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SendNotifyMessageA(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SendNotifyMessageW(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetActiveWindow(hwnd : super::HWND) -> super::HWND);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn SetAdditionalForegroundBoostProcesses(toplevelwindow : super::HWND, processhandlecount : u32, processhandlearray : *const super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetCapture(hwnd : super::HWND) -> super::HWND);
windows_link::link!("user32.dll" "system" fn SetCaretBlinkTime(umseconds : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SetCaretPos(x : i32, y : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetClassLongA(hwnd : super::HWND, nindex : i32, dwnewlong : i32) -> u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetClassLongPtrA(hwnd : super::HWND, nindex : i32, dwnewlong : isize) -> usize);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetClassLongPtrW(hwnd : super::HWND, nindex : i32, dwnewlong : isize) -> usize);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetClassLongW(hwnd : super::HWND, nindex : i32, dwnewlong : i32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetClassWord(hwnd : super::HWND, nindex : i32, wnewword : u16) -> u16);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn SetClipboardData(uformat : u32, hmem : super::HANDLE) -> super::HANDLE);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetClipboardViewer(hwndnewviewer : super::HWND) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetCoalescableTimer(hwnd : super::HWND, nidevent : usize, uelapse : u32, lptimerfunc : TIMERPROC, utolerancedelay : u32) -> usize);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetCursor(hcursor : super::HCURSOR) -> super::HCURSOR);
windows_link::link!("user32.dll" "system" fn SetCursorPos(x : i32, y : i32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SetDebugErrorLevel(dwlevel : u32));
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetDialogControlDpiChangeBehavior(hwnd : super::HWND, mask : DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values : DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetDialogDpiChangeBehavior(hdlg : super::HWND, mask : DIALOG_DPI_CHANGE_BEHAVIORS, values : DIALOG_DPI_CHANGE_BEHAVIORS) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SetDisplayAutoRotationPreferences(orientation : ORIENTATION_PREFERENCE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn SetDisplayConfig(numpatharrayelements : u32, patharray : *const super::DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements : u32, modeinfoarray : *const super::DISPLAYCONFIG_MODE_INFO, flags : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetDlgItemInt(hdlg : super::HWND, niddlgitem : i32, uvalue : u32, bsigned : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetDlgItemTextA(hdlg : super::HWND, niddlgitem : i32, lpstring : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetDlgItemTextW(hdlg : super::HWND, niddlgitem : i32, lpstring : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SetDoubleClickTime(param0 : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetFocus(hwnd : super::HWND) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetForegroundWindow(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetGestureConfig(hwnd : super::HWND, dwreserved : u32, cids : u32, pgestureconfig : *const GESTURECONFIG, cbsize : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SetKeyboardState(lpkeystate : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SetLastErrorEx(dwerrcode : u32, dwtype : u32));
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetLayeredWindowAttributes(hwnd : super::HWND, crkey : super::COLORREF, balpha : u8, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetMenu(hwnd : super::HWND, hmenu : super::HMENU) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetMenuContextHelpId(param0 : super::HMENU, param1 : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetMenuDefaultItem(hmenu : super::HMENU, uitem : u32, fbypos : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetMenuInfo(param0 : super::HMENU, param1 : *const MENUINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetMenuItemBitmaps(hmenu : super::HMENU, uposition : u32, uflags : u32, hbitmapunchecked : super::HBITMAP, hbitmapchecked : super::HBITMAP) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetMenuItemInfoA(hmenu : super::HMENU, item : u32, fbypositon : windows_sys::core::BOOL, lpmii : *const MENUITEMINFOA) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetMenuItemInfoW(hmenu : super::HMENU, item : u32, fbypositon : windows_sys::core::BOOL, lpmii : *const MENUITEMINFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn SetMessageExtraInfo(lparam : super::LPARAM) -> super::LPARAM);
windows_link::link!("user32.dll" "system" fn SetMessageQueue(cmessagesmax : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetParent(hwndchild : super::HWND, hwndnewparent : super::HWND) -> super::HWND);
windows_link::link!("user32.dll" "system" fn SetPhysicalCursorPos(x : i32, y : i32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SetProcessDPIAware() -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SetProcessDefaultLayout(dwdefaultlayout : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetProcessDpiAwarenessContext(value : super::DPI_AWARENESS_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SetProcessRestrictionExemption(fenableexemption : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn SetProcessWindowStation(hwinsta : super::HWINSTA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn SetPropA(hwnd : super::HWND, lpstring : windows_sys::core::PCSTR, hdata : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn SetPropW(hwnd : super::HWND, lpstring : windows_sys::core::PCWSTR, hdata : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetRect(lprc : *mut super::RECT, xleft : i32, ytop : i32, xright : i32, ybottom : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetRectEmpty(lprc : *mut super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetScrollInfo(hwnd : super::HWND, nbar : i32, lpsi : *const SCROLLINFO, redraw : windows_sys::core::BOOL) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetScrollPos(hwnd : super::HWND, nbar : i32, npos : i32, bredraw : windows_sys::core::BOOL) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetScrollRange(hwnd : super::HWND, nbar : i32, nminpos : i32, nmaxpos : i32, bredraw : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetSysColors(celements : i32, lpaelements : *const i32, lpargbvalues : *const super::COLORREF) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetSystemCursor(hcur : super::HCURSOR, id : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetThreadDesktop(hdesktop : super::HDESK) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetThreadDpiAwarenessContext(dpicontext : super::DPI_AWARENESS_CONTEXT) -> super::DPI_AWARENESS_CONTEXT);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetThreadDpiHostingBehavior(value : super::DPI_HOSTING_BEHAVIOR) -> super::DPI_HOSTING_BEHAVIOR);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetTimer(hwnd : super::HWND, nidevent : usize, uelapse : u32, lptimerfunc : TIMERPROC) -> usize);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn SetUserObjectInformationA(hobj : super::HANDLE, nindex : i32, pvinfo : *const core::ffi::c_void, nlength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn SetUserObjectInformationW(hobj : super::HANDLE, nindex : i32, pvinfo : *const core::ffi::c_void, nlength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn SetUserObjectSecurity(hobj : super::HANDLE, psirequested : *const u32, psid : super::PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SetWinEventHook(eventmin : u32, eventmax : u32, hmodwineventproc : super::HMODULE, pfnwineventproc : WINEVENTPROC, idprocess : u32, idthread : u32, dwflags : u32) -> super::HWINEVENTHOOK);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetWindowContextHelpId(param0 : super::HWND, param1 : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetWindowDisplayAffinity(hwnd : super::HWND, dwaffinity : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetWindowFeedbackSetting(hwnd : super::HWND, feedback : FEEDBACK_TYPE, dwflags : u32, size : u32, configuration : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetWindowLongA(hwnd : super::HWND, nindex : i32, dwnewlong : i32) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetWindowLongPtrA(hwnd : super::HWND, nindex : i32, dwnewlong : isize) -> isize);
#[cfg(target_pointer_width = "32")]
pub use SetWindowLongA as SetWindowLongPtrA;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetWindowLongPtrW(hwnd : super::HWND, nindex : i32, dwnewlong : isize) -> isize);
#[cfg(target_pointer_width = "32")]
pub use SetWindowLongW as SetWindowLongPtrW;
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetWindowLongW(hwnd : super::HWND, nindex : i32, dwnewlong : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetWindowPlacement(hwnd : super::HWND, lpwndpl : *const WINDOWPLACEMENT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetWindowPos(hwnd : super::HWND, hwndinsertafter : super::HWND, x : i32, y : i32, cx : i32, cy : i32, uflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SetWindowRgn(hwnd : super::HWND, hrgn : super::HRGN, bredraw : windows_sys::core::BOOL) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetWindowTextA(hwnd : super::HWND, lpstring : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetWindowTextW(hwnd : super::HWND, lpstring : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SetWindowWord(hwnd : super::HWND, nindex : i32, wnewword : u16) -> u16);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SetWindowsHookA(nfiltertype : i32, pfnfilterproc : HOOKPROC) -> super::HHOOK);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SetWindowsHookExA(idhook : i32, lpfn : HOOKPROC, hmod : super::HINSTANCE, dwthreadid : u32) -> super::HHOOK);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SetWindowsHookExW(idhook : i32, lpfn : HOOKPROC, hmod : super::HINSTANCE, dwthreadid : u32) -> super::HHOOK);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn SetWindowsHookW(nfiltertype : i32, pfnfilterproc : HOOKPROC) -> super::HHOOK);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ShowCaret(hwnd : super::HWND) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn ShowCursor(bshow : windows_sys::core::BOOL) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ShowOwnedPopups(hwnd : super::HWND, fshow : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ShowScrollBar(hwnd : super::HWND, wbar : i32, bshow : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ShowWindow(hwnd : super::HWND, ncmdshow : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ShowWindowAsync(hwnd : super::HWND, ncmdshow : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ShutdownBlockReasonCreate(hwnd : super::HWND, pwszreason : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ShutdownBlockReasonDestroy(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ShutdownBlockReasonQuery(hwnd : super::HWND, pwszbuff : windows_sys::core::PWSTR, pcchbuff : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SkipPointerFrameMessages(pointerid : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SoundSentry() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SubtractRect(lprcdst : *mut super::RECT, lprcsrc1 : *const super::RECT, lprcsrc2 : *const super::RECT) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SwapMouseButton(fswap : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SwitchDesktop(hdesktop : super::HDESK) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn SwitchToThisWindow(hwnd : super::HWND, funknown : windows_sys::core::BOOL));
windows_link::link!("user32.dll" "system" fn SystemParametersInfoA(uiaction : u32, uiparam : u32, pvparam : *mut core::ffi::c_void, fwinini : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SystemParametersInfoForDpi(uiaction : u32, uiparam : u32, pvparam : *mut core::ffi::c_void, fwinini : u32, dpi : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SystemParametersInfoW(uiaction : u32, uiparam : u32, pvparam : *mut core::ffi::c_void, fwinini : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn TabbedTextOutA(hdc : super::HDC, x : i32, y : i32, lpstring : windows_sys::core::PCSTR, chcount : i32, ntabpositions : i32, lpntabstoppositions : *const i32, ntaborigin : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn TabbedTextOutW(hdc : super::HDC, x : i32, y : i32, lpstring : windows_sys::core::PCWSTR, chcount : i32, ntabpositions : i32, lpntabstoppositions : *const i32, ntaborigin : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn TileWindows(hwndparent : super::HWND, whow : u32, lprect : *const super::RECT, ckids : u32, lpkids : *const super::HWND) -> u16);
windows_link::link!("user32.dll" "system" fn ToAscii(uvirtkey : u32, uscancode : u32, lpkeystate : *const u8, lpchar : *mut u16, uflags : u32) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn ToAsciiEx(uvirtkey : u32, uscancode : u32, lpkeystate : *const u8, lpchar : *mut u16, uflags : u32, dwhkl : super::HKL) -> i32);
windows_link::link!("user32.dll" "system" fn ToUnicode(wvirtkey : u32, wscancode : u32, lpkeystate : *const u8, pwszbuff : windows_sys::core::PWSTR, cchbuff : i32, wflags : u32) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn ToUnicodeEx(wvirtkey : u32, wscancode : u32, lpkeystate : *const u8, pwszbuff : windows_sys::core::PWSTR, cchbuff : i32, wflags : u32, dwhkl : super::HKL) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn TrackMouseEvent(lpeventtrack : *mut TRACKMOUSEEVENT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn TrackPopupMenu(hmenu : super::HMENU, uflags : u32, x : i32, y : i32, nreserved : i32, hwnd : super::HWND, prcrect : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn TrackPopupMenuEx(hmenu : super::HMENU, uflags : u32, x : i32, y : i32, hwnd : super::HWND, lptpm : *const TPMPARAMS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn TranslateAcceleratorA(hwnd : super::HWND, hacctable : super::HACCEL, lpmsg : *const MSG) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn TranslateAcceleratorW(hwnd : super::HWND, hacctable : super::HACCEL, lpmsg : *const MSG) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn TranslateMDISysAccel(hwndclient : super::HWND, lpmsg : *const MSG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn TranslateMessage(lpmsg : *const MSG) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn UnhookWinEvent(hwineventhook : super::HWINEVENTHOOK) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn UnhookWindowsHook(ncode : i32, pfnfilterproc : HOOKPROC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn UnhookWindowsHookEx(hhk : super::HHOOK) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn UnionRect(lprcdst : *mut super::RECT, lprcsrc1 : *const super::RECT, lprcsrc2 : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn UnloadKeyboardLayout(hkl : super::HKL) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn UnregisterClassA(lpclassname : windows_sys::core::PCSTR, hinstance : super::HINSTANCE) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn UnregisterClassW(lpclassname : windows_sys::core::PCWSTR, hinstance : super::HINSTANCE) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn UnregisterDeviceNotification(handle : HDEVNOTIFY) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn UnregisterHotKey(hwnd : super::HWND, id : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn UnregisterPointerInputTarget(hwnd : super::HWND, pointertype : POINTER_INPUT_TYPE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn UnregisterPointerInputTargetEx(hwnd : super::HWND, pointertype : POINTER_INPUT_TYPE) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn UnregisterPowerSettingNotification(handle : HPOWERNOTIFY) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn UnregisterSuspendResumeNotification(handle : HPOWERNOTIFY) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn UnregisterTouchWindow(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("user32.dll" "system" fn UpdateLayeredWindow(hwnd : super::HWND, hdcdst : super::HDC, pptdst : *const super::POINT, psize : *const super::SIZE, hdcsrc : super::HDC, pptsrc : *const super::POINT, crkey : super::COLORREF, pblend : *const super::BLENDFUNCTION, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("user32.dll" "system" fn UpdateLayeredWindowIndirect(hwnd : super::HWND, pulwinfo : *const UPDATELAYEREDWINDOWINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn UpdateWindow(hwnd : super::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn UserHandleGrantAccess(huserhandle : super::HANDLE, hjob : super::HANDLE, bgrant : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ValidateRect(hwnd : super::HWND, lprect : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("user32.dll" "system" fn ValidateRgn(hwnd : super::HWND, hrgn : super::HRGN) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn VkKeyScanA(ch : i8) -> i16);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn VkKeyScanExA(ch : i8, dwhkl : super::HKL) -> i16);
#[cfg(feature = "minwindef")]
windows_link::link!("user32.dll" "system" fn VkKeyScanExW(ch : u16, dwhkl : super::HKL) -> i16);
windows_link::link!("user32.dll" "system" fn VkKeyScanW(ch : u16) -> i16);
#[cfg(feature = "winnt")]
windows_link::link!("user32.dll" "system" fn WaitForInputIdle(hprocess : super::HANDLE, dwmilliseconds : u32) -> u32);
windows_link::link!("user32.dll" "system" fn WaitMessage() -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn WinHelpA(hwndmain : super::HWND, lpszhelp : windows_sys::core::PCSTR, ucommand : u32, dwdata : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn WinHelpW(hwndmain : super::HWND, lpszhelp : windows_sys::core::PCWSTR, ucommand : u32, dwdata : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn WindowFromDC(hdc : super::HDC) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn WindowFromPhysicalPoint(point : super::POINT) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn WindowFromPoint(point : super::POINT) -> super::HWND);
windows_link::link!("user32.dll" "system" fn keybd_event(bvk : u8, bscan : u8, dwflags : u32, dwextrainfo : usize));
windows_link::link!("user32.dll" "system" fn mouse_event(dwflags : u32, dx : u32, dy : u32, dwdata : u32, dwextrainfo : usize));
windows_link::link!("user32.dll" "C" fn wsprintfA(param0 : windows_sys::core::PSTR, param1 : windows_sys::core::PCSTR, ...) -> i32);
windows_link::link!("user32.dll" "C" fn wsprintfW(param0 : windows_sys::core::PWSTR, param1 : windows_sys::core::PCWSTR, ...) -> i32);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("user32.dll" "system" fn wvsprintfA(param0 : windows_sys::core::PSTR, param1 : windows_sys::core::PCSTR, arglist : *const i8) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
windows_link::link!("user32.dll" "system" fn wvsprintfA(param0 : windows_sys::core::PSTR, param1 : windows_sys::core::PCSTR, arglist : super::va_list) -> i32);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("user32.dll" "system" fn wvsprintfW(param0 : windows_sys::core::PWSTR, param1 : windows_sys::core::PCWSTR, arglist : *const i8) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
windows_link::link!("user32.dll" "system" fn wvsprintfW(param0 : windows_sys::core::PWSTR, param1 : windows_sys::core::PCWSTR, arglist : super::va_list) -> i32);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ACCEL {
    pub fVirt: u8,
    pub key: u16,
    pub cmd: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ACCESSTIMEOUT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iTimeOutMSec: u32,
}
pub const ALERT_SYSTEM_CRITICAL: u32 = 5;
pub const ALERT_SYSTEM_ERROR: u32 = 3;
pub const ALERT_SYSTEM_INFORMATIONAL: u32 = 1;
pub const ALERT_SYSTEM_QUERY: u32 = 4;
pub const ALERT_SYSTEM_WARNING: u32 = 2;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct ALTTABINFO {
    pub cbSize: u32,
    pub cItems: i32,
    pub cColumns: i32,
    pub cRows: i32,
    pub iColFocus: i32,
    pub iRowFocus: i32,
    pub cxItem: i32,
    pub cyItem: i32,
    pub ptStart: super::POINT,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ANIMATIONINFO {
    pub cbSize: u32,
    pub iMinAnimate: i32,
}
pub const APPCOMMAND_BASS_BOOST: u32 = 20;
pub const APPCOMMAND_BASS_DOWN: u32 = 19;
pub const APPCOMMAND_BASS_UP: u32 = 21;
pub const APPCOMMAND_BROWSER_BACKWARD: u32 = 1;
pub const APPCOMMAND_BROWSER_FAVORITES: u32 = 6;
pub const APPCOMMAND_BROWSER_FORWARD: u32 = 2;
pub const APPCOMMAND_BROWSER_HOME: u32 = 7;
pub const APPCOMMAND_BROWSER_REFRESH: u32 = 3;
pub const APPCOMMAND_BROWSER_SEARCH: u32 = 5;
pub const APPCOMMAND_BROWSER_STOP: u32 = 4;
pub const APPCOMMAND_CLOSE: u32 = 31;
pub const APPCOMMAND_COPY: u32 = 36;
pub const APPCOMMAND_CORRECTION_LIST: u32 = 45;
pub const APPCOMMAND_CUT: u32 = 37;
pub const APPCOMMAND_DELETE: u32 = 53;
pub const APPCOMMAND_DICTATE_OR_COMMAND_CONTROL_TOGGLE: u32 = 43;
pub const APPCOMMAND_DWM_FLIP3D: u32 = 54;
pub const APPCOMMAND_FIND: u32 = 28;
pub const APPCOMMAND_FORWARD_MAIL: u32 = 40;
pub const APPCOMMAND_HELP: u32 = 27;
pub const APPCOMMAND_LAUNCH_APP1: u32 = 17;
pub const APPCOMMAND_LAUNCH_APP2: u32 = 18;
pub const APPCOMMAND_LAUNCH_MAIL: u32 = 15;
pub const APPCOMMAND_LAUNCH_MEDIA_SELECT: u32 = 16;
pub const APPCOMMAND_MEDIA_CHANNEL_DOWN: u32 = 52;
pub const APPCOMMAND_MEDIA_CHANNEL_UP: u32 = 51;
pub const APPCOMMAND_MEDIA_FAST_FORWARD: u32 = 49;
pub const APPCOMMAND_MEDIA_NEXTTRACK: u32 = 11;
pub const APPCOMMAND_MEDIA_PAUSE: u32 = 47;
pub const APPCOMMAND_MEDIA_PLAY: u32 = 46;
pub const APPCOMMAND_MEDIA_PLAY_PAUSE: u32 = 14;
pub const APPCOMMAND_MEDIA_PREVIOUSTRACK: u32 = 12;
pub const APPCOMMAND_MEDIA_RECORD: u32 = 48;
pub const APPCOMMAND_MEDIA_REWIND: u32 = 50;
pub const APPCOMMAND_MEDIA_STOP: u32 = 13;
pub const APPCOMMAND_MICROPHONE_VOLUME_DOWN: u32 = 25;
pub const APPCOMMAND_MICROPHONE_VOLUME_MUTE: u32 = 24;
pub const APPCOMMAND_MICROPHONE_VOLUME_UP: u32 = 26;
pub const APPCOMMAND_MIC_ON_OFF_TOGGLE: u32 = 44;
pub const APPCOMMAND_NEW: u32 = 29;
pub const APPCOMMAND_OPEN: u32 = 30;
pub const APPCOMMAND_PASTE: u32 = 38;
pub const APPCOMMAND_PRINT: u32 = 33;
pub const APPCOMMAND_REDO: u32 = 35;
pub const APPCOMMAND_REPLY_TO_MAIL: u32 = 39;
pub const APPCOMMAND_SAVE: u32 = 32;
pub const APPCOMMAND_SEND_MAIL: u32 = 41;
pub const APPCOMMAND_SPELL_CHECK: u32 = 42;
pub const APPCOMMAND_TREBLE_DOWN: u32 = 22;
pub const APPCOMMAND_TREBLE_UP: u32 = 23;
pub const APPCOMMAND_UNDO: u32 = 34;
pub const APPCOMMAND_VOLUME_DOWN: u32 = 9;
pub const APPCOMMAND_VOLUME_MUTE: u32 = 8;
pub const APPCOMMAND_VOLUME_UP: u32 = 10;
pub const ARW_BOTTOMLEFT: u32 = 0;
pub const ARW_BOTTOMRIGHT: u32 = 1;
pub const ARW_DOWN: u32 = 4;
pub const ARW_HIDE: u32 = 8;
pub const ARW_LEFT: u32 = 0;
pub const ARW_RIGHT: u32 = 0;
pub const ARW_STARTMASK: u32 = 3;
pub const ARW_STARTRIGHT: u32 = 1;
pub const ARW_STARTTOP: u32 = 2;
pub const ARW_TOPLEFT: u32 = 2;
pub const ARW_TOPRIGHT: u32 = 3;
pub const ARW_UP: u32 = 4;
pub const AR_DISABLED: AR_STATE = 1;
pub const AR_DOCKED: AR_STATE = 64;
pub const AR_ENABLED: AR_STATE = 0;
pub const AR_LAPTOP: AR_STATE = 128;
pub const AR_MULTIMON: AR_STATE = 8;
pub const AR_NOSENSOR: AR_STATE = 16;
pub const AR_NOT_SUPPORTED: AR_STATE = 32;
pub const AR_REMOTESESSION: AR_STATE = 4;
pub type AR_STATE = u32;
pub const AR_SUPPRESSED: AR_STATE = 2;
pub const ASFW_ANY: u32 = 4294967295;
pub const ATF_ONOFFFEEDBACK: u32 = 2;
pub const ATF_TIMEOUTON: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct AUDIODESCRIPTION {
    pub cbSize: u32,
    pub Enabled: windows_sys::core::BOOL,
    pub Locale: super::LCID,
}
pub const AW_ACTIVATE: u32 = 131072;
pub const AW_BLEND: u32 = 524288;
pub const AW_CENTER: u32 = 16;
pub const AW_HIDE: u32 = 65536;
pub const AW_HOR_NEGATIVE: u32 = 2;
pub const AW_HOR_POSITIVE: u32 = 1;
pub const AW_SLIDE: u32 = 262144;
pub const AW_VER_NEGATIVE: u32 = 8;
pub const AW_VER_POSITIVE: u32 = 4;
pub const BDR_INNER: u32 = 12;
pub const BDR_OUTER: u32 = 3;
pub const BDR_RAISED: u32 = 5;
pub const BDR_RAISEDINNER: u32 = 4;
pub const BDR_RAISEDOUTER: u32 = 1;
pub const BDR_SUNKEN: u32 = 10;
pub const BDR_SUNKENINNER: u32 = 8;
pub const BDR_SUNKENOUTER: u32 = 2;
pub const BF_ADJUST: u32 = 8192;
pub const BF_BOTTOM: u32 = 8;
pub const BF_BOTTOMLEFT: u32 = 9;
pub const BF_BOTTOMRIGHT: u32 = 12;
pub const BF_DIAGONAL: u32 = 16;
pub const BF_DIAGONAL_ENDBOTTOMLEFT: u32 = 25;
pub const BF_DIAGONAL_ENDBOTTOMRIGHT: u32 = 28;
pub const BF_DIAGONAL_ENDTOPLEFT: u32 = 19;
pub const BF_DIAGONAL_ENDTOPRIGHT: u32 = 22;
pub const BF_FLAT: u32 = 16384;
pub const BF_LEFT: u32 = 1;
pub const BF_MIDDLE: u32 = 2048;
pub const BF_MONO: u32 = 32768;
pub const BF_RECT: u32 = 15;
pub const BF_RIGHT: u32 = 4;
pub const BF_SOFT: u32 = 4096;
pub const BF_TOP: u32 = 2;
pub const BF_TOPLEFT: u32 = 3;
pub const BF_TOPRIGHT: u32 = 6;
pub const BM_CLICK: u32 = 245;
pub const BM_GETCHECK: u32 = 240;
pub const BM_GETIMAGE: u32 = 246;
pub const BM_GETSTATE: u32 = 242;
pub const BM_SETCHECK: u32 = 241;
pub const BM_SETDONTCLICK: u32 = 248;
pub const BM_SETIMAGE: u32 = 247;
pub const BM_SETSTATE: u32 = 243;
pub const BM_SETSTYLE: u32 = 244;
pub const BN_CLICKED: u32 = 0;
pub const BN_DBLCLK: u32 = 5;
pub const BN_DISABLE: u32 = 4;
pub const BN_DOUBLECLICKED: u32 = 5;
pub const BN_HILITE: u32 = 2;
pub const BN_KILLFOCUS: u32 = 7;
pub const BN_PAINT: u32 = 1;
pub const BN_PUSHED: u32 = 2;
pub const BN_SETFOCUS: u32 = 6;
pub const BN_UNHILITE: u32 = 3;
pub const BN_UNPUSHED: u32 = 3;
pub const BROADCAST_QUERY_DENY: u32 = 1112363332;
pub const BSF_ALLOWSFW: u32 = 128;
pub const BSF_FLUSHDISK: u32 = 4;
pub const BSF_FORCEIFHUNG: u32 = 32;
pub const BSF_IGNORECURRENTTASK: u32 = 2;
pub const BSF_LUID: u32 = 1024;
pub const BSF_NOHANG: u32 = 8;
pub const BSF_NOTIMEOUTIFNOTHUNG: u32 = 64;
pub const BSF_POSTMESSAGE: u32 = 16;
pub const BSF_QUERY: u32 = 1;
pub const BSF_RETURNHDESK: u32 = 512;
pub const BSF_SENDNOTIFYMESSAGE: u32 = 256;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct BSMINFO {
    pub cbSize: u32,
    pub hdesk: super::HDESK,
    pub hwnd: super::HWND,
    pub luid: super::LUID,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for BSMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BSM_ALLCOMPONENTS: u32 = 0;
pub const BSM_ALLDESKTOPS: u32 = 16;
pub const BSM_APPLICATIONS: u32 = 8;
pub const BSM_INSTALLABLEDRIVERS: u32 = 4;
pub const BSM_NETDRIVER: u32 = 2;
pub const BSM_VXDS: u32 = 1;
pub const BST_CHECKED: u32 = 1;
pub const BST_FOCUS: u32 = 8;
pub const BST_INDETERMINATE: u32 = 2;
pub const BST_PUSHED: u32 = 4;
pub const BST_UNCHECKED: u32 = 0;
pub const BS_3STATE: u32 = 5;
pub const BS_AUTO3STATE: u32 = 6;
pub const BS_AUTOCHECKBOX: u32 = 3;
pub const BS_AUTORADIOBUTTON: u32 = 9;
pub const BS_BITMAP: u32 = 128;
pub const BS_BOTTOM: u32 = 2048;
pub const BS_CENTER: u32 = 768;
pub const BS_CHECKBOX: u32 = 2;
pub const BS_DEFPUSHBUTTON: u32 = 1;
pub const BS_FLAT: u32 = 32768;
pub const BS_GROUPBOX: u32 = 7;
pub const BS_ICON: u32 = 64;
pub const BS_LEFT: u32 = 256;
pub const BS_LEFTTEXT: u32 = 32;
pub const BS_MULTILINE: u32 = 8192;
pub const BS_NOTIFY: u32 = 16384;
pub const BS_OWNERDRAW: u32 = 11;
pub const BS_PUSHBOX: u32 = 10;
pub const BS_PUSHBUTTON: u32 = 0;
pub const BS_PUSHLIKE: u32 = 4096;
pub const BS_RADIOBUTTON: u32 = 4;
pub const BS_RIGHT: u32 = 512;
pub const BS_RIGHTBUTTON: u32 = 32;
pub const BS_TEXT: u32 = 0;
pub const BS_TOP: u32 = 1024;
pub const BS_TYPEMASK: u32 = 15;
pub const BS_USERBUTTON: u32 = 8;
pub const BS_VCENTER: u32 = 3072;
pub const CALERT_SYSTEM: u32 = 6;
pub const CBN_CLOSEUP: u32 = 8;
pub const CBN_DBLCLK: u32 = 2;
pub const CBN_DROPDOWN: u32 = 7;
pub const CBN_EDITCHANGE: u32 = 5;
pub const CBN_EDITUPDATE: u32 = 6;
pub const CBN_ERRSPACE: i32 = -1;
pub const CBN_KILLFOCUS: u32 = 4;
pub const CBN_SELCHANGE: u32 = 1;
pub const CBN_SELENDCANCEL: u32 = 10;
pub const CBN_SELENDOK: u32 = 9;
pub const CBN_SETFOCUS: u32 = 3;
pub const CBS_AUTOHSCROLL: u32 = 64;
pub const CBS_DISABLENOSCROLL: u32 = 2048;
pub const CBS_DROPDOWN: u32 = 2;
pub const CBS_DROPDOWNLIST: u32 = 3;
pub const CBS_HASSTRINGS: u32 = 512;
pub const CBS_LOWERCASE: u32 = 16384;
pub const CBS_NOINTEGRALHEIGHT: u32 = 1024;
pub const CBS_OEMCONVERT: u32 = 128;
pub const CBS_OWNERDRAWFIXED: u32 = 16;
pub const CBS_OWNERDRAWVARIABLE: u32 = 32;
pub const CBS_SIMPLE: u32 = 1;
pub const CBS_SORT: u32 = 256;
pub const CBS_UPPERCASE: u32 = 8192;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct CBTACTIVATESTRUCT {
    pub fMouse: windows_sys::core::BOOL,
    pub hWndActive: super::HWND,
}
#[cfg(feature = "windef")]
impl Default for CBTACTIVATESTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CBT_CREATEWND = CBT_CREATEWNDA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct CBT_CREATEWNDA {
    pub lpcs: *mut CREATESTRUCTA,
    pub hwndInsertAfter: super::HWND,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for CBT_CREATEWNDA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct CBT_CREATEWNDW {
    pub lpcs: *mut CREATESTRUCTW,
    pub hwndInsertAfter: super::HWND,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for CBT_CREATEWNDW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CB_ADDSTRING: u32 = 323;
pub const CB_DELETESTRING: u32 = 324;
pub const CB_DIR: u32 = 325;
pub const CB_ERR: i32 = -1;
pub const CB_ERRSPACE: i32 = -2;
pub const CB_FINDSTRING: u32 = 332;
pub const CB_FINDSTRINGEXACT: u32 = 344;
pub const CB_GETCOMBOBOXINFO: u32 = 356;
pub const CB_GETCOUNT: u32 = 326;
pub const CB_GETCURSEL: u32 = 327;
pub const CB_GETDROPPEDCONTROLRECT: u32 = 338;
pub const CB_GETDROPPEDSTATE: u32 = 343;
pub const CB_GETDROPPEDWIDTH: u32 = 351;
pub const CB_GETEDITSEL: u32 = 320;
pub const CB_GETEXTENDEDUI: u32 = 342;
pub const CB_GETHORIZONTALEXTENT: u32 = 349;
pub const CB_GETITEMDATA: u32 = 336;
pub const CB_GETITEMHEIGHT: u32 = 340;
pub const CB_GETLBTEXT: u32 = 328;
pub const CB_GETLBTEXTLEN: u32 = 329;
pub const CB_GETLOCALE: u32 = 346;
pub const CB_GETTOPINDEX: u32 = 347;
pub const CB_INITSTORAGE: u32 = 353;
pub const CB_INSERTSTRING: u32 = 330;
pub const CB_LIMITTEXT: u32 = 321;
pub const CB_MSGMAX: u32 = 357;
pub const CB_OKAY: u32 = 0;
pub const CB_RESETCONTENT: u32 = 331;
pub const CB_SELECTSTRING: u32 = 333;
pub const CB_SETCURSEL: u32 = 334;
pub const CB_SETDROPPEDWIDTH: u32 = 352;
pub const CB_SETEDITSEL: u32 = 322;
pub const CB_SETEXTENDEDUI: u32 = 341;
pub const CB_SETHORIZONTALEXTENT: u32 = 350;
pub const CB_SETITEMDATA: u32 = 337;
pub const CB_SETITEMHEIGHT: u32 = 339;
pub const CB_SETLOCALE: u32 = 345;
pub const CB_SETTOPINDEX: u32 = 348;
pub const CB_SHOWDROPDOWN: u32 = 335;
pub const CCHILDREN_SCROLLBAR: u32 = 5;
pub const CCHILDREN_TITLEBAR: u32 = 5;
pub const CDS_DISABLE_UNSAFE_MODES: u32 = 512;
pub const CDS_ENABLE_UNSAFE_MODES: u32 = 256;
pub const CDS_FULLSCREEN: u32 = 4;
pub const CDS_GLOBAL: u32 = 8;
pub const CDS_NORESET: u32 = 268435456;
pub const CDS_RESET: u32 = 1073741824;
pub const CDS_RESET_EX: u32 = 536870912;
pub const CDS_SET_PRIMARY: u32 = 16;
pub const CDS_TEST: u32 = 2;
pub const CDS_UPDATEREGISTRY: u32 = 1;
pub const CDS_VIDEOPARAMETERS: u32 = 32;
pub const CF_BITMAP: u32 = 2;
pub const CF_DIB: u32 = 8;
pub const CF_DIBV5: u32 = 17;
pub const CF_DIF: u32 = 5;
pub const CF_DSPBITMAP: u32 = 130;
pub const CF_DSPENHMETAFILE: u32 = 142;
pub const CF_DSPMETAFILEPICT: u32 = 131;
pub const CF_DSPTEXT: u32 = 129;
pub const CF_ENHMETAFILE: u32 = 14;
pub const CF_GDIOBJFIRST: u32 = 768;
pub const CF_GDIOBJLAST: u32 = 1023;
pub const CF_HDROP: u32 = 15;
pub const CF_LOCALE: u32 = 16;
pub const CF_MAX: u32 = 18;
pub const CF_METAFILEPICT: u32 = 3;
pub const CF_OEMTEXT: u32 = 7;
pub const CF_OWNERDISPLAY: u32 = 128;
pub const CF_PALETTE: u32 = 9;
pub const CF_PENDATA: u32 = 10;
pub const CF_PRIVATEFIRST: u32 = 512;
pub const CF_PRIVATELAST: u32 = 767;
pub const CF_RIFF: u32 = 11;
pub const CF_SYLK: u32 = 4;
pub const CF_TEXT: u32 = 1;
pub const CF_TIFF: u32 = 6;
pub const CF_UNICODETEXT: u32 = 13;
pub const CF_WAVE: u32 = 12;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CHANGEFILTERSTRUCT {
    pub cbSize: u32,
    pub ExtStatus: u32,
}
pub const CHILDID_SELF: u32 = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct CLIENTCREATESTRUCT {
    pub hWindowMenu: super::HANDLE,
    pub idFirstChild: u32,
}
#[cfg(feature = "winnt")]
impl Default for CLIENTCREATESTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COLOR_3DDKSHADOW: u32 = 21;
pub const COLOR_3DFACE: u32 = 15;
pub const COLOR_3DHIGHLIGHT: u32 = 20;
pub const COLOR_3DHILIGHT: u32 = 20;
pub const COLOR_3DLIGHT: u32 = 22;
pub const COLOR_3DSHADOW: u32 = 16;
pub const COLOR_ACTIVEBORDER: u32 = 10;
pub const COLOR_ACTIVECAPTION: u32 = 2;
pub const COLOR_APPWORKSPACE: u32 = 12;
pub const COLOR_BACKGROUND: u32 = 1;
pub const COLOR_BTNFACE: u32 = 15;
pub const COLOR_BTNHIGHLIGHT: u32 = 20;
pub const COLOR_BTNHILIGHT: u32 = 20;
pub const COLOR_BTNSHADOW: u32 = 16;
pub const COLOR_BTNTEXT: u32 = 18;
pub const COLOR_CAPTIONTEXT: u32 = 9;
pub const COLOR_DESKTOP: u32 = 1;
pub const COLOR_GRADIENTACTIVECAPTION: u32 = 27;
pub const COLOR_GRADIENTINACTIVECAPTION: u32 = 28;
pub const COLOR_GRAYTEXT: u32 = 17;
pub const COLOR_HIGHLIGHT: u32 = 13;
pub const COLOR_HIGHLIGHTTEXT: u32 = 14;
pub const COLOR_HOTLIGHT: u32 = 26;
pub const COLOR_INACTIVEBORDER: u32 = 11;
pub const COLOR_INACTIVECAPTION: u32 = 3;
pub const COLOR_INACTIVECAPTIONTEXT: u32 = 19;
pub const COLOR_INFOBK: u32 = 24;
pub const COLOR_INFOTEXT: u32 = 23;
pub const COLOR_MENU: u32 = 4;
pub const COLOR_MENUBAR: u32 = 30;
pub const COLOR_MENUHILIGHT: u32 = 29;
pub const COLOR_MENUTEXT: u32 = 7;
pub const COLOR_SCROLLBAR: u32 = 0;
pub const COLOR_WINDOW: u32 = 5;
pub const COLOR_WINDOWFRAME: u32 = 6;
pub const COLOR_WINDOWTEXT: u32 = 8;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct COMBOBOXINFO {
    pub cbSize: u32,
    pub rcItem: super::RECT,
    pub rcButton: super::RECT,
    pub stateButton: u32,
    pub hwndCombo: super::HWND,
    pub hwndItem: super::HWND,
    pub hwndList: super::HWND,
}
#[cfg(feature = "windef")]
impl Default for COMBOBOXINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct COMPAREITEMSTRUCT {
    pub CtlType: u32,
    pub CtlID: u32,
    pub hwndItem: super::HWND,
    pub itemID1: u32,
    pub itemData1: usize,
    pub itemID2: u32,
    pub itemData2: usize,
    pub dwLocaleId: u32,
}
#[cfg(feature = "windef")]
impl Default for COMPAREITEMSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const CONSOLE_APPLICATION_16BIT: u32 = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CONSOLE_APPLICATION_16BIT: u32 = 0;
pub const CONSOLE_CARET_SELECTION: u32 = 1;
pub const CONSOLE_CARET_VISIBLE: u32 = 2;
pub const CONTACTVISUALIZATION_OFF: u32 = 0;
pub const CONTACTVISUALIZATION_ON: u32 = 1;
pub const CONTACTVISUALIZATION_PRESENTATIONMODE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COPYDATASTRUCT {
    pub dwData: usize,
    pub cbData: u32,
    pub lpData: *mut core::ffi::c_void,
}
impl Default for COPYDATASTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CREATEPROCESS_MANIFEST_RESOURCE_ID: windows_sys::core::PCWSTR = 1 as _;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CREATESTRUCT = CREATESTRUCTA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct CREATESTRUCTA {
    pub lpCreateParams: *mut core::ffi::c_void,
    pub hInstance: super::HINSTANCE,
    pub hMenu: super::HMENU,
    pub hwndParent: super::HWND,
    pub cy: i32,
    pub cx: i32,
    pub y: i32,
    pub x: i32,
    pub style: i32,
    pub lpszName: windows_sys::core::PCSTR,
    pub lpszClass: windows_sys::core::PCSTR,
    pub dwExStyle: u32,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for CREATESTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct CREATESTRUCTW {
    pub lpCreateParams: *mut core::ffi::c_void,
    pub hInstance: super::HINSTANCE,
    pub hMenu: super::HMENU,
    pub hwndParent: super::HWND,
    pub cy: i32,
    pub cx: i32,
    pub y: i32,
    pub x: i32,
    pub style: i32,
    pub lpszName: windows_sys::core::PCWSTR,
    pub lpszClass: windows_sys::core::PCWSTR,
    pub dwExStyle: u32,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for CREATESTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CSOUND_SYSTEM: u32 = 16;
pub const CS_BYTEALIGNCLIENT: u32 = 4096;
pub const CS_BYTEALIGNWINDOW: u32 = 8192;
pub const CS_CLASSDC: u32 = 64;
pub const CS_DBLCLKS: u32 = 8;
pub const CS_DROPSHADOW: u32 = 131072;
pub const CS_GLOBALCLASS: u32 = 16384;
pub const CS_HREDRAW: u32 = 2;
pub const CS_IME: u32 = 65536;
pub const CS_NOCLOSE: u32 = 512;
pub const CS_OWNDC: u32 = 32;
pub const CS_PARENTDC: u32 = 128;
pub const CS_SAVEBITS: u32 = 2048;
pub const CS_VREDRAW: u32 = 1;
pub const CTLCOLOR_BTN: u32 = 3;
pub const CTLCOLOR_DLG: u32 = 4;
pub const CTLCOLOR_EDIT: u32 = 1;
pub const CTLCOLOR_LISTBOX: u32 = 2;
pub const CTLCOLOR_MAX: u32 = 7;
pub const CTLCOLOR_MSGBOX: u32 = 0;
pub const CTLCOLOR_SCROLLBAR: u32 = 5;
pub const CTLCOLOR_STATIC: u32 = 6;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct CURSORINFO {
    pub cbSize: u32,
    pub flags: u32,
    pub hCursor: super::HCURSOR,
    pub ptScreenPos: super::POINT,
}
#[cfg(feature = "windef")]
impl Default for CURSORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CURSORSHAPE {
    pub xHotSpot: i32,
    pub yHotSpot: i32,
    pub cx: i32,
    pub cy: i32,
    pub cbWidth: i32,
    pub Planes: u8,
    pub BitsPixel: u8,
}
pub const CURSOR_CREATION_SCALING_DEFAULT: u32 = 2;
pub const CURSOR_CREATION_SCALING_NONE: u32 = 1;
pub const CURSOR_INVISIBLE: u32 = 0;
pub const CURSOR_SHOWING: u32 = 1;
pub const CURSOR_SUPPRESSED: u32 = 2;
pub const CWF_CREATE_ONLY: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct CWPRETSTRUCT {
    pub lResult: super::LRESULT,
    pub lParam: super::LPARAM,
    pub wParam: super::WPARAM,
    pub message: u32,
    pub hwnd: super::HWND,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for CWPRETSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct CWPSTRUCT {
    pub lParam: super::LPARAM,
    pub wParam: super::WPARAM,
    pub message: u32,
    pub hwnd: super::HWND,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for CWPSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CWP_ALL: u32 = 0;
pub const CWP_SKIPDISABLED: u32 = 2;
pub const CWP_SKIPINVISIBLE: u32 = 1;
pub const CWP_SKIPTRANSPARENT: u32 = 4;
pub const CW_USEDEFAULT: i32 = -2147483648;
pub const DCDC_DEFAULT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 0;
pub const DCDC_DISABLE_FONT_UPDATE: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 1;
pub const DCDC_DISABLE_RELAYOUT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 2;
pub const DCX_CACHE: u32 = 2;
pub const DCX_CLIPCHILDREN: u32 = 8;
pub const DCX_CLIPSIBLINGS: u32 = 16;
pub const DCX_EXCLUDERGN: u32 = 64;
pub const DCX_EXCLUDEUPDATE: u32 = 256;
pub const DCX_INTERSECTRGN: u32 = 128;
pub const DCX_INTERSECTUPDATE: u32 = 512;
pub const DCX_LOCKWINDOWUPDATE: u32 = 1024;
pub const DCX_NORESETATTRS: u32 = 4;
pub const DCX_PARENTCLIP: u32 = 32;
pub const DCX_VALIDATE: u32 = 2097152;
pub const DCX_WINDOW: u32 = 1;
pub const DC_ACTIVE: u32 = 1;
pub const DC_BUTTONS: u32 = 4096;
pub const DC_GRADIENT: u32 = 32;
pub const DC_HASDEFID: u32 = 21323;
pub const DC_ICON: u32 = 4;
pub const DC_INBUTTON: u32 = 16;
pub const DC_SMALLCAP: u32 = 2;
pub const DC_TEXT: u32 = 8;
pub const DDC_DEFAULT: DIALOG_DPI_CHANGE_BEHAVIORS = 0;
pub const DDC_DISABLE_ALL: DIALOG_DPI_CHANGE_BEHAVIORS = 1;
pub const DDC_DISABLE_CONTROL_RELAYOUT: DIALOG_DPI_CHANGE_BEHAVIORS = 4;
pub const DDC_DISABLE_RESIZE: DIALOG_DPI_CHANGE_BEHAVIORS = 2;
pub const DDL_ARCHIVE: u32 = 32;
pub const DDL_DIRECTORY: u32 = 16;
pub const DDL_DRIVES: u32 = 16384;
pub const DDL_EXCLUSIVE: u32 = 32768;
pub const DDL_HIDDEN: u32 = 2;
pub const DDL_POSTMSGS: u32 = 8192;
pub const DDL_READONLY: u32 = 1;
pub const DDL_READWRITE: u32 = 0;
pub const DDL_SYSTEM: u32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct DEBUGHOOKINFO {
    pub idThread: u32,
    pub idThreadInstaller: u32,
    pub lParam: super::LPARAM,
    pub wParam: super::WPARAM,
    pub code: i32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct DELETEITEMSTRUCT {
    pub CtlType: u32,
    pub CtlID: u32,
    pub itemID: u32,
    pub hwndItem: super::HWND,
    pub itemData: usize,
}
#[cfg(feature = "windef")]
impl Default for DELETEITEMSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
pub type DESKTOPENUMPROC = DESKTOPENUMPROCA;
#[cfg(feature = "minwindef")]
pub type DESKTOPENUMPROCA = NAMEENUMPROCA;
#[cfg(feature = "minwindef")]
pub type DESKTOPENUMPROCW = NAMEENUMPROCW;
pub const DESKTOP_CREATEMENU: u32 = 4;
pub const DESKTOP_CREATEWINDOW: u32 = 2;
pub const DESKTOP_ENUMERATE: u32 = 64;
pub const DESKTOP_HOOKCONTROL: u32 = 8;
pub const DESKTOP_JOURNALPLAYBACK: u32 = 32;
pub const DESKTOP_JOURNALRECORD: u32 = 16;
pub const DESKTOP_READOBJECTS: u32 = 1;
pub const DESKTOP_SWITCHDESKTOP: u32 = 256;
pub const DESKTOP_WRITEOBJECTS: u32 = 128;
pub const DEVICE_NOTIFY_ALL_INTERFACE_CLASSES: u32 = 4;
pub const DEVICE_NOTIFY_SERVICE_HANDLE: u32 = 1;
pub const DEVICE_NOTIFY_WINDOW_HANDLE: u32 = 0;
pub const DFCS_ADJUSTRECT: u32 = 8192;
pub const DFCS_BUTTON3STATE: u32 = 8;
pub const DFCS_BUTTONCHECK: u32 = 0;
pub const DFCS_BUTTONPUSH: u32 = 16;
pub const DFCS_BUTTONRADIO: u32 = 4;
pub const DFCS_BUTTONRADIOIMAGE: u32 = 1;
pub const DFCS_BUTTONRADIOMASK: u32 = 2;
pub const DFCS_CAPTIONCLOSE: u32 = 0;
pub const DFCS_CAPTIONHELP: u32 = 4;
pub const DFCS_CAPTIONMAX: u32 = 2;
pub const DFCS_CAPTIONMIN: u32 = 1;
pub const DFCS_CAPTIONRESTORE: u32 = 3;
pub const DFCS_CHECKED: u32 = 1024;
pub const DFCS_FLAT: u32 = 16384;
pub const DFCS_HOT: u32 = 4096;
pub const DFCS_INACTIVE: u32 = 256;
pub const DFCS_MENUARROW: u32 = 0;
pub const DFCS_MENUARROWRIGHT: u32 = 4;
pub const DFCS_MENUBULLET: u32 = 2;
pub const DFCS_MENUCHECK: u32 = 1;
pub const DFCS_MONO: u32 = 32768;
pub const DFCS_PUSHED: u32 = 512;
pub const DFCS_SCROLLCOMBOBOX: u32 = 5;
pub const DFCS_SCROLLDOWN: u32 = 1;
pub const DFCS_SCROLLLEFT: u32 = 2;
pub const DFCS_SCROLLRIGHT: u32 = 3;
pub const DFCS_SCROLLSIZEGRIP: u32 = 8;
pub const DFCS_SCROLLSIZEGRIPRIGHT: u32 = 16;
pub const DFCS_SCROLLUP: u32 = 0;
pub const DFCS_TRANSPARENT: u32 = 2048;
pub const DFC_BUTTON: u32 = 4;
pub const DFC_CAPTION: u32 = 1;
pub const DFC_MENU: u32 = 2;
pub const DFC_POPUPMENU: u32 = 5;
pub const DFC_SCROLL: u32 = 3;
pub const DF_ALLOWOTHERACCOUNTHOOK: u32 = 1;
pub type DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = u32;
pub type DIALOG_DPI_CHANGE_BEHAVIORS = u32;
pub const DIFFERENCE: u32 = 11;
pub const DISP_CHANGE_BADDUALVIEW: i32 = -6;
pub const DISP_CHANGE_BADFLAGS: i32 = -4;
pub const DISP_CHANGE_BADMODE: i32 = -2;
pub const DISP_CHANGE_BADPARAM: i32 = -5;
pub const DISP_CHANGE_FAILED: i32 = -1;
pub const DISP_CHANGE_NOTUPDATED: i32 = -3;
pub const DISP_CHANGE_RESTART: u32 = 1;
pub const DISP_CHANGE_SUCCESSFUL: u32 = 0;
pub const DI_COMPAT: u32 = 4;
pub const DI_DEFAULTSIZE: u32 = 8;
pub const DI_IMAGE: u32 = 2;
pub const DI_MASK: u32 = 1;
pub const DI_NOMIRROR: u32 = 16;
pub const DI_NORMAL: u32 = 3;
pub const DLGC_BUTTON: u32 = 8192;
pub const DLGC_DEFPUSHBUTTON: u32 = 16;
pub const DLGC_HASSETSEL: u32 = 8;
pub const DLGC_RADIOBUTTON: u32 = 64;
pub const DLGC_STATIC: u32 = 256;
pub const DLGC_UNDEFPUSHBUTTON: u32 = 32;
pub const DLGC_WANTALLKEYS: u32 = 4;
pub const DLGC_WANTARROWS: u32 = 1;
pub const DLGC_WANTCHARS: u32 = 128;
pub const DLGC_WANTMESSAGE: u32 = 4;
pub const DLGC_WANTTAB: u32 = 2;
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct DLGITEMTEMPLATE {
    pub style: u32,
    pub dwExtendedStyle: u32,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
    pub id: u16,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type DLGPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: u32, param2: super::WPARAM, param3: super::LPARAM) -> isize>;
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct DLGTEMPLATE {
    pub style: u32,
    pub dwExtendedStyle: u32,
    pub cdit: u16,
    pub x: i16,
    pub y: i16,
    pub cx: i16,
    pub cy: i16,
}
pub const DLGWINDOWEXTRA: u32 = 30;
pub const DM_GETDEFID: u32 = 1024;
pub const DM_POINTERHITTEST: u32 = 592;
pub const DM_REPOSITION: u32 = 1026;
pub const DM_SETDEFID: u32 = 1025;
pub const DOF_DIRECTORY: u32 = 32771;
pub const DOF_DOCUMENT: u32 = 32770;
pub const DOF_EXECUTABLE: u32 = 32769;
pub const DOF_MULTIPLE: u32 = 32772;
pub const DOF_PROGMAN: u32 = 1;
pub const DOF_SHELLDATA: u32 = 2;
pub const DO_DROPFILE: u32 = 1162627398;
pub const DO_PRINTFILE: u32 = 1414419024;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct DRAWITEMSTRUCT {
    pub CtlType: u32,
    pub CtlID: u32,
    pub itemID: u32,
    pub itemAction: u32,
    pub itemState: u32,
    pub hwndItem: super::HWND,
    pub hDC: super::HDC,
    pub rcItem: super::RECT,
    pub itemData: usize,
}
#[cfg(feature = "windef")]
impl Default for DRAWITEMSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type DRAWSTATEPROC = Option<unsafe extern "system" fn(hdc: super::HDC, ldata: super::LPARAM, wdata: super::WPARAM, cx: i32, cy: i32) -> windows_sys::core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DRAWTEXTPARAMS {
    pub cbSize: u32,
    pub iTabLength: i32,
    pub iLeftMargin: i32,
    pub iRightMargin: i32,
    pub uiLengthDrawn: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct DROPSTRUCT {
    pub hwndSource: super::HWND,
    pub hwndSink: super::HWND,
    pub wFmt: u32,
    pub dwData: usize,
    pub ptDrop: super::POINT,
    pub dwControlData: u32,
}
#[cfg(feature = "windef")]
impl Default for DROPSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DSS_DISABLED: u32 = 32;
pub const DSS_HIDEPREFIX: u32 = 512;
pub const DSS_MONO: u32 = 128;
pub const DSS_NORMAL: u32 = 0;
pub const DSS_PREFIXONLY: u32 = 1024;
pub const DSS_RIGHT: u32 = 32768;
pub const DSS_UNION: u32 = 16;
pub const DST_BITMAP: u32 = 4;
pub const DST_COMPLEX: u32 = 0;
pub const DST_ICON: u32 = 3;
pub const DST_PREFIXTEXT: u32 = 2;
pub const DST_TEXT: u32 = 1;
pub const DS_3DLOOK: u32 = 4;
pub const DS_ABSALIGN: u32 = 1;
pub const DS_CENTER: u32 = 2048;
pub const DS_CENTERMOUSE: u32 = 4096;
pub const DS_CONTEXTHELP: u32 = 8192;
pub const DS_CONTROL: u32 = 1024;
pub const DS_FIXEDSYS: u32 = 8;
pub const DS_LOCALEDIT: u32 = 32;
pub const DS_MODALFRAME: u32 = 128;
pub const DS_NOFAILCREATE: u32 = 16;
pub const DS_NOIDLEMSG: u32 = 256;
pub const DS_SETFONT: u32 = 64;
pub const DS_SETFOREGROUND: u32 = 512;
pub const DS_SHELLFONT: u32 = 72;
pub const DS_SYSMODAL: u32 = 2;
pub const DT_BOTTOM: u32 = 8;
pub const DT_CALCRECT: u32 = 1024;
pub const DT_CENTER: u32 = 1;
pub const DT_EDITCONTROL: u32 = 8192;
pub const DT_END_ELLIPSIS: u32 = 32768;
pub const DT_EXPANDTABS: u32 = 64;
pub const DT_EXTERNALLEADING: u32 = 512;
pub const DT_HIDEPREFIX: u32 = 1048576;
pub const DT_INTERNAL: u32 = 4096;
pub const DT_LEFT: u32 = 0;
pub const DT_MODIFYSTRING: u32 = 65536;
pub const DT_NOCLIP: u32 = 256;
pub const DT_NOFULLWIDTHCHARBREAK: u32 = 524288;
pub const DT_NOPREFIX: u32 = 2048;
pub const DT_PATH_ELLIPSIS: u32 = 16384;
pub const DT_PREFIXONLY: u32 = 2097152;
pub const DT_RIGHT: u32 = 2;
pub const DT_RTLREADING: u32 = 131072;
pub const DT_SINGLELINE: u32 = 32;
pub const DT_TABSTOP: u32 = 128;
pub const DT_TOP: u32 = 0;
pub const DT_VCENTER: u32 = 4;
pub const DT_WORDBREAK: u32 = 16;
pub const DT_WORD_ELLIPSIS: u32 = 262144;
pub const DWLP_MSGRESULT: u32 = 0;
pub const DWL_DLGPROC: u32 = 4;
pub const DWL_MSGRESULT: u32 = 0;
pub const DWL_USER: u32 = 8;
pub const EC_LEFTMARGIN: u32 = 1;
pub const EC_RIGHTMARGIN: u32 = 2;
pub const EC_USEFONTINFO: u32 = 65535;
pub const EDD_GET_DEVICE_INTERFACE_NAME: u32 = 1;
pub const EDGE_BUMP: u32 = 9;
pub const EDGE_ETCHED: u32 = 6;
pub const EDGE_RAISED: u32 = 5;
pub const EDGE_SUNKEN: u32 = 10;
pub type EDITWORDBREAKPROC = EDITWORDBREAKPROCA;
pub type EDITWORDBREAKPROCA = Option<unsafe extern "system" fn(lpch: windows_sys::core::PCSTR, ichcurrent: i32, cch: i32, code: i32) -> i32>;
pub type EDITWORDBREAKPROCW = Option<unsafe extern "system" fn(lpch: windows_sys::core::PCWSTR, ichcurrent: i32, cch: i32, code: i32) -> i32>;
pub type EDIT_CONTROL_FEATURE = i32;
pub const EDIT_CONTROL_FEATURE_ENTERPRISE_DATA_PROTECTION_PASTE_SUPPORT: EDIT_CONTROL_FEATURE = 0;
pub const EDIT_CONTROL_FEATURE_PASTE_NOTIFICATIONS: EDIT_CONTROL_FEATURE = 1;
pub const EDS_RAWMODE: u32 = 2;
pub const EDS_ROTATEDMODE: u32 = 4;
pub const EIMES_CANCELCOMPSTRINFOCUS: u32 = 2;
pub const EIMES_COMPLETECOMPSTRKILLFOCUS: u32 = 4;
pub const EIMES_GETCOMPSTRATONCE: u32 = 1;
pub const EMSIS_COMPOSITIONSTRING: u32 = 1;
pub const EM_CANUNDO: u32 = 198;
pub const EM_CHARFROMPOS: u32 = 215;
pub const EM_EMPTYUNDOBUFFER: u32 = 205;
pub const EM_ENABLEFEATURE: u32 = 218;
pub const EM_FMTLINES: u32 = 200;
pub const EM_GETFIRSTVISIBLELINE: u32 = 206;
pub const EM_GETHANDLE: u32 = 189;
pub const EM_GETIMESTATUS: u32 = 217;
pub const EM_GETLIMITTEXT: u32 = 213;
pub const EM_GETLINE: u32 = 196;
pub const EM_GETLINECOUNT: u32 = 186;
pub const EM_GETMARGINS: u32 = 212;
pub const EM_GETMODIFY: u32 = 184;
pub const EM_GETPASSWORDCHAR: u32 = 210;
pub const EM_GETRECT: u32 = 178;
pub const EM_GETSEL: u32 = 176;
pub const EM_GETTHUMB: u32 = 190;
pub const EM_GETWORDBREAKPROC: u32 = 209;
pub const EM_LIMITTEXT: u32 = 197;
pub const EM_LINEFROMCHAR: u32 = 201;
pub const EM_LINEINDEX: u32 = 187;
pub const EM_LINELENGTH: u32 = 193;
pub const EM_LINESCROLL: u32 = 182;
pub const EM_POSFROMCHAR: u32 = 214;
pub const EM_REPLACESEL: u32 = 194;
pub const EM_SCROLL: u32 = 181;
pub const EM_SCROLLCARET: u32 = 183;
pub const EM_SETHANDLE: u32 = 188;
pub const EM_SETIMESTATUS: u32 = 216;
pub const EM_SETLIMITTEXT: u32 = 197;
pub const EM_SETMARGINS: u32 = 211;
pub const EM_SETMODIFY: u32 = 185;
pub const EM_SETPASSWORDCHAR: u32 = 204;
pub const EM_SETREADONLY: u32 = 207;
pub const EM_SETRECT: u32 = 179;
pub const EM_SETRECTNP: u32 = 180;
pub const EM_SETSEL: u32 = 177;
pub const EM_SETTABSTOPS: u32 = 203;
pub const EM_SETWORDBREAKPROC: u32 = 208;
pub const EM_UNDO: u32 = 199;
pub const ENDSESSION_CLOSEAPP: u32 = 1;
pub const ENDSESSION_CRITICAL: u32 = 1073741824;
pub const ENDSESSION_LOGOFF: u32 = 2147483648;
pub const ENUM_CURRENT_SETTINGS: u32 = 4294967295;
pub const ENUM_REGISTRY_SETTINGS: u32 = 4294967294;
pub const EN_AFTER_PASTE: u32 = 2049;
pub const EN_ALIGN_LTR_EC: u32 = 1792;
pub const EN_ALIGN_RTL_EC: u32 = 1793;
pub const EN_BEFORE_PASTE: u32 = 2048;
pub const EN_CHANGE: u32 = 768;
pub const EN_ERRSPACE: u32 = 1280;
pub const EN_HSCROLL: u32 = 1537;
pub const EN_KILLFOCUS: u32 = 512;
pub const EN_MAXTEXT: u32 = 1281;
pub const EN_SETFOCUS: u32 = 256;
pub const EN_UPDATE: u32 = 1024;
pub const EN_VSCROLL: u32 = 1538;
pub const ESB_DISABLE_BOTH: u32 = 3;
pub const ESB_DISABLE_DOWN: u32 = 2;
pub const ESB_DISABLE_LEFT: u32 = 1;
pub const ESB_DISABLE_LTUP: u32 = 1;
pub const ESB_DISABLE_RIGHT: u32 = 2;
pub const ESB_DISABLE_RTDN: u32 = 2;
pub const ESB_DISABLE_UP: u32 = 1;
pub const ESB_ENABLE_BOTH: u32 = 0;
pub const ES_AUTOHSCROLL: u32 = 128;
pub const ES_AUTOVSCROLL: u32 = 64;
pub const ES_CENTER: u32 = 1;
pub const ES_LEFT: u32 = 0;
pub const ES_LOWERCASE: u32 = 16;
pub const ES_MULTILINE: u32 = 4;
pub const ES_NOHIDESEL: u32 = 256;
pub const ES_NUMBER: u32 = 8192;
pub const ES_OEMCONVERT: u32 = 1024;
pub const ES_PASSWORD: u32 = 32;
pub const ES_READONLY: u32 = 2048;
pub const ES_RIGHT: u32 = 2;
pub const ES_UPPERCASE: u32 = 8;
pub const ES_WANTRETURN: u32 = 4096;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EVENTMSG {
    pub message: u32,
    pub paramL: u32,
    pub paramH: u32,
    pub time: u32,
    pub hwnd: super::HWND,
}
#[cfg(feature = "windef")]
impl Default for EVENTMSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVENT_AIA_END: u32 = 45055;
pub const EVENT_AIA_START: u32 = 40960;
pub const EVENT_CONSOLE_CARET: u32 = 16385;
pub const EVENT_CONSOLE_END: u32 = 16639;
pub const EVENT_CONSOLE_END_APPLICATION: u32 = 16391;
pub const EVENT_CONSOLE_LAYOUT: u32 = 16389;
pub const EVENT_CONSOLE_START_APPLICATION: u32 = 16390;
pub const EVENT_CONSOLE_UPDATE_REGION: u32 = 16386;
pub const EVENT_CONSOLE_UPDATE_SCROLL: u32 = 16388;
pub const EVENT_CONSOLE_UPDATE_SIMPLE: u32 = 16387;
pub const EVENT_MAX: u32 = 2147483647;
pub const EVENT_MIN: u32 = 1;
pub const EVENT_OBJECT_ACCELERATORCHANGE: u32 = 32786;
pub const EVENT_OBJECT_CLOAKED: u32 = 32791;
pub const EVENT_OBJECT_CONTENTSCROLLED: u32 = 32789;
pub const EVENT_OBJECT_CREATE: u32 = 32768;
pub const EVENT_OBJECT_DEFACTIONCHANGE: u32 = 32785;
pub const EVENT_OBJECT_DESCRIPTIONCHANGE: u32 = 32781;
pub const EVENT_OBJECT_DESTROY: u32 = 32769;
pub const EVENT_OBJECT_DRAGCANCEL: u32 = 32802;
pub const EVENT_OBJECT_DRAGCOMPLETE: u32 = 32803;
pub const EVENT_OBJECT_DRAGDROPPED: u32 = 32806;
pub const EVENT_OBJECT_DRAGENTER: u32 = 32804;
pub const EVENT_OBJECT_DRAGLEAVE: u32 = 32805;
pub const EVENT_OBJECT_DRAGSTART: u32 = 32801;
pub const EVENT_OBJECT_END: u32 = 33023;
pub const EVENT_OBJECT_FOCUS: u32 = 32773;
pub const EVENT_OBJECT_HELPCHANGE: u32 = 32784;
pub const EVENT_OBJECT_HIDE: u32 = 32771;
pub const EVENT_OBJECT_HOSTEDOBJECTSINVALIDATED: u32 = 32800;
pub const EVENT_OBJECT_IME_CHANGE: u32 = 32809;
pub const EVENT_OBJECT_IME_HIDE: u32 = 32808;
pub const EVENT_OBJECT_IME_SHOW: u32 = 32807;
pub const EVENT_OBJECT_INVOKED: u32 = 32787;
pub const EVENT_OBJECT_LIVEREGIONCHANGED: u32 = 32793;
pub const EVENT_OBJECT_LOCATIONCHANGE: u32 = 32779;
pub const EVENT_OBJECT_NAMECHANGE: u32 = 32780;
pub const EVENT_OBJECT_PARENTCHANGE: u32 = 32783;
pub const EVENT_OBJECT_REORDER: u32 = 32772;
pub const EVENT_OBJECT_SELECTION: u32 = 32774;
pub const EVENT_OBJECT_SELECTIONADD: u32 = 32775;
pub const EVENT_OBJECT_SELECTIONREMOVE: u32 = 32776;
pub const EVENT_OBJECT_SELECTIONWITHIN: u32 = 32777;
pub const EVENT_OBJECT_SHOW: u32 = 32770;
pub const EVENT_OBJECT_STATECHANGE: u32 = 32778;
pub const EVENT_OBJECT_TEXTEDIT_CONVERSIONTARGETCHANGED: u32 = 32816;
pub const EVENT_OBJECT_TEXTSELECTIONCHANGED: u32 = 32788;
pub const EVENT_OBJECT_UNCLOAKED: u32 = 32792;
pub const EVENT_OBJECT_VALUECHANGE: u32 = 32782;
pub const EVENT_OEM_DEFINED_END: u32 = 511;
pub const EVENT_OEM_DEFINED_START: u32 = 257;
pub const EVENT_SYSTEM_ALERT: u32 = 2;
pub const EVENT_SYSTEM_ARRANGMENTPREVIEW: u32 = 32790;
pub const EVENT_SYSTEM_CAPTUREEND: u32 = 9;
pub const EVENT_SYSTEM_CAPTURESTART: u32 = 8;
pub const EVENT_SYSTEM_CONTEXTHELPEND: u32 = 13;
pub const EVENT_SYSTEM_CONTEXTHELPSTART: u32 = 12;
pub const EVENT_SYSTEM_DESKTOPSWITCH: u32 = 32;
pub const EVENT_SYSTEM_DIALOGEND: u32 = 17;
pub const EVENT_SYSTEM_DIALOGSTART: u32 = 16;
pub const EVENT_SYSTEM_DRAGDROPEND: u32 = 15;
pub const EVENT_SYSTEM_DRAGDROPSTART: u32 = 14;
pub const EVENT_SYSTEM_END: u32 = 255;
pub const EVENT_SYSTEM_FOREGROUND: u32 = 3;
pub const EVENT_SYSTEM_IME_KEY_NOTIFICATION: u32 = 41;
pub const EVENT_SYSTEM_MENUEND: u32 = 5;
pub const EVENT_SYSTEM_MENUPOPUPEND: u32 = 7;
pub const EVENT_SYSTEM_MENUPOPUPSTART: u32 = 6;
pub const EVENT_SYSTEM_MENUSTART: u32 = 4;
pub const EVENT_SYSTEM_MINIMIZEEND: u32 = 23;
pub const EVENT_SYSTEM_MINIMIZESTART: u32 = 22;
pub const EVENT_SYSTEM_MOVESIZEEND: u32 = 11;
pub const EVENT_SYSTEM_MOVESIZESTART: u32 = 10;
pub const EVENT_SYSTEM_SCROLLINGEND: u32 = 19;
pub const EVENT_SYSTEM_SCROLLINGSTART: u32 = 18;
pub const EVENT_SYSTEM_SOUND: u32 = 1;
pub const EVENT_SYSTEM_SWITCHEND: u32 = 21;
pub const EVENT_SYSTEM_SWITCHER_APPDROPPED: u32 = 38;
pub const EVENT_SYSTEM_SWITCHER_APPGRABBED: u32 = 36;
pub const EVENT_SYSTEM_SWITCHER_APPOVERTARGET: u32 = 37;
pub const EVENT_SYSTEM_SWITCHER_CANCELLED: u32 = 39;
pub const EVENT_SYSTEM_SWITCHSTART: u32 = 20;
pub const EVENT_UIA_EVENTID_END: u32 = 20223;
pub const EVENT_UIA_EVENTID_START: u32 = 19968;
pub const EVENT_UIA_PROPID_END: u32 = 30207;
pub const EVENT_UIA_PROPID_START: u32 = 29952;
pub const EWX_ARSO: u32 = 67108864;
pub const EWX_BOOTOPTIONS: u32 = 16777216;
pub const EWX_CHECK_SAFE_FOR_SERVER: u32 = 134217728;
pub const EWX_FORCE: u32 = 4;
pub const EWX_FORCEIFHUNG: u32 = 16;
pub const EWX_HYBRID_SHUTDOWN: u32 = 4194304;
pub const EWX_LOGOFF: u32 = 0;
pub const EWX_POWEROFF: u32 = 8;
pub const EWX_QUICKRESOLVE: u32 = 32;
pub const EWX_REBOOT: u32 = 2;
pub const EWX_RESTARTAPPS: u32 = 64;
pub const EWX_SHUTDOWN: u32 = 1;
pub const EWX_SYSTEM_INITIATED: u32 = 268435456;
pub const FALT: u32 = 16;
pub const FAPPCOMMAND_KEY: u32 = 0;
pub const FAPPCOMMAND_MASK: u32 = 61440;
pub const FAPPCOMMAND_MOUSE: u32 = 32768;
pub const FAPPCOMMAND_OEM: u32 = 4096;
pub const FCONTROL: u32 = 8;
pub const FEEDBACK_GESTURE_PRESSANDTAP: FEEDBACK_TYPE = 11;
pub const FEEDBACK_MAX: FEEDBACK_TYPE = -1;
pub const FEEDBACK_PEN_BARRELVISUALIZATION: FEEDBACK_TYPE = 2;
pub const FEEDBACK_PEN_DOUBLETAP: FEEDBACK_TYPE = 4;
pub const FEEDBACK_PEN_PRESSANDHOLD: FEEDBACK_TYPE = 5;
pub const FEEDBACK_PEN_RIGHTTAP: FEEDBACK_TYPE = 6;
pub const FEEDBACK_PEN_TAP: FEEDBACK_TYPE = 3;
pub const FEEDBACK_TOUCH_CONTACTVISUALIZATION: FEEDBACK_TYPE = 1;
pub const FEEDBACK_TOUCH_DOUBLETAP: FEEDBACK_TYPE = 8;
pub const FEEDBACK_TOUCH_PRESSANDHOLD: FEEDBACK_TYPE = 9;
pub const FEEDBACK_TOUCH_RIGHTTAP: FEEDBACK_TYPE = 10;
pub const FEEDBACK_TOUCH_TAP: FEEDBACK_TYPE = 7;
pub type FEEDBACK_TYPE = i32;
pub const FE_FONTSMOOTHINGCLEARTYPE: u32 = 2;
pub const FE_FONTSMOOTHINGORIENTATIONBGR: u32 = 0;
pub const FE_FONTSMOOTHINGORIENTATIONRGB: u32 = 1;
pub const FE_FONTSMOOTHINGSTANDARD: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILTERKEYS {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iWaitMSec: u32,
    pub iDelayMSec: u32,
    pub iRepeatMSec: u32,
    pub iBounceMSec: u32,
}
pub const FKF_AVAILABLE: u32 = 2;
pub const FKF_CLICKON: u32 = 64;
pub const FKF_CONFIRMHOTKEY: u32 = 8;
pub const FKF_FILTERKEYSON: u32 = 1;
pub const FKF_HOTKEYACTIVE: u32 = 4;
pub const FKF_HOTKEYSOUND: u32 = 16;
pub const FKF_INDICATOR: u32 = 32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct FLASHWINFO {
    pub cbSize: u32,
    pub hwnd: super::HWND,
    pub dwFlags: u32,
    pub uCount: u32,
    pub dwTimeout: u32,
}
#[cfg(feature = "windef")]
impl Default for FLASHWINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FLASHW_ALL: u32 = 3;
pub const FLASHW_CAPTION: u32 = 1;
pub const FLASHW_STOP: u32 = 0;
pub const FLASHW_TIMER: u32 = 4;
pub const FLASHW_TIMERNOFG: u32 = 12;
pub const FLASHW_TRAY: u32 = 2;
pub const FNOINVERT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FRAME_MARGIN {
    pub left: i16,
    pub right: i16,
    pub top: i16,
    pub bottom: i16,
}
pub const FSHIFT: u32 = 4;
pub const FVIRTKEY: u32 = 1;
pub const GA_PARENT: u32 = 1;
pub const GA_ROOT: u32 = 2;
pub const GA_ROOTOWNER: u32 = 3;
pub const GCF_INCLUDE_ANCESTORS: u32 = 1;
pub const GCLP_HBRBACKGROUND: i32 = -10;
pub const GCLP_HCURSOR: i32 = -12;
pub const GCLP_HICON: i32 = -14;
pub const GCLP_HICONSM: i32 = -34;
pub const GCLP_HMODULE: i32 = -16;
pub const GCLP_MENUNAME: i32 = -8;
pub const GCLP_WNDPROC: i32 = -24;
pub const GCL_CBCLSEXTRA: i32 = -20;
pub const GCL_CBWNDEXTRA: i32 = -18;
#[cfg(target_arch = "x86")]
pub const GCL_HBRBACKGROUND: i32 = -10;
#[cfg(target_arch = "x86")]
pub const GCL_HCURSOR: i32 = -12;
#[cfg(target_arch = "x86")]
pub const GCL_HICON: i32 = -14;
#[cfg(target_arch = "x86")]
pub const GCL_HICONSM: i32 = -34;
#[cfg(target_arch = "x86")]
pub const GCL_HMODULE: i32 = -16;
#[cfg(target_arch = "x86")]
pub const GCL_MENUNAME: i32 = -8;
pub const GCL_STYLE: i32 = -26;
#[cfg(target_arch = "x86")]
pub const GCL_WNDPROC: i32 = -24;
pub const GCW_ATOM: i32 = -32;
pub const GC_ALLGESTURES: u32 = 1;
pub const GC_PAN: u32 = 1;
pub const GC_PAN_WITH_GUTTER: u32 = 8;
pub const GC_PAN_WITH_INERTIA: u32 = 16;
pub const GC_PAN_WITH_SINGLE_FINGER_HORIZONTALLY: u32 = 4;
pub const GC_PAN_WITH_SINGLE_FINGER_VERTICALLY: u32 = 2;
pub const GC_PRESSANDTAP: u32 = 1;
pub const GC_ROLLOVER: u32 = 1;
pub const GC_ROTATE: u32 = 1;
pub const GC_TWOFINGERTAP: u32 = 1;
pub const GC_ZOOM: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GESTURECONFIG {
    pub dwID: u32,
    pub dwWant: u32,
    pub dwBlock: u32,
}
pub const GESTURECONFIGMAXCOUNT: u32 = 256;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct GESTUREINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwID: u32,
    pub hwndTarget: super::HWND,
    pub ptsLocation: super::POINTS,
    pub dwInstanceID: u32,
    pub dwSequenceID: u32,
    pub ullArguments: u64,
    pub cbExtraArgs: u32,
}
#[cfg(feature = "windef")]
impl Default for GESTUREINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct GESTURENOTIFYSTRUCT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTarget: super::HWND,
    pub ptsLocation: super::POINTS,
    pub dwInstanceID: u32,
}
#[cfg(feature = "windef")]
impl Default for GESTURENOTIFYSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GESTUREVISUALIZATION_DOUBLETAP: u32 = 2;
pub const GESTUREVISUALIZATION_OFF: u32 = 0;
pub const GESTUREVISUALIZATION_ON: u32 = 31;
pub const GESTUREVISUALIZATION_PRESSANDHOLD: u32 = 8;
pub const GESTUREVISUALIZATION_PRESSANDTAP: u32 = 4;
pub const GESTUREVISUALIZATION_RIGHTTAP: u32 = 16;
pub const GESTUREVISUALIZATION_TAP: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GETCLIPBMETADATA {
    pub Version: u32,
    pub IsDelayRendered: windows_sys::core::BOOL,
    pub IsSynthetic: windows_sys::core::BOOL,
}
pub const GF_BEGIN: u32 = 1;
pub const GF_END: u32 = 4;
pub const GF_INERTIA: u32 = 2;
pub const GIDC_ARRIVAL: u32 = 1;
pub const GIDC_REMOVAL: u32 = 2;
pub const GID_BEGIN: u32 = 1;
pub const GID_END: u32 = 2;
pub const GID_PAN: u32 = 4;
pub const GID_PRESSANDTAP: u32 = 7;
pub const GID_ROLLOVER: u32 = 7;
pub const GID_ROTATE: u32 = 5;
pub const GID_TWOFINGERTAP: u32 = 6;
pub const GID_ZOOM: u32 = 3;
pub const GMDI_GOINTOPOPUPS: u32 = 2;
pub const GMDI_USEDISABLED: u32 = 1;
pub const GMMP_USE_DISPLAY_POINTS: u32 = 1;
pub const GMMP_USE_HIGH_RESOLUTION_POINTS: u32 = 2;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type GRAYSTRINGPROC = Option<unsafe extern "system" fn(param0: super::HDC, param1: super::LPARAM, param2: i32) -> windows_sys::core::BOOL>;
pub const GR_GDIOBJECTS: u32 = 0;
pub const GR_GDIOBJECTS_PEAK: u32 = 2;
#[cfg(feature = "winnt")]
pub const GR_GLOBAL: super::HANDLE = -2 as _;
pub const GR_USEROBJECTS: u32 = 1;
pub const GR_USEROBJECTS_PEAK: u32 = 4;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct GUITHREADINFO {
    pub cbSize: u32,
    pub flags: u32,
    pub hwndActive: super::HWND,
    pub hwndFocus: super::HWND,
    pub hwndCapture: super::HWND,
    pub hwndMenuOwner: super::HWND,
    pub hwndMoveSize: super::HWND,
    pub hwndCaret: super::HWND,
    pub rcCaret: super::RECT,
}
#[cfg(feature = "windef")]
impl Default for GUITHREADINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const GUI_16BITTASK: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const GUI_16BITTASK: u32 = 0;
pub const GUI_CARETBLINKING: u32 = 1;
pub const GUI_INMENUMODE: u32 = 4;
pub const GUI_INMOVESIZE: u32 = 2;
pub const GUI_POPUPMENUMODE: u32 = 16;
pub const GUI_SYSTEMMENUMODE: u32 = 8;
pub const GWFS_INCLUDE_ANCESTORS: u32 = 1;
pub const GWLP_HINSTANCE: i32 = -6;
pub const GWLP_HWNDPARENT: i32 = -8;
pub const GWLP_ID: i32 = -12;
pub const GWLP_USERDATA: i32 = -21;
pub const GWLP_WNDPROC: i32 = -4;
pub const GWL_EXSTYLE: i32 = -20;
#[cfg(target_arch = "x86")]
pub const GWL_HINSTANCE: i32 = -6;
#[cfg(target_arch = "x86")]
pub const GWL_HWNDPARENT: i32 = -8;
pub const GWL_ID: i32 = -12;
pub const GWL_STYLE: i32 = -16;
#[cfg(target_arch = "x86")]
pub const GWL_USERDATA: i32 = -21;
#[cfg(target_arch = "x86")]
pub const GWL_WNDPROC: i32 = -4;
pub const GW_CHILD: u32 = 5;
pub const GW_ENABLEDPOPUP: u32 = 6;
pub const GW_HWNDFIRST: u32 = 0;
pub const GW_HWNDLAST: u32 = 1;
pub const GW_HWNDNEXT: u32 = 2;
pub const GW_HWNDPREV: u32 = 3;
pub const GW_MAX: u32 = 6;
pub const GW_OWNER: u32 = 4;
pub type HANDEDNESS = i32;
pub const HANDEDNESS_LEFT: HANDEDNESS = 0;
pub const HANDEDNESS_RIGHT: HANDEDNESS = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct HARDWAREHOOKSTRUCT {
    pub hwnd: super::HWND,
    pub message: u32,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for HARDWAREHOOKSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HARDWAREINPUT {
    pub uMsg: u32,
    pub wParamL: u16,
    pub wParamH: u16,
}
#[cfg(feature = "windef")]
pub const HBMMENU_CALLBACK: super::HBITMAP = -1 as _;
#[cfg(feature = "windef")]
pub const HBMMENU_MBAR_CLOSE: super::HBITMAP = 5 as _;
#[cfg(feature = "windef")]
pub const HBMMENU_MBAR_CLOSE_D: super::HBITMAP = 6 as _;
#[cfg(feature = "windef")]
pub const HBMMENU_MBAR_MINIMIZE: super::HBITMAP = 3 as _;
#[cfg(feature = "windef")]
pub const HBMMENU_MBAR_MINIMIZE_D: super::HBITMAP = 7 as _;
#[cfg(feature = "windef")]
pub const HBMMENU_MBAR_RESTORE: super::HBITMAP = 2 as _;
#[cfg(feature = "windef")]
pub const HBMMENU_POPUP_CLOSE: super::HBITMAP = 8 as _;
#[cfg(feature = "windef")]
pub const HBMMENU_POPUP_MAXIMIZE: super::HBITMAP = 10 as _;
#[cfg(feature = "windef")]
pub const HBMMENU_POPUP_MINIMIZE: super::HBITMAP = 11 as _;
#[cfg(feature = "windef")]
pub const HBMMENU_POPUP_RESTORE: super::HBITMAP = 9 as _;
#[cfg(feature = "windef")]
pub const HBMMENU_SYSTEM: super::HBITMAP = 1 as _;
pub const HCBT_ACTIVATE: u32 = 5;
pub const HCBT_CLICKSKIPPED: u32 = 6;
pub const HCBT_CREATEWND: u32 = 3;
pub const HCBT_DESTROYWND: u32 = 4;
pub const HCBT_KEYSKIPPED: u32 = 7;
pub const HCBT_MINMAX: u32 = 1;
pub const HCBT_MOVESIZE: u32 = 0;
pub const HCBT_QS: u32 = 2;
pub const HCBT_SETFOCUS: u32 = 9;
pub const HCBT_SYSCOMMAND: u32 = 8;
pub const HCF_AVAILABLE: u32 = 2;
pub const HCF_CONFIRMHOTKEY: u32 = 8;
pub const HCF_DEFAULTDESKTOP: u32 = 512;
pub const HCF_HIGHCONTRASTON: u32 = 1;
pub const HCF_HOTKEYACTIVE: u32 = 4;
pub const HCF_HOTKEYAVAILABLE: u32 = 64;
pub const HCF_HOTKEYSOUND: u32 = 16;
pub const HCF_INDICATOR: u32 = 32;
pub const HCF_LOGONDESKTOP: u32 = 256;
pub const HCF_OPTION_NOTHEMECHANGE: u32 = 4096;
pub const HC_ACTION: u32 = 0;
pub const HC_GETNEXT: u32 = 1;
pub const HC_NOREM: u32 = 3;
pub const HC_NOREMOVE: u32 = 3;
pub const HC_SKIP: u32 = 2;
pub const HC_SYSMODALOFF: u32 = 5;
pub const HC_SYSMODALON: u32 = 4;
pub type HDEVNOTIFY = *mut core::ffi::c_void;
#[cfg(feature = "winnt")]
pub type HDWP = super::HANDLE;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct HELPINFO {
    pub cbSize: u32,
    pub iContextType: i32,
    pub iCtrlId: i32,
    pub hItemHandle: super::HANDLE,
    pub dwContextId: usize,
    pub MousePos: super::POINT,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for HELPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HELPINFO_MENUITEM: u32 = 2;
pub const HELPINFO_WINDOW: u32 = 1;
pub type HELPPOLY = u32;
pub type HELPWININFO = HELPWININFOA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HELPWININFOA {
    pub wStructSize: i32,
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    pub wMax: i32,
    pub rgchMember: [i8; 2],
}
impl Default for HELPWININFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HELPWININFOW {
    pub wStructSize: i32,
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    pub wMax: i32,
    pub rgchMember: [u16; 2],
}
impl Default for HELPWININFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HELP_COMMAND: u32 = 258;
pub const HELP_CONTENTS: u32 = 3;
pub const HELP_CONTEXT: u32 = 1;
pub const HELP_CONTEXTMENU: u32 = 10;
pub const HELP_CONTEXTPOPUP: u32 = 8;
pub const HELP_FINDER: u32 = 11;
pub const HELP_FORCEFILE: u32 = 9;
pub const HELP_HELPONHELP: u32 = 4;
pub const HELP_INDEX: u32 = 3;
pub const HELP_KEY: u32 = 257;
pub const HELP_MULTIKEY: u32 = 513;
pub const HELP_PARTIALKEY: u32 = 261;
pub const HELP_QUIT: u32 = 2;
pub const HELP_SETCONTENTS: u32 = 5;
pub const HELP_SETINDEX: u32 = 5;
pub const HELP_SETPOPUP_POS: u32 = 13;
pub const HELP_SETWINPOS: u32 = 515;
pub const HELP_TCARD: u32 = 32768;
pub const HELP_TCARD_DATA: u32 = 16;
pub const HELP_TCARD_OTHER_CALLER: u32 = 17;
pub const HELP_WM_HELP: u32 = 12;
pub type HGESTUREINFO = *mut core::ffi::c_void;
pub const HIDE_WINDOW: u32 = 0;
pub type HIGHCONTRAST = HIGHCONTRASTA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HIGHCONTRASTA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpszDefaultScheme: windows_sys::core::PSTR,
}
impl Default for HIGHCONTRASTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HIGHCONTRASTW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpszDefaultScheme: windows_sys::core::PWSTR,
}
impl Default for HIGHCONTRASTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HKL_NEXT: u32 = 1;
pub const HKL_PREV: u32 = 0;
#[cfg(feature = "minwindef")]
pub type HOOKPROC = Option<unsafe extern "system" fn(code: i32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT>;
pub const HOVER_DEFAULT: u32 = 4294967295;
pub type HPOWERNOTIFY = *mut core::ffi::c_void;
pub type HRAWINPUT = *mut core::ffi::c_void;
pub const HSHELL_ACCESSIBILITYSTATE: u32 = 11;
pub const HSHELL_ACTIVATESHELLWINDOW: u32 = 3;
pub const HSHELL_APPCOMMAND: u32 = 12;
pub const HSHELL_ENDTASK: u32 = 10;
pub const HSHELL_FLASH: u32 = 32774;
pub const HSHELL_GETMINRECT: u32 = 5;
pub const HSHELL_HIGHBIT: u32 = 32768;
pub const HSHELL_LANGUAGE: u32 = 8;
pub const HSHELL_MONITORCHANGED: u32 = 16;
pub const HSHELL_REDRAW: u32 = 6;
pub const HSHELL_RUDEAPPACTIVATED: u32 = 32772;
pub const HSHELL_SYSMENU: u32 = 9;
pub const HSHELL_TASKMAN: u32 = 7;
pub const HSHELL_WINDOWACTIVATED: u32 = 4;
pub const HSHELL_WINDOWCREATED: u32 = 1;
pub const HSHELL_WINDOWDESTROYED: u32 = 2;
pub const HSHELL_WINDOWREPLACED: u32 = 13;
pub const HSHELL_WINDOWREPLACING: u32 = 14;
pub type HSYNTHETICPOINTERDEVICE = *mut core::ffi::c_void;
pub const HTBORDER: u32 = 18;
pub const HTBOTTOM: u32 = 15;
pub const HTBOTTOMLEFT: u32 = 16;
pub const HTBOTTOMRIGHT: u32 = 17;
pub const HTCAPTION: u32 = 2;
pub const HTCLIENT: u32 = 1;
pub const HTCLOSE: u32 = 20;
pub const HTERROR: i32 = -2;
pub const HTGROWBOX: u32 = 4;
pub const HTHELP: u32 = 21;
pub const HTHSCROLL: u32 = 6;
pub const HTLEFT: u32 = 10;
pub const HTMAXBUTTON: u32 = 9;
pub const HTMENU: u32 = 5;
pub const HTMINBUTTON: u32 = 8;
pub const HTNOWHERE: u32 = 0;
pub const HTOBJECT: u32 = 19;
pub type HTOUCHINPUT = *mut core::ffi::c_void;
pub const HTREDUCE: u32 = 8;
pub const HTRIGHT: u32 = 11;
pub const HTSIZE: u32 = 4;
pub const HTSIZEFIRST: u32 = 10;
pub const HTSIZELAST: u32 = 17;
pub const HTSYSMENU: u32 = 3;
pub const HTTOP: u32 = 12;
pub const HTTOPLEFT: u32 = 13;
pub const HTTOPRIGHT: u32 = 14;
pub const HTTRANSPARENT: i32 = -1;
pub const HTVSCROLL: u32 = 7;
pub const HTZOOM: u32 = 9;
#[cfg(feature = "windef")]
pub const HWND_BOTTOM: super::HWND = 1 as _;
#[cfg(feature = "windef")]
pub const HWND_BROADCAST: super::HWND = 65535 as _;
#[cfg(feature = "windef")]
pub const HWND_DESKTOP: super::HWND = 0 as _;
#[cfg(feature = "windef")]
pub const HWND_MESSAGE: super::HWND = -3 as _;
#[cfg(feature = "windef")]
pub const HWND_NOTOPMOST: super::HWND = -2 as _;
#[cfg(feature = "windef")]
pub const HWND_TOP: super::HWND = 0 as _;
#[cfg(feature = "windef")]
pub const HWND_TOPMOST: super::HWND = -1 as _;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct ICONINFO {
    pub fIcon: windows_sys::core::BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: super::HBITMAP,
    pub hbmColor: super::HBITMAP,
}
#[cfg(feature = "windef")]
impl Default for ICONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
pub type ICONINFOEX = ICONINFOEXA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct ICONINFOEXA {
    pub cbSize: u32,
    pub fIcon: windows_sys::core::BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: super::HBITMAP,
    pub hbmColor: super::HBITMAP,
    pub wResID: u16,
    pub szModName: [i8; 260],
    pub szResName: [i8; 260],
}
#[cfg(feature = "windef")]
impl Default for ICONINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct ICONINFOEXW {
    pub cbSize: u32,
    pub fIcon: windows_sys::core::BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: super::HBITMAP,
    pub hbmColor: super::HBITMAP,
    pub wResID: u16,
    pub szModName: [u16; 260],
    pub szResName: [u16; 260],
}
#[cfg(feature = "windef")]
impl Default for ICONINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "wingdi")]
pub type ICONMETRICS = ICONMETRICSA;
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Default)]
pub struct ICONMETRICSA {
    pub cbSize: u32,
    pub iHorzSpacing: i32,
    pub iVertSpacing: i32,
    pub iTitleWrap: i32,
    pub lfFont: super::LOGFONTA,
}
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Default)]
pub struct ICONMETRICSW {
    pub cbSize: u32,
    pub iHorzSpacing: i32,
    pub iVertSpacing: i32,
    pub iTitleWrap: i32,
    pub lfFont: super::LOGFONTW,
}
pub const ICON_BIG: u32 = 1;
pub const ICON_SMALL: u32 = 0;
pub const ICON_SMALL2: u32 = 2;
pub const IDABORT: u32 = 3;
pub const IDANI_CAPTION: u32 = 3;
pub const IDANI_OPEN: u32 = 1;
pub const IDCANCEL: u32 = 2;
pub const IDCLOSE: u32 = 8;
pub const IDCONTINUE: u32 = 11;
pub const IDC_APPSTARTING: windows_sys::core::PCWSTR = 32650 as _;
pub const IDC_ARROW: windows_sys::core::PCWSTR = 32512 as _;
pub const IDC_CROSS: windows_sys::core::PCWSTR = 32515 as _;
pub const IDC_HAND: windows_sys::core::PCWSTR = 32649 as _;
pub const IDC_HELP: windows_sys::core::PCWSTR = 32651 as _;
pub const IDC_IBEAM: windows_sys::core::PCWSTR = 32513 as _;
pub const IDC_ICON: windows_sys::core::PCWSTR = 32641 as _;
pub const IDC_NO: windows_sys::core::PCWSTR = 32648 as _;
pub const IDC_PERSON: windows_sys::core::PCWSTR = 32672 as _;
pub const IDC_PIN: windows_sys::core::PCWSTR = 32671 as _;
pub const IDC_SIZE: windows_sys::core::PCWSTR = 32640 as _;
pub const IDC_SIZEALL: windows_sys::core::PCWSTR = 32646 as _;
pub const IDC_SIZENESW: windows_sys::core::PCWSTR = 32643 as _;
pub const IDC_SIZENS: windows_sys::core::PCWSTR = 32645 as _;
pub const IDC_SIZENWSE: windows_sys::core::PCWSTR = 32642 as _;
pub const IDC_SIZEWE: windows_sys::core::PCWSTR = 32644 as _;
pub const IDC_UPARROW: windows_sys::core::PCWSTR = 32516 as _;
pub const IDC_WAIT: windows_sys::core::PCWSTR = 32514 as _;
pub const IDHELP: u32 = 9;
pub const IDHOT_SNAPDESKTOP: i32 = -2;
pub const IDHOT_SNAPWINDOW: i32 = -1;
pub const IDH_CANCEL: u32 = 28444;
pub const IDH_GENERIC_HELP_BUTTON: u32 = 28442;
pub const IDH_HELP: u32 = 28445;
pub const IDH_MISSING_CONTEXT: u32 = 28441;
pub const IDH_NO_HELP: u32 = 28440;
pub const IDH_OK: u32 = 28443;
pub const IDIGNORE: u32 = 5;
pub const IDI_APPLICATION: windows_sys::core::PCWSTR = 32512 as _;
pub const IDI_ASTERISK: windows_sys::core::PCWSTR = 32516 as _;
pub const IDI_EXCLAMATION: windows_sys::core::PCWSTR = 32515 as _;
pub const IDI_HAND: windows_sys::core::PCWSTR = 32513 as _;
pub const IDI_QUESTION: windows_sys::core::PCWSTR = 32514 as _;
pub const IDI_SHIELD: windows_sys::core::PCWSTR = 32518 as _;
pub const IDI_WINLOGO: windows_sys::core::PCWSTR = 32517 as _;
pub const IDNO: u32 = 7;
pub const IDOK: u32 = 1;
pub const IDRETRY: u32 = 4;
pub const IDTIMEOUT: u32 = 32000;
pub const IDTRYAGAIN: u32 = 10;
pub const IDYES: u32 = 6;
pub const IMAGE_BITMAP: u32 = 0;
pub const IMAGE_CURSOR: u32 = 2;
pub const IMAGE_ENHMETAFILE: u32 = 3;
pub const IMAGE_ICON: u32 = 1;
pub const IMDT_KEYBOARD: INPUT_MESSAGE_DEVICE_TYPE = 1;
pub const IMDT_MOUSE: INPUT_MESSAGE_DEVICE_TYPE = 2;
pub const IMDT_PEN: INPUT_MESSAGE_DEVICE_TYPE = 8;
pub const IMDT_TOUCH: INPUT_MESSAGE_DEVICE_TYPE = 4;
pub const IMDT_TOUCHPAD: INPUT_MESSAGE_DEVICE_TYPE = 16;
pub const IMDT_UNAVAILABLE: INPUT_MESSAGE_DEVICE_TYPE = 0;
pub const IMO_HARDWARE: INPUT_MESSAGE_ORIGIN_ID = 1;
pub const IMO_INJECTED: INPUT_MESSAGE_ORIGIN_ID = 2;
pub const IMO_SYSTEM: INPUT_MESSAGE_ORIGIN_ID = 4;
pub const IMO_UNAVAILABLE: INPUT_MESSAGE_ORIGIN_ID = 0;
pub const INDEXID_CONTAINER: u32 = 0;
pub const INDEXID_OBJECT: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT {
    pub r#type: u32,
    pub Anonymous: INPUT_0,
}
impl Default for INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INPUT_0 {
    pub mi: MOUSEINPUT,
    pub ki: KEYBDINPUT,
    pub hi: HARDWAREINPUT,
}
impl Default for INPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INPUTLANGCHANGE_BACKWARD: u32 = 4;
pub const INPUTLANGCHANGE_FORWARD: u32 = 2;
pub const INPUTLANGCHANGE_SYSCHARSET: u32 = 1;
pub const INPUT_HARDWARE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INPUT_INJECTION_VALUE {
    pub page: u16,
    pub usage: u16,
    pub value: i32,
    pub index: u16,
}
pub const INPUT_KEYBOARD: u32 = 1;
pub type INPUT_MESSAGE_DEVICE_TYPE = i32;
pub type INPUT_MESSAGE_ORIGIN_ID = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INPUT_MESSAGE_SOURCE {
    pub deviceType: INPUT_MESSAGE_DEVICE_TYPE,
    pub originId: INPUT_MESSAGE_ORIGIN_ID,
}
pub const INPUT_MOUSE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT_TRANSFORM {
    pub Anonymous: INPUT_TRANSFORM_0,
}
impl Default for INPUT_TRANSFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INPUT_TRANSFORM_0 {
    pub Anonymous: INPUT_TRANSFORM_0_0,
    pub m: [[f32; 4]; 4],
}
impl Default for INPUT_TRANSFORM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INPUT_TRANSFORM_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}
pub const INVALID_MONITOR_TOPOLOGY_ID: u32 = 0;
pub const ISMEX_CALLBACK: u32 = 4;
pub const ISMEX_NOSEND: u32 = 0;
pub const ISMEX_NOTIFY: u32 = 2;
pub const ISMEX_REPLIED: u32 = 8;
pub const ISMEX_SEND: u32 = 1;
pub const ISOLATIONAWARE_MANIFEST_RESOURCE_ID: windows_sys::core::PCWSTR = 2 as _;
pub const ISOLATIONAWARE_NOSTATICIMPORT_MANIFEST_RESOURCE_ID: windows_sys::core::PCWSTR = 3 as _;
pub const ISOLATIONPOLICY_BROWSER_MANIFEST_RESOURCE_ID: windows_sys::core::PCWSTR = 5 as _;
pub const ISOLATIONPOLICY_MANIFEST_RESOURCE_ID: windows_sys::core::PCWSTR = 4 as _;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KBDLLHOOKSTRUCT {
    pub vkCode: u32,
    pub scanCode: u32,
    pub flags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KEYBDINPUT {
    pub wVk: u16,
    pub wScan: u16,
    pub dwFlags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
pub const KEYBOARD_OVERRUN_MAKE_CODE: u32 = 255;
pub const KEYEVENTF_EXTENDEDKEY: u32 = 1;
pub const KEYEVENTF_KEYUP: u32 = 2;
pub const KEYEVENTF_SCANCODE: u32 = 8;
pub const KEYEVENTF_UNICODE: u32 = 4;
pub const KF_ALTDOWN: u32 = 8192;
pub const KF_DLGMODE: u32 = 2048;
pub const KF_EXTENDED: u32 = 256;
pub const KF_MENUMODE: u32 = 4096;
pub const KF_REPEAT: u32 = 16384;
pub const KF_UP: u32 = 32768;
pub const KLF_ACTIVATE: u32 = 1;
pub const KLF_NOTELLSHELL: u32 = 128;
pub const KLF_REORDER: u32 = 8;
pub const KLF_REPLACELANG: u32 = 16;
pub const KLF_RESET: u32 = 1073741824;
pub const KLF_SETFORPROCESS: u32 = 256;
pub const KLF_SHIFTLOCK: u32 = 65536;
pub const KLF_SUBSTITUTE_OK: u32 = 2;
pub const KL_NAMELENGTH: u32 = 9;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LASTINPUTINFO {
    pub cbSize: u32,
    pub dwTime: u32,
}
pub const LBN_DBLCLK: u32 = 2;
pub const LBN_ERRSPACE: i32 = -2;
pub const LBN_KILLFOCUS: u32 = 5;
pub const LBN_SELCANCEL: u32 = 3;
pub const LBN_SELCHANGE: u32 = 1;
pub const LBN_SETFOCUS: u32 = 4;
pub const LBS_COMBOBOX: u32 = 32768;
pub const LBS_DISABLENOSCROLL: u32 = 4096;
pub const LBS_EXTENDEDSEL: u32 = 2048;
pub const LBS_HASSTRINGS: u32 = 64;
pub const LBS_MULTICOLUMN: u32 = 512;
pub const LBS_MULTIPLESEL: u32 = 8;
pub const LBS_NODATA: u32 = 8192;
pub const LBS_NOINTEGRALHEIGHT: u32 = 256;
pub const LBS_NOREDRAW: u32 = 4;
pub const LBS_NOSEL: u32 = 16384;
pub const LBS_NOTIFY: u32 = 1;
pub const LBS_OWNERDRAWFIXED: u32 = 16;
pub const LBS_OWNERDRAWVARIABLE: u32 = 32;
pub const LBS_SORT: u32 = 2;
pub const LBS_STANDARD: u32 = 10485763;
pub const LBS_USETABSTOPS: u32 = 128;
pub const LBS_WANTKEYBOARDINPUT: u32 = 1024;
pub const LB_ADDFILE: u32 = 406;
pub const LB_ADDSTRING: u32 = 384;
pub const LB_CTLCODE: u32 = 0;
pub const LB_DELETESTRING: u32 = 386;
pub const LB_DIR: u32 = 397;
pub const LB_ERR: i32 = -1;
pub const LB_ERRSPACE: i32 = -2;
pub const LB_FINDSTRING: u32 = 399;
pub const LB_FINDSTRINGEXACT: u32 = 418;
pub const LB_GETANCHORINDEX: u32 = 413;
pub const LB_GETCARETINDEX: u32 = 415;
pub const LB_GETCOUNT: u32 = 395;
pub const LB_GETCURSEL: u32 = 392;
pub const LB_GETHORIZONTALEXTENT: u32 = 403;
pub const LB_GETITEMDATA: u32 = 409;
pub const LB_GETITEMHEIGHT: u32 = 417;
pub const LB_GETITEMRECT: u32 = 408;
pub const LB_GETLISTBOXINFO: u32 = 434;
pub const LB_GETLOCALE: u32 = 422;
pub const LB_GETSEL: u32 = 391;
pub const LB_GETSELCOUNT: u32 = 400;
pub const LB_GETSELITEMS: u32 = 401;
pub const LB_GETTEXT: u32 = 393;
pub const LB_GETTEXTLEN: u32 = 394;
pub const LB_GETTOPINDEX: u32 = 398;
pub const LB_INITSTORAGE: u32 = 424;
pub const LB_INSERTSTRING: u32 = 385;
pub const LB_ITEMFROMPOINT: u32 = 425;
pub const LB_MSGMAX: u32 = 435;
pub const LB_OKAY: u32 = 0;
pub const LB_RESETCONTENT: u32 = 388;
pub const LB_SELECTSTRING: u32 = 396;
pub const LB_SELITEMRANGE: u32 = 411;
pub const LB_SELITEMRANGEEX: u32 = 387;
pub const LB_SETANCHORINDEX: u32 = 412;
pub const LB_SETCARETINDEX: u32 = 414;
pub const LB_SETCOLUMNWIDTH: u32 = 405;
pub const LB_SETCOUNT: u32 = 423;
pub const LB_SETCURSEL: u32 = 390;
pub const LB_SETHORIZONTALEXTENT: u32 = 404;
pub const LB_SETITEMDATA: u32 = 410;
pub const LB_SETITEMHEIGHT: u32 = 416;
pub const LB_SETLOCALE: u32 = 421;
pub const LB_SETSEL: u32 = 389;
pub const LB_SETTABSTOPS: u32 = 402;
pub const LB_SETTOPINDEX: u32 = 407;
pub type LEGACY_TOUCHPAD_FEATURES = u32;
pub const LEGACY_TOUCHPAD_FEATURE_ENABLE_DISABLE: LEGACY_TOUCHPAD_FEATURES = 1;
pub const LEGACY_TOUCHPAD_FEATURE_NONE: LEGACY_TOUCHPAD_FEATURES = 0;
pub const LEGACY_TOUCHPAD_FEATURE_REVERSE_SCROLL_DIRECTION: LEGACY_TOUCHPAD_FEATURES = 4;
pub const LLKHF_ALTDOWN: u32 = 32;
pub const LLKHF_EXTENDED: u32 = 1;
pub const LLKHF_INJECTED: u32 = 16;
pub const LLKHF_LOWER_IL_INJECTED: u32 = 2;
pub const LLKHF_UP: u32 = 128;
pub const LLMHF_INJECTED: u32 = 1;
pub const LLMHF_LOWER_IL_INJECTED: u32 = 2;
pub type LPACCEL = *mut ACCEL;
pub type LPACCESSTIMEOUT = *mut ACCESSTIMEOUT;
#[cfg(feature = "windef")]
pub type LPALTTABINFO = *mut ALTTABINFO;
pub type LPANIMATIONINFO = *mut ANIMATIONINFO;
#[cfg(feature = "winnt")]
pub type LPAUDIODESCRIPTION = *mut AUDIODESCRIPTION;
#[cfg(feature = "windef")]
pub type LPCBTACTIVATESTRUCT = *mut CBTACTIVATESTRUCT;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPCBT_CREATEWND = LPCBT_CREATEWNDA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPCBT_CREATEWNDA = *mut CBT_CREATEWNDA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPCBT_CREATEWNDW = *mut CBT_CREATEWNDW;
pub type LPCDLGTEMPLATE = LPCDLGTEMPLATEA;
pub type LPCDLGTEMPLATEA = *const DLGTEMPLATE;
pub type LPCDLGTEMPLATEW = *const DLGTEMPLATE;
#[cfg(feature = "winnt")]
pub type LPCLIENTCREATESTRUCT = *mut CLIENTCREATESTRUCT;
#[cfg(feature = "windef")]
pub type LPCMENUINFO = *const MENUINFO;
#[cfg(feature = "windef")]
pub type LPCMENUITEMINFO = LPCMENUITEMINFOA;
#[cfg(feature = "windef")]
pub type LPCMENUITEMINFOA = *const MENUITEMINFOA;
#[cfg(feature = "windef")]
pub type LPCMENUITEMINFOW = *const MENUITEMINFOW;
#[cfg(feature = "windef")]
pub type LPCOMBOBOXINFO = *mut COMBOBOXINFO;
#[cfg(feature = "windef")]
pub type LPCOMPAREITEMSTRUCT = *mut COMPAREITEMSTRUCT;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPCREATESTRUCT = LPCREATESTRUCTA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPCREATESTRUCTA = *mut CREATESTRUCTA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPCREATESTRUCTW = *mut CREATESTRUCTW;
pub type LPCSCROLLINFO = *const SCROLLINFO;
#[cfg(feature = "windef")]
pub type LPCURSORINFO = *mut CURSORINFO;
pub type LPCURSORSHAPE = *mut CURSORSHAPE;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPCWPRETSTRUCT = *mut CWPRETSTRUCT;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPCWPSTRUCT = *mut CWPSTRUCT;
#[cfg(feature = "minwindef")]
pub type LPDEBUGHOOKINFO = *mut DEBUGHOOKINFO;
#[cfg(feature = "windef")]
pub type LPDELETEITEMSTRUCT = *mut DELETEITEMSTRUCT;
pub type LPDLGITEMTEMPLATE = LPDLGITEMTEMPLATEA;
pub type LPDLGITEMTEMPLATEA = *mut DLGITEMTEMPLATE;
pub type LPDLGITEMTEMPLATEW = *mut DLGITEMTEMPLATE;
pub type LPDLGTEMPLATE = LPDLGTEMPLATEA;
pub type LPDLGTEMPLATEA = *mut DLGTEMPLATE;
pub type LPDLGTEMPLATEW = *mut DLGTEMPLATE;
#[cfg(feature = "windef")]
pub type LPDRAWITEMSTRUCT = *mut DRAWITEMSTRUCT;
pub type LPDRAWTEXTPARAMS = *mut DRAWTEXTPARAMS;
#[cfg(feature = "windef")]
pub type LPDROPSTRUCT = *mut DROPSTRUCT;
#[cfg(feature = "windef")]
pub type LPEVENTMSG = *mut EVENTMSG;
#[cfg(feature = "windef")]
pub type LPEVENTMSGMSG = *mut EVENTMSG;
pub type LPFILTERKEYS = *mut FILTERKEYS;
#[cfg(feature = "windef")]
pub type LPGUITHREADINFO = *mut GUITHREADINFO;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPHARDWAREHOOKSTRUCT = *mut HARDWAREHOOKSTRUCT;
pub type LPHARDWAREINPUT = *mut HARDWAREINPUT;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type LPHELPINFO = *mut HELPINFO;
pub type LPHELPWININFO = LPHELPWININFOA;
pub type LPHELPWININFOA = *mut HELPWININFOA;
pub type LPHELPWININFOW = *mut HELPWININFOW;
pub type LPHIGHCONTRAST = LPHIGHCONTRASTA;
pub type LPHIGHCONTRASTA = *mut HIGHCONTRASTA;
pub type LPHIGHCONTRASTW = *mut HIGHCONTRASTW;
#[cfg(feature = "wingdi")]
pub type LPICONMETRICS = LPICONMETRICSA;
#[cfg(feature = "wingdi")]
pub type LPICONMETRICSA = *mut ICONMETRICSA;
#[cfg(feature = "wingdi")]
pub type LPICONMETRICSW = *mut ICONMETRICSW;
pub type LPINPUT = *mut INPUT;
pub type LPKBDLLHOOKSTRUCT = *mut KBDLLHOOKSTRUCT;
pub type LPKEYBDINPUT = *mut KEYBDINPUT;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type LPMDICREATESTRUCT = LPMDICREATESTRUCTA;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type LPMDICREATESTRUCTA = *mut MDICREATESTRUCTA;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type LPMDICREATESTRUCTW = *mut MDICREATESTRUCTW;
#[cfg(feature = "windef")]
pub type LPMDINEXTMENU = *mut MDINEXTMENU;
pub type LPMEASUREITEMSTRUCT = *mut MEASUREITEMSTRUCT;
#[cfg(feature = "windef")]
pub type LPMENUBARINFO = *mut MENUBARINFO;
#[cfg(feature = "windef")]
pub type LPMENUINFO = *mut MENUINFO;
#[cfg(feature = "windef")]
pub type LPMENUITEMINFO = LPMENUITEMINFOA;
#[cfg(feature = "windef")]
pub type LPMENUITEMINFOA = *mut MENUITEMINFOA;
#[cfg(feature = "windef")]
pub type LPMENUITEMINFOW = *mut MENUITEMINFOW;
pub type LPMENUTEMPLATE = LPMENUTEMPLATEA;
pub type LPMENUTEMPLATEA = *mut core::ffi::c_void;
pub type LPMENUTEMPLATEW = *mut core::ffi::c_void;
pub type LPMINIMIZEDMETRICS = *mut MINIMIZEDMETRICS;
#[cfg(feature = "windef")]
pub type LPMINMAXINFO = *mut MINMAXINFO;
#[cfg(feature = "windef")]
pub type LPMONITORINFO = *mut MONITORINFO;
#[cfg(feature = "windef")]
pub type LPMONITORINFOEX = LPMONITORINFOEXA;
#[cfg(feature = "windef")]
pub type LPMONITORINFOEXA = *mut MONITORINFOEXA;
#[cfg(feature = "windef")]
pub type LPMONITORINFOEXW = *mut MONITORINFOEXW;
#[cfg(feature = "windef")]
pub type LPMOUSEHOOKSTRUCT = *mut MOUSEHOOKSTRUCT;
#[cfg(feature = "windef")]
pub type LPMOUSEHOOKSTRUCTEX = *mut MOUSEHOOKSTRUCTEX;
pub type LPMOUSEINPUT = *mut MOUSEINPUT;
pub type LPMOUSEKEYS = *mut MOUSEKEYS;
pub type LPMOUSEMOVEPOINT = *mut MOUSEMOVEPOINT;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPMSG = *mut MSG;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type LPMSGBOXPARAMS = LPMSGBOXPARAMSA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type LPMSGBOXPARAMSA = *mut MSGBOXPARAMSA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type LPMSGBOXPARAMSW = *mut MSGBOXPARAMSW;
#[cfg(feature = "windef")]
pub type LPMSLLHOOKSTRUCT = *mut MSLLHOOKSTRUCT;
pub type LPMULTIKEYHELP = LPMULTIKEYHELPA;
pub type LPMULTIKEYHELPA = *mut MULTIKEYHELPA;
pub type LPMULTIKEYHELPW = *mut MULTIKEYHELPW;
#[cfg(feature = "windef")]
pub type LPNCCALCSIZE_PARAMS = *mut NCCALCSIZE_PARAMS;
#[cfg(feature = "windef")]
pub type LPNMHDR = *mut NMHDR;
#[cfg(feature = "wingdi")]
pub type LPNONCLIENTMETRICS = LPNONCLIENTMETRICSA;
#[cfg(feature = "wingdi")]
pub type LPNONCLIENTMETRICSA = *mut NONCLIENTMETRICSA;
#[cfg(feature = "wingdi")]
pub type LPNONCLIENTMETRICSW = *mut NONCLIENTMETRICSW;
#[cfg(feature = "windef")]
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;
pub type LPRAWHID = *mut RAWHID;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type LPRAWINPUT = *mut RAWINPUT;
#[cfg(feature = "windef")]
pub type LPRAWINPUTDEVICE = *mut RAWINPUTDEVICE;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type LPRAWINPUTHEADER = *mut RAWINPUTHEADER;
pub type LPRAWKEYBOARD = *mut RAWKEYBOARD;
pub type LPRAWMOUSE = *mut RAWMOUSE;
pub type LPRID_DEVICE_INFO = *mut RID_DEVICE_INFO;
#[cfg(feature = "windef")]
pub type LPSCROLLBARINFO = *mut SCROLLBARINFO;
pub type LPSCROLLINFO = *mut SCROLLINFO;
pub type LPSERIALKEYS = LPSERIALKEYSA;
pub type LPSERIALKEYSA = *mut SERIALKEYSA;
pub type LPSERIALKEYSW = *mut SERIALKEYSW;
#[cfg(feature = "windef")]
pub type LPSHELLHOOKINFO = *mut SHELLHOOKINFO;
pub type LPSOUNDSENTRY = LPSOUNDSENTRYA;
pub type LPSOUNDSENTRYA = *mut SOUNDSENTRYA;
pub type LPSOUNDSENTRYW = *mut SOUNDSENTRYW;
pub type LPSTICKYKEYS = *mut STICKYKEYS;
pub type LPSTYLESTRUCT = *mut STYLESTRUCT;
#[cfg(feature = "windef")]
pub type LPTITLEBARINFO = *mut TITLEBARINFO;
#[cfg(feature = "windef")]
pub type LPTITLEBARINFOEX = *mut TITLEBARINFOEX;
pub type LPTOGGLEKEYS = *mut TOGGLEKEYS;
#[cfg(feature = "windef")]
pub type LPTPMPARAMS = *mut TPMPARAMS;
#[cfg(feature = "windef")]
pub type LPTRACKMOUSEEVENT = *mut TRACKMOUSEEVENT;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPWINDOWINFO = *mut WINDOWINFO;
#[cfg(feature = "windef")]
pub type LPWINDOWPLACEMENT = *mut WINDOWPLACEMENT;
#[cfg(feature = "windef")]
pub type LPWINDOWPOS = *mut WINDOWPOS;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPWNDCLASS = LPWNDCLASSA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPWNDCLASSA = *mut WNDCLASSA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPWNDCLASSEX = LPWNDCLASSEXA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPWNDCLASSEXA = *mut WNDCLASSEXA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPWNDCLASSEXW = *mut WNDCLASSEXW;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPWNDCLASSW = *mut WNDCLASSW;
pub const LR_COLOR: u32 = 2;
pub const LR_COPYDELETEORG: u32 = 8;
pub const LR_COPYFROMRESOURCE: u32 = 16384;
pub const LR_COPYRETURNORG: u32 = 4;
pub const LR_CREATEDIBSECTION: u32 = 8192;
pub const LR_DEFAULTCOLOR: u32 = 0;
pub const LR_DEFAULTSIZE: u32 = 64;
pub const LR_LOADFROMFILE: u32 = 16;
pub const LR_LOADMAP3DCOLORS: u32 = 4096;
pub const LR_LOADTRANSPARENT: u32 = 32;
pub const LR_MONOCHROME: u32 = 1;
pub const LR_SHARED: u32 = 32768;
pub const LR_VGACOLOR: u32 = 128;
pub const LSFW_LOCK: u32 = 1;
pub const LSFW_UNLOCK: u32 = 2;
pub const LWA_ALPHA: u32 = 2;
pub const LWA_COLORKEY: u32 = 1;
pub const MAPVK_VK_TO_CHAR: u32 = 2;
pub const MAPVK_VK_TO_VSC: u32 = 0;
pub const MAPVK_VK_TO_VSC_EX: u32 = 4;
pub const MAPVK_VSC_TO_VK: u32 = 1;
pub const MAPVK_VSC_TO_VK_EX: u32 = 3;
pub const MAX_LOGICALDPIOVERRIDE: u32 = 2;
pub const MAX_STR_BLOCKREASON: u32 = 256;
pub const MAX_TOUCH_COUNT: u32 = 256;
pub const MAX_TOUCH_PREDICTION_FILTER_TAPS: u32 = 3;
pub const MA_ACTIVATE: u32 = 1;
pub const MA_ACTIVATEANDEAT: u32 = 2;
pub const MA_NOACTIVATE: u32 = 3;
pub const MA_NOACTIVATEANDEAT: u32 = 4;
pub const MB_ABORTRETRYIGNORE: u32 = 2;
pub const MB_APPLMODAL: u32 = 0;
pub const MB_CANCELTRYCONTINUE: u32 = 6;
pub const MB_DEFAULT_DESKTOP_ONLY: u32 = 131072;
pub const MB_DEFBUTTON1: u32 = 0;
pub const MB_DEFBUTTON2: u32 = 256;
pub const MB_DEFBUTTON3: u32 = 512;
pub const MB_DEFBUTTON4: u32 = 768;
pub const MB_DEFMASK: u32 = 3840;
pub const MB_HELP: u32 = 16384;
pub const MB_ICONASTERISK: u32 = 64;
pub const MB_ICONERROR: u32 = 16;
pub const MB_ICONEXCLAMATION: u32 = 48;
pub const MB_ICONHAND: u32 = 16;
pub const MB_ICONINFORMATION: u32 = 64;
pub const MB_ICONMASK: u32 = 240;
pub const MB_ICONQUESTION: u32 = 32;
pub const MB_ICONSTOP: u32 = 16;
pub const MB_ICONWARNING: u32 = 48;
pub const MB_MISCMASK: u32 = 49152;
pub const MB_MODEMASK: u32 = 12288;
pub const MB_NOFOCUS: u32 = 32768;
pub const MB_OK: u32 = 0;
pub const MB_OKCANCEL: u32 = 1;
pub const MB_RETRYCANCEL: u32 = 5;
pub const MB_RIGHT: u32 = 524288;
pub const MB_RTLREADING: u32 = 1048576;
pub const MB_SERVICE_NOTIFICATION: u32 = 2097152;
pub const MB_SERVICE_NOTIFICATION_NT3X: u32 = 262144;
pub const MB_SETFOREGROUND: u32 = 65536;
pub const MB_SYSTEMMODAL: u32 = 4096;
pub const MB_TASKMODAL: u32 = 8192;
pub const MB_TOPMOST: u32 = 262144;
pub const MB_TYPEMASK: u32 = 15;
pub const MB_USERICON: u32 = 128;
pub const MB_YESNO: u32 = 4;
pub const MB_YESNOCANCEL: u32 = 3;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type MDICREATESTRUCT = MDICREATESTRUCTA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MDICREATESTRUCTA {
    pub szClass: windows_sys::core::PCSTR,
    pub szTitle: windows_sys::core::PCSTR,
    pub hOwner: super::HANDLE,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub style: u32,
    pub lParam: super::LPARAM,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for MDICREATESTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MDICREATESTRUCTW {
    pub szClass: windows_sys::core::PCWSTR,
    pub szTitle: windows_sys::core::PCWSTR,
    pub hOwner: super::HANDLE,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub style: u32,
    pub lParam: super::LPARAM,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for MDICREATESTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MDINEXTMENU {
    pub hmenuIn: super::HMENU,
    pub hmenuNext: super::HMENU,
    pub hwndNext: super::HWND,
}
#[cfg(feature = "windef")]
impl Default for MDINEXTMENU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MDIS_ALLCHILDSTYLES: u32 = 1;
pub const MDITILE_HORIZONTAL: u32 = 1;
pub const MDITILE_SKIPDISABLED: u32 = 2;
pub const MDITILE_VERTICAL: u32 = 0;
pub const MDITILE_ZORDER: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MEASUREITEMSTRUCT {
    pub CtlType: u32,
    pub CtlID: u32,
    pub itemID: u32,
    pub itemWidth: u32,
    pub itemHeight: u32,
    pub itemData: usize,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MENUBARINFO {
    pub cbSize: u32,
    pub rcBar: super::RECT,
    pub hMenu: super::HMENU,
    pub hwndMenu: super::HWND,
    pub _bitfield: windows_sys::core::BOOL,
}
#[cfg(feature = "windef")]
impl Default for MENUBARINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MENUGETOBJECTINFO {
    pub dwFlags: u32,
    pub uPos: u32,
    pub hmenu: super::HMENU,
    pub riid: *mut core::ffi::c_void,
    pub pvObj: *mut core::ffi::c_void,
}
#[cfg(feature = "windef")]
impl Default for MENUGETOBJECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MENUINFO {
    pub cbSize: u32,
    pub fMask: u32,
    pub dwStyle: u32,
    pub cyMax: u32,
    pub hbrBack: super::HBRUSH,
    pub dwContextHelpID: u32,
    pub dwMenuData: usize,
}
#[cfg(feature = "windef")]
impl Default for MENUINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
pub type MENUITEMINFO = MENUITEMINFOA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MENUITEMINFOA {
    pub cbSize: u32,
    pub fMask: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hSubMenu: super::HMENU,
    pub hbmpChecked: super::HBITMAP,
    pub hbmpUnchecked: super::HBITMAP,
    pub dwItemData: usize,
    pub dwTypeData: windows_sys::core::PSTR,
    pub cch: u32,
    pub hbmpItem: super::HBITMAP,
}
#[cfg(feature = "windef")]
impl Default for MENUITEMINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MENUITEMINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hSubMenu: super::HMENU,
    pub hbmpChecked: super::HBITMAP,
    pub hbmpUnchecked: super::HBITMAP,
    pub dwItemData: usize,
    pub dwTypeData: windows_sys::core::PWSTR,
    pub cch: u32,
    pub hbmpItem: super::HBITMAP,
}
#[cfg(feature = "windef")]
impl Default for MENUITEMINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MENUITEMTEMPLATE {
    pub mtOption: u16,
    pub mtID: u16,
    pub mtString: [u16; 1],
}
impl Default for MENUITEMTEMPLATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MENUITEMTEMPLATEHEADER {
    pub versionNumber: u16,
    pub offset: u16,
}
pub type MENUTEMPLATE = MENUTEMPLATEA;
pub type MENUTEMPLATEA = core::ffi::c_void;
pub type MENUTEMPLATEW = core::ffi::c_void;
pub const MENU_ALL_ACCESS: u32 = 2035711;
pub const MENU_CHECK_ITEM: u32 = 256;
pub const MENU_DELETE_MENU: u32 = 32;
pub const MENU_ENABLE_ITEM: u32 = 128;
pub const MENU_EXECUTE_ACCESS: u32 = 131072;
pub const MENU_GET_ITEM_DATA: u32 = 2;
pub const MENU_GET_ITEM_INFO: u32 = 1;
pub const MENU_GET_SUBMENU: u32 = 4;
pub const MENU_INSERT_ITEM: u32 = 16;
pub const MENU_INSERT_MENU: u32 = 8;
pub const MENU_READ_ACCESS: u32 = 131079;
pub const MENU_SET_DEFAULT_ITEM: u32 = 512;
pub const MENU_SET_ITEM_DATA: u32 = 1024;
pub const MENU_SET_ITEM_INFO: u32 = 64;
pub const MENU_SET_SUBMENU: u32 = 2048;
pub const MENU_WRITE_ACCESS: u32 = 135160;
pub const METRICS_USEDEFAULT: i32 = -1;
pub const MFS_CHECKED: u32 = 8;
pub const MFS_DEFAULT: u32 = 4096;
pub const MFS_DISABLED: u32 = 3;
pub const MFS_ENABLED: u32 = 0;
pub const MFS_GRAYED: u32 = 3;
pub const MFS_HILITE: u32 = 128;
pub const MFS_UNCHECKED: u32 = 0;
pub const MFS_UNHILITE: u32 = 0;
pub const MFT_BITMAP: u32 = 4;
pub const MFT_MENUBARBREAK: u32 = 32;
pub const MFT_MENUBREAK: u32 = 64;
pub const MFT_OWNERDRAW: u32 = 256;
pub const MFT_RADIOCHECK: u32 = 512;
pub const MFT_RIGHTJUSTIFY: u32 = 16384;
pub const MFT_RIGHTORDER: u32 = 8192;
pub const MFT_SEPARATOR: u32 = 2048;
pub const MFT_STRING: u32 = 0;
pub const MF_APPEND: u32 = 256;
pub const MF_BITMAP: u32 = 4;
pub const MF_BYCOMMAND: u32 = 0;
pub const MF_BYPOSITION: u32 = 1024;
pub const MF_CHANGE: u32 = 128;
pub const MF_CHECKED: u32 = 8;
pub const MF_DEFAULT: u32 = 4096;
pub const MF_DELETE: u32 = 512;
pub const MF_DISABLED: u32 = 2;
pub const MF_ENABLED: u32 = 0;
pub const MF_END: u32 = 128;
pub const MF_GRAYED: u32 = 1;
pub const MF_HELP: u32 = 16384;
pub const MF_HILITE: u32 = 128;
pub const MF_INSERT: u32 = 0;
pub const MF_MENUBARBREAK: u32 = 32;
pub const MF_MENUBREAK: u32 = 64;
pub const MF_MOUSESELECT: u32 = 32768;
pub const MF_OWNERDRAW: u32 = 256;
pub const MF_POPUP: u32 = 16;
pub const MF_REMOVE: u32 = 4096;
pub const MF_RIGHTJUSTIFY: u32 = 16384;
pub const MF_SEPARATOR: u32 = 2048;
pub const MF_STRING: u32 = 0;
pub const MF_SYSMENU: u32 = 8192;
pub const MF_UNCHECKED: u32 = 0;
pub const MF_UNHILITE: u32 = 0;
pub const MF_USECHECKBITMAPS: u32 = 512;
pub const MIIM_BITMAP: u32 = 128;
pub const MIIM_CHECKMARKS: u32 = 8;
pub const MIIM_DATA: u32 = 32;
pub const MIIM_FTYPE: u32 = 256;
pub const MIIM_ID: u32 = 2;
pub const MIIM_STATE: u32 = 1;
pub const MIIM_STRING: u32 = 64;
pub const MIIM_SUBMENU: u32 = 4;
pub const MIIM_TYPE: u32 = 16;
pub const MIM_APPLYTOSUBMENUS: u32 = 2147483648;
pub const MIM_BACKGROUND: u32 = 2;
pub const MIM_HELPID: u32 = 4;
pub const MIM_MAXHEIGHT: u32 = 1;
pub const MIM_MENUDATA: u32 = 8;
pub const MIM_STYLE: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MINIMIZEDMETRICS {
    pub cbSize: u32,
    pub iWidth: i32,
    pub iHorzGap: i32,
    pub iVertGap: i32,
    pub iArrange: i32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct MINMAXINFO {
    pub ptReserved: super::POINT,
    pub ptMaxSize: super::POINT,
    pub ptMaxPosition: super::POINT,
    pub ptMinTrackSize: super::POINT,
    pub ptMaxTrackSize: super::POINT,
}
pub const MIN_LOGICALDPIOVERRIDE: i32 = -2;
pub const MKF_AVAILABLE: u32 = 2;
pub const MKF_CONFIRMHOTKEY: u32 = 8;
pub const MKF_HOTKEYACTIVE: u32 = 4;
pub const MKF_HOTKEYSOUND: u32 = 16;
pub const MKF_INDICATOR: u32 = 32;
pub const MKF_LEFTBUTTONDOWN: u32 = 16777216;
pub const MKF_LEFTBUTTONSEL: u32 = 268435456;
pub const MKF_MODIFIERS: u32 = 64;
pub const MKF_MOUSEKEYSON: u32 = 1;
pub const MKF_MOUSEMODE: u32 = 2147483648;
pub const MKF_REPLACENUMBERS: u32 = 128;
pub const MKF_RIGHTBUTTONDOWN: u32 = 33554432;
pub const MKF_RIGHTBUTTONSEL: u32 = 536870912;
pub const MK_CONTROL: u32 = 8;
pub const MK_LBUTTON: u32 = 1;
pub const MK_MBUTTON: u32 = 16;
pub const MK_RBUTTON: u32 = 2;
pub const MK_SHIFT: u32 = 4;
pub const MK_XBUTTON1: u32 = 32;
pub const MK_XBUTTON2: u32 = 64;
pub const MNC_CLOSE: u32 = 1;
pub const MNC_EXECUTE: u32 = 2;
pub const MNC_IGNORE: u32 = 0;
pub const MNC_SELECT: u32 = 3;
pub const MND_CONTINUE: u32 = 0;
pub const MND_ENDMENU: u32 = 1;
pub const MNGOF_BOTTOMGAP: u32 = 2;
pub const MNGOF_TOPGAP: u32 = 1;
pub const MNGO_NOERROR: u32 = 1;
pub const MNGO_NOINTERFACE: u32 = 0;
pub const MNS_AUTODISMISS: u32 = 268435456;
pub const MNS_CHECKORBMP: u32 = 67108864;
pub const MNS_DRAGDROP: u32 = 536870912;
pub const MNS_MODELESS: u32 = 1073741824;
pub const MNS_NOCHECK: u32 = 2147483648;
pub const MNS_NOTIFYBYPOS: u32 = 134217728;
pub const MN_GETHMENU: u32 = 481;
pub const MOD_ALT: u32 = 1;
pub const MOD_CONTROL: u32 = 2;
pub const MOD_NOREPEAT: u32 = 16384;
pub const MOD_SHIFT: u32 = 4;
pub const MOD_WIN: u32 = 8;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type MONITORENUMPROC = Option<unsafe extern "system" fn(param0: super::HMONITOR, param1: super::HDC, param2: *mut super::RECT, param3: super::LPARAM) -> windows_sys::core::BOOL>;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: super::RECT,
    pub rcWork: super::RECT,
    pub dwFlags: u32,
}
#[cfg(feature = "windef")]
pub type MONITORINFOEX = MONITORINFOEXA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MONITORINFOEXA {
    pub Base: MONITORINFO,
    pub szDevice: [i8; 32],
}
#[cfg(feature = "windef")]
impl Default for MONITORINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MONITORINFOEXW {
    pub Base: MONITORINFO,
    pub szDevice: [u16; 32],
}
#[cfg(feature = "windef")]
impl Default for MONITORINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MONITORINFOF_PRIMARY: u32 = 1;
pub const MONITOR_DEFAULTTONEAREST: u32 = 2;
pub const MONITOR_DEFAULTTONULL: u32 = 0;
pub const MONITOR_DEFAULTTOPRIMARY: u32 = 1;
pub const MOUSEEVENTF_ABSOLUTE: u32 = 32768;
pub const MOUSEEVENTF_HWHEEL: u32 = 4096;
pub const MOUSEEVENTF_LEFTDOWN: u32 = 2;
pub const MOUSEEVENTF_LEFTUP: u32 = 4;
pub const MOUSEEVENTF_MIDDLEDOWN: u32 = 32;
pub const MOUSEEVENTF_MIDDLEUP: u32 = 64;
pub const MOUSEEVENTF_MOVE: u32 = 1;
pub const MOUSEEVENTF_MOVE_NOCOALESCE: u32 = 8192;
pub const MOUSEEVENTF_RIGHTDOWN: u32 = 8;
pub const MOUSEEVENTF_RIGHTUP: u32 = 16;
pub const MOUSEEVENTF_VIRTUALDESK: u32 = 16384;
pub const MOUSEEVENTF_WHEEL: u32 = 2048;
pub const MOUSEEVENTF_XDOWN: u32 = 128;
pub const MOUSEEVENTF_XUP: u32 = 256;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MOUSEHOOKSTRUCT {
    pub pt: super::POINT,
    pub hwnd: super::HWND,
    pub wHitTestCode: u32,
    pub dwExtraInfo: usize,
}
#[cfg(feature = "windef")]
impl Default for MOUSEHOOKSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct MOUSEHOOKSTRUCTEX {
    pub Base: MOUSEHOOKSTRUCT,
    pub mouseData: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MOUSEINPUT {
    pub dx: i32,
    pub dy: i32,
    pub mouseData: u32,
    pub dwFlags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MOUSEKEYS {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iMaxSpeed: u32,
    pub iTimeToMaxSpeed: u32,
    pub iCtrlSpeed: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MOUSEMOVEPOINT {
    pub x: i32,
    pub y: i32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
pub const MOUSEWHEEL_ROUTING_FOCUS: u32 = 0;
pub const MOUSEWHEEL_ROUTING_HYBRID: u32 = 1;
pub const MOUSEWHEEL_ROUTING_MOUSE_POS: u32 = 2;
pub const MOUSE_ATTRIBUTES_CHANGED: u32 = 4;
pub const MOUSE_MOVE_ABSOLUTE: u32 = 1;
pub const MOUSE_MOVE_NOCOALESCE: u32 = 8;
pub const MOUSE_MOVE_RELATIVE: u32 = 0;
pub const MOUSE_VIRTUAL_DESKTOP: u32 = 2;
pub type MOVESIZE_OPERATION = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct MSG {
    pub hwnd: super::HWND,
    pub message: u32,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
    pub time: u32,
    pub pt: super::POINT,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for MSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type MSGBOXCALLBACK = Option<unsafe extern "system" fn(lphelpinfo: *mut HELPINFO)>;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type MSGBOXPARAMS = MSGBOXPARAMSA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MSGBOXPARAMSA {
    pub cbSize: u32,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub lpszText: windows_sys::core::PCSTR,
    pub lpszCaption: windows_sys::core::PCSTR,
    pub dwStyle: u32,
    pub lpszIcon: windows_sys::core::PCSTR,
    pub dwContextHelpId: usize,
    pub lpfnMsgBoxCallback: MSGBOXCALLBACK,
    pub dwLanguageId: u32,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl Default for MSGBOXPARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MSGBOXPARAMSW {
    pub cbSize: u32,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub lpszText: windows_sys::core::PCWSTR,
    pub lpszCaption: windows_sys::core::PCWSTR,
    pub dwStyle: u32,
    pub lpszIcon: windows_sys::core::PCWSTR,
    pub dwContextHelpId: usize,
    pub lpfnMsgBoxCallback: MSGBOXCALLBACK,
    pub dwLanguageId: u32,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl Default for MSGBOXPARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSGFLTINFO_ALLOWED_HIGHER: u32 = 3;
pub const MSGFLTINFO_ALREADYALLOWED_FORWND: u32 = 1;
pub const MSGFLTINFO_ALREADYDISALLOWED_FORWND: u32 = 2;
pub const MSGFLTINFO_NONE: u32 = 0;
pub const MSGFLT_ADD: u32 = 1;
pub const MSGFLT_ALLOW: u32 = 1;
pub const MSGFLT_DISALLOW: u32 = 2;
pub const MSGFLT_REMOVE: u32 = 2;
pub const MSGFLT_RESET: u32 = 0;
pub const MSGF_DIALOGBOX: u32 = 0;
pub const MSGF_MAX: u32 = 8;
pub const MSGF_MENU: u32 = 2;
pub const MSGF_MESSAGEBOX: u32 = 1;
pub const MSGF_NEXTWINDOW: u32 = 6;
pub const MSGF_SCROLLBAR: u32 = 5;
pub const MSGF_USER: u32 = 4096;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct MSLLHOOKSTRUCT {
    pub pt: super::POINT,
    pub mouseData: u32,
    pub flags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
pub const MSO_MOVE: MOVESIZE_OPERATION = 9;
pub const MSO_SIZE_BOTTOM: MOVESIZE_OPERATION = 6;
pub const MSO_SIZE_BOTTOMLEFT: MOVESIZE_OPERATION = 7;
pub const MSO_SIZE_BOTTOMRIGHT: MOVESIZE_OPERATION = 8;
pub const MSO_SIZE_LEFT: MOVESIZE_OPERATION = 1;
pub const MSO_SIZE_RIGHT: MOVESIZE_OPERATION = 2;
pub const MSO_SIZE_TOP: MOVESIZE_OPERATION = 3;
pub const MSO_SIZE_TOPLEFT: MOVESIZE_OPERATION = 4;
pub const MSO_SIZE_TOPRIGHT: MOVESIZE_OPERATION = 5;
pub type MULTIKEYHELP = MULTIKEYHELPA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MULTIKEYHELPA {
    pub mkSize: u32,
    pub mkKeylist: i8,
    pub szKeyphrase: [i8; 1],
}
impl Default for MULTIKEYHELPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MULTIKEYHELPW {
    pub mkSize: u32,
    pub mkKeylist: u16,
    pub szKeyphrase: [u16; 1],
}
impl Default for MULTIKEYHELPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MWMO_ALERTABLE: u32 = 2;
pub const MWMO_INPUTAVAILABLE: u32 = 4;
pub const MWMO_WAITALL: u32 = 1;
#[cfg(feature = "minwindef")]
pub type NAMEENUMPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: super::LPARAM) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type NAMEENUMPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: super::LPARAM) -> windows_sys::core::BOOL>;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct NCCALCSIZE_PARAMS {
    pub rgrc: [super::RECT; 3],
    pub lppos: PWINDOWPOS,
}
#[cfg(feature = "windef")]
impl Default for NCCALCSIZE_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NFR_ANSI: u32 = 1;
pub const NFR_UNICODE: u32 = 2;
pub const NF_QUERY: u32 = 3;
pub const NF_REQUERY: u32 = 4;
pub const NID_EXTERNAL_PEN: u32 = 8;
pub const NID_EXTERNAL_TOUCH: u32 = 2;
pub const NID_INTEGRATED_PEN: u32 = 4;
pub const NID_INTEGRATED_TOUCH: u32 = 1;
pub const NID_MULTI_INPUT: u32 = 64;
pub const NID_READY: u32 = 128;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct NMHDR {
    pub hwndFrom: super::HWND,
    pub idFrom: usize,
    pub code: u32,
}
#[cfg(feature = "windef")]
impl Default for NMHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "wingdi")]
pub type NONCLIENTMETRICS = NONCLIENTMETRICSA;
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Default)]
pub struct NONCLIENTMETRICSA {
    pub cbSize: u32,
    pub iBorderWidth: i32,
    pub iScrollWidth: i32,
    pub iScrollHeight: i32,
    pub iCaptionWidth: i32,
    pub iCaptionHeight: i32,
    pub lfCaptionFont: super::LOGFONTA,
    pub iSmCaptionWidth: i32,
    pub iSmCaptionHeight: i32,
    pub lfSmCaptionFont: super::LOGFONTA,
    pub iMenuWidth: i32,
    pub iMenuHeight: i32,
    pub lfMenuFont: super::LOGFONTA,
    pub lfStatusFont: super::LOGFONTA,
    pub lfMessageFont: super::LOGFONTA,
    pub iPaddedBorderWidth: i32,
}
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Default)]
pub struct NONCLIENTMETRICSW {
    pub cbSize: u32,
    pub iBorderWidth: i32,
    pub iScrollWidth: i32,
    pub iScrollHeight: i32,
    pub iCaptionWidth: i32,
    pub iCaptionHeight: i32,
    pub lfCaptionFont: super::LOGFONTW,
    pub iSmCaptionWidth: i32,
    pub iSmCaptionHeight: i32,
    pub lfSmCaptionFont: super::LOGFONTW,
    pub iMenuWidth: i32,
    pub iMenuHeight: i32,
    pub lfMenuFont: super::LOGFONTW,
    pub lfStatusFont: super::LOGFONTW,
    pub lfMessageFont: super::LOGFONTW,
    pub iPaddedBorderWidth: i32,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type NPCWPRETSTRUCT = *mut CWPRETSTRUCT;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type NPCWPSTRUCT = *mut CWPSTRUCT;
#[cfg(feature = "minwindef")]
pub type NPDEBUGHOOKINFO = *mut DEBUGHOOKINFO;
#[cfg(feature = "windef")]
pub type NPEVENTMSG = *mut EVENTMSG;
#[cfg(feature = "windef")]
pub type NPEVENTMSGMSG = *mut EVENTMSG;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type NPMSG = *mut MSG;
#[cfg(feature = "windef")]
pub type NPPAINTSTRUCT = *mut PAINTSTRUCT;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type NPWNDCLASS = NPWNDCLASSA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type NPWNDCLASSA = *mut WNDCLASSA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type NPWNDCLASSEX = NPWNDCLASSEXA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type NPWNDCLASSEXA = *mut WNDCLASSEXA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type NPWNDCLASSEXW = *mut WNDCLASSEXW;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type NPWNDCLASSW = *mut WNDCLASSW;
pub const OBJID_ALERT: i32 = -10;
pub const OBJID_CARET: i32 = -8;
pub const OBJID_CLIENT: i32 = -4;
pub const OBJID_CURSOR: i32 = -9;
pub const OBJID_HSCROLL: i32 = -6;
pub const OBJID_MENU: i32 = -3;
pub const OBJID_NATIVEOM: i32 = -16;
pub const OBJID_QUERYCLASSNAMEIDX: i32 = -12;
pub const OBJID_SIZEGRIP: i32 = -7;
pub const OBJID_SOUND: i32 = -11;
pub const OBJID_SYSMENU: i32 = -1;
pub const OBJID_TITLEBAR: i32 = -2;
pub const OBJID_VSCROLL: i32 = -5;
pub const OBJID_WINDOW: i32 = 0;
pub const ODA_DRAWENTIRE: u32 = 1;
pub const ODA_FOCUS: u32 = 4;
pub const ODA_SELECT: u32 = 2;
pub const ODS_CHECKED: u32 = 8;
pub const ODS_COMBOBOXEDIT: u32 = 4096;
pub const ODS_DEFAULT: u32 = 32;
pub const ODS_DISABLED: u32 = 4;
pub const ODS_FOCUS: u32 = 16;
pub const ODS_GRAYED: u32 = 2;
pub const ODS_HOTLIGHT: u32 = 64;
pub const ODS_INACTIVE: u32 = 128;
pub const ODS_NOACCEL: u32 = 256;
pub const ODS_NOFOCUSRECT: u32 = 512;
pub const ODS_SELECTED: u32 = 1;
pub const ODT_BUTTON: u32 = 4;
pub const ODT_COMBOBOX: u32 = 3;
pub const ODT_LISTBOX: u32 = 2;
pub const ODT_MENU: u32 = 1;
pub const ODT_STATIC: u32 = 5;
pub const ORD_LANGDRIVER: u32 = 1;
pub type ORIENTATION_PREFERENCE = u32;
pub const ORIENTATION_PREFERENCE_LANDSCAPE: ORIENTATION_PREFERENCE = 1;
pub const ORIENTATION_PREFERENCE_LANDSCAPE_FLIPPED: ORIENTATION_PREFERENCE = 4;
pub const ORIENTATION_PREFERENCE_NONE: ORIENTATION_PREFERENCE = 0;
pub const ORIENTATION_PREFERENCE_PORTRAIT: ORIENTATION_PREFERENCE = 2;
pub const ORIENTATION_PREFERENCE_PORTRAIT_FLIPPED: ORIENTATION_PREFERENCE = 8;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct PAINTSTRUCT {
    pub hdc: super::HDC,
    pub fErase: windows_sys::core::BOOL,
    pub rcPaint: super::RECT,
    pub fRestore: windows_sys::core::BOOL,
    pub fIncUpdate: windows_sys::core::BOOL,
    pub rgbReserved: [u8; 32],
}
#[cfg(feature = "windef")]
impl Default for PAINTSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
pub type PALTTABINFO = *mut ALTTABINFO;
pub type PAR_STATE = *mut AR_STATE;
pub const PA_ACTIVATE: u32 = 1;
pub const PA_NOACTIVATE: u32 = 3;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PBSMINFO = *mut BSMINFO;
pub const PBTF_APMRESUMEFROMFAILURE: u32 = 1;
pub const PBT_APMBATTERYLOW: u32 = 9;
pub const PBT_APMOEMEVENT: u32 = 11;
pub const PBT_APMPOWERSTATUSCHANGE: u32 = 10;
pub const PBT_APMQUERYSTANDBY: u32 = 1;
pub const PBT_APMQUERYSTANDBYFAILED: u32 = 3;
pub const PBT_APMQUERYSUSPEND: u32 = 0;
pub const PBT_APMQUERYSUSPENDFAILED: u32 = 2;
pub const PBT_APMRESUMEAUTOMATIC: u32 = 18;
pub const PBT_APMRESUMECRITICAL: u32 = 6;
pub const PBT_APMRESUMESTANDBY: u32 = 8;
pub const PBT_APMRESUMESUSPEND: u32 = 7;
pub const PBT_APMSTANDBY: u32 = 5;
pub const PBT_APMSUSPEND: u32 = 4;
pub const PBT_POWERSETTINGCHANGE: u32 = 32787;
#[cfg(feature = "windef")]
pub type PCGESTUREINFO = *const GESTUREINFO;
pub type PCHANGEFILTERSTRUCT = *mut CHANGEFILTERSTRUCT;
#[cfg(feature = "windef")]
pub type PCOMBOBOXINFO = *mut COMBOBOXINFO;
#[cfg(feature = "windef")]
pub type PCOMPAREITEMSTRUCT = *mut COMPAREITEMSTRUCT;
pub type PCOPYDATASTRUCT = *mut COPYDATASTRUCT;
#[cfg(feature = "windef")]
pub type PCRAWINPUTDEVICE = *const RAWINPUTDEVICE;
#[cfg(feature = "winnt")]
pub type PCTOUCHINPUT = *const TOUCHINPUT;
#[cfg(feature = "windef")]
pub type PCURSORINFO = *mut CURSORINFO;
#[cfg(feature = "windef")]
pub type PCWINDOW_ACTION = *const WINDOW_ACTION;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PCWPRETSTRUCT = *mut CWPRETSTRUCT;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PCWPSTRUCT = *mut CWPSTRUCT;
pub const PDC_ARRIVAL: u32 = 1;
pub const PDC_MAPPING_CHANGE: u32 = 256;
pub const PDC_MODE_ASPECTRATIOPRESERVED: u32 = 2048;
pub const PDC_MODE_CENTERED: u32 = 128;
pub const PDC_MODE_DEFAULT: u32 = 64;
pub const PDC_ORIENTATION_0: u32 = 4;
pub const PDC_ORIENTATION_180: u32 = 16;
pub const PDC_ORIENTATION_270: u32 = 32;
pub const PDC_ORIENTATION_90: u32 = 8;
pub const PDC_ORIGIN: u32 = 1024;
pub const PDC_REMOVAL: u32 = 2;
pub const PDC_RESOLUTION: u32 = 512;
#[cfg(feature = "minwindef")]
pub type PDEBUGHOOKINFO = *mut DEBUGHOOKINFO;
#[cfg(feature = "windef")]
pub type PDELETEITEMSTRUCT = *mut DELETEITEMSTRUCT;
pub type PDLGITEMTEMPLATE = PDLGITEMTEMPLATEA;
pub type PDLGITEMTEMPLATEA = *mut DLGITEMTEMPLATE;
pub type PDLGITEMTEMPLATEW = *mut DLGITEMTEMPLATE;
#[cfg(feature = "windef")]
pub type PDRAWITEMSTRUCT = *mut DRAWITEMSTRUCT;
#[cfg(feature = "windef")]
pub type PDROPSTRUCT = *mut DROPSTRUCT;
pub const PENARBITRATIONTYPE_FIS: u32 = 2;
pub const PENARBITRATIONTYPE_MAX: u32 = 4;
pub const PENARBITRATIONTYPE_NONE: u32 = 0;
pub const PENARBITRATIONTYPE_SPT: u32 = 3;
pub const PENARBITRATIONTYPE_WIN8: u32 = 1;
pub const PENVISUALIZATION_CURSOR: u32 = 32;
pub const PENVISUALIZATION_DOUBLETAP: u32 = 2;
pub const PENVISUALIZATION_OFF: u32 = 0;
pub const PENVISUALIZATION_ON: u32 = 35;
pub const PENVISUALIZATION_TAP: u32 = 1;
pub type PEN_FLAGS = u32;
pub const PEN_FLAG_BARREL: u32 = 1;
pub const PEN_FLAG_ERASER: u32 = 4;
pub const PEN_FLAG_INVERTED: u32 = 2;
pub const PEN_FLAG_NONE: u32 = 0;
pub type PEN_MASK = u32;
pub const PEN_MASK_NONE: u32 = 0;
pub const PEN_MASK_PRESSURE: u32 = 1;
pub const PEN_MASK_ROTATION: u32 = 2;
pub const PEN_MASK_TILT_X: u32 = 4;
pub const PEN_MASK_TILT_Y: u32 = 8;
#[cfg(feature = "windef")]
pub type PEVENTMSG = *mut EVENTMSG;
#[cfg(feature = "windef")]
pub type PEVENTMSGMSG = *mut EVENTMSG;
#[cfg(feature = "windef")]
pub type PFLASHWINFO = *mut FLASHWINFO;
pub type PGESTURECONFIG = *mut GESTURECONFIG;
#[cfg(feature = "windef")]
pub type PGESTUREINFO = *mut GESTUREINFO;
#[cfg(feature = "windef")]
pub type PGESTURENOTIFYSTRUCT = *mut GESTURENOTIFYSTRUCT;
pub type PGETCLIPBMETADATA = *mut GETCLIPBMETADATA;
#[cfg(feature = "windef")]
pub type PGUITHREADINFO = *mut GUITHREADINFO;
pub type PHANDEDNESS = *mut HANDEDNESS;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PHARDWAREHOOKSTRUCT = *mut HARDWAREHOOKSTRUCT;
pub type PHARDWAREINPUT = *mut HARDWAREINPUT;
pub type PHDEVNOTIFY = *mut HDEVNOTIFY;
pub type PHELPWININFO = PHELPWININFOA;
pub type PHELPWININFOA = *mut HELPWININFOA;
pub type PHELPWININFOW = *mut HELPWININFOW;
pub type PHPOWERNOTIFY = *mut HPOWERNOTIFY;
#[cfg(feature = "windef")]
pub type PICONINFO = *mut ICONINFO;
#[cfg(feature = "windef")]
pub type PICONINFOEX = PICONINFOEXA;
#[cfg(feature = "windef")]
pub type PICONINFOEXA = *mut ICONINFOEXA;
#[cfg(feature = "windef")]
pub type PICONINFOEXW = *mut ICONINFOEXW;
#[cfg(feature = "wingdi")]
pub type PICONMETRICS = PICONMETRICSA;
#[cfg(feature = "wingdi")]
pub type PICONMETRICSA = *mut ICONMETRICSA;
#[cfg(feature = "wingdi")]
pub type PICONMETRICSW = *mut ICONMETRICSW;
pub type PINPUT = *mut INPUT;
pub type PINPUT_INJECTION_VALUE = *mut INPUT_INJECTION_VALUE;
pub type PKBDLLHOOKSTRUCT = *mut KBDLLHOOKSTRUCT;
pub type PKEYBDINPUT = *mut KEYBDINPUT;
pub type PLASTINPUTINFO = *mut LASTINPUTINFO;
pub const PMB_ACTIVE: u32 = 1;
#[cfg(feature = "windef")]
pub type PMDINEXTMENU = *mut MDINEXTMENU;
pub type PMEASUREITEMSTRUCT = *mut MEASUREITEMSTRUCT;
#[cfg(feature = "windef")]
pub type PMENUBARINFO = *mut MENUBARINFO;
#[cfg(feature = "windef")]
pub type PMENUGETOBJECTINFO = *mut MENUGETOBJECTINFO;
pub type PMENUITEMTEMPLATE = *mut MENUITEMTEMPLATE;
pub type PMENUITEMTEMPLATEHEADER = *mut MENUITEMTEMPLATEHEADER;
pub type PMINIMIZEDMETRICS = *mut MINIMIZEDMETRICS;
#[cfg(feature = "windef")]
pub type PMINMAXINFO = *mut MINMAXINFO;
#[cfg(feature = "windef")]
pub type PMOUSEHOOKSTRUCT = *mut MOUSEHOOKSTRUCT;
#[cfg(feature = "windef")]
pub type PMOUSEHOOKSTRUCTEX = *mut MOUSEHOOKSTRUCTEX;
pub type PMOUSEINPUT = *mut MOUSEINPUT;
pub type PMOUSEMOVEPOINT = *mut MOUSEMOVEPOINT;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PMSG = *mut MSG;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type PMSGBOXPARAMS = PMSGBOXPARAMSA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type PMSGBOXPARAMSA = *mut MSGBOXPARAMSA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type PMSGBOXPARAMSW = *mut MSGBOXPARAMSW;
#[cfg(feature = "windef")]
pub type PMSLLHOOKSTRUCT = *mut MSLLHOOKSTRUCT;
pub type PMULTIKEYHELP = PMULTIKEYHELPA;
pub type PMULTIKEYHELPA = *mut MULTIKEYHELPA;
pub type PMULTIKEYHELPW = *mut MULTIKEYHELPW;
pub const PM_NOREMOVE: u32 = 0;
pub const PM_NOYIELD: u32 = 2;
pub const PM_QS_INPUT: u32 = 470220800;
pub const PM_QS_PAINT: u32 = 2097152;
pub const PM_QS_POSTMESSAGE: u32 = 9961472;
pub const PM_QS_SENDMESSAGE: u32 = 4194304;
pub const PM_REMOVE: u32 = 1;
#[cfg(feature = "wingdi")]
pub type PNONCLIENTMETRICS = PNONCLIENTMETRICSA;
#[cfg(feature = "wingdi")]
pub type PNONCLIENTMETRICSA = *mut NONCLIENTMETRICSA;
#[cfg(feature = "wingdi")]
pub type PNONCLIENTMETRICSW = *mut NONCLIENTMETRICSW;
pub type POINTER_BUTTON_CHANGE_TYPE = i32;
pub const POINTER_CHANGE_FIFTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 9;
pub const POINTER_CHANGE_FIFTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 10;
pub const POINTER_CHANGE_FIRSTBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 1;
pub const POINTER_CHANGE_FIRSTBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 2;
pub const POINTER_CHANGE_FOURTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 7;
pub const POINTER_CHANGE_FOURTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 8;
pub const POINTER_CHANGE_NONE: POINTER_BUTTON_CHANGE_TYPE = 0;
pub const POINTER_CHANGE_SECONDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 3;
pub const POINTER_CHANGE_SECONDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 4;
pub const POINTER_CHANGE_THIRDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 5;
pub const POINTER_CHANGE_THIRDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 6;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POINTER_DEVICE_CURSOR_INFO {
    pub cursorId: u32,
    pub cursor: POINTER_DEVICE_CURSOR_TYPE,
}
pub type POINTER_DEVICE_CURSOR_TYPE = i32;
pub const POINTER_DEVICE_CURSOR_TYPE_ERASER: POINTER_DEVICE_CURSOR_TYPE = 2;
pub const POINTER_DEVICE_CURSOR_TYPE_MAX: POINTER_DEVICE_CURSOR_TYPE = -1;
pub const POINTER_DEVICE_CURSOR_TYPE_TIP: POINTER_DEVICE_CURSOR_TYPE = 1;
pub const POINTER_DEVICE_CURSOR_TYPE_UNKNOWN: POINTER_DEVICE_CURSOR_TYPE = 0;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct POINTER_DEVICE_INFO {
    pub displayOrientation: u32,
    pub device: super::HANDLE,
    pub pointerDeviceType: POINTER_DEVICE_TYPE,
    pub monitor: super::HMONITOR,
    pub startingCursorId: u32,
    pub maxActiveContacts: u16,
    pub productString: [u16; 520],
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for POINTER_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POINTER_DEVICE_PRODUCT_STRING_MAX: u32 = 520;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POINTER_DEVICE_PROPERTY {
    pub logicalMin: i32,
    pub logicalMax: i32,
    pub physicalMin: i32,
    pub physicalMax: i32,
    pub unit: u32,
    pub unitExponent: u32,
    pub usagePageId: u16,
    pub usageId: u16,
}
pub type POINTER_DEVICE_TYPE = i32;
pub const POINTER_DEVICE_TYPE_EXTERNAL_PEN: POINTER_DEVICE_TYPE = 2;
pub const POINTER_DEVICE_TYPE_INTEGRATED_PEN: POINTER_DEVICE_TYPE = 1;
pub const POINTER_DEVICE_TYPE_MAX: POINTER_DEVICE_TYPE = -1;
pub const POINTER_DEVICE_TYPE_TOUCH: POINTER_DEVICE_TYPE = 3;
pub const POINTER_DEVICE_TYPE_TOUCH_PAD: POINTER_DEVICE_TYPE = 4;
pub const POINTER_FEEDBACK_DEFAULT: POINTER_FEEDBACK_MODE = 1;
pub const POINTER_FEEDBACK_INDIRECT: POINTER_FEEDBACK_MODE = 2;
pub type POINTER_FEEDBACK_MODE = i32;
pub const POINTER_FEEDBACK_NONE: POINTER_FEEDBACK_MODE = 3;
pub type POINTER_FLAGS = u32;
pub const POINTER_FLAG_CANCELED: u32 = 32768;
pub const POINTER_FLAG_CAPTURECHANGED: u32 = 2097152;
pub const POINTER_FLAG_CONFIDENCE: u32 = 16384;
pub const POINTER_FLAG_DOWN: u32 = 65536;
pub const POINTER_FLAG_FIFTHBUTTON: u32 = 256;
pub const POINTER_FLAG_FIRSTBUTTON: u32 = 16;
pub const POINTER_FLAG_FOURTHBUTTON: u32 = 128;
pub const POINTER_FLAG_HASTRANSFORM: u32 = 4194304;
pub const POINTER_FLAG_HWHEEL: u32 = 1048576;
pub const POINTER_FLAG_INCONTACT: u32 = 4;
pub const POINTER_FLAG_INRANGE: u32 = 2;
pub const POINTER_FLAG_NEW: u32 = 1;
pub const POINTER_FLAG_NONE: u32 = 0;
pub const POINTER_FLAG_PRIMARY: u32 = 8192;
pub const POINTER_FLAG_SECONDBUTTON: u32 = 32;
pub const POINTER_FLAG_THIRDBUTTON: u32 = 64;
pub const POINTER_FLAG_UP: u32 = 262144;
pub const POINTER_FLAG_UPDATE: u32 = 131072;
pub const POINTER_FLAG_WHEEL: u32 = 524288;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct POINTER_INFO {
    pub pointerType: POINTER_INPUT_TYPE,
    pub pointerId: u32,
    pub frameId: u32,
    pub pointerFlags: POINTER_FLAGS,
    pub sourceDevice: super::HANDLE,
    pub hwndTarget: super::HWND,
    pub ptPixelLocation: super::POINT,
    pub ptHimetricLocation: super::POINT,
    pub ptPixelLocationRaw: super::POINT,
    pub ptHimetricLocationRaw: super::POINT,
    pub dwTime: u32,
    pub historyCount: u32,
    pub InputData: i32,
    pub dwKeyStates: u32,
    pub PerformanceCount: u64,
    pub ButtonChangeType: POINTER_BUTTON_CHANGE_TYPE,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for POINTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type POINTER_INPUT_TYPE = u32;
pub const POINTER_MESSAGE_FLAG_CANCELED: u32 = 32768;
pub const POINTER_MESSAGE_FLAG_CONFIDENCE: u32 = 16384;
pub const POINTER_MESSAGE_FLAG_FIFTHBUTTON: u32 = 256;
pub const POINTER_MESSAGE_FLAG_FIRSTBUTTON: u32 = 16;
pub const POINTER_MESSAGE_FLAG_FOURTHBUTTON: u32 = 128;
pub const POINTER_MESSAGE_FLAG_INCONTACT: u32 = 4;
pub const POINTER_MESSAGE_FLAG_INRANGE: u32 = 2;
pub const POINTER_MESSAGE_FLAG_NEW: u32 = 1;
pub const POINTER_MESSAGE_FLAG_PRIMARY: u32 = 8192;
pub const POINTER_MESSAGE_FLAG_SECONDBUTTON: u32 = 32;
pub const POINTER_MESSAGE_FLAG_THIRDBUTTON: u32 = 64;
pub const POINTER_MOD_CTRL: u32 = 8;
pub const POINTER_MOD_SHIFT: u32 = 4;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct POINTER_PEN_INFO {
    pub pointerInfo: POINTER_INFO,
    pub penFlags: PEN_FLAGS,
    pub penMask: PEN_MASK,
    pub pressure: u32,
    pub rotation: u32,
    pub tiltX: i32,
    pub tiltY: i32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct POINTER_TOUCH_INFO {
    pub pointerInfo: POINTER_INFO,
    pub touchFlags: TOUCH_FLAGS,
    pub touchMask: TOUCH_MASK,
    pub rcContact: super::RECT,
    pub rcContactRaw: super::RECT,
    pub orientation: u32,
    pub pressure: u32,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct POINTER_TYPE_INFO {
    pub r#type: POINTER_INPUT_TYPE,
    pub Anonymous: POINTER_TYPE_INFO_0,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for POINTER_TYPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union POINTER_TYPE_INFO_0 {
    pub pointerInfo: POINTER_INFO,
    pub touchInfo: POINTER_TOUCH_INFO,
    pub penInfo: POINTER_PEN_INFO,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for POINTER_TYPE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POWERBROADCAST_SETTING {
    pub PowerSetting: windows_sys::core::GUID,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
impl Default for POWERBROADCAST_SETTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
pub type PPAINTSTRUCT = *mut PAINTSTRUCT;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PPOINTER_TYPE_INFO = *mut POINTER_TYPE_INFO;
pub type PPOWERBROADCAST_SETTING = *mut POWERBROADCAST_SETTING;
pub type PRAWHID = *mut RAWHID;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PRAWINPUT = *mut RAWINPUT;
#[cfg(feature = "windef")]
pub type PRAWINPUTDEVICE = *mut RAWINPUTDEVICE;
#[cfg(feature = "winnt")]
pub type PRAWINPUTDEVICELIST = *mut RAWINPUTDEVICELIST;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PRAWINPUTHEADER = *mut RAWINPUTHEADER;
pub type PRAWKEYBOARD = *mut RAWKEYBOARD;
pub type PRAWMOUSE = *mut RAWMOUSE;
pub type PREGISTERCLASSNAMEW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR) -> bool>;
pub const PRF_CHECKVISIBLE: u32 = 1;
pub const PRF_CHILDREN: u32 = 16;
pub const PRF_CLIENT: u32 = 4;
pub const PRF_ERASEBKGND: u32 = 8;
pub const PRF_NONCLIENT: u32 = 2;
pub const PRF_OWNED: u32 = 32;
pub type PRID_DEVICE_INFO = *mut RID_DEVICE_INFO;
pub type PRID_DEVICE_INFO_HID = *mut RID_DEVICE_INFO_HID;
pub type PRID_DEVICE_INFO_KEYBOARD = *mut RID_DEVICE_INFO_KEYBOARD;
pub type PRID_DEVICE_INFO_MOUSE = *mut RID_DEVICE_INFO_MOUSE;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PROPENUMPROC = PROPENUMPROCA;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PROPENUMPROCA = Option<unsafe extern "system" fn(param0: super::HWND, param1: windows_sys::core::PCSTR, param2: super::HANDLE) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PROPENUMPROCEX = PROPENUMPROCEXA;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PROPENUMPROCEXA = Option<unsafe extern "system" fn(param0: super::HWND, param1: windows_sys::core::PCSTR, param2: super::HANDLE, param3: usize) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PROPENUMPROCEXW = Option<unsafe extern "system" fn(param0: super::HWND, param1: windows_sys::core::PCWSTR, param2: super::HANDLE, param3: usize) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PROPENUMPROCW = Option<unsafe extern "system" fn(param0: super::HWND, param1: windows_sys::core::PCWSTR, param2: super::HANDLE) -> windows_sys::core::BOOL>;
#[cfg(feature = "windef")]
pub type PSCROLLBARINFO = *mut SCROLLBARINFO;
#[cfg(feature = "windef")]
pub type PTITLEBARINFO = *mut TITLEBARINFO;
#[cfg(feature = "windef")]
pub type PTITLEBARINFOEX = *mut TITLEBARINFOEX;
#[cfg(feature = "winnt")]
pub type PTOUCHINPUT = *mut TOUCHINPUT;
pub type PTOUCHPAD_PARAMETERS_V1 = *mut TOUCHPAD_PARAMETERS_V1;
pub type PTOUCHPAD_PARAMETERS_V2 = *mut TOUCHPAD_PARAMETERS_V2;
pub type PTOUCHPREDICTIONPARAMETERS = *mut TOUCHPREDICTIONPARAMETERS;
#[cfg(feature = "windef")]
pub type PTOUCH_HIT_TESTING_INPUT = *mut TOUCH_HIT_TESTING_INPUT;
#[cfg(feature = "windef")]
pub type PTOUCH_HIT_TESTING_PROXIMITY_EVALUATION = *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION;
pub const PT_MOUSE: tagPOINTER_INPUT_TYPE = 4;
pub const PT_PEN: tagPOINTER_INPUT_TYPE = 3;
pub const PT_POINTER: tagPOINTER_INPUT_TYPE = 1;
pub const PT_TOUCH: tagPOINTER_INPUT_TYPE = 2;
pub const PT_TOUCHPAD: tagPOINTER_INPUT_TYPE = 5;
#[cfg(all(feature = "windef", feature = "wingdi"))]
pub type PUPDATELAYEREDWINDOWINFO = *mut UPDATELAYEREDWINDOWINFO;
pub type PUSAGE_PROPERTIES = *mut USAGE_PROPERTIES;
pub type PUSEROBJECTFLAGS = *mut USEROBJECTFLAGS;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PWINDOWINFO = *mut WINDOWINFO;
#[cfg(feature = "windef")]
pub type PWINDOWPLACEMENT = *mut WINDOWPLACEMENT;
#[cfg(feature = "windef")]
pub type PWINDOWPOS = *mut WINDOWPOS;
#[cfg(feature = "windef")]
pub type PWINDOW_ACTION = *mut WINDOW_ACTION;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PWNDCLASS = PWNDCLASSA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PWNDCLASSA = *mut WNDCLASSA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PWNDCLASSEX = PWNDCLASSEXA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PWNDCLASSEXA = *mut WNDCLASSEXA;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PWNDCLASSEXW = *mut WNDCLASSEXW;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PWNDCLASSW = *mut WNDCLASSW;
pub const PWR_CRITICALRESUME: u32 = 3;
pub const PWR_FAIL: i32 = -1;
pub const PWR_OK: u32 = 1;
pub const PWR_SUSPENDREQUEST: u32 = 1;
pub const PWR_SUSPENDRESUME: u32 = 2;
pub type PWTSSESSION_NOTIFICATION = *mut WTSSESSION_NOTIFICATION;
pub const PW_CLIENTONLY: u32 = 1;
pub const PW_RENDERFULLCONTENT: u32 = 2;
pub const QS_ALLEVENTS: u32 = 7359;
pub const QS_ALLINPUT: u32 = 7423;
pub const QS_ALLPOSTMESSAGE: u32 = 256;
pub const QS_HOTKEY: u32 = 128;
pub const QS_INPUT: u32 = 7175;
pub const QS_KEY: u32 = 1;
pub const QS_MOUSE: u32 = 6;
pub const QS_MOUSEBUTTON: u32 = 4;
pub const QS_MOUSEMOVE: u32 = 2;
pub const QS_PAINT: u32 = 32;
pub const QS_POINTER: u32 = 4096;
pub const QS_POSTMESSAGE: u32 = 8;
pub const QS_RAWINPUT: u32 = 1024;
pub const QS_SENDMESSAGE: u32 = 64;
pub const QS_TIMER: u32 = 16;
pub const QS_TOUCH: u32 = 2048;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RAWHID {
    pub dwSizeHid: u32,
    pub dwCount: u32,
    pub bRawData: [u8; 1],
}
impl Default for RAWHID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RAWINPUT {
    pub header: RAWINPUTHEADER,
    pub data: RAWINPUT_0,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for RAWINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union RAWINPUT_0 {
    pub mouse: RAWMOUSE,
    pub keyboard: RAWKEYBOARD,
    pub hid: RAWHID,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for RAWINPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct RAWINPUTDEVICE {
    pub usUsagePage: u16,
    pub usUsage: u16,
    pub dwFlags: u32,
    pub hwndTarget: super::HWND,
}
#[cfg(feature = "windef")]
impl Default for RAWINPUTDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct RAWINPUTDEVICELIST {
    pub hDevice: super::HANDLE,
    pub dwType: u32,
}
#[cfg(feature = "winnt")]
impl Default for RAWINPUTDEVICELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RAWINPUTHEADER {
    pub dwType: u32,
    pub dwSize: u32,
    pub hDevice: super::HANDLE,
    pub wParam: super::WPARAM,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for RAWINPUTHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RAWKEYBOARD {
    pub MakeCode: u16,
    pub Flags: u16,
    pub Reserved: u16,
    pub VKey: u16,
    pub Message: u32,
    pub ExtraInformation: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RAWMOUSE {
    pub usFlags: u16,
    pub Anonymous: RAWMOUSE_0,
    pub ulRawButtons: u32,
    pub lLastX: i32,
    pub lLastY: i32,
    pub ulExtraInformation: u32,
}
impl Default for RAWMOUSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RAWMOUSE_0 {
    pub ulButtons: u32,
    pub Anonymous: RAWMOUSE_0_0,
}
impl Default for RAWMOUSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RAWMOUSE_0_0 {
    pub usButtonFlags: u16,
    pub usButtonData: u16,
}
pub const RDW_ALLCHILDREN: u32 = 128;
pub const RDW_ERASE: u32 = 4;
pub const RDW_ERASENOW: u32 = 512;
pub const RDW_FRAME: u32 = 1024;
pub const RDW_INTERNALPAINT: u32 = 2;
pub const RDW_INVALIDATE: u32 = 1;
pub const RDW_NOCHILDREN: u32 = 64;
pub const RDW_NOERASE: u32 = 32;
pub const RDW_NOFRAME: u32 = 2048;
pub const RDW_NOINTERNALPAINT: u32 = 16;
pub const RDW_UPDATENOW: u32 = 256;
pub const RDW_VALIDATE: u32 = 8;
pub const RES_CURSOR: u32 = 2;
pub const RES_ICON: u32 = 1;
pub const RIDEV_APPKEYS: u32 = 1024;
pub const RIDEV_CAPTUREMOUSE: u32 = 512;
pub const RIDEV_DEVNOTIFY: u32 = 8192;
pub const RIDEV_EXCLUDE: u32 = 16;
pub const RIDEV_EXINPUTSINK: u32 = 4096;
pub const RIDEV_EXMODEMASK: u32 = 240;
pub const RIDEV_INPUTSINK: u32 = 256;
pub const RIDEV_NOHOTKEYS: u32 = 512;
pub const RIDEV_NOLEGACY: u32 = 48;
pub const RIDEV_PAGEONLY: u32 = 32;
pub const RIDEV_REMOVE: u32 = 1;
pub const RIDI_DEVICEINFO: u32 = 536870923;
pub const RIDI_DEVICENAME: u32 = 536870919;
pub const RIDI_PREPARSEDDATA: u32 = 536870917;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RID_DEVICE_INFO {
    pub cbSize: u32,
    pub dwType: u32,
    pub Anonymous: RID_DEVICE_INFO_0,
}
impl Default for RID_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RID_DEVICE_INFO_0 {
    pub mouse: RID_DEVICE_INFO_MOUSE,
    pub keyboard: RID_DEVICE_INFO_KEYBOARD,
    pub hid: RID_DEVICE_INFO_HID,
}
impl Default for RID_DEVICE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RID_DEVICE_INFO_HID {
    pub dwVendorId: u32,
    pub dwProductId: u32,
    pub dwVersionNumber: u32,
    pub usUsagePage: u16,
    pub usUsage: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RID_DEVICE_INFO_KEYBOARD {
    pub dwType: u32,
    pub dwSubType: u32,
    pub dwKeyboardMode: u32,
    pub dwNumberOfFunctionKeys: u32,
    pub dwNumberOfIndicators: u32,
    pub dwNumberOfKeysTotal: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RID_DEVICE_INFO_MOUSE {
    pub dwId: u32,
    pub dwNumberOfButtons: u32,
    pub dwSampleRate: u32,
    pub fHasHorizontalWheel: windows_sys::core::BOOL,
}
pub const RID_HEADER: u32 = 268435461;
pub const RID_INPUT: u32 = 268435459;
pub const RIM_INPUT: u32 = 0;
pub const RIM_INPUTSINK: u32 = 1;
pub const RIM_TYPEHID: u32 = 2;
pub const RIM_TYPEKEYBOARD: u32 = 1;
pub const RIM_TYPEMAX: u32 = 2;
pub const RIM_TYPEMOUSE: u32 = 0;
pub const RI_KEY_BREAK: u32 = 1;
pub const RI_KEY_E0: u32 = 2;
pub const RI_KEY_E1: u32 = 4;
pub const RI_KEY_MAKE: u32 = 0;
pub const RI_KEY_TERMSRV_SET_LED: u32 = 8;
pub const RI_KEY_TERMSRV_SHADOW: u32 = 16;
pub const RI_MOUSE_BUTTON_1_DOWN: u32 = 1;
pub const RI_MOUSE_BUTTON_1_UP: u32 = 2;
pub const RI_MOUSE_BUTTON_2_DOWN: u32 = 4;
pub const RI_MOUSE_BUTTON_2_UP: u32 = 8;
pub const RI_MOUSE_BUTTON_3_DOWN: u32 = 16;
pub const RI_MOUSE_BUTTON_3_UP: u32 = 32;
pub const RI_MOUSE_BUTTON_4_DOWN: u32 = 64;
pub const RI_MOUSE_BUTTON_4_UP: u32 = 128;
pub const RI_MOUSE_BUTTON_5_DOWN: u32 = 256;
pub const RI_MOUSE_BUTTON_5_UP: u32 = 512;
pub const RI_MOUSE_HWHEEL: u32 = 2048;
pub const RI_MOUSE_LEFT_BUTTON_DOWN: u32 = 1;
pub const RI_MOUSE_LEFT_BUTTON_UP: u32 = 2;
pub const RI_MOUSE_MIDDLE_BUTTON_DOWN: u32 = 16;
pub const RI_MOUSE_MIDDLE_BUTTON_UP: u32 = 32;
pub const RI_MOUSE_RIGHT_BUTTON_DOWN: u32 = 4;
pub const RI_MOUSE_RIGHT_BUTTON_UP: u32 = 8;
pub const RI_MOUSE_WHEEL: u32 = 1024;
pub const RT_ACCELERATOR: windows_sys::core::PCWSTR = 9 as _;
pub const RT_ANICURSOR: windows_sys::core::PCWSTR = 21 as _;
pub const RT_ANIICON: windows_sys::core::PCWSTR = 22 as _;
pub const RT_BITMAP: windows_sys::core::PCWSTR = 2 as _;
pub const RT_CURSOR: windows_sys::core::PCWSTR = 1 as _;
pub const RT_DIALOG: windows_sys::core::PCWSTR = 5 as _;
pub const RT_DLGINCLUDE: windows_sys::core::PCWSTR = 17 as _;
pub const RT_FONT: windows_sys::core::PCWSTR = 8 as _;
pub const RT_FONTDIR: windows_sys::core::PCWSTR = 7 as _;
pub const RT_HTML: windows_sys::core::PCWSTR = 23 as _;
pub const RT_ICON: windows_sys::core::PCWSTR = 3 as _;
pub const RT_MANIFEST: windows_sys::core::PCWSTR = 24 as _;
pub const RT_MENU: windows_sys::core::PCWSTR = 4 as _;
pub const RT_MESSAGETABLE: windows_sys::core::PCWSTR = 11 as _;
pub const RT_PLUGPLAY: windows_sys::core::PCWSTR = 19 as _;
pub const RT_RCDATA: windows_sys::core::PCWSTR = 10 as _;
pub const RT_STRING: windows_sys::core::PCWSTR = 6 as _;
pub const RT_VERSION: windows_sys::core::PCWSTR = 16 as _;
pub const RT_VXD: windows_sys::core::PCWSTR = 20 as _;
pub const SBM_ENABLE_ARROWS: u32 = 228;
pub const SBM_GETPOS: u32 = 225;
pub const SBM_GETRANGE: u32 = 227;
pub const SBM_GETSCROLLBARINFO: u32 = 235;
pub const SBM_GETSCROLLINFO: u32 = 234;
pub const SBM_SETPOS: u32 = 224;
pub const SBM_SETRANGE: u32 = 226;
pub const SBM_SETRANGEREDRAW: u32 = 230;
pub const SBM_SETSCROLLINFO: u32 = 233;
pub const SBS_BOTTOMALIGN: u32 = 4;
pub const SBS_HORZ: u32 = 0;
pub const SBS_LEFTALIGN: u32 = 2;
pub const SBS_RIGHTALIGN: u32 = 4;
pub const SBS_SIZEBOX: u32 = 8;
pub const SBS_SIZEBOXBOTTOMRIGHTALIGN: u32 = 4;
pub const SBS_SIZEBOXTOPLEFTALIGN: u32 = 2;
pub const SBS_SIZEGRIP: u32 = 16;
pub const SBS_TOPALIGN: u32 = 2;
pub const SBS_VERT: u32 = 1;
pub const SB_BOTH: u32 = 3;
pub const SB_BOTTOM: u32 = 7;
pub const SB_CTL: u32 = 2;
pub const SB_ENDSCROLL: u32 = 8;
pub const SB_HORZ: u32 = 0;
pub const SB_LEFT: u32 = 6;
pub const SB_LINEDOWN: u32 = 1;
pub const SB_LINELEFT: u32 = 0;
pub const SB_LINERIGHT: u32 = 1;
pub const SB_LINEUP: u32 = 0;
pub const SB_MIN: u32 = 0;
pub const SB_PAGEDOWN: u32 = 3;
pub const SB_PAGELEFT: u32 = 2;
pub const SB_PAGERIGHT: u32 = 3;
pub const SB_PAGEUP: u32 = 2;
pub const SB_RIGHT: u32 = 7;
pub const SB_THUMBPOSITION: u32 = 4;
pub const SB_THUMBTRACK: u32 = 5;
pub const SB_TOP: u32 = 6;
pub const SB_VERT: u32 = 1;
pub const SCF_ISSECURE: u32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct SCROLLBARINFO {
    pub cbSize: u32,
    pub rcScrollBar: super::RECT,
    pub dxyLineButton: i32,
    pub xyThumbTop: i32,
    pub xyThumbBottom: i32,
    pub reserved: i32,
    pub rgstate: [u32; 6],
}
#[cfg(feature = "windef")]
impl Default for SCROLLBARINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SCROLLINFO {
    pub cbSize: u32,
    pub fMask: u32,
    pub nMin: i32,
    pub nMax: i32,
    pub nPage: u32,
    pub nPos: i32,
    pub nTrackPos: i32,
}
pub const SC_ARRANGE: u32 = 61712;
pub const SC_CLOSE: u32 = 61536;
pub const SC_CONTEXTHELP: u32 = 61824;
pub const SC_DEFAULT: u32 = 61792;
pub const SC_HOTKEY: u32 = 61776;
pub const SC_HSCROLL: u32 = 61568;
pub const SC_ICON: u32 = 61472;
pub const SC_KEYMENU: u32 = 61696;
pub const SC_MAXIMIZE: u32 = 61488;
pub const SC_MINIMIZE: u32 = 61472;
pub const SC_MONITORPOWER: u32 = 61808;
pub const SC_MOUSEMENU: u32 = 61584;
pub const SC_MOVE: u32 = 61456;
pub const SC_NEXTWINDOW: u32 = 61504;
pub const SC_PREVWINDOW: u32 = 61520;
pub const SC_RESTORE: u32 = 61728;
pub const SC_SCREENSAVE: u32 = 61760;
pub const SC_SEPARATOR: u32 = 61455;
pub const SC_SIZE: u32 = 61440;
pub const SC_TASKLIST: u32 = 61744;
pub const SC_VSCROLL: u32 = 61552;
pub const SC_ZOOM: u32 = 61488;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type SENDASYNCPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: u32, param2: usize, param3: super::LRESULT)>;
pub type SERIALKEYS = SERIALKEYSA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERIALKEYSA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpszActivePort: windows_sys::core::PSTR,
    pub lpszPort: windows_sys::core::PSTR,
    pub iBaudRate: u32,
    pub iPortState: u32,
    pub iActive: u32,
}
impl Default for SERIALKEYSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERIALKEYSW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpszActivePort: windows_sys::core::PWSTR,
    pub lpszPort: windows_sys::core::PWSTR,
    pub iBaudRate: u32,
    pub iPortState: u32,
    pub iActive: u32,
}
impl Default for SERIALKEYSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERKF_AVAILABLE: u32 = 2;
pub const SERKF_INDICATOR: u32 = 4;
pub const SERKF_SERIALKEYSON: u32 = 1;
pub const SETWALLPAPER_DEFAULT: windows_sys::core::PCWSTR = -1 as _;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct SHELLHOOKINFO {
    pub hwnd: super::HWND,
    pub rc: super::RECT,
}
#[cfg(feature = "windef")]
impl Default for SHELLHOOKINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SHOW_FULLSCREEN: u32 = 3;
pub const SHOW_ICONWINDOW: u32 = 2;
pub const SHOW_OPENNOACTIVATE: u32 = 4;
pub const SHOW_OPENWINDOW: u32 = 1;
pub const SIF_ALL: u32 = 23;
pub const SIF_DISABLENOSCROLL: u32 = 8;
pub const SIF_PAGE: u32 = 2;
pub const SIF_POS: u32 = 4;
pub const SIF_RANGE: u32 = 1;
pub const SIF_TRACKPOS: u32 = 16;
pub const SIZEFULLSCREEN: u32 = 2;
pub const SIZEICONIC: u32 = 1;
pub const SIZENORMAL: u32 = 0;
pub const SIZEZOOMHIDE: u32 = 4;
pub const SIZEZOOMSHOW: u32 = 3;
pub const SIZE_MAXHIDE: u32 = 4;
pub const SIZE_MAXIMIZED: u32 = 2;
pub const SIZE_MAXSHOW: u32 = 3;
pub const SIZE_MINIMIZED: u32 = 1;
pub const SIZE_RESTORED: u32 = 0;
pub const SKF_AUDIBLEFEEDBACK: u32 = 64;
pub const SKF_AVAILABLE: u32 = 2;
pub const SKF_CONFIRMHOTKEY: u32 = 8;
pub const SKF_HOTKEYACTIVE: u32 = 4;
pub const SKF_HOTKEYSOUND: u32 = 16;
pub const SKF_INDICATOR: u32 = 32;
pub const SKF_LALTLATCHED: u32 = 268435456;
pub const SKF_LALTLOCKED: u32 = 1048576;
pub const SKF_LCTLLATCHED: u32 = 67108864;
pub const SKF_LCTLLOCKED: u32 = 262144;
pub const SKF_LSHIFTLATCHED: u32 = 16777216;
pub const SKF_LSHIFTLOCKED: u32 = 65536;
pub const SKF_LWINLATCHED: u32 = 1073741824;
pub const SKF_LWINLOCKED: u32 = 4194304;
pub const SKF_RALTLATCHED: u32 = 536870912;
pub const SKF_RALTLOCKED: u32 = 2097152;
pub const SKF_RCTLLATCHED: u32 = 134217728;
pub const SKF_RCTLLOCKED: u32 = 524288;
pub const SKF_RSHIFTLATCHED: u32 = 33554432;
pub const SKF_RSHIFTLOCKED: u32 = 131072;
pub const SKF_RWINLATCHED: u32 = 2147483648;
pub const SKF_RWINLOCKED: u32 = 8388608;
pub const SKF_STICKYKEYSON: u32 = 1;
pub const SKF_TRISTATE: u32 = 128;
pub const SKF_TWOKEYSOFF: u32 = 256;
pub const SLE_ERROR: u32 = 1;
pub const SLE_MINORERROR: u32 = 2;
pub const SLE_WARNING: u32 = 3;
pub const SMTO_ABORTIFHUNG: u32 = 2;
pub const SMTO_BLOCK: u32 = 1;
pub const SMTO_ERRORONEXIT: u32 = 32;
pub const SMTO_NORMAL: u32 = 0;
pub const SMTO_NOTIMEOUTIFNOTHUNG: u32 = 8;
pub const SM_ARRANGE: u32 = 56;
pub const SM_CARETBLINKINGENABLED: u32 = 8194;
pub const SM_CLEANBOOT: u32 = 67;
pub const SM_CMETRICS: u32 = 97;
pub const SM_CMONITORS: u32 = 80;
pub const SM_CMOUSEBUTTONS: u32 = 43;
pub const SM_CONVERTIBLESLATEMODE: u32 = 8195;
pub const SM_CXBORDER: u32 = 5;
pub const SM_CXCURSOR: u32 = 13;
pub const SM_CXDLGFRAME: u32 = 7;
pub const SM_CXDOUBLECLK: u32 = 36;
pub const SM_CXDRAG: u32 = 68;
pub const SM_CXEDGE: u32 = 45;
pub const SM_CXFIXEDFRAME: u32 = 7;
pub const SM_CXFOCUSBORDER: u32 = 83;
pub const SM_CXFRAME: u32 = 32;
pub const SM_CXFULLSCREEN: u32 = 16;
pub const SM_CXHSCROLL: u32 = 21;
pub const SM_CXHTHUMB: u32 = 10;
pub const SM_CXICON: u32 = 11;
pub const SM_CXICONSPACING: u32 = 38;
pub const SM_CXMAXIMIZED: u32 = 61;
pub const SM_CXMAXTRACK: u32 = 59;
pub const SM_CXMENUCHECK: u32 = 71;
pub const SM_CXMENUSIZE: u32 = 54;
pub const SM_CXMIN: u32 = 28;
pub const SM_CXMINIMIZED: u32 = 57;
pub const SM_CXMINSPACING: u32 = 47;
pub const SM_CXMINTRACK: u32 = 34;
pub const SM_CXPADDEDBORDER: u32 = 92;
pub const SM_CXSCREEN: u32 = 0;
pub const SM_CXSIZE: u32 = 30;
pub const SM_CXSIZEFRAME: u32 = 32;
pub const SM_CXSMICON: u32 = 49;
pub const SM_CXSMSIZE: u32 = 52;
pub const SM_CXVIRTUALSCREEN: u32 = 78;
pub const SM_CXVSCROLL: u32 = 2;
pub const SM_CYBORDER: u32 = 6;
pub const SM_CYCAPTION: u32 = 4;
pub const SM_CYCURSOR: u32 = 14;
pub const SM_CYDLGFRAME: u32 = 8;
pub const SM_CYDOUBLECLK: u32 = 37;
pub const SM_CYDRAG: u32 = 69;
pub const SM_CYEDGE: u32 = 46;
pub const SM_CYFIXEDFRAME: u32 = 8;
pub const SM_CYFOCUSBORDER: u32 = 84;
pub const SM_CYFRAME: u32 = 33;
pub const SM_CYFULLSCREEN: u32 = 17;
pub const SM_CYHSCROLL: u32 = 3;
pub const SM_CYICON: u32 = 12;
pub const SM_CYICONSPACING: u32 = 39;
pub const SM_CYKANJIWINDOW: u32 = 18;
pub const SM_CYMAXIMIZED: u32 = 62;
pub const SM_CYMAXTRACK: u32 = 60;
pub const SM_CYMENU: u32 = 15;
pub const SM_CYMENUCHECK: u32 = 72;
pub const SM_CYMENUSIZE: u32 = 55;
pub const SM_CYMIN: u32 = 29;
pub const SM_CYMINIMIZED: u32 = 58;
pub const SM_CYMINSPACING: u32 = 48;
pub const SM_CYMINTRACK: u32 = 35;
pub const SM_CYSCREEN: u32 = 1;
pub const SM_CYSIZE: u32 = 31;
pub const SM_CYSIZEFRAME: u32 = 33;
pub const SM_CYSMCAPTION: u32 = 51;
pub const SM_CYSMICON: u32 = 50;
pub const SM_CYSMSIZE: u32 = 53;
pub const SM_CYVIRTUALSCREEN: u32 = 79;
pub const SM_CYVSCROLL: u32 = 20;
pub const SM_CYVTHUMB: u32 = 9;
pub const SM_DBCSENABLED: u32 = 42;
pub const SM_DEBUG: u32 = 22;
pub const SM_DIGITIZER: u32 = 94;
pub const SM_IMMENABLED: u32 = 82;
pub const SM_MAXIMUMTOUCHES: u32 = 95;
pub const SM_MEDIACENTER: u32 = 87;
pub const SM_MENUDROPALIGNMENT: u32 = 40;
pub const SM_MIDEASTENABLED: u32 = 74;
pub const SM_MOUSEHORIZONTALWHEELPRESENT: u32 = 91;
pub const SM_MOUSEPRESENT: u32 = 19;
pub const SM_MOUSEWHEELPRESENT: u32 = 75;
pub const SM_NETWORK: u32 = 63;
pub const SM_PENWINDOWS: u32 = 41;
pub const SM_REMOTECONTROL: u32 = 8193;
pub const SM_REMOTESESSION: u32 = 4096;
pub const SM_RESERVED1: u32 = 24;
pub const SM_RESERVED2: u32 = 25;
pub const SM_RESERVED3: u32 = 26;
pub const SM_RESERVED4: u32 = 27;
pub const SM_SAMEDISPLAYFORMAT: u32 = 81;
pub const SM_SECURE: u32 = 44;
pub const SM_SERVERR2: u32 = 89;
pub const SM_SHOWSOUNDS: u32 = 70;
pub const SM_SHUTTINGDOWN: u32 = 8192;
pub const SM_SLOWMACHINE: u32 = 73;
pub const SM_STARTER: u32 = 88;
pub const SM_SWAPBUTTON: u32 = 23;
pub const SM_SYSTEMDOCKED: u32 = 8196;
pub const SM_TABLETPC: u32 = 86;
pub const SM_XVIRTUALSCREEN: u32 = 76;
pub const SM_YVIRTUALSCREEN: u32 = 77;
pub type SOUNDSENTRY = SOUNDSENTRYA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SOUNDSENTRYA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iFSTextEffect: u32,
    pub iFSTextEffectMSec: u32,
    pub iFSTextEffectColorBits: u32,
    pub iFSGrafEffect: u32,
    pub iFSGrafEffectMSec: u32,
    pub iFSGrafEffectColor: u32,
    pub iWindowsEffect: u32,
    pub iWindowsEffectMSec: u32,
    pub lpszWindowsEffectDLL: windows_sys::core::PSTR,
    pub iWindowsEffectOrdinal: u32,
}
impl Default for SOUNDSENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SOUNDSENTRYW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iFSTextEffect: u32,
    pub iFSTextEffectMSec: u32,
    pub iFSTextEffectColorBits: u32,
    pub iFSGrafEffect: u32,
    pub iFSGrafEffectMSec: u32,
    pub iFSGrafEffectColor: u32,
    pub iWindowsEffect: u32,
    pub iWindowsEffectMSec: u32,
    pub lpszWindowsEffectDLL: windows_sys::core::PWSTR,
    pub iWindowsEffectOrdinal: u32,
}
impl Default for SOUNDSENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SOUND_SYSTEM_APPEND: u32 = 14;
pub const SOUND_SYSTEM_APPSTART: u32 = 12;
pub const SOUND_SYSTEM_BEEP: u32 = 3;
pub const SOUND_SYSTEM_ERROR: u32 = 4;
pub const SOUND_SYSTEM_FAULT: u32 = 13;
pub const SOUND_SYSTEM_INFORMATION: u32 = 7;
pub const SOUND_SYSTEM_MAXIMIZE: u32 = 8;
pub const SOUND_SYSTEM_MENUCOMMAND: u32 = 15;
pub const SOUND_SYSTEM_MENUPOPUP: u32 = 16;
pub const SOUND_SYSTEM_MINIMIZE: u32 = 9;
pub const SOUND_SYSTEM_QUESTION: u32 = 5;
pub const SOUND_SYSTEM_RESTOREDOWN: u32 = 11;
pub const SOUND_SYSTEM_RESTOREUP: u32 = 10;
pub const SOUND_SYSTEM_SHUTDOWN: u32 = 2;
pub const SOUND_SYSTEM_STARTUP: u32 = 1;
pub const SOUND_SYSTEM_WARNING: u32 = 6;
pub const SPIF_SENDCHANGE: u32 = 2;
pub const SPIF_SENDWININICHANGE: u32 = 2;
pub const SPIF_UPDATEINIFILE: u32 = 1;
pub const SPI_GETACCESSTIMEOUT: u32 = 60;
pub const SPI_GETACTIVEWINDOWTRACKING: u32 = 4096;
pub const SPI_GETACTIVEWNDTRKTIMEOUT: u32 = 8194;
pub const SPI_GETACTIVEWNDTRKZORDER: u32 = 4108;
pub const SPI_GETANIMATION: u32 = 72;
pub const SPI_GETAUDIODESCRIPTION: u32 = 116;
pub const SPI_GETBEEP: u32 = 1;
pub const SPI_GETBLOCKSENDINPUTRESETS: u32 = 4134;
pub const SPI_GETBORDER: u32 = 5;
pub const SPI_GETCARETBROWSING: u32 = 4172;
pub const SPI_GETCARETTIMEOUT: u32 = 8226;
pub const SPI_GETCARETWIDTH: u32 = 8198;
pub const SPI_GETCLEARTYPE: u32 = 4168;
pub const SPI_GETCLIENTAREAANIMATION: u32 = 4162;
pub const SPI_GETCOMBOBOXANIMATION: u32 = 4100;
pub const SPI_GETCONTACTVISUALIZATION: u32 = 8216;
pub const SPI_GETCURSORSHADOW: u32 = 4122;
pub const SPI_GETDEFAULTINPUTLANG: u32 = 89;
pub const SPI_GETDESKWALLPAPER: u32 = 115;
pub const SPI_GETDISABLEOVERLAPPEDCONTENT: u32 = 4160;
pub const SPI_GETDOCKMOVING: u32 = 144;
pub const SPI_GETDRAGFROMMAXIMIZE: u32 = 140;
pub const SPI_GETDRAGFULLWINDOWS: u32 = 38;
pub const SPI_GETDROPSHADOW: u32 = 4132;
pub const SPI_GETFASTTASKSWITCH: u32 = 35;
pub const SPI_GETFILTERKEYS: u32 = 50;
pub const SPI_GETFLATMENU: u32 = 4130;
pub const SPI_GETFOCUSBORDERHEIGHT: u32 = 8208;
pub const SPI_GETFOCUSBORDERWIDTH: u32 = 8206;
pub const SPI_GETFONTSMOOTHING: u32 = 74;
pub const SPI_GETFONTSMOOTHINGCONTRAST: u32 = 8204;
pub const SPI_GETFONTSMOOTHINGORIENTATION: u32 = 8210;
pub const SPI_GETFONTSMOOTHINGTYPE: u32 = 8202;
pub const SPI_GETFOREGROUNDFLASHCOUNT: u32 = 8196;
pub const SPI_GETFOREGROUNDLOCKTIMEOUT: u32 = 8192;
pub const SPI_GETGESTUREVISUALIZATION: u32 = 8218;
pub const SPI_GETGRADIENTCAPTIONS: u32 = 4104;
pub const SPI_GETGRIDGRANULARITY: u32 = 18;
pub const SPI_GETHANDEDNESS: u32 = 8228;
pub const SPI_GETHIGHCONTRAST: u32 = 66;
pub const SPI_GETHOTTRACKING: u32 = 4110;
pub const SPI_GETHUNGAPPTIMEOUT: u32 = 120;
pub const SPI_GETICONMETRICS: u32 = 45;
pub const SPI_GETICONTITLELOGFONT: u32 = 31;
pub const SPI_GETICONTITLEWRAP: u32 = 25;
pub const SPI_GETKEYBOARDCUES: u32 = 4106;
pub const SPI_GETKEYBOARDDELAY: u32 = 22;
pub const SPI_GETKEYBOARDPREF: u32 = 68;
pub const SPI_GETKEYBOARDSPEED: u32 = 10;
pub const SPI_GETLISTBOXSMOOTHSCROLLING: u32 = 4102;
pub const SPI_GETLOGICALDPIOVERRIDE: u32 = 158;
pub const SPI_GETLOWPOWERACTIVE: u32 = 83;
pub const SPI_GETLOWPOWERTIMEOUT: u32 = 79;
pub const SPI_GETMENUANIMATION: u32 = 4098;
pub const SPI_GETMENUDROPALIGNMENT: u32 = 27;
pub const SPI_GETMENUFADE: u32 = 4114;
pub const SPI_GETMENURECT: u32 = 162;
pub const SPI_GETMENUSHOWDELAY: u32 = 106;
pub const SPI_GETMENUUNDERLINES: u32 = 4106;
pub const SPI_GETMESSAGEDURATION: u32 = 8214;
pub const SPI_GETMINIMIZEDMETRICS: u32 = 43;
pub const SPI_GETMINIMUMHITRADIUS: u32 = 8212;
pub const SPI_GETMOUSE: u32 = 3;
pub const SPI_GETMOUSECLICKLOCK: u32 = 4126;
pub const SPI_GETMOUSECLICKLOCKTIME: u32 = 8200;
pub const SPI_GETMOUSEDOCKTHRESHOLD: u32 = 126;
pub const SPI_GETMOUSEDRAGOUTTHRESHOLD: u32 = 132;
pub const SPI_GETMOUSEHOVERHEIGHT: u32 = 100;
pub const SPI_GETMOUSEHOVERTIME: u32 = 102;
pub const SPI_GETMOUSEHOVERWIDTH: u32 = 98;
pub const SPI_GETMOUSEKEYS: u32 = 54;
pub const SPI_GETMOUSESIDEMOVETHRESHOLD: u32 = 136;
pub const SPI_GETMOUSESONAR: u32 = 4124;
pub const SPI_GETMOUSESPEED: u32 = 112;
pub const SPI_GETMOUSETRAILS: u32 = 94;
pub const SPI_GETMOUSEVANISH: u32 = 4128;
pub const SPI_GETMOUSEWHEELROUTING: u32 = 8220;
pub const SPI_GETNONCLIENTMETRICS: u32 = 41;
pub const SPI_GETPENARBITRATIONTYPE: u32 = 8224;
pub const SPI_GETPENDOCKTHRESHOLD: u32 = 128;
pub const SPI_GETPENDRAGOUTTHRESHOLD: u32 = 134;
pub const SPI_GETPENSIDEMOVETHRESHOLD: u32 = 138;
pub const SPI_GETPENVISUALIZATION: u32 = 8222;
pub const SPI_GETPOWEROFFACTIVE: u32 = 84;
pub const SPI_GETPOWEROFFTIMEOUT: u32 = 80;
pub const SPI_GETSCREENREADER: u32 = 70;
pub const SPI_GETSCREENSAVEACTIVE: u32 = 16;
pub const SPI_GETSCREENSAVERRUNNING: u32 = 114;
pub const SPI_GETSCREENSAVESECURE: u32 = 118;
pub const SPI_GETSCREENSAVETIMEOUT: u32 = 14;
pub const SPI_GETSELECTIONFADE: u32 = 4116;
pub const SPI_GETSERIALKEYS: u32 = 62;
pub const SPI_GETSHOWIMEUI: u32 = 110;
pub const SPI_GETSHOWSOUNDS: u32 = 56;
pub const SPI_GETSNAPSIZING: u32 = 142;
pub const SPI_GETSNAPTODEFBUTTON: u32 = 95;
pub const SPI_GETSOUNDSENTRY: u32 = 64;
pub const SPI_GETSPEECHRECOGNITION: u32 = 4170;
pub const SPI_GETSTICKYKEYS: u32 = 58;
pub const SPI_GETSYSTEMLANGUAGEBAR: u32 = 4176;
pub const SPI_GETTHREADLOCALINPUTSETTINGS: u32 = 4174;
pub const SPI_GETTOGGLEKEYS: u32 = 52;
pub const SPI_GETTOOLTIPANIMATION: u32 = 4118;
pub const SPI_GETTOOLTIPFADE: u32 = 4120;
pub const SPI_GETTOUCHPADPARAMETERS: u32 = 174;
pub const SPI_GETTOUCHPREDICTIONPARAMETERS: u32 = 156;
pub const SPI_GETUIEFFECTS: u32 = 4158;
pub const SPI_GETWAITTOKILLSERVICETIMEOUT: u32 = 124;
pub const SPI_GETWAITTOKILLTIMEOUT: u32 = 122;
pub const SPI_GETWHEELSCROLLCHARS: u32 = 108;
pub const SPI_GETWHEELSCROLLLINES: u32 = 104;
pub const SPI_GETWINARRANGING: u32 = 130;
pub const SPI_GETWINDOWSEXTENSION: u32 = 92;
pub const SPI_GETWORKAREA: u32 = 48;
pub const SPI_ICONHORIZONTALSPACING: u32 = 13;
pub const SPI_ICONVERTICALSPACING: u32 = 24;
pub const SPI_LANGDRIVER: u32 = 12;
pub const SPI_SCREENSAVERRUNNING: u32 = 97;
pub const SPI_SETACCESSTIMEOUT: u32 = 61;
pub const SPI_SETACTIVEWINDOWTRACKING: u32 = 4097;
pub const SPI_SETACTIVEWNDTRKTIMEOUT: u32 = 8195;
pub const SPI_SETACTIVEWNDTRKZORDER: u32 = 4109;
pub const SPI_SETANIMATION: u32 = 73;
pub const SPI_SETAUDIODESCRIPTION: u32 = 117;
pub const SPI_SETBEEP: u32 = 2;
pub const SPI_SETBLOCKSENDINPUTRESETS: u32 = 4135;
pub const SPI_SETBORDER: u32 = 6;
pub const SPI_SETCARETBROWSING: u32 = 4173;
pub const SPI_SETCARETTIMEOUT: u32 = 8227;
pub const SPI_SETCARETWIDTH: u32 = 8199;
pub const SPI_SETCLEARTYPE: u32 = 4169;
pub const SPI_SETCLIENTAREAANIMATION: u32 = 4163;
pub const SPI_SETCOMBOBOXANIMATION: u32 = 4101;
pub const SPI_SETCONTACTVISUALIZATION: u32 = 8217;
pub const SPI_SETCURSORS: u32 = 87;
pub const SPI_SETCURSORSHADOW: u32 = 4123;
pub const SPI_SETDEFAULTINPUTLANG: u32 = 90;
pub const SPI_SETDESKPATTERN: u32 = 21;
pub const SPI_SETDESKWALLPAPER: u32 = 20;
pub const SPI_SETDISABLEOVERLAPPEDCONTENT: u32 = 4161;
pub const SPI_SETDOCKMOVING: u32 = 145;
pub const SPI_SETDOUBLECLICKTIME: u32 = 32;
pub const SPI_SETDOUBLECLKHEIGHT: u32 = 30;
pub const SPI_SETDOUBLECLKWIDTH: u32 = 29;
pub const SPI_SETDRAGFROMMAXIMIZE: u32 = 141;
pub const SPI_SETDRAGFULLWINDOWS: u32 = 37;
pub const SPI_SETDRAGHEIGHT: u32 = 77;
pub const SPI_SETDRAGWIDTH: u32 = 76;
pub const SPI_SETDROPSHADOW: u32 = 4133;
pub const SPI_SETFASTTASKSWITCH: u32 = 36;
pub const SPI_SETFILTERKEYS: u32 = 51;
pub const SPI_SETFLATMENU: u32 = 4131;
pub const SPI_SETFOCUSBORDERHEIGHT: u32 = 8209;
pub const SPI_SETFOCUSBORDERWIDTH: u32 = 8207;
pub const SPI_SETFONTSMOOTHING: u32 = 75;
pub const SPI_SETFONTSMOOTHINGCONTRAST: u32 = 8205;
pub const SPI_SETFONTSMOOTHINGORIENTATION: u32 = 8211;
pub const SPI_SETFONTSMOOTHINGTYPE: u32 = 8203;
pub const SPI_SETFOREGROUNDFLASHCOUNT: u32 = 8197;
pub const SPI_SETFOREGROUNDLOCKTIMEOUT: u32 = 8193;
pub const SPI_SETGESTUREVISUALIZATION: u32 = 8219;
pub const SPI_SETGRADIENTCAPTIONS: u32 = 4105;
pub const SPI_SETGRIDGRANULARITY: u32 = 19;
pub const SPI_SETHANDEDNESS: u32 = 8229;
pub const SPI_SETHANDHELD: u32 = 78;
pub const SPI_SETHIGHCONTRAST: u32 = 67;
pub const SPI_SETHOTTRACKING: u32 = 4111;
pub const SPI_SETHUNGAPPTIMEOUT: u32 = 121;
pub const SPI_SETICONMETRICS: u32 = 46;
pub const SPI_SETICONS: u32 = 88;
pub const SPI_SETICONTITLELOGFONT: u32 = 34;
pub const SPI_SETICONTITLEWRAP: u32 = 26;
pub const SPI_SETKEYBOARDCUES: u32 = 4107;
pub const SPI_SETKEYBOARDDELAY: u32 = 23;
pub const SPI_SETKEYBOARDPREF: u32 = 69;
pub const SPI_SETKEYBOARDSPEED: u32 = 11;
pub const SPI_SETLANGTOGGLE: u32 = 91;
pub const SPI_SETLISTBOXSMOOTHSCROLLING: u32 = 4103;
pub const SPI_SETLOGICALDPIOVERRIDE: u32 = 159;
pub const SPI_SETLOWPOWERACTIVE: u32 = 85;
pub const SPI_SETLOWPOWERTIMEOUT: u32 = 81;
pub const SPI_SETMENUANIMATION: u32 = 4099;
pub const SPI_SETMENUDROPALIGNMENT: u32 = 28;
pub const SPI_SETMENUFADE: u32 = 4115;
pub const SPI_SETMENURECT: u32 = 163;
pub const SPI_SETMENUSHOWDELAY: u32 = 107;
pub const SPI_SETMENUUNDERLINES: u32 = 4107;
pub const SPI_SETMESSAGEDURATION: u32 = 8215;
pub const SPI_SETMINIMIZEDMETRICS: u32 = 44;
pub const SPI_SETMINIMUMHITRADIUS: u32 = 8213;
pub const SPI_SETMOUSE: u32 = 4;
pub const SPI_SETMOUSEBUTTONSWAP: u32 = 33;
pub const SPI_SETMOUSECLICKLOCK: u32 = 4127;
pub const SPI_SETMOUSECLICKLOCKTIME: u32 = 8201;
pub const SPI_SETMOUSEDOCKTHRESHOLD: u32 = 127;
pub const SPI_SETMOUSEDRAGOUTTHRESHOLD: u32 = 133;
pub const SPI_SETMOUSEHOVERHEIGHT: u32 = 101;
pub const SPI_SETMOUSEHOVERTIME: u32 = 103;
pub const SPI_SETMOUSEHOVERWIDTH: u32 = 99;
pub const SPI_SETMOUSEKEYS: u32 = 55;
pub const SPI_SETMOUSESIDEMOVETHRESHOLD: u32 = 137;
pub const SPI_SETMOUSESONAR: u32 = 4125;
pub const SPI_SETMOUSESPEED: u32 = 113;
pub const SPI_SETMOUSETRAILS: u32 = 93;
pub const SPI_SETMOUSEVANISH: u32 = 4129;
pub const SPI_SETMOUSEWHEELROUTING: u32 = 8221;
pub const SPI_SETNONCLIENTMETRICS: u32 = 42;
pub const SPI_SETPENARBITRATIONTYPE: u32 = 8225;
pub const SPI_SETPENDOCKTHRESHOLD: u32 = 129;
pub const SPI_SETPENDRAGOUTTHRESHOLD: u32 = 135;
pub const SPI_SETPENSIDEMOVETHRESHOLD: u32 = 139;
pub const SPI_SETPENVISUALIZATION: u32 = 8223;
pub const SPI_SETPENWINDOWS: u32 = 49;
pub const SPI_SETPOWEROFFACTIVE: u32 = 86;
pub const SPI_SETPOWEROFFTIMEOUT: u32 = 82;
pub const SPI_SETSCREENREADER: u32 = 71;
pub const SPI_SETSCREENSAVEACTIVE: u32 = 17;
pub const SPI_SETSCREENSAVERRUNNING: u32 = 97;
pub const SPI_SETSCREENSAVESECURE: u32 = 119;
pub const SPI_SETSCREENSAVETIMEOUT: u32 = 15;
pub const SPI_SETSELECTIONFADE: u32 = 4117;
pub const SPI_SETSERIALKEYS: u32 = 63;
pub const SPI_SETSHOWIMEUI: u32 = 111;
pub const SPI_SETSHOWSOUNDS: u32 = 57;
pub const SPI_SETSNAPSIZING: u32 = 143;
pub const SPI_SETSNAPTODEFBUTTON: u32 = 96;
pub const SPI_SETSOUNDSENTRY: u32 = 65;
pub const SPI_SETSPEECHRECOGNITION: u32 = 4171;
pub const SPI_SETSTICKYKEYS: u32 = 59;
pub const SPI_SETSYSTEMLANGUAGEBAR: u32 = 4177;
pub const SPI_SETTHREADLOCALINPUTSETTINGS: u32 = 4175;
pub const SPI_SETTOGGLEKEYS: u32 = 53;
pub const SPI_SETTOOLTIPANIMATION: u32 = 4119;
pub const SPI_SETTOOLTIPFADE: u32 = 4121;
pub const SPI_SETTOUCHPADPARAMETERS: u32 = 175;
pub const SPI_SETTOUCHPREDICTIONPARAMETERS: u32 = 157;
pub const SPI_SETUIEFFECTS: u32 = 4159;
pub const SPI_SETWAITTOKILLSERVICETIMEOUT: u32 = 125;
pub const SPI_SETWAITTOKILLTIMEOUT: u32 = 123;
pub const SPI_SETWHEELSCROLLCHARS: u32 = 109;
pub const SPI_SETWHEELSCROLLLINES: u32 = 105;
pub const SPI_SETWINARRANGING: u32 = 131;
pub const SPI_SETWORKAREA: u32 = 47;
pub const SSF_AVAILABLE: u32 = 2;
pub const SSF_INDICATOR: u32 = 4;
pub const SSF_SOUNDSENTRYON: u32 = 1;
pub const SSGF_DISPLAY: u32 = 3;
pub const SSGF_NONE: u32 = 0;
pub const SSTF_BORDER: u32 = 2;
pub const SSTF_CHARS: u32 = 1;
pub const SSTF_DISPLAY: u32 = 3;
pub const SSTF_NONE: u32 = 0;
pub const SSWF_CUSTOM: u32 = 4;
pub const SSWF_DISPLAY: u32 = 3;
pub const SSWF_NONE: u32 = 0;
pub const SSWF_TITLE: u32 = 1;
pub const SSWF_WINDOW: u32 = 2;
pub const SS_BITMAP: u32 = 14;
pub const SS_BLACKFRAME: u32 = 7;
pub const SS_BLACKRECT: u32 = 4;
pub const SS_CENTER: u32 = 1;
pub const SS_CENTERIMAGE: u32 = 512;
pub const SS_EDITCONTROL: u32 = 8192;
pub const SS_ELLIPSISMASK: u32 = 49152;
pub const SS_ENDELLIPSIS: u32 = 16384;
pub const SS_ENHMETAFILE: u32 = 15;
pub const SS_ETCHEDFRAME: u32 = 18;
pub const SS_ETCHEDHORZ: u32 = 16;
pub const SS_ETCHEDVERT: u32 = 17;
pub const SS_GRAYFRAME: u32 = 8;
pub const SS_GRAYRECT: u32 = 5;
pub const SS_ICON: u32 = 3;
pub const SS_LEFT: u32 = 0;
pub const SS_LEFTNOWORDWRAP: u32 = 12;
pub const SS_NOPREFIX: u32 = 128;
pub const SS_NOTIFY: u32 = 256;
pub const SS_OWNERDRAW: u32 = 13;
pub const SS_PATHELLIPSIS: u32 = 32768;
pub const SS_REALSIZECONTROL: u32 = 64;
pub const SS_REALSIZEIMAGE: u32 = 2048;
pub const SS_RIGHT: u32 = 2;
pub const SS_RIGHTJUST: u32 = 1024;
pub const SS_SIMPLE: u32 = 11;
pub const SS_SUNKEN: u32 = 4096;
pub const SS_TYPEMASK: u32 = 31;
pub const SS_USERITEM: u32 = 10;
pub const SS_WHITEFRAME: u32 = 9;
pub const SS_WHITERECT: u32 = 6;
pub const SS_WORDELLIPSIS: u32 = 49152;
pub const STATE_SYSTEM_ALERT_HIGH: u32 = 268435456;
pub const STATE_SYSTEM_ALERT_LOW: u32 = 67108864;
pub const STATE_SYSTEM_ALERT_MEDIUM: u32 = 134217728;
pub const STATE_SYSTEM_ANIMATED: u32 = 16384;
pub const STATE_SYSTEM_BUSY: u32 = 2048;
pub const STATE_SYSTEM_CHECKED: u32 = 16;
pub const STATE_SYSTEM_COLLAPSED: u32 = 1024;
pub const STATE_SYSTEM_DEFAULT: u32 = 256;
pub const STATE_SYSTEM_EXPANDED: u32 = 512;
pub const STATE_SYSTEM_EXTSELECTABLE: u32 = 33554432;
pub const STATE_SYSTEM_FLOATING: u32 = 4096;
pub const STATE_SYSTEM_FOCUSABLE: u32 = 1048576;
pub const STATE_SYSTEM_FOCUSED: u32 = 4;
pub const STATE_SYSTEM_HOTTRACKED: u32 = 128;
pub const STATE_SYSTEM_INDETERMINATE: u32 = 32;
pub const STATE_SYSTEM_INVISIBLE: u32 = 32768;
pub const STATE_SYSTEM_LINKED: u32 = 4194304;
pub const STATE_SYSTEM_MARQUEED: u32 = 8192;
pub const STATE_SYSTEM_MIXED: u32 = 32;
pub const STATE_SYSTEM_MOVEABLE: u32 = 262144;
pub const STATE_SYSTEM_MULTISELECTABLE: u32 = 16777216;
pub const STATE_SYSTEM_OFFSCREEN: u32 = 65536;
pub const STATE_SYSTEM_PRESSED: u32 = 8;
pub const STATE_SYSTEM_PROTECTED: u32 = 536870912;
pub const STATE_SYSTEM_READONLY: u32 = 64;
pub const STATE_SYSTEM_SELECTABLE: u32 = 2097152;
pub const STATE_SYSTEM_SELECTED: u32 = 2;
pub const STATE_SYSTEM_SELFVOICING: u32 = 524288;
pub const STATE_SYSTEM_SIZEABLE: u32 = 131072;
pub const STATE_SYSTEM_TRAVERSED: u32 = 8388608;
pub const STATE_SYSTEM_UNAVAILABLE: u32 = 1;
pub const STATE_SYSTEM_VALID: u32 = 1073741823;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STICKYKEYS {
    pub cbSize: u32,
    pub dwFlags: u32,
}
pub const STM_GETICON: u32 = 369;
pub const STM_GETIMAGE: u32 = 371;
pub const STM_MSGMAX: u32 = 372;
pub const STM_SETICON: u32 = 368;
pub const STM_SETIMAGE: u32 = 370;
pub const STN_CLICKED: u32 = 0;
pub const STN_DBLCLK: u32 = 1;
pub const STN_DISABLE: u32 = 3;
pub const STN_ENABLE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STYLESTRUCT {
    pub styleOld: u32,
    pub styleNew: u32,
}
pub const SWP_ASYNCWINDOWPOS: u32 = 16384;
pub const SWP_DEFERERASE: u32 = 8192;
pub const SWP_DRAWFRAME: u32 = 32;
pub const SWP_FRAMECHANGED: u32 = 32;
pub const SWP_HIDEWINDOW: u32 = 128;
pub const SWP_NOACTIVATE: u32 = 16;
pub const SWP_NOCOPYBITS: u32 = 256;
pub const SWP_NOMOVE: u32 = 2;
pub const SWP_NONE: u32 = 0;
pub const SWP_NOOWNERZORDER: u32 = 512;
pub const SWP_NOREDRAW: u32 = 8;
pub const SWP_NOREPOSITION: u32 = 512;
pub const SWP_NOSENDCHANGING: u32 = 1024;
pub const SWP_NOSIZE: u32 = 1;
pub const SWP_NOZORDER: u32 = 4;
pub const SWP_SHOWWINDOW: u32 = 64;
pub const SW_ERASE: u32 = 4;
pub const SW_FORCEMINIMIZE: u32 = 11;
pub const SW_HIDE: u32 = 0;
pub const SW_INVALIDATE: u32 = 2;
pub const SW_MAX: u32 = 11;
pub const SW_MAXIMIZE: u32 = 3;
pub const SW_MINIMIZE: u32 = 6;
pub const SW_NORMAL: u32 = 1;
pub const SW_OTHERUNZOOM: u32 = 4;
pub const SW_OTHERZOOM: u32 = 2;
pub const SW_PARENTCLOSING: u32 = 1;
pub const SW_PARENTOPENING: u32 = 3;
pub const SW_RESTORE: u32 = 9;
pub const SW_SCROLLCHILDREN: u32 = 1;
pub const SW_SHOW: u32 = 5;
pub const SW_SHOWDEFAULT: u32 = 10;
pub const SW_SHOWMAXIMIZED: u32 = 3;
pub const SW_SHOWMINIMIZED: u32 = 2;
pub const SW_SHOWMINNOACTIVE: u32 = 7;
pub const SW_SHOWNA: u32 = 8;
pub const SW_SHOWNOACTIVATE: u32 = 4;
pub const SW_SHOWNORMAL: u32 = 1;
pub const SW_SMOOTHSCROLL: u32 = 16;
pub const TDF_REGISTER: TOOLTIP_DISMISS_FLAGS = 1;
pub const TDF_UNREGISTER: TOOLTIP_DISMISS_FLAGS = 2;
#[cfg(feature = "windef")]
pub type TIMERPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: u32, param2: usize, param3: u32)>;
pub const TIMERV_COALESCING_MAX: u32 = 2147483637;
pub const TIMERV_COALESCING_MIN: u32 = 1;
pub const TIMERV_DEFAULT_COALESCING: u32 = 0;
pub const TIMERV_NO_COALESCING: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct TITLEBARINFO {
    pub cbSize: u32,
    pub rcTitleBar: super::RECT,
    pub rgstate: [u32; 6],
}
#[cfg(feature = "windef")]
impl Default for TITLEBARINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct TITLEBARINFOEX {
    pub cbSize: u32,
    pub rcTitleBar: super::RECT,
    pub rgstate: [u32; 6],
    pub rgrect: [super::RECT; 6],
}
#[cfg(feature = "windef")]
impl Default for TITLEBARINFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TKF_AVAILABLE: u32 = 2;
pub const TKF_CONFIRMHOTKEY: u32 = 8;
pub const TKF_HOTKEYACTIVE: u32 = 4;
pub const TKF_HOTKEYSOUND: u32 = 16;
pub const TKF_INDICATOR: u32 = 32;
pub const TKF_TOGGLEKEYSON: u32 = 1;
pub const TME_CANCEL: u32 = 2147483648;
pub const TME_HOVER: u32 = 1;
pub const TME_LEAVE: u32 = 2;
pub const TME_NONCLIENT: u32 = 16;
pub const TME_QUERY: u32 = 1073741824;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TOGGLEKEYS {
    pub cbSize: u32,
    pub dwFlags: u32,
}
pub type TOOLTIP_DISMISS_FLAGS = i32;
pub const TOUCHEVENTF_DOWN: u32 = 2;
pub const TOUCHEVENTF_INRANGE: u32 = 8;
pub const TOUCHEVENTF_MOVE: u32 = 1;
pub const TOUCHEVENTF_NOCOALESCE: u32 = 32;
pub const TOUCHEVENTF_PALM: u32 = 128;
pub const TOUCHEVENTF_PEN: u32 = 64;
pub const TOUCHEVENTF_PRIMARY: u32 = 16;
pub const TOUCHEVENTF_UP: u32 = 4;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct TOUCHINPUT {
    pub x: i32,
    pub y: i32,
    pub hSource: super::HANDLE,
    pub dwID: u32,
    pub dwFlags: u32,
    pub dwMask: u32,
    pub dwTime: u32,
    pub dwExtraInfo: usize,
    pub cxContact: u32,
    pub cyContact: u32,
}
#[cfg(feature = "winnt")]
impl Default for TOUCHINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TOUCHINPUTMASKF_CONTACTAREA: u32 = 4;
pub const TOUCHINPUTMASKF_EXTRAINFO: u32 = 2;
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TOUCHPAD_PARAMETERS_V1 {
    pub versionNumber: u32,
    pub maxSupportedContacts: u32,
    pub legacyTouchpadFeatures: LEGACY_TOUCHPAD_FEATURES,
    pub _bitfield1: windows_sys::core::BOOL,
    pub _bitfield2: windows_sys::core::BOOL,
    pub sensitivityLevel: TOUCHPAD_SENSITIVITY_LEVEL,
    pub cursorSpeed: u32,
    pub feedbackIntensity: u32,
    pub clickForceSensitivity: u32,
    pub rightClickZoneWidth: u32,
    pub rightClickZoneHeight: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TOUCHPAD_PARAMETERS_V2 {
    pub Base: TOUCHPAD_PARAMETERS_V1,
    pub _bitfield: windows_sys::core::BOOL,
}
pub const TOUCHPAD_PARAMETERS_VERSION_1: u32 = 1;
pub const TOUCHPAD_PARAMETERS_VERSION_2: u32 = 2;
pub type TOUCHPAD_SENSITIVITY_LEVEL = i32;
pub const TOUCHPAD_SENSITIVITY_LEVEL_HIGH_SENSITIVITY: TOUCHPAD_SENSITIVITY_LEVEL = 1;
pub const TOUCHPAD_SENSITIVITY_LEVEL_LEAST_SENSITIVE: TOUCHPAD_SENSITIVITY_LEVEL = 4;
pub const TOUCHPAD_SENSITIVITY_LEVEL_LOW_SENSITIVITY: TOUCHPAD_SENSITIVITY_LEVEL = 3;
pub const TOUCHPAD_SENSITIVITY_LEVEL_MEDIUM_SENSITIVITY: TOUCHPAD_SENSITIVITY_LEVEL = 2;
pub const TOUCHPAD_SENSITIVITY_LEVEL_MOST_SENSITIVE: TOUCHPAD_SENSITIVITY_LEVEL = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TOUCHPREDICTIONPARAMETERS {
    pub cbSize: u32,
    pub dwLatency: u32,
    pub dwSampleTime: u32,
    pub bUseHWTimeStamp: u32,
}
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_LATENCY: u32 = 8;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_DELTA: f32 = 0.001;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_EXPO_SMOOTH_ALPHA: f32 = 0.99;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_LEARNING_RATE: f32 = 0.001;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_MAX: f32 = 0.999;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_MIN: f32 = 0.9;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_SAMPLETIME: u32 = 8;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_USE_HW_TIMESTAMP: u32 = 1;
pub const TOUCH_FEEDBACK_DEFAULT: u32 = 1;
pub const TOUCH_FEEDBACK_INDIRECT: u32 = 2;
pub const TOUCH_FEEDBACK_NONE: u32 = 3;
pub type TOUCH_FLAGS = u32;
pub const TOUCH_FLAG_NONE: u32 = 0;
pub const TOUCH_HIT_TESTING_CLIENT: u32 = 1;
pub const TOUCH_HIT_TESTING_DEFAULT: u32 = 0;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct TOUCH_HIT_TESTING_INPUT {
    pub pointerId: u32,
    pub point: super::POINT,
    pub boundingBox: super::RECT,
    pub nonOccludedBoundingBox: super::RECT,
    pub orientation: u32,
}
pub const TOUCH_HIT_TESTING_NONE: u32 = 2;
pub const TOUCH_HIT_TESTING_PROXIMITY_CLOSEST: u32 = 0;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    pub score: u16,
    pub adjustedPoint: super::POINT,
}
pub const TOUCH_HIT_TESTING_PROXIMITY_FARTHEST: u32 = 4095;
pub type TOUCH_MASK = u32;
pub const TOUCH_MASK_CONTACTAREA: u32 = 1;
pub const TOUCH_MASK_NONE: u32 = 0;
pub const TOUCH_MASK_ORIENTATION: u32 = 2;
pub const TOUCH_MASK_PRESSURE: u32 = 4;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct TPMPARAMS {
    pub cbSize: u32,
    pub rcExclude: super::RECT,
}
pub const TPM_BOTTOMALIGN: u32 = 32;
pub const TPM_CENTERALIGN: u32 = 4;
pub const TPM_HORIZONTAL: u32 = 0;
pub const TPM_HORNEGANIMATION: u32 = 2048;
pub const TPM_HORPOSANIMATION: u32 = 1024;
pub const TPM_LAYOUTRTL: u32 = 32768;
pub const TPM_LEFTALIGN: u32 = 0;
pub const TPM_LEFTBUTTON: u32 = 0;
pub const TPM_NOANIMATION: u32 = 16384;
pub const TPM_NONOTIFY: u32 = 128;
pub const TPM_RECURSE: u32 = 1;
pub const TPM_RETURNCMD: u32 = 256;
pub const TPM_RIGHTALIGN: u32 = 8;
pub const TPM_RIGHTBUTTON: u32 = 2;
pub const TPM_TOPALIGN: u32 = 0;
pub const TPM_VCENTERALIGN: u32 = 16;
pub const TPM_VERNEGANIMATION: u32 = 8192;
pub const TPM_VERPOSANIMATION: u32 = 4096;
pub const TPM_VERTICAL: u32 = 64;
pub const TPM_WORKAREA: u32 = 65536;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct TRACKMOUSEEVENT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTrack: super::HWND,
    pub dwHoverTime: u32,
}
#[cfg(feature = "windef")]
impl Default for TRACKMOUSEEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TWF_FINETOUCH: u32 = 1;
pub const TWF_WANTPALM: u32 = 2;
pub const UISF_ACTIVE: u32 = 4;
pub const UISF_HIDEACCEL: u32 = 2;
pub const UISF_HIDEFOCUS: u32 = 1;
pub const UIS_CLEAR: u32 = 2;
pub const UIS_INITIALIZE: u32 = 3;
pub const UIS_SET: u32 = 1;
pub const ULW_ALPHA: u32 = 2;
pub const ULW_COLORKEY: u32 = 1;
pub const ULW_EX_NORESIZE: u32 = 8;
pub const ULW_OPAQUE: u32 = 4;
pub const UNICODE_NOCHAR: u32 = 65535;
pub const UOI_FLAGS: u32 = 1;
pub const UOI_HEAPSIZE: u32 = 5;
pub const UOI_IO: u32 = 6;
pub const UOI_NAME: u32 = 2;
pub const UOI_TIMERPROC_EXCEPTION_SUPPRESSION: u32 = 7;
pub const UOI_TYPE: u32 = 3;
pub const UOI_USER_SID: u32 = 4;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub struct UPDATELAYEREDWINDOWINFO {
    pub cbSize: u32,
    pub hdcDst: super::HDC,
    pub pptDst: *const super::POINT,
    pub psize: *const super::SIZE,
    pub hdcSrc: super::HDC,
    pub pptSrc: *const super::POINT,
    pub crKey: super::COLORREF,
    pub pblend: *const super::BLENDFUNCTION,
    pub dwFlags: u32,
    pub prcDirty: *const super::RECT,
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
impl Default for UPDATELAYEREDWINDOWINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USAGE_PROPERTIES {
    pub level: u16,
    pub page: u16,
    pub usage: u16,
    pub logicalMinimum: i32,
    pub logicalMaximum: i32,
    pub unit: u16,
    pub exponent: u16,
    pub count: u8,
    pub physicalMinimum: i32,
    pub physicalMaximum: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USEROBJECTFLAGS {
    pub fInherit: windows_sys::core::BOOL,
    pub fReserved: windows_sys::core::BOOL,
    pub dwFlags: u32,
}
pub const USER_DEFAULT_SCREEN_DPI: u32 = 96;
pub const USER_TIMER_MAXIMUM: u32 = 2147483647;
pub const USER_TIMER_MINIMUM: u32 = 10;
pub const VK_ACCEPT: u32 = 30;
pub const VK_ADD: u32 = 107;
pub const VK_APPS: u32 = 93;
pub const VK_ATTN: u32 = 246;
pub const VK_BACK: u32 = 8;
pub const VK_BROWSER_BACK: u32 = 166;
pub const VK_BROWSER_FAVORITES: u32 = 171;
pub const VK_BROWSER_FORWARD: u32 = 167;
pub const VK_BROWSER_HOME: u32 = 172;
pub const VK_BROWSER_REFRESH: u32 = 168;
pub const VK_BROWSER_SEARCH: u32 = 170;
pub const VK_BROWSER_STOP: u32 = 169;
pub const VK_CANCEL: u32 = 3;
pub const VK_CAPITAL: u32 = 20;
pub const VK_CLEAR: u32 = 12;
pub const VK_CONTROL: u32 = 17;
pub const VK_CONVERT: u32 = 28;
pub const VK_CRSEL: u32 = 247;
pub const VK_DECIMAL: u32 = 110;
pub const VK_DELETE: u32 = 46;
pub const VK_DIVIDE: u32 = 111;
pub const VK_DOWN: u32 = 40;
pub const VK_END: u32 = 35;
pub const VK_EREOF: u32 = 249;
pub const VK_ESCAPE: u32 = 27;
pub const VK_EXECUTE: u32 = 43;
pub const VK_EXSEL: u32 = 248;
pub const VK_F1: u32 = 112;
pub const VK_F10: u32 = 121;
pub const VK_F11: u32 = 122;
pub const VK_F12: u32 = 123;
pub const VK_F13: u32 = 124;
pub const VK_F14: u32 = 125;
pub const VK_F15: u32 = 126;
pub const VK_F16: u32 = 127;
pub const VK_F17: u32 = 128;
pub const VK_F18: u32 = 129;
pub const VK_F19: u32 = 130;
pub const VK_F2: u32 = 113;
pub const VK_F20: u32 = 131;
pub const VK_F21: u32 = 132;
pub const VK_F22: u32 = 133;
pub const VK_F23: u32 = 134;
pub const VK_F24: u32 = 135;
pub const VK_F3: u32 = 114;
pub const VK_F4: u32 = 115;
pub const VK_F5: u32 = 116;
pub const VK_F6: u32 = 117;
pub const VK_F7: u32 = 118;
pub const VK_F8: u32 = 119;
pub const VK_F9: u32 = 120;
pub const VK_FINAL: u32 = 24;
pub const VK_GAMEPAD_A: u32 = 195;
pub const VK_GAMEPAD_B: u32 = 196;
pub const VK_GAMEPAD_DPAD_DOWN: u32 = 204;
pub const VK_GAMEPAD_DPAD_LEFT: u32 = 205;
pub const VK_GAMEPAD_DPAD_RIGHT: u32 = 206;
pub const VK_GAMEPAD_DPAD_UP: u32 = 203;
pub const VK_GAMEPAD_LEFT_SHOULDER: u32 = 200;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: u32 = 209;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: u32 = 212;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: u32 = 214;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: u32 = 213;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP: u32 = 211;
pub const VK_GAMEPAD_LEFT_TRIGGER: u32 = 201;
pub const VK_GAMEPAD_MENU: u32 = 207;
pub const VK_GAMEPAD_RIGHT_SHOULDER: u32 = 199;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: u32 = 210;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: u32 = 216;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: u32 = 218;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: u32 = 217;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: u32 = 215;
pub const VK_GAMEPAD_RIGHT_TRIGGER: u32 = 202;
pub const VK_GAMEPAD_VIEW: u32 = 208;
pub const VK_GAMEPAD_X: u32 = 197;
pub const VK_GAMEPAD_Y: u32 = 198;
pub const VK_HANGEUL: u32 = 21;
pub const VK_HANGUL: u32 = 21;
pub const VK_HANJA: u32 = 25;
pub const VK_HELP: u32 = 47;
pub const VK_HOME: u32 = 36;
pub const VK_ICO_00: u32 = 228;
pub const VK_ICO_CLEAR: u32 = 230;
pub const VK_ICO_HELP: u32 = 227;
pub const VK_IME_OFF: u32 = 26;
pub const VK_IME_ON: u32 = 22;
pub const VK_INSERT: u32 = 45;
pub const VK_JUNJA: u32 = 23;
pub const VK_KANA: u32 = 21;
pub const VK_KANJI: u32 = 25;
pub const VK_LAUNCH_APP1: u32 = 182;
pub const VK_LAUNCH_APP2: u32 = 183;
pub const VK_LAUNCH_MAIL: u32 = 180;
pub const VK_LAUNCH_MEDIA_SELECT: u32 = 181;
pub const VK_LBUTTON: u32 = 1;
pub const VK_LCONTROL: u32 = 162;
pub const VK_LEFT: u32 = 37;
pub const VK_LMENU: u32 = 164;
pub const VK_LSHIFT: u32 = 160;
pub const VK_LWIN: u32 = 91;
pub const VK_MBUTTON: u32 = 4;
pub const VK_MEDIA_NEXT_TRACK: u32 = 176;
pub const VK_MEDIA_PLAY_PAUSE: u32 = 179;
pub const VK_MEDIA_PREV_TRACK: u32 = 177;
pub const VK_MEDIA_STOP: u32 = 178;
pub const VK_MENU: u32 = 18;
pub const VK_MODECHANGE: u32 = 31;
pub const VK_MULTIPLY: u32 = 106;
pub const VK_NAVIGATION_ACCEPT: u32 = 142;
pub const VK_NAVIGATION_CANCEL: u32 = 143;
pub const VK_NAVIGATION_DOWN: u32 = 139;
pub const VK_NAVIGATION_LEFT: u32 = 140;
pub const VK_NAVIGATION_MENU: u32 = 137;
pub const VK_NAVIGATION_RIGHT: u32 = 141;
pub const VK_NAVIGATION_UP: u32 = 138;
pub const VK_NAVIGATION_VIEW: u32 = 136;
pub const VK_NEXT: u32 = 34;
pub const VK_NONAME: u32 = 252;
pub const VK_NONCONVERT: u32 = 29;
pub const VK_NUMLOCK: u32 = 144;
pub const VK_NUMPAD0: u32 = 96;
pub const VK_NUMPAD1: u32 = 97;
pub const VK_NUMPAD2: u32 = 98;
pub const VK_NUMPAD3: u32 = 99;
pub const VK_NUMPAD4: u32 = 100;
pub const VK_NUMPAD5: u32 = 101;
pub const VK_NUMPAD6: u32 = 102;
pub const VK_NUMPAD7: u32 = 103;
pub const VK_NUMPAD8: u32 = 104;
pub const VK_NUMPAD9: u32 = 105;
pub const VK_OEM_1: u32 = 186;
pub const VK_OEM_102: u32 = 226;
pub const VK_OEM_2: u32 = 191;
pub const VK_OEM_3: u32 = 192;
pub const VK_OEM_4: u32 = 219;
pub const VK_OEM_5: u32 = 220;
pub const VK_OEM_6: u32 = 221;
pub const VK_OEM_7: u32 = 222;
pub const VK_OEM_8: u32 = 223;
pub const VK_OEM_ATTN: u32 = 240;
pub const VK_OEM_AUTO: u32 = 243;
pub const VK_OEM_AX: u32 = 225;
pub const VK_OEM_BACKTAB: u32 = 245;
pub const VK_OEM_CLEAR: u32 = 254;
pub const VK_OEM_COMMA: u32 = 188;
pub const VK_OEM_COPY: u32 = 242;
pub const VK_OEM_CUSEL: u32 = 239;
pub const VK_OEM_ENLW: u32 = 244;
pub const VK_OEM_FINISH: u32 = 241;
pub const VK_OEM_FJ_JISHO: u32 = 146;
pub const VK_OEM_FJ_LOYA: u32 = 149;
pub const VK_OEM_FJ_MASSHOU: u32 = 147;
pub const VK_OEM_FJ_ROYA: u32 = 150;
pub const VK_OEM_FJ_TOUROKU: u32 = 148;
pub const VK_OEM_JUMP: u32 = 234;
pub const VK_OEM_MINUS: u32 = 189;
pub const VK_OEM_NEC_EQUAL: u32 = 146;
pub const VK_OEM_PA1: u32 = 235;
pub const VK_OEM_PA2: u32 = 236;
pub const VK_OEM_PA3: u32 = 237;
pub const VK_OEM_PERIOD: u32 = 190;
pub const VK_OEM_PLUS: u32 = 187;
pub const VK_OEM_RESET: u32 = 233;
pub const VK_OEM_WSCTRL: u32 = 238;
pub const VK_PA1: u32 = 253;
pub const VK_PACKET: u32 = 231;
pub const VK_PAUSE: u32 = 19;
pub const VK_PLAY: u32 = 250;
pub const VK_PRINT: u32 = 42;
pub const VK_PRIOR: u32 = 33;
pub const VK_PROCESSKEY: u32 = 229;
pub const VK_RBUTTON: u32 = 2;
pub const VK_RCONTROL: u32 = 163;
pub const VK_RETURN: u32 = 13;
pub const VK_RIGHT: u32 = 39;
pub const VK_RMENU: u32 = 165;
pub const VK_RSHIFT: u32 = 161;
pub const VK_RWIN: u32 = 92;
pub const VK_SCROLL: u32 = 145;
pub const VK_SELECT: u32 = 41;
pub const VK_SEPARATOR: u32 = 108;
pub const VK_SHIFT: u32 = 16;
pub const VK_SLEEP: u32 = 95;
pub const VK_SNAPSHOT: u32 = 44;
pub const VK_SPACE: u32 = 32;
pub const VK_SUBTRACT: u32 = 109;
pub const VK_TAB: u32 = 9;
pub const VK_UP: u32 = 38;
pub const VK_VOLUME_DOWN: u32 = 174;
pub const VK_VOLUME_MUTE: u32 = 173;
pub const VK_VOLUME_UP: u32 = 175;
pub const VK_XBUTTON1: u32 = 5;
pub const VK_XBUTTON2: u32 = 6;
pub const VK_ZOOM: u32 = 251;
pub const WAK_ACTIVATE: WINDOW_ACTION_KINDS = 16;
pub const WAK_COALESCEABLE: WINDOW_ACTION_KINDS = 31;
pub const WAK_DISPLAY_CHANGE: WINDOW_ACTION_KINDS = 512;
pub const WAK_FIT_TO_MONITOR: WINDOW_ACTION_KINDS = 256;
pub const WAK_INSERT_AFTER: WINDOW_ACTION_KINDS = 8;
pub const WAK_MOVE_TO_MONITOR: WINDOW_ACTION_KINDS = 128;
pub const WAK_NONE: WINDOW_ACTION_KINDS = 0;
pub const WAK_NORMAL_RECT: WINDOW_ACTION_KINDS = 64;
pub const WAK_PLACEMENT_STATE: WINDOW_ACTION_KINDS = 32;
pub const WAK_POSITION: WINDOW_ACTION_KINDS = 2;
pub const WAK_SIZE: WINDOW_ACTION_KINDS = 4;
pub const WAK_SYSTEM_OPERATION: WINDOW_ACTION_KINDS = 1024;
pub const WAK_VISIBILITY: WINDOW_ACTION_KINDS = 1;
pub const WAM_ACTIVATE_FOREGROUND: WINDOW_ACTION_MODIFIERS = 2;
pub const WAM_ACTIVATE_INPUT: WINDOW_ACTION_MODIFIERS = 4;
pub const WAM_ACTIVATE_NO_ZORDER: WINDOW_ACTION_MODIFIERS = 8;
pub const WAM_DPI: WINDOW_ACTION_MODIFIERS = 512;
pub const WAM_FRAME_BOUNDS: WINDOW_ACTION_MODIFIERS = 1;
pub const WAM_INSERT_AFTER_NO_OWNER: WINDOW_ACTION_MODIFIERS = 16;
pub const WAM_NONE: WINDOW_ACTION_MODIFIERS = 0;
pub const WAM_RESTORE_TO_ARRANGED: WINDOW_ACTION_MODIFIERS = 128;
pub const WAM_RESTORE_TO_MAXIMIZED: WINDOW_ACTION_MODIFIERS = 64;
pub const WAM_RESTORE_TO_NORMAL: WINDOW_ACTION_MODIFIERS = 32;
pub const WAM_SCALED_TO_MONITOR: WINDOW_ACTION_MODIFIERS = 1024;
pub const WAM_WORK_AREA: WINDOW_ACTION_MODIFIERS = 256;
pub const WA_ACTIVE: u32 = 1;
pub const WA_CLICKACTIVE: u32 = 2;
pub const WA_INACTIVE: u32 = 0;
pub const WB_ISDELIMITER: u32 = 2;
pub const WB_LEFT: u32 = 0;
pub const WB_RIGHT: u32 = 1;
pub const WDA_EXCLUDEFROMCAPTURE: u32 = 17;
pub const WDA_MONITOR: u32 = 1;
pub const WDA_NONE: u32 = 0;
pub const WHEEL_DELTA: u32 = 120;
pub const WHEEL_PAGESCROLL: i32 = -1;
pub const WH_CALLWNDPROC: u32 = 4;
pub const WH_CALLWNDPROCRET: u32 = 12;
pub const WH_CBT: u32 = 5;
pub const WH_DEBUG: u32 = 9;
pub const WH_FOREGROUNDIDLE: u32 = 11;
pub const WH_GETMESSAGE: u32 = 3;
pub const WH_JOURNALPLAYBACK: u32 = 1;
pub const WH_JOURNALRECORD: u32 = 0;
pub const WH_KEYBOARD: u32 = 2;
pub const WH_KEYBOARD_LL: u32 = 13;
pub const WH_MAX: u32 = 14;
pub const WH_MAXHOOK: u32 = 14;
pub const WH_MIN: i32 = -1;
pub const WH_MINHOOK: i32 = -1;
pub const WH_MOUSE: u32 = 7;
pub const WH_MOUSE_LL: u32 = 14;
pub const WH_MSGFILTER: i32 = -1;
pub const WH_SHELL: u32 = 10;
pub const WH_SYSMSGFILTER: u32 = 6;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct WINDOWINFO {
    pub cbSize: u32,
    pub rcWindow: super::RECT,
    pub rcClient: super::RECT,
    pub dwStyle: u32,
    pub dwExStyle: u32,
    pub dwWindowStatus: u32,
    pub cxWindowBorders: u32,
    pub cyWindowBorders: u32,
    pub atomWindowType: super::ATOM,
    pub wCreatorVersion: u16,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct WINDOWPLACEMENT {
    pub length: u32,
    pub flags: u32,
    pub showCmd: u32,
    pub ptMinPosition: super::POINT,
    pub ptMaxPosition: super::POINT,
    pub rcNormalPosition: super::RECT,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct WINDOWPOS {
    pub hwnd: super::HWND,
    pub hwndInsertAfter: super::HWND,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub flags: u32,
}
#[cfg(feature = "windef")]
impl Default for WINDOWPOS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct WINDOW_ACTION {
    pub kinds: WINDOW_ACTION_KINDS,
    pub modifiers: WINDOW_ACTION_MODIFIERS,
    pub visible: windows_sys::core::BOOL,
    pub position: super::POINT,
    pub size: super::SIZE,
    pub insertAfter: super::HWND,
    pub placementState: WINDOW_PLACEMENT_STATE,
    pub normalRect: super::RECT,
    pub workArea: super::RECT,
    pub dpi: u32,
    pub pointOnMonitor: super::POINT,
    pub monitorTopologyId: u32,
}
#[cfg(feature = "windef")]
impl Default for WINDOW_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WINDOW_ACTION_KINDS = u32;
pub type WINDOW_ACTION_MODIFIERS = u32;
pub type WINDOW_PLACEMENT_STATE = i32;
#[cfg(feature = "windef")]
pub type WINEVENTPROC = Option<unsafe extern "system" fn(hwineventhook: super::HWINEVENTHOOK, event: u32, hwnd: super::HWND, idobject: i32, idchild: i32, ideventthread: u32, dwmseventtime: u32)>;
pub const WINEVENT_INCONTEXT: u32 = 4;
pub const WINEVENT_OUTOFCONTEXT: u32 = 0;
pub const WINEVENT_SKIPOWNPROCESS: u32 = 2;
pub const WINEVENT_SKIPOWNTHREAD: u32 = 1;
#[cfg(feature = "minwindef")]
pub type WINSTAENUMPROC = WINSTAENUMPROCA;
#[cfg(feature = "minwindef")]
pub type WINSTAENUMPROCA = NAMEENUMPROCA;
#[cfg(feature = "minwindef")]
pub type WINSTAENUMPROCW = NAMEENUMPROCW;
pub const WINSTA_ACCESSCLIPBOARD: u32 = 4;
pub const WINSTA_ACCESSGLOBALATOMS: u32 = 32;
pub const WINSTA_ALL_ACCESS: u32 = 895;
pub const WINSTA_CREATEDESKTOP: u32 = 8;
pub const WINSTA_ENUMDESKTOPS: u32 = 1;
pub const WINSTA_ENUMERATE: u32 = 256;
pub const WINSTA_EXITWINDOWS: u32 = 64;
pub const WINSTA_READATTRIBUTES: u32 = 2;
pub const WINSTA_READSCREEN: u32 = 512;
pub const WINSTA_WRITEATTRIBUTES: u32 = 16;
pub const WMSZ_BOTTOM: u32 = 6;
pub const WMSZ_BOTTOMLEFT: u32 = 7;
pub const WMSZ_BOTTOMRIGHT: u32 = 8;
pub const WMSZ_LEFT: u32 = 1;
pub const WMSZ_RIGHT: u32 = 2;
pub const WMSZ_TOP: u32 = 3;
pub const WMSZ_TOPLEFT: u32 = 4;
pub const WMSZ_TOPRIGHT: u32 = 5;
pub const WM_ACTIVATE: u32 = 6;
pub const WM_ACTIVATEAPP: u32 = 28;
pub const WM_AFXFIRST: u32 = 864;
pub const WM_AFXLAST: u32 = 895;
pub const WM_APP: u32 = 32768;
pub const WM_APPCOMMAND: u32 = 793;
pub const WM_ASKCBFORMATNAME: u32 = 780;
pub const WM_CANCELJOURNAL: u32 = 75;
pub const WM_CANCELMODE: u32 = 31;
pub const WM_CAPTURECHANGED: u32 = 533;
pub const WM_CHANGECBCHAIN: u32 = 781;
pub const WM_CHANGEUISTATE: u32 = 295;
pub const WM_CHAR: u32 = 258;
pub const WM_CHARTOITEM: u32 = 47;
pub const WM_CHILDACTIVATE: u32 = 34;
pub const WM_CLEAR: u32 = 771;
pub const WM_CLIPBOARDUPDATE: u32 = 797;
pub const WM_CLOAKED_STATE_CHANGED: u32 = 839;
pub const WM_CLOSE: u32 = 16;
pub const WM_COMMAND: u32 = 273;
pub const WM_COMMNOTIFY: u32 = 68;
pub const WM_COMPACTING: u32 = 65;
pub const WM_COMPAREITEM: u32 = 57;
pub const WM_CONTEXTMENU: u32 = 123;
pub const WM_COPY: u32 = 769;
pub const WM_COPYDATA: u32 = 74;
pub const WM_CREATE: u32 = 1;
pub const WM_CTLCOLORBTN: u32 = 309;
pub const WM_CTLCOLORDLG: u32 = 310;
pub const WM_CTLCOLOREDIT: u32 = 307;
pub const WM_CTLCOLORLISTBOX: u32 = 308;
pub const WM_CTLCOLORMSGBOX: u32 = 306;
pub const WM_CTLCOLORSCROLLBAR: u32 = 311;
pub const WM_CTLCOLORSTATIC: u32 = 312;
pub const WM_CUT: u32 = 768;
pub const WM_DEADCHAR: u32 = 259;
pub const WM_DELETEITEM: u32 = 45;
pub const WM_DESTROY: u32 = 2;
pub const WM_DESTROYCLIPBOARD: u32 = 775;
pub const WM_DEVICECHANGE: u32 = 537;
pub const WM_DEVMODECHANGE: u32 = 27;
pub const WM_DISPLAYCHANGE: u32 = 126;
pub const WM_DPICHANGED: u32 = 736;
pub const WM_DPICHANGED_AFTERPARENT: u32 = 739;
pub const WM_DPICHANGED_BEFOREPARENT: u32 = 738;
pub const WM_DRAWCLIPBOARD: u32 = 776;
pub const WM_DRAWITEM: u32 = 43;
pub const WM_DROPFILES: u32 = 563;
pub const WM_DWMCOLORIZATIONCOLORCHANGED: u32 = 800;
pub const WM_DWMCOMPOSITIONCHANGED: u32 = 798;
pub const WM_DWMNCRENDERINGCHANGED: u32 = 799;
pub const WM_DWMSENDICONICLIVEPREVIEWBITMAP: u32 = 806;
pub const WM_DWMSENDICONICTHUMBNAIL: u32 = 803;
pub const WM_DWMWINDOWMAXIMIZEDCHANGE: u32 = 801;
pub const WM_ENABLE: u32 = 10;
pub const WM_ENDSESSION: u32 = 22;
pub const WM_ENTERIDLE: u32 = 289;
pub const WM_ENTERMENULOOP: u32 = 529;
pub const WM_ENTERSIZEMOVE: u32 = 561;
pub const WM_ERASEBKGND: u32 = 20;
pub const WM_EXITMENULOOP: u32 = 530;
pub const WM_EXITSIZEMOVE: u32 = 562;
pub const WM_FONTCHANGE: u32 = 29;
pub const WM_GESTURE: u32 = 281;
pub const WM_GESTURENOTIFY: u32 = 282;
pub const WM_GETDLGCODE: u32 = 135;
pub const WM_GETDPISCALEDSIZE: u32 = 740;
pub const WM_GETFONT: u32 = 49;
pub const WM_GETHOTKEY: u32 = 51;
pub const WM_GETICON: u32 = 127;
pub const WM_GETMINMAXINFO: u32 = 36;
pub const WM_GETOBJECT: u32 = 61;
pub const WM_GETTEXT: u32 = 13;
pub const WM_GETTEXTLENGTH: u32 = 14;
pub const WM_GETTITLEBARINFOEX: u32 = 831;
pub const WM_HANDHELDFIRST: u32 = 856;
pub const WM_HANDHELDLAST: u32 = 863;
pub const WM_HELP: u32 = 83;
pub const WM_HOTKEY: u32 = 786;
pub const WM_HSCROLL: u32 = 276;
pub const WM_HSCROLLCLIPBOARD: u32 = 782;
pub const WM_ICONERASEBKGND: u32 = 39;
pub const WM_IME_CHAR: u32 = 646;
pub const WM_IME_COMPOSITION: u32 = 271;
pub const WM_IME_COMPOSITIONFULL: u32 = 644;
pub const WM_IME_CONTROL: u32 = 643;
pub const WM_IME_ENDCOMPOSITION: u32 = 270;
pub const WM_IME_KEYDOWN: u32 = 656;
pub const WM_IME_KEYLAST: u32 = 271;
pub const WM_IME_KEYUP: u32 = 657;
pub const WM_IME_NOTIFY: u32 = 642;
pub const WM_IME_REQUEST: u32 = 648;
pub const WM_IME_SELECT: u32 = 645;
pub const WM_IME_SETCONTEXT: u32 = 641;
pub const WM_IME_STARTCOMPOSITION: u32 = 269;
pub const WM_INITDIALOG: u32 = 272;
pub const WM_INITMENU: u32 = 278;
pub const WM_INITMENUPOPUP: u32 = 279;
pub const WM_INPUT: u32 = 255;
pub const WM_INPUTLANGCHANGE: u32 = 81;
pub const WM_INPUTLANGCHANGEREQUEST: u32 = 80;
pub const WM_INPUT_DEVICE_CHANGE: u32 = 254;
pub const WM_INTERCEPTED_WINDOW_ACTION: u32 = 838;
pub const WM_KEYDOWN: u32 = 256;
pub const WM_KEYFIRST: u32 = 256;
pub const WM_KEYLAST: u32 = 265;
pub const WM_KEYUP: u32 = 257;
pub const WM_KILLFOCUS: u32 = 8;
pub const WM_LBUTTONDBLCLK: u32 = 515;
pub const WM_LBUTTONDOWN: u32 = 513;
pub const WM_LBUTTONUP: u32 = 514;
pub const WM_MBUTTONDBLCLK: u32 = 521;
pub const WM_MBUTTONDOWN: u32 = 519;
pub const WM_MBUTTONUP: u32 = 520;
pub const WM_MDIACTIVATE: u32 = 546;
pub const WM_MDICASCADE: u32 = 551;
pub const WM_MDICREATE: u32 = 544;
pub const WM_MDIDESTROY: u32 = 545;
pub const WM_MDIGETACTIVE: u32 = 553;
pub const WM_MDIICONARRANGE: u32 = 552;
pub const WM_MDIMAXIMIZE: u32 = 549;
pub const WM_MDINEXT: u32 = 548;
pub const WM_MDIREFRESHMENU: u32 = 564;
pub const WM_MDIRESTORE: u32 = 547;
pub const WM_MDISETMENU: u32 = 560;
pub const WM_MDITILE: u32 = 550;
pub const WM_MEASUREITEM: u32 = 44;
pub const WM_MENUCHAR: u32 = 288;
pub const WM_MENUCOMMAND: u32 = 294;
pub const WM_MENUDRAG: u32 = 291;
pub const WM_MENUGETOBJECT: u32 = 292;
pub const WM_MENURBUTTONUP: u32 = 290;
pub const WM_MENUSELECT: u32 = 287;
pub const WM_MOUSEACTIVATE: u32 = 33;
pub const WM_MOUSEFIRST: u32 = 512;
pub const WM_MOUSEHOVER: u32 = 673;
pub const WM_MOUSEHWHEEL: u32 = 526;
pub const WM_MOUSELAST: u32 = 526;
pub const WM_MOUSELEAVE: u32 = 675;
pub const WM_MOUSEMOVE: u32 = 512;
pub const WM_MOUSEWHEEL: u32 = 522;
pub const WM_MOVE: u32 = 3;
pub const WM_MOVING: u32 = 534;
pub const WM_NCACTIVATE: u32 = 134;
pub const WM_NCCALCSIZE: u32 = 131;
pub const WM_NCCREATE: u32 = 129;
pub const WM_NCDESTROY: u32 = 130;
pub const WM_NCHITTEST: u32 = 132;
pub const WM_NCLBUTTONDBLCLK: u32 = 163;
pub const WM_NCLBUTTONDOWN: u32 = 161;
pub const WM_NCLBUTTONUP: u32 = 162;
pub const WM_NCMBUTTONDBLCLK: u32 = 169;
pub const WM_NCMBUTTONDOWN: u32 = 167;
pub const WM_NCMBUTTONUP: u32 = 168;
pub const WM_NCMOUSEHOVER: u32 = 672;
pub const WM_NCMOUSELEAVE: u32 = 674;
pub const WM_NCMOUSEMOVE: u32 = 160;
pub const WM_NCPAINT: u32 = 133;
pub const WM_NCPOINTERDOWN: u32 = 578;
pub const WM_NCPOINTERUP: u32 = 579;
pub const WM_NCPOINTERUPDATE: u32 = 577;
pub const WM_NCRBUTTONDBLCLK: u32 = 166;
pub const WM_NCRBUTTONDOWN: u32 = 164;
pub const WM_NCRBUTTONUP: u32 = 165;
pub const WM_NCXBUTTONDBLCLK: u32 = 173;
pub const WM_NCXBUTTONDOWN: u32 = 171;
pub const WM_NCXBUTTONUP: u32 = 172;
pub const WM_NEXTDLGCTL: u32 = 40;
pub const WM_NEXTMENU: u32 = 531;
pub const WM_NOTIFY: u32 = 78;
pub const WM_NOTIFYFORMAT: u32 = 85;
pub const WM_NULL: u32 = 0;
pub const WM_PAINT: u32 = 15;
pub const WM_PAINTCLIPBOARD: u32 = 777;
pub const WM_PAINTICON: u32 = 38;
pub const WM_PALETTECHANGED: u32 = 785;
pub const WM_PALETTEISCHANGING: u32 = 784;
pub const WM_PARENTNOTIFY: u32 = 528;
pub const WM_PASTE: u32 = 770;
pub const WM_PENWINFIRST: u32 = 896;
pub const WM_PENWINLAST: u32 = 911;
pub const WM_POINTERACTIVATE: u32 = 587;
pub const WM_POINTERCAPTURECHANGED: u32 = 588;
pub const WM_POINTERDEVICECHANGE: u32 = 568;
pub const WM_POINTERDEVICEINRANGE: u32 = 569;
pub const WM_POINTERDEVICEOUTOFRANGE: u32 = 570;
pub const WM_POINTERDOWN: u32 = 582;
pub const WM_POINTERENTER: u32 = 585;
pub const WM_POINTERHWHEEL: u32 = 591;
pub const WM_POINTERLEAVE: u32 = 586;
pub const WM_POINTERROUTEDAWAY: u32 = 594;
pub const WM_POINTERROUTEDRELEASED: u32 = 595;
pub const WM_POINTERROUTEDTO: u32 = 593;
pub const WM_POINTERUP: u32 = 583;
pub const WM_POINTERUPDATE: u32 = 581;
pub const WM_POINTERWHEEL: u32 = 590;
pub const WM_POWER: u32 = 72;
pub const WM_POWERBROADCAST: u32 = 536;
pub const WM_PRINT: u32 = 791;
pub const WM_PRINTCLIENT: u32 = 792;
pub const WM_QUERYDRAGICON: u32 = 55;
pub const WM_QUERYENDSESSION: u32 = 17;
pub const WM_QUERYNEWPALETTE: u32 = 783;
pub const WM_QUERYOPEN: u32 = 19;
pub const WM_QUERYUISTATE: u32 = 297;
pub const WM_QUEUESYNC: u32 = 35;
pub const WM_QUIT: u32 = 18;
pub const WM_RBUTTONDBLCLK: u32 = 518;
pub const WM_RBUTTONDOWN: u32 = 516;
pub const WM_RBUTTONUP: u32 = 517;
pub const WM_RENDERALLFORMATS: u32 = 774;
pub const WM_RENDERFORMAT: u32 = 773;
pub const WM_SETCURSOR: u32 = 32;
pub const WM_SETFOCUS: u32 = 7;
pub const WM_SETFONT: u32 = 48;
pub const WM_SETHOTKEY: u32 = 50;
pub const WM_SETICON: u32 = 128;
pub const WM_SETREDRAW: u32 = 11;
pub const WM_SETTEXT: u32 = 12;
pub const WM_SETTINGCHANGE: u32 = 26;
pub const WM_SHOWWINDOW: u32 = 24;
pub const WM_SIZE: u32 = 5;
pub const WM_SIZECLIPBOARD: u32 = 779;
pub const WM_SIZING: u32 = 532;
pub const WM_SPOOLERSTATUS: u32 = 42;
pub const WM_STYLECHANGED: u32 = 125;
pub const WM_STYLECHANGING: u32 = 124;
pub const WM_SYNCPAINT: u32 = 136;
pub const WM_SYSCHAR: u32 = 262;
pub const WM_SYSCOLORCHANGE: u32 = 21;
pub const WM_SYSCOMMAND: u32 = 274;
pub const WM_SYSDEADCHAR: u32 = 263;
pub const WM_SYSKEYDOWN: u32 = 260;
pub const WM_SYSKEYUP: u32 = 261;
pub const WM_TABLET_FIRST: u32 = 704;
pub const WM_TABLET_LAST: u32 = 735;
pub const WM_TCARD: u32 = 82;
pub const WM_THEMECHANGED: u32 = 794;
pub const WM_TIMECHANGE: u32 = 30;
pub const WM_TIMER: u32 = 275;
pub const WM_TOOLTIPDISMISS: u32 = 837;
pub const WM_TOUCH: u32 = 576;
pub const WM_TOUCHHITTESTING: u32 = 589;
pub const WM_UNDO: u32 = 772;
pub const WM_UNICHAR: u32 = 265;
pub const WM_UNINITMENUPOPUP: u32 = 293;
pub const WM_UPDATEUISTATE: u32 = 296;
pub const WM_USER: u32 = 1024;
pub const WM_USERCHANGED: u32 = 84;
pub const WM_VKEYTOITEM: u32 = 46;
pub const WM_VSCROLL: u32 = 277;
pub const WM_VSCROLLCLIPBOARD: u32 = 778;
pub const WM_WINDOWPOSCHANGED: u32 = 71;
pub const WM_WINDOWPOSCHANGING: u32 = 70;
pub const WM_WININICHANGE: u32 = 26;
pub const WM_WTSSESSION_CHANGE: u32 = 689;
pub const WM_XBUTTONDBLCLK: u32 = 525;
pub const WM_XBUTTONDOWN: u32 = 523;
pub const WM_XBUTTONUP: u32 = 524;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type WNDCLASS = WNDCLASSA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct WNDCLASSA {
    pub style: u32,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::HINSTANCE,
    pub hIcon: super::HICON,
    pub hCursor: super::HCURSOR,
    pub hbrBackground: super::HBRUSH,
    pub lpszMenuName: windows_sys::core::PCSTR,
    pub lpszClassName: windows_sys::core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for WNDCLASSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type WNDCLASSEX = WNDCLASSEXA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct WNDCLASSEXA {
    pub cbSize: u32,
    pub style: u32,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::HINSTANCE,
    pub hIcon: super::HICON,
    pub hCursor: super::HCURSOR,
    pub hbrBackground: super::HBRUSH,
    pub lpszMenuName: windows_sys::core::PCSTR,
    pub lpszClassName: windows_sys::core::PCSTR,
    pub hIconSm: super::HICON,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for WNDCLASSEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct WNDCLASSEXW {
    pub cbSize: u32,
    pub style: u32,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::HINSTANCE,
    pub hIcon: super::HICON,
    pub hCursor: super::HCURSOR,
    pub hbrBackground: super::HBRUSH,
    pub lpszMenuName: windows_sys::core::PCWSTR,
    pub lpszClassName: windows_sys::core::PCWSTR,
    pub hIconSm: super::HICON,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for WNDCLASSEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct WNDCLASSW {
    pub style: u32,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::HINSTANCE,
    pub hIcon: super::HICON,
    pub hCursor: super::HCURSOR,
    pub hbrBackground: super::HBRUSH,
    pub lpszMenuName: windows_sys::core::PCWSTR,
    pub lpszClassName: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for WNDCLASSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type WNDENUMPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: super::LPARAM) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type WNDPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: u32, param2: super::WPARAM, param3: super::LPARAM) -> super::LRESULT>;
pub const WPF_ASYNCWINDOWPLACEMENT: u32 = 4;
pub const WPF_RESTORETOMAXIMIZED: u32 = 2;
pub const WPF_SETMINPOSITION: u32 = 1;
pub const WPS_ARRANGED: WINDOW_PLACEMENT_STATE = 3;
pub const WPS_MAXIMIZED: WINDOW_PLACEMENT_STATE = 1;
pub const WPS_MINIMIZED: WINDOW_PLACEMENT_STATE = 2;
pub const WPS_NORMAL: WINDOW_PLACEMENT_STATE = 0;
pub const WSF_VISIBLE: u32 = 1;
pub const WS_ACTIVECAPTION: u32 = 1;
pub const WS_BORDER: u32 = 8388608;
pub const WS_CAPTION: u32 = 12582912;
pub const WS_CHILD: u32 = 1073741824;
pub const WS_CHILDWINDOW: u32 = 1073741824;
pub const WS_CLIPCHILDREN: u32 = 33554432;
pub const WS_CLIPSIBLINGS: u32 = 67108864;
pub const WS_DISABLED: u32 = 134217728;
pub const WS_DLGFRAME: u32 = 4194304;
pub const WS_EX_ACCEPTFILES: u32 = 16;
pub const WS_EX_APPWINDOW: u32 = 262144;
pub const WS_EX_CLIENTEDGE: u32 = 512;
pub const WS_EX_COMPOSITED: u32 = 33554432;
pub const WS_EX_CONTEXTHELP: u32 = 1024;
pub const WS_EX_CONTROLPARENT: u32 = 65536;
pub const WS_EX_DLGMODALFRAME: u32 = 1;
pub const WS_EX_LAYERED: u32 = 524288;
pub const WS_EX_LAYOUTRTL: u32 = 4194304;
pub const WS_EX_LEFT: u32 = 0;
pub const WS_EX_LEFTSCROLLBAR: u32 = 16384;
pub const WS_EX_LTRREADING: u32 = 0;
pub const WS_EX_MDICHILD: u32 = 64;
pub const WS_EX_NOACTIVATE: u32 = 134217728;
pub const WS_EX_NOINHERITLAYOUT: u32 = 1048576;
pub const WS_EX_NOPARENTNOTIFY: u32 = 4;
pub const WS_EX_NOREDIRECTIONBITMAP: u32 = 2097152;
pub const WS_EX_OVERLAPPEDWINDOW: u32 = 768;
pub const WS_EX_PALETTEWINDOW: u32 = 392;
pub const WS_EX_RIGHT: u32 = 4096;
pub const WS_EX_RIGHTSCROLLBAR: u32 = 0;
pub const WS_EX_RTLREADING: u32 = 8192;
pub const WS_EX_STATICEDGE: u32 = 131072;
pub const WS_EX_TOOLWINDOW: u32 = 128;
pub const WS_EX_TOPMOST: u32 = 8;
pub const WS_EX_TRANSPARENT: u32 = 32;
pub const WS_EX_WINDOWEDGE: u32 = 256;
pub const WS_GROUP: u32 = 131072;
pub const WS_HSCROLL: u32 = 1048576;
pub const WS_ICONIC: u32 = 536870912;
pub const WS_MAXIMIZE: u32 = 16777216;
pub const WS_MAXIMIZEBOX: u32 = 65536;
pub const WS_MINIMIZE: u32 = 536870912;
pub const WS_MINIMIZEBOX: u32 = 131072;
pub const WS_OVERLAPPED: u32 = 0;
pub const WS_OVERLAPPEDWINDOW: u32 = 13565952;
pub const WS_POPUP: u32 = 2147483648;
pub const WS_POPUPWINDOW: i32 = -2138570752;
pub const WS_SIZEBOX: u32 = 262144;
pub const WS_SYSMENU: u32 = 524288;
pub const WS_TABSTOP: u32 = 65536;
pub const WS_THICKFRAME: u32 = 262144;
pub const WS_TILED: u32 = 0;
pub const WS_TILEDWINDOW: u32 = 13565952;
pub const WS_VISIBLE: u32 = 268435456;
pub const WS_VSCROLL: u32 = 2097152;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WTSSESSION_NOTIFICATION {
    pub cbSize: u32,
    pub dwSessionId: u32,
}
pub const WTS_CONSOLE_CONNECT: u32 = 1;
pub const WTS_CONSOLE_DISCONNECT: u32 = 2;
pub const WTS_REMOTE_CONNECT: u32 = 3;
pub const WTS_REMOTE_DISCONNECT: u32 = 4;
pub const WTS_SESSION_CREATE: u32 = 10;
pub const WTS_SESSION_DESKTOP_READY: u32 = 15;
pub const WTS_SESSION_LOCK: u32 = 7;
pub const WTS_SESSION_LOGOFF: u32 = 6;
pub const WTS_SESSION_LOGON: u32 = 5;
pub const WTS_SESSION_REMOTE_CONTROL: u32 = 9;
pub const WTS_SESSION_TERMINATE: u32 = 11;
pub const WTS_SESSION_UNLOCK: u32 = 8;
pub const WVR_ALIGNBOTTOM: u32 = 64;
pub const WVR_ALIGNLEFT: u32 = 32;
pub const WVR_ALIGNRIGHT: u32 = 128;
pub const WVR_ALIGNTOP: u32 = 16;
pub const WVR_HREDRAW: u32 = 256;
pub const WVR_REDRAW: u32 = 768;
pub const WVR_VALIDRECTS: u32 = 1024;
pub const WVR_VREDRAW: u32 = 512;
pub const XBUTTON1: u32 = 1;
pub const XBUTTON2: u32 = 2;
pub type tagPOINTER_INPUT_TYPE = i32;
