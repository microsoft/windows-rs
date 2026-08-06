#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ActivateKeyboardLayout(hkl: super::HKL, flags: u32) -> super::HKL {
    windows_core::link!("user32.dll" "system" fn ActivateKeyboardLayout(hkl : super::HKL, flags : u32) -> super::HKL);
    unsafe { ActivateKeyboardLayout(hkl, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AddClipboardFormatListener(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn AddClipboardFormatListener(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { AddClipboardFormatListener(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AdjustWindowRect(lprect: *mut super::RECT, dwstyle: u32, bmenu: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn AdjustWindowRect(lprect : *mut super::RECT, dwstyle : u32, bmenu : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AdjustWindowRect(lprect as _, dwstyle, bmenu.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AdjustWindowRectEx(lprect: *mut super::RECT, dwstyle: u32, bmenu: bool, dwexstyle: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn AdjustWindowRectEx(lprect : *mut super::RECT, dwstyle : u32, bmenu : windows_core::BOOL, dwexstyle : u32) -> windows_core::BOOL);
    unsafe { AdjustWindowRectEx(lprect as _, dwstyle, bmenu.into(), dwexstyle) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AdjustWindowRectExForDpi(lprect: *mut super::RECT, dwstyle: u32, bmenu: bool, dwexstyle: u32, dpi: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn AdjustWindowRectExForDpi(lprect : *mut super::RECT, dwstyle : u32, bmenu : windows_core::BOOL, dwexstyle : u32, dpi : u32) -> windows_core::BOOL);
    unsafe { AdjustWindowRectExForDpi(lprect as _, dwstyle, bmenu.into(), dwexstyle, dpi) }
}
#[inline]
pub unsafe fn AllowSetForegroundWindow(dwprocessid: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn AllowSetForegroundWindow(dwprocessid : u32) -> windows_core::BOOL);
    unsafe { AllowSetForegroundWindow(dwprocessid) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AnimateWindow(hwnd: super::HWND, dwtime: u32, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn AnimateWindow(hwnd : super::HWND, dwtime : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { AnimateWindow(hwnd, dwtime, dwflags) }
}
#[inline]
pub unsafe fn AnyPopup() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn AnyPopup() -> windows_core::BOOL);
    unsafe { AnyPopup() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AppendMenuA<P3>(hmenu: super::HMENU, uflags: u32, uidnewitem: usize, lpnewitem: P3) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn AppendMenuA(hmenu : super::HMENU, uflags : u32, uidnewitem : usize, lpnewitem : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { AppendMenuA(hmenu, uflags, uidnewitem, lpnewitem.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AppendMenuW<P3>(hmenu: super::HMENU, uflags: u32, uidnewitem: usize, lpnewitem: P3) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn AppendMenuW(hmenu : super::HMENU, uflags : u32, uidnewitem : usize, lpnewitem : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { AppendMenuW(hmenu, uflags, uidnewitem, lpnewitem.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ApplyWindowAction(hwnd: super::HWND, paction: *mut WINDOW_ACTION) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ApplyWindowAction(hwnd : super::HWND, paction : *mut WINDOW_ACTION) -> windows_core::BOOL);
    unsafe { ApplyWindowAction(hwnd, paction as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AreDpiAwarenessContextsEqual(dpicontexta: super::DPI_AWARENESS_CONTEXT, dpicontextb: super::DPI_AWARENESS_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn AreDpiAwarenessContextsEqual(dpicontexta : super::DPI_AWARENESS_CONTEXT, dpicontextb : super::DPI_AWARENESS_CONTEXT) -> windows_core::BOOL);
    unsafe { AreDpiAwarenessContextsEqual(dpicontexta, dpicontextb) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ArrangeIconicWindows(hwnd: super::HWND) -> u32 {
    windows_core::link!("user32.dll" "system" fn ArrangeIconicWindows(hwnd : super::HWND) -> u32);
    unsafe { ArrangeIconicWindows(hwnd) }
}
#[inline]
pub unsafe fn AttachThreadInput(idattach: u32, idattachto: u32, fattach: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn AttachThreadInput(idattach : u32, idattachto : u32, fattach : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AttachThreadInput(idattach, idattachto, fattach.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn BeginDeferWindowPos(nnumwindows: i32) -> HDWP {
    windows_core::link!("user32.dll" "system" fn BeginDeferWindowPos(nnumwindows : i32) -> HDWP);
    unsafe { BeginDeferWindowPos(nnumwindows) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn BeginPaint(hwnd: super::HWND, lppaint: *mut PAINTSTRUCT) -> super::HDC {
    windows_core::link!("user32.dll" "system" fn BeginPaint(hwnd : super::HWND, lppaint : *mut PAINTSTRUCT) -> super::HDC);
    unsafe { BeginPaint(hwnd, lppaint as _) }
}
#[inline]
pub unsafe fn BlockInput(fblockit: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn BlockInput(fblockit : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { BlockInput(fblockit.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn BringWindowToTop(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn BringWindowToTop(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { BringWindowToTop(hwnd) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn BroadcastSystemMessageA(flags: u32, lpinfo: Option<*mut u32>, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> i32 {
    windows_core::link!("user32.dll" "system" fn BroadcastSystemMessageA(flags : u32, lpinfo : *mut u32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> i32);
    unsafe { BroadcastSystemMessageA(flags, lpinfo.unwrap_or(core::mem::zeroed()) as _, msg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn BroadcastSystemMessageExA(flags: u32, lpinfo: Option<*mut u32>, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM, pbsminfo: Option<*mut BSMINFO>) -> i32 {
    windows_core::link!("user32.dll" "system" fn BroadcastSystemMessageExA(flags : u32, lpinfo : *mut u32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM, pbsminfo : *mut BSMINFO) -> i32);
    unsafe { BroadcastSystemMessageExA(flags, lpinfo.unwrap_or(core::mem::zeroed()) as _, msg, wparam, lparam, pbsminfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn BroadcastSystemMessageExW(flags: u32, lpinfo: Option<*mut u32>, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM, pbsminfo: Option<*mut BSMINFO>) -> i32 {
    windows_core::link!("user32.dll" "system" fn BroadcastSystemMessageExW(flags : u32, lpinfo : *mut u32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM, pbsminfo : *mut BSMINFO) -> i32);
    unsafe { BroadcastSystemMessageExW(flags, lpinfo.unwrap_or(core::mem::zeroed()) as _, msg, wparam, lparam, pbsminfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn BroadcastSystemMessageW(flags: u32, lpinfo: Option<*mut u32>, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> i32 {
    windows_core::link!("user32.dll" "system" fn BroadcastSystemMessageW(flags : u32, lpinfo : *mut u32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> i32);
    unsafe { BroadcastSystemMessageW(flags, lpinfo.unwrap_or(core::mem::zeroed()) as _, msg, wparam, lparam) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CalculatePopupWindowPosition(anchorpoint: *const super::POINT, windowsize: *const super::SIZE, flags: u32, excluderect: Option<*const super::RECT>, popupwindowposition: *mut super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CalculatePopupWindowPosition(anchorpoint : *const super::POINT, windowsize : *const super::SIZE, flags : u32, excluderect : *const super::RECT, popupwindowposition : *mut super::RECT) -> windows_core::BOOL);
    unsafe { CalculatePopupWindowPosition(anchorpoint, windowsize, flags, excluderect.unwrap_or(core::mem::zeroed()) as _, popupwindowposition as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CallMsgFilterA(lpmsg: *const MSG, ncode: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CallMsgFilterA(lpmsg : *const MSG, ncode : i32) -> windows_core::BOOL);
    unsafe { CallMsgFilterA(lpmsg, ncode) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CallMsgFilterW(lpmsg: *const MSG, ncode: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CallMsgFilterW(lpmsg : *const MSG, ncode : i32) -> windows_core::BOOL);
    unsafe { CallMsgFilterW(lpmsg, ncode) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CallNextHookEx(hhk: Option<super::HHOOK>, ncode: i32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn CallNextHookEx(hhk : super::HHOOK, ncode : i32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { CallNextHookEx(hhk.unwrap_or(core::mem::zeroed()) as _, ncode, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CallWindowProcA(lpprevwndfunc: WNDPROC, hwnd: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn CallWindowProcA(lpprevwndfunc : WNDPROC, hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { CallWindowProcA(lpprevwndfunc, hwnd, msg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CallWindowProcW(lpprevwndfunc: WNDPROC, hwnd: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn CallWindowProcW(lpprevwndfunc : WNDPROC, hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { CallWindowProcW(lpprevwndfunc, hwnd, msg, wparam, lparam) }
}
#[inline]
pub unsafe fn CancelShutdown() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CancelShutdown() -> windows_core::BOOL);
    unsafe { CancelShutdown() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CascadeWindows(hwndparent: Option<super::HWND>, whow: u32, lprect: Option<*const super::RECT>, lpkids: Option<&[super::HWND]>) -> u16 {
    windows_core::link!("user32.dll" "system" fn CascadeWindows(hwndparent : super::HWND, whow : u32, lprect : *const super::RECT, ckids : u32, lpkids : *const super::HWND) -> u16);
    unsafe { CascadeWindows(hwndparent.unwrap_or(core::mem::zeroed()) as _, whow, lprect.unwrap_or(core::mem::zeroed()) as _, lpkids.map_or(0, |slice| slice.len().try_into().unwrap()), lpkids.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ChangeClipboardChain(hwndremove: super::HWND, hwndnewnext: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ChangeClipboardChain(hwndremove : super::HWND, hwndnewnext : super::HWND) -> windows_core::BOOL);
    unsafe { ChangeClipboardChain(hwndremove, hwndnewnext) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ChangeDisplaySettingsA(lpdevmode: Option<*const super::DEVMODEA>, dwflags: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn ChangeDisplaySettingsA(lpdevmode : *const super::DEVMODEA, dwflags : u32) -> i32);
    unsafe { ChangeDisplaySettingsA(lpdevmode.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ChangeDisplaySettingsExA<P0>(lpszdevicename: P0, lpdevmode: Option<*const super::DEVMODEA>, hwnd: Option<super::HWND>, dwflags: u32, lparam: Option<*const core::ffi::c_void>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn ChangeDisplaySettingsExA(lpszdevicename : windows_core::PCSTR, lpdevmode : *const super::DEVMODEA, hwnd : super::HWND, dwflags : u32, lparam : *const core::ffi::c_void) -> i32);
    unsafe { ChangeDisplaySettingsExA(lpszdevicename.param().abi(), lpdevmode.unwrap_or(core::mem::zeroed()) as _, hwnd.unwrap_or(core::mem::zeroed()) as _, dwflags, lparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ChangeDisplaySettingsExW<P0>(lpszdevicename: P0, lpdevmode: Option<*const super::DEVMODEW>, hwnd: Option<super::HWND>, dwflags: u32, lparam: Option<*const core::ffi::c_void>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn ChangeDisplaySettingsExW(lpszdevicename : windows_core::PCWSTR, lpdevmode : *const super::DEVMODEW, hwnd : super::HWND, dwflags : u32, lparam : *const core::ffi::c_void) -> i32);
    unsafe { ChangeDisplaySettingsExW(lpszdevicename.param().abi(), lpdevmode.unwrap_or(core::mem::zeroed()) as _, hwnd.unwrap_or(core::mem::zeroed()) as _, dwflags, lparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ChangeDisplaySettingsW(lpdevmode: Option<*const super::DEVMODEW>, dwflags: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn ChangeDisplaySettingsW(lpdevmode : *const super::DEVMODEW, dwflags : u32) -> i32);
    unsafe { ChangeDisplaySettingsW(lpdevmode.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ChangeMenuA<P2>(hmenu: super::HMENU, cmd: u32, lpsznewitem: P2, cmdinsert: u32, flags: u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn ChangeMenuA(hmenu : super::HMENU, cmd : u32, lpsznewitem : windows_core::PCSTR, cmdinsert : u32, flags : u32) -> windows_core::BOOL);
    unsafe { ChangeMenuA(hmenu, cmd, lpsznewitem.param().abi(), cmdinsert, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ChangeMenuW<P2>(hmenu: super::HMENU, cmd: u32, lpsznewitem: P2, cmdinsert: u32, flags: u32) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn ChangeMenuW(hmenu : super::HMENU, cmd : u32, lpsznewitem : windows_core::PCWSTR, cmdinsert : u32, flags : u32) -> windows_core::BOOL);
    unsafe { ChangeMenuW(hmenu, cmd, lpsznewitem.param().abi(), cmdinsert, flags) }
}
#[inline]
pub unsafe fn ChangeWindowMessageFilter(message: u32, dwflag: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ChangeWindowMessageFilter(message : u32, dwflag : u32) -> windows_core::BOOL);
    unsafe { ChangeWindowMessageFilter(message, dwflag) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ChangeWindowMessageFilterEx(hwnd: super::HWND, message: u32, action: u32, pchangefilterstruct: Option<*mut CHANGEFILTERSTRUCT>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ChangeWindowMessageFilterEx(hwnd : super::HWND, message : u32, action : u32, pchangefilterstruct : *mut CHANGEFILTERSTRUCT) -> windows_core::BOOL);
    unsafe { ChangeWindowMessageFilterEx(hwnd, message, action, pchangefilterstruct.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CharLowerA(lpsz: windows_core::PSTR) -> windows_core::PSTR {
    windows_core::link!("user32.dll" "system" fn CharLowerA(lpsz : windows_core::PSTR) -> windows_core::PSTR);
    unsafe { CharLowerA(lpsz) }
}
#[inline]
pub unsafe fn CharLowerBuffA(lpsz: &mut [u8]) -> u32 {
    windows_core::link!("user32.dll" "system" fn CharLowerBuffA(lpsz : windows_core::PSTR, cchlength : u32) -> u32);
    unsafe { CharLowerBuffA(core::mem::transmute(lpsz.as_mut_ptr()), lpsz.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn CharLowerBuffW(lpsz: &mut [u16]) -> u32 {
    windows_core::link!("user32.dll" "system" fn CharLowerBuffW(lpsz : windows_core::PWSTR, cchlength : u32) -> u32);
    unsafe { CharLowerBuffW(core::mem::transmute(lpsz.as_mut_ptr()), lpsz.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn CharLowerW(lpsz: windows_core::PWSTR) -> windows_core::PWSTR {
    windows_core::link!("user32.dll" "system" fn CharLowerW(lpsz : windows_core::PWSTR) -> windows_core::PWSTR);
    unsafe { CharLowerW(lpsz) }
}
#[inline]
pub unsafe fn CharNextA<P0>(lpsz: P0) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CharNextA(lpsz : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { CharNextA(lpsz.param().abi()) }
}
#[inline]
pub unsafe fn CharNextExA<P1>(codepage: u16, lpcurrentchar: P1, dwflags: u32) -> windows_core::PSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CharNextExA(codepage : u16, lpcurrentchar : windows_core::PCSTR, dwflags : u32) -> windows_core::PSTR);
    unsafe { CharNextExA(codepage, lpcurrentchar.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn CharNextW<P0>(lpsz: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CharNextW(lpsz : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { CharNextW(lpsz.param().abi()) }
}
#[inline]
pub unsafe fn CharPrevA<P0, P1>(lpszstart: P0, lpszcurrent: P1) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CharPrevA(lpszstart : windows_core::PCSTR, lpszcurrent : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { CharPrevA(lpszstart.param().abi(), lpszcurrent.param().abi()) }
}
#[inline]
pub unsafe fn CharPrevExA<P1, P2>(codepage: u16, lpstart: P1, lpcurrentchar: P2, dwflags: u32) -> windows_core::PSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CharPrevExA(codepage : u16, lpstart : windows_core::PCSTR, lpcurrentchar : windows_core::PCSTR, dwflags : u32) -> windows_core::PSTR);
    unsafe { CharPrevExA(codepage, lpstart.param().abi(), lpcurrentchar.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn CharPrevW<P0, P1>(lpszstart: P0, lpszcurrent: P1) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CharPrevW(lpszstart : windows_core::PCWSTR, lpszcurrent : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { CharPrevW(lpszstart.param().abi(), lpszcurrent.param().abi()) }
}
#[inline]
pub unsafe fn CharToOemA<P0>(psrc: P0, pdst: windows_core::PSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CharToOemA(psrc : windows_core::PCSTR, pdst : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { CharToOemA(psrc.param().abi(), pdst) }
}
#[inline]
pub unsafe fn CharToOemBuffA<P0>(lpszsrc: P0, lpszdst: windows_core::PSTR, cchdstlength: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CharToOemBuffA(lpszsrc : windows_core::PCSTR, lpszdst : windows_core::PSTR, cchdstlength : u32) -> windows_core::BOOL);
    unsafe { CharToOemBuffA(lpszsrc.param().abi(), lpszdst, cchdstlength) }
}
#[inline]
pub unsafe fn CharToOemBuffW<P0>(lpszsrc: P0, lpszdst: windows_core::PSTR, cchdstlength: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CharToOemBuffW(lpszsrc : windows_core::PCWSTR, lpszdst : windows_core::PSTR, cchdstlength : u32) -> windows_core::BOOL);
    unsafe { CharToOemBuffW(lpszsrc.param().abi(), lpszdst, cchdstlength) }
}
#[inline]
pub unsafe fn CharToOemW<P0>(psrc: P0, pdst: windows_core::PSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CharToOemW(psrc : windows_core::PCWSTR, pdst : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { CharToOemW(psrc.param().abi(), pdst) }
}
#[inline]
pub unsafe fn CharUpperA(lpsz: windows_core::PSTR) -> windows_core::PSTR {
    windows_core::link!("user32.dll" "system" fn CharUpperA(lpsz : windows_core::PSTR) -> windows_core::PSTR);
    unsafe { CharUpperA(lpsz) }
}
#[inline]
pub unsafe fn CharUpperBuffA(lpsz: &mut [u8]) -> u32 {
    windows_core::link!("user32.dll" "system" fn CharUpperBuffA(lpsz : windows_core::PSTR, cchlength : u32) -> u32);
    unsafe { CharUpperBuffA(core::mem::transmute(lpsz.as_mut_ptr()), lpsz.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn CharUpperBuffW(lpsz: &mut [u16]) -> u32 {
    windows_core::link!("user32.dll" "system" fn CharUpperBuffW(lpsz : windows_core::PWSTR, cchlength : u32) -> u32);
    unsafe { CharUpperBuffW(core::mem::transmute(lpsz.as_mut_ptr()), lpsz.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn CharUpperW(lpsz: windows_core::PWSTR) -> windows_core::PWSTR {
    windows_core::link!("user32.dll" "system" fn CharUpperW(lpsz : windows_core::PWSTR) -> windows_core::PWSTR);
    unsafe { CharUpperW(lpsz) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CheckDlgButton(hdlg: super::HWND, nidbutton: i32, ucheck: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CheckDlgButton(hdlg : super::HWND, nidbutton : i32, ucheck : u32) -> windows_core::BOOL);
    unsafe { CheckDlgButton(hdlg, nidbutton, ucheck) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CheckMenuItem(hmenu: super::HMENU, uidcheckitem: u32, ucheck: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn CheckMenuItem(hmenu : super::HMENU, uidcheckitem : u32, ucheck : u32) -> u32);
    unsafe { CheckMenuItem(hmenu, uidcheckitem, ucheck) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CheckMenuRadioItem(hmenu: super::HMENU, first: u32, last: u32, check: u32, flags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CheckMenuRadioItem(hmenu : super::HMENU, first : u32, last : u32, check : u32, flags : u32) -> windows_core::BOOL);
    unsafe { CheckMenuRadioItem(hmenu, first, last, check, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CheckRadioButton(hdlg: super::HWND, nidfirstbutton: i32, nidlastbutton: i32, nidcheckbutton: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CheckRadioButton(hdlg : super::HWND, nidfirstbutton : i32, nidlastbutton : i32, nidcheckbutton : i32) -> windows_core::BOOL);
    unsafe { CheckRadioButton(hdlg, nidfirstbutton, nidlastbutton, nidcheckbutton) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ChildWindowFromPoint(hwndparent: super::HWND, point: super::POINT) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn ChildWindowFromPoint(hwndparent : super::HWND, point : super::POINT) -> super::HWND);
    unsafe { ChildWindowFromPoint(hwndparent, point) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ChildWindowFromPointEx(hwnd: super::HWND, pt: super::POINT, flags: u32) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn ChildWindowFromPointEx(hwnd : super::HWND, pt : super::POINT, flags : u32) -> super::HWND);
    unsafe { ChildWindowFromPointEx(hwnd, pt, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ClientToScreen(hwnd: super::HWND, lppoint: *mut super::POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ClientToScreen(hwnd : super::HWND, lppoint : *mut super::POINT) -> windows_core::BOOL);
    unsafe { ClientToScreen(hwnd, lppoint as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ClipCursor(lprect: Option<*const super::RECT>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ClipCursor(lprect : *const super::RECT) -> windows_core::BOOL);
    unsafe { ClipCursor(lprect.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CloseClipboard() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CloseClipboard() -> windows_core::BOOL);
    unsafe { CloseClipboard() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CloseDesktop(hdesktop: super::HDESK) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CloseDesktop(hdesktop : super::HDESK) -> windows_core::BOOL);
    unsafe { CloseDesktop(hdesktop) }
}
#[inline]
pub unsafe fn CloseGestureInfoHandle(hgestureinfo: HGESTUREINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CloseGestureInfoHandle(hgestureinfo : HGESTUREINFO) -> windows_core::BOOL);
    unsafe { CloseGestureInfoHandle(hgestureinfo) }
}
#[inline]
pub unsafe fn CloseTouchInputHandle(htouchinput: HTOUCHINPUT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CloseTouchInputHandle(htouchinput : HTOUCHINPUT) -> windows_core::BOOL);
    unsafe { CloseTouchInputHandle(htouchinput) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CloseWindow(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CloseWindow(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { CloseWindow(hwnd) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CloseWindowStation(hwinsta: super::HWINSTA) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CloseWindowStation(hwinsta : super::HWINSTA) -> windows_core::BOOL);
    unsafe { CloseWindowStation(hwinsta) }
}
#[inline]
pub unsafe fn ConvertPrimaryPointerToMouseDrag() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ConvertPrimaryPointerToMouseDrag() -> windows_core::BOOL);
    unsafe { ConvertPrimaryPointerToMouseDrag() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ConvertToInterceptWindow(toplevelwindow: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ConvertToInterceptWindow(toplevelwindow : super::HWND) -> windows_core::BOOL);
    unsafe { ConvertToInterceptWindow(toplevelwindow) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CopyAcceleratorTableA(haccelsrc: super::HACCEL, lpacceldst: Option<*mut ACCEL>, caccelentries: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn CopyAcceleratorTableA(haccelsrc : super::HACCEL, lpacceldst : *mut ACCEL, caccelentries : i32) -> i32);
    unsafe { CopyAcceleratorTableA(haccelsrc, lpacceldst.unwrap_or(core::mem::zeroed()) as _, caccelentries) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CopyAcceleratorTableW(haccelsrc: super::HACCEL, lpacceldst: Option<*mut ACCEL>, caccelentries: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn CopyAcceleratorTableW(haccelsrc : super::HACCEL, lpacceldst : *mut ACCEL, caccelentries : i32) -> i32);
    unsafe { CopyAcceleratorTableW(haccelsrc, lpacceldst.unwrap_or(core::mem::zeroed()) as _, caccelentries) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CopyIcon(hicon: super::HICON) -> super::HICON {
    windows_core::link!("user32.dll" "system" fn CopyIcon(hicon : super::HICON) -> super::HICON);
    unsafe { CopyIcon(hicon) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CopyImage(h: super::HANDLE, r#type: u32, cx: i32, cy: i32, flags: u32) -> super::HANDLE {
    windows_core::link!("user32.dll" "system" fn CopyImage(h : super::HANDLE, r#type : u32, cx : i32, cy : i32, flags : u32) -> super::HANDLE);
    unsafe { CopyImage(h, r#type, cx, cy, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CopyRect(lprcdst: *mut super::RECT, lprcsrc: *const super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CopyRect(lprcdst : *mut super::RECT, lprcsrc : *const super::RECT) -> windows_core::BOOL);
    unsafe { CopyRect(lprcdst as _, lprcsrc) }
}
#[inline]
pub unsafe fn CountClipboardFormats() -> i32 {
    windows_core::link!("user32.dll" "system" fn CountClipboardFormats() -> i32);
    unsafe { CountClipboardFormats() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateAcceleratorTableA(paccel: &[ACCEL]) -> super::HACCEL {
    windows_core::link!("user32.dll" "system" fn CreateAcceleratorTableA(paccel : *const ACCEL, caccel : i32) -> super::HACCEL);
    unsafe { CreateAcceleratorTableA(paccel.as_ptr(), paccel.len().try_into().unwrap()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateAcceleratorTableW(paccel: &[ACCEL]) -> super::HACCEL {
    windows_core::link!("user32.dll" "system" fn CreateAcceleratorTableW(paccel : *const ACCEL, caccel : i32) -> super::HACCEL);
    unsafe { CreateAcceleratorTableW(paccel.as_ptr(), paccel.len().try_into().unwrap()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateCaret(hwnd: super::HWND, hbitmap: Option<super::HBITMAP>, nwidth: i32, nheight: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CreateCaret(hwnd : super::HWND, hbitmap : super::HBITMAP, nwidth : i32, nheight : i32) -> windows_core::BOOL);
    unsafe { CreateCaret(hwnd, hbitmap.unwrap_or(core::mem::zeroed()) as _, nwidth, nheight) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateCursor(hinst: Option<super::HINSTANCE>, xhotspot: i32, yhotspot: i32, nwidth: i32, nheight: i32, pvandplane: *const core::ffi::c_void, pvxorplane: *const core::ffi::c_void) -> super::HCURSOR {
    windows_core::link!("user32.dll" "system" fn CreateCursor(hinst : super::HINSTANCE, xhotspot : i32, yhotspot : i32, nwidth : i32, nheight : i32, pvandplane : *const core::ffi::c_void, pvxorplane : *const core::ffi::c_void) -> super::HCURSOR);
    unsafe { CreateCursor(hinst.unwrap_or(core::mem::zeroed()) as _, xhotspot, yhotspot, nwidth, nheight, pvandplane, pvxorplane) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn CreateDesktopA<P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: Option<*const super::DEVMODEA>, dwflags: u32, dwdesiredaccess: super::ACCESS_MASK, lpsa: Option<*const super::SECURITY_ATTRIBUTES>) -> super::HDESK
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateDesktopA(lpszdesktop : windows_core::PCSTR, lpszdevice : windows_core::PCSTR, pdevmode : *const super::DEVMODEA, dwflags : u32, dwdesiredaccess : super::ACCESS_MASK, lpsa : *const super::SECURITY_ATTRIBUTES) -> super::HDESK);
    unsafe { CreateDesktopA(lpszdesktop.param().abi(), lpszdevice.param().abi(), pdevmode.unwrap_or(core::mem::zeroed()) as _, dwflags, dwdesiredaccess, lpsa.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn CreateDesktopExA<P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: Option<*const super::DEVMODEA>, dwflags: u32, dwdesiredaccess: super::ACCESS_MASK, lpsa: Option<*const super::SECURITY_ATTRIBUTES>, ulheapsize: u32, pvoid: Option<*const core::ffi::c_void>) -> super::HDESK
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateDesktopExA(lpszdesktop : windows_core::PCSTR, lpszdevice : windows_core::PCSTR, pdevmode : *const super::DEVMODEA, dwflags : u32, dwdesiredaccess : super::ACCESS_MASK, lpsa : *const super::SECURITY_ATTRIBUTES, ulheapsize : u32, pvoid : *const core::ffi::c_void) -> super::HDESK);
    unsafe { CreateDesktopExA(lpszdesktop.param().abi(), lpszdevice.param().abi(), pdevmode.unwrap_or(core::mem::zeroed()) as _, dwflags, dwdesiredaccess, lpsa.unwrap_or(core::mem::zeroed()) as _, ulheapsize, pvoid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn CreateDesktopExW<P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: Option<*const super::DEVMODEW>, dwflags: u32, dwdesiredaccess: super::ACCESS_MASK, lpsa: Option<*const super::SECURITY_ATTRIBUTES>, ulheapsize: u32, pvoid: Option<*const core::ffi::c_void>) -> super::HDESK
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateDesktopExW(lpszdesktop : windows_core::PCWSTR, lpszdevice : windows_core::PCWSTR, pdevmode : *const super::DEVMODEW, dwflags : u32, dwdesiredaccess : super::ACCESS_MASK, lpsa : *const super::SECURITY_ATTRIBUTES, ulheapsize : u32, pvoid : *const core::ffi::c_void) -> super::HDESK);
    unsafe { CreateDesktopExW(lpszdesktop.param().abi(), lpszdevice.param().abi(), pdevmode.unwrap_or(core::mem::zeroed()) as _, dwflags, dwdesiredaccess, lpsa.unwrap_or(core::mem::zeroed()) as _, ulheapsize, pvoid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn CreateDesktopW<P0, P1>(lpszdesktop: P0, lpszdevice: P1, pdevmode: Option<*const super::DEVMODEW>, dwflags: u32, dwdesiredaccess: super::ACCESS_MASK, lpsa: Option<*const super::SECURITY_ATTRIBUTES>) -> super::HDESK
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateDesktopW(lpszdesktop : windows_core::PCWSTR, lpszdevice : windows_core::PCWSTR, pdevmode : *const super::DEVMODEW, dwflags : u32, dwdesiredaccess : super::ACCESS_MASK, lpsa : *const super::SECURITY_ATTRIBUTES) -> super::HDESK);
    unsafe { CreateDesktopW(lpszdesktop.param().abi(), lpszdevice.param().abi(), pdevmode.unwrap_or(core::mem::zeroed()) as _, dwflags, dwdesiredaccess, lpsa.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateDialogIndirectParamA(hinstance: Option<super::HINSTANCE>, lptemplate: *const DLGTEMPLATE, hwndparent: Option<super::HWND>, lpdialogfunc: DLGPROC, dwinitparam: super::LPARAM) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn CreateDialogIndirectParamA(hinstance : super::HINSTANCE, lptemplate : *const DLGTEMPLATE, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> super::HWND);
    unsafe { CreateDialogIndirectParamA(hinstance.unwrap_or(core::mem::zeroed()) as _, lptemplate, hwndparent.unwrap_or(core::mem::zeroed()) as _, lpdialogfunc, dwinitparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateDialogIndirectParamW(hinstance: Option<super::HINSTANCE>, lptemplate: *const DLGTEMPLATE, hwndparent: Option<super::HWND>, lpdialogfunc: DLGPROC, dwinitparam: super::LPARAM) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn CreateDialogIndirectParamW(hinstance : super::HINSTANCE, lptemplate : *const DLGTEMPLATE, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> super::HWND);
    unsafe { CreateDialogIndirectParamW(hinstance.unwrap_or(core::mem::zeroed()) as _, lptemplate, hwndparent.unwrap_or(core::mem::zeroed()) as _, lpdialogfunc, dwinitparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateDialogParamA<P1>(hinstance: Option<super::HINSTANCE>, lptemplatename: P1, hwndparent: Option<super::HWND>, lpdialogfunc: DLGPROC, dwinitparam: super::LPARAM) -> super::HWND
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateDialogParamA(hinstance : super::HINSTANCE, lptemplatename : windows_core::PCSTR, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> super::HWND);
    unsafe { CreateDialogParamA(hinstance.unwrap_or(core::mem::zeroed()) as _, lptemplatename.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, lpdialogfunc, dwinitparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateDialogParamW<P1>(hinstance: Option<super::HINSTANCE>, lptemplatename: P1, hwndparent: Option<super::HWND>, lpdialogfunc: DLGPROC, dwinitparam: super::LPARAM) -> super::HWND
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateDialogParamW(hinstance : super::HINSTANCE, lptemplatename : windows_core::PCWSTR, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> super::HWND);
    unsafe { CreateDialogParamW(hinstance.unwrap_or(core::mem::zeroed()) as _, lptemplatename.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, lpdialogfunc, dwinitparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateIcon(hinstance: Option<super::HINSTANCE>, nwidth: i32, nheight: i32, cplanes: u8, cbitspixel: u8, lpbandbits: *const u8, lpbxorbits: *const u8) -> super::HICON {
    windows_core::link!("user32.dll" "system" fn CreateIcon(hinstance : super::HINSTANCE, nwidth : i32, nheight : i32, cplanes : u8, cbitspixel : u8, lpbandbits : *const u8, lpbxorbits : *const u8) -> super::HICON);
    unsafe { CreateIcon(hinstance.unwrap_or(core::mem::zeroed()) as _, nwidth, nheight, cplanes, cbitspixel, lpbandbits, lpbxorbits) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateIconFromResource(presbits: &[u8], ficon: bool, dwver: u32) -> super::HICON {
    windows_core::link!("user32.dll" "system" fn CreateIconFromResource(presbits : *const u8, dwressize : u32, ficon : windows_core::BOOL, dwver : u32) -> super::HICON);
    unsafe { CreateIconFromResource(presbits.as_ptr(), presbits.len().try_into().unwrap(), ficon.into(), dwver) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateIconFromResourceEx(presbits: &[u8], ficon: bool, dwver: u32, cxdesired: i32, cydesired: i32, flags: u32) -> super::HICON {
    windows_core::link!("user32.dll" "system" fn CreateIconFromResourceEx(presbits : *const u8, dwressize : u32, ficon : windows_core::BOOL, dwver : u32, cxdesired : i32, cydesired : i32, flags : u32) -> super::HICON);
    unsafe { CreateIconFromResourceEx(presbits.as_ptr(), presbits.len().try_into().unwrap(), ficon.into(), dwver, cxdesired, cydesired, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateIconIndirect(piconinfo: *const ICONINFO) -> super::HICON {
    windows_core::link!("user32.dll" "system" fn CreateIconIndirect(piconinfo : *const ICONINFO) -> super::HICON);
    unsafe { CreateIconIndirect(piconinfo) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateMDIWindowA<P0, P1>(lpclassname: P0, lpwindowname: P1, dwstyle: u32, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: Option<super::HWND>, hinstance: Option<super::HINSTANCE>, lparam: super::LPARAM) -> super::HWND
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateMDIWindowA(lpclassname : windows_core::PCSTR, lpwindowname : windows_core::PCSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::HWND, hinstance : super::HINSTANCE, lparam : super::LPARAM) -> super::HWND);
    unsafe { CreateMDIWindowA(lpclassname.param().abi(), lpwindowname.param().abi(), dwstyle, x, y, nwidth, nheight, hwndparent.unwrap_or(core::mem::zeroed()) as _, hinstance.unwrap_or(core::mem::zeroed()) as _, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateMDIWindowW<P0, P1>(lpclassname: P0, lpwindowname: P1, dwstyle: u32, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: Option<super::HWND>, hinstance: Option<super::HINSTANCE>, lparam: super::LPARAM) -> super::HWND
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateMDIWindowW(lpclassname : windows_core::PCWSTR, lpwindowname : windows_core::PCWSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::HWND, hinstance : super::HINSTANCE, lparam : super::LPARAM) -> super::HWND);
    unsafe { CreateMDIWindowW(lpclassname.param().abi(), lpwindowname.param().abi(), dwstyle, x, y, nwidth, nheight, hwndparent.unwrap_or(core::mem::zeroed()) as _, hinstance.unwrap_or(core::mem::zeroed()) as _, lparam) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreateMenu() -> super::HMENU {
    windows_core::link!("user32.dll" "system" fn CreateMenu() -> super::HMENU);
    unsafe { CreateMenu() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn CreatePopupMenu() -> super::HMENU {
    windows_core::link!("user32.dll" "system" fn CreatePopupMenu() -> super::HMENU);
    unsafe { CreatePopupMenu() }
}
#[inline]
pub unsafe fn CreateSyntheticPointerDevice(pointertype: POINTER_INPUT_TYPE, maxcount: u32, mode: POINTER_FEEDBACK_MODE) -> HSYNTHETICPOINTERDEVICE {
    windows_core::link!("user32.dll" "system" fn CreateSyntheticPointerDevice(pointertype : POINTER_INPUT_TYPE, maxcount : u32, mode : POINTER_FEEDBACK_MODE) -> HSYNTHETICPOINTERDEVICE);
    unsafe { CreateSyntheticPointerDevice(pointertype, maxcount, mode) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateWindowExA<P1, P2>(dwexstyle: u32, lpclassname: P1, lpwindowname: P2, dwstyle: u32, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: Option<super::HWND>, hmenu: Option<super::HMENU>, hinstance: Option<super::HINSTANCE>, lpparam: Option<*const core::ffi::c_void>) -> super::HWND
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateWindowExA(dwexstyle : u32, lpclassname : windows_core::PCSTR, lpwindowname : windows_core::PCSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::HWND, hmenu : super::HMENU, hinstance : super::HINSTANCE, lpparam : *const core::ffi::c_void) -> super::HWND);
    unsafe { CreateWindowExA(dwexstyle, lpclassname.param().abi(), lpwindowname.param().abi(), dwstyle, x, y, nwidth, nheight, hwndparent.unwrap_or(core::mem::zeroed()) as _, hmenu.unwrap_or(core::mem::zeroed()) as _, hinstance.unwrap_or(core::mem::zeroed()) as _, lpparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn CreateWindowExW<P1, P2>(dwexstyle: u32, lpclassname: P1, lpwindowname: P2, dwstyle: u32, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: Option<super::HWND>, hmenu: Option<super::HMENU>, hinstance: Option<super::HINSTANCE>, lpparam: Option<*const core::ffi::c_void>) -> super::HWND
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateWindowExW(dwexstyle : u32, lpclassname : windows_core::PCWSTR, lpwindowname : windows_core::PCWSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::HWND, hmenu : super::HMENU, hinstance : super::HINSTANCE, lpparam : *const core::ffi::c_void) -> super::HWND);
    unsafe { CreateWindowExW(dwexstyle, lpclassname.param().abi(), lpwindowname.param().abi(), dwstyle, x, y, nwidth, nheight, hwndparent.unwrap_or(core::mem::zeroed()) as _, hmenu.unwrap_or(core::mem::zeroed()) as _, hinstance.unwrap_or(core::mem::zeroed()) as _, lpparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn CreateWindowStationA<P0>(lpwinsta: P0, dwflags: u32, dwdesiredaccess: super::ACCESS_MASK, lpsa: Option<*const super::SECURITY_ATTRIBUTES>) -> super::HWINSTA
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateWindowStationA(lpwinsta : windows_core::PCSTR, dwflags : u32, dwdesiredaccess : super::ACCESS_MASK, lpsa : *const super::SECURITY_ATTRIBUTES) -> super::HWINSTA);
    unsafe { CreateWindowStationA(lpwinsta.param().abi(), dwflags, dwdesiredaccess, lpsa.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn CreateWindowStationW<P0>(lpwinsta: P0, dwflags: u32, dwdesiredaccess: super::ACCESS_MASK, lpsa: Option<*const super::SECURITY_ATTRIBUTES>) -> super::HWINSTA
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateWindowStationW(lpwinsta : windows_core::PCWSTR, dwflags : u32, dwdesiredaccess : super::ACCESS_MASK, lpsa : *const super::SECURITY_ATTRIBUTES) -> super::HWINSTA);
    unsafe { CreateWindowStationW(lpwinsta.param().abi(), dwflags, dwdesiredaccess, lpsa.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DefDlgProcA(hdlg: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn DefDlgProcA(hdlg : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { DefDlgProcA(hdlg, msg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DefDlgProcW(hdlg: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn DefDlgProcW(hdlg : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { DefDlgProcW(hdlg, msg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DefFrameProcA(hwnd: super::HWND, hwndmdiclient: Option<super::HWND>, umsg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn DefFrameProcA(hwnd : super::HWND, hwndmdiclient : super::HWND, umsg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { DefFrameProcA(hwnd, hwndmdiclient.unwrap_or(core::mem::zeroed()) as _, umsg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DefFrameProcW(hwnd: super::HWND, hwndmdiclient: Option<super::HWND>, umsg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn DefFrameProcW(hwnd : super::HWND, hwndmdiclient : super::HWND, umsg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { DefFrameProcW(hwnd, hwndmdiclient.unwrap_or(core::mem::zeroed()) as _, umsg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DefMDIChildProcA(hwnd: super::HWND, umsg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn DefMDIChildProcA(hwnd : super::HWND, umsg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { DefMDIChildProcA(hwnd, umsg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DefMDIChildProcW(hwnd: super::HWND, umsg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn DefMDIChildProcW(hwnd : super::HWND, umsg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { DefMDIChildProcW(hwnd, umsg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn DefRawInputProc(parawinput: &[PRAWINPUT], cbsizeheader: u32) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn DefRawInputProc(parawinput : *const PRAWINPUT, ninput : i32, cbsizeheader : u32) -> super::LRESULT);
    unsafe { DefRawInputProc(parawinput.as_ptr(), parawinput.len().try_into().unwrap(), cbsizeheader) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DefWindowProcA(hwnd: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn DefWindowProcA(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { DefWindowProcA(hwnd, msg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DefWindowProcW(hwnd: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn DefWindowProcW(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn DeferWindowPos(hwinposinfo: HDWP, hwnd: super::HWND, hwndinsertafter: Option<super::HWND>, x: i32, y: i32, cx: i32, cy: i32, uflags: u32) -> HDWP {
    windows_core::link!("user32.dll" "system" fn DeferWindowPos(hwinposinfo : HDWP, hwnd : super::HWND, hwndinsertafter : super::HWND, x : i32, y : i32, cx : i32, cy : i32, uflags : u32) -> HDWP);
    unsafe { DeferWindowPos(hwinposinfo, hwnd, hwndinsertafter.unwrap_or(core::mem::zeroed()) as _, x, y, cx, cy, uflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DeleteMenu(hmenu: super::HMENU, uposition: u32, uflags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DeleteMenu(hmenu : super::HMENU, uposition : u32, uflags : u32) -> windows_core::BOOL);
    unsafe { DeleteMenu(hmenu, uposition, uflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DeregisterShellHookWindow(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DeregisterShellHookWindow(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { DeregisterShellHookWindow(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DestroyAcceleratorTable(haccel: super::HACCEL) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DestroyAcceleratorTable(haccel : super::HACCEL) -> windows_core::BOOL);
    unsafe { DestroyAcceleratorTable(haccel) }
}
#[inline]
pub unsafe fn DestroyCaret() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DestroyCaret() -> windows_core::BOOL);
    unsafe { DestroyCaret() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DestroyCursor(hcursor: super::HCURSOR) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DestroyCursor(hcursor : super::HCURSOR) -> windows_core::BOOL);
    unsafe { DestroyCursor(hcursor) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DestroyIcon(hicon: super::HICON) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DestroyIcon(hicon : super::HICON) -> windows_core::BOOL);
    unsafe { DestroyIcon(hicon) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DestroyMenu(hmenu: super::HMENU) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DestroyMenu(hmenu : super::HMENU) -> windows_core::BOOL);
    unsafe { DestroyMenu(hmenu) }
}
#[inline]
pub unsafe fn DestroySyntheticPointerDevice(device: HSYNTHETICPOINTERDEVICE) {
    windows_core::link!("user32.dll" "system" fn DestroySyntheticPointerDevice(device : HSYNTHETICPOINTERDEVICE));
    unsafe { DestroySyntheticPointerDevice(device) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DestroyWindow(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DestroyWindow(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { DestroyWindow(hwnd) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DialogBoxIndirectParamA(hinstance: Option<super::HINSTANCE>, hdialogtemplate: *const DLGTEMPLATE, hwndparent: Option<super::HWND>, lpdialogfunc: DLGPROC, dwinitparam: super::LPARAM) -> isize {
    windows_core::link!("user32.dll" "system" fn DialogBoxIndirectParamA(hinstance : super::HINSTANCE, hdialogtemplate : *const DLGTEMPLATE, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> isize);
    unsafe { DialogBoxIndirectParamA(hinstance.unwrap_or(core::mem::zeroed()) as _, hdialogtemplate, hwndparent.unwrap_or(core::mem::zeroed()) as _, lpdialogfunc, dwinitparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DialogBoxIndirectParamW(hinstance: Option<super::HINSTANCE>, hdialogtemplate: *const DLGTEMPLATE, hwndparent: Option<super::HWND>, lpdialogfunc: DLGPROC, dwinitparam: super::LPARAM) -> isize {
    windows_core::link!("user32.dll" "system" fn DialogBoxIndirectParamW(hinstance : super::HINSTANCE, hdialogtemplate : *const DLGTEMPLATE, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> isize);
    unsafe { DialogBoxIndirectParamW(hinstance.unwrap_or(core::mem::zeroed()) as _, hdialogtemplate, hwndparent.unwrap_or(core::mem::zeroed()) as _, lpdialogfunc, dwinitparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DialogBoxParamA<P1>(hinstance: Option<super::HINSTANCE>, lptemplatename: P1, hwndparent: Option<super::HWND>, lpdialogfunc: DLGPROC, dwinitparam: super::LPARAM) -> isize
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn DialogBoxParamA(hinstance : super::HINSTANCE, lptemplatename : windows_core::PCSTR, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> isize);
    unsafe { DialogBoxParamA(hinstance.unwrap_or(core::mem::zeroed()) as _, lptemplatename.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, lpdialogfunc, dwinitparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DialogBoxParamW<P1>(hinstance: Option<super::HINSTANCE>, lptemplatename: P1, hwndparent: Option<super::HWND>, lpdialogfunc: DLGPROC, dwinitparam: super::LPARAM) -> isize
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn DialogBoxParamW(hinstance : super::HINSTANCE, lptemplatename : windows_core::PCWSTR, hwndparent : super::HWND, lpdialogfunc : DLGPROC, dwinitparam : super::LPARAM) -> isize);
    unsafe { DialogBoxParamW(hinstance.unwrap_or(core::mem::zeroed()) as _, lptemplatename.param().abi(), hwndparent.unwrap_or(core::mem::zeroed()) as _, lpdialogfunc, dwinitparam) }
}
#[inline]
pub unsafe fn DisableProcessWindowsGhosting() {
    windows_core::link!("user32.dll" "system" fn DisableProcessWindowsGhosting());
    unsafe { DisableProcessWindowsGhosting() }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DispatchMessageA(lpmsg: *const MSG) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn DispatchMessageA(lpmsg : *const MSG) -> super::LRESULT);
    unsafe { DispatchMessageA(lpmsg) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DispatchMessageW(lpmsg: *const MSG) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn DispatchMessageW(lpmsg : *const MSG) -> super::LRESULT);
    unsafe { DispatchMessageW(lpmsg) }
}
#[cfg(all(feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn DisplayConfigGetDeviceInfo(requestpacket: *mut super::DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32 {
    windows_core::link!("user32.dll" "system" fn DisplayConfigGetDeviceInfo(requestpacket : *mut super::DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32);
    unsafe { DisplayConfigGetDeviceInfo(requestpacket as _) }
}
#[cfg(all(feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn DisplayConfigSetDeviceInfo(setpacket: *const super::DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32 {
    windows_core::link!("user32.dll" "system" fn DisplayConfigSetDeviceInfo(setpacket : *const super::DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32);
    unsafe { DisplayConfigSetDeviceInfo(setpacket) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DlgDirListA(hdlg: super::HWND, lppathspec: windows_core::PSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn DlgDirListA(hdlg : super::HWND, lppathspec : windows_core::PSTR, nidlistbox : i32, nidstaticpath : i32, ufiletype : u32) -> i32);
    unsafe { DlgDirListA(hdlg, lppathspec, nidlistbox, nidstaticpath, ufiletype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DlgDirListComboBoxA(hdlg: super::HWND, lppathspec: windows_core::PSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn DlgDirListComboBoxA(hdlg : super::HWND, lppathspec : windows_core::PSTR, nidcombobox : i32, nidstaticpath : i32, ufiletype : u32) -> i32);
    unsafe { DlgDirListComboBoxA(hdlg, lppathspec, nidcombobox, nidstaticpath, ufiletype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DlgDirListComboBoxW(hdlg: super::HWND, lppathspec: windows_core::PWSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn DlgDirListComboBoxW(hdlg : super::HWND, lppathspec : windows_core::PWSTR, nidcombobox : i32, nidstaticpath : i32, ufiletype : u32) -> i32);
    unsafe { DlgDirListComboBoxW(hdlg, lppathspec, nidcombobox, nidstaticpath, ufiletype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DlgDirListW(hdlg: super::HWND, lppathspec: windows_core::PWSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn DlgDirListW(hdlg : super::HWND, lppathspec : windows_core::PWSTR, nidlistbox : i32, nidstaticpath : i32, ufiletype : u32) -> i32);
    unsafe { DlgDirListW(hdlg, lppathspec, nidlistbox, nidstaticpath, ufiletype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DlgDirSelectComboBoxExA(hwnddlg: super::HWND, lpstring: windows_core::PSTR, cchout: i32, idcombobox: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DlgDirSelectComboBoxExA(hwnddlg : super::HWND, lpstring : windows_core::PSTR, cchout : i32, idcombobox : i32) -> windows_core::BOOL);
    unsafe { DlgDirSelectComboBoxExA(hwnddlg, lpstring, cchout, idcombobox) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DlgDirSelectComboBoxExW(hwnddlg: super::HWND, lpstring: windows_core::PWSTR, cchout: i32, idcombobox: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DlgDirSelectComboBoxExW(hwnddlg : super::HWND, lpstring : windows_core::PWSTR, cchout : i32, idcombobox : i32) -> windows_core::BOOL);
    unsafe { DlgDirSelectComboBoxExW(hwnddlg, lpstring, cchout, idcombobox) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DlgDirSelectExA(hwnddlg: super::HWND, lpstring: windows_core::PSTR, chcount: i32, idlistbox: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DlgDirSelectExA(hwnddlg : super::HWND, lpstring : windows_core::PSTR, chcount : i32, idlistbox : i32) -> windows_core::BOOL);
    unsafe { DlgDirSelectExA(hwnddlg, lpstring, chcount, idlistbox) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DlgDirSelectExW(hwnddlg: super::HWND, lpstring: windows_core::PWSTR, chcount: i32, idlistbox: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DlgDirSelectExW(hwnddlg : super::HWND, lpstring : windows_core::PWSTR, chcount : i32, idlistbox : i32) -> windows_core::BOOL);
    unsafe { DlgDirSelectExW(hwnddlg, lpstring, chcount, idlistbox) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DragDetect(hwnd: super::HWND, pt: super::POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DragDetect(hwnd : super::HWND, pt : super::POINT) -> windows_core::BOOL);
    unsafe { DragDetect(hwnd, pt) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DragObject(hwndparent: super::HWND, hwndfrom: super::HWND, fmt: u32, data: usize, hcur: Option<super::HCURSOR>) -> u32 {
    windows_core::link!("user32.dll" "system" fn DragObject(hwndparent : super::HWND, hwndfrom : super::HWND, fmt : u32, data : usize, hcur : super::HCURSOR) -> u32);
    unsafe { DragObject(hwndparent, hwndfrom, fmt, data, hcur.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawAnimatedRects(hwnd: Option<super::HWND>, idani: i32, lprcfrom: *const super::RECT, lprcto: *const super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DrawAnimatedRects(hwnd : super::HWND, idani : i32, lprcfrom : *const super::RECT, lprcto : *const super::RECT) -> windows_core::BOOL);
    unsafe { DrawAnimatedRects(hwnd.unwrap_or(core::mem::zeroed()) as _, idani, lprcfrom, lprcto) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawCaption(hwnd: super::HWND, hdc: super::HDC, lprect: *const super::RECT, flags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DrawCaption(hwnd : super::HWND, hdc : super::HDC, lprect : *const super::RECT, flags : u32) -> windows_core::BOOL);
    unsafe { DrawCaption(hwnd, hdc, lprect, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawEdge(hdc: super::HDC, qrc: *mut super::RECT, edge: u32, grfflags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DrawEdge(hdc : super::HDC, qrc : *mut super::RECT, edge : u32, grfflags : u32) -> windows_core::BOOL);
    unsafe { DrawEdge(hdc, qrc as _, edge, grfflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawFocusRect(hdc: super::HDC, lprc: *const super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DrawFocusRect(hdc : super::HDC, lprc : *const super::RECT) -> windows_core::BOOL);
    unsafe { DrawFocusRect(hdc, lprc) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawFrameControl(hdc: super::HDC, lprc: *mut super::RECT, utype: u32, ustate: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DrawFrameControl(hdc : super::HDC, lprc : *mut super::RECT, utype : u32, ustate : u32) -> windows_core::BOOL);
    unsafe { DrawFrameControl(hdc, lprc as _, utype, ustate) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawIcon(hdc: super::HDC, x: i32, y: i32, hicon: super::HICON) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DrawIcon(hdc : super::HDC, x : i32, y : i32, hicon : super::HICON) -> windows_core::BOOL);
    unsafe { DrawIcon(hdc, x, y, hicon) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawIconEx(hdc: super::HDC, xleft: i32, ytop: i32, hicon: super::HICON, cxwidth: i32, cywidth: i32, istepifanicur: u32, hbrflickerfreedraw: Option<super::HBRUSH>, diflags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DrawIconEx(hdc : super::HDC, xleft : i32, ytop : i32, hicon : super::HICON, cxwidth : i32, cywidth : i32, istepifanicur : u32, hbrflickerfreedraw : super::HBRUSH, diflags : u32) -> windows_core::BOOL);
    unsafe { DrawIconEx(hdc, xleft, ytop, hicon, cxwidth, cywidth, istepifanicur, hbrflickerfreedraw.unwrap_or(core::mem::zeroed()) as _, diflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawMenuBar(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DrawMenuBar(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { DrawMenuBar(hwnd) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DrawStateA(hdc: super::HDC, hbrfore: Option<super::HBRUSH>, qfncallback: DRAWSTATEPROC, ldata: super::LPARAM, wdata: super::WPARAM, x: i32, y: i32, cx: i32, cy: i32, uflags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DrawStateA(hdc : super::HDC, hbrfore : super::HBRUSH, qfncallback : DRAWSTATEPROC, ldata : super::LPARAM, wdata : super::WPARAM, x : i32, y : i32, cx : i32, cy : i32, uflags : u32) -> windows_core::BOOL);
    unsafe { DrawStateA(hdc, hbrfore.unwrap_or(core::mem::zeroed()) as _, qfncallback, ldata, wdata, x, y, cx, cy, uflags) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn DrawStateW(hdc: super::HDC, hbrfore: Option<super::HBRUSH>, qfncallback: DRAWSTATEPROC, ldata: super::LPARAM, wdata: super::WPARAM, x: i32, y: i32, cx: i32, cy: i32, uflags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DrawStateW(hdc : super::HDC, hbrfore : super::HBRUSH, qfncallback : DRAWSTATEPROC, ldata : super::LPARAM, wdata : super::WPARAM, x : i32, y : i32, cx : i32, cy : i32, uflags : u32) -> windows_core::BOOL);
    unsafe { DrawStateW(hdc, hbrfore.unwrap_or(core::mem::zeroed()) as _, qfncallback, ldata, wdata, x, y, cx, cy, uflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawTextA<P1>(hdc: super::HDC, lpchtext: P1, cchtext: i32, lprc: *mut super::RECT, format: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn DrawTextA(hdc : super::HDC, lpchtext : windows_core::PCSTR, cchtext : i32, lprc : *mut super::RECT, format : u32) -> i32);
    unsafe { DrawTextA(hdc, lpchtext.param().abi(), cchtext, lprc as _, format) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawTextExA<P1>(hdc: super::HDC, lpchtext: P1, cchtext: i32, lprc: *mut super::RECT, format: u32, lpdtp: Option<*const DRAWTEXTPARAMS>) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn DrawTextExA(hdc : super::HDC, lpchtext : windows_core::PCSTR, cchtext : i32, lprc : *mut super::RECT, format : u32, lpdtp : *const DRAWTEXTPARAMS) -> i32);
    unsafe { DrawTextExA(hdc, lpchtext.param().abi(), cchtext, lprc as _, format, lpdtp.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawTextExW<P1>(hdc: super::HDC, lpchtext: P1, cchtext: i32, lprc: *mut super::RECT, format: u32, lpdtp: Option<*const DRAWTEXTPARAMS>) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn DrawTextExW(hdc : super::HDC, lpchtext : windows_core::PCWSTR, cchtext : i32, lprc : *mut super::RECT, format : u32, lpdtp : *const DRAWTEXTPARAMS) -> i32);
    unsafe { DrawTextExW(hdc, lpchtext.param().abi(), cchtext, lprc as _, format, lpdtp.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DrawTextW<P1>(hdc: super::HDC, lpchtext: P1, cchtext: i32, lprc: *mut super::RECT, format: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn DrawTextW(hdc : super::HDC, lpchtext : windows_core::PCWSTR, cchtext : i32, lprc : *mut super::RECT, format : u32) -> i32);
    unsafe { DrawTextW(hdc, lpchtext.param().abi(), cchtext, lprc as _, format) }
}
#[inline]
pub unsafe fn EmptyClipboard() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EmptyClipboard() -> windows_core::BOOL);
    unsafe { EmptyClipboard() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn EnableMenuItem(hmenu: super::HMENU, uidenableitem: u32, uenable: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnableMenuItem(hmenu : super::HMENU, uidenableitem : u32, uenable : u32) -> windows_core::BOOL);
    unsafe { EnableMenuItem(hmenu, uidenableitem, uenable) }
}
#[inline]
pub unsafe fn EnableMouseInPointer(fenable: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnableMouseInPointer(fenable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { EnableMouseInPointer(fenable.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn EnableNonClientDpiScaling(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnableNonClientDpiScaling(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { EnableNonClientDpiScaling(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn EnableScrollBar(hwnd: super::HWND, wsbflags: u32, warrows: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnableScrollBar(hwnd : super::HWND, wsbflags : u32, warrows : u32) -> windows_core::BOOL);
    unsafe { EnableScrollBar(hwnd, wsbflags, warrows) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn EnableWindow(hwnd: super::HWND, benable: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnableWindow(hwnd : super::HWND, benable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { EnableWindow(hwnd, benable.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EndDeferWindowPos(hwinposinfo: HDWP) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EndDeferWindowPos(hwinposinfo : HDWP) -> windows_core::BOOL);
    unsafe { EndDeferWindowPos(hwinposinfo) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn EndDialog(hdlg: super::HWND, nresult: isize) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EndDialog(hdlg : super::HWND, nresult : isize) -> windows_core::BOOL);
    unsafe { EndDialog(hdlg, nresult) }
}
#[inline]
pub unsafe fn EndMenu() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EndMenu() -> windows_core::BOOL);
    unsafe { EndMenu() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn EndPaint(hwnd: super::HWND, lppaint: *const PAINTSTRUCT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EndPaint(hwnd : super::HWND, lppaint : *const PAINTSTRUCT) -> windows_core::BOOL);
    unsafe { EndPaint(hwnd, lppaint) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn EnterMoveSizeLoop(hwnd: super::HWND, ptcursor: super::POINT, movesizecode: MOVESIZE_OPERATION) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnterMoveSizeLoop(hwnd : super::HWND, ptcursor : super::POINT, movesizecode : MOVESIZE_OPERATION) -> windows_core::BOOL);
    unsafe { EnterMoveSizeLoop(hwnd, ptcursor, movesizecode) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn EnumChildWindows(hwndparent: Option<super::HWND>, lpenumfunc: WNDENUMPROC, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnumChildWindows(hwndparent : super::HWND, lpenumfunc : WNDENUMPROC, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { EnumChildWindows(hwndparent.unwrap_or(core::mem::zeroed()) as _, lpenumfunc, lparam) }
}
#[inline]
pub unsafe fn EnumClipboardFormats(format: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn EnumClipboardFormats(format : u32) -> u32);
    unsafe { EnumClipboardFormats(format) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn EnumDesktopWindows(hdesktop: Option<super::HDESK>, lpfn: WNDENUMPROC, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnumDesktopWindows(hdesktop : super::HDESK, lpfn : WNDENUMPROC, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { EnumDesktopWindows(hdesktop.unwrap_or(core::mem::zeroed()) as _, lpfn, lparam) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn EnumDesktopsA(hwinsta: Option<super::HWINSTA>, lpenumfunc: DESKTOPENUMPROCA, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnumDesktopsA(hwinsta : super::HWINSTA, lpenumfunc : DESKTOPENUMPROCA, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { EnumDesktopsA(hwinsta.unwrap_or(core::mem::zeroed()) as _, lpenumfunc, lparam) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn EnumDesktopsW(hwinsta: Option<super::HWINSTA>, lpenumfunc: DESKTOPENUMPROCW, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnumDesktopsW(hwinsta : super::HWINSTA, lpenumfunc : DESKTOPENUMPROCW, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { EnumDesktopsW(hwinsta.unwrap_or(core::mem::zeroed()) as _, lpenumfunc, lparam) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn EnumDisplayDevicesA<P0>(lpdevice: P0, idevnum: u32, lpdisplaydevice: *mut super::DISPLAY_DEVICEA, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn EnumDisplayDevicesA(lpdevice : windows_core::PCSTR, idevnum : u32, lpdisplaydevice : *mut super::DISPLAY_DEVICEA, dwflags : u32) -> windows_core::BOOL);
    unsafe { EnumDisplayDevicesA(lpdevice.param().abi(), idevnum, lpdisplaydevice as _, dwflags) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn EnumDisplayDevicesW<P0>(lpdevice: P0, idevnum: u32, lpdisplaydevice: *mut super::DISPLAY_DEVICEW, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn EnumDisplayDevicesW(lpdevice : windows_core::PCWSTR, idevnum : u32, lpdisplaydevice : *mut super::DISPLAY_DEVICEW, dwflags : u32) -> windows_core::BOOL);
    unsafe { EnumDisplayDevicesW(lpdevice.param().abi(), idevnum, lpdisplaydevice as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn EnumDisplayMonitors(hdc: Option<super::HDC>, lprcclip: Option<*const super::RECT>, lpfnenum: MONITORENUMPROC, dwdata: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnumDisplayMonitors(hdc : super::HDC, lprcclip : *const super::RECT, lpfnenum : MONITORENUMPROC, dwdata : super::LPARAM) -> windows_core::BOOL);
    unsafe { EnumDisplayMonitors(hdc.unwrap_or(core::mem::zeroed()) as _, lprcclip.unwrap_or(core::mem::zeroed()) as _, lpfnenum, dwdata) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn EnumDisplaySettingsA<P0>(lpszdevicename: P0, imodenum: u32, lpdevmode: *mut super::DEVMODEA) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn EnumDisplaySettingsA(lpszdevicename : windows_core::PCSTR, imodenum : u32, lpdevmode : *mut super::DEVMODEA) -> windows_core::BOOL);
    unsafe { EnumDisplaySettingsA(lpszdevicename.param().abi(), imodenum, lpdevmode as _) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn EnumDisplaySettingsExA<P0>(lpszdevicename: P0, imodenum: u32, lpdevmode: *mut super::DEVMODEA, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn EnumDisplaySettingsExA(lpszdevicename : windows_core::PCSTR, imodenum : u32, lpdevmode : *mut super::DEVMODEA, dwflags : u32) -> windows_core::BOOL);
    unsafe { EnumDisplaySettingsExA(lpszdevicename.param().abi(), imodenum, lpdevmode as _, dwflags) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn EnumDisplaySettingsExW<P0>(lpszdevicename: P0, imodenum: u32, lpdevmode: *mut super::DEVMODEW, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn EnumDisplaySettingsExW(lpszdevicename : windows_core::PCWSTR, imodenum : u32, lpdevmode : *mut super::DEVMODEW, dwflags : u32) -> windows_core::BOOL);
    unsafe { EnumDisplaySettingsExW(lpszdevicename.param().abi(), imodenum, lpdevmode as _, dwflags) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn EnumDisplaySettingsW<P0>(lpszdevicename: P0, imodenum: u32, lpdevmode: *mut super::DEVMODEW) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn EnumDisplaySettingsW(lpszdevicename : windows_core::PCWSTR, imodenum : u32, lpdevmode : *mut super::DEVMODEW) -> windows_core::BOOL);
    unsafe { EnumDisplaySettingsW(lpszdevicename.param().abi(), imodenum, lpdevmode as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn EnumPropsA(hwnd: super::HWND, lpenumfunc: PROPENUMPROCA) -> i32 {
    windows_core::link!("user32.dll" "system" fn EnumPropsA(hwnd : super::HWND, lpenumfunc : PROPENUMPROCA) -> i32);
    unsafe { EnumPropsA(hwnd, lpenumfunc) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn EnumPropsExA(hwnd: super::HWND, lpenumfunc: PROPENUMPROCEXA, lparam: super::LPARAM) -> i32 {
    windows_core::link!("user32.dll" "system" fn EnumPropsExA(hwnd : super::HWND, lpenumfunc : PROPENUMPROCEXA, lparam : super::LPARAM) -> i32);
    unsafe { EnumPropsExA(hwnd, lpenumfunc, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn EnumPropsExW(hwnd: super::HWND, lpenumfunc: PROPENUMPROCEXW, lparam: super::LPARAM) -> i32 {
    windows_core::link!("user32.dll" "system" fn EnumPropsExW(hwnd : super::HWND, lpenumfunc : PROPENUMPROCEXW, lparam : super::LPARAM) -> i32);
    unsafe { EnumPropsExW(hwnd, lpenumfunc, lparam) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn EnumPropsW(hwnd: super::HWND, lpenumfunc: PROPENUMPROCW) -> i32 {
    windows_core::link!("user32.dll" "system" fn EnumPropsW(hwnd : super::HWND, lpenumfunc : PROPENUMPROCW) -> i32);
    unsafe { EnumPropsW(hwnd, lpenumfunc) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn EnumThreadWindows(dwthreadid: u32, lpfn: WNDENUMPROC, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnumThreadWindows(dwthreadid : u32, lpfn : WNDENUMPROC, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { EnumThreadWindows(dwthreadid, lpfn, lparam) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn EnumWindowStationsA(lpenumfunc: WINSTAENUMPROCA, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnumWindowStationsA(lpenumfunc : WINSTAENUMPROCA, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { EnumWindowStationsA(lpenumfunc, lparam) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn EnumWindowStationsW(lpenumfunc: WINSTAENUMPROCW, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnumWindowStationsW(lpenumfunc : WINSTAENUMPROCW, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { EnumWindowStationsW(lpenumfunc, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn EnumWindows(lpenumfunc: WNDENUMPROC, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnumWindows(lpenumfunc : WNDENUMPROC, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { EnumWindows(lpenumfunc, lparam) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn EqualRect(lprc1: *const super::RECT, lprc2: *const super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EqualRect(lprc1 : *const super::RECT, lprc2 : *const super::RECT) -> windows_core::BOOL);
    unsafe { EqualRect(lprc1, lprc2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn EvaluateProximityToPolygon(controlpolygon: &[super::POINT], phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EvaluateProximityToPolygon(numvertices : u32, controlpolygon : *const super::POINT, phittestinginput : *const TOUCH_HIT_TESTING_INPUT, pproximityeval : *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> windows_core::BOOL);
    unsafe { EvaluateProximityToPolygon(controlpolygon.len().try_into().unwrap(), controlpolygon.as_ptr(), phittestinginput, pproximityeval as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn EvaluateProximityToRect(controlboundingbox: *const super::RECT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EvaluateProximityToRect(controlboundingbox : *const super::RECT, phittestinginput : *const TOUCH_HIT_TESTING_INPUT, pproximityeval : *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> windows_core::BOOL);
    unsafe { EvaluateProximityToRect(controlboundingbox, phittestinginput, pproximityeval as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ExcludeUpdateRgn(hdc: super::HDC, hwnd: super::HWND) -> i32 {
    windows_core::link!("user32.dll" "system" fn ExcludeUpdateRgn(hdc : super::HDC, hwnd : super::HWND) -> i32);
    unsafe { ExcludeUpdateRgn(hdc, hwnd) }
}
#[inline]
pub unsafe fn ExitWindowsEx(uflags: u32, dwreason: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ExitWindowsEx(uflags : u32, dwreason : u32) -> windows_core::BOOL);
    unsafe { ExitWindowsEx(uflags, dwreason) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FillRect(hdc: super::HDC, lprc: *const super::RECT, hbr: super::HBRUSH) -> i32 {
    windows_core::link!("user32.dll" "system" fn FillRect(hdc : super::HDC, lprc : *const super::RECT, hbr : super::HBRUSH) -> i32);
    unsafe { FillRect(hdc, lprc, hbr) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FindWindowA<P0, P1>(lpclassname: P0, lpwindowname: P1) -> super::HWND
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn FindWindowA(lpclassname : windows_core::PCSTR, lpwindowname : windows_core::PCSTR) -> super::HWND);
    unsafe { FindWindowA(lpclassname.param().abi(), lpwindowname.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FindWindowExA<P2, P3>(hwndparent: Option<super::HWND>, hwndchildafter: Option<super::HWND>, lpszclass: P2, lpszwindow: P3) -> super::HWND
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn FindWindowExA(hwndparent : super::HWND, hwndchildafter : super::HWND, lpszclass : windows_core::PCSTR, lpszwindow : windows_core::PCSTR) -> super::HWND);
    unsafe { FindWindowExA(hwndparent.unwrap_or(core::mem::zeroed()) as _, hwndchildafter.unwrap_or(core::mem::zeroed()) as _, lpszclass.param().abi(), lpszwindow.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FindWindowExW<P2, P3>(hwndparent: Option<super::HWND>, hwndchildafter: Option<super::HWND>, lpszclass: P2, lpszwindow: P3) -> super::HWND
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn FindWindowExW(hwndparent : super::HWND, hwndchildafter : super::HWND, lpszclass : windows_core::PCWSTR, lpszwindow : windows_core::PCWSTR) -> super::HWND);
    unsafe { FindWindowExW(hwndparent.unwrap_or(core::mem::zeroed()) as _, hwndchildafter.unwrap_or(core::mem::zeroed()) as _, lpszclass.param().abi(), lpszwindow.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FindWindowW<P0, P1>(lpclassname: P0, lpwindowname: P1) -> super::HWND
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn FindWindowW(lpclassname : windows_core::PCWSTR, lpwindowname : windows_core::PCWSTR) -> super::HWND);
    unsafe { FindWindowW(lpclassname.param().abi(), lpwindowname.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlashWindow(hwnd: super::HWND, binvert: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn FlashWindow(hwnd : super::HWND, binvert : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { FlashWindow(hwnd, binvert.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FlashWindowEx(pfwi: *const FLASHWINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn FlashWindowEx(pfwi : *const FLASHWINFO) -> windows_core::BOOL);
    unsafe { FlashWindowEx(pfwi) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn FrameRect(hdc: super::HDC, lprc: *const super::RECT, hbr: super::HBRUSH) -> i32 {
    windows_core::link!("user32.dll" "system" fn FrameRect(hdc : super::HDC, lprc : *const super::RECT, hbr : super::HBRUSH) -> i32);
    unsafe { FrameRect(hdc, lprc, hbr) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetActiveWindow() -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetActiveWindow() -> super::HWND);
    unsafe { GetActiveWindow() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetAltTabInfoA(hwnd: Option<super::HWND>, iitem: i32, pati: *mut ALTTABINFO, pszitemtext: Option<windows_core::PSTR>, cchitemtext: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetAltTabInfoA(hwnd : super::HWND, iitem : i32, pati : *mut ALTTABINFO, pszitemtext : windows_core::PSTR, cchitemtext : u32) -> windows_core::BOOL);
    unsafe { GetAltTabInfoA(hwnd.unwrap_or(core::mem::zeroed()) as _, iitem, pati as _, pszitemtext.unwrap_or(core::mem::zeroed()) as _, cchitemtext) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetAltTabInfoW(hwnd: Option<super::HWND>, iitem: i32, pati: *mut ALTTABINFO, pszitemtext: Option<windows_core::PWSTR>, cchitemtext: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetAltTabInfoW(hwnd : super::HWND, iitem : i32, pati : *mut ALTTABINFO, pszitemtext : windows_core::PWSTR, cchitemtext : u32) -> windows_core::BOOL);
    unsafe { GetAltTabInfoW(hwnd.unwrap_or(core::mem::zeroed()) as _, iitem, pati as _, pszitemtext.unwrap_or(core::mem::zeroed()) as _, cchitemtext) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetAncestor(hwnd: super::HWND, gaflags: u32) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetAncestor(hwnd : super::HWND, gaflags : u32) -> super::HWND);
    unsafe { GetAncestor(hwnd, gaflags) }
}
#[inline]
pub unsafe fn GetAsyncKeyState(vkey: i32) -> i16 {
    windows_core::link!("user32.dll" "system" fn GetAsyncKeyState(vkey : i32) -> i16);
    unsafe { GetAsyncKeyState(vkey) }
}
#[inline]
pub unsafe fn GetAutoRotationState(pstate: *mut AR_STATE) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetAutoRotationState(pstate : *mut AR_STATE) -> windows_core::BOOL);
    unsafe { GetAutoRotationState(pstate as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetAwarenessFromDpiAwarenessContext(value: super::DPI_AWARENESS_CONTEXT) -> super::DPI_AWARENESS {
    windows_core::link!("user32.dll" "system" fn GetAwarenessFromDpiAwarenessContext(value : super::DPI_AWARENESS_CONTEXT) -> super::DPI_AWARENESS);
    unsafe { GetAwarenessFromDpiAwarenessContext(value) }
}
#[inline]
pub unsafe fn GetCIMSSM(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetCIMSSM(inputmessagesource : *mut INPUT_MESSAGE_SOURCE) -> windows_core::BOOL);
    unsafe { GetCIMSSM(inputmessagesource as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetCapture() -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetCapture() -> super::HWND);
    unsafe { GetCapture() }
}
#[inline]
pub unsafe fn GetCaretBlinkTime() -> u32 {
    windows_core::link!("user32.dll" "system" fn GetCaretBlinkTime() -> u32);
    unsafe { GetCaretBlinkTime() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetCaretPos(lppoint: *mut super::POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetCaretPos(lppoint : *mut super::POINT) -> windows_core::BOOL);
    unsafe { GetCaretPos(lppoint as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetClassInfoA<P1>(hinstance: Option<super::HINSTANCE>, lpclassname: P1, lpwndclass: *mut WNDCLASSA) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn GetClassInfoA(hinstance : super::HINSTANCE, lpclassname : windows_core::PCSTR, lpwndclass : *mut WNDCLASSA) -> windows_core::BOOL);
    unsafe { GetClassInfoA(hinstance.unwrap_or(core::mem::zeroed()) as _, lpclassname.param().abi(), lpwndclass as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetClassInfoExA<P1>(hinstance: Option<super::HINSTANCE>, lpszclass: P1, lpwcx: *mut WNDCLASSEXA) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn GetClassInfoExA(hinstance : super::HINSTANCE, lpszclass : windows_core::PCSTR, lpwcx : *mut WNDCLASSEXA) -> windows_core::BOOL);
    unsafe { GetClassInfoExA(hinstance.unwrap_or(core::mem::zeroed()) as _, lpszclass.param().abi(), lpwcx as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetClassInfoExW<P1>(hinstance: Option<super::HINSTANCE>, lpszclass: P1, lpwcx: *mut WNDCLASSEXW) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn GetClassInfoExW(hinstance : super::HINSTANCE, lpszclass : windows_core::PCWSTR, lpwcx : *mut WNDCLASSEXW) -> windows_core::BOOL);
    unsafe { GetClassInfoExW(hinstance.unwrap_or(core::mem::zeroed()) as _, lpszclass.param().abi(), lpwcx as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetClassInfoW<P1>(hinstance: Option<super::HINSTANCE>, lpclassname: P1, lpwndclass: *mut WNDCLASSW) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn GetClassInfoW(hinstance : super::HINSTANCE, lpclassname : windows_core::PCWSTR, lpwndclass : *mut WNDCLASSW) -> windows_core::BOOL);
    unsafe { GetClassInfoW(hinstance.unwrap_or(core::mem::zeroed()) as _, lpclassname.param().abi(), lpwndclass as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetClassLongA(hwnd: super::HWND, nindex: i32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetClassLongA(hwnd : super::HWND, nindex : i32) -> u32);
    unsafe { GetClassLongA(hwnd, nindex) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetClassLongPtrA(hwnd: super::HWND, nindex: i32) -> usize {
    windows_core::link!("user32.dll" "system" fn GetClassLongPtrA(hwnd : super::HWND, nindex : i32) -> usize);
    unsafe { GetClassLongPtrA(hwnd, nindex) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetClassLongPtrW(hwnd: super::HWND, nindex: i32) -> usize {
    windows_core::link!("user32.dll" "system" fn GetClassLongPtrW(hwnd : super::HWND, nindex : i32) -> usize);
    unsafe { GetClassLongPtrW(hwnd, nindex) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetClassLongW(hwnd: super::HWND, nindex: i32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetClassLongW(hwnd : super::HWND, nindex : i32) -> u32);
    unsafe { GetClassLongW(hwnd, nindex) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetClassNameA(hwnd: super::HWND, lpclassname: windows_core::PSTR, nmaxcount: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetClassNameA(hwnd : super::HWND, lpclassname : windows_core::PSTR, nmaxcount : i32) -> i32);
    unsafe { GetClassNameA(hwnd, lpclassname, nmaxcount) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetClassNameW(hwnd: super::HWND, lpclassname: windows_core::PWSTR, nmaxcount: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetClassNameW(hwnd : super::HWND, lpclassname : windows_core::PWSTR, nmaxcount : i32) -> i32);
    unsafe { GetClassNameW(hwnd, lpclassname, nmaxcount) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetClassWord(hwnd: super::HWND, nindex: i32) -> u16 {
    windows_core::link!("user32.dll" "system" fn GetClassWord(hwnd : super::HWND, nindex : i32) -> u16);
    unsafe { GetClassWord(hwnd, nindex) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetClientRect(hwnd: super::HWND, lprect: *mut super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetClientRect(hwnd : super::HWND, lprect : *mut super::RECT) -> windows_core::BOOL);
    unsafe { GetClientRect(hwnd, lprect as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetClipCursor(lprect: *mut super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetClipCursor(lprect : *mut super::RECT) -> windows_core::BOOL);
    unsafe { GetClipCursor(lprect as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetClipboardData(uformat: u32) -> super::HANDLE {
    windows_core::link!("user32.dll" "system" fn GetClipboardData(uformat : u32) -> super::HANDLE);
    unsafe { GetClipboardData(uformat) }
}
#[inline]
pub unsafe fn GetClipboardFormatNameA(format: u32, lpszformatname: windows_core::PSTR, cchmaxcount: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetClipboardFormatNameA(format : u32, lpszformatname : windows_core::PSTR, cchmaxcount : i32) -> i32);
    unsafe { GetClipboardFormatNameA(format, lpszformatname, cchmaxcount) }
}
#[inline]
pub unsafe fn GetClipboardFormatNameW(format: u32, lpszformatname: windows_core::PWSTR, cchmaxcount: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetClipboardFormatNameW(format : u32, lpszformatname : windows_core::PWSTR, cchmaxcount : i32) -> i32);
    unsafe { GetClipboardFormatNameW(format, lpszformatname, cchmaxcount) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetClipboardOwner() -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetClipboardOwner() -> super::HWND);
    unsafe { GetClipboardOwner() }
}
#[inline]
pub unsafe fn GetClipboardSequenceNumber() -> u32 {
    windows_core::link!("user32.dll" "system" fn GetClipboardSequenceNumber() -> u32);
    unsafe { GetClipboardSequenceNumber() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetClipboardViewer() -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetClipboardViewer() -> super::HWND);
    unsafe { GetClipboardViewer() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetComboBoxInfo(hwndcombo: super::HWND, pcbi: *mut COMBOBOXINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetComboBoxInfo(hwndcombo : super::HWND, pcbi : *mut COMBOBOXINFO) -> windows_core::BOOL);
    unsafe { GetComboBoxInfo(hwndcombo, pcbi as _) }
}
#[inline]
pub unsafe fn GetCurrentInputMessageSource(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetCurrentInputMessageSource(inputmessagesource : *mut INPUT_MESSAGE_SOURCE) -> windows_core::BOOL);
    unsafe { GetCurrentInputMessageSource(inputmessagesource as _) }
}
#[inline]
pub unsafe fn GetCurrentMonitorTopologyId() -> u32 {
    windows_core::link!("user32.dll" "system" fn GetCurrentMonitorTopologyId() -> u32);
    unsafe { GetCurrentMonitorTopologyId() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetCursor() -> super::HCURSOR {
    windows_core::link!("user32.dll" "system" fn GetCursor() -> super::HCURSOR);
    unsafe { GetCursor() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetCursorInfo(pci: *mut CURSORINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetCursorInfo(pci : *mut CURSORINFO) -> windows_core::BOOL);
    unsafe { GetCursorInfo(pci as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetCursorPos(lppoint: *mut super::POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetCursorPos(lppoint : *mut super::POINT) -> windows_core::BOOL);
    unsafe { GetCursorPos(lppoint as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetDC(hwnd: Option<super::HWND>) -> super::HDC {
    windows_core::link!("user32.dll" "system" fn GetDC(hwnd : super::HWND) -> super::HDC);
    unsafe { GetDC(hwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetDCEx(hwnd: Option<super::HWND>, hrgnclip: Option<super::HRGN>, flags: u32) -> super::HDC {
    windows_core::link!("user32.dll" "system" fn GetDCEx(hwnd : super::HWND, hrgnclip : super::HRGN, flags : u32) -> super::HDC);
    unsafe { GetDCEx(hwnd.unwrap_or(core::mem::zeroed()) as _, hrgnclip.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetDesktopWindow() -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetDesktopWindow() -> super::HWND);
    unsafe { GetDesktopWindow() }
}
#[inline]
pub unsafe fn GetDialogBaseUnits() -> i32 {
    windows_core::link!("user32.dll" "system" fn GetDialogBaseUnits() -> i32);
    unsafe { GetDialogBaseUnits() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetDialogControlDpiChangeBehavior(hwnd: super::HWND) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    windows_core::link!("user32.dll" "system" fn GetDialogControlDpiChangeBehavior(hwnd : super::HWND) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS);
    unsafe { GetDialogControlDpiChangeBehavior(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetDialogDpiChangeBehavior(hdlg: super::HWND) -> DIALOG_DPI_CHANGE_BEHAVIORS {
    windows_core::link!("user32.dll" "system" fn GetDialogDpiChangeBehavior(hdlg : super::HWND) -> DIALOG_DPI_CHANGE_BEHAVIORS);
    unsafe { GetDialogDpiChangeBehavior(hdlg) }
}
#[inline]
pub unsafe fn GetDisplayAutoRotationPreferences(porientation: *mut ORIENTATION_PREFERENCE) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetDisplayAutoRotationPreferences(porientation : *mut ORIENTATION_PREFERENCE) -> windows_core::BOOL);
    unsafe { GetDisplayAutoRotationPreferences(porientation as _) }
}
#[inline]
pub unsafe fn GetDisplayConfigBufferSizes(flags: u32, numpatharrayelements: *mut u32, nummodeinfoarrayelements: *mut u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetDisplayConfigBufferSizes(flags : u32, numpatharrayelements : *mut u32, nummodeinfoarrayelements : *mut u32) -> i32);
    unsafe { GetDisplayConfigBufferSizes(flags, numpatharrayelements as _, nummodeinfoarrayelements as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetDlgCtrlID(hwnd: super::HWND) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetDlgCtrlID(hwnd : super::HWND) -> i32);
    unsafe { GetDlgCtrlID(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetDlgItem(hdlg: Option<super::HWND>, niddlgitem: i32) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetDlgItem(hdlg : super::HWND, niddlgitem : i32) -> super::HWND);
    unsafe { GetDlgItem(hdlg.unwrap_or(core::mem::zeroed()) as _, niddlgitem) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetDlgItemInt(hdlg: super::HWND, niddlgitem: i32, lptranslated: Option<*mut windows_core::BOOL>, bsigned: bool) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetDlgItemInt(hdlg : super::HWND, niddlgitem : i32, lptranslated : *mut windows_core::BOOL, bsigned : windows_core::BOOL) -> u32);
    unsafe { GetDlgItemInt(hdlg, niddlgitem, lptranslated.unwrap_or(core::mem::zeroed()) as _, bsigned.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetDlgItemTextA(hdlg: super::HWND, niddlgitem: i32, lpstring: windows_core::PSTR, cchmax: i32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetDlgItemTextA(hdlg : super::HWND, niddlgitem : i32, lpstring : windows_core::PSTR, cchmax : i32) -> u32);
    unsafe { GetDlgItemTextA(hdlg, niddlgitem, lpstring, cchmax) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetDlgItemTextW(hdlg: super::HWND, niddlgitem: i32, lpstring: windows_core::PWSTR, cchmax: i32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetDlgItemTextW(hdlg : super::HWND, niddlgitem : i32, lpstring : windows_core::PWSTR, cchmax : i32) -> u32);
    unsafe { GetDlgItemTextW(hdlg, niddlgitem, lpstring, cchmax) }
}
#[inline]
pub unsafe fn GetDoubleClickTime() -> u32 {
    windows_core::link!("user32.dll" "system" fn GetDoubleClickTime() -> u32);
    unsafe { GetDoubleClickTime() }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetDpiAwarenessContextForProcess(hprocess: super::HANDLE) -> super::DPI_AWARENESS_CONTEXT {
    windows_core::link!("user32.dll" "system" fn GetDpiAwarenessContextForProcess(hprocess : super::HANDLE) -> super::DPI_AWARENESS_CONTEXT);
    unsafe { GetDpiAwarenessContextForProcess(hprocess) }
}
#[inline]
pub unsafe fn GetDpiForSystem() -> u32 {
    windows_core::link!("user32.dll" "system" fn GetDpiForSystem() -> u32);
    unsafe { GetDpiForSystem() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetDpiForWindow(hwnd: super::HWND) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetDpiForWindow(hwnd : super::HWND) -> u32);
    unsafe { GetDpiForWindow(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetDpiFromDpiAwarenessContext(value: super::DPI_AWARENESS_CONTEXT) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetDpiFromDpiAwarenessContext(value : super::DPI_AWARENESS_CONTEXT) -> u32);
    unsafe { GetDpiFromDpiAwarenessContext(value) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetFocus() -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetFocus() -> super::HWND);
    unsafe { GetFocus() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetForegroundWindow() -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetForegroundWindow() -> super::HWND);
    unsafe { GetForegroundWindow() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetGUIThreadInfo(idthread: u32, pgui: *mut GUITHREADINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetGUIThreadInfo(idthread : u32, pgui : *mut GUITHREADINFO) -> windows_core::BOOL);
    unsafe { GetGUIThreadInfo(idthread, pgui as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetGestureConfig(hwnd: super::HWND, dwreserved: u32, dwflags: u32, pcids: *const u32, pgestureconfig: *mut GESTURECONFIG, cbsize: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetGestureConfig(hwnd : super::HWND, dwreserved : u32, dwflags : u32, pcids : *const u32, pgestureconfig : *mut GESTURECONFIG, cbsize : u32) -> windows_core::BOOL);
    unsafe { GetGestureConfig(hwnd, dwreserved, dwflags, pcids, pgestureconfig as _, cbsize) }
}
#[inline]
pub unsafe fn GetGestureExtraArgs(hgestureinfo: HGESTUREINFO, cbextraargs: u32, pextraargs: *mut u8) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetGestureExtraArgs(hgestureinfo : HGESTUREINFO, cbextraargs : u32, pextraargs : *mut u8) -> windows_core::BOOL);
    unsafe { GetGestureExtraArgs(hgestureinfo, cbextraargs, pextraargs as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetGestureInfo(hgestureinfo: HGESTUREINFO, pgestureinfo: *mut GESTUREINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetGestureInfo(hgestureinfo : HGESTUREINFO, pgestureinfo : *mut GESTUREINFO) -> windows_core::BOOL);
    unsafe { GetGestureInfo(hgestureinfo, pgestureinfo as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetGuiResources(hprocess: super::HANDLE, uiflags: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetGuiResources(hprocess : super::HANDLE, uiflags : u32) -> u32);
    unsafe { GetGuiResources(hprocess, uiflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetIconInfo(hicon: super::HICON, piconinfo: *mut ICONINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetIconInfo(hicon : super::HICON, piconinfo : *mut ICONINFO) -> windows_core::BOOL);
    unsafe { GetIconInfo(hicon, piconinfo as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetIconInfoExA(hicon: super::HICON, piconinfo: *mut ICONINFOEXA) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetIconInfoExA(hicon : super::HICON, piconinfo : *mut ICONINFOEXA) -> windows_core::BOOL);
    unsafe { GetIconInfoExA(hicon, piconinfo as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetIconInfoExW(hicon: super::HICON, piconinfo: *mut ICONINFOEXW) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetIconInfoExW(hicon : super::HICON, piconinfo : *mut ICONINFOEXW) -> windows_core::BOOL);
    unsafe { GetIconInfoExW(hicon, piconinfo as _) }
}
#[inline]
pub unsafe fn GetInputState() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetInputState() -> windows_core::BOOL);
    unsafe { GetInputState() }
}
#[inline]
pub unsafe fn GetKBCodePage() -> u32 {
    windows_core::link!("user32.dll" "system" fn GetKBCodePage() -> u32);
    unsafe { GetKBCodePage() }
}
#[inline]
pub unsafe fn GetKeyNameTextA(lparam: i32, lpstring: windows_core::PSTR, cchsize: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetKeyNameTextA(lparam : i32, lpstring : windows_core::PSTR, cchsize : i32) -> i32);
    unsafe { GetKeyNameTextA(lparam, lpstring, cchsize) }
}
#[inline]
pub unsafe fn GetKeyNameTextW(lparam: i32, lpstring: windows_core::PWSTR, cchsize: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetKeyNameTextW(lparam : i32, lpstring : windows_core::PWSTR, cchsize : i32) -> i32);
    unsafe { GetKeyNameTextW(lparam, lpstring, cchsize) }
}
#[inline]
pub unsafe fn GetKeyState(nvirtkey: i32) -> i16 {
    windows_core::link!("user32.dll" "system" fn GetKeyState(nvirtkey : i32) -> i16);
    unsafe { GetKeyState(nvirtkey) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetKeyboardLayout(idthread: u32) -> super::HKL {
    windows_core::link!("user32.dll" "system" fn GetKeyboardLayout(idthread : u32) -> super::HKL);
    unsafe { GetKeyboardLayout(idthread) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetKeyboardLayoutList(nbuff: i32, lplist: Option<*mut super::HKL>) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetKeyboardLayoutList(nbuff : i32, lplist : *mut super::HKL) -> i32);
    unsafe { GetKeyboardLayoutList(nbuff, lplist.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetKeyboardLayoutNameA(pwszklid: windows_core::PSTR) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetKeyboardLayoutNameA(pwszklid : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { GetKeyboardLayoutNameA(pwszklid) }
}
#[inline]
pub unsafe fn GetKeyboardLayoutNameW(pwszklid: windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetKeyboardLayoutNameW(pwszklid : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { GetKeyboardLayoutNameW(pwszklid) }
}
#[inline]
pub unsafe fn GetKeyboardState(lpkeystate: *mut u8) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetKeyboardState(lpkeystate : *mut u8) -> windows_core::BOOL);
    unsafe { GetKeyboardState(lpkeystate as _) }
}
#[inline]
pub unsafe fn GetKeyboardType(ntypeflag: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetKeyboardType(ntypeflag : i32) -> i32);
    unsafe { GetKeyboardType(ntypeflag) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetLastActivePopup(hwnd: super::HWND) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetLastActivePopup(hwnd : super::HWND) -> super::HWND);
    unsafe { GetLastActivePopup(hwnd) }
}
#[inline]
pub unsafe fn GetLastInputInfo(plii: *mut LASTINPUTINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetLastInputInfo(plii : *mut LASTINPUTINFO) -> windows_core::BOOL);
    unsafe { GetLastInputInfo(plii as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetLayeredWindowAttributes(hwnd: super::HWND, pcrkey: Option<*mut super::COLORREF>, pbalpha: Option<*mut u8>, pdwflags: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetLayeredWindowAttributes(hwnd : super::HWND, pcrkey : *mut super::COLORREF, pbalpha : *mut u8, pdwflags : *mut u32) -> windows_core::BOOL);
    unsafe { GetLayeredWindowAttributes(hwnd, pcrkey.unwrap_or(core::mem::zeroed()) as _, pbalpha.unwrap_or(core::mem::zeroed()) as _, pdwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetListBoxInfo(hwnd: super::HWND) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetListBoxInfo(hwnd : super::HWND) -> u32);
    unsafe { GetListBoxInfo(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMenu(hwnd: super::HWND) -> super::HMENU {
    windows_core::link!("user32.dll" "system" fn GetMenu(hwnd : super::HWND) -> super::HMENU);
    unsafe { GetMenu(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMenuBarInfo(hwnd: super::HWND, idobject: i32, iditem: i32, pmbi: *mut MENUBARINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetMenuBarInfo(hwnd : super::HWND, idobject : i32, iditem : i32, pmbi : *mut MENUBARINFO) -> windows_core::BOOL);
    unsafe { GetMenuBarInfo(hwnd, idobject, iditem, pmbi as _) }
}
#[inline]
pub unsafe fn GetMenuCheckMarkDimensions() -> i32 {
    windows_core::link!("user32.dll" "system" fn GetMenuCheckMarkDimensions() -> i32);
    unsafe { GetMenuCheckMarkDimensions() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMenuContextHelpId(param0: super::HMENU) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetMenuContextHelpId(param0 : super::HMENU) -> u32);
    unsafe { GetMenuContextHelpId(param0) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMenuDefaultItem(hmenu: super::HMENU, fbypos: u32, gmdiflags: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetMenuDefaultItem(hmenu : super::HMENU, fbypos : u32, gmdiflags : u32) -> u32);
    unsafe { GetMenuDefaultItem(hmenu, fbypos, gmdiflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMenuInfo(param0: super::HMENU, param1: *mut MENUINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetMenuInfo(param0 : super::HMENU, param1 : *mut MENUINFO) -> windows_core::BOOL);
    unsafe { GetMenuInfo(param0, param1 as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMenuItemCount(hmenu: Option<super::HMENU>) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetMenuItemCount(hmenu : super::HMENU) -> i32);
    unsafe { GetMenuItemCount(hmenu.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMenuItemID(hmenu: super::HMENU, npos: i32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetMenuItemID(hmenu : super::HMENU, npos : i32) -> u32);
    unsafe { GetMenuItemID(hmenu, npos) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMenuItemInfoA(hmenu: super::HMENU, item: u32, fbyposition: bool, lpmii: *mut MENUITEMINFOA) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetMenuItemInfoA(hmenu : super::HMENU, item : u32, fbyposition : windows_core::BOOL, lpmii : *mut MENUITEMINFOA) -> windows_core::BOOL);
    unsafe { GetMenuItemInfoA(hmenu, item, fbyposition.into(), lpmii as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMenuItemInfoW(hmenu: super::HMENU, item: u32, fbyposition: bool, lpmii: *mut MENUITEMINFOW) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetMenuItemInfoW(hmenu : super::HMENU, item : u32, fbyposition : windows_core::BOOL, lpmii : *mut MENUITEMINFOW) -> windows_core::BOOL);
    unsafe { GetMenuItemInfoW(hmenu, item, fbyposition.into(), lpmii as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMenuItemRect(hwnd: Option<super::HWND>, hmenu: super::HMENU, uitem: u32, lprcitem: *mut super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetMenuItemRect(hwnd : super::HWND, hmenu : super::HMENU, uitem : u32, lprcitem : *mut super::RECT) -> windows_core::BOOL);
    unsafe { GetMenuItemRect(hwnd.unwrap_or(core::mem::zeroed()) as _, hmenu, uitem, lprcitem as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMenuState(hmenu: super::HMENU, uid: u32, uflags: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetMenuState(hmenu : super::HMENU, uid : u32, uflags : u32) -> u32);
    unsafe { GetMenuState(hmenu, uid, uflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMenuStringA(hmenu: super::HMENU, uiditem: u32, lpstring: Option<windows_core::PSTR>, cchmax: i32, flags: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetMenuStringA(hmenu : super::HMENU, uiditem : u32, lpstring : windows_core::PSTR, cchmax : i32, flags : u32) -> i32);
    unsafe { GetMenuStringA(hmenu, uiditem, lpstring.unwrap_or(core::mem::zeroed()) as _, cchmax, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMenuStringW(hmenu: super::HMENU, uiditem: u32, lpstring: Option<windows_core::PWSTR>, cchmax: i32, flags: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetMenuStringW(hmenu : super::HMENU, uiditem : u32, lpstring : windows_core::PWSTR, cchmax : i32, flags : u32) -> i32);
    unsafe { GetMenuStringW(hmenu, uiditem, lpstring.unwrap_or(core::mem::zeroed()) as _, cchmax, flags) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetMessageA(lpmsg: *mut MSG, hwnd: Option<super::HWND>, wmsgfiltermin: u32, wmsgfiltermax: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetMessageA(lpmsg : *mut MSG, hwnd : super::HWND, wmsgfiltermin : u32, wmsgfiltermax : u32) -> windows_core::BOOL);
    unsafe { GetMessageA(lpmsg as _, hwnd.unwrap_or(core::mem::zeroed()) as _, wmsgfiltermin, wmsgfiltermax) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetMessageExtraInfo() -> super::LPARAM {
    windows_core::link!("user32.dll" "system" fn GetMessageExtraInfo() -> super::LPARAM);
    unsafe { GetMessageExtraInfo() }
}
#[inline]
pub unsafe fn GetMessagePos() -> u32 {
    windows_core::link!("user32.dll" "system" fn GetMessagePos() -> u32);
    unsafe { GetMessagePos() }
}
#[inline]
pub unsafe fn GetMessageTime() -> i32 {
    windows_core::link!("user32.dll" "system" fn GetMessageTime() -> i32);
    unsafe { GetMessageTime() }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetMessageW(lpmsg: *mut MSG, hwnd: Option<super::HWND>, wmsgfiltermin: u32, wmsgfiltermax: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetMessageW(lpmsg : *mut MSG, hwnd : super::HWND, wmsgfiltermin : u32, wmsgfiltermax : u32) -> windows_core::BOOL);
    unsafe { GetMessageW(lpmsg as _, hwnd.unwrap_or(core::mem::zeroed()) as _, wmsgfiltermin, wmsgfiltermax) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMonitorInfoA(hmonitor: super::HMONITOR, lpmi: *mut MONITORINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetMonitorInfoA(hmonitor : super::HMONITOR, lpmi : *mut MONITORINFO) -> windows_core::BOOL);
    unsafe { GetMonitorInfoA(hmonitor, lpmi as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetMonitorInfoW(hmonitor: super::HMONITOR, lpmi: *mut MONITORINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetMonitorInfoW(hmonitor : super::HMONITOR, lpmi : *mut MONITORINFO) -> windows_core::BOOL);
    unsafe { GetMonitorInfoW(hmonitor, lpmi as _) }
}
#[inline]
pub unsafe fn GetMouseMovePointsEx(cbsize: u32, lppt: *const MOUSEMOVEPOINT, lpptbuf: *mut MOUSEMOVEPOINT, nbufpoints: i32, resolution: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetMouseMovePointsEx(cbsize : u32, lppt : *const MOUSEMOVEPOINT, lpptbuf : *mut MOUSEMOVEPOINT, nbufpoints : i32, resolution : u32) -> i32);
    unsafe { GetMouseMovePointsEx(cbsize, lppt, lpptbuf as _, nbufpoints, resolution) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetNextDlgGroupItem(hdlg: super::HWND, hctl: Option<super::HWND>, bprevious: bool) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetNextDlgGroupItem(hdlg : super::HWND, hctl : super::HWND, bprevious : windows_core::BOOL) -> super::HWND);
    unsafe { GetNextDlgGroupItem(hdlg, hctl.unwrap_or(core::mem::zeroed()) as _, bprevious.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetNextDlgTabItem(hdlg: super::HWND, hctl: Option<super::HWND>, bprevious: bool) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetNextDlgTabItem(hdlg : super::HWND, hctl : super::HWND, bprevious : windows_core::BOOL) -> super::HWND);
    unsafe { GetNextDlgTabItem(hdlg, hctl.unwrap_or(core::mem::zeroed()) as _, bprevious.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetOpenClipboardWindow() -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetOpenClipboardWindow() -> super::HWND);
    unsafe { GetOpenClipboardWindow() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetParent(hwnd: super::HWND) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetParent(hwnd : super::HWND) -> super::HWND);
    unsafe { GetParent(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetPhysicalCursorPos(lppoint: *mut super::POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPhysicalCursorPos(lppoint : *mut super::POINT) -> windows_core::BOOL);
    unsafe { GetPhysicalCursorPos(lppoint as _) }
}
#[inline]
pub unsafe fn GetPointerCursorId(pointerid: u32, cursorid: *mut u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerCursorId(pointerid : u32, cursorid : *mut u32) -> windows_core::BOOL);
    unsafe { GetPointerCursorId(pointerid, cursorid as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerDevice(device: super::HANDLE, pointerdevice: *mut POINTER_DEVICE_INFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerDevice(device : super::HANDLE, pointerdevice : *mut POINTER_DEVICE_INFO) -> windows_core::BOOL);
    unsafe { GetPointerDevice(device, pointerdevice as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetPointerDeviceCursors(device: super::HANDLE, cursorcount: *mut u32, devicecursors: Option<*mut POINTER_DEVICE_CURSOR_INFO>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerDeviceCursors(device : super::HANDLE, cursorcount : *mut u32, devicecursors : *mut POINTER_DEVICE_CURSOR_INFO) -> windows_core::BOOL);
    unsafe { GetPointerDeviceCursors(device, cursorcount as _, devicecursors.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetPointerDeviceProperties(device: super::HANDLE, propertycount: *mut u32, pointerproperties: Option<*mut POINTER_DEVICE_PROPERTY>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerDeviceProperties(device : super::HANDLE, propertycount : *mut u32, pointerproperties : *mut POINTER_DEVICE_PROPERTY) -> windows_core::BOOL);
    unsafe { GetPointerDeviceProperties(device, propertycount as _, pointerproperties.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerDeviceRects(device: super::HANDLE, pointerdevicerect: *mut super::RECT, displayrect: *mut super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerDeviceRects(device : super::HANDLE, pointerdevicerect : *mut super::RECT, displayrect : *mut super::RECT) -> windows_core::BOOL);
    unsafe { GetPointerDeviceRects(device, pointerdevicerect as _, displayrect as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerDevices(devicecount: *mut u32, pointerdevices: Option<*mut POINTER_DEVICE_INFO>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerDevices(devicecount : *mut u32, pointerdevices : *mut POINTER_DEVICE_INFO) -> windows_core::BOOL);
    unsafe { GetPointerDevices(devicecount as _, pointerdevices.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerFrameInfo(pointerid: u32, pointercount: *mut u32, pointerinfo: Option<*mut POINTER_INFO>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerFrameInfo(pointerid : u32, pointercount : *mut u32, pointerinfo : *mut POINTER_INFO) -> windows_core::BOOL);
    unsafe { GetPointerFrameInfo(pointerid, pointercount as _, pointerinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerFrameInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, pointerinfo: Option<*mut POINTER_INFO>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerFrameInfoHistory(pointerid : u32, entriescount : *mut u32, pointercount : *mut u32, pointerinfo : *mut POINTER_INFO) -> windows_core::BOOL);
    unsafe { GetPointerFrameInfoHistory(pointerid, entriescount as _, pointercount as _, pointerinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerFramePenInfo(pointerid: u32, pointercount: *mut u32, peninfo: Option<*mut POINTER_PEN_INFO>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerFramePenInfo(pointerid : u32, pointercount : *mut u32, peninfo : *mut POINTER_PEN_INFO) -> windows_core::BOOL);
    unsafe { GetPointerFramePenInfo(pointerid, pointercount as _, peninfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerFramePenInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, peninfo: Option<*mut POINTER_PEN_INFO>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerFramePenInfoHistory(pointerid : u32, entriescount : *mut u32, pointercount : *mut u32, peninfo : *mut POINTER_PEN_INFO) -> windows_core::BOOL);
    unsafe { GetPointerFramePenInfoHistory(pointerid, entriescount as _, pointercount as _, peninfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerFrameTouchInfo(pointerid: u32, pointercount: *mut u32, touchinfo: Option<*mut POINTER_TOUCH_INFO>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerFrameTouchInfo(pointerid : u32, pointercount : *mut u32, touchinfo : *mut POINTER_TOUCH_INFO) -> windows_core::BOOL);
    unsafe { GetPointerFrameTouchInfo(pointerid, pointercount as _, touchinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerFrameTouchInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, touchinfo: Option<*mut POINTER_TOUCH_INFO>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerFrameTouchInfoHistory(pointerid : u32, entriescount : *mut u32, pointercount : *mut u32, touchinfo : *mut POINTER_TOUCH_INFO) -> windows_core::BOOL);
    unsafe { GetPointerFrameTouchInfoHistory(pointerid, entriescount as _, pointercount as _, touchinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerInfo(pointerid: u32, pointerinfo: *mut POINTER_INFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerInfo(pointerid : u32, pointerinfo : *mut POINTER_INFO) -> windows_core::BOOL);
    unsafe { GetPointerInfo(pointerid, pointerinfo as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerInfoHistory(pointerid: u32, entriescount: *mut u32, pointerinfo: Option<*mut POINTER_INFO>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerInfoHistory(pointerid : u32, entriescount : *mut u32, pointerinfo : *mut POINTER_INFO) -> windows_core::BOOL);
    unsafe { GetPointerInfoHistory(pointerid, entriescount as _, pointerinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPointerInputTransform(pointerid: u32, historycount: u32, inputtransform: *mut INPUT_TRANSFORM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerInputTransform(pointerid : u32, historycount : u32, inputtransform : *mut INPUT_TRANSFORM) -> windows_core::BOOL);
    unsafe { GetPointerInputTransform(pointerid, historycount, inputtransform as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerPenInfo(pointerid: u32, peninfo: *mut POINTER_PEN_INFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerPenInfo(pointerid : u32, peninfo : *mut POINTER_PEN_INFO) -> windows_core::BOOL);
    unsafe { GetPointerPenInfo(pointerid, peninfo as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerPenInfoHistory(pointerid: u32, entriescount: *mut u32, peninfo: Option<*mut POINTER_PEN_INFO>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerPenInfoHistory(pointerid : u32, entriescount : *mut u32, peninfo : *mut POINTER_PEN_INFO) -> windows_core::BOOL);
    unsafe { GetPointerPenInfoHistory(pointerid, entriescount as _, peninfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerTouchInfo(pointerid: u32, touchinfo: *mut POINTER_TOUCH_INFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerTouchInfo(pointerid : u32, touchinfo : *mut POINTER_TOUCH_INFO) -> windows_core::BOOL);
    unsafe { GetPointerTouchInfo(pointerid, touchinfo as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPointerTouchInfoHistory(pointerid: u32, entriescount: *mut u32, touchinfo: Option<*mut POINTER_TOUCH_INFO>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerTouchInfoHistory(pointerid : u32, entriescount : *mut u32, touchinfo : *mut POINTER_TOUCH_INFO) -> windows_core::BOOL);
    unsafe { GetPointerTouchInfoHistory(pointerid, entriescount as _, touchinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPointerType(pointerid: u32, pointertype: *mut POINTER_INPUT_TYPE) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetPointerType(pointerid : u32, pointertype : *mut POINTER_INPUT_TYPE) -> windows_core::BOOL);
    unsafe { GetPointerType(pointerid, pointertype as _) }
}
#[inline]
pub unsafe fn GetPriorityClipboardFormat(paformatprioritylist: &[u32]) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetPriorityClipboardFormat(paformatprioritylist : *const u32, cformats : i32) -> i32);
    unsafe { GetPriorityClipboardFormat(paformatprioritylist.as_ptr(), paformatprioritylist.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetProcessDefaultLayout(pdwdefaultlayout: *mut u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetProcessDefaultLayout(pdwdefaultlayout : *mut u32) -> windows_core::BOOL);
    unsafe { GetProcessDefaultLayout(pdwdefaultlayout as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetProcessWindowStation() -> super::HWINSTA {
    windows_core::link!("user32.dll" "system" fn GetProcessWindowStation() -> super::HWINSTA);
    unsafe { GetProcessWindowStation() }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPropA<P1>(hwnd: super::HWND, lpstring: P1) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn GetPropA(hwnd : super::HWND, lpstring : windows_core::PCSTR) -> super::HANDLE);
    unsafe { GetPropA(hwnd, lpstring.param().abi()) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetPropW<P1>(hwnd: super::HWND, lpstring: P1) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn GetPropW(hwnd : super::HWND, lpstring : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { GetPropW(hwnd, lpstring.param().abi()) }
}
#[inline]
pub unsafe fn GetQueueStatus(flags: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetQueueStatus(flags : u32) -> u32);
    unsafe { GetQueueStatus(flags) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetRawInputBuffer(pdata: Option<*mut RAWINPUT>, pcbsize: *mut u32, cbsizeheader: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetRawInputBuffer(pdata : *mut RAWINPUT, pcbsize : *mut u32, cbsizeheader : u32) -> u32);
    unsafe { GetRawInputBuffer(pdata.unwrap_or(core::mem::zeroed()) as _, pcbsize as _, cbsizeheader) }
}
#[inline]
pub unsafe fn GetRawInputData(hrawinput: HRAWINPUT, uicommand: u32, pdata: Option<*mut core::ffi::c_void>, pcbsize: *mut u32, cbsizeheader: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetRawInputData(hrawinput : HRAWINPUT, uicommand : u32, pdata : *mut core::ffi::c_void, pcbsize : *mut u32, cbsizeheader : u32) -> u32);
    unsafe { GetRawInputData(hrawinput, uicommand, pdata.unwrap_or(core::mem::zeroed()) as _, pcbsize as _, cbsizeheader) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetRawInputDeviceInfoA(hdevice: Option<super::HANDLE>, uicommand: u32, pdata: Option<*mut core::ffi::c_void>, pcbsize: *mut u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetRawInputDeviceInfoA(hdevice : super::HANDLE, uicommand : u32, pdata : *mut core::ffi::c_void, pcbsize : *mut u32) -> u32);
    unsafe { GetRawInputDeviceInfoA(hdevice.unwrap_or(core::mem::zeroed()) as _, uicommand, pdata.unwrap_or(core::mem::zeroed()) as _, pcbsize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetRawInputDeviceInfoW(hdevice: Option<super::HANDLE>, uicommand: u32, pdata: Option<*mut core::ffi::c_void>, pcbsize: *mut u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetRawInputDeviceInfoW(hdevice : super::HANDLE, uicommand : u32, pdata : *mut core::ffi::c_void, pcbsize : *mut u32) -> u32);
    unsafe { GetRawInputDeviceInfoW(hdevice.unwrap_or(core::mem::zeroed()) as _, uicommand, pdata.unwrap_or(core::mem::zeroed()) as _, pcbsize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetRawInputDeviceList(prawinputdevicelist: Option<*mut RAWINPUTDEVICELIST>, puinumdevices: *mut u32, cbsize: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetRawInputDeviceList(prawinputdevicelist : *mut RAWINPUTDEVICELIST, puinumdevices : *mut u32, cbsize : u32) -> u32);
    unsafe { GetRawInputDeviceList(prawinputdevicelist.unwrap_or(core::mem::zeroed()) as _, puinumdevices as _, cbsize) }
}
#[inline]
pub unsafe fn GetRawPointerDeviceData(pointerid: u32, historycount: u32, pproperties: &[POINTER_DEVICE_PROPERTY], pvalues: *mut i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetRawPointerDeviceData(pointerid : u32, historycount : u32, propertiescount : u32, pproperties : *const POINTER_DEVICE_PROPERTY, pvalues : *mut i32) -> windows_core::BOOL);
    unsafe { GetRawPointerDeviceData(pointerid, historycount, pproperties.len().try_into().unwrap(), pproperties.as_ptr(), pvalues as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetRegisteredRawInputDevices(prawinputdevices: Option<*mut RAWINPUTDEVICE>, puinumdevices: *mut u32, cbsize: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetRegisteredRawInputDevices(prawinputdevices : *mut RAWINPUTDEVICE, puinumdevices : *mut u32, cbsize : u32) -> u32);
    unsafe { GetRegisteredRawInputDevices(prawinputdevices.unwrap_or(core::mem::zeroed()) as _, puinumdevices as _, cbsize) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetScrollBarInfo(hwnd: super::HWND, idobject: i32, psbi: *mut SCROLLBARINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetScrollBarInfo(hwnd : super::HWND, idobject : i32, psbi : *mut SCROLLBARINFO) -> windows_core::BOOL);
    unsafe { GetScrollBarInfo(hwnd, idobject, psbi as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetScrollInfo(hwnd: super::HWND, nbar: i32, lpsi: *mut SCROLLINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetScrollInfo(hwnd : super::HWND, nbar : i32, lpsi : *mut SCROLLINFO) -> windows_core::BOOL);
    unsafe { GetScrollInfo(hwnd, nbar, lpsi as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetScrollPos(hwnd: super::HWND, nbar: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetScrollPos(hwnd : super::HWND, nbar : i32) -> i32);
    unsafe { GetScrollPos(hwnd, nbar) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetScrollRange(hwnd: super::HWND, nbar: i32, lpminpos: *mut i32, lpmaxpos: *mut i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetScrollRange(hwnd : super::HWND, nbar : i32, lpminpos : *mut i32, lpmaxpos : *mut i32) -> windows_core::BOOL);
    unsafe { GetScrollRange(hwnd, nbar, lpminpos as _, lpmaxpos as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetShellWindow() -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetShellWindow() -> super::HWND);
    unsafe { GetShellWindow() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetSubMenu(hmenu: super::HMENU, npos: i32) -> super::HMENU {
    windows_core::link!("user32.dll" "system" fn GetSubMenu(hmenu : super::HMENU, npos : i32) -> super::HMENU);
    unsafe { GetSubMenu(hmenu, npos) }
}
#[inline]
pub unsafe fn GetSysColor(nindex: i32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetSysColor(nindex : i32) -> u32);
    unsafe { GetSysColor(nindex) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetSysColorBrush(nindex: i32) -> super::HBRUSH {
    windows_core::link!("user32.dll" "system" fn GetSysColorBrush(nindex : i32) -> super::HBRUSH);
    unsafe { GetSysColorBrush(nindex) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetSystemDpiForProcess(hprocess: super::HANDLE) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetSystemDpiForProcess(hprocess : super::HANDLE) -> u32);
    unsafe { GetSystemDpiForProcess(hprocess) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetSystemMenu(hwnd: super::HWND, brevert: bool) -> super::HMENU {
    windows_core::link!("user32.dll" "system" fn GetSystemMenu(hwnd : super::HWND, brevert : windows_core::BOOL) -> super::HMENU);
    unsafe { GetSystemMenu(hwnd, brevert.into()) }
}
#[inline]
pub unsafe fn GetSystemMetrics(nindex: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetSystemMetrics(nindex : i32) -> i32);
    unsafe { GetSystemMetrics(nindex) }
}
#[inline]
pub unsafe fn GetSystemMetricsForDpi(nindex: i32, dpi: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetSystemMetricsForDpi(nindex : i32, dpi : u32) -> i32);
    unsafe { GetSystemMetricsForDpi(nindex, dpi) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetTabbedTextExtentA(hdc: super::HDC, lpstring: &[u8], lpntabstoppositions: Option<&[i32]>) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetTabbedTextExtentA(hdc : super::HDC, lpstring : windows_core::PCSTR, chcount : i32, ntabpositions : i32, lpntabstoppositions : *const i32) -> u32);
    unsafe { GetTabbedTextExtentA(hdc, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), lpntabstoppositions.map_or(0, |slice| slice.len().try_into().unwrap()), lpntabstoppositions.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetTabbedTextExtentW(hdc: super::HDC, lpstring: &[u16], lpntabstoppositions: Option<&[i32]>) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetTabbedTextExtentW(hdc : super::HDC, lpstring : windows_core::PCWSTR, chcount : i32, ntabpositions : i32, lpntabstoppositions : *const i32) -> u32);
    unsafe { GetTabbedTextExtentW(hdc, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), lpntabstoppositions.map_or(0, |slice| slice.len().try_into().unwrap()), lpntabstoppositions.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetThreadDesktop(dwthreadid: u32) -> super::HDESK {
    windows_core::link!("user32.dll" "system" fn GetThreadDesktop(dwthreadid : u32) -> super::HDESK);
    unsafe { GetThreadDesktop(dwthreadid) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetThreadDpiAwarenessContext() -> super::DPI_AWARENESS_CONTEXT {
    windows_core::link!("user32.dll" "system" fn GetThreadDpiAwarenessContext() -> super::DPI_AWARENESS_CONTEXT);
    unsafe { GetThreadDpiAwarenessContext() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetThreadDpiHostingBehavior() -> super::DPI_HOSTING_BEHAVIOR {
    windows_core::link!("user32.dll" "system" fn GetThreadDpiHostingBehavior() -> super::DPI_HOSTING_BEHAVIOR);
    unsafe { GetThreadDpiHostingBehavior() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetTitleBarInfo(hwnd: super::HWND, pti: *mut TITLEBARINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetTitleBarInfo(hwnd : super::HWND, pti : *mut TITLEBARINFO) -> windows_core::BOOL);
    unsafe { GetTitleBarInfo(hwnd, pti as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetTopWindow(hwnd: Option<super::HWND>) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetTopWindow(hwnd : super::HWND) -> super::HWND);
    unsafe { GetTopWindow(hwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetTouchInputInfo(htouchinput: HTOUCHINPUT, cinputs: u32, pinputs: *mut TOUCHINPUT, cbsize: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetTouchInputInfo(htouchinput : HTOUCHINPUT, cinputs : u32, pinputs : *mut TOUCHINPUT, cbsize : i32) -> windows_core::BOOL);
    unsafe { GetTouchInputInfo(htouchinput, cinputs, pinputs as _, cbsize) }
}
#[inline]
pub unsafe fn GetUnpredictedMessagePos() -> u32 {
    windows_core::link!("user32.dll" "system" fn GetUnpredictedMessagePos() -> u32);
    unsafe { GetUnpredictedMessagePos() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetUpdateRect(hwnd: super::HWND, lprect: Option<*mut super::RECT>, berase: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetUpdateRect(hwnd : super::HWND, lprect : *mut super::RECT, berase : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetUpdateRect(hwnd, lprect.unwrap_or(core::mem::zeroed()) as _, berase.into()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetUpdateRgn(hwnd: super::HWND, hrgn: super::HRGN, berase: bool) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetUpdateRgn(hwnd : super::HWND, hrgn : super::HRGN, berase : windows_core::BOOL) -> i32);
    unsafe { GetUpdateRgn(hwnd, hrgn, berase.into()) }
}
#[inline]
pub unsafe fn GetUpdatedClipboardFormats(lpuiformats: *mut u32, cformats: u32, pcformatsout: *mut u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetUpdatedClipboardFormats(lpuiformats : *mut u32, cformats : u32, pcformatsout : *mut u32) -> windows_core::BOOL);
    unsafe { GetUpdatedClipboardFormats(lpuiformats as _, cformats, pcformatsout as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetUserObjectInformationA(hobj: super::HANDLE, nindex: i32, pvinfo: Option<*mut core::ffi::c_void>, nlength: u32, lpnlengthneeded: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetUserObjectInformationA(hobj : super::HANDLE, nindex : i32, pvinfo : *mut core::ffi::c_void, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetUserObjectInformationA(hobj, nindex, pvinfo.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetUserObjectInformationW(hobj: super::HANDLE, nindex: i32, pvinfo: Option<*mut core::ffi::c_void>, nlength: u32, lpnlengthneeded: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetUserObjectInformationW(hobj : super::HANDLE, nindex : i32, pvinfo : *mut core::ffi::c_void, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetUserObjectInformationW(hobj, nindex, pvinfo.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetUserObjectSecurity(hobj: super::HANDLE, psirequested: *const u32, psid: Option<super::PSECURITY_DESCRIPTOR>, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetUserObjectSecurity(hobj : super::HANDLE, psirequested : *const u32, psid : super::PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetUserObjectSecurity(hobj, psirequested, psid.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindow(hwnd: super::HWND, ucmd: u32) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn GetWindow(hwnd : super::HWND, ucmd : u32) -> super::HWND);
    unsafe { GetWindow(hwnd, ucmd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowContextHelpId(param0: super::HWND) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetWindowContextHelpId(param0 : super::HWND) -> u32);
    unsafe { GetWindowContextHelpId(param0) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowDC(hwnd: Option<super::HWND>) -> super::HDC {
    windows_core::link!("user32.dll" "system" fn GetWindowDC(hwnd : super::HWND) -> super::HDC);
    unsafe { GetWindowDC(hwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowDisplayAffinity(hwnd: super::HWND, pdwaffinity: *mut u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetWindowDisplayAffinity(hwnd : super::HWND, pdwaffinity : *mut u32) -> windows_core::BOOL);
    unsafe { GetWindowDisplayAffinity(hwnd, pdwaffinity as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowDpiAwarenessContext(hwnd: super::HWND) -> super::DPI_AWARENESS_CONTEXT {
    windows_core::link!("user32.dll" "system" fn GetWindowDpiAwarenessContext(hwnd : super::HWND) -> super::DPI_AWARENESS_CONTEXT);
    unsafe { GetWindowDpiAwarenessContext(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowDpiHostingBehavior(hwnd: super::HWND) -> super::DPI_HOSTING_BEHAVIOR {
    windows_core::link!("user32.dll" "system" fn GetWindowDpiHostingBehavior(hwnd : super::HWND) -> super::DPI_HOSTING_BEHAVIOR);
    unsafe { GetWindowDpiHostingBehavior(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowFeedbackSetting(hwnd: super::HWND, feedback: FEEDBACK_TYPE, dwflags: u32, psize: *mut u32, config: Option<*mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetWindowFeedbackSetting(hwnd : super::HWND, feedback : FEEDBACK_TYPE, dwflags : u32, psize : *mut u32, config : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetWindowFeedbackSetting(hwnd, feedback, dwflags, psize as _, config.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetWindowInfo(hwnd: super::HWND, pwi: *mut WINDOWINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetWindowInfo(hwnd : super::HWND, pwi : *mut WINDOWINFO) -> windows_core::BOOL);
    unsafe { GetWindowInfo(hwnd, pwi as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowLongA(hwnd: super::HWND, nindex: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetWindowLongA(hwnd : super::HWND, nindex : i32) -> i32);
    unsafe { GetWindowLongA(hwnd, nindex) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowLongPtrA(hwnd: super::HWND, nindex: i32) -> isize {
    windows_core::link!("user32.dll" "system" fn GetWindowLongPtrA(hwnd : super::HWND, nindex : i32) -> isize);
    unsafe { GetWindowLongPtrA(hwnd, nindex) }
}
#[cfg(target_pointer_width = "32")]
pub use GetWindowLongA as GetWindowLongPtrA;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowLongPtrW(hwnd: super::HWND, nindex: i32) -> isize {
    windows_core::link!("user32.dll" "system" fn GetWindowLongPtrW(hwnd : super::HWND, nindex : i32) -> isize);
    unsafe { GetWindowLongPtrW(hwnd, nindex) }
}
#[cfg(target_pointer_width = "32")]
pub use GetWindowLongW as GetWindowLongPtrW;
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowLongW(hwnd: super::HWND, nindex: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetWindowLongW(hwnd : super::HWND, nindex : i32) -> i32);
    unsafe { GetWindowLongW(hwnd, nindex) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowModuleFileNameA(hwnd: super::HWND, pszfilename: windows_core::PSTR, cchfilenamemax: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetWindowModuleFileNameA(hwnd : super::HWND, pszfilename : windows_core::PSTR, cchfilenamemax : u32) -> u32);
    unsafe { GetWindowModuleFileNameA(hwnd, pszfilename, cchfilenamemax) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowModuleFileNameW(hwnd: super::HWND, pszfilename: windows_core::PWSTR, cchfilenamemax: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetWindowModuleFileNameW(hwnd : super::HWND, pszfilename : windows_core::PWSTR, cchfilenamemax : u32) -> u32);
    unsafe { GetWindowModuleFileNameW(hwnd, pszfilename, cchfilenamemax) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowPlacement(hwnd: super::HWND, lpwndpl: *mut WINDOWPLACEMENT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetWindowPlacement(hwnd : super::HWND, lpwndpl : *mut WINDOWPLACEMENT) -> windows_core::BOOL);
    unsafe { GetWindowPlacement(hwnd, lpwndpl as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowRect(hwnd: super::HWND, lprect: *mut super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetWindowRect(hwnd : super::HWND, lprect : *mut super::RECT) -> windows_core::BOOL);
    unsafe { GetWindowRect(hwnd, lprect as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetWindowRgn(hwnd: super::HWND, hrgn: super::HRGN) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetWindowRgn(hwnd : super::HWND, hrgn : super::HRGN) -> i32);
    unsafe { GetWindowRgn(hwnd, hrgn) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowRgnBox(hwnd: super::HWND, lprc: *mut super::RECT) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetWindowRgnBox(hwnd : super::HWND, lprc : *mut super::RECT) -> i32);
    unsafe { GetWindowRgnBox(hwnd, lprc as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowTextA(hwnd: super::HWND, lpstring: windows_core::PSTR, nmaxcount: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetWindowTextA(hwnd : super::HWND, lpstring : windows_core::PSTR, nmaxcount : i32) -> i32);
    unsafe { GetWindowTextA(hwnd, lpstring, nmaxcount) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowTextLengthA(hwnd: super::HWND) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetWindowTextLengthA(hwnd : super::HWND) -> i32);
    unsafe { GetWindowTextLengthA(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowTextLengthW(hwnd: super::HWND) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetWindowTextLengthW(hwnd : super::HWND) -> i32);
    unsafe { GetWindowTextLengthW(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowTextW(hwnd: super::HWND, lpstring: windows_core::PWSTR, nmaxcount: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetWindowTextW(hwnd : super::HWND, lpstring : windows_core::PWSTR, nmaxcount : i32) -> i32);
    unsafe { GetWindowTextW(hwnd, lpstring, nmaxcount) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowThreadProcessId(hwnd: super::HWND, lpdwprocessid: Option<*mut u32>) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetWindowThreadProcessId(hwnd : super::HWND, lpdwprocessid : *mut u32) -> u32);
    unsafe { GetWindowThreadProcessId(hwnd, lpdwprocessid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetWindowWord(hwnd: super::HWND, nindex: i32) -> u16 {
    windows_core::link!("user32.dll" "system" fn GetWindowWord(hwnd : super::HWND, nindex : i32) -> u16);
    unsafe { GetWindowWord(hwnd, nindex) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GrayStringA(hdc: super::HDC, hbrush: Option<super::HBRUSH>, lpoutputfunc: GRAYSTRINGPROC, lpdata: super::LPARAM, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GrayStringA(hdc : super::HDC, hbrush : super::HBRUSH, lpoutputfunc : GRAYSTRINGPROC, lpdata : super::LPARAM, ncount : i32, x : i32, y : i32, nwidth : i32, nheight : i32) -> windows_core::BOOL);
    unsafe { GrayStringA(hdc, hbrush.unwrap_or(core::mem::zeroed()) as _, lpoutputfunc, lpdata, ncount, x, y, nwidth, nheight) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GrayStringW(hdc: super::HDC, hbrush: Option<super::HBRUSH>, lpoutputfunc: GRAYSTRINGPROC, lpdata: super::LPARAM, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GrayStringW(hdc : super::HDC, hbrush : super::HBRUSH, lpoutputfunc : GRAYSTRINGPROC, lpdata : super::LPARAM, ncount : i32, x : i32, y : i32, nwidth : i32, nheight : i32) -> windows_core::BOOL);
    unsafe { GrayStringW(hdc, hbrush.unwrap_or(core::mem::zeroed()) as _, lpoutputfunc, lpdata, ncount, x, y, nwidth, nheight) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HideCaret(hwnd: Option<super::HWND>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn HideCaret(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { HideCaret(hwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HiliteMenuItem(hwnd: super::HWND, hmenu: super::HMENU, uidhiliteitem: u32, uhilite: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn HiliteMenuItem(hwnd : super::HWND, hmenu : super::HMENU, uidhiliteitem : u32, uhilite : u32) -> windows_core::BOOL);
    unsafe { HiliteMenuItem(hwnd, hmenu, uidhiliteitem, uhilite) }
}
#[inline]
pub unsafe fn InSendMessage() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn InSendMessage() -> windows_core::BOOL);
    unsafe { InSendMessage() }
}
#[inline]
pub unsafe fn InSendMessageEx(lpreserved: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("user32.dll" "system" fn InSendMessageEx(lpreserved : *const core::ffi::c_void) -> u32);
    unsafe { InSendMessageEx(lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InflateRect(lprc: *mut super::RECT, dx: i32, dy: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn InflateRect(lprc : *mut super::RECT, dx : i32, dy : i32) -> windows_core::BOOL);
    unsafe { InflateRect(lprc as _, dx, dy) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InheritWindowMonitor(hwnd: super::HWND, hwndinherit: Option<super::HWND>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn InheritWindowMonitor(hwnd : super::HWND, hwndinherit : super::HWND) -> windows_core::BOOL);
    unsafe { InheritWindowMonitor(hwnd, hwndinherit.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InitializeTouchInjection(maxcount: u32, dwmode: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn InitializeTouchInjection(maxcount : u32, dwmode : u32) -> windows_core::BOOL);
    unsafe { InitializeTouchInjection(maxcount, dwmode) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn InjectSyntheticPointerInput(device: HSYNTHETICPOINTERDEVICE, pointerinfo: &[POINTER_TYPE_INFO]) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn InjectSyntheticPointerInput(device : HSYNTHETICPOINTERDEVICE, pointerinfo : *const POINTER_TYPE_INFO, count : u32) -> windows_core::BOOL);
    unsafe { InjectSyntheticPointerInput(device, pointerinfo.as_ptr(), pointerinfo.len().try_into().unwrap()) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn InjectTouchInput(contacts: &[POINTER_TOUCH_INFO]) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn InjectTouchInput(count : u32, contacts : *const POINTER_TOUCH_INFO) -> windows_core::BOOL);
    unsafe { InjectTouchInput(contacts.len().try_into().unwrap(), contacts.as_ptr()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InsertMenuA<P4>(hmenu: super::HMENU, uposition: u32, uflags: u32, uidnewitem: usize, lpnewitem: P4) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn InsertMenuA(hmenu : super::HMENU, uposition : u32, uflags : u32, uidnewitem : usize, lpnewitem : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { InsertMenuA(hmenu, uposition, uflags, uidnewitem, lpnewitem.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InsertMenuItemA(hmenu: super::HMENU, item: u32, fbyposition: bool, lpmi: *const MENUITEMINFOA) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn InsertMenuItemA(hmenu : super::HMENU, item : u32, fbyposition : windows_core::BOOL, lpmi : *const MENUITEMINFOA) -> windows_core::BOOL);
    unsafe { InsertMenuItemA(hmenu, item, fbyposition.into(), lpmi) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InsertMenuItemW(hmenu: super::HMENU, item: u32, fbyposition: bool, lpmi: *const MENUITEMINFOW) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn InsertMenuItemW(hmenu : super::HMENU, item : u32, fbyposition : windows_core::BOOL, lpmi : *const MENUITEMINFOW) -> windows_core::BOOL);
    unsafe { InsertMenuItemW(hmenu, item, fbyposition.into(), lpmi) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InsertMenuW<P4>(hmenu: super::HMENU, uposition: u32, uflags: u32, uidnewitem: usize, lpnewitem: P4) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn InsertMenuW(hmenu : super::HMENU, uposition : u32, uflags : u32, uidnewitem : usize, lpnewitem : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { InsertMenuW(hmenu, uposition, uflags, uidnewitem, lpnewitem.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InternalGetWindowText(hwnd: super::HWND, pstring: windows_core::PWSTR, cchmaxcount: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn InternalGetWindowText(hwnd : super::HWND, pstring : windows_core::PWSTR, cchmaxcount : i32) -> i32);
    unsafe { InternalGetWindowText(hwnd, pstring, cchmaxcount) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IntersectRect(lprcdst: *mut super::RECT, lprcsrc1: *const super::RECT, lprcsrc2: *const super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IntersectRect(lprcdst : *mut super::RECT, lprcsrc1 : *const super::RECT, lprcsrc2 : *const super::RECT) -> windows_core::BOOL);
    unsafe { IntersectRect(lprcdst as _, lprcsrc1, lprcsrc2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InvalidateRect(hwnd: Option<super::HWND>, lprect: Option<*const super::RECT>, berase: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn InvalidateRect(hwnd : super::HWND, lprect : *const super::RECT, berase : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { InvalidateRect(hwnd.unwrap_or(core::mem::zeroed()) as _, lprect.unwrap_or(core::mem::zeroed()) as _, berase.into()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn InvalidateRgn(hwnd: super::HWND, hrgn: Option<super::HRGN>, berase: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn InvalidateRgn(hwnd : super::HWND, hrgn : super::HRGN, berase : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { InvalidateRgn(hwnd, hrgn.unwrap_or(core::mem::zeroed()) as _, berase.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn InvertRect(hdc: super::HDC, lprc: *const super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn InvertRect(hdc : super::HDC, lprc : *const super::RECT) -> windows_core::BOOL);
    unsafe { InvertRect(hdc, lprc) }
}
#[inline]
pub unsafe fn IsCharAlphaA(ch: i8) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsCharAlphaA(ch : i8) -> windows_core::BOOL);
    unsafe { IsCharAlphaA(ch) }
}
#[inline]
pub unsafe fn IsCharAlphaNumericA(ch: i8) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsCharAlphaNumericA(ch : i8) -> windows_core::BOOL);
    unsafe { IsCharAlphaNumericA(ch) }
}
#[inline]
pub unsafe fn IsCharAlphaNumericW(ch: u16) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsCharAlphaNumericW(ch : u16) -> windows_core::BOOL);
    unsafe { IsCharAlphaNumericW(ch) }
}
#[inline]
pub unsafe fn IsCharAlphaW(ch: u16) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsCharAlphaW(ch : u16) -> windows_core::BOOL);
    unsafe { IsCharAlphaW(ch) }
}
#[inline]
pub unsafe fn IsCharLowerA(ch: i8) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsCharLowerA(ch : i8) -> windows_core::BOOL);
    unsafe { IsCharLowerA(ch) }
}
#[inline]
pub unsafe fn IsCharLowerW(ch: u16) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsCharLowerW(ch : u16) -> windows_core::BOOL);
    unsafe { IsCharLowerW(ch) }
}
#[inline]
pub unsafe fn IsCharUpperA(ch: i8) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsCharUpperA(ch : i8) -> windows_core::BOOL);
    unsafe { IsCharUpperA(ch) }
}
#[inline]
pub unsafe fn IsCharUpperW(ch: u16) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsCharUpperW(ch : u16) -> windows_core::BOOL);
    unsafe { IsCharUpperW(ch) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsChild(hwndparent: super::HWND, hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsChild(hwndparent : super::HWND, hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { IsChild(hwndparent, hwnd) }
}
#[inline]
pub unsafe fn IsClipboardFormatAvailable(format: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsClipboardFormatAvailable(format : u32) -> windows_core::BOOL);
    unsafe { IsClipboardFormatAvailable(format) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn IsDialogMessageA(hdlg: super::HWND, lpmsg: *const MSG) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsDialogMessageA(hdlg : super::HWND, lpmsg : *const MSG) -> windows_core::BOOL);
    unsafe { IsDialogMessageA(hdlg, lpmsg) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn IsDialogMessageW(hdlg: super::HWND, lpmsg: *const MSG) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsDialogMessageW(hdlg : super::HWND, lpmsg : *const MSG) -> windows_core::BOOL);
    unsafe { IsDialogMessageW(hdlg, lpmsg) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsDlgButtonChecked(hdlg: super::HWND, nidbutton: i32) -> u32 {
    windows_core::link!("user32.dll" "system" fn IsDlgButtonChecked(hdlg : super::HWND, nidbutton : i32) -> u32);
    unsafe { IsDlgButtonChecked(hdlg, nidbutton) }
}
#[inline]
pub unsafe fn IsGUIThread(bconvert: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsGUIThread(bconvert : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { IsGUIThread(bconvert.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsHungAppWindow(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsHungAppWindow(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { IsHungAppWindow(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsIconic(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsIconic(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { IsIconic(hwnd) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsImmersiveProcess(hprocess: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsImmersiveProcess(hprocess : super::HANDLE) -> windows_core::BOOL);
    unsafe { IsImmersiveProcess(hprocess) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsInterceptWindow(toplevelwindow: super::HWND, isintercept: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsInterceptWindow(toplevelwindow : super::HWND, isintercept : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { IsInterceptWindow(toplevelwindow, isintercept as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsMenu(hmenu: super::HMENU) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsMenu(hmenu : super::HMENU) -> windows_core::BOOL);
    unsafe { IsMenu(hmenu) }
}
#[inline]
pub unsafe fn IsMouseInPointerEnabled() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsMouseInPointerEnabled() -> windows_core::BOOL);
    unsafe { IsMouseInPointerEnabled() }
}
#[inline]
pub unsafe fn IsProcessDPIAware() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsProcessDPIAware() -> windows_core::BOOL);
    unsafe { IsProcessDPIAware() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsRectEmpty(lprc: *const super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsRectEmpty(lprc : *const super::RECT) -> windows_core::BOOL);
    unsafe { IsRectEmpty(lprc) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsTouchWindow(hwnd: super::HWND, pulflags: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsTouchWindow(hwnd : super::HWND, pulflags : *mut u32) -> windows_core::BOOL);
    unsafe { IsTouchWindow(hwnd, pulflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsValidDpiAwarenessContext(value: super::DPI_AWARENESS_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsValidDpiAwarenessContext(value : super::DPI_AWARENESS_CONTEXT) -> windows_core::BOOL);
    unsafe { IsValidDpiAwarenessContext(value) }
}
#[inline]
pub unsafe fn IsWinEventHookInstalled(event: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsWinEventHookInstalled(event : u32) -> windows_core::BOOL);
    unsafe { IsWinEventHookInstalled(event) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsWindow(hwnd: Option<super::HWND>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsWindow(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { IsWindow(hwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsWindowArranged(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsWindowArranged(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { IsWindowArranged(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsWindowEnabled(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsWindowEnabled(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { IsWindowEnabled(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsWindowUnicode(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsWindowUnicode(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { IsWindowUnicode(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsWindowVisible(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsWindowVisible(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { IsWindowVisible(hwnd) }
}
#[inline]
pub unsafe fn IsWow64Message() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsWow64Message() -> windows_core::BOOL);
    unsafe { IsWow64Message() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn IsZoomed(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsZoomed(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { IsZoomed(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn KillTimer(hwnd: Option<super::HWND>, uidevent: usize) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn KillTimer(hwnd : super::HWND, uidevent : usize) -> windows_core::BOOL);
    unsafe { KillTimer(hwnd.unwrap_or(core::mem::zeroed()) as _, uidevent) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadAcceleratorsA<P1>(hinstance: Option<super::HINSTANCE>, lptablename: P1) -> super::HACCEL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadAcceleratorsA(hinstance : super::HINSTANCE, lptablename : windows_core::PCSTR) -> super::HACCEL);
    unsafe { LoadAcceleratorsA(hinstance.unwrap_or(core::mem::zeroed()) as _, lptablename.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadAcceleratorsW<P1>(hinstance: Option<super::HINSTANCE>, lptablename: P1) -> super::HACCEL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadAcceleratorsW(hinstance : super::HINSTANCE, lptablename : windows_core::PCWSTR) -> super::HACCEL);
    unsafe { LoadAcceleratorsW(hinstance.unwrap_or(core::mem::zeroed()) as _, lptablename.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadBitmapA<P1>(hinstance: Option<super::HINSTANCE>, lpbitmapname: P1) -> super::HBITMAP
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadBitmapA(hinstance : super::HINSTANCE, lpbitmapname : windows_core::PCSTR) -> super::HBITMAP);
    unsafe { LoadBitmapA(hinstance.unwrap_or(core::mem::zeroed()) as _, lpbitmapname.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadBitmapW<P1>(hinstance: Option<super::HINSTANCE>, lpbitmapname: P1) -> super::HBITMAP
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadBitmapW(hinstance : super::HINSTANCE, lpbitmapname : windows_core::PCWSTR) -> super::HBITMAP);
    unsafe { LoadBitmapW(hinstance.unwrap_or(core::mem::zeroed()) as _, lpbitmapname.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadCursorA<P1>(hinstance: Option<super::HINSTANCE>, lpcursorname: P1) -> super::HCURSOR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadCursorA(hinstance : super::HINSTANCE, lpcursorname : windows_core::PCSTR) -> super::HCURSOR);
    unsafe { LoadCursorA(hinstance.unwrap_or(core::mem::zeroed()) as _, lpcursorname.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn LoadCursorFromFileA<P0>(lpfilename: P0) -> super::HCURSOR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadCursorFromFileA(lpfilename : windows_core::PCSTR) -> super::HCURSOR);
    unsafe { LoadCursorFromFileA(lpfilename.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn LoadCursorFromFileW<P0>(lpfilename: P0) -> super::HCURSOR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadCursorFromFileW(lpfilename : windows_core::PCWSTR) -> super::HCURSOR);
    unsafe { LoadCursorFromFileW(lpfilename.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadCursorW<P1>(hinstance: Option<super::HINSTANCE>, lpcursorname: P1) -> super::HCURSOR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadCursorW(hinstance : super::HINSTANCE, lpcursorname : windows_core::PCWSTR) -> super::HCURSOR);
    unsafe { LoadCursorW(hinstance.unwrap_or(core::mem::zeroed()) as _, lpcursorname.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadIconA<P1>(hinstance: Option<super::HINSTANCE>, lpiconname: P1) -> super::HICON
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadIconA(hinstance : super::HINSTANCE, lpiconname : windows_core::PCSTR) -> super::HICON);
    unsafe { LoadIconA(hinstance.unwrap_or(core::mem::zeroed()) as _, lpiconname.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadIconW<P1>(hinstance: Option<super::HINSTANCE>, lpiconname: P1) -> super::HICON
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadIconW(hinstance : super::HINSTANCE, lpiconname : windows_core::PCWSTR) -> super::HICON);
    unsafe { LoadIconW(hinstance.unwrap_or(core::mem::zeroed()) as _, lpiconname.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn LoadImageA<P1>(hinst: Option<super::HINSTANCE>, name: P1, r#type: u32, cx: i32, cy: i32, fuload: u32) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadImageA(hinst : super::HINSTANCE, name : windows_core::PCSTR, r#type : u32, cx : i32, cy : i32, fuload : u32) -> super::HANDLE);
    unsafe { LoadImageA(hinst.unwrap_or(core::mem::zeroed()) as _, name.param().abi(), r#type, cx, cy, fuload) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn LoadImageW<P1>(hinst: Option<super::HINSTANCE>, name: P1, r#type: u32, cx: i32, cy: i32, fuload: u32) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadImageW(hinst : super::HINSTANCE, name : windows_core::PCWSTR, r#type : u32, cx : i32, cy : i32, fuload : u32) -> super::HANDLE);
    unsafe { LoadImageW(hinst.unwrap_or(core::mem::zeroed()) as _, name.param().abi(), r#type, cx, cy, fuload) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LoadKeyboardLayoutA<P0>(pwszklid: P0, flags: u32) -> super::HKL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadKeyboardLayoutA(pwszklid : windows_core::PCSTR, flags : u32) -> super::HKL);
    unsafe { LoadKeyboardLayoutA(pwszklid.param().abi(), flags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LoadKeyboardLayoutW<P0>(pwszklid: P0, flags: u32) -> super::HKL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadKeyboardLayoutW(pwszklid : windows_core::PCWSTR, flags : u32) -> super::HKL);
    unsafe { LoadKeyboardLayoutW(pwszklid.param().abi(), flags) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadMenuA<P1>(hinstance: Option<super::HINSTANCE>, lpmenuname: P1) -> super::HMENU
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadMenuA(hinstance : super::HINSTANCE, lpmenuname : windows_core::PCSTR) -> super::HMENU);
    unsafe { LoadMenuA(hinstance.unwrap_or(core::mem::zeroed()) as _, lpmenuname.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn LoadMenuIndirectA(lpmenutemplate: *const MENUTEMPLATEA) -> super::HMENU {
    windows_core::link!("user32.dll" "system" fn LoadMenuIndirectA(lpmenutemplate : *const MENUTEMPLATEA) -> super::HMENU);
    unsafe { LoadMenuIndirectA(lpmenutemplate) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn LoadMenuIndirectW(lpmenutemplate: *const MENUTEMPLATEW) -> super::HMENU {
    windows_core::link!("user32.dll" "system" fn LoadMenuIndirectW(lpmenutemplate : *const MENUTEMPLATEW) -> super::HMENU);
    unsafe { LoadMenuIndirectW(lpmenutemplate) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn LoadMenuW<P1>(hinstance: Option<super::HINSTANCE>, lpmenuname: P1) -> super::HMENU
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadMenuW(hinstance : super::HINSTANCE, lpmenuname : windows_core::PCWSTR) -> super::HMENU);
    unsafe { LoadMenuW(hinstance.unwrap_or(core::mem::zeroed()) as _, lpmenuname.param().abi()) }
}
#[inline]
pub unsafe fn LockSetForegroundWindow(ulockcode: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn LockSetForegroundWindow(ulockcode : u32) -> windows_core::BOOL);
    unsafe { LockSetForegroundWindow(ulockcode) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn LockWindowUpdate(hwndlock: Option<super::HWND>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn LockWindowUpdate(hwndlock : super::HWND) -> windows_core::BOOL);
    unsafe { LockWindowUpdate(hwndlock.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn LockWorkStation() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn LockWorkStation() -> windows_core::BOOL);
    unsafe { LockWorkStation() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn LogicalToPhysicalPoint(hwnd: super::HWND, lppoint: *mut super::POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn LogicalToPhysicalPoint(hwnd : super::HWND, lppoint : *mut super::POINT) -> windows_core::BOOL);
    unsafe { LogicalToPhysicalPoint(hwnd, lppoint as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn LogicalToPhysicalPointForPerMonitorDPI(hwnd: Option<super::HWND>, lppoint: *mut super::POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn LogicalToPhysicalPointForPerMonitorDPI(hwnd : super::HWND, lppoint : *mut super::POINT) -> windows_core::BOOL);
    unsafe { LogicalToPhysicalPointForPerMonitorDPI(hwnd.unwrap_or(core::mem::zeroed()) as _, lppoint as _) }
}
#[inline]
pub unsafe fn LookupIconIdFromDirectory(presbits: *const u8, ficon: bool) -> i32 {
    windows_core::link!("user32.dll" "system" fn LookupIconIdFromDirectory(presbits : *const u8, ficon : windows_core::BOOL) -> i32);
    unsafe { LookupIconIdFromDirectory(presbits, ficon.into()) }
}
#[inline]
pub unsafe fn LookupIconIdFromDirectoryEx(presbits: *const u8, ficon: bool, cxdesired: i32, cydesired: i32, flags: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn LookupIconIdFromDirectoryEx(presbits : *const u8, ficon : windows_core::BOOL, cxdesired : i32, cydesired : i32, flags : u32) -> i32);
    unsafe { LookupIconIdFromDirectoryEx(presbits, ficon.into(), cxdesired, cydesired, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MapDialogRect(hdlg: super::HWND, lprect: *mut super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn MapDialogRect(hdlg : super::HWND, lprect : *mut super::RECT) -> windows_core::BOOL);
    unsafe { MapDialogRect(hdlg, lprect as _) }
}
#[inline]
pub unsafe fn MapVirtualKeyA(ucode: u32, umaptype: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn MapVirtualKeyA(ucode : u32, umaptype : u32) -> u32);
    unsafe { MapVirtualKeyA(ucode, umaptype) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MapVirtualKeyExA(ucode: u32, umaptype: u32, dwhkl: Option<super::HKL>) -> u32 {
    windows_core::link!("user32.dll" "system" fn MapVirtualKeyExA(ucode : u32, umaptype : u32, dwhkl : super::HKL) -> u32);
    unsafe { MapVirtualKeyExA(ucode, umaptype, dwhkl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MapVirtualKeyExW(ucode: u32, umaptype: u32, dwhkl: Option<super::HKL>) -> u32 {
    windows_core::link!("user32.dll" "system" fn MapVirtualKeyExW(ucode : u32, umaptype : u32, dwhkl : super::HKL) -> u32);
    unsafe { MapVirtualKeyExW(ucode, umaptype, dwhkl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MapVirtualKeyW(ucode: u32, umaptype: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn MapVirtualKeyW(ucode : u32, umaptype : u32) -> u32);
    unsafe { MapVirtualKeyW(ucode, umaptype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MapWindowPoints(hwndfrom: Option<super::HWND>, hwndto: Option<super::HWND>, lppoints: &mut [super::POINT]) -> i32 {
    windows_core::link!("user32.dll" "system" fn MapWindowPoints(hwndfrom : super::HWND, hwndto : super::HWND, lppoints : *mut super::POINT, cpoints : u32) -> i32);
    unsafe { MapWindowPoints(hwndfrom.unwrap_or(core::mem::zeroed()) as _, hwndto.unwrap_or(core::mem::zeroed()) as _, lppoints.as_mut_ptr(), lppoints.len().try_into().unwrap()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MenuItemFromPoint(hwnd: Option<super::HWND>, hmenu: super::HMENU, ptscreen: super::POINT) -> i32 {
    windows_core::link!("user32.dll" "system" fn MenuItemFromPoint(hwnd : super::HWND, hmenu : super::HMENU, ptscreen : super::POINT) -> i32);
    unsafe { MenuItemFromPoint(hwnd.unwrap_or(core::mem::zeroed()) as _, hmenu, ptscreen) }
}
#[inline]
pub unsafe fn MessageBeep(utype: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn MessageBeep(utype : u32) -> windows_core::BOOL);
    unsafe { MessageBeep(utype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MessageBoxA<P1, P2>(hwnd: Option<super::HWND>, lptext: P1, lpcaption: P2, utype: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn MessageBoxA(hwnd : super::HWND, lptext : windows_core::PCSTR, lpcaption : windows_core::PCSTR, utype : u32) -> i32);
    unsafe { MessageBoxA(hwnd.unwrap_or(core::mem::zeroed()) as _, lptext.param().abi(), lpcaption.param().abi(), utype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MessageBoxExA<P1, P2>(hwnd: Option<super::HWND>, lptext: P1, lpcaption: P2, utype: u32, wlanguageid: u16) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn MessageBoxExA(hwnd : super::HWND, lptext : windows_core::PCSTR, lpcaption : windows_core::PCSTR, utype : u32, wlanguageid : u16) -> i32);
    unsafe { MessageBoxExA(hwnd.unwrap_or(core::mem::zeroed()) as _, lptext.param().abi(), lpcaption.param().abi(), utype, wlanguageid) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MessageBoxExW<P1, P2>(hwnd: Option<super::HWND>, lptext: P1, lpcaption: P2, utype: u32, wlanguageid: u16) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn MessageBoxExW(hwnd : super::HWND, lptext : windows_core::PCWSTR, lpcaption : windows_core::PCWSTR, utype : u32, wlanguageid : u16) -> i32);
    unsafe { MessageBoxExW(hwnd.unwrap_or(core::mem::zeroed()) as _, lptext.param().abi(), lpcaption.param().abi(), utype, wlanguageid) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn MessageBoxIndirectA(lpmbp: *const MSGBOXPARAMSA) -> i32 {
    windows_core::link!("user32.dll" "system" fn MessageBoxIndirectA(lpmbp : *const MSGBOXPARAMSA) -> i32);
    unsafe { MessageBoxIndirectA(lpmbp) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn MessageBoxIndirectW(lpmbp: *const MSGBOXPARAMSW) -> i32 {
    windows_core::link!("user32.dll" "system" fn MessageBoxIndirectW(lpmbp : *const MSGBOXPARAMSW) -> i32);
    unsafe { MessageBoxIndirectW(lpmbp) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MessageBoxW<P1, P2>(hwnd: Option<super::HWND>, lptext: P1, lpcaption: P2, utype: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn MessageBoxW(hwnd : super::HWND, lptext : windows_core::PCWSTR, lpcaption : windows_core::PCWSTR, utype : u32) -> i32);
    unsafe { MessageBoxW(hwnd.unwrap_or(core::mem::zeroed()) as _, lptext.param().abi(), lpcaption.param().abi(), utype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ModifyMenuA<P4>(hmnu: super::HMENU, uposition: u32, uflags: u32, uidnewitem: usize, lpnewitem: P4) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn ModifyMenuA(hmnu : super::HMENU, uposition : u32, uflags : u32, uidnewitem : usize, lpnewitem : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ModifyMenuA(hmnu, uposition, uflags, uidnewitem, lpnewitem.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ModifyMenuW<P4>(hmnu: super::HMENU, uposition: u32, uflags: u32, uidnewitem: usize, lpnewitem: P4) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn ModifyMenuW(hmnu : super::HMENU, uposition : u32, uflags : u32, uidnewitem : usize, lpnewitem : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ModifyMenuW(hmnu, uposition, uflags, uidnewitem, lpnewitem.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MonitorFromPoint(pt: super::POINT, dwflags: u32) -> super::HMONITOR {
    windows_core::link!("user32.dll" "system" fn MonitorFromPoint(pt : super::POINT, dwflags : u32) -> super::HMONITOR);
    unsafe { MonitorFromPoint(pt, dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MonitorFromRect(lprc: *const super::RECT, dwflags: u32) -> super::HMONITOR {
    windows_core::link!("user32.dll" "system" fn MonitorFromRect(lprc : *const super::RECT, dwflags : u32) -> super::HMONITOR);
    unsafe { MonitorFromRect(lprc, dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MonitorFromWindow(hwnd: super::HWND, dwflags: u32) -> super::HMONITOR {
    windows_core::link!("user32.dll" "system" fn MonitorFromWindow(hwnd : super::HWND, dwflags : u32) -> super::HMONITOR);
    unsafe { MonitorFromWindow(hwnd, dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MoveWindow(hwnd: super::HWND, x: i32, y: i32, nwidth: i32, nheight: i32, brepaint: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn MoveWindow(hwnd : super::HWND, x : i32, y : i32, nwidth : i32, nheight : i32, brepaint : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { MoveWindow(hwnd, x, y, nwidth, nheight, brepaint.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MsgWaitForMultipleObjects(phandles: Option<&[super::HANDLE]>, fwaitall: bool, dwmilliseconds: u32, dwwakemask: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn MsgWaitForMultipleObjects(ncount : u32, phandles : *const super::HANDLE, fwaitall : windows_core::BOOL, dwmilliseconds : u32, dwwakemask : u32) -> u32);
    unsafe { MsgWaitForMultipleObjects(phandles.map_or(0, |slice| slice.len().try_into().unwrap()), phandles.map_or(core::ptr::null(), |slice| slice.as_ptr()), fwaitall.into(), dwmilliseconds, dwwakemask) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MsgWaitForMultipleObjectsEx(phandles: Option<&[super::HANDLE]>, dwmilliseconds: u32, dwwakemask: u32, dwflags: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn MsgWaitForMultipleObjectsEx(ncount : u32, phandles : *const super::HANDLE, dwmilliseconds : u32, dwwakemask : u32, dwflags : u32) -> u32);
    unsafe { MsgWaitForMultipleObjectsEx(phandles.map_or(0, |slice| slice.len().try_into().unwrap()), phandles.map_or(core::ptr::null(), |slice| slice.as_ptr()), dwmilliseconds, dwwakemask, dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn NotifyWinEvent(event: u32, hwnd: super::HWND, idobject: i32, idchild: i32) {
    windows_core::link!("user32.dll" "system" fn NotifyWinEvent(event : u32, hwnd : super::HWND, idobject : i32, idchild : i32));
    unsafe { NotifyWinEvent(event, hwnd, idobject, idchild) }
}
#[inline]
pub unsafe fn OemKeyScan(woemchar: u16) -> u32 {
    windows_core::link!("user32.dll" "system" fn OemKeyScan(woemchar : u16) -> u32);
    unsafe { OemKeyScan(woemchar) }
}
#[inline]
pub unsafe fn OemToCharA<P0>(psrc: P0, pdst: windows_core::PSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn OemToCharA(psrc : windows_core::PCSTR, pdst : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { OemToCharA(psrc.param().abi(), pdst) }
}
#[inline]
pub unsafe fn OemToCharBuffA<P0>(lpszsrc: P0, lpszdst: windows_core::PSTR, cchdstlength: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn OemToCharBuffA(lpszsrc : windows_core::PCSTR, lpszdst : windows_core::PSTR, cchdstlength : u32) -> windows_core::BOOL);
    unsafe { OemToCharBuffA(lpszsrc.param().abi(), lpszdst, cchdstlength) }
}
#[inline]
pub unsafe fn OemToCharBuffW<P0>(lpszsrc: P0, lpszdst: windows_core::PWSTR, cchdstlength: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn OemToCharBuffW(lpszsrc : windows_core::PCSTR, lpszdst : windows_core::PWSTR, cchdstlength : u32) -> windows_core::BOOL);
    unsafe { OemToCharBuffW(lpszsrc.param().abi(), lpszdst, cchdstlength) }
}
#[inline]
pub unsafe fn OemToCharW<P0>(psrc: P0, pdst: windows_core::PWSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn OemToCharW(psrc : windows_core::PCSTR, pdst : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { OemToCharW(psrc.param().abi(), pdst) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn OffsetRect(lprc: *mut super::RECT, dx: i32, dy: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn OffsetRect(lprc : *mut super::RECT, dx : i32, dy : i32) -> windows_core::BOOL);
    unsafe { OffsetRect(lprc as _, dx, dy) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn OpenClipboard(hwndnewowner: Option<super::HWND>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn OpenClipboard(hwndnewowner : super::HWND) -> windows_core::BOOL);
    unsafe { OpenClipboard(hwndnewowner.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn OpenDesktopA<P0>(lpszdesktop: P0, dwflags: u32, finherit: bool, dwdesiredaccess: super::ACCESS_MASK) -> super::HDESK
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn OpenDesktopA(lpszdesktop : windows_core::PCSTR, dwflags : u32, finherit : windows_core::BOOL, dwdesiredaccess : super::ACCESS_MASK) -> super::HDESK);
    unsafe { OpenDesktopA(lpszdesktop.param().abi(), dwflags, finherit.into(), dwdesiredaccess) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn OpenDesktopW<P0>(lpszdesktop: P0, dwflags: u32, finherit: bool, dwdesiredaccess: super::ACCESS_MASK) -> super::HDESK
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn OpenDesktopW(lpszdesktop : windows_core::PCWSTR, dwflags : u32, finherit : windows_core::BOOL, dwdesiredaccess : super::ACCESS_MASK) -> super::HDESK);
    unsafe { OpenDesktopW(lpszdesktop.param().abi(), dwflags, finherit.into(), dwdesiredaccess) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn OpenIcon(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn OpenIcon(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { OpenIcon(hwnd) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn OpenInputDesktop(dwflags: u32, finherit: bool, dwdesiredaccess: super::ACCESS_MASK) -> super::HDESK {
    windows_core::link!("user32.dll" "system" fn OpenInputDesktop(dwflags : u32, finherit : windows_core::BOOL, dwdesiredaccess : super::ACCESS_MASK) -> super::HDESK);
    unsafe { OpenInputDesktop(dwflags, finherit.into(), dwdesiredaccess) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn OpenWindowStationA<P0>(lpszwinsta: P0, finherit: bool, dwdesiredaccess: super::ACCESS_MASK) -> super::HWINSTA
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn OpenWindowStationA(lpszwinsta : windows_core::PCSTR, finherit : windows_core::BOOL, dwdesiredaccess : super::ACCESS_MASK) -> super::HWINSTA);
    unsafe { OpenWindowStationA(lpszwinsta.param().abi(), finherit.into(), dwdesiredaccess) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn OpenWindowStationW<P0>(lpszwinsta: P0, finherit: bool, dwdesiredaccess: super::ACCESS_MASK) -> super::HWINSTA
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn OpenWindowStationW(lpszwinsta : windows_core::PCWSTR, finherit : windows_core::BOOL, dwdesiredaccess : super::ACCESS_MASK) -> super::HWINSTA);
    unsafe { OpenWindowStationW(lpszwinsta.param().abi(), finherit.into(), dwdesiredaccess) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn PackTouchHitTestingProximityEvaluation(phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn PackTouchHitTestingProximityEvaluation(phittestinginput : *const TOUCH_HIT_TESTING_INPUT, pproximityeval : *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::LRESULT);
    unsafe { PackTouchHitTestingProximityEvaluation(phittestinginput, pproximityeval) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn PaintDesktop(hdc: super::HDC) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PaintDesktop(hdc : super::HDC) -> windows_core::BOOL);
    unsafe { PaintDesktop(hdc) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn PeekMessageA(lpmsg: *mut MSG, hwnd: Option<super::HWND>, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PeekMessageA(lpmsg : *mut MSG, hwnd : super::HWND, wmsgfiltermin : u32, wmsgfiltermax : u32, wremovemsg : u32) -> windows_core::BOOL);
    unsafe { PeekMessageA(lpmsg as _, hwnd.unwrap_or(core::mem::zeroed()) as _, wmsgfiltermin, wmsgfiltermax, wremovemsg) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn PeekMessageW(lpmsg: *mut MSG, hwnd: Option<super::HWND>, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PeekMessageW(lpmsg : *mut MSG, hwnd : super::HWND, wmsgfiltermin : u32, wmsgfiltermax : u32, wremovemsg : u32) -> windows_core::BOOL);
    unsafe { PeekMessageW(lpmsg as _, hwnd.unwrap_or(core::mem::zeroed()) as _, wmsgfiltermin, wmsgfiltermax, wremovemsg) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn PhysicalToLogicalPoint(hwnd: super::HWND, lppoint: *mut super::POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PhysicalToLogicalPoint(hwnd : super::HWND, lppoint : *mut super::POINT) -> windows_core::BOOL);
    unsafe { PhysicalToLogicalPoint(hwnd, lppoint as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn PhysicalToLogicalPointForPerMonitorDPI(hwnd: Option<super::HWND>, lppoint: *mut super::POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PhysicalToLogicalPointForPerMonitorDPI(hwnd : super::HWND, lppoint : *mut super::POINT) -> windows_core::BOOL);
    unsafe { PhysicalToLogicalPointForPerMonitorDPI(hwnd.unwrap_or(core::mem::zeroed()) as _, lppoint as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn PostMessageA(hwnd: Option<super::HWND>, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PostMessageA(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { PostMessageA(hwnd.unwrap_or(core::mem::zeroed()) as _, msg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn PostMessageW(hwnd: Option<super::HWND>, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PostMessageW(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { PostMessageW(hwnd.unwrap_or(core::mem::zeroed()) as _, msg, wparam, lparam) }
}
#[inline]
pub unsafe fn PostQuitMessage(nexitcode: i32) {
    windows_core::link!("user32.dll" "system" fn PostQuitMessage(nexitcode : i32));
    unsafe { PostQuitMessage(nexitcode) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PostThreadMessageA(idthread: u32, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PostThreadMessageA(idthread : u32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { PostThreadMessageA(idthread, msg, wparam, lparam) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PostThreadMessageW(idthread: u32, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PostThreadMessageW(idthread : u32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { PostThreadMessageW(idthread, msg, wparam, lparam) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn PrintWindow(hwnd: super::HWND, hdcblt: super::HDC, nflags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PrintWindow(hwnd : super::HWND, hdcblt : super::HDC, nflags : u32) -> windows_core::BOOL);
    unsafe { PrintWindow(hwnd, hdcblt, nflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn PrivateExtractIconsA<P0>(szfilename: P0, niconindex: i32, cxicon: i32, cyicon: i32, phicon: Option<*mut super::HICON>, piconid: Option<*mut u32>, nicons: u32, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn PrivateExtractIconsA(szfilename : windows_core::PCSTR, niconindex : i32, cxicon : i32, cyicon : i32, phicon : *mut super::HICON, piconid : *mut u32, nicons : u32, flags : u32) -> u32);
    unsafe { PrivateExtractIconsA(szfilename.param().abi(), niconindex, cxicon, cyicon, phicon.unwrap_or(core::mem::zeroed()) as _, piconid.unwrap_or(core::mem::zeroed()) as _, nicons, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn PrivateExtractIconsW<P0>(szfilename: P0, niconindex: i32, cxicon: i32, cyicon: i32, phicon: Option<*mut super::HICON>, piconid: Option<*mut u32>, nicons: u32, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn PrivateExtractIconsW(szfilename : windows_core::PCWSTR, niconindex : i32, cxicon : i32, cyicon : i32, phicon : *mut super::HICON, piconid : *mut u32, nicons : u32, flags : u32) -> u32);
    unsafe { PrivateExtractIconsW(szfilename.param().abi(), niconindex, cxicon, cyicon, phicon.unwrap_or(core::mem::zeroed()) as _, piconid.unwrap_or(core::mem::zeroed()) as _, nicons, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn PtInRect(lprc: *const super::RECT, pt: super::POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PtInRect(lprc : *const super::RECT, pt : super::POINT) -> windows_core::BOOL);
    unsafe { PtInRect(lprc, pt) }
}
#[cfg(all(feature = "windef", feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn QueryDisplayConfig(flags: u32, numpatharrayelements: *mut u32, patharray: *mut super::DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: *mut u32, modeinfoarray: *mut super::DISPLAYCONFIG_MODE_INFO, currenttopologyid: *mut super::DISPLAYCONFIG_TOPOLOGY_ID) -> i32 {
    windows_core::link!("user32.dll" "system" fn QueryDisplayConfig(flags : u32, numpatharrayelements : *mut u32, patharray : *mut super::DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements : *mut u32, modeinfoarray : *mut super::DISPLAYCONFIG_MODE_INFO, currenttopologyid : *mut super::DISPLAYCONFIG_TOPOLOGY_ID) -> i32);
    unsafe { QueryDisplayConfig(flags, numpatharrayelements as _, patharray as _, nummodeinfoarrayelements as _, modeinfoarray as _, currenttopologyid as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RealChildWindowFromPoint(hwndparent: super::HWND, ptparentclientcoords: super::POINT) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn RealChildWindowFromPoint(hwndparent : super::HWND, ptparentclientcoords : super::POINT) -> super::HWND);
    unsafe { RealChildWindowFromPoint(hwndparent, ptparentclientcoords) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RealGetWindowClassA(hwnd: super::HWND, ptszclassname: windows_core::PSTR, cchclassnamemax: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn RealGetWindowClassA(hwnd : super::HWND, ptszclassname : windows_core::PSTR, cchclassnamemax : u32) -> u32);
    unsafe { RealGetWindowClassA(hwnd, ptszclassname, cchclassnamemax) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RealGetWindowClassW(hwnd: super::HWND, ptszclassname: windows_core::PWSTR, cchclassnamemax: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn RealGetWindowClassW(hwnd : super::HWND, ptszclassname : windows_core::PWSTR, cchclassnamemax : u32) -> u32);
    unsafe { RealGetWindowClassW(hwnd, ptszclassname, cchclassnamemax) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn RedrawWindow(hwnd: Option<super::HWND>, lprcupdate: Option<*const super::RECT>, hrgnupdate: Option<super::HRGN>, flags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn RedrawWindow(hwnd : super::HWND, lprcupdate : *const super::RECT, hrgnupdate : super::HRGN, flags : u32) -> windows_core::BOOL);
    unsafe { RedrawWindow(hwnd.unwrap_or(core::mem::zeroed()) as _, lprcupdate.unwrap_or(core::mem::zeroed()) as _, hrgnupdate.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn RegisterClassA(lpwndclass: *const WNDCLASSA) -> super::ATOM {
    windows_core::link!("user32.dll" "system" fn RegisterClassA(lpwndclass : *const WNDCLASSA) -> super::ATOM);
    unsafe { RegisterClassA(lpwndclass) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn RegisterClassExA(param0: *const WNDCLASSEXA) -> super::ATOM {
    windows_core::link!("user32.dll" "system" fn RegisterClassExA(param0 : *const WNDCLASSEXA) -> super::ATOM);
    unsafe { RegisterClassExA(param0) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn RegisterClassExW(param0: *const WNDCLASSEXW) -> super::ATOM {
    windows_core::link!("user32.dll" "system" fn RegisterClassExW(param0 : *const WNDCLASSEXW) -> super::ATOM);
    unsafe { RegisterClassExW(param0) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn RegisterClassW(lpwndclass: *const WNDCLASSW) -> super::ATOM {
    windows_core::link!("user32.dll" "system" fn RegisterClassW(lpwndclass : *const WNDCLASSW) -> super::ATOM);
    unsafe { RegisterClassW(lpwndclass) }
}
#[inline]
pub unsafe fn RegisterClipboardFormatA<P0>(lpszformat: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn RegisterClipboardFormatA(lpszformat : windows_core::PCSTR) -> u32);
    unsafe { RegisterClipboardFormatA(lpszformat.param().abi()) }
}
#[inline]
pub unsafe fn RegisterClipboardFormatW<P0>(lpszformat: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn RegisterClipboardFormatW(lpszformat : windows_core::PCWSTR) -> u32);
    unsafe { RegisterClipboardFormatW(lpszformat.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RegisterCloakedNotification(hwnd: super::HWND, fregister: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn RegisterCloakedNotification(hwnd : super::HWND, fregister : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { RegisterCloakedNotification(hwnd, fregister.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RegisterDeviceNotificationA(hrecipient: super::HANDLE, notificationfilter: *const core::ffi::c_void, flags: u32) -> HDEVNOTIFY {
    windows_core::link!("user32.dll" "system" fn RegisterDeviceNotificationA(hrecipient : super::HANDLE, notificationfilter : *const core::ffi::c_void, flags : u32) -> HDEVNOTIFY);
    unsafe { RegisterDeviceNotificationA(hrecipient, notificationfilter, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RegisterDeviceNotificationW(hrecipient: super::HANDLE, notificationfilter: *const core::ffi::c_void, flags: u32) -> HDEVNOTIFY {
    windows_core::link!("user32.dll" "system" fn RegisterDeviceNotificationW(hrecipient : super::HANDLE, notificationfilter : *const core::ffi::c_void, flags : u32) -> HDEVNOTIFY);
    unsafe { RegisterDeviceNotificationW(hrecipient, notificationfilter, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RegisterForTooltipDismissNotification(hwnd: super::HWND, tdflags: TOOLTIP_DISMISS_FLAGS) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn RegisterForTooltipDismissNotification(hwnd : super::HWND, tdflags : TOOLTIP_DISMISS_FLAGS) -> windows_core::BOOL);
    unsafe { RegisterForTooltipDismissNotification(hwnd, tdflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RegisterHotKey(hwnd: Option<super::HWND>, id: i32, fsmodifiers: u32, vk: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn RegisterHotKey(hwnd : super::HWND, id : i32, fsmodifiers : u32, vk : u32) -> windows_core::BOOL);
    unsafe { RegisterHotKey(hwnd.unwrap_or(core::mem::zeroed()) as _, id, fsmodifiers, vk) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RegisterPointerDeviceNotifications(window: super::HWND, notifyrange: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn RegisterPointerDeviceNotifications(window : super::HWND, notifyrange : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { RegisterPointerDeviceNotifications(window, notifyrange.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RegisterPointerInputTarget(hwnd: super::HWND, pointertype: POINTER_INPUT_TYPE) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn RegisterPointerInputTarget(hwnd : super::HWND, pointertype : POINTER_INPUT_TYPE) -> windows_core::BOOL);
    unsafe { RegisterPointerInputTarget(hwnd, pointertype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RegisterPointerInputTargetEx(hwnd: super::HWND, pointertype: POINTER_INPUT_TYPE, fobserve: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn RegisterPointerInputTargetEx(hwnd : super::HWND, pointertype : POINTER_INPUT_TYPE, fobserve : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { RegisterPointerInputTargetEx(hwnd, pointertype, fobserve.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RegisterPowerSettingNotification(hrecipient: super::HANDLE, powersettingguid: *const windows_core::GUID, flags: u32) -> HPOWERNOTIFY {
    windows_core::link!("user32.dll" "system" fn RegisterPowerSettingNotification(hrecipient : super::HANDLE, powersettingguid : *const windows_core::GUID, flags : u32) -> HPOWERNOTIFY);
    unsafe { RegisterPowerSettingNotification(hrecipient, powersettingguid, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RegisterRawInputDevices(prawinputdevices: &[RAWINPUTDEVICE], cbsize: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn RegisterRawInputDevices(prawinputdevices : *const RAWINPUTDEVICE, uinumdevices : u32, cbsize : u32) -> windows_core::BOOL);
    unsafe { RegisterRawInputDevices(prawinputdevices.as_ptr(), prawinputdevices.len().try_into().unwrap(), cbsize) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RegisterShellHookWindow(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn RegisterShellHookWindow(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { RegisterShellHookWindow(hwnd) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RegisterSuspendResumeNotification(hrecipient: super::HANDLE, flags: u32) -> HPOWERNOTIFY {
    windows_core::link!("user32.dll" "system" fn RegisterSuspendResumeNotification(hrecipient : super::HANDLE, flags : u32) -> HPOWERNOTIFY);
    unsafe { RegisterSuspendResumeNotification(hrecipient, flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RegisterTouchHitTestingWindow(hwnd: super::HWND, value: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn RegisterTouchHitTestingWindow(hwnd : super::HWND, value : u32) -> windows_core::BOOL);
    unsafe { RegisterTouchHitTestingWindow(hwnd, value) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RegisterTouchWindow(hwnd: super::HWND, ulflags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn RegisterTouchWindow(hwnd : super::HWND, ulflags : u32) -> windows_core::BOOL);
    unsafe { RegisterTouchWindow(hwnd, ulflags) }
}
#[inline]
pub unsafe fn RegisterWindowMessageA<P0>(lpstring: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn RegisterWindowMessageA(lpstring : windows_core::PCSTR) -> u32);
    unsafe { RegisterWindowMessageA(lpstring.param().abi()) }
}
#[inline]
pub unsafe fn RegisterWindowMessageW<P0>(lpstring: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn RegisterWindowMessageW(lpstring : windows_core::PCWSTR) -> u32);
    unsafe { RegisterWindowMessageW(lpstring.param().abi()) }
}
#[inline]
pub unsafe fn ReleaseCapture() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ReleaseCapture() -> windows_core::BOOL);
    unsafe { ReleaseCapture() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ReleaseDC(hwnd: Option<super::HWND>, hdc: super::HDC) -> i32 {
    windows_core::link!("user32.dll" "system" fn ReleaseDC(hwnd : super::HWND, hdc : super::HDC) -> i32);
    unsafe { ReleaseDC(hwnd.unwrap_or(core::mem::zeroed()) as _, hdc) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RemoveClipboardFormatListener(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn RemoveClipboardFormatListener(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { RemoveClipboardFormatListener(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RemoveMenu(hmenu: super::HMENU, uposition: u32, uflags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn RemoveMenu(hmenu : super::HMENU, uposition : u32, uflags : u32) -> windows_core::BOOL);
    unsafe { RemoveMenu(hmenu, uposition, uflags) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn RemovePropA<P1>(hwnd: super::HWND, lpstring: P1) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn RemovePropA(hwnd : super::HWND, lpstring : windows_core::PCSTR) -> super::HANDLE);
    unsafe { RemovePropA(hwnd, lpstring.param().abi()) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn RemovePropW<P1>(hwnd: super::HWND, lpstring: P1) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn RemovePropW(hwnd : super::HWND, lpstring : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { RemovePropW(hwnd, lpstring.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ReplyMessage(lresult: super::LRESULT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ReplyMessage(lresult : super::LRESULT) -> windows_core::BOOL);
    unsafe { ReplyMessage(lresult) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScreenToClient(hwnd: super::HWND, lppoint: *mut super::POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ScreenToClient(hwnd : super::HWND, lppoint : *mut super::POINT) -> windows_core::BOOL);
    unsafe { ScreenToClient(hwnd, lppoint as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ScrollDC(hdc: super::HDC, dx: i32, dy: i32, lprcscroll: Option<*const super::RECT>, lprcclip: Option<*const super::RECT>, hrgnupdate: Option<super::HRGN>, lprcupdate: Option<*mut super::RECT>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ScrollDC(hdc : super::HDC, dx : i32, dy : i32, lprcscroll : *const super::RECT, lprcclip : *const super::RECT, hrgnupdate : super::HRGN, lprcupdate : *mut super::RECT) -> windows_core::BOOL);
    unsafe { ScrollDC(hdc, dx, dy, lprcscroll.unwrap_or(core::mem::zeroed()) as _, lprcclip.unwrap_or(core::mem::zeroed()) as _, hrgnupdate.unwrap_or(core::mem::zeroed()) as _, lprcupdate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ScrollWindow(hwnd: super::HWND, xamount: i32, yamount: i32, lprect: Option<*const super::RECT>, lpcliprect: Option<*const super::RECT>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ScrollWindow(hwnd : super::HWND, xamount : i32, yamount : i32, lprect : *const super::RECT, lpcliprect : *const super::RECT) -> windows_core::BOOL);
    unsafe { ScrollWindow(hwnd, xamount, yamount, lprect.unwrap_or(core::mem::zeroed()) as _, lpcliprect.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ScrollWindowEx(hwnd: super::HWND, dx: i32, dy: i32, prcscroll: Option<*const super::RECT>, prcclip: Option<*const super::RECT>, hrgnupdate: Option<super::HRGN>, prcupdate: Option<*mut super::RECT>, flags: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn ScrollWindowEx(hwnd : super::HWND, dx : i32, dy : i32, prcscroll : *const super::RECT, prcclip : *const super::RECT, hrgnupdate : super::HRGN, prcupdate : *mut super::RECT, flags : u32) -> i32);
    unsafe { ScrollWindowEx(hwnd, dx, dy, prcscroll.unwrap_or(core::mem::zeroed()) as _, prcclip.unwrap_or(core::mem::zeroed()) as _, hrgnupdate.unwrap_or(core::mem::zeroed()) as _, prcupdate.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SendDlgItemMessageA(hdlg: super::HWND, niddlgitem: i32, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn SendDlgItemMessageA(hdlg : super::HWND, niddlgitem : i32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { SendDlgItemMessageA(hdlg, niddlgitem, msg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SendDlgItemMessageW(hdlg: super::HWND, niddlgitem: i32, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn SendDlgItemMessageW(hdlg : super::HWND, niddlgitem : i32, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { SendDlgItemMessageW(hdlg, niddlgitem, msg, wparam, lparam) }
}
#[inline]
pub unsafe fn SendInput(pinputs: &[INPUT], cbsize: i32) -> u32 {
    windows_core::link!("user32.dll" "system" fn SendInput(cinputs : u32, pinputs : *const INPUT, cbsize : i32) -> u32);
    unsafe { SendInput(pinputs.len().try_into().unwrap(), pinputs.as_ptr(), cbsize) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SendMessageA(hwnd: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn SendMessageA(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { SendMessageA(hwnd, msg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SendMessageCallbackA(hwnd: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM, lpresultcallback: SENDASYNCPROC, dwdata: usize) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SendMessageCallbackA(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM, lpresultcallback : SENDASYNCPROC, dwdata : usize) -> windows_core::BOOL);
    unsafe { SendMessageCallbackA(hwnd, msg, wparam, lparam, lpresultcallback, dwdata) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SendMessageCallbackW(hwnd: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM, lpresultcallback: SENDASYNCPROC, dwdata: usize) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SendMessageCallbackW(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM, lpresultcallback : SENDASYNCPROC, dwdata : usize) -> windows_core::BOOL);
    unsafe { SendMessageCallbackW(hwnd, msg, wparam, lparam, lpresultcallback, dwdata) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SendMessageTimeoutA(hwnd: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM, fuflags: u32, utimeout: u32, lpdwresult: Option<*mut usize>) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn SendMessageTimeoutA(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM, fuflags : u32, utimeout : u32, lpdwresult : *mut usize) -> super::LRESULT);
    unsafe { SendMessageTimeoutA(hwnd, msg, wparam, lparam, fuflags, utimeout, lpdwresult.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SendMessageTimeoutW(hwnd: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM, fuflags: u32, utimeout: u32, lpdwresult: Option<*mut usize>) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn SendMessageTimeoutW(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM, fuflags : u32, utimeout : u32, lpdwresult : *mut usize) -> super::LRESULT);
    unsafe { SendMessageTimeoutW(hwnd, msg, wparam, lparam, fuflags, utimeout, lpdwresult.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SendMessageW(hwnd: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT {
    windows_core::link!("user32.dll" "system" fn SendMessageW(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> super::LRESULT);
    unsafe { SendMessageW(hwnd, msg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SendNotifyMessageA(hwnd: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SendNotifyMessageA(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { SendNotifyMessageA(hwnd, msg, wparam, lparam) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SendNotifyMessageW(hwnd: super::HWND, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SendNotifyMessageW(hwnd : super::HWND, msg : u32, wparam : super::WPARAM, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { SendNotifyMessageW(hwnd, msg, wparam, lparam) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetActiveWindow(hwnd: super::HWND) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn SetActiveWindow(hwnd : super::HWND) -> super::HWND);
    unsafe { SetActiveWindow(hwnd) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn SetAdditionalForegroundBoostProcesses(toplevelwindow: super::HWND, processhandlearray: &[super::HANDLE]) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetAdditionalForegroundBoostProcesses(toplevelwindow : super::HWND, processhandlecount : u32, processhandlearray : *const super::HANDLE) -> windows_core::BOOL);
    unsafe { SetAdditionalForegroundBoostProcesses(toplevelwindow, processhandlearray.len().try_into().unwrap(), processhandlearray.as_ptr()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetCapture(hwnd: super::HWND) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn SetCapture(hwnd : super::HWND) -> super::HWND);
    unsafe { SetCapture(hwnd) }
}
#[inline]
pub unsafe fn SetCaretBlinkTime(umseconds: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetCaretBlinkTime(umseconds : u32) -> windows_core::BOOL);
    unsafe { SetCaretBlinkTime(umseconds) }
}
#[inline]
pub unsafe fn SetCaretPos(x: i32, y: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetCaretPos(x : i32, y : i32) -> windows_core::BOOL);
    unsafe { SetCaretPos(x, y) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetClassLongA(hwnd: super::HWND, nindex: i32, dwnewlong: i32) -> u32 {
    windows_core::link!("user32.dll" "system" fn SetClassLongA(hwnd : super::HWND, nindex : i32, dwnewlong : i32) -> u32);
    unsafe { SetClassLongA(hwnd, nindex, dwnewlong) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetClassLongPtrA(hwnd: super::HWND, nindex: i32, dwnewlong: isize) -> usize {
    windows_core::link!("user32.dll" "system" fn SetClassLongPtrA(hwnd : super::HWND, nindex : i32, dwnewlong : isize) -> usize);
    unsafe { SetClassLongPtrA(hwnd, nindex, dwnewlong) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetClassLongPtrW(hwnd: super::HWND, nindex: i32, dwnewlong: isize) -> usize {
    windows_core::link!("user32.dll" "system" fn SetClassLongPtrW(hwnd : super::HWND, nindex : i32, dwnewlong : isize) -> usize);
    unsafe { SetClassLongPtrW(hwnd, nindex, dwnewlong) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetClassLongW(hwnd: super::HWND, nindex: i32, dwnewlong: i32) -> u32 {
    windows_core::link!("user32.dll" "system" fn SetClassLongW(hwnd : super::HWND, nindex : i32, dwnewlong : i32) -> u32);
    unsafe { SetClassLongW(hwnd, nindex, dwnewlong) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetClassWord(hwnd: super::HWND, nindex: i32, wnewword: u16) -> u16 {
    windows_core::link!("user32.dll" "system" fn SetClassWord(hwnd : super::HWND, nindex : i32, wnewword : u16) -> u16);
    unsafe { SetClassWord(hwnd, nindex, wnewword) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetClipboardData(uformat: u32, hmem: Option<super::HANDLE>) -> super::HANDLE {
    windows_core::link!("user32.dll" "system" fn SetClipboardData(uformat : u32, hmem : super::HANDLE) -> super::HANDLE);
    unsafe { SetClipboardData(uformat, hmem.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetClipboardViewer(hwndnewviewer: super::HWND) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn SetClipboardViewer(hwndnewviewer : super::HWND) -> super::HWND);
    unsafe { SetClipboardViewer(hwndnewviewer) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetCoalescableTimer(hwnd: Option<super::HWND>, nidevent: usize, uelapse: u32, lptimerfunc: TIMERPROC, utolerancedelay: u32) -> usize {
    windows_core::link!("user32.dll" "system" fn SetCoalescableTimer(hwnd : super::HWND, nidevent : usize, uelapse : u32, lptimerfunc : TIMERPROC, utolerancedelay : u32) -> usize);
    unsafe { SetCoalescableTimer(hwnd.unwrap_or(core::mem::zeroed()) as _, nidevent, uelapse, lptimerfunc, utolerancedelay) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetCursor(hcursor: Option<super::HCURSOR>) -> super::HCURSOR {
    windows_core::link!("user32.dll" "system" fn SetCursor(hcursor : super::HCURSOR) -> super::HCURSOR);
    unsafe { SetCursor(hcursor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetCursorPos(x: i32, y: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetCursorPos(x : i32, y : i32) -> windows_core::BOOL);
    unsafe { SetCursorPos(x, y) }
}
#[inline]
pub unsafe fn SetDebugErrorLevel(dwlevel: u32) {
    windows_core::link!("user32.dll" "system" fn SetDebugErrorLevel(dwlevel : u32));
    unsafe { SetDebugErrorLevel(dwlevel) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetDialogControlDpiChangeBehavior(hwnd: super::HWND, mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetDialogControlDpiChangeBehavior(hwnd : super::HWND, mask : DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values : DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> windows_core::BOOL);
    unsafe { SetDialogControlDpiChangeBehavior(hwnd, mask, values) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetDialogDpiChangeBehavior(hdlg: super::HWND, mask: DIALOG_DPI_CHANGE_BEHAVIORS, values: DIALOG_DPI_CHANGE_BEHAVIORS) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetDialogDpiChangeBehavior(hdlg : super::HWND, mask : DIALOG_DPI_CHANGE_BEHAVIORS, values : DIALOG_DPI_CHANGE_BEHAVIORS) -> windows_core::BOOL);
    unsafe { SetDialogDpiChangeBehavior(hdlg, mask, values) }
}
#[inline]
pub unsafe fn SetDisplayAutoRotationPreferences(orientation: ORIENTATION_PREFERENCE) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetDisplayAutoRotationPreferences(orientation : ORIENTATION_PREFERENCE) -> windows_core::BOOL);
    unsafe { SetDisplayAutoRotationPreferences(orientation) }
}
#[cfg(all(feature = "windef", feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn SetDisplayConfig(patharray: Option<&[super::DISPLAYCONFIG_PATH_INFO]>, modeinfoarray: Option<&[super::DISPLAYCONFIG_MODE_INFO]>, flags: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn SetDisplayConfig(numpatharrayelements : u32, patharray : *const super::DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements : u32, modeinfoarray : *const super::DISPLAYCONFIG_MODE_INFO, flags : u32) -> i32);
    unsafe { SetDisplayConfig(patharray.map_or(0, |slice| slice.len().try_into().unwrap()), patharray.map_or(core::ptr::null(), |slice| slice.as_ptr()), modeinfoarray.map_or(0, |slice| slice.len().try_into().unwrap()), modeinfoarray.map_or(core::ptr::null(), |slice| slice.as_ptr()), flags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetDlgItemInt(hdlg: super::HWND, niddlgitem: i32, uvalue: u32, bsigned: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetDlgItemInt(hdlg : super::HWND, niddlgitem : i32, uvalue : u32, bsigned : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetDlgItemInt(hdlg, niddlgitem, uvalue, bsigned.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetDlgItemTextA<P2>(hdlg: super::HWND, niddlgitem: i32, lpstring: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn SetDlgItemTextA(hdlg : super::HWND, niddlgitem : i32, lpstring : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetDlgItemTextA(hdlg, niddlgitem, lpstring.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetDlgItemTextW<P2>(hdlg: super::HWND, niddlgitem: i32, lpstring: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn SetDlgItemTextW(hdlg : super::HWND, niddlgitem : i32, lpstring : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetDlgItemTextW(hdlg, niddlgitem, lpstring.param().abi()) }
}
#[inline]
pub unsafe fn SetDoubleClickTime(param0: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetDoubleClickTime(param0 : u32) -> windows_core::BOOL);
    unsafe { SetDoubleClickTime(param0) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetFocus(hwnd: Option<super::HWND>) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn SetFocus(hwnd : super::HWND) -> super::HWND);
    unsafe { SetFocus(hwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetForegroundWindow(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetForegroundWindow(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { SetForegroundWindow(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetGestureConfig(hwnd: super::HWND, dwreserved: u32, pgestureconfig: &[GESTURECONFIG], cbsize: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetGestureConfig(hwnd : super::HWND, dwreserved : u32, cids : u32, pgestureconfig : *const GESTURECONFIG, cbsize : u32) -> windows_core::BOOL);
    unsafe { SetGestureConfig(hwnd, dwreserved, pgestureconfig.len().try_into().unwrap(), pgestureconfig.as_ptr(), cbsize) }
}
#[inline]
pub unsafe fn SetKeyboardState(lpkeystate: &[u8; 256]) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetKeyboardState(lpkeystate : *const u8) -> windows_core::BOOL);
    unsafe { SetKeyboardState(lpkeystate.as_ptr()) }
}
#[inline]
pub unsafe fn SetLastErrorEx(dwerrcode: u32, dwtype: u32) {
    windows_core::link!("user32.dll" "system" fn SetLastErrorEx(dwerrcode : u32, dwtype : u32));
    unsafe { SetLastErrorEx(dwerrcode, dwtype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetLayeredWindowAttributes(hwnd: super::HWND, crkey: super::COLORREF, balpha: u8, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetLayeredWindowAttributes(hwnd : super::HWND, crkey : super::COLORREF, balpha : u8, dwflags : u32) -> windows_core::BOOL);
    unsafe { SetLayeredWindowAttributes(hwnd, crkey, balpha, dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetMenu(hwnd: super::HWND, hmenu: Option<super::HMENU>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetMenu(hwnd : super::HWND, hmenu : super::HMENU) -> windows_core::BOOL);
    unsafe { SetMenu(hwnd, hmenu.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetMenuContextHelpId(param0: super::HMENU, param1: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetMenuContextHelpId(param0 : super::HMENU, param1 : u32) -> windows_core::BOOL);
    unsafe { SetMenuContextHelpId(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetMenuDefaultItem(hmenu: super::HMENU, uitem: u32, fbypos: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetMenuDefaultItem(hmenu : super::HMENU, uitem : u32, fbypos : u32) -> windows_core::BOOL);
    unsafe { SetMenuDefaultItem(hmenu, uitem, fbypos) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetMenuInfo(param0: super::HMENU, param1: *const MENUINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetMenuInfo(param0 : super::HMENU, param1 : *const MENUINFO) -> windows_core::BOOL);
    unsafe { SetMenuInfo(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetMenuItemBitmaps(hmenu: super::HMENU, uposition: u32, uflags: u32, hbitmapunchecked: Option<super::HBITMAP>, hbitmapchecked: Option<super::HBITMAP>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetMenuItemBitmaps(hmenu : super::HMENU, uposition : u32, uflags : u32, hbitmapunchecked : super::HBITMAP, hbitmapchecked : super::HBITMAP) -> windows_core::BOOL);
    unsafe { SetMenuItemBitmaps(hmenu, uposition, uflags, hbitmapunchecked.unwrap_or(core::mem::zeroed()) as _, hbitmapchecked.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetMenuItemInfoA(hmenu: super::HMENU, item: u32, fbypositon: bool, lpmii: *const MENUITEMINFOA) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetMenuItemInfoA(hmenu : super::HMENU, item : u32, fbypositon : windows_core::BOOL, lpmii : *const MENUITEMINFOA) -> windows_core::BOOL);
    unsafe { SetMenuItemInfoA(hmenu, item, fbypositon.into(), lpmii) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetMenuItemInfoW(hmenu: super::HMENU, item: u32, fbypositon: bool, lpmii: *const MENUITEMINFOW) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetMenuItemInfoW(hmenu : super::HMENU, item : u32, fbypositon : windows_core::BOOL, lpmii : *const MENUITEMINFOW) -> windows_core::BOOL);
    unsafe { SetMenuItemInfoW(hmenu, item, fbypositon.into(), lpmii) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SetMessageExtraInfo(lparam: super::LPARAM) -> super::LPARAM {
    windows_core::link!("user32.dll" "system" fn SetMessageExtraInfo(lparam : super::LPARAM) -> super::LPARAM);
    unsafe { SetMessageExtraInfo(lparam) }
}
#[inline]
pub unsafe fn SetMessageQueue(cmessagesmax: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetMessageQueue(cmessagesmax : i32) -> windows_core::BOOL);
    unsafe { SetMessageQueue(cmessagesmax) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetParent(hwndchild: super::HWND, hwndnewparent: Option<super::HWND>) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn SetParent(hwndchild : super::HWND, hwndnewparent : super::HWND) -> super::HWND);
    unsafe { SetParent(hwndchild, hwndnewparent.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetPhysicalCursorPos(x: i32, y: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetPhysicalCursorPos(x : i32, y : i32) -> windows_core::BOOL);
    unsafe { SetPhysicalCursorPos(x, y) }
}
#[inline]
pub unsafe fn SetProcessDPIAware() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetProcessDPIAware() -> windows_core::BOOL);
    unsafe { SetProcessDPIAware() }
}
#[inline]
pub unsafe fn SetProcessDefaultLayout(dwdefaultlayout: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetProcessDefaultLayout(dwdefaultlayout : u32) -> windows_core::BOOL);
    unsafe { SetProcessDefaultLayout(dwdefaultlayout) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetProcessDpiAwarenessContext(value: super::DPI_AWARENESS_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetProcessDpiAwarenessContext(value : super::DPI_AWARENESS_CONTEXT) -> windows_core::BOOL);
    unsafe { SetProcessDpiAwarenessContext(value) }
}
#[inline]
pub unsafe fn SetProcessRestrictionExemption(fenableexemption: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetProcessRestrictionExemption(fenableexemption : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetProcessRestrictionExemption(fenableexemption.into()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SetProcessWindowStation(hwinsta: super::HWINSTA) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetProcessWindowStation(hwinsta : super::HWINSTA) -> windows_core::BOOL);
    unsafe { SetProcessWindowStation(hwinsta) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn SetPropA<P1>(hwnd: super::HWND, lpstring: P1, hdata: Option<super::HANDLE>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn SetPropA(hwnd : super::HWND, lpstring : windows_core::PCSTR, hdata : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetPropA(hwnd, lpstring.param().abi(), hdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn SetPropW<P1>(hwnd: super::HWND, lpstring: P1, hdata: Option<super::HANDLE>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn SetPropW(hwnd : super::HWND, lpstring : windows_core::PCWSTR, hdata : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetPropW(hwnd, lpstring.param().abi(), hdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetRect(lprc: *mut super::RECT, xleft: i32, ytop: i32, xright: i32, ybottom: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetRect(lprc : *mut super::RECT, xleft : i32, ytop : i32, xright : i32, ybottom : i32) -> windows_core::BOOL);
    unsafe { SetRect(lprc as _, xleft, ytop, xright, ybottom) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetRectEmpty(lprc: *mut super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetRectEmpty(lprc : *mut super::RECT) -> windows_core::BOOL);
    unsafe { SetRectEmpty(lprc as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetScrollInfo(hwnd: super::HWND, nbar: i32, lpsi: *const SCROLLINFO, redraw: bool) -> i32 {
    windows_core::link!("user32.dll" "system" fn SetScrollInfo(hwnd : super::HWND, nbar : i32, lpsi : *const SCROLLINFO, redraw : windows_core::BOOL) -> i32);
    unsafe { SetScrollInfo(hwnd, nbar, lpsi, redraw.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetScrollPos(hwnd: super::HWND, nbar: i32, npos: i32, bredraw: bool) -> i32 {
    windows_core::link!("user32.dll" "system" fn SetScrollPos(hwnd : super::HWND, nbar : i32, npos : i32, bredraw : windows_core::BOOL) -> i32);
    unsafe { SetScrollPos(hwnd, nbar, npos, bredraw.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetScrollRange(hwnd: super::HWND, nbar: i32, nminpos: i32, nmaxpos: i32, bredraw: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetScrollRange(hwnd : super::HWND, nbar : i32, nminpos : i32, nmaxpos : i32, bredraw : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetScrollRange(hwnd, nbar, nminpos, nmaxpos, bredraw.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetSysColors(celements: i32, lpaelements: *const i32, lpargbvalues: *const super::COLORREF) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetSysColors(celements : i32, lpaelements : *const i32, lpargbvalues : *const super::COLORREF) -> windows_core::BOOL);
    unsafe { SetSysColors(celements, lpaelements, lpargbvalues) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetSystemCursor(hcur: super::HCURSOR, id: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetSystemCursor(hcur : super::HCURSOR, id : u32) -> windows_core::BOOL);
    unsafe { SetSystemCursor(hcur, id) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetThreadDesktop(hdesktop: super::HDESK) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetThreadDesktop(hdesktop : super::HDESK) -> windows_core::BOOL);
    unsafe { SetThreadDesktop(hdesktop) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetThreadDpiAwarenessContext(dpicontext: super::DPI_AWARENESS_CONTEXT) -> super::DPI_AWARENESS_CONTEXT {
    windows_core::link!("user32.dll" "system" fn SetThreadDpiAwarenessContext(dpicontext : super::DPI_AWARENESS_CONTEXT) -> super::DPI_AWARENESS_CONTEXT);
    unsafe { SetThreadDpiAwarenessContext(dpicontext) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetThreadDpiHostingBehavior(value: super::DPI_HOSTING_BEHAVIOR) -> super::DPI_HOSTING_BEHAVIOR {
    windows_core::link!("user32.dll" "system" fn SetThreadDpiHostingBehavior(value : super::DPI_HOSTING_BEHAVIOR) -> super::DPI_HOSTING_BEHAVIOR);
    unsafe { SetThreadDpiHostingBehavior(value) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetTimer(hwnd: Option<super::HWND>, nidevent: usize, uelapse: u32, lptimerfunc: TIMERPROC) -> usize {
    windows_core::link!("user32.dll" "system" fn SetTimer(hwnd : super::HWND, nidevent : usize, uelapse : u32, lptimerfunc : TIMERPROC) -> usize);
    unsafe { SetTimer(hwnd.unwrap_or(core::mem::zeroed()) as _, nidevent, uelapse, lptimerfunc) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetUserObjectInformationA(hobj: super::HANDLE, nindex: i32, pvinfo: *const core::ffi::c_void, nlength: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetUserObjectInformationA(hobj : super::HANDLE, nindex : i32, pvinfo : *const core::ffi::c_void, nlength : u32) -> windows_core::BOOL);
    unsafe { SetUserObjectInformationA(hobj, nindex, pvinfo, nlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetUserObjectInformationW(hobj: super::HANDLE, nindex: i32, pvinfo: *const core::ffi::c_void, nlength: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetUserObjectInformationW(hobj : super::HANDLE, nindex : i32, pvinfo : *const core::ffi::c_void, nlength : u32) -> windows_core::BOOL);
    unsafe { SetUserObjectInformationW(hobj, nindex, pvinfo, nlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetUserObjectSecurity(hobj: super::HANDLE, psirequested: *const u32, psid: super::PSECURITY_DESCRIPTOR) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetUserObjectSecurity(hobj : super::HANDLE, psirequested : *const u32, psid : super::PSECURITY_DESCRIPTOR) -> windows_core::BOOL);
    unsafe { SetUserObjectSecurity(hobj, psirequested, psid) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SetWinEventHook(eventmin: u32, eventmax: u32, hmodwineventproc: Option<super::HMODULE>, pfnwineventproc: WINEVENTPROC, idprocess: u32, idthread: u32, dwflags: u32) -> super::HWINEVENTHOOK {
    windows_core::link!("user32.dll" "system" fn SetWinEventHook(eventmin : u32, eventmax : u32, hmodwineventproc : super::HMODULE, pfnwineventproc : WINEVENTPROC, idprocess : u32, idthread : u32, dwflags : u32) -> super::HWINEVENTHOOK);
    unsafe { SetWinEventHook(eventmin, eventmax, hmodwineventproc.unwrap_or(core::mem::zeroed()) as _, pfnwineventproc, idprocess, idthread, dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetWindowContextHelpId(param0: super::HWND, param1: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetWindowContextHelpId(param0 : super::HWND, param1 : u32) -> windows_core::BOOL);
    unsafe { SetWindowContextHelpId(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetWindowDisplayAffinity(hwnd: super::HWND, dwaffinity: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetWindowDisplayAffinity(hwnd : super::HWND, dwaffinity : u32) -> windows_core::BOOL);
    unsafe { SetWindowDisplayAffinity(hwnd, dwaffinity) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetWindowFeedbackSetting(hwnd: super::HWND, feedback: FEEDBACK_TYPE, dwflags: u32, size: u32, configuration: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetWindowFeedbackSetting(hwnd : super::HWND, feedback : FEEDBACK_TYPE, dwflags : u32, size : u32, configuration : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetWindowFeedbackSetting(hwnd, feedback, dwflags, size, configuration.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetWindowLongA(hwnd: super::HWND, nindex: i32, dwnewlong: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn SetWindowLongA(hwnd : super::HWND, nindex : i32, dwnewlong : i32) -> i32);
    unsafe { SetWindowLongA(hwnd, nindex, dwnewlong) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetWindowLongPtrA(hwnd: super::HWND, nindex: i32, dwnewlong: isize) -> isize {
    windows_core::link!("user32.dll" "system" fn SetWindowLongPtrA(hwnd : super::HWND, nindex : i32, dwnewlong : isize) -> isize);
    unsafe { SetWindowLongPtrA(hwnd, nindex, dwnewlong) }
}
#[cfg(target_pointer_width = "32")]
pub use SetWindowLongA as SetWindowLongPtrA;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetWindowLongPtrW(hwnd: super::HWND, nindex: i32, dwnewlong: isize) -> isize {
    windows_core::link!("user32.dll" "system" fn SetWindowLongPtrW(hwnd : super::HWND, nindex : i32, dwnewlong : isize) -> isize);
    unsafe { SetWindowLongPtrW(hwnd, nindex, dwnewlong) }
}
#[cfg(target_pointer_width = "32")]
pub use SetWindowLongW as SetWindowLongPtrW;
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetWindowLongW(hwnd: super::HWND, nindex: i32, dwnewlong: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn SetWindowLongW(hwnd : super::HWND, nindex : i32, dwnewlong : i32) -> i32);
    unsafe { SetWindowLongW(hwnd, nindex, dwnewlong) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetWindowPlacement(hwnd: super::HWND, lpwndpl: *const WINDOWPLACEMENT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetWindowPlacement(hwnd : super::HWND, lpwndpl : *const WINDOWPLACEMENT) -> windows_core::BOOL);
    unsafe { SetWindowPlacement(hwnd, lpwndpl) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetWindowPos(hwnd: super::HWND, hwndinsertafter: Option<super::HWND>, x: i32, y: i32, cx: i32, cy: i32, uflags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetWindowPos(hwnd : super::HWND, hwndinsertafter : super::HWND, x : i32, y : i32, cx : i32, cy : i32, uflags : u32) -> windows_core::BOOL);
    unsafe { SetWindowPos(hwnd, hwndinsertafter.unwrap_or(core::mem::zeroed()) as _, x, y, cx, cy, uflags) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SetWindowRgn(hwnd: super::HWND, hrgn: Option<super::HRGN>, bredraw: bool) -> i32 {
    windows_core::link!("user32.dll" "system" fn SetWindowRgn(hwnd : super::HWND, hrgn : super::HRGN, bredraw : windows_core::BOOL) -> i32);
    unsafe { SetWindowRgn(hwnd, hrgn.unwrap_or(core::mem::zeroed()) as _, bredraw.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetWindowTextA<P1>(hwnd: super::HWND, lpstring: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn SetWindowTextA(hwnd : super::HWND, lpstring : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetWindowTextA(hwnd, lpstring.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetWindowTextW<P1>(hwnd: super::HWND, lpstring: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn SetWindowTextW(hwnd : super::HWND, lpstring : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetWindowTextW(hwnd, lpstring.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetWindowWord(hwnd: super::HWND, nindex: i32, wnewword: u16) -> u16 {
    windows_core::link!("user32.dll" "system" fn SetWindowWord(hwnd : super::HWND, nindex : i32, wnewword : u16) -> u16);
    unsafe { SetWindowWord(hwnd, nindex, wnewword) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SetWindowsHookA(nfiltertype: i32, pfnfilterproc: HOOKPROC) -> super::HHOOK {
    windows_core::link!("user32.dll" "system" fn SetWindowsHookA(nfiltertype : i32, pfnfilterproc : HOOKPROC) -> super::HHOOK);
    unsafe { SetWindowsHookA(nfiltertype, pfnfilterproc) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SetWindowsHookExA(idhook: i32, lpfn: HOOKPROC, hmod: Option<super::HINSTANCE>, dwthreadid: u32) -> super::HHOOK {
    windows_core::link!("user32.dll" "system" fn SetWindowsHookExA(idhook : i32, lpfn : HOOKPROC, hmod : super::HINSTANCE, dwthreadid : u32) -> super::HHOOK);
    unsafe { SetWindowsHookExA(idhook, lpfn, hmod.unwrap_or(core::mem::zeroed()) as _, dwthreadid) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SetWindowsHookExW(idhook: i32, lpfn: HOOKPROC, hmod: Option<super::HINSTANCE>, dwthreadid: u32) -> super::HHOOK {
    windows_core::link!("user32.dll" "system" fn SetWindowsHookExW(idhook : i32, lpfn : HOOKPROC, hmod : super::HINSTANCE, dwthreadid : u32) -> super::HHOOK);
    unsafe { SetWindowsHookExW(idhook, lpfn, hmod.unwrap_or(core::mem::zeroed()) as _, dwthreadid) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn SetWindowsHookW(nfiltertype: i32, pfnfilterproc: HOOKPROC) -> super::HHOOK {
    windows_core::link!("user32.dll" "system" fn SetWindowsHookW(nfiltertype : i32, pfnfilterproc : HOOKPROC) -> super::HHOOK);
    unsafe { SetWindowsHookW(nfiltertype, pfnfilterproc) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ShowCaret(hwnd: Option<super::HWND>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ShowCaret(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { ShowCaret(hwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ShowCursor(bshow: bool) -> i32 {
    windows_core::link!("user32.dll" "system" fn ShowCursor(bshow : windows_core::BOOL) -> i32);
    unsafe { ShowCursor(bshow.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ShowOwnedPopups(hwnd: super::HWND, fshow: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ShowOwnedPopups(hwnd : super::HWND, fshow : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ShowOwnedPopups(hwnd, fshow.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ShowScrollBar(hwnd: super::HWND, wbar: i32, bshow: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ShowScrollBar(hwnd : super::HWND, wbar : i32, bshow : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ShowScrollBar(hwnd, wbar, bshow.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ShowWindow(hwnd: super::HWND, ncmdshow: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ShowWindow(hwnd : super::HWND, ncmdshow : i32) -> windows_core::BOOL);
    unsafe { ShowWindow(hwnd, ncmdshow) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ShowWindowAsync(hwnd: super::HWND, ncmdshow: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ShowWindowAsync(hwnd : super::HWND, ncmdshow : i32) -> windows_core::BOOL);
    unsafe { ShowWindowAsync(hwnd, ncmdshow) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ShutdownBlockReasonCreate<P1>(hwnd: super::HWND, pwszreason: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn ShutdownBlockReasonCreate(hwnd : super::HWND, pwszreason : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ShutdownBlockReasonCreate(hwnd, pwszreason.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ShutdownBlockReasonDestroy(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ShutdownBlockReasonDestroy(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { ShutdownBlockReasonDestroy(hwnd) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ShutdownBlockReasonQuery(hwnd: super::HWND, pwszbuff: Option<windows_core::PWSTR>, pcchbuff: *mut u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ShutdownBlockReasonQuery(hwnd : super::HWND, pwszbuff : windows_core::PWSTR, pcchbuff : *mut u32) -> windows_core::BOOL);
    unsafe { ShutdownBlockReasonQuery(hwnd, pwszbuff.unwrap_or(core::mem::zeroed()) as _, pcchbuff as _) }
}
#[inline]
pub unsafe fn SkipPointerFrameMessages(pointerid: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SkipPointerFrameMessages(pointerid : u32) -> windows_core::BOOL);
    unsafe { SkipPointerFrameMessages(pointerid) }
}
#[inline]
pub unsafe fn SoundSentry() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SoundSentry() -> windows_core::BOOL);
    unsafe { SoundSentry() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SubtractRect(lprcdst: *mut super::RECT, lprcsrc1: *const super::RECT, lprcsrc2: *const super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SubtractRect(lprcdst : *mut super::RECT, lprcsrc1 : *const super::RECT, lprcsrc2 : *const super::RECT) -> windows_core::BOOL);
    unsafe { SubtractRect(lprcdst as _, lprcsrc1, lprcsrc2) }
}
#[inline]
pub unsafe fn SwapMouseButton(fswap: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SwapMouseButton(fswap : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SwapMouseButton(fswap.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SwitchDesktop(hdesktop: super::HDESK) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SwitchDesktop(hdesktop : super::HDESK) -> windows_core::BOOL);
    unsafe { SwitchDesktop(hdesktop) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SwitchToThisWindow(hwnd: super::HWND, funknown: bool) {
    windows_core::link!("user32.dll" "system" fn SwitchToThisWindow(hwnd : super::HWND, funknown : windows_core::BOOL));
    unsafe { SwitchToThisWindow(hwnd, funknown.into()) }
}
#[inline]
pub unsafe fn SystemParametersInfoA(uiaction: u32, uiparam: u32, pvparam: *mut core::ffi::c_void, fwinini: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SystemParametersInfoA(uiaction : u32, uiparam : u32, pvparam : *mut core::ffi::c_void, fwinini : u32) -> windows_core::BOOL);
    unsafe { SystemParametersInfoA(uiaction, uiparam, pvparam as _, fwinini) }
}
#[inline]
pub unsafe fn SystemParametersInfoForDpi(uiaction: u32, uiparam: u32, pvparam: *mut core::ffi::c_void, fwinini: u32, dpi: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SystemParametersInfoForDpi(uiaction : u32, uiparam : u32, pvparam : *mut core::ffi::c_void, fwinini : u32, dpi : u32) -> windows_core::BOOL);
    unsafe { SystemParametersInfoForDpi(uiaction, uiparam, pvparam as _, fwinini, dpi) }
}
#[inline]
pub unsafe fn SystemParametersInfoW(uiaction: u32, uiparam: u32, pvparam: *mut core::ffi::c_void, fwinini: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SystemParametersInfoW(uiaction : u32, uiparam : u32, pvparam : *mut core::ffi::c_void, fwinini : u32) -> windows_core::BOOL);
    unsafe { SystemParametersInfoW(uiaction, uiparam, pvparam as _, fwinini) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn TabbedTextOutA(hdc: super::HDC, x: i32, y: i32, lpstring: &[u8], lpntabstoppositions: Option<&[i32]>, ntaborigin: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn TabbedTextOutA(hdc : super::HDC, x : i32, y : i32, lpstring : windows_core::PCSTR, chcount : i32, ntabpositions : i32, lpntabstoppositions : *const i32, ntaborigin : i32) -> i32);
    unsafe { TabbedTextOutA(hdc, x, y, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), lpntabstoppositions.map_or(0, |slice| slice.len().try_into().unwrap()), lpntabstoppositions.map_or(core::ptr::null(), |slice| slice.as_ptr()), ntaborigin) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn TabbedTextOutW(hdc: super::HDC, x: i32, y: i32, lpstring: &[u16], lpntabstoppositions: Option<&[i32]>, ntaborigin: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn TabbedTextOutW(hdc : super::HDC, x : i32, y : i32, lpstring : windows_core::PCWSTR, chcount : i32, ntabpositions : i32, lpntabstoppositions : *const i32, ntaborigin : i32) -> i32);
    unsafe { TabbedTextOutW(hdc, x, y, core::mem::transmute(lpstring.as_ptr()), lpstring.len().try_into().unwrap(), lpntabstoppositions.map_or(0, |slice| slice.len().try_into().unwrap()), lpntabstoppositions.map_or(core::ptr::null(), |slice| slice.as_ptr()), ntaborigin) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn TileWindows(hwndparent: Option<super::HWND>, whow: u32, lprect: Option<*const super::RECT>, lpkids: Option<&[super::HWND]>) -> u16 {
    windows_core::link!("user32.dll" "system" fn TileWindows(hwndparent : super::HWND, whow : u32, lprect : *const super::RECT, ckids : u32, lpkids : *const super::HWND) -> u16);
    unsafe { TileWindows(hwndparent.unwrap_or(core::mem::zeroed()) as _, whow, lprect.unwrap_or(core::mem::zeroed()) as _, lpkids.map_or(0, |slice| slice.len().try_into().unwrap()), lpkids.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
}
#[inline]
pub unsafe fn ToAscii(uvirtkey: u32, uscancode: u32, lpkeystate: Option<&[u8; 256]>, lpchar: *mut u16, uflags: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn ToAscii(uvirtkey : u32, uscancode : u32, lpkeystate : *const u8, lpchar : *mut u16, uflags : u32) -> i32);
    unsafe { ToAscii(uvirtkey, uscancode, lpkeystate.map_or(core::ptr::null(), |slice| slice.as_ptr()), lpchar as _, uflags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ToAsciiEx(uvirtkey: u32, uscancode: u32, lpkeystate: Option<&[u8; 256]>, lpchar: *mut u16, uflags: u32, dwhkl: Option<super::HKL>) -> i32 {
    windows_core::link!("user32.dll" "system" fn ToAsciiEx(uvirtkey : u32, uscancode : u32, lpkeystate : *const u8, lpchar : *mut u16, uflags : u32, dwhkl : super::HKL) -> i32);
    unsafe { ToAsciiEx(uvirtkey, uscancode, lpkeystate.map_or(core::ptr::null(), |slice| slice.as_ptr()), lpchar as _, uflags, dwhkl.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ToUnicode(wvirtkey: u32, wscancode: u32, lpkeystate: Option<*const u8>, pwszbuff: windows_core::PWSTR, cchbuff: i32, wflags: u32) -> i32 {
    windows_core::link!("user32.dll" "system" fn ToUnicode(wvirtkey : u32, wscancode : u32, lpkeystate : *const u8, pwszbuff : windows_core::PWSTR, cchbuff : i32, wflags : u32) -> i32);
    unsafe { ToUnicode(wvirtkey, wscancode, lpkeystate.unwrap_or(core::mem::zeroed()) as _, pwszbuff, cchbuff, wflags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ToUnicodeEx(wvirtkey: u32, wscancode: u32, lpkeystate: *const u8, pwszbuff: windows_core::PWSTR, cchbuff: i32, wflags: u32, dwhkl: Option<super::HKL>) -> i32 {
    windows_core::link!("user32.dll" "system" fn ToUnicodeEx(wvirtkey : u32, wscancode : u32, lpkeystate : *const u8, pwszbuff : windows_core::PWSTR, cchbuff : i32, wflags : u32, dwhkl : super::HKL) -> i32);
    unsafe { ToUnicodeEx(wvirtkey, wscancode, lpkeystate, pwszbuff, cchbuff, wflags, dwhkl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn TrackMouseEvent(lpeventtrack: *mut TRACKMOUSEEVENT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn TrackMouseEvent(lpeventtrack : *mut TRACKMOUSEEVENT) -> windows_core::BOOL);
    unsafe { TrackMouseEvent(lpeventtrack as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn TrackPopupMenu(hmenu: super::HMENU, uflags: u32, x: i32, y: i32, nreserved: Option<i32>, hwnd: super::HWND, prcrect: Option<*const super::RECT>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn TrackPopupMenu(hmenu : super::HMENU, uflags : u32, x : i32, y : i32, nreserved : i32, hwnd : super::HWND, prcrect : *const super::RECT) -> windows_core::BOOL);
    unsafe { TrackPopupMenu(hmenu, uflags, x, y, nreserved.unwrap_or(core::mem::zeroed()) as _, hwnd, prcrect.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn TrackPopupMenuEx(hmenu: super::HMENU, uflags: u32, x: i32, y: i32, hwnd: super::HWND, lptpm: Option<*const TPMPARAMS>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn TrackPopupMenuEx(hmenu : super::HMENU, uflags : u32, x : i32, y : i32, hwnd : super::HWND, lptpm : *const TPMPARAMS) -> windows_core::BOOL);
    unsafe { TrackPopupMenuEx(hmenu, uflags, x, y, hwnd, lptpm.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn TranslateAcceleratorA(hwnd: super::HWND, hacctable: super::HACCEL, lpmsg: *const MSG) -> i32 {
    windows_core::link!("user32.dll" "system" fn TranslateAcceleratorA(hwnd : super::HWND, hacctable : super::HACCEL, lpmsg : *const MSG) -> i32);
    unsafe { TranslateAcceleratorA(hwnd, hacctable, lpmsg) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn TranslateAcceleratorW(hwnd: super::HWND, hacctable: super::HACCEL, lpmsg: *const MSG) -> i32 {
    windows_core::link!("user32.dll" "system" fn TranslateAcceleratorW(hwnd : super::HWND, hacctable : super::HACCEL, lpmsg : *const MSG) -> i32);
    unsafe { TranslateAcceleratorW(hwnd, hacctable, lpmsg) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn TranslateMDISysAccel(hwndclient: super::HWND, lpmsg: *const MSG) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn TranslateMDISysAccel(hwndclient : super::HWND, lpmsg : *const MSG) -> windows_core::BOOL);
    unsafe { TranslateMDISysAccel(hwndclient, lpmsg) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn TranslateMessage(lpmsg: *const MSG) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn TranslateMessage(lpmsg : *const MSG) -> windows_core::BOOL);
    unsafe { TranslateMessage(lpmsg) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UnhookWinEvent(hwineventhook: super::HWINEVENTHOOK) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnhookWinEvent(hwineventhook : super::HWINEVENTHOOK) -> windows_core::BOOL);
    unsafe { UnhookWinEvent(hwineventhook) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn UnhookWindowsHook(ncode: i32, pfnfilterproc: HOOKPROC) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnhookWindowsHook(ncode : i32, pfnfilterproc : HOOKPROC) -> windows_core::BOOL);
    unsafe { UnhookWindowsHook(ncode, pfnfilterproc) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UnhookWindowsHookEx(hhk: super::HHOOK) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnhookWindowsHookEx(hhk : super::HHOOK) -> windows_core::BOOL);
    unsafe { UnhookWindowsHookEx(hhk) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UnionRect(lprcdst: *mut super::RECT, lprcsrc1: *const super::RECT, lprcsrc2: *const super::RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnionRect(lprcdst : *mut super::RECT, lprcsrc1 : *const super::RECT, lprcsrc2 : *const super::RECT) -> windows_core::BOOL);
    unsafe { UnionRect(lprcdst as _, lprcsrc1, lprcsrc2) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn UnloadKeyboardLayout(hkl: super::HKL) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnloadKeyboardLayout(hkl : super::HKL) -> windows_core::BOOL);
    unsafe { UnloadKeyboardLayout(hkl) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn UnregisterClassA<P0>(lpclassname: P0, hinstance: Option<super::HINSTANCE>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn UnregisterClassA(lpclassname : windows_core::PCSTR, hinstance : super::HINSTANCE) -> windows_core::BOOL);
    unsafe { UnregisterClassA(lpclassname.param().abi(), hinstance.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn UnregisterClassW<P0>(lpclassname: P0, hinstance: Option<super::HINSTANCE>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn UnregisterClassW(lpclassname : windows_core::PCWSTR, hinstance : super::HINSTANCE) -> windows_core::BOOL);
    unsafe { UnregisterClassW(lpclassname.param().abi(), hinstance.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn UnregisterDeviceNotification(handle: HDEVNOTIFY) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnregisterDeviceNotification(handle : HDEVNOTIFY) -> windows_core::BOOL);
    unsafe { UnregisterDeviceNotification(handle) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UnregisterHotKey(hwnd: Option<super::HWND>, id: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnregisterHotKey(hwnd : super::HWND, id : i32) -> windows_core::BOOL);
    unsafe { UnregisterHotKey(hwnd.unwrap_or(core::mem::zeroed()) as _, id) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UnregisterPointerInputTarget(hwnd: super::HWND, pointertype: POINTER_INPUT_TYPE) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnregisterPointerInputTarget(hwnd : super::HWND, pointertype : POINTER_INPUT_TYPE) -> windows_core::BOOL);
    unsafe { UnregisterPointerInputTarget(hwnd, pointertype) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UnregisterPointerInputTargetEx(hwnd: super::HWND, pointertype: POINTER_INPUT_TYPE) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnregisterPointerInputTargetEx(hwnd : super::HWND, pointertype : POINTER_INPUT_TYPE) -> windows_core::BOOL);
    unsafe { UnregisterPointerInputTargetEx(hwnd, pointertype) }
}
#[inline]
pub unsafe fn UnregisterPowerSettingNotification(handle: HPOWERNOTIFY) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnregisterPowerSettingNotification(handle : HPOWERNOTIFY) -> windows_core::BOOL);
    unsafe { UnregisterPowerSettingNotification(handle) }
}
#[inline]
pub unsafe fn UnregisterSuspendResumeNotification(handle: HPOWERNOTIFY) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnregisterSuspendResumeNotification(handle : HPOWERNOTIFY) -> windows_core::BOOL);
    unsafe { UnregisterSuspendResumeNotification(handle) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UnregisterTouchWindow(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnregisterTouchWindow(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { UnregisterTouchWindow(hwnd) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn UpdateLayeredWindow(hwnd: super::HWND, hdcdst: Option<super::HDC>, pptdst: Option<*const super::POINT>, psize: Option<*const super::SIZE>, hdcsrc: Option<super::HDC>, pptsrc: Option<*const super::POINT>, crkey: super::COLORREF, pblend: Option<*const super::BLENDFUNCTION>, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UpdateLayeredWindow(hwnd : super::HWND, hdcdst : super::HDC, pptdst : *const super::POINT, psize : *const super::SIZE, hdcsrc : super::HDC, pptsrc : *const super::POINT, crkey : super::COLORREF, pblend : *const super::BLENDFUNCTION, dwflags : u32) -> windows_core::BOOL);
    unsafe { UpdateLayeredWindow(hwnd, hdcdst.unwrap_or(core::mem::zeroed()) as _, pptdst.unwrap_or(core::mem::zeroed()) as _, psize.unwrap_or(core::mem::zeroed()) as _, hdcsrc.unwrap_or(core::mem::zeroed()) as _, pptsrc.unwrap_or(core::mem::zeroed()) as _, crkey, pblend.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn UpdateLayeredWindowIndirect(hwnd: super::HWND, pulwinfo: *const UPDATELAYEREDWINDOWINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UpdateLayeredWindowIndirect(hwnd : super::HWND, pulwinfo : *const UPDATELAYEREDWINDOWINFO) -> windows_core::BOOL);
    unsafe { UpdateLayeredWindowIndirect(hwnd, pulwinfo) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UpdateWindow(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UpdateWindow(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { UpdateWindow(hwnd) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UserHandleGrantAccess(huserhandle: super::HANDLE, hjob: super::HANDLE, bgrant: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UserHandleGrantAccess(huserhandle : super::HANDLE, hjob : super::HANDLE, bgrant : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { UserHandleGrantAccess(huserhandle, hjob, bgrant.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ValidateRect(hwnd: Option<super::HWND>, lprect: Option<*const super::RECT>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ValidateRect(hwnd : super::HWND, lprect : *const super::RECT) -> windows_core::BOOL);
    unsafe { ValidateRect(hwnd.unwrap_or(core::mem::zeroed()) as _, lprect.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ValidateRgn(hwnd: super::HWND, hrgn: Option<super::HRGN>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ValidateRgn(hwnd : super::HWND, hrgn : super::HRGN) -> windows_core::BOOL);
    unsafe { ValidateRgn(hwnd, hrgn.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn VkKeyScanA(ch: i8) -> i16 {
    windows_core::link!("user32.dll" "system" fn VkKeyScanA(ch : i8) -> i16);
    unsafe { VkKeyScanA(ch) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn VkKeyScanExA(ch: i8, dwhkl: super::HKL) -> i16 {
    windows_core::link!("user32.dll" "system" fn VkKeyScanExA(ch : i8, dwhkl : super::HKL) -> i16);
    unsafe { VkKeyScanExA(ch, dwhkl) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn VkKeyScanExW(ch: u16, dwhkl: super::HKL) -> i16 {
    windows_core::link!("user32.dll" "system" fn VkKeyScanExW(ch : u16, dwhkl : super::HKL) -> i16);
    unsafe { VkKeyScanExW(ch, dwhkl) }
}
#[inline]
pub unsafe fn VkKeyScanW(ch: u16) -> i16 {
    windows_core::link!("user32.dll" "system" fn VkKeyScanW(ch : u16) -> i16);
    unsafe { VkKeyScanW(ch) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WaitForInputIdle(hprocess: super::HANDLE, dwmilliseconds: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn WaitForInputIdle(hprocess : super::HANDLE, dwmilliseconds : u32) -> u32);
    unsafe { WaitForInputIdle(hprocess, dwmilliseconds) }
}
#[inline]
pub unsafe fn WaitMessage() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn WaitMessage() -> windows_core::BOOL);
    unsafe { WaitMessage() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WinHelpA<P1>(hwndmain: Option<super::HWND>, lpszhelp: P1, ucommand: u32, dwdata: usize) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn WinHelpA(hwndmain : super::HWND, lpszhelp : windows_core::PCSTR, ucommand : u32, dwdata : usize) -> windows_core::BOOL);
    unsafe { WinHelpA(hwndmain.unwrap_or(core::mem::zeroed()) as _, lpszhelp.param().abi(), ucommand, dwdata) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WinHelpW<P1>(hwndmain: Option<super::HWND>, lpszhelp: P1, ucommand: u32, dwdata: usize) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn WinHelpW(hwndmain : super::HWND, lpszhelp : windows_core::PCWSTR, ucommand : u32, dwdata : usize) -> windows_core::BOOL);
    unsafe { WinHelpW(hwndmain.unwrap_or(core::mem::zeroed()) as _, lpszhelp.param().abi(), ucommand, dwdata) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WindowFromDC(hdc: super::HDC) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn WindowFromDC(hdc : super::HDC) -> super::HWND);
    unsafe { WindowFromDC(hdc) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WindowFromPhysicalPoint(point: super::POINT) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn WindowFromPhysicalPoint(point : super::POINT) -> super::HWND);
    unsafe { WindowFromPhysicalPoint(point) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WindowFromPoint(point: super::POINT) -> super::HWND {
    windows_core::link!("user32.dll" "system" fn WindowFromPoint(point : super::POINT) -> super::HWND);
    unsafe { WindowFromPoint(point) }
}
#[inline]
pub unsafe fn keybd_event(bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: usize) {
    windows_core::link!("user32.dll" "system" fn keybd_event(bvk : u8, bscan : u8, dwflags : u32, dwextrainfo : usize));
    unsafe { keybd_event(bvk, bscan, dwflags, dwextrainfo) }
}
#[inline]
pub unsafe fn mouse_event(dwflags: u32, dx: u32, dy: u32, dwdata: u32, dwextrainfo: usize) {
    windows_core::link!("user32.dll" "system" fn mouse_event(dwflags : u32, dx : u32, dy : u32, dwdata : u32, dwextrainfo : usize));
    unsafe { mouse_event(dwflags, dx, dy, dwdata, dwextrainfo) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn wvsprintfA<P1>(param0: windows_core::PSTR, param1: P1, arglist: *const i8) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn wvsprintfA(param0 : windows_core::PSTR, param1 : windows_core::PCSTR, arglist : *const i8) -> i32);
    unsafe { wvsprintfA(param0, param1.param().abi(), arglist) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
#[inline]
pub unsafe fn wvsprintfA<P1>(param0: windows_core::PSTR, param1: P1, arglist: super::va_list) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("user32.dll" "system" fn wvsprintfA(param0 : windows_core::PSTR, param1 : windows_core::PCSTR, arglist : super::va_list) -> i32);
    unsafe { wvsprintfA(param0, param1.param().abi(), arglist) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn wvsprintfW<P1>(param0: windows_core::PWSTR, param1: P1, arglist: *const i8) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn wvsprintfW(param0 : windows_core::PWSTR, param1 : windows_core::PCWSTR, arglist : *const i8) -> i32);
    unsafe { wvsprintfW(param0, param1.param().abi(), arglist) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "vadefs")]
#[inline]
pub unsafe fn wvsprintfW<P1>(param0: windows_core::PWSTR, param1: P1, arglist: super::va_list) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn wvsprintfW(param0 : windows_core::PWSTR, param1 : windows_core::PCWSTR, arglist : super::va_list) -> i32);
    unsafe { wvsprintfW(param0, param1.param().abi(), arglist) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACCEL {
    pub fVirt: u8,
    pub key: u16,
    pub cmd: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACCESSTIMEOUT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iTimeOutMSec: u32,
}
pub const ALERT_SYSTEM_CRITICAL: i32 = 5;
pub const ALERT_SYSTEM_ERROR: i32 = 3;
pub const ALERT_SYSTEM_INFORMATIONAL: i32 = 1;
pub const ALERT_SYSTEM_QUERY: i32 = 4;
pub const ALERT_SYSTEM_WARNING: i32 = 2;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ANIMATIONINFO {
    pub cbSize: u32,
    pub iMinAnimate: i32,
}
pub const APPCOMMAND_BASS_BOOST: i32 = 20;
pub const APPCOMMAND_BASS_DOWN: i32 = 19;
pub const APPCOMMAND_BASS_UP: i32 = 21;
pub const APPCOMMAND_BROWSER_BACKWARD: i32 = 1;
pub const APPCOMMAND_BROWSER_FAVORITES: i32 = 6;
pub const APPCOMMAND_BROWSER_FORWARD: i32 = 2;
pub const APPCOMMAND_BROWSER_HOME: i32 = 7;
pub const APPCOMMAND_BROWSER_REFRESH: i32 = 3;
pub const APPCOMMAND_BROWSER_SEARCH: i32 = 5;
pub const APPCOMMAND_BROWSER_STOP: i32 = 4;
pub const APPCOMMAND_CLOSE: i32 = 31;
pub const APPCOMMAND_COPY: i32 = 36;
pub const APPCOMMAND_CORRECTION_LIST: i32 = 45;
pub const APPCOMMAND_CUT: i32 = 37;
pub const APPCOMMAND_DELETE: i32 = 53;
pub const APPCOMMAND_DICTATE_OR_COMMAND_CONTROL_TOGGLE: i32 = 43;
pub const APPCOMMAND_DWM_FLIP3D: i32 = 54;
pub const APPCOMMAND_FIND: i32 = 28;
pub const APPCOMMAND_FORWARD_MAIL: i32 = 40;
pub const APPCOMMAND_HELP: i32 = 27;
pub const APPCOMMAND_LAUNCH_APP1: i32 = 17;
pub const APPCOMMAND_LAUNCH_APP2: i32 = 18;
pub const APPCOMMAND_LAUNCH_MAIL: i32 = 15;
pub const APPCOMMAND_LAUNCH_MEDIA_SELECT: i32 = 16;
pub const APPCOMMAND_MEDIA_CHANNEL_DOWN: i32 = 52;
pub const APPCOMMAND_MEDIA_CHANNEL_UP: i32 = 51;
pub const APPCOMMAND_MEDIA_FAST_FORWARD: i32 = 49;
pub const APPCOMMAND_MEDIA_NEXTTRACK: i32 = 11;
pub const APPCOMMAND_MEDIA_PAUSE: i32 = 47;
pub const APPCOMMAND_MEDIA_PLAY: i32 = 46;
pub const APPCOMMAND_MEDIA_PLAY_PAUSE: i32 = 14;
pub const APPCOMMAND_MEDIA_PREVIOUSTRACK: i32 = 12;
pub const APPCOMMAND_MEDIA_RECORD: i32 = 48;
pub const APPCOMMAND_MEDIA_REWIND: i32 = 50;
pub const APPCOMMAND_MEDIA_STOP: i32 = 13;
pub const APPCOMMAND_MICROPHONE_VOLUME_DOWN: i32 = 25;
pub const APPCOMMAND_MICROPHONE_VOLUME_MUTE: i32 = 24;
pub const APPCOMMAND_MICROPHONE_VOLUME_UP: i32 = 26;
pub const APPCOMMAND_MIC_ON_OFF_TOGGLE: i32 = 44;
pub const APPCOMMAND_NEW: i32 = 29;
pub const APPCOMMAND_OPEN: i32 = 30;
pub const APPCOMMAND_PASTE: i32 = 38;
pub const APPCOMMAND_PRINT: i32 = 33;
pub const APPCOMMAND_REDO: i32 = 35;
pub const APPCOMMAND_REPLY_TO_MAIL: i32 = 39;
pub const APPCOMMAND_SAVE: i32 = 32;
pub const APPCOMMAND_SEND_MAIL: i32 = 41;
pub const APPCOMMAND_SPELL_CHECK: i32 = 42;
pub const APPCOMMAND_TREBLE_DOWN: i32 = 22;
pub const APPCOMMAND_TREBLE_UP: i32 = 23;
pub const APPCOMMAND_UNDO: i32 = 34;
pub const APPCOMMAND_VOLUME_DOWN: i32 = 9;
pub const APPCOMMAND_VOLUME_MUTE: i32 = 8;
pub const APPCOMMAND_VOLUME_UP: i32 = 10;
pub const ARW_BOTTOMLEFT: i32 = 0;
pub const ARW_BOTTOMRIGHT: i32 = 1;
pub const ARW_DOWN: i32 = 4;
pub const ARW_HIDE: i32 = 8;
pub const ARW_LEFT: i32 = 0;
pub const ARW_RIGHT: i32 = 0;
pub const ARW_STARTMASK: i32 = 3;
pub const ARW_STARTRIGHT: i32 = 1;
pub const ARW_STARTTOP: i32 = 2;
pub const ARW_TOPLEFT: i32 = 2;
pub const ARW_TOPRIGHT: i32 = 3;
pub const ARW_UP: i32 = 4;
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
pub const ATF_ONOFFFEEDBACK: i32 = 2;
pub const ATF_TIMEOUTON: i32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDIODESCRIPTION {
    pub cbSize: u32,
    pub Enabled: windows_core::BOOL,
    pub Locale: super::LCID,
}
pub const AW_ACTIVATE: i32 = 131072;
pub const AW_BLEND: i32 = 524288;
pub const AW_CENTER: i32 = 16;
pub const AW_HIDE: i32 = 65536;
pub const AW_HOR_NEGATIVE: i32 = 2;
pub const AW_HOR_POSITIVE: i32 = 1;
pub const AW_SLIDE: i32 = 262144;
pub const AW_VER_NEGATIVE: i32 = 8;
pub const AW_VER_POSITIVE: i32 = 4;
pub const BDR_INNER: i32 = 12;
pub const BDR_OUTER: i32 = 3;
pub const BDR_RAISED: i32 = 5;
pub const BDR_RAISEDINNER: i32 = 4;
pub const BDR_RAISEDOUTER: i32 = 1;
pub const BDR_SUNKEN: i32 = 10;
pub const BDR_SUNKENINNER: i32 = 8;
pub const BDR_SUNKENOUTER: i32 = 2;
pub const BF_ADJUST: i32 = 8192;
pub const BF_BOTTOM: i32 = 8;
pub const BF_BOTTOMLEFT: i32 = 9;
pub const BF_BOTTOMRIGHT: i32 = 12;
pub const BF_DIAGONAL: i32 = 16;
pub const BF_DIAGONAL_ENDBOTTOMLEFT: i32 = 25;
pub const BF_DIAGONAL_ENDBOTTOMRIGHT: i32 = 28;
pub const BF_DIAGONAL_ENDTOPLEFT: i32 = 19;
pub const BF_DIAGONAL_ENDTOPRIGHT: i32 = 22;
pub const BF_FLAT: i32 = 16384;
pub const BF_LEFT: i32 = 1;
pub const BF_MIDDLE: i32 = 2048;
pub const BF_MONO: i32 = 32768;
pub const BF_RECT: i32 = 15;
pub const BF_RIGHT: i32 = 4;
pub const BF_SOFT: i32 = 4096;
pub const BF_TOP: i32 = 2;
pub const BF_TOPLEFT: i32 = 3;
pub const BF_TOPRIGHT: i32 = 6;
pub const BM_CLICK: i32 = 245;
pub const BM_GETCHECK: i32 = 240;
pub const BM_GETIMAGE: i32 = 246;
pub const BM_GETSTATE: i32 = 242;
pub const BM_SETCHECK: i32 = 241;
pub const BM_SETDONTCLICK: i32 = 248;
pub const BM_SETIMAGE: i32 = 247;
pub const BM_SETSTATE: i32 = 243;
pub const BM_SETSTYLE: i32 = 244;
pub const BN_CLICKED: i32 = 0;
pub const BN_DBLCLK: i32 = 5;
pub const BN_DISABLE: i32 = 4;
pub const BN_DOUBLECLICKED: i32 = 5;
pub const BN_HILITE: i32 = 2;
pub const BN_KILLFOCUS: i32 = 7;
pub const BN_PAINT: i32 = 1;
pub const BN_PUSHED: i32 = 2;
pub const BN_SETFOCUS: i32 = 6;
pub const BN_UNHILITE: i32 = 3;
pub const BN_UNPUSHED: i32 = 3;
pub const BROADCAST_QUERY_DENY: i32 = 1112363332;
pub const BSF_ALLOWSFW: i32 = 128;
pub const BSF_FLUSHDISK: i32 = 4;
pub const BSF_FORCEIFHUNG: i32 = 32;
pub const BSF_IGNORECURRENTTASK: i32 = 2;
pub const BSF_LUID: i32 = 1024;
pub const BSF_NOHANG: i32 = 8;
pub const BSF_NOTIMEOUTIFNOTHUNG: i32 = 64;
pub const BSF_POSTMESSAGE: i32 = 16;
pub const BSF_QUERY: i32 = 1;
pub const BSF_RETURNHDESK: i32 = 512;
pub const BSF_SENDNOTIFYMESSAGE: i32 = 256;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BSMINFO {
    pub cbSize: u32,
    pub hdesk: super::HDESK,
    pub hwnd: super::HWND,
    pub luid: super::LUID,
}
pub const BSM_ALLCOMPONENTS: i32 = 0;
pub const BSM_ALLDESKTOPS: i32 = 16;
pub const BSM_APPLICATIONS: i32 = 8;
pub const BSM_INSTALLABLEDRIVERS: i32 = 4;
pub const BSM_NETDRIVER: i32 = 2;
pub const BSM_VXDS: i32 = 1;
pub const BST_CHECKED: i32 = 1;
pub const BST_FOCUS: i32 = 8;
pub const BST_INDETERMINATE: i32 = 2;
pub const BST_PUSHED: i32 = 4;
pub const BST_UNCHECKED: i32 = 0;
pub const BS_3STATE: i32 = 5;
pub const BS_AUTO3STATE: i32 = 6;
pub const BS_AUTOCHECKBOX: i32 = 3;
pub const BS_AUTORADIOBUTTON: i32 = 9;
pub const BS_BITMAP: i32 = 128;
pub const BS_BOTTOM: i32 = 2048;
pub const BS_CENTER: i32 = 768;
pub const BS_CHECKBOX: i32 = 2;
pub const BS_DEFPUSHBUTTON: i32 = 1;
pub const BS_FLAT: i32 = 32768;
pub const BS_GROUPBOX: i32 = 7;
pub const BS_ICON: i32 = 64;
pub const BS_LEFT: i32 = 256;
pub const BS_LEFTTEXT: i32 = 32;
pub const BS_MULTILINE: i32 = 8192;
pub const BS_NOTIFY: i32 = 16384;
pub const BS_OWNERDRAW: i32 = 11;
pub const BS_PUSHBOX: i32 = 10;
pub const BS_PUSHBUTTON: i32 = 0;
pub const BS_PUSHLIKE: i32 = 4096;
pub const BS_RADIOBUTTON: i32 = 4;
pub const BS_RIGHT: i32 = 512;
pub const BS_RIGHTBUTTON: i32 = 32;
pub const BS_TEXT: i32 = 0;
pub const BS_TOP: i32 = 1024;
pub const BS_TYPEMASK: i32 = 15;
pub const BS_USERBUTTON: i32 = 8;
pub const BS_VCENTER: i32 = 3072;
pub const CALERT_SYSTEM: i32 = 6;
pub const CBN_CLOSEUP: i32 = 8;
pub const CBN_DBLCLK: i32 = 2;
pub const CBN_DROPDOWN: i32 = 7;
pub const CBN_EDITCHANGE: i32 = 5;
pub const CBN_EDITUPDATE: i32 = 6;
pub const CBN_ERRSPACE: i32 = -1;
pub const CBN_KILLFOCUS: i32 = 4;
pub const CBN_SELCHANGE: i32 = 1;
pub const CBN_SELENDCANCEL: i32 = 10;
pub const CBN_SELENDOK: i32 = 9;
pub const CBN_SETFOCUS: i32 = 3;
pub const CBS_AUTOHSCROLL: i32 = 64;
pub const CBS_DISABLENOSCROLL: i32 = 2048;
pub const CBS_DROPDOWN: i32 = 2;
pub const CBS_DROPDOWNLIST: i32 = 3;
pub const CBS_HASSTRINGS: i32 = 512;
pub const CBS_LOWERCASE: i32 = 16384;
pub const CBS_NOINTEGRALHEIGHT: i32 = 1024;
pub const CBS_OEMCONVERT: i32 = 128;
pub const CBS_OWNERDRAWFIXED: i32 = 16;
pub const CBS_OWNERDRAWVARIABLE: i32 = 32;
pub const CBS_SIMPLE: i32 = 1;
pub const CBS_SORT: i32 = 256;
pub const CBS_UPPERCASE: i32 = 8192;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CBTACTIVATESTRUCT {
    pub fMouse: windows_core::BOOL,
    pub hWndActive: super::HWND,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CBT_CREATEWND = CBT_CREATEWNDA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const CB_ADDSTRING: i32 = 323;
pub const CB_DELETESTRING: i32 = 324;
pub const CB_DIR: i32 = 325;
pub const CB_ERR: i32 = -1;
pub const CB_ERRSPACE: i32 = -2;
pub const CB_FINDSTRING: i32 = 332;
pub const CB_FINDSTRINGEXACT: i32 = 344;
pub const CB_GETCOMBOBOXINFO: i32 = 356;
pub const CB_GETCOUNT: i32 = 326;
pub const CB_GETCURSEL: i32 = 327;
pub const CB_GETDROPPEDCONTROLRECT: i32 = 338;
pub const CB_GETDROPPEDSTATE: i32 = 343;
pub const CB_GETDROPPEDWIDTH: i32 = 351;
pub const CB_GETEDITSEL: i32 = 320;
pub const CB_GETEXTENDEDUI: i32 = 342;
pub const CB_GETHORIZONTALEXTENT: i32 = 349;
pub const CB_GETITEMDATA: i32 = 336;
pub const CB_GETITEMHEIGHT: i32 = 340;
pub const CB_GETLBTEXT: i32 = 328;
pub const CB_GETLBTEXTLEN: i32 = 329;
pub const CB_GETLOCALE: i32 = 346;
pub const CB_GETTOPINDEX: i32 = 347;
pub const CB_INITSTORAGE: i32 = 353;
pub const CB_INSERTSTRING: i32 = 330;
pub const CB_LIMITTEXT: i32 = 321;
pub const CB_MSGMAX: i32 = 357;
pub const CB_OKAY: i32 = 0;
pub const CB_RESETCONTENT: i32 = 331;
pub const CB_SELECTSTRING: i32 = 333;
pub const CB_SETCURSEL: i32 = 334;
pub const CB_SETDROPPEDWIDTH: i32 = 352;
pub const CB_SETEDITSEL: i32 = 322;
pub const CB_SETEXTENDEDUI: i32 = 341;
pub const CB_SETHORIZONTALEXTENT: i32 = 350;
pub const CB_SETITEMDATA: i32 = 337;
pub const CB_SETITEMHEIGHT: i32 = 339;
pub const CB_SETLOCALE: i32 = 345;
pub const CB_SETTOPINDEX: i32 = 348;
pub const CB_SHOWDROPDOWN: i32 = 335;
pub const CCHILDREN_SCROLLBAR: i32 = 5;
pub const CCHILDREN_TITLEBAR: i32 = 5;
pub const CDS_DISABLE_UNSAFE_MODES: i32 = 512;
pub const CDS_ENABLE_UNSAFE_MODES: i32 = 256;
pub const CDS_FULLSCREEN: i32 = 4;
pub const CDS_GLOBAL: i32 = 8;
pub const CDS_NORESET: i32 = 268435456;
pub const CDS_RESET: i32 = 1073741824;
pub const CDS_RESET_EX: i32 = 536870912;
pub const CDS_SET_PRIMARY: i32 = 16;
pub const CDS_TEST: i32 = 2;
pub const CDS_UPDATEREGISTRY: i32 = 1;
pub const CDS_VIDEOPARAMETERS: i32 = 32;
pub const CF_BITMAP: i32 = 2;
pub const CF_DIB: i32 = 8;
pub const CF_DIBV5: i32 = 17;
pub const CF_DIF: i32 = 5;
pub const CF_DSPBITMAP: i32 = 130;
pub const CF_DSPENHMETAFILE: i32 = 142;
pub const CF_DSPMETAFILEPICT: i32 = 131;
pub const CF_DSPTEXT: i32 = 129;
pub const CF_ENHMETAFILE: i32 = 14;
pub const CF_GDIOBJFIRST: i32 = 768;
pub const CF_GDIOBJLAST: i32 = 1023;
pub const CF_HDROP: i32 = 15;
pub const CF_LOCALE: i32 = 16;
pub const CF_MAX: i32 = 18;
pub const CF_METAFILEPICT: i32 = 3;
pub const CF_OEMTEXT: i32 = 7;
pub const CF_OWNERDISPLAY: i32 = 128;
pub const CF_PALETTE: i32 = 9;
pub const CF_PENDATA: i32 = 10;
pub const CF_PRIVATEFIRST: i32 = 512;
pub const CF_PRIVATELAST: i32 = 767;
pub const CF_RIFF: i32 = 11;
pub const CF_SYLK: i32 = 4;
pub const CF_TEXT: i32 = 1;
pub const CF_TIFF: i32 = 6;
pub const CF_UNICODETEXT: i32 = 13;
pub const CF_WAVE: i32 = 12;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGEFILTERSTRUCT {
    pub cbSize: u32,
    pub ExtStatus: u32,
}
pub const CHILDID_SELF: i32 = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLIENTCREATESTRUCT {
    pub hWindowMenu: super::HANDLE,
    pub idFirstChild: u32,
}
pub const COLOR_3DDKSHADOW: i32 = 21;
pub const COLOR_3DFACE: i32 = 15;
pub const COLOR_3DHIGHLIGHT: i32 = 20;
pub const COLOR_3DHILIGHT: i32 = 20;
pub const COLOR_3DLIGHT: i32 = 22;
pub const COLOR_3DSHADOW: i32 = 16;
pub const COLOR_ACTIVEBORDER: i32 = 10;
pub const COLOR_ACTIVECAPTION: i32 = 2;
pub const COLOR_APPWORKSPACE: i32 = 12;
pub const COLOR_BACKGROUND: i32 = 1;
pub const COLOR_BTNFACE: i32 = 15;
pub const COLOR_BTNHIGHLIGHT: i32 = 20;
pub const COLOR_BTNHILIGHT: i32 = 20;
pub const COLOR_BTNSHADOW: i32 = 16;
pub const COLOR_BTNTEXT: i32 = 18;
pub const COLOR_CAPTIONTEXT: i32 = 9;
pub const COLOR_DESKTOP: i32 = 1;
pub const COLOR_GRADIENTACTIVECAPTION: i32 = 27;
pub const COLOR_GRADIENTINACTIVECAPTION: i32 = 28;
pub const COLOR_GRAYTEXT: i32 = 17;
pub const COLOR_HIGHLIGHT: i32 = 13;
pub const COLOR_HIGHLIGHTTEXT: i32 = 14;
pub const COLOR_HOTLIGHT: i32 = 26;
pub const COLOR_INACTIVEBORDER: i32 = 11;
pub const COLOR_INACTIVECAPTION: i32 = 3;
pub const COLOR_INACTIVECAPTIONTEXT: i32 = 19;
pub const COLOR_INFOBK: i32 = 24;
pub const COLOR_INFOTEXT: i32 = 23;
pub const COLOR_MENU: i32 = 4;
pub const COLOR_MENUBAR: i32 = 30;
pub const COLOR_MENUHILIGHT: i32 = 29;
pub const COLOR_MENUTEXT: i32 = 7;
pub const COLOR_SCROLLBAR: i32 = 0;
pub const COLOR_WINDOW: i32 = 5;
pub const COLOR_WINDOWFRAME: i32 = 6;
pub const COLOR_WINDOWTEXT: i32 = 8;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMBOBOXINFO {
    pub cbSize: u32,
    pub rcItem: super::RECT,
    pub rcButton: super::RECT,
    pub stateButton: u32,
    pub hwndCombo: super::HWND,
    pub hwndItem: super::HWND,
    pub hwndList: super::HWND,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[cfg(target_arch = "x86")]
pub const CONSOLE_APPLICATION_16BIT: i32 = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CONSOLE_APPLICATION_16BIT: i32 = 0;
pub const CONSOLE_CARET_SELECTION: i32 = 1;
pub const CONSOLE_CARET_VISIBLE: i32 = 2;
pub const CONTACTVISUALIZATION_OFF: i32 = 0;
pub const CONTACTVISUALIZATION_ON: i32 = 1;
pub const CONTACTVISUALIZATION_PRESENTATIONMODE: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const CREATEPROCESS_MANIFEST_RESOURCE_ID: windows_core::PCWSTR = windows_core::PCWSTR(1 as _);
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CREATESTRUCT = CREATESTRUCTA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub lpszName: windows_core::PCSTR,
    pub lpszClass: windows_core::PCSTR,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub lpszName: windows_core::PCWSTR,
    pub lpszClass: windows_core::PCWSTR,
    pub dwExStyle: u32,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for CREATESTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CSOUND_SYSTEM: i32 = 16;
pub const CS_BYTEALIGNCLIENT: i32 = 4096;
pub const CS_BYTEALIGNWINDOW: i32 = 8192;
pub const CS_CLASSDC: i32 = 64;
pub const CS_DBLCLKS: i32 = 8;
pub const CS_DROPSHADOW: i32 = 131072;
pub const CS_GLOBALCLASS: i32 = 16384;
pub const CS_HREDRAW: i32 = 2;
pub const CS_IME: i32 = 65536;
pub const CS_NOCLOSE: i32 = 512;
pub const CS_OWNDC: i32 = 32;
pub const CS_PARENTDC: i32 = 128;
pub const CS_SAVEBITS: i32 = 2048;
pub const CS_VREDRAW: i32 = 1;
pub const CTLCOLOR_BTN: i32 = 3;
pub const CTLCOLOR_DLG: i32 = 4;
pub const CTLCOLOR_EDIT: i32 = 1;
pub const CTLCOLOR_LISTBOX: i32 = 2;
pub const CTLCOLOR_MAX: i32 = 7;
pub const CTLCOLOR_MSGBOX: i32 = 0;
pub const CTLCOLOR_SCROLLBAR: i32 = 5;
pub const CTLCOLOR_STATIC: i32 = 6;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CURSORINFO {
    pub cbSize: u32,
    pub flags: u32,
    pub hCursor: super::HCURSOR,
    pub ptScreenPos: super::POINT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CURSORSHAPE {
    pub xHotSpot: i32,
    pub yHotSpot: i32,
    pub cx: i32,
    pub cy: i32,
    pub cbWidth: i32,
    pub Planes: u8,
    pub BitsPixel: u8,
}
pub const CURSOR_CREATION_SCALING_DEFAULT: i32 = 2;
pub const CURSOR_CREATION_SCALING_NONE: i32 = 1;
pub const CURSOR_INVISIBLE: i32 = 0;
pub const CURSOR_SHOWING: i32 = 1;
pub const CURSOR_SUPPRESSED: i32 = 2;
pub const CWF_CREATE_ONLY: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CWPRETSTRUCT {
    pub lResult: super::LRESULT,
    pub lParam: super::LPARAM,
    pub wParam: super::WPARAM,
    pub message: u32,
    pub hwnd: super::HWND,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CWPSTRUCT {
    pub lParam: super::LPARAM,
    pub wParam: super::WPARAM,
    pub message: u32,
    pub hwnd: super::HWND,
}
pub const CWP_ALL: i32 = 0;
pub const CWP_SKIPDISABLED: i32 = 2;
pub const CWP_SKIPINVISIBLE: i32 = 1;
pub const CWP_SKIPTRANSPARENT: i32 = 4;
pub const CW_USEDEFAULT: i32 = -2147483648;
pub const DCDC_DEFAULT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 0;
pub const DCDC_DISABLE_FONT_UPDATE: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 1;
pub const DCDC_DISABLE_RELAYOUT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 2;
pub const DCX_CACHE: i32 = 2;
pub const DCX_CLIPCHILDREN: i32 = 8;
pub const DCX_CLIPSIBLINGS: i32 = 16;
pub const DCX_EXCLUDERGN: i32 = 64;
pub const DCX_EXCLUDEUPDATE: i32 = 256;
pub const DCX_INTERSECTRGN: i32 = 128;
pub const DCX_INTERSECTUPDATE: i32 = 512;
pub const DCX_LOCKWINDOWUPDATE: i32 = 1024;
pub const DCX_NORESETATTRS: i32 = 4;
pub const DCX_PARENTCLIP: i32 = 32;
pub const DCX_VALIDATE: i32 = 2097152;
pub const DCX_WINDOW: i32 = 1;
pub const DC_ACTIVE: i32 = 1;
pub const DC_BUTTONS: i32 = 4096;
pub const DC_GRADIENT: i32 = 32;
pub const DC_HASDEFID: i32 = 21323;
pub const DC_ICON: i32 = 4;
pub const DC_INBUTTON: i32 = 16;
pub const DC_SMALLCAP: i32 = 2;
pub const DC_TEXT: i32 = 8;
pub const DDC_DEFAULT: DIALOG_DPI_CHANGE_BEHAVIORS = 0;
pub const DDC_DISABLE_ALL: DIALOG_DPI_CHANGE_BEHAVIORS = 1;
pub const DDC_DISABLE_CONTROL_RELAYOUT: DIALOG_DPI_CHANGE_BEHAVIORS = 4;
pub const DDC_DISABLE_RESIZE: DIALOG_DPI_CHANGE_BEHAVIORS = 2;
pub const DDL_ARCHIVE: i32 = 32;
pub const DDL_DIRECTORY: i32 = 16;
pub const DDL_DRIVES: i32 = 16384;
pub const DDL_EXCLUSIVE: i32 = 32768;
pub const DDL_HIDDEN: i32 = 2;
pub const DDL_POSTMSGS: i32 = 8192;
pub const DDL_READONLY: i32 = 1;
pub const DDL_READWRITE: i32 = 0;
pub const DDL_SYSTEM: i32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEBUGHOOKINFO {
    pub idThread: u32,
    pub idThreadInstaller: u32,
    pub lParam: super::LPARAM,
    pub wParam: super::WPARAM,
    pub code: i32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DELETEITEMSTRUCT {
    pub CtlType: u32,
    pub CtlID: u32,
    pub itemID: u32,
    pub hwndItem: super::HWND,
    pub itemData: usize,
}
#[cfg(feature = "minwindef")]
pub type DESKTOPENUMPROC = DESKTOPENUMPROCA;
#[cfg(feature = "minwindef")]
pub type DESKTOPENUMPROCA = NAMEENUMPROCA;
#[cfg(feature = "minwindef")]
pub type DESKTOPENUMPROCW = NAMEENUMPROCW;
pub const DESKTOP_CREATEMENU: i32 = 4;
pub const DESKTOP_CREATEWINDOW: i32 = 2;
pub const DESKTOP_ENUMERATE: i32 = 64;
pub const DESKTOP_HOOKCONTROL: i32 = 8;
pub const DESKTOP_JOURNALPLAYBACK: i32 = 32;
pub const DESKTOP_JOURNALRECORD: i32 = 16;
pub const DESKTOP_READOBJECTS: i32 = 1;
pub const DESKTOP_SWITCHDESKTOP: i32 = 256;
pub const DESKTOP_WRITEOBJECTS: i32 = 128;
pub const DEVICE_NOTIFY_ALL_INTERFACE_CLASSES: i32 = 4;
pub const DEVICE_NOTIFY_SERVICE_HANDLE: i32 = 1;
pub const DEVICE_NOTIFY_WINDOW_HANDLE: i32 = 0;
pub const DFCS_ADJUSTRECT: i32 = 8192;
pub const DFCS_BUTTON3STATE: i32 = 8;
pub const DFCS_BUTTONCHECK: i32 = 0;
pub const DFCS_BUTTONPUSH: i32 = 16;
pub const DFCS_BUTTONRADIO: i32 = 4;
pub const DFCS_BUTTONRADIOIMAGE: i32 = 1;
pub const DFCS_BUTTONRADIOMASK: i32 = 2;
pub const DFCS_CAPTIONCLOSE: i32 = 0;
pub const DFCS_CAPTIONHELP: i32 = 4;
pub const DFCS_CAPTIONMAX: i32 = 2;
pub const DFCS_CAPTIONMIN: i32 = 1;
pub const DFCS_CAPTIONRESTORE: i32 = 3;
pub const DFCS_CHECKED: i32 = 1024;
pub const DFCS_FLAT: i32 = 16384;
pub const DFCS_HOT: i32 = 4096;
pub const DFCS_INACTIVE: i32 = 256;
pub const DFCS_MENUARROW: i32 = 0;
pub const DFCS_MENUARROWRIGHT: i32 = 4;
pub const DFCS_MENUBULLET: i32 = 2;
pub const DFCS_MENUCHECK: i32 = 1;
pub const DFCS_MONO: i32 = 32768;
pub const DFCS_PUSHED: i32 = 512;
pub const DFCS_SCROLLCOMBOBOX: i32 = 5;
pub const DFCS_SCROLLDOWN: i32 = 1;
pub const DFCS_SCROLLLEFT: i32 = 2;
pub const DFCS_SCROLLRIGHT: i32 = 3;
pub const DFCS_SCROLLSIZEGRIP: i32 = 8;
pub const DFCS_SCROLLSIZEGRIPRIGHT: i32 = 16;
pub const DFCS_SCROLLUP: i32 = 0;
pub const DFCS_TRANSPARENT: i32 = 2048;
pub const DFC_BUTTON: i32 = 4;
pub const DFC_CAPTION: i32 = 1;
pub const DFC_MENU: i32 = 2;
pub const DFC_POPUPMENU: i32 = 5;
pub const DFC_SCROLL: i32 = 3;
pub const DF_ALLOWOTHERACCOUNTHOOK: i32 = 1;
pub type DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = u32;
pub type DIALOG_DPI_CHANGE_BEHAVIORS = u32;
pub const DIFFERENCE: i32 = 11;
pub const DISP_CHANGE_BADDUALVIEW: i32 = -6;
pub const DISP_CHANGE_BADFLAGS: i32 = -4;
pub const DISP_CHANGE_BADMODE: i32 = -2;
pub const DISP_CHANGE_BADPARAM: i32 = -5;
pub const DISP_CHANGE_FAILED: i32 = -1;
pub const DISP_CHANGE_NOTUPDATED: i32 = -3;
pub const DISP_CHANGE_RESTART: i32 = 1;
pub const DISP_CHANGE_SUCCESSFUL: i32 = 0;
pub const DI_COMPAT: i32 = 4;
pub const DI_DEFAULTSIZE: i32 = 8;
pub const DI_IMAGE: i32 = 2;
pub const DI_MASK: i32 = 1;
pub const DI_NOMIRROR: i32 = 16;
pub const DI_NORMAL: i32 = 3;
pub const DLGC_BUTTON: i32 = 8192;
pub const DLGC_DEFPUSHBUTTON: i32 = 16;
pub const DLGC_HASSETSEL: i32 = 8;
pub const DLGC_RADIOBUTTON: i32 = 64;
pub const DLGC_STATIC: i32 = 256;
pub const DLGC_UNDEFPUSHBUTTON: i32 = 32;
pub const DLGC_WANTALLKEYS: i32 = 4;
pub const DLGC_WANTARROWS: i32 = 1;
pub const DLGC_WANTCHARS: i32 = 128;
pub const DLGC_WANTMESSAGE: i32 = 4;
pub const DLGC_WANTTAB: i32 = 2;
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
pub const DLGWINDOWEXTRA: i32 = 30;
pub const DM_GETDEFID: i32 = 1024;
pub const DM_POINTERHITTEST: i32 = 592;
pub const DM_REPOSITION: i32 = 1026;
pub const DM_SETDEFID: i32 = 1025;
pub const DOF_DIRECTORY: i32 = 32771;
pub const DOF_DOCUMENT: i32 = 32770;
pub const DOF_EXECUTABLE: i32 = 32769;
pub const DOF_MULTIPLE: i32 = 32772;
pub const DOF_PROGMAN: i32 = 1;
pub const DOF_SHELLDATA: i32 = 2;
pub const DO_DROPFILE: i32 = 1162627398;
pub const DO_PRINTFILE: i32 = 1414419024;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type DRAWSTATEPROC = Option<unsafe extern "system" fn(hdc: super::HDC, ldata: super::LPARAM, wdata: super::WPARAM, cx: i32, cy: i32) -> windows_core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRAWTEXTPARAMS {
    pub cbSize: u32,
    pub iTabLength: i32,
    pub iLeftMargin: i32,
    pub iRightMargin: i32,
    pub uiLengthDrawn: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DROPSTRUCT {
    pub hwndSource: super::HWND,
    pub hwndSink: super::HWND,
    pub wFmt: u32,
    pub dwData: usize,
    pub ptDrop: super::POINT,
    pub dwControlData: u32,
}
pub const DSS_DISABLED: i32 = 32;
pub const DSS_HIDEPREFIX: i32 = 512;
pub const DSS_MONO: i32 = 128;
pub const DSS_NORMAL: i32 = 0;
pub const DSS_PREFIXONLY: i32 = 1024;
pub const DSS_RIGHT: i32 = 32768;
pub const DSS_UNION: i32 = 16;
pub const DST_BITMAP: i32 = 4;
pub const DST_COMPLEX: i32 = 0;
pub const DST_ICON: i32 = 3;
pub const DST_PREFIXTEXT: i32 = 2;
pub const DST_TEXT: i32 = 1;
pub const DS_3DLOOK: i32 = 4;
pub const DS_ABSALIGN: i32 = 1;
pub const DS_CENTER: i32 = 2048;
pub const DS_CENTERMOUSE: i32 = 4096;
pub const DS_CONTEXTHELP: i32 = 8192;
pub const DS_CONTROL: i32 = 1024;
pub const DS_FIXEDSYS: i32 = 8;
pub const DS_LOCALEDIT: i32 = 32;
pub const DS_MODALFRAME: i32 = 128;
pub const DS_NOFAILCREATE: i32 = 16;
pub const DS_NOIDLEMSG: i32 = 256;
pub const DS_SETFONT: i32 = 64;
pub const DS_SETFOREGROUND: i32 = 512;
pub const DS_SHELLFONT: i32 = 72;
pub const DS_SYSMODAL: i32 = 2;
pub const DT_BOTTOM: i32 = 8;
pub const DT_CALCRECT: i32 = 1024;
pub const DT_CENTER: i32 = 1;
pub const DT_EDITCONTROL: i32 = 8192;
pub const DT_END_ELLIPSIS: i32 = 32768;
pub const DT_EXPANDTABS: i32 = 64;
pub const DT_EXTERNALLEADING: i32 = 512;
pub const DT_HIDEPREFIX: i32 = 1048576;
pub const DT_INTERNAL: i32 = 4096;
pub const DT_LEFT: i32 = 0;
pub const DT_MODIFYSTRING: i32 = 65536;
pub const DT_NOCLIP: i32 = 256;
pub const DT_NOFULLWIDTHCHARBREAK: i32 = 524288;
pub const DT_NOPREFIX: i32 = 2048;
pub const DT_PATH_ELLIPSIS: i32 = 16384;
pub const DT_PREFIXONLY: i32 = 2097152;
pub const DT_RIGHT: i32 = 2;
pub const DT_RTLREADING: i32 = 131072;
pub const DT_SINGLELINE: i32 = 32;
pub const DT_TABSTOP: i32 = 128;
pub const DT_TOP: i32 = 0;
pub const DT_VCENTER: i32 = 4;
pub const DT_WORDBREAK: i32 = 16;
pub const DT_WORD_ELLIPSIS: i32 = 262144;
pub const DWLP_MSGRESULT: i32 = 0;
pub const DWL_DLGPROC: i32 = 4;
pub const DWL_MSGRESULT: i32 = 0;
pub const DWL_USER: i32 = 8;
pub const EC_LEFTMARGIN: i32 = 1;
pub const EC_RIGHTMARGIN: i32 = 2;
pub const EC_USEFONTINFO: i32 = 65535;
pub const EDD_GET_DEVICE_INTERFACE_NAME: i32 = 1;
pub const EDGE_BUMP: i32 = 9;
pub const EDGE_ETCHED: i32 = 6;
pub const EDGE_RAISED: i32 = 5;
pub const EDGE_SUNKEN: i32 = 10;
pub type EDITWORDBREAKPROC = EDITWORDBREAKPROCA;
pub type EDITWORDBREAKPROCA = Option<unsafe extern "system" fn(lpch: windows_core::PCSTR, ichcurrent: i32, cch: i32, code: i32) -> i32>;
pub type EDITWORDBREAKPROCW = Option<unsafe extern "system" fn(lpch: windows_core::PCWSTR, ichcurrent: i32, cch: i32, code: i32) -> i32>;
pub type EDIT_CONTROL_FEATURE = i32;
pub const EDIT_CONTROL_FEATURE_ENTERPRISE_DATA_PROTECTION_PASTE_SUPPORT: EDIT_CONTROL_FEATURE = 0;
pub const EDIT_CONTROL_FEATURE_PASTE_NOTIFICATIONS: EDIT_CONTROL_FEATURE = 1;
pub const EDS_RAWMODE: i32 = 2;
pub const EDS_ROTATEDMODE: i32 = 4;
pub const EIMES_CANCELCOMPSTRINFOCUS: i32 = 2;
pub const EIMES_COMPLETECOMPSTRKILLFOCUS: i32 = 4;
pub const EIMES_GETCOMPSTRATONCE: i32 = 1;
pub const EMSIS_COMPOSITIONSTRING: i32 = 1;
pub const EM_CANUNDO: i32 = 198;
pub const EM_CHARFROMPOS: i32 = 215;
pub const EM_EMPTYUNDOBUFFER: i32 = 205;
pub const EM_ENABLEFEATURE: i32 = 218;
pub const EM_FMTLINES: i32 = 200;
pub const EM_GETFIRSTVISIBLELINE: i32 = 206;
pub const EM_GETHANDLE: i32 = 189;
pub const EM_GETIMESTATUS: i32 = 217;
pub const EM_GETLIMITTEXT: i32 = 213;
pub const EM_GETLINE: i32 = 196;
pub const EM_GETLINECOUNT: i32 = 186;
pub const EM_GETMARGINS: i32 = 212;
pub const EM_GETMODIFY: i32 = 184;
pub const EM_GETPASSWORDCHAR: i32 = 210;
pub const EM_GETRECT: i32 = 178;
pub const EM_GETSEL: i32 = 176;
pub const EM_GETTHUMB: i32 = 190;
pub const EM_GETWORDBREAKPROC: i32 = 209;
pub const EM_LIMITTEXT: i32 = 197;
pub const EM_LINEFROMCHAR: i32 = 201;
pub const EM_LINEINDEX: i32 = 187;
pub const EM_LINELENGTH: i32 = 193;
pub const EM_LINESCROLL: i32 = 182;
pub const EM_POSFROMCHAR: i32 = 214;
pub const EM_REPLACESEL: i32 = 194;
pub const EM_SCROLL: i32 = 181;
pub const EM_SCROLLCARET: i32 = 183;
pub const EM_SETHANDLE: i32 = 188;
pub const EM_SETIMESTATUS: i32 = 216;
pub const EM_SETLIMITTEXT: i32 = 197;
pub const EM_SETMARGINS: i32 = 211;
pub const EM_SETMODIFY: i32 = 185;
pub const EM_SETPASSWORDCHAR: i32 = 204;
pub const EM_SETREADONLY: i32 = 207;
pub const EM_SETRECT: i32 = 179;
pub const EM_SETRECTNP: i32 = 180;
pub const EM_SETSEL: i32 = 177;
pub const EM_SETTABSTOPS: i32 = 203;
pub const EM_SETWORDBREAKPROC: i32 = 208;
pub const EM_UNDO: i32 = 199;
pub const ENDSESSION_CLOSEAPP: i32 = 1;
pub const ENDSESSION_CRITICAL: i32 = 1073741824;
pub const ENDSESSION_LOGOFF: u32 = 2147483648;
pub const ENUM_CURRENT_SETTINGS: u32 = 4294967295;
pub const ENUM_REGISTRY_SETTINGS: u32 = 4294967294;
pub const EN_AFTER_PASTE: i32 = 2049;
pub const EN_ALIGN_LTR_EC: i32 = 1792;
pub const EN_ALIGN_RTL_EC: i32 = 1793;
pub const EN_BEFORE_PASTE: i32 = 2048;
pub const EN_CHANGE: i32 = 768;
pub const EN_ERRSPACE: i32 = 1280;
pub const EN_HSCROLL: i32 = 1537;
pub const EN_KILLFOCUS: i32 = 512;
pub const EN_MAXTEXT: i32 = 1281;
pub const EN_SETFOCUS: i32 = 256;
pub const EN_UPDATE: i32 = 1024;
pub const EN_VSCROLL: i32 = 1538;
pub const ESB_DISABLE_BOTH: i32 = 3;
pub const ESB_DISABLE_DOWN: i32 = 2;
pub const ESB_DISABLE_LEFT: i32 = 1;
pub const ESB_DISABLE_LTUP: i32 = 1;
pub const ESB_DISABLE_RIGHT: i32 = 2;
pub const ESB_DISABLE_RTDN: i32 = 2;
pub const ESB_DISABLE_UP: i32 = 1;
pub const ESB_ENABLE_BOTH: i32 = 0;
pub const ES_AUTOHSCROLL: i32 = 128;
pub const ES_AUTOVSCROLL: i32 = 64;
pub const ES_CENTER: i32 = 1;
pub const ES_LEFT: i32 = 0;
pub const ES_LOWERCASE: i32 = 16;
pub const ES_MULTILINE: i32 = 4;
pub const ES_NOHIDESEL: i32 = 256;
pub const ES_NUMBER: i32 = 8192;
pub const ES_OEMCONVERT: i32 = 1024;
pub const ES_PASSWORD: i32 = 32;
pub const ES_READONLY: i32 = 2048;
pub const ES_RIGHT: i32 = 2;
pub const ES_UPPERCASE: i32 = 8;
pub const ES_WANTRETURN: i32 = 4096;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENTMSG {
    pub message: u32,
    pub paramL: u32,
    pub paramH: u32,
    pub time: u32,
    pub hwnd: super::HWND,
}
pub const EVENT_AIA_END: i32 = 45055;
pub const EVENT_AIA_START: i32 = 40960;
pub const EVENT_CONSOLE_CARET: i32 = 16385;
pub const EVENT_CONSOLE_END: i32 = 16639;
pub const EVENT_CONSOLE_END_APPLICATION: i32 = 16391;
pub const EVENT_CONSOLE_LAYOUT: i32 = 16389;
pub const EVENT_CONSOLE_START_APPLICATION: i32 = 16390;
pub const EVENT_CONSOLE_UPDATE_REGION: i32 = 16386;
pub const EVENT_CONSOLE_UPDATE_SCROLL: i32 = 16388;
pub const EVENT_CONSOLE_UPDATE_SIMPLE: i32 = 16387;
pub const EVENT_MAX: i32 = 2147483647;
pub const EVENT_MIN: i32 = 1;
pub const EVENT_OBJECT_ACCELERATORCHANGE: i32 = 32786;
pub const EVENT_OBJECT_CLOAKED: i32 = 32791;
pub const EVENT_OBJECT_CONTENTSCROLLED: i32 = 32789;
pub const EVENT_OBJECT_CREATE: i32 = 32768;
pub const EVENT_OBJECT_DEFACTIONCHANGE: i32 = 32785;
pub const EVENT_OBJECT_DESCRIPTIONCHANGE: i32 = 32781;
pub const EVENT_OBJECT_DESTROY: i32 = 32769;
pub const EVENT_OBJECT_DRAGCANCEL: i32 = 32802;
pub const EVENT_OBJECT_DRAGCOMPLETE: i32 = 32803;
pub const EVENT_OBJECT_DRAGDROPPED: i32 = 32806;
pub const EVENT_OBJECT_DRAGENTER: i32 = 32804;
pub const EVENT_OBJECT_DRAGLEAVE: i32 = 32805;
pub const EVENT_OBJECT_DRAGSTART: i32 = 32801;
pub const EVENT_OBJECT_END: i32 = 33023;
pub const EVENT_OBJECT_FOCUS: i32 = 32773;
pub const EVENT_OBJECT_HELPCHANGE: i32 = 32784;
pub const EVENT_OBJECT_HIDE: i32 = 32771;
pub const EVENT_OBJECT_HOSTEDOBJECTSINVALIDATED: i32 = 32800;
pub const EVENT_OBJECT_IME_CHANGE: i32 = 32809;
pub const EVENT_OBJECT_IME_HIDE: i32 = 32808;
pub const EVENT_OBJECT_IME_SHOW: i32 = 32807;
pub const EVENT_OBJECT_INVOKED: i32 = 32787;
pub const EVENT_OBJECT_LIVEREGIONCHANGED: i32 = 32793;
pub const EVENT_OBJECT_LOCATIONCHANGE: i32 = 32779;
pub const EVENT_OBJECT_NAMECHANGE: i32 = 32780;
pub const EVENT_OBJECT_PARENTCHANGE: i32 = 32783;
pub const EVENT_OBJECT_REORDER: i32 = 32772;
pub const EVENT_OBJECT_SELECTION: i32 = 32774;
pub const EVENT_OBJECT_SELECTIONADD: i32 = 32775;
pub const EVENT_OBJECT_SELECTIONREMOVE: i32 = 32776;
pub const EVENT_OBJECT_SELECTIONWITHIN: i32 = 32777;
pub const EVENT_OBJECT_SHOW: i32 = 32770;
pub const EVENT_OBJECT_STATECHANGE: i32 = 32778;
pub const EVENT_OBJECT_TEXTEDIT_CONVERSIONTARGETCHANGED: i32 = 32816;
pub const EVENT_OBJECT_TEXTSELECTIONCHANGED: i32 = 32788;
pub const EVENT_OBJECT_UNCLOAKED: i32 = 32792;
pub const EVENT_OBJECT_VALUECHANGE: i32 = 32782;
pub const EVENT_OEM_DEFINED_END: i32 = 511;
pub const EVENT_OEM_DEFINED_START: i32 = 257;
pub const EVENT_SYSTEM_ALERT: i32 = 2;
pub const EVENT_SYSTEM_ARRANGMENTPREVIEW: i32 = 32790;
pub const EVENT_SYSTEM_CAPTUREEND: i32 = 9;
pub const EVENT_SYSTEM_CAPTURESTART: i32 = 8;
pub const EVENT_SYSTEM_CONTEXTHELPEND: i32 = 13;
pub const EVENT_SYSTEM_CONTEXTHELPSTART: i32 = 12;
pub const EVENT_SYSTEM_DESKTOPSWITCH: i32 = 32;
pub const EVENT_SYSTEM_DIALOGEND: i32 = 17;
pub const EVENT_SYSTEM_DIALOGSTART: i32 = 16;
pub const EVENT_SYSTEM_DRAGDROPEND: i32 = 15;
pub const EVENT_SYSTEM_DRAGDROPSTART: i32 = 14;
pub const EVENT_SYSTEM_END: i32 = 255;
pub const EVENT_SYSTEM_FOREGROUND: i32 = 3;
pub const EVENT_SYSTEM_IME_KEY_NOTIFICATION: i32 = 41;
pub const EVENT_SYSTEM_MENUEND: i32 = 5;
pub const EVENT_SYSTEM_MENUPOPUPEND: i32 = 7;
pub const EVENT_SYSTEM_MENUPOPUPSTART: i32 = 6;
pub const EVENT_SYSTEM_MENUSTART: i32 = 4;
pub const EVENT_SYSTEM_MINIMIZEEND: i32 = 23;
pub const EVENT_SYSTEM_MINIMIZESTART: i32 = 22;
pub const EVENT_SYSTEM_MOVESIZEEND: i32 = 11;
pub const EVENT_SYSTEM_MOVESIZESTART: i32 = 10;
pub const EVENT_SYSTEM_SCROLLINGEND: i32 = 19;
pub const EVENT_SYSTEM_SCROLLINGSTART: i32 = 18;
pub const EVENT_SYSTEM_SOUND: i32 = 1;
pub const EVENT_SYSTEM_SWITCHEND: i32 = 21;
pub const EVENT_SYSTEM_SWITCHER_APPDROPPED: i32 = 38;
pub const EVENT_SYSTEM_SWITCHER_APPGRABBED: i32 = 36;
pub const EVENT_SYSTEM_SWITCHER_APPOVERTARGET: i32 = 37;
pub const EVENT_SYSTEM_SWITCHER_CANCELLED: i32 = 39;
pub const EVENT_SYSTEM_SWITCHSTART: i32 = 20;
pub const EVENT_UIA_EVENTID_END: i32 = 20223;
pub const EVENT_UIA_EVENTID_START: i32 = 19968;
pub const EVENT_UIA_PROPID_END: i32 = 30207;
pub const EVENT_UIA_PROPID_START: i32 = 29952;
pub const EWX_ARSO: i32 = 67108864;
pub const EWX_BOOTOPTIONS: i32 = 16777216;
pub const EWX_CHECK_SAFE_FOR_SERVER: i32 = 134217728;
pub const EWX_FORCE: i32 = 4;
pub const EWX_FORCEIFHUNG: i32 = 16;
pub const EWX_HYBRID_SHUTDOWN: i32 = 4194304;
pub const EWX_LOGOFF: i32 = 0;
pub const EWX_POWEROFF: i32 = 8;
pub const EWX_QUICKRESOLVE: i32 = 32;
pub const EWX_REBOOT: i32 = 2;
pub const EWX_RESTARTAPPS: i32 = 64;
pub const EWX_SHUTDOWN: i32 = 1;
pub const EWX_SYSTEM_INITIATED: i32 = 268435456;
pub const FALT: i32 = 16;
pub const FAPPCOMMAND_KEY: i32 = 0;
pub const FAPPCOMMAND_MASK: i32 = 61440;
pub const FAPPCOMMAND_MOUSE: i32 = 32768;
pub const FAPPCOMMAND_OEM: i32 = 4096;
pub const FCONTROL: i32 = 8;
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
pub const FE_FONTSMOOTHINGCLEARTYPE: i32 = 2;
pub const FE_FONTSMOOTHINGORIENTATIONBGR: i32 = 0;
pub const FE_FONTSMOOTHINGORIENTATIONRGB: i32 = 1;
pub const FE_FONTSMOOTHINGSTANDARD: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILTERKEYS {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iWaitMSec: u32,
    pub iDelayMSec: u32,
    pub iRepeatMSec: u32,
    pub iBounceMSec: u32,
}
pub const FKF_AVAILABLE: i32 = 2;
pub const FKF_CLICKON: i32 = 64;
pub const FKF_CONFIRMHOTKEY: i32 = 8;
pub const FKF_FILTERKEYSON: i32 = 1;
pub const FKF_HOTKEYACTIVE: i32 = 4;
pub const FKF_HOTKEYSOUND: i32 = 16;
pub const FKF_INDICATOR: i32 = 32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FLASHWINFO {
    pub cbSize: u32,
    pub hwnd: super::HWND,
    pub dwFlags: u32,
    pub uCount: u32,
    pub dwTimeout: u32,
}
pub const FLASHW_ALL: i32 = 3;
pub const FLASHW_CAPTION: i32 = 1;
pub const FLASHW_STOP: i32 = 0;
pub const FLASHW_TIMER: i32 = 4;
pub const FLASHW_TIMERNOFG: i32 = 12;
pub const FLASHW_TRAY: i32 = 2;
pub const FNOINVERT: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FRAME_MARGIN {
    pub left: i16,
    pub right: i16,
    pub top: i16,
    pub bottom: i16,
}
pub const FSHIFT: i32 = 4;
pub const FVIRTKEY: i32 = 1;
pub const GA_PARENT: i32 = 1;
pub const GA_ROOT: i32 = 2;
pub const GA_ROOTOWNER: i32 = 3;
pub const GCF_INCLUDE_ANCESTORS: i32 = 1;
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
pub const GC_ALLGESTURES: i32 = 1;
pub const GC_PAN: i32 = 1;
pub const GC_PAN_WITH_GUTTER: i32 = 8;
pub const GC_PAN_WITH_INERTIA: i32 = 16;
pub const GC_PAN_WITH_SINGLE_FINGER_HORIZONTALLY: i32 = 4;
pub const GC_PAN_WITH_SINGLE_FINGER_VERTICALLY: i32 = 2;
pub const GC_PRESSANDTAP: i32 = 1;
pub const GC_ROLLOVER: i32 = 1;
pub const GC_ROTATE: i32 = 1;
pub const GC_TWOFINGERTAP: i32 = 1;
pub const GC_ZOOM: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GESTURECONFIG {
    pub dwID: u32,
    pub dwWant: u32,
    pub dwBlock: u32,
}
pub const GESTURECONFIGMAXCOUNT: i32 = 256;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GESTURENOTIFYSTRUCT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTarget: super::HWND,
    pub ptsLocation: super::POINTS,
    pub dwInstanceID: u32,
}
pub const GESTUREVISUALIZATION_DOUBLETAP: i32 = 2;
pub const GESTUREVISUALIZATION_OFF: i32 = 0;
pub const GESTUREVISUALIZATION_ON: i32 = 31;
pub const GESTUREVISUALIZATION_PRESSANDHOLD: i32 = 8;
pub const GESTUREVISUALIZATION_PRESSANDTAP: i32 = 4;
pub const GESTUREVISUALIZATION_RIGHTTAP: i32 = 16;
pub const GESTUREVISUALIZATION_TAP: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GETCLIPBMETADATA {
    pub Version: u32,
    pub IsDelayRendered: windows_core::BOOL,
    pub IsSynthetic: windows_core::BOOL,
}
pub const GF_BEGIN: i32 = 1;
pub const GF_END: i32 = 4;
pub const GF_INERTIA: i32 = 2;
pub const GIDC_ARRIVAL: i32 = 1;
pub const GIDC_REMOVAL: i32 = 2;
pub const GID_BEGIN: i32 = 1;
pub const GID_END: i32 = 2;
pub const GID_PAN: i32 = 4;
pub const GID_PRESSANDTAP: i32 = 7;
pub const GID_ROLLOVER: i32 = 7;
pub const GID_ROTATE: i32 = 5;
pub const GID_TWOFINGERTAP: i32 = 6;
pub const GID_ZOOM: i32 = 3;
pub const GMDI_GOINTOPOPUPS: i32 = 2;
pub const GMDI_USEDISABLED: i32 = 1;
pub const GMMP_USE_DISPLAY_POINTS: i32 = 1;
pub const GMMP_USE_HIGH_RESOLUTION_POINTS: i32 = 2;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type GRAYSTRINGPROC = Option<unsafe extern "system" fn(param0: super::HDC, param1: super::LPARAM, param2: i32) -> windows_core::BOOL>;
pub const GR_GDIOBJECTS: i32 = 0;
pub const GR_GDIOBJECTS_PEAK: i32 = 2;
#[cfg(feature = "winnt")]
pub const GR_GLOBAL: super::HANDLE = super::HANDLE(-2 as _);
pub const GR_USEROBJECTS: i32 = 1;
pub const GR_USEROBJECTS_PEAK: i32 = 4;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[cfg(target_arch = "x86")]
pub const GUI_16BITTASK: i32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const GUI_16BITTASK: i32 = 0;
pub const GUI_CARETBLINKING: i32 = 1;
pub const GUI_INMENUMODE: i32 = 4;
pub const GUI_INMOVESIZE: i32 = 2;
pub const GUI_POPUPMENUMODE: i32 = 16;
pub const GUI_SYSTEMMENUMODE: i32 = 8;
pub const GWFS_INCLUDE_ANCESTORS: i32 = 1;
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
pub const GW_CHILD: i32 = 5;
pub const GW_ENABLEDPOPUP: i32 = 6;
pub const GW_HWNDFIRST: i32 = 0;
pub const GW_HWNDLAST: i32 = 1;
pub const GW_HWNDNEXT: i32 = 2;
pub const GW_HWNDPREV: i32 = 3;
pub const GW_MAX: i32 = 6;
pub const GW_OWNER: i32 = 4;
pub type HANDEDNESS = i32;
pub const HANDEDNESS_LEFT: HANDEDNESS = 0;
pub const HANDEDNESS_RIGHT: HANDEDNESS = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HARDWAREHOOKSTRUCT {
    pub hwnd: super::HWND,
    pub message: u32,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HARDWAREINPUT {
    pub uMsg: u32,
    pub wParamL: u16,
    pub wParamH: u16,
}
#[cfg(feature = "windef")]
pub const HBMMENU_CALLBACK: super::HBITMAP = super::HBITMAP(-1 as _);
#[cfg(feature = "windef")]
pub const HBMMENU_MBAR_CLOSE: super::HBITMAP = super::HBITMAP(5 as _);
#[cfg(feature = "windef")]
pub const HBMMENU_MBAR_CLOSE_D: super::HBITMAP = super::HBITMAP(6 as _);
#[cfg(feature = "windef")]
pub const HBMMENU_MBAR_MINIMIZE: super::HBITMAP = super::HBITMAP(3 as _);
#[cfg(feature = "windef")]
pub const HBMMENU_MBAR_MINIMIZE_D: super::HBITMAP = super::HBITMAP(7 as _);
#[cfg(feature = "windef")]
pub const HBMMENU_MBAR_RESTORE: super::HBITMAP = super::HBITMAP(2 as _);
#[cfg(feature = "windef")]
pub const HBMMENU_POPUP_CLOSE: super::HBITMAP = super::HBITMAP(8 as _);
#[cfg(feature = "windef")]
pub const HBMMENU_POPUP_MAXIMIZE: super::HBITMAP = super::HBITMAP(10 as _);
#[cfg(feature = "windef")]
pub const HBMMENU_POPUP_MINIMIZE: super::HBITMAP = super::HBITMAP(11 as _);
#[cfg(feature = "windef")]
pub const HBMMENU_POPUP_RESTORE: super::HBITMAP = super::HBITMAP(9 as _);
#[cfg(feature = "windef")]
pub const HBMMENU_SYSTEM: super::HBITMAP = super::HBITMAP(1 as _);
pub const HCBT_ACTIVATE: i32 = 5;
pub const HCBT_CLICKSKIPPED: i32 = 6;
pub const HCBT_CREATEWND: i32 = 3;
pub const HCBT_DESTROYWND: i32 = 4;
pub const HCBT_KEYSKIPPED: i32 = 7;
pub const HCBT_MINMAX: i32 = 1;
pub const HCBT_MOVESIZE: i32 = 0;
pub const HCBT_QS: i32 = 2;
pub const HCBT_SETFOCUS: i32 = 9;
pub const HCBT_SYSCOMMAND: i32 = 8;
pub const HCF_AVAILABLE: i32 = 2;
pub const HCF_CONFIRMHOTKEY: i32 = 8;
pub const HCF_DEFAULTDESKTOP: i32 = 512;
pub const HCF_HIGHCONTRASTON: i32 = 1;
pub const HCF_HOTKEYACTIVE: i32 = 4;
pub const HCF_HOTKEYAVAILABLE: i32 = 64;
pub const HCF_HOTKEYSOUND: i32 = 16;
pub const HCF_INDICATOR: i32 = 32;
pub const HCF_LOGONDESKTOP: i32 = 256;
pub const HCF_OPTION_NOTHEMECHANGE: i32 = 4096;
pub const HC_ACTION: i32 = 0;
pub const HC_GETNEXT: i32 = 1;
pub const HC_NOREM: i32 = 3;
pub const HC_NOREMOVE: i32 = 3;
pub const HC_SKIP: i32 = 2;
pub const HC_SYSMODALOFF: i32 = 5;
pub const HC_SYSMODALON: i32 = 4;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDEVNOTIFY(pub *mut core::ffi::c_void);
impl Default for HDEVNOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type HDWP = super::HANDLE;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HELPINFO {
    pub cbSize: u32,
    pub iContextType: i32,
    pub iCtrlId: i32,
    pub hItemHandle: super::HANDLE,
    pub dwContextId: usize,
    pub MousePos: super::POINT,
}
pub const HELPINFO_MENUITEM: i32 = 2;
pub const HELPINFO_WINDOW: i32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HELPPOLY(pub u32);
pub type HELPWININFO = HELPWININFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const HELP_COMMAND: i32 = 258;
pub const HELP_CONTENTS: i32 = 3;
pub const HELP_CONTEXT: i32 = 1;
pub const HELP_CONTEXTMENU: i32 = 10;
pub const HELP_CONTEXTPOPUP: i32 = 8;
pub const HELP_FINDER: i32 = 11;
pub const HELP_FORCEFILE: i32 = 9;
pub const HELP_HELPONHELP: i32 = 4;
pub const HELP_INDEX: i32 = 3;
pub const HELP_KEY: i32 = 257;
pub const HELP_MULTIKEY: i32 = 513;
pub const HELP_PARTIALKEY: i32 = 261;
pub const HELP_QUIT: i32 = 2;
pub const HELP_SETCONTENTS: i32 = 5;
pub const HELP_SETINDEX: i32 = 5;
pub const HELP_SETPOPUP_POS: i32 = 13;
pub const HELP_SETWINPOS: i32 = 515;
pub const HELP_TCARD: i32 = 32768;
pub const HELP_TCARD_DATA: i32 = 16;
pub const HELP_TCARD_OTHER_CALLER: i32 = 17;
pub const HELP_WM_HELP: i32 = 12;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGESTUREINFO(pub *mut core::ffi::c_void);
impl Default for HGESTUREINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HIDE_WINDOW: i32 = 0;
pub type HIGHCONTRAST = HIGHCONTRASTA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HIGHCONTRASTA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpszDefaultScheme: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HIGHCONTRASTW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpszDefaultScheme: windows_core::PWSTR,
}
pub const HKL_NEXT: i32 = 1;
pub const HKL_PREV: i32 = 0;
#[cfg(feature = "minwindef")]
pub type HOOKPROC = Option<unsafe extern "system" fn(code: i32, wparam: super::WPARAM, lparam: super::LPARAM) -> super::LRESULT>;
pub const HOVER_DEFAULT: u32 = 4294967295;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPOWERNOTIFY(pub *mut core::ffi::c_void);
impl Default for HPOWERNOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRAWINPUT(pub *mut core::ffi::c_void);
impl Default for HRAWINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HSHELL_ACCESSIBILITYSTATE: i32 = 11;
pub const HSHELL_ACTIVATESHELLWINDOW: i32 = 3;
pub const HSHELL_APPCOMMAND: i32 = 12;
pub const HSHELL_ENDTASK: i32 = 10;
pub const HSHELL_FLASH: i32 = 32774;
pub const HSHELL_GETMINRECT: i32 = 5;
pub const HSHELL_HIGHBIT: i32 = 32768;
pub const HSHELL_LANGUAGE: i32 = 8;
pub const HSHELL_MONITORCHANGED: i32 = 16;
pub const HSHELL_REDRAW: i32 = 6;
pub const HSHELL_RUDEAPPACTIVATED: i32 = 32772;
pub const HSHELL_SYSMENU: i32 = 9;
pub const HSHELL_TASKMAN: i32 = 7;
pub const HSHELL_WINDOWACTIVATED: i32 = 4;
pub const HSHELL_WINDOWCREATED: i32 = 1;
pub const HSHELL_WINDOWDESTROYED: i32 = 2;
pub const HSHELL_WINDOWREPLACED: i32 = 13;
pub const HSHELL_WINDOWREPLACING: i32 = 14;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSYNTHETICPOINTERDEVICE(pub *mut core::ffi::c_void);
impl Default for HSYNTHETICPOINTERDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTBORDER: i32 = 18;
pub const HTBOTTOM: i32 = 15;
pub const HTBOTTOMLEFT: i32 = 16;
pub const HTBOTTOMRIGHT: i32 = 17;
pub const HTCAPTION: i32 = 2;
pub const HTCLIENT: i32 = 1;
pub const HTCLOSE: i32 = 20;
pub const HTERROR: i32 = -2;
pub const HTGROWBOX: i32 = 4;
pub const HTHELP: i32 = 21;
pub const HTHSCROLL: i32 = 6;
pub const HTLEFT: i32 = 10;
pub const HTMAXBUTTON: i32 = 9;
pub const HTMENU: i32 = 5;
pub const HTMINBUTTON: i32 = 8;
pub const HTNOWHERE: i32 = 0;
pub const HTOBJECT: i32 = 19;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HTOUCHINPUT(pub *mut core::ffi::c_void);
impl Default for HTOUCHINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTREDUCE: i32 = 8;
pub const HTRIGHT: i32 = 11;
pub const HTSIZE: i32 = 4;
pub const HTSIZEFIRST: i32 = 10;
pub const HTSIZELAST: i32 = 17;
pub const HTSYSMENU: i32 = 3;
pub const HTTOP: i32 = 12;
pub const HTTOPLEFT: i32 = 13;
pub const HTTOPRIGHT: i32 = 14;
pub const HTTRANSPARENT: i32 = -1;
pub const HTVSCROLL: i32 = 7;
pub const HTZOOM: i32 = 9;
#[cfg(feature = "windef")]
pub const HWND_BOTTOM: super::HWND = super::HWND(1 as _);
#[cfg(feature = "windef")]
pub const HWND_BROADCAST: super::HWND = super::HWND(65535 as _);
#[cfg(feature = "windef")]
pub const HWND_DESKTOP: super::HWND = super::HWND(0 as _);
#[cfg(feature = "windef")]
pub const HWND_MESSAGE: super::HWND = super::HWND(-3 as _);
#[cfg(feature = "windef")]
pub const HWND_NOTOPMOST: super::HWND = super::HWND(-2 as _);
#[cfg(feature = "windef")]
pub const HWND_TOP: super::HWND = super::HWND(0 as _);
#[cfg(feature = "windef")]
pub const HWND_TOPMOST: super::HWND = super::HWND(-1 as _);
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ICONINFO {
    pub fIcon: windows_core::BOOL,
    pub xHotspot: u32,
    pub yHotspot: u32,
    pub hbmMask: super::HBITMAP,
    pub hbmColor: super::HBITMAP,
}
#[cfg(feature = "windef")]
pub type ICONINFOEX = ICONINFOEXA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ICONINFOEXA {
    pub cbSize: u32,
    pub fIcon: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ICONINFOEXW {
    pub cbSize: u32,
    pub fIcon: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ICONMETRICSA {
    pub cbSize: u32,
    pub iHorzSpacing: i32,
    pub iVertSpacing: i32,
    pub iTitleWrap: i32,
    pub lfFont: super::LOGFONTA,
}
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ICONMETRICSW {
    pub cbSize: u32,
    pub iHorzSpacing: i32,
    pub iVertSpacing: i32,
    pub iTitleWrap: i32,
    pub lfFont: super::LOGFONTW,
}
pub const ICON_BIG: i32 = 1;
pub const ICON_SMALL: i32 = 0;
pub const ICON_SMALL2: i32 = 2;
pub const IDABORT: i32 = 3;
pub const IDANI_CAPTION: i32 = 3;
pub const IDANI_OPEN: i32 = 1;
pub const IDCANCEL: i32 = 2;
pub const IDCLOSE: i32 = 8;
pub const IDCONTINUE: i32 = 11;
pub const IDC_APPSTARTING: windows_core::PCWSTR = windows_core::PCWSTR(32650 as _);
pub const IDC_ARROW: windows_core::PCWSTR = windows_core::PCWSTR(32512 as _);
pub const IDC_CROSS: windows_core::PCWSTR = windows_core::PCWSTR(32515 as _);
pub const IDC_HAND: windows_core::PCWSTR = windows_core::PCWSTR(32649 as _);
pub const IDC_HELP: windows_core::PCWSTR = windows_core::PCWSTR(32651 as _);
pub const IDC_IBEAM: windows_core::PCWSTR = windows_core::PCWSTR(32513 as _);
pub const IDC_ICON: windows_core::PCWSTR = windows_core::PCWSTR(32641 as _);
pub const IDC_NO: windows_core::PCWSTR = windows_core::PCWSTR(32648 as _);
pub const IDC_PERSON: windows_core::PCWSTR = windows_core::PCWSTR(32672 as _);
pub const IDC_PIN: windows_core::PCWSTR = windows_core::PCWSTR(32671 as _);
pub const IDC_SIZE: windows_core::PCWSTR = windows_core::PCWSTR(32640 as _);
pub const IDC_SIZEALL: windows_core::PCWSTR = windows_core::PCWSTR(32646 as _);
pub const IDC_SIZENESW: windows_core::PCWSTR = windows_core::PCWSTR(32643 as _);
pub const IDC_SIZENS: windows_core::PCWSTR = windows_core::PCWSTR(32645 as _);
pub const IDC_SIZENWSE: windows_core::PCWSTR = windows_core::PCWSTR(32642 as _);
pub const IDC_SIZEWE: windows_core::PCWSTR = windows_core::PCWSTR(32644 as _);
pub const IDC_UPARROW: windows_core::PCWSTR = windows_core::PCWSTR(32516 as _);
pub const IDC_WAIT: windows_core::PCWSTR = windows_core::PCWSTR(32514 as _);
pub const IDHELP: i32 = 9;
pub const IDHOT_SNAPDESKTOP: i32 = -2;
pub const IDHOT_SNAPWINDOW: i32 = -1;
pub const IDH_CANCEL: i32 = 28444;
pub const IDH_GENERIC_HELP_BUTTON: i32 = 28442;
pub const IDH_HELP: i32 = 28445;
pub const IDH_MISSING_CONTEXT: i32 = 28441;
pub const IDH_NO_HELP: i32 = 28440;
pub const IDH_OK: i32 = 28443;
pub const IDIGNORE: i32 = 5;
pub const IDI_APPLICATION: windows_core::PCWSTR = windows_core::PCWSTR(32512 as _);
pub const IDI_ASTERISK: windows_core::PCWSTR = windows_core::PCWSTR(32516 as _);
pub const IDI_EXCLAMATION: windows_core::PCWSTR = windows_core::PCWSTR(32515 as _);
pub const IDI_HAND: windows_core::PCWSTR = windows_core::PCWSTR(32513 as _);
pub const IDI_QUESTION: windows_core::PCWSTR = windows_core::PCWSTR(32514 as _);
pub const IDI_SHIELD: windows_core::PCWSTR = windows_core::PCWSTR(32518 as _);
pub const IDI_WINLOGO: windows_core::PCWSTR = windows_core::PCWSTR(32517 as _);
pub const IDNO: i32 = 7;
pub const IDOK: i32 = 1;
pub const IDRETRY: i32 = 4;
pub const IDTIMEOUT: i32 = 32000;
pub const IDTRYAGAIN: i32 = 10;
pub const IDYES: i32 = 6;
pub const IMAGE_BITMAP: i32 = 0;
pub const IMAGE_CURSOR: i32 = 2;
pub const IMAGE_ENHMETAFILE: i32 = 3;
pub const IMAGE_ICON: i32 = 1;
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
pub const INDEXID_CONTAINER: i32 = 0;
pub const INDEXID_OBJECT: i32 = 0;
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
pub const INPUTLANGCHANGE_BACKWARD: i32 = 4;
pub const INPUTLANGCHANGE_FORWARD: i32 = 2;
pub const INPUTLANGCHANGE_SYSCHARSET: i32 = 1;
pub const INPUT_HARDWARE: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INPUT_INJECTION_VALUE {
    pub page: u16,
    pub usage: u16,
    pub value: i32,
    pub index: u16,
}
pub const INPUT_KEYBOARD: i32 = 1;
pub type INPUT_MESSAGE_DEVICE_TYPE = i32;
pub type INPUT_MESSAGE_ORIGIN_ID = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INPUT_MESSAGE_SOURCE {
    pub deviceType: INPUT_MESSAGE_DEVICE_TYPE,
    pub originId: INPUT_MESSAGE_ORIGIN_ID,
}
pub const INPUT_MOUSE: i32 = 0;
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const INVALID_MONITOR_TOPOLOGY_ID: i32 = 0;
pub const ISMEX_CALLBACK: i32 = 4;
pub const ISMEX_NOSEND: i32 = 0;
pub const ISMEX_NOTIFY: i32 = 2;
pub const ISMEX_REPLIED: i32 = 8;
pub const ISMEX_SEND: i32 = 1;
pub const ISOLATIONAWARE_MANIFEST_RESOURCE_ID: windows_core::PCWSTR = windows_core::PCWSTR(2 as _);
pub const ISOLATIONAWARE_NOSTATICIMPORT_MANIFEST_RESOURCE_ID: windows_core::PCWSTR = windows_core::PCWSTR(3 as _);
pub const ISOLATIONPOLICY_BROWSER_MANIFEST_RESOURCE_ID: windows_core::PCWSTR = windows_core::PCWSTR(5 as _);
pub const ISOLATIONPOLICY_MANIFEST_RESOURCE_ID: windows_core::PCWSTR = windows_core::PCWSTR(4 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KBDLLHOOKSTRUCT {
    pub vkCode: u32,
    pub scanCode: u32,
    pub flags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KEYBDINPUT {
    pub wVk: u16,
    pub wScan: u16,
    pub dwFlags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
pub const KEYBOARD_OVERRUN_MAKE_CODE: i32 = 255;
pub const KEYEVENTF_EXTENDEDKEY: i32 = 1;
pub const KEYEVENTF_KEYUP: i32 = 2;
pub const KEYEVENTF_SCANCODE: i32 = 8;
pub const KEYEVENTF_UNICODE: i32 = 4;
pub const KF_ALTDOWN: i32 = 8192;
pub const KF_DLGMODE: i32 = 2048;
pub const KF_EXTENDED: i32 = 256;
pub const KF_MENUMODE: i32 = 4096;
pub const KF_REPEAT: i32 = 16384;
pub const KF_UP: i32 = 32768;
pub const KLF_ACTIVATE: i32 = 1;
pub const KLF_NOTELLSHELL: i32 = 128;
pub const KLF_REORDER: i32 = 8;
pub const KLF_REPLACELANG: i32 = 16;
pub const KLF_RESET: i32 = 1073741824;
pub const KLF_SETFORPROCESS: i32 = 256;
pub const KLF_SHIFTLOCK: i32 = 65536;
pub const KLF_SUBSTITUTE_OK: i32 = 2;
pub const KL_NAMELENGTH: i32 = 9;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LASTINPUTINFO {
    pub cbSize: u32,
    pub dwTime: u32,
}
pub const LBN_DBLCLK: i32 = 2;
pub const LBN_ERRSPACE: i32 = -2;
pub const LBN_KILLFOCUS: i32 = 5;
pub const LBN_SELCANCEL: i32 = 3;
pub const LBN_SELCHANGE: i32 = 1;
pub const LBN_SETFOCUS: i32 = 4;
pub const LBS_COMBOBOX: i32 = 32768;
pub const LBS_DISABLENOSCROLL: i32 = 4096;
pub const LBS_EXTENDEDSEL: i32 = 2048;
pub const LBS_HASSTRINGS: i32 = 64;
pub const LBS_MULTICOLUMN: i32 = 512;
pub const LBS_MULTIPLESEL: i32 = 8;
pub const LBS_NODATA: i32 = 8192;
pub const LBS_NOINTEGRALHEIGHT: i32 = 256;
pub const LBS_NOREDRAW: i32 = 4;
pub const LBS_NOSEL: i32 = 16384;
pub const LBS_NOTIFY: i32 = 1;
pub const LBS_OWNERDRAWFIXED: i32 = 16;
pub const LBS_OWNERDRAWVARIABLE: i32 = 32;
pub const LBS_SORT: i32 = 2;
pub const LBS_STANDARD: i32 = 10485763;
pub const LBS_USETABSTOPS: i32 = 128;
pub const LBS_WANTKEYBOARDINPUT: i32 = 1024;
pub const LB_ADDFILE: i32 = 406;
pub const LB_ADDSTRING: i32 = 384;
pub const LB_CTLCODE: i32 = 0;
pub const LB_DELETESTRING: i32 = 386;
pub const LB_DIR: i32 = 397;
pub const LB_ERR: i32 = -1;
pub const LB_ERRSPACE: i32 = -2;
pub const LB_FINDSTRING: i32 = 399;
pub const LB_FINDSTRINGEXACT: i32 = 418;
pub const LB_GETANCHORINDEX: i32 = 413;
pub const LB_GETCARETINDEX: i32 = 415;
pub const LB_GETCOUNT: i32 = 395;
pub const LB_GETCURSEL: i32 = 392;
pub const LB_GETHORIZONTALEXTENT: i32 = 403;
pub const LB_GETITEMDATA: i32 = 409;
pub const LB_GETITEMHEIGHT: i32 = 417;
pub const LB_GETITEMRECT: i32 = 408;
pub const LB_GETLISTBOXINFO: i32 = 434;
pub const LB_GETLOCALE: i32 = 422;
pub const LB_GETSEL: i32 = 391;
pub const LB_GETSELCOUNT: i32 = 400;
pub const LB_GETSELITEMS: i32 = 401;
pub const LB_GETTEXT: i32 = 393;
pub const LB_GETTEXTLEN: i32 = 394;
pub const LB_GETTOPINDEX: i32 = 398;
pub const LB_INITSTORAGE: i32 = 424;
pub const LB_INSERTSTRING: i32 = 385;
pub const LB_ITEMFROMPOINT: i32 = 425;
pub const LB_MSGMAX: i32 = 435;
pub const LB_OKAY: i32 = 0;
pub const LB_RESETCONTENT: i32 = 388;
pub const LB_SELECTSTRING: i32 = 396;
pub const LB_SELITEMRANGE: i32 = 411;
pub const LB_SELITEMRANGEEX: i32 = 387;
pub const LB_SETANCHORINDEX: i32 = 412;
pub const LB_SETCARETINDEX: i32 = 414;
pub const LB_SETCOLUMNWIDTH: i32 = 405;
pub const LB_SETCOUNT: i32 = 423;
pub const LB_SETCURSEL: i32 = 390;
pub const LB_SETHORIZONTALEXTENT: i32 = 404;
pub const LB_SETITEMDATA: i32 = 410;
pub const LB_SETITEMHEIGHT: i32 = 416;
pub const LB_SETLOCALE: i32 = 421;
pub const LB_SETSEL: i32 = 389;
pub const LB_SETTABSTOPS: i32 = 402;
pub const LB_SETTOPINDEX: i32 = 407;
pub type LEGACY_TOUCHPAD_FEATURES = u32;
pub const LEGACY_TOUCHPAD_FEATURE_ENABLE_DISABLE: LEGACY_TOUCHPAD_FEATURES = 1;
pub const LEGACY_TOUCHPAD_FEATURE_NONE: LEGACY_TOUCHPAD_FEATURES = 0;
pub const LEGACY_TOUCHPAD_FEATURE_REVERSE_SCROLL_DIRECTION: LEGACY_TOUCHPAD_FEATURES = 4;
pub const LLKHF_ALTDOWN: i32 = 32;
pub const LLKHF_EXTENDED: i32 = 1;
pub const LLKHF_INJECTED: i32 = 16;
pub const LLKHF_LOWER_IL_INJECTED: i32 = 2;
pub const LLKHF_UP: i32 = 128;
pub const LLMHF_INJECTED: i32 = 1;
pub const LLMHF_LOWER_IL_INJECTED: i32 = 2;
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMENUTEMPLATEA(pub *mut core::ffi::c_void);
impl Default for LPMENUTEMPLATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMENUTEMPLATEW(pub *mut core::ffi::c_void);
impl Default for LPMENUTEMPLATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
pub const LR_COLOR: i32 = 2;
pub const LR_COPYDELETEORG: i32 = 8;
pub const LR_COPYFROMRESOURCE: i32 = 16384;
pub const LR_COPYRETURNORG: i32 = 4;
pub const LR_CREATEDIBSECTION: i32 = 8192;
pub const LR_DEFAULTCOLOR: i32 = 0;
pub const LR_DEFAULTSIZE: i32 = 64;
pub const LR_LOADFROMFILE: i32 = 16;
pub const LR_LOADMAP3DCOLORS: i32 = 4096;
pub const LR_LOADTRANSPARENT: i32 = 32;
pub const LR_MONOCHROME: i32 = 1;
pub const LR_SHARED: i32 = 32768;
pub const LR_VGACOLOR: i32 = 128;
pub const LSFW_LOCK: i32 = 1;
pub const LSFW_UNLOCK: i32 = 2;
pub const LWA_ALPHA: i32 = 2;
pub const LWA_COLORKEY: i32 = 1;
pub const MAPVK_VK_TO_CHAR: i32 = 2;
pub const MAPVK_VK_TO_VSC: i32 = 0;
pub const MAPVK_VK_TO_VSC_EX: i32 = 4;
pub const MAPVK_VSC_TO_VK: i32 = 1;
pub const MAPVK_VSC_TO_VK_EX: i32 = 3;
pub const MAX_LOGICALDPIOVERRIDE: i32 = 2;
pub const MAX_STR_BLOCKREASON: i32 = 256;
pub const MAX_TOUCH_COUNT: i32 = 256;
pub const MAX_TOUCH_PREDICTION_FILTER_TAPS: i32 = 3;
pub const MA_ACTIVATE: i32 = 1;
pub const MA_ACTIVATEANDEAT: i32 = 2;
pub const MA_NOACTIVATE: i32 = 3;
pub const MA_NOACTIVATEANDEAT: i32 = 4;
pub const MB_ABORTRETRYIGNORE: i32 = 2;
pub const MB_APPLMODAL: i32 = 0;
pub const MB_CANCELTRYCONTINUE: i32 = 6;
pub const MB_DEFAULT_DESKTOP_ONLY: i32 = 131072;
pub const MB_DEFBUTTON1: i32 = 0;
pub const MB_DEFBUTTON2: i32 = 256;
pub const MB_DEFBUTTON3: i32 = 512;
pub const MB_DEFBUTTON4: i32 = 768;
pub const MB_DEFMASK: i32 = 3840;
pub const MB_HELP: i32 = 16384;
pub const MB_ICONASTERISK: i32 = 64;
pub const MB_ICONERROR: i32 = 16;
pub const MB_ICONEXCLAMATION: i32 = 48;
pub const MB_ICONHAND: i32 = 16;
pub const MB_ICONINFORMATION: i32 = 64;
pub const MB_ICONMASK: i32 = 240;
pub const MB_ICONQUESTION: i32 = 32;
pub const MB_ICONSTOP: i32 = 16;
pub const MB_ICONWARNING: i32 = 48;
pub const MB_MISCMASK: i32 = 49152;
pub const MB_MODEMASK: i32 = 12288;
pub const MB_NOFOCUS: i32 = 32768;
pub const MB_OK: i32 = 0;
pub const MB_OKCANCEL: i32 = 1;
pub const MB_RETRYCANCEL: i32 = 5;
pub const MB_RIGHT: i32 = 524288;
pub const MB_RTLREADING: i32 = 1048576;
pub const MB_SERVICE_NOTIFICATION: i32 = 2097152;
pub const MB_SERVICE_NOTIFICATION_NT3X: i32 = 262144;
pub const MB_SETFOREGROUND: i32 = 65536;
pub const MB_SYSTEMMODAL: i32 = 4096;
pub const MB_TASKMODAL: i32 = 8192;
pub const MB_TOPMOST: i32 = 262144;
pub const MB_TYPEMASK: i32 = 15;
pub const MB_USERICON: i32 = 128;
pub const MB_YESNO: i32 = 4;
pub const MB_YESNOCANCEL: i32 = 3;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type MDICREATESTRUCT = MDICREATESTRUCTA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MDICREATESTRUCTA {
    pub szClass: windows_core::PCSTR,
    pub szTitle: windows_core::PCSTR,
    pub hOwner: super::HANDLE,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub style: u32,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MDICREATESTRUCTW {
    pub szClass: windows_core::PCWSTR,
    pub szTitle: windows_core::PCWSTR,
    pub hOwner: super::HANDLE,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub style: u32,
    pub lParam: super::LPARAM,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MDINEXTMENU {
    pub hmenuIn: super::HMENU,
    pub hmenuNext: super::HMENU,
    pub hwndNext: super::HWND,
}
pub const MDIS_ALLCHILDSTYLES: i32 = 1;
pub const MDITILE_HORIZONTAL: i32 = 1;
pub const MDITILE_SKIPDISABLED: i32 = 2;
pub const MDITILE_VERTICAL: i32 = 0;
pub const MDITILE_ZORDER: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MENUBARINFO {
    pub cbSize: u32,
    pub rcBar: super::RECT,
    pub hMenu: super::HMENU,
    pub hwndMenu: super::HWND,
    pub _bitfield: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub type MENUITEMINFO = MENUITEMINFOA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub dwTypeData: windows_core::PSTR,
    pub cch: u32,
    pub hbmpItem: super::HBITMAP,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub dwTypeData: windows_core::PWSTR,
    pub cch: u32,
    pub hbmpItem: super::HBITMAP,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MENUITEMTEMPLATEHEADER {
    pub versionNumber: u16,
    pub offset: u16,
}
pub type MENUTEMPLATE = MENUTEMPLATEA;
pub type MENUTEMPLATEA = core::ffi::c_void;
pub type MENUTEMPLATEW = core::ffi::c_void;
pub const MENU_ALL_ACCESS: i32 = 2035711;
pub const MENU_CHECK_ITEM: i32 = 256;
pub const MENU_DELETE_MENU: i32 = 32;
pub const MENU_ENABLE_ITEM: i32 = 128;
pub const MENU_EXECUTE_ACCESS: i32 = 131072;
pub const MENU_GET_ITEM_DATA: i32 = 2;
pub const MENU_GET_ITEM_INFO: i32 = 1;
pub const MENU_GET_SUBMENU: i32 = 4;
pub const MENU_INSERT_ITEM: i32 = 16;
pub const MENU_INSERT_MENU: i32 = 8;
pub const MENU_READ_ACCESS: i32 = 131079;
pub const MENU_SET_DEFAULT_ITEM: i32 = 512;
pub const MENU_SET_ITEM_DATA: i32 = 1024;
pub const MENU_SET_ITEM_INFO: i32 = 64;
pub const MENU_SET_SUBMENU: i32 = 2048;
pub const MENU_WRITE_ACCESS: i32 = 135160;
pub const METRICS_USEDEFAULT: i32 = -1;
pub const MFS_CHECKED: i32 = 8;
pub const MFS_DEFAULT: i32 = 4096;
pub const MFS_DISABLED: i32 = 3;
pub const MFS_ENABLED: i32 = 0;
pub const MFS_GRAYED: i32 = 3;
pub const MFS_HILITE: i32 = 128;
pub const MFS_UNCHECKED: i32 = 0;
pub const MFS_UNHILITE: i32 = 0;
pub const MFT_BITMAP: i32 = 4;
pub const MFT_MENUBARBREAK: i32 = 32;
pub const MFT_MENUBREAK: i32 = 64;
pub const MFT_OWNERDRAW: i32 = 256;
pub const MFT_RADIOCHECK: i32 = 512;
pub const MFT_RIGHTJUSTIFY: i32 = 16384;
pub const MFT_RIGHTORDER: i32 = 8192;
pub const MFT_SEPARATOR: i32 = 2048;
pub const MFT_STRING: i32 = 0;
pub const MF_APPEND: i32 = 256;
pub const MF_BITMAP: i32 = 4;
pub const MF_BYCOMMAND: i32 = 0;
pub const MF_BYPOSITION: i32 = 1024;
pub const MF_CHANGE: i32 = 128;
pub const MF_CHECKED: i32 = 8;
pub const MF_DEFAULT: i32 = 4096;
pub const MF_DELETE: i32 = 512;
pub const MF_DISABLED: i32 = 2;
pub const MF_ENABLED: i32 = 0;
pub const MF_END: i32 = 128;
pub const MF_GRAYED: i32 = 1;
pub const MF_HELP: i32 = 16384;
pub const MF_HILITE: i32 = 128;
pub const MF_INSERT: i32 = 0;
pub const MF_MENUBARBREAK: i32 = 32;
pub const MF_MENUBREAK: i32 = 64;
pub const MF_MOUSESELECT: i32 = 32768;
pub const MF_OWNERDRAW: i32 = 256;
pub const MF_POPUP: i32 = 16;
pub const MF_REMOVE: i32 = 4096;
pub const MF_RIGHTJUSTIFY: i32 = 16384;
pub const MF_SEPARATOR: i32 = 2048;
pub const MF_STRING: i32 = 0;
pub const MF_SYSMENU: i32 = 8192;
pub const MF_UNCHECKED: i32 = 0;
pub const MF_UNHILITE: i32 = 0;
pub const MF_USECHECKBITMAPS: i32 = 512;
pub const MIIM_BITMAP: i32 = 128;
pub const MIIM_CHECKMARKS: i32 = 8;
pub const MIIM_DATA: i32 = 32;
pub const MIIM_FTYPE: i32 = 256;
pub const MIIM_ID: i32 = 2;
pub const MIIM_STATE: i32 = 1;
pub const MIIM_STRING: i32 = 64;
pub const MIIM_SUBMENU: i32 = 4;
pub const MIIM_TYPE: i32 = 16;
pub const MIM_APPLYTOSUBMENUS: u32 = 2147483648;
pub const MIM_BACKGROUND: i32 = 2;
pub const MIM_HELPID: i32 = 4;
pub const MIM_MAXHEIGHT: i32 = 1;
pub const MIM_MENUDATA: i32 = 8;
pub const MIM_STYLE: i32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MINIMIZEDMETRICS {
    pub cbSize: u32,
    pub iWidth: i32,
    pub iHorzGap: i32,
    pub iVertGap: i32,
    pub iArrange: i32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MINMAXINFO {
    pub ptReserved: super::POINT,
    pub ptMaxSize: super::POINT,
    pub ptMaxPosition: super::POINT,
    pub ptMinTrackSize: super::POINT,
    pub ptMaxTrackSize: super::POINT,
}
pub const MIN_LOGICALDPIOVERRIDE: i32 = -2;
pub const MKF_AVAILABLE: i32 = 2;
pub const MKF_CONFIRMHOTKEY: i32 = 8;
pub const MKF_HOTKEYACTIVE: i32 = 4;
pub const MKF_HOTKEYSOUND: i32 = 16;
pub const MKF_INDICATOR: i32 = 32;
pub const MKF_LEFTBUTTONDOWN: i32 = 16777216;
pub const MKF_LEFTBUTTONSEL: i32 = 268435456;
pub const MKF_MODIFIERS: i32 = 64;
pub const MKF_MOUSEKEYSON: i32 = 1;
pub const MKF_MOUSEMODE: u32 = 2147483648;
pub const MKF_REPLACENUMBERS: i32 = 128;
pub const MKF_RIGHTBUTTONDOWN: i32 = 33554432;
pub const MKF_RIGHTBUTTONSEL: i32 = 536870912;
pub const MK_CONTROL: i32 = 8;
pub const MK_LBUTTON: i32 = 1;
pub const MK_MBUTTON: i32 = 16;
pub const MK_RBUTTON: i32 = 2;
pub const MK_SHIFT: i32 = 4;
pub const MK_XBUTTON1: i32 = 32;
pub const MK_XBUTTON2: i32 = 64;
pub const MNC_CLOSE: i32 = 1;
pub const MNC_EXECUTE: i32 = 2;
pub const MNC_IGNORE: i32 = 0;
pub const MNC_SELECT: i32 = 3;
pub const MND_CONTINUE: i32 = 0;
pub const MND_ENDMENU: i32 = 1;
pub const MNGOF_BOTTOMGAP: i32 = 2;
pub const MNGOF_TOPGAP: i32 = 1;
pub const MNGO_NOERROR: i32 = 1;
pub const MNGO_NOINTERFACE: i32 = 0;
pub const MNS_AUTODISMISS: i32 = 268435456;
pub const MNS_CHECKORBMP: i32 = 67108864;
pub const MNS_DRAGDROP: i32 = 536870912;
pub const MNS_MODELESS: i32 = 1073741824;
pub const MNS_NOCHECK: u32 = 2147483648;
pub const MNS_NOTIFYBYPOS: i32 = 134217728;
pub const MN_GETHMENU: i32 = 481;
pub const MOD_ALT: i32 = 1;
pub const MOD_CONTROL: i32 = 2;
pub const MOD_NOREPEAT: i32 = 16384;
pub const MOD_SHIFT: i32 = 4;
pub const MOD_WIN: i32 = 8;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type MONITORENUMPROC = Option<unsafe extern "system" fn(param0: super::HMONITOR, param1: super::HDC, param2: *mut super::RECT, param3: super::LPARAM) -> windows_core::BOOL>;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const MONITORINFOF_PRIMARY: i32 = 1;
pub const MONITOR_DEFAULTTONEAREST: i32 = 2;
pub const MONITOR_DEFAULTTONULL: i32 = 0;
pub const MONITOR_DEFAULTTOPRIMARY: i32 = 1;
pub const MOUSEEVENTF_ABSOLUTE: i32 = 32768;
pub const MOUSEEVENTF_HWHEEL: i32 = 4096;
pub const MOUSEEVENTF_LEFTDOWN: i32 = 2;
pub const MOUSEEVENTF_LEFTUP: i32 = 4;
pub const MOUSEEVENTF_MIDDLEDOWN: i32 = 32;
pub const MOUSEEVENTF_MIDDLEUP: i32 = 64;
pub const MOUSEEVENTF_MOVE: i32 = 1;
pub const MOUSEEVENTF_MOVE_NOCOALESCE: i32 = 8192;
pub const MOUSEEVENTF_RIGHTDOWN: i32 = 8;
pub const MOUSEEVENTF_RIGHTUP: i32 = 16;
pub const MOUSEEVENTF_VIRTUALDESK: i32 = 16384;
pub const MOUSEEVENTF_WHEEL: i32 = 2048;
pub const MOUSEEVENTF_XDOWN: i32 = 128;
pub const MOUSEEVENTF_XUP: i32 = 256;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOUSEHOOKSTRUCT {
    pub pt: super::POINT,
    pub hwnd: super::HWND,
    pub wHitTestCode: u32,
    pub dwExtraInfo: usize,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOUSEHOOKSTRUCTEX {
    pub Base: MOUSEHOOKSTRUCT,
    pub mouseData: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOUSEINPUT {
    pub dx: i32,
    pub dy: i32,
    pub mouseData: u32,
    pub dwFlags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOUSEMOVEPOINT {
    pub x: i32,
    pub y: i32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
pub const MOUSEWHEEL_ROUTING_FOCUS: i32 = 0;
pub const MOUSEWHEEL_ROUTING_HYBRID: i32 = 1;
pub const MOUSEWHEEL_ROUTING_MOUSE_POS: i32 = 2;
pub const MOUSE_ATTRIBUTES_CHANGED: i32 = 4;
pub const MOUSE_MOVE_ABSOLUTE: i32 = 1;
pub const MOUSE_MOVE_NOCOALESCE: i32 = 8;
pub const MOUSE_MOVE_RELATIVE: i32 = 0;
pub const MOUSE_VIRTUAL_DESKTOP: i32 = 2;
pub type MOVESIZE_OPERATION = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSG {
    pub hwnd: super::HWND,
    pub message: u32,
    pub wParam: super::WPARAM,
    pub lParam: super::LPARAM,
    pub time: u32,
    pub pt: super::POINT,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type MSGBOXCALLBACK = Option<unsafe extern "system" fn(lphelpinfo: *mut HELPINFO)>;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type MSGBOXPARAMS = MSGBOXPARAMSA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct MSGBOXPARAMSA {
    pub cbSize: u32,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub lpszText: windows_core::PCSTR,
    pub lpszCaption: windows_core::PCSTR,
    pub dwStyle: u32,
    pub lpszIcon: windows_core::PCSTR,
    pub dwContextHelpId: usize,
    pub lpfnMsgBoxCallback: MSGBOXCALLBACK,
    pub dwLanguageId: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct MSGBOXPARAMSW {
    pub cbSize: u32,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub lpszText: windows_core::PCWSTR,
    pub lpszCaption: windows_core::PCWSTR,
    pub dwStyle: u32,
    pub lpszIcon: windows_core::PCWSTR,
    pub dwContextHelpId: usize,
    pub lpfnMsgBoxCallback: MSGBOXCALLBACK,
    pub dwLanguageId: u32,
}
pub const MSGFLTINFO_ALLOWED_HIGHER: i32 = 3;
pub const MSGFLTINFO_ALREADYALLOWED_FORWND: i32 = 1;
pub const MSGFLTINFO_ALREADYDISALLOWED_FORWND: i32 = 2;
pub const MSGFLTINFO_NONE: i32 = 0;
pub const MSGFLT_ADD: i32 = 1;
pub const MSGFLT_ALLOW: i32 = 1;
pub const MSGFLT_DISALLOW: i32 = 2;
pub const MSGFLT_REMOVE: i32 = 2;
pub const MSGFLT_RESET: i32 = 0;
pub const MSGF_DIALOGBOX: i32 = 0;
pub const MSGF_MAX: i32 = 8;
pub const MSGF_MENU: i32 = 2;
pub const MSGF_MESSAGEBOX: i32 = 1;
pub const MSGF_NEXTWINDOW: i32 = 6;
pub const MSGF_SCROLLBAR: i32 = 5;
pub const MSGF_USER: i32 = 4096;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const MWMO_ALERTABLE: i32 = 2;
pub const MWMO_INPUTAVAILABLE: i32 = 4;
pub const MWMO_WAITALL: i32 = 1;
#[cfg(feature = "minwindef")]
pub type NAMEENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: super::LPARAM) -> windows_core::BOOL>;
#[cfg(feature = "minwindef")]
pub type NAMEENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::LPARAM) -> windows_core::BOOL>;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const NFR_ANSI: i32 = 1;
pub const NFR_UNICODE: i32 = 2;
pub const NF_QUERY: i32 = 3;
pub const NF_REQUERY: i32 = 4;
pub const NID_EXTERNAL_PEN: i32 = 8;
pub const NID_EXTERNAL_TOUCH: i32 = 2;
pub const NID_INTEGRATED_PEN: i32 = 4;
pub const NID_INTEGRATED_TOUCH: i32 = 1;
pub const NID_MULTI_INPUT: i32 = 64;
pub const NID_READY: i32 = 128;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMHDR {
    pub hwndFrom: super::HWND,
    pub idFrom: usize,
    pub code: u32,
}
#[cfg(feature = "wingdi")]
pub type NONCLIENTMETRICS = NONCLIENTMETRICSA;
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const ODA_DRAWENTIRE: i32 = 1;
pub const ODA_FOCUS: i32 = 4;
pub const ODA_SELECT: i32 = 2;
pub const ODS_CHECKED: i32 = 8;
pub const ODS_COMBOBOXEDIT: i32 = 4096;
pub const ODS_DEFAULT: i32 = 32;
pub const ODS_DISABLED: i32 = 4;
pub const ODS_FOCUS: i32 = 16;
pub const ODS_GRAYED: i32 = 2;
pub const ODS_HOTLIGHT: i32 = 64;
pub const ODS_INACTIVE: i32 = 128;
pub const ODS_NOACCEL: i32 = 256;
pub const ODS_NOFOCUSRECT: i32 = 512;
pub const ODS_SELECTED: i32 = 1;
pub const ODT_BUTTON: i32 = 4;
pub const ODT_COMBOBOX: i32 = 3;
pub const ODT_LISTBOX: i32 = 2;
pub const ODT_MENU: i32 = 1;
pub const ODT_STATIC: i32 = 5;
pub const ORD_LANGDRIVER: i32 = 1;
pub type ORIENTATION_PREFERENCE = u32;
pub const ORIENTATION_PREFERENCE_LANDSCAPE: ORIENTATION_PREFERENCE = 1;
pub const ORIENTATION_PREFERENCE_LANDSCAPE_FLIPPED: ORIENTATION_PREFERENCE = 4;
pub const ORIENTATION_PREFERENCE_NONE: ORIENTATION_PREFERENCE = 0;
pub const ORIENTATION_PREFERENCE_PORTRAIT: ORIENTATION_PREFERENCE = 2;
pub const ORIENTATION_PREFERENCE_PORTRAIT_FLIPPED: ORIENTATION_PREFERENCE = 8;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PAINTSTRUCT {
    pub hdc: super::HDC,
    pub fErase: windows_core::BOOL,
    pub rcPaint: super::RECT,
    pub fRestore: windows_core::BOOL,
    pub fIncUpdate: windows_core::BOOL,
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
pub const PA_ACTIVATE: i32 = 1;
pub const PA_NOACTIVATE: i32 = 3;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PBSMINFO = *mut BSMINFO;
pub const PBTF_APMRESUMEFROMFAILURE: i32 = 1;
pub const PBT_APMBATTERYLOW: i32 = 9;
pub const PBT_APMOEMEVENT: i32 = 11;
pub const PBT_APMPOWERSTATUSCHANGE: i32 = 10;
pub const PBT_APMQUERYSTANDBY: i32 = 1;
pub const PBT_APMQUERYSTANDBYFAILED: i32 = 3;
pub const PBT_APMQUERYSUSPEND: i32 = 0;
pub const PBT_APMQUERYSUSPENDFAILED: i32 = 2;
pub const PBT_APMRESUMEAUTOMATIC: i32 = 18;
pub const PBT_APMRESUMECRITICAL: i32 = 6;
pub const PBT_APMRESUMESTANDBY: i32 = 8;
pub const PBT_APMRESUMESUSPEND: i32 = 7;
pub const PBT_APMSTANDBY: i32 = 5;
pub const PBT_APMSUSPEND: i32 = 4;
pub const PBT_POWERSETTINGCHANGE: i32 = 32787;
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
pub const PDC_ARRIVAL: i32 = 1;
pub const PDC_MAPPING_CHANGE: i32 = 256;
pub const PDC_MODE_ASPECTRATIOPRESERVED: i32 = 2048;
pub const PDC_MODE_CENTERED: i32 = 128;
pub const PDC_MODE_DEFAULT: i32 = 64;
pub const PDC_ORIENTATION_0: i32 = 4;
pub const PDC_ORIENTATION_180: i32 = 16;
pub const PDC_ORIENTATION_270: i32 = 32;
pub const PDC_ORIENTATION_90: i32 = 8;
pub const PDC_ORIGIN: i32 = 1024;
pub const PDC_REMOVAL: i32 = 2;
pub const PDC_RESOLUTION: i32 = 512;
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
pub const PENARBITRATIONTYPE_FIS: i32 = 2;
pub const PENARBITRATIONTYPE_MAX: i32 = 4;
pub const PENARBITRATIONTYPE_NONE: i32 = 0;
pub const PENARBITRATIONTYPE_SPT: i32 = 3;
pub const PENARBITRATIONTYPE_WIN8: i32 = 1;
pub const PENVISUALIZATION_CURSOR: i32 = 32;
pub const PENVISUALIZATION_DOUBLETAP: i32 = 2;
pub const PENVISUALIZATION_OFF: i32 = 0;
pub const PENVISUALIZATION_ON: i32 = 35;
pub const PENVISUALIZATION_TAP: i32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PEN_FLAGS(pub u32);
pub const PEN_FLAG_BARREL: i32 = 1;
pub const PEN_FLAG_ERASER: i32 = 4;
pub const PEN_FLAG_INVERTED: i32 = 2;
pub const PEN_FLAG_NONE: i32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PEN_MASK(pub u32);
pub const PEN_MASK_NONE: i32 = 0;
pub const PEN_MASK_PRESSURE: i32 = 1;
pub const PEN_MASK_ROTATION: i32 = 2;
pub const PEN_MASK_TILT_X: i32 = 4;
pub const PEN_MASK_TILT_Y: i32 = 8;
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
pub const PMB_ACTIVE: i32 = 1;
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
pub const PM_NOREMOVE: i32 = 0;
pub const PM_NOYIELD: i32 = 2;
pub const PM_QS_INPUT: i32 = 470220800;
pub const PM_QS_PAINT: i32 = 2097152;
pub const PM_QS_POSTMESSAGE: i32 = 9961472;
pub const PM_QS_SENDMESSAGE: i32 = 4194304;
pub const PM_REMOVE: i32 = 1;
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const POINTER_DEVICE_PRODUCT_STRING_MAX: i32 = 520;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct POINTER_FLAGS(pub u32);
pub const POINTER_FLAG_CANCELED: i32 = 32768;
pub const POINTER_FLAG_CAPTURECHANGED: i32 = 2097152;
pub const POINTER_FLAG_CONFIDENCE: i32 = 16384;
pub const POINTER_FLAG_DOWN: i32 = 65536;
pub const POINTER_FLAG_FIFTHBUTTON: i32 = 256;
pub const POINTER_FLAG_FIRSTBUTTON: i32 = 16;
pub const POINTER_FLAG_FOURTHBUTTON: i32 = 128;
pub const POINTER_FLAG_HASTRANSFORM: i32 = 4194304;
pub const POINTER_FLAG_HWHEEL: i32 = 1048576;
pub const POINTER_FLAG_INCONTACT: i32 = 4;
pub const POINTER_FLAG_INRANGE: i32 = 2;
pub const POINTER_FLAG_NEW: i32 = 1;
pub const POINTER_FLAG_NONE: i32 = 0;
pub const POINTER_FLAG_PRIMARY: i32 = 8192;
pub const POINTER_FLAG_SECONDBUTTON: i32 = 32;
pub const POINTER_FLAG_THIRDBUTTON: i32 = 64;
pub const POINTER_FLAG_UP: i32 = 262144;
pub const POINTER_FLAG_UPDATE: i32 = 131072;
pub const POINTER_FLAG_WHEEL: i32 = 524288;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct POINTER_INPUT_TYPE(pub u32);
pub const POINTER_MESSAGE_FLAG_CANCELED: i32 = 32768;
pub const POINTER_MESSAGE_FLAG_CONFIDENCE: i32 = 16384;
pub const POINTER_MESSAGE_FLAG_FIFTHBUTTON: i32 = 256;
pub const POINTER_MESSAGE_FLAG_FIRSTBUTTON: i32 = 16;
pub const POINTER_MESSAGE_FLAG_FOURTHBUTTON: i32 = 128;
pub const POINTER_MESSAGE_FLAG_INCONTACT: i32 = 4;
pub const POINTER_MESSAGE_FLAG_INRANGE: i32 = 2;
pub const POINTER_MESSAGE_FLAG_NEW: i32 = 1;
pub const POINTER_MESSAGE_FLAG_PRIMARY: i32 = 8192;
pub const POINTER_MESSAGE_FLAG_SECONDBUTTON: i32 = 32;
pub const POINTER_MESSAGE_FLAG_THIRDBUTTON: i32 = 64;
pub const POINTER_MOD_CTRL: i32 = 8;
pub const POINTER_MOD_SHIFT: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POWERBROADCAST_SETTING {
    pub PowerSetting: windows_core::GUID,
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
pub type PREGISTERCLASSNAMEW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR) -> bool>;
pub const PRF_CHECKVISIBLE: i32 = 1;
pub const PRF_CHILDREN: i32 = 16;
pub const PRF_CLIENT: i32 = 4;
pub const PRF_ERASEBKGND: i32 = 8;
pub const PRF_NONCLIENT: i32 = 2;
pub const PRF_OWNED: i32 = 32;
pub type PRID_DEVICE_INFO = *mut RID_DEVICE_INFO;
pub type PRID_DEVICE_INFO_HID = *mut RID_DEVICE_INFO_HID;
pub type PRID_DEVICE_INFO_KEYBOARD = *mut RID_DEVICE_INFO_KEYBOARD;
pub type PRID_DEVICE_INFO_MOUSE = *mut RID_DEVICE_INFO_MOUSE;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PROPENUMPROC = PROPENUMPROCA;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PROPENUMPROCA = Option<unsafe extern "system" fn(param0: super::HWND, param1: windows_core::PCSTR, param2: super::HANDLE) -> windows_core::BOOL>;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PROPENUMPROCEX = PROPENUMPROCEXA;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PROPENUMPROCEXA = Option<unsafe extern "system" fn(param0: super::HWND, param1: windows_core::PCSTR, param2: super::HANDLE, param3: usize) -> windows_core::BOOL>;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PROPENUMPROCEXW = Option<unsafe extern "system" fn(param0: super::HWND, param1: windows_core::PCWSTR, param2: super::HANDLE, param3: usize) -> windows_core::BOOL>;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PROPENUMPROCW = Option<unsafe extern "system" fn(param0: super::HWND, param1: windows_core::PCWSTR, param2: super::HANDLE) -> windows_core::BOOL>;
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
pub const PWR_CRITICALRESUME: i32 = 3;
pub const PWR_FAIL: i32 = -1;
pub const PWR_OK: i32 = 1;
pub const PWR_SUSPENDREQUEST: i32 = 1;
pub const PWR_SUSPENDRESUME: i32 = 2;
pub type PWTSSESSION_NOTIFICATION = *mut WTSSESSION_NOTIFICATION;
pub const PW_CLIENTONLY: i32 = 1;
pub const PW_RENDERFULLCONTENT: i32 = 2;
pub const QS_ALLEVENTS: i32 = 7359;
pub const QS_ALLINPUT: i32 = 7423;
pub const QS_ALLPOSTMESSAGE: i32 = 256;
pub const QS_HOTKEY: i32 = 128;
pub const QS_INPUT: i32 = 7175;
pub const QS_KEY: i32 = 1;
pub const QS_MOUSE: i32 = 6;
pub const QS_MOUSEBUTTON: i32 = 4;
pub const QS_MOUSEMOVE: i32 = 2;
pub const QS_PAINT: i32 = 32;
pub const QS_POINTER: i32 = 4096;
pub const QS_POSTMESSAGE: i32 = 8;
pub const QS_RAWINPUT: i32 = 1024;
pub const QS_SENDMESSAGE: i32 = 64;
pub const QS_TIMER: i32 = 16;
pub const QS_TOUCH: i32 = 2048;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RAWINPUTDEVICE {
    pub usUsagePage: u16,
    pub usUsage: u16,
    pub dwFlags: u32,
    pub hwndTarget: super::HWND,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RAWINPUTDEVICELIST {
    pub hDevice: super::HANDLE,
    pub dwType: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RAWINPUTHEADER {
    pub dwType: u32,
    pub dwSize: u32,
    pub hDevice: super::HANDLE,
    pub wParam: super::WPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RAWMOUSE_0_0 {
    pub usButtonFlags: u16,
    pub usButtonData: u16,
}
pub const RDW_ALLCHILDREN: i32 = 128;
pub const RDW_ERASE: i32 = 4;
pub const RDW_ERASENOW: i32 = 512;
pub const RDW_FRAME: i32 = 1024;
pub const RDW_INTERNALPAINT: i32 = 2;
pub const RDW_INVALIDATE: i32 = 1;
pub const RDW_NOCHILDREN: i32 = 64;
pub const RDW_NOERASE: i32 = 32;
pub const RDW_NOFRAME: i32 = 2048;
pub const RDW_NOINTERNALPAINT: i32 = 16;
pub const RDW_UPDATENOW: i32 = 256;
pub const RDW_VALIDATE: i32 = 8;
pub const RES_CURSOR: i32 = 2;
pub const RES_ICON: i32 = 1;
pub const RIDEV_APPKEYS: i32 = 1024;
pub const RIDEV_CAPTUREMOUSE: i32 = 512;
pub const RIDEV_DEVNOTIFY: i32 = 8192;
pub const RIDEV_EXCLUDE: i32 = 16;
pub const RIDEV_EXINPUTSINK: i32 = 4096;
pub const RIDEV_EXMODEMASK: i32 = 240;
pub const RIDEV_INPUTSINK: i32 = 256;
pub const RIDEV_NOHOTKEYS: i32 = 512;
pub const RIDEV_NOLEGACY: i32 = 48;
pub const RIDEV_PAGEONLY: i32 = 32;
pub const RIDEV_REMOVE: i32 = 1;
pub const RIDI_DEVICEINFO: i32 = 536870923;
pub const RIDI_DEVICENAME: i32 = 536870919;
pub const RIDI_PREPARSEDDATA: i32 = 536870917;
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RID_DEVICE_INFO_HID {
    pub dwVendorId: u32,
    pub dwProductId: u32,
    pub dwVersionNumber: u32,
    pub usUsagePage: u16,
    pub usUsage: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RID_DEVICE_INFO_KEYBOARD {
    pub dwType: u32,
    pub dwSubType: u32,
    pub dwKeyboardMode: u32,
    pub dwNumberOfFunctionKeys: u32,
    pub dwNumberOfIndicators: u32,
    pub dwNumberOfKeysTotal: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RID_DEVICE_INFO_MOUSE {
    pub dwId: u32,
    pub dwNumberOfButtons: u32,
    pub dwSampleRate: u32,
    pub fHasHorizontalWheel: windows_core::BOOL,
}
pub const RID_HEADER: i32 = 268435461;
pub const RID_INPUT: i32 = 268435459;
pub const RIM_INPUT: i32 = 0;
pub const RIM_INPUTSINK: i32 = 1;
pub const RIM_TYPEHID: i32 = 2;
pub const RIM_TYPEKEYBOARD: i32 = 1;
pub const RIM_TYPEMAX: i32 = 2;
pub const RIM_TYPEMOUSE: i32 = 0;
pub const RI_KEY_BREAK: i32 = 1;
pub const RI_KEY_E0: i32 = 2;
pub const RI_KEY_E1: i32 = 4;
pub const RI_KEY_MAKE: i32 = 0;
pub const RI_KEY_TERMSRV_SET_LED: i32 = 8;
pub const RI_KEY_TERMSRV_SHADOW: i32 = 16;
pub const RI_MOUSE_BUTTON_1_DOWN: i32 = 1;
pub const RI_MOUSE_BUTTON_1_UP: i32 = 2;
pub const RI_MOUSE_BUTTON_2_DOWN: i32 = 4;
pub const RI_MOUSE_BUTTON_2_UP: i32 = 8;
pub const RI_MOUSE_BUTTON_3_DOWN: i32 = 16;
pub const RI_MOUSE_BUTTON_3_UP: i32 = 32;
pub const RI_MOUSE_BUTTON_4_DOWN: i32 = 64;
pub const RI_MOUSE_BUTTON_4_UP: i32 = 128;
pub const RI_MOUSE_BUTTON_5_DOWN: i32 = 256;
pub const RI_MOUSE_BUTTON_5_UP: i32 = 512;
pub const RI_MOUSE_HWHEEL: i32 = 2048;
pub const RI_MOUSE_LEFT_BUTTON_DOWN: i32 = 1;
pub const RI_MOUSE_LEFT_BUTTON_UP: i32 = 2;
pub const RI_MOUSE_MIDDLE_BUTTON_DOWN: i32 = 16;
pub const RI_MOUSE_MIDDLE_BUTTON_UP: i32 = 32;
pub const RI_MOUSE_RIGHT_BUTTON_DOWN: i32 = 4;
pub const RI_MOUSE_RIGHT_BUTTON_UP: i32 = 8;
pub const RI_MOUSE_WHEEL: i32 = 1024;
pub const RT_ACCELERATOR: windows_core::PCWSTR = windows_core::PCWSTR(9 as _);
pub const RT_ANICURSOR: windows_core::PCWSTR = windows_core::PCWSTR(21 as _);
pub const RT_ANIICON: windows_core::PCWSTR = windows_core::PCWSTR(22 as _);
pub const RT_BITMAP: windows_core::PCWSTR = windows_core::PCWSTR(2 as _);
pub const RT_CURSOR: windows_core::PCWSTR = windows_core::PCWSTR(1 as _);
pub const RT_DIALOG: windows_core::PCWSTR = windows_core::PCWSTR(5 as _);
pub const RT_DLGINCLUDE: windows_core::PCWSTR = windows_core::PCWSTR(17 as _);
pub const RT_FONT: windows_core::PCWSTR = windows_core::PCWSTR(8 as _);
pub const RT_FONTDIR: windows_core::PCWSTR = windows_core::PCWSTR(7 as _);
pub const RT_HTML: windows_core::PCWSTR = windows_core::PCWSTR(23 as _);
pub const RT_ICON: windows_core::PCWSTR = windows_core::PCWSTR(3 as _);
pub const RT_MANIFEST: windows_core::PCWSTR = windows_core::PCWSTR(24 as _);
pub const RT_MENU: windows_core::PCWSTR = windows_core::PCWSTR(4 as _);
pub const RT_MESSAGETABLE: windows_core::PCWSTR = windows_core::PCWSTR(11 as _);
pub const RT_PLUGPLAY: windows_core::PCWSTR = windows_core::PCWSTR(19 as _);
pub const RT_RCDATA: windows_core::PCWSTR = windows_core::PCWSTR(10 as _);
pub const RT_STRING: windows_core::PCWSTR = windows_core::PCWSTR(6 as _);
pub const RT_VERSION: windows_core::PCWSTR = windows_core::PCWSTR(16 as _);
pub const RT_VXD: windows_core::PCWSTR = windows_core::PCWSTR(20 as _);
pub const SBM_ENABLE_ARROWS: i32 = 228;
pub const SBM_GETPOS: i32 = 225;
pub const SBM_GETRANGE: i32 = 227;
pub const SBM_GETSCROLLBARINFO: i32 = 235;
pub const SBM_GETSCROLLINFO: i32 = 234;
pub const SBM_SETPOS: i32 = 224;
pub const SBM_SETRANGE: i32 = 226;
pub const SBM_SETRANGEREDRAW: i32 = 230;
pub const SBM_SETSCROLLINFO: i32 = 233;
pub const SBS_BOTTOMALIGN: i32 = 4;
pub const SBS_HORZ: i32 = 0;
pub const SBS_LEFTALIGN: i32 = 2;
pub const SBS_RIGHTALIGN: i32 = 4;
pub const SBS_SIZEBOX: i32 = 8;
pub const SBS_SIZEBOXBOTTOMRIGHTALIGN: i32 = 4;
pub const SBS_SIZEBOXTOPLEFTALIGN: i32 = 2;
pub const SBS_SIZEGRIP: i32 = 16;
pub const SBS_TOPALIGN: i32 = 2;
pub const SBS_VERT: i32 = 1;
pub const SB_BOTH: i32 = 3;
pub const SB_BOTTOM: i32 = 7;
pub const SB_CTL: i32 = 2;
pub const SB_ENDSCROLL: i32 = 8;
pub const SB_HORZ: i32 = 0;
pub const SB_LEFT: i32 = 6;
pub const SB_LINEDOWN: i32 = 1;
pub const SB_LINELEFT: i32 = 0;
pub const SB_LINERIGHT: i32 = 1;
pub const SB_LINEUP: i32 = 0;
pub const SB_MIN: i32 = 0;
pub const SB_PAGEDOWN: i32 = 3;
pub const SB_PAGELEFT: i32 = 2;
pub const SB_PAGERIGHT: i32 = 3;
pub const SB_PAGEUP: i32 = 2;
pub const SB_RIGHT: i32 = 7;
pub const SB_THUMBPOSITION: i32 = 4;
pub const SB_THUMBTRACK: i32 = 5;
pub const SB_TOP: i32 = 6;
pub const SB_VERT: i32 = 1;
pub const SCF_ISSECURE: i32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCROLLINFO {
    pub cbSize: u32,
    pub fMask: u32,
    pub nMin: i32,
    pub nMax: i32,
    pub nPage: u32,
    pub nPos: i32,
    pub nTrackPos: i32,
}
pub const SC_ARRANGE: i32 = 61712;
pub const SC_CLOSE: i32 = 61536;
pub const SC_CONTEXTHELP: i32 = 61824;
pub const SC_DEFAULT: i32 = 61792;
pub const SC_HOTKEY: i32 = 61776;
pub const SC_HSCROLL: i32 = 61568;
pub const SC_ICON: i32 = 61472;
pub const SC_KEYMENU: i32 = 61696;
pub const SC_MAXIMIZE: i32 = 61488;
pub const SC_MINIMIZE: i32 = 61472;
pub const SC_MONITORPOWER: i32 = 61808;
pub const SC_MOUSEMENU: i32 = 61584;
pub const SC_MOVE: i32 = 61456;
pub const SC_NEXTWINDOW: i32 = 61504;
pub const SC_PREVWINDOW: i32 = 61520;
pub const SC_RESTORE: i32 = 61728;
pub const SC_SCREENSAVE: i32 = 61760;
pub const SC_SEPARATOR: i32 = 61455;
pub const SC_SIZE: i32 = 61440;
pub const SC_TASKLIST: i32 = 61744;
pub const SC_VSCROLL: i32 = 61552;
pub const SC_ZOOM: i32 = 61488;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type SENDASYNCPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: u32, param2: usize, param3: super::LRESULT)>;
pub type SERIALKEYS = SERIALKEYSA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERIALKEYSA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpszActivePort: windows_core::PSTR,
    pub lpszPort: windows_core::PSTR,
    pub iBaudRate: u32,
    pub iPortState: u32,
    pub iActive: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERIALKEYSW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpszActivePort: windows_core::PWSTR,
    pub lpszPort: windows_core::PWSTR,
    pub iBaudRate: u32,
    pub iPortState: u32,
    pub iActive: u32,
}
pub const SERKF_AVAILABLE: i32 = 2;
pub const SERKF_INDICATOR: i32 = 4;
pub const SERKF_SERIALKEYSON: i32 = 1;
pub const SETWALLPAPER_DEFAULT: windows_core::PCWSTR = windows_core::PCWSTR(-1 as _);
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHELLHOOKINFO {
    pub hwnd: super::HWND,
    pub rc: super::RECT,
}
pub const SHOW_FULLSCREEN: i32 = 3;
pub const SHOW_ICONWINDOW: i32 = 2;
pub const SHOW_OPENNOACTIVATE: i32 = 4;
pub const SHOW_OPENWINDOW: i32 = 1;
pub const SIF_ALL: i32 = 23;
pub const SIF_DISABLENOSCROLL: i32 = 8;
pub const SIF_PAGE: i32 = 2;
pub const SIF_POS: i32 = 4;
pub const SIF_RANGE: i32 = 1;
pub const SIF_TRACKPOS: i32 = 16;
pub const SIZEFULLSCREEN: i32 = 2;
pub const SIZEICONIC: i32 = 1;
pub const SIZENORMAL: i32 = 0;
pub const SIZEZOOMHIDE: i32 = 4;
pub const SIZEZOOMSHOW: i32 = 3;
pub const SIZE_MAXHIDE: i32 = 4;
pub const SIZE_MAXIMIZED: i32 = 2;
pub const SIZE_MAXSHOW: i32 = 3;
pub const SIZE_MINIMIZED: i32 = 1;
pub const SIZE_RESTORED: i32 = 0;
pub const SKF_AUDIBLEFEEDBACK: i32 = 64;
pub const SKF_AVAILABLE: i32 = 2;
pub const SKF_CONFIRMHOTKEY: i32 = 8;
pub const SKF_HOTKEYACTIVE: i32 = 4;
pub const SKF_HOTKEYSOUND: i32 = 16;
pub const SKF_INDICATOR: i32 = 32;
pub const SKF_LALTLATCHED: i32 = 268435456;
pub const SKF_LALTLOCKED: i32 = 1048576;
pub const SKF_LCTLLATCHED: i32 = 67108864;
pub const SKF_LCTLLOCKED: i32 = 262144;
pub const SKF_LSHIFTLATCHED: i32 = 16777216;
pub const SKF_LSHIFTLOCKED: i32 = 65536;
pub const SKF_LWINLATCHED: i32 = 1073741824;
pub const SKF_LWINLOCKED: i32 = 4194304;
pub const SKF_RALTLATCHED: i32 = 536870912;
pub const SKF_RALTLOCKED: i32 = 2097152;
pub const SKF_RCTLLATCHED: i32 = 134217728;
pub const SKF_RCTLLOCKED: i32 = 524288;
pub const SKF_RSHIFTLATCHED: i32 = 33554432;
pub const SKF_RSHIFTLOCKED: i32 = 131072;
pub const SKF_RWINLATCHED: u32 = 2147483648;
pub const SKF_RWINLOCKED: i32 = 8388608;
pub const SKF_STICKYKEYSON: i32 = 1;
pub const SKF_TRISTATE: i32 = 128;
pub const SKF_TWOKEYSOFF: i32 = 256;
pub const SLE_ERROR: i32 = 1;
pub const SLE_MINORERROR: i32 = 2;
pub const SLE_WARNING: i32 = 3;
pub const SMTO_ABORTIFHUNG: i32 = 2;
pub const SMTO_BLOCK: i32 = 1;
pub const SMTO_ERRORONEXIT: i32 = 32;
pub const SMTO_NORMAL: i32 = 0;
pub const SMTO_NOTIMEOUTIFNOTHUNG: i32 = 8;
pub const SM_ARRANGE: i32 = 56;
pub const SM_CARETBLINKINGENABLED: i32 = 8194;
pub const SM_CLEANBOOT: i32 = 67;
pub const SM_CMETRICS: i32 = 97;
pub const SM_CMONITORS: i32 = 80;
pub const SM_CMOUSEBUTTONS: i32 = 43;
pub const SM_CONVERTIBLESLATEMODE: i32 = 8195;
pub const SM_CXBORDER: i32 = 5;
pub const SM_CXCURSOR: i32 = 13;
pub const SM_CXDLGFRAME: i32 = 7;
pub const SM_CXDOUBLECLK: i32 = 36;
pub const SM_CXDRAG: i32 = 68;
pub const SM_CXEDGE: i32 = 45;
pub const SM_CXFIXEDFRAME: i32 = 7;
pub const SM_CXFOCUSBORDER: i32 = 83;
pub const SM_CXFRAME: i32 = 32;
pub const SM_CXFULLSCREEN: i32 = 16;
pub const SM_CXHSCROLL: i32 = 21;
pub const SM_CXHTHUMB: i32 = 10;
pub const SM_CXICON: i32 = 11;
pub const SM_CXICONSPACING: i32 = 38;
pub const SM_CXMAXIMIZED: i32 = 61;
pub const SM_CXMAXTRACK: i32 = 59;
pub const SM_CXMENUCHECK: i32 = 71;
pub const SM_CXMENUSIZE: i32 = 54;
pub const SM_CXMIN: i32 = 28;
pub const SM_CXMINIMIZED: i32 = 57;
pub const SM_CXMINSPACING: i32 = 47;
pub const SM_CXMINTRACK: i32 = 34;
pub const SM_CXPADDEDBORDER: i32 = 92;
pub const SM_CXSCREEN: i32 = 0;
pub const SM_CXSIZE: i32 = 30;
pub const SM_CXSIZEFRAME: i32 = 32;
pub const SM_CXSMICON: i32 = 49;
pub const SM_CXSMSIZE: i32 = 52;
pub const SM_CXVIRTUALSCREEN: i32 = 78;
pub const SM_CXVSCROLL: i32 = 2;
pub const SM_CYBORDER: i32 = 6;
pub const SM_CYCAPTION: i32 = 4;
pub const SM_CYCURSOR: i32 = 14;
pub const SM_CYDLGFRAME: i32 = 8;
pub const SM_CYDOUBLECLK: i32 = 37;
pub const SM_CYDRAG: i32 = 69;
pub const SM_CYEDGE: i32 = 46;
pub const SM_CYFIXEDFRAME: i32 = 8;
pub const SM_CYFOCUSBORDER: i32 = 84;
pub const SM_CYFRAME: i32 = 33;
pub const SM_CYFULLSCREEN: i32 = 17;
pub const SM_CYHSCROLL: i32 = 3;
pub const SM_CYICON: i32 = 12;
pub const SM_CYICONSPACING: i32 = 39;
pub const SM_CYKANJIWINDOW: i32 = 18;
pub const SM_CYMAXIMIZED: i32 = 62;
pub const SM_CYMAXTRACK: i32 = 60;
pub const SM_CYMENU: i32 = 15;
pub const SM_CYMENUCHECK: i32 = 72;
pub const SM_CYMENUSIZE: i32 = 55;
pub const SM_CYMIN: i32 = 29;
pub const SM_CYMINIMIZED: i32 = 58;
pub const SM_CYMINSPACING: i32 = 48;
pub const SM_CYMINTRACK: i32 = 35;
pub const SM_CYSCREEN: i32 = 1;
pub const SM_CYSIZE: i32 = 31;
pub const SM_CYSIZEFRAME: i32 = 33;
pub const SM_CYSMCAPTION: i32 = 51;
pub const SM_CYSMICON: i32 = 50;
pub const SM_CYSMSIZE: i32 = 53;
pub const SM_CYVIRTUALSCREEN: i32 = 79;
pub const SM_CYVSCROLL: i32 = 20;
pub const SM_CYVTHUMB: i32 = 9;
pub const SM_DBCSENABLED: i32 = 42;
pub const SM_DEBUG: i32 = 22;
pub const SM_DIGITIZER: i32 = 94;
pub const SM_IMMENABLED: i32 = 82;
pub const SM_MAXIMUMTOUCHES: i32 = 95;
pub const SM_MEDIACENTER: i32 = 87;
pub const SM_MENUDROPALIGNMENT: i32 = 40;
pub const SM_MIDEASTENABLED: i32 = 74;
pub const SM_MOUSEHORIZONTALWHEELPRESENT: i32 = 91;
pub const SM_MOUSEPRESENT: i32 = 19;
pub const SM_MOUSEWHEELPRESENT: i32 = 75;
pub const SM_NETWORK: i32 = 63;
pub const SM_PENWINDOWS: i32 = 41;
pub const SM_REMOTECONTROL: i32 = 8193;
pub const SM_REMOTESESSION: i32 = 4096;
pub const SM_RESERVED1: i32 = 24;
pub const SM_RESERVED2: i32 = 25;
pub const SM_RESERVED3: i32 = 26;
pub const SM_RESERVED4: i32 = 27;
pub const SM_SAMEDISPLAYFORMAT: i32 = 81;
pub const SM_SECURE: i32 = 44;
pub const SM_SERVERR2: i32 = 89;
pub const SM_SHOWSOUNDS: i32 = 70;
pub const SM_SHUTTINGDOWN: i32 = 8192;
pub const SM_SLOWMACHINE: i32 = 73;
pub const SM_STARTER: i32 = 88;
pub const SM_SWAPBUTTON: i32 = 23;
pub const SM_SYSTEMDOCKED: i32 = 8196;
pub const SM_TABLETPC: i32 = 86;
pub const SM_XVIRTUALSCREEN: i32 = 76;
pub const SM_YVIRTUALSCREEN: i32 = 77;
pub type SOUNDSENTRY = SOUNDSENTRYA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub lpszWindowsEffectDLL: windows_core::PSTR,
    pub iWindowsEffectOrdinal: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub lpszWindowsEffectDLL: windows_core::PWSTR,
    pub iWindowsEffectOrdinal: u32,
}
pub const SOUND_SYSTEM_APPEND: i32 = 14;
pub const SOUND_SYSTEM_APPSTART: i32 = 12;
pub const SOUND_SYSTEM_BEEP: i32 = 3;
pub const SOUND_SYSTEM_ERROR: i32 = 4;
pub const SOUND_SYSTEM_FAULT: i32 = 13;
pub const SOUND_SYSTEM_INFORMATION: i32 = 7;
pub const SOUND_SYSTEM_MAXIMIZE: i32 = 8;
pub const SOUND_SYSTEM_MENUCOMMAND: i32 = 15;
pub const SOUND_SYSTEM_MENUPOPUP: i32 = 16;
pub const SOUND_SYSTEM_MINIMIZE: i32 = 9;
pub const SOUND_SYSTEM_QUESTION: i32 = 5;
pub const SOUND_SYSTEM_RESTOREDOWN: i32 = 11;
pub const SOUND_SYSTEM_RESTOREUP: i32 = 10;
pub const SOUND_SYSTEM_SHUTDOWN: i32 = 2;
pub const SOUND_SYSTEM_STARTUP: i32 = 1;
pub const SOUND_SYSTEM_WARNING: i32 = 6;
pub const SPIF_SENDCHANGE: i32 = 2;
pub const SPIF_SENDWININICHANGE: i32 = 2;
pub const SPIF_UPDATEINIFILE: i32 = 1;
pub const SPI_GETACCESSTIMEOUT: i32 = 60;
pub const SPI_GETACTIVEWINDOWTRACKING: i32 = 4096;
pub const SPI_GETACTIVEWNDTRKTIMEOUT: i32 = 8194;
pub const SPI_GETACTIVEWNDTRKZORDER: i32 = 4108;
pub const SPI_GETANIMATION: i32 = 72;
pub const SPI_GETAUDIODESCRIPTION: i32 = 116;
pub const SPI_GETBEEP: i32 = 1;
pub const SPI_GETBLOCKSENDINPUTRESETS: i32 = 4134;
pub const SPI_GETBORDER: i32 = 5;
pub const SPI_GETCARETBROWSING: i32 = 4172;
pub const SPI_GETCARETTIMEOUT: i32 = 8226;
pub const SPI_GETCARETWIDTH: i32 = 8198;
pub const SPI_GETCLEARTYPE: i32 = 4168;
pub const SPI_GETCLIENTAREAANIMATION: i32 = 4162;
pub const SPI_GETCOMBOBOXANIMATION: i32 = 4100;
pub const SPI_GETCONTACTVISUALIZATION: i32 = 8216;
pub const SPI_GETCURSORSHADOW: i32 = 4122;
pub const SPI_GETDEFAULTINPUTLANG: i32 = 89;
pub const SPI_GETDESKWALLPAPER: i32 = 115;
pub const SPI_GETDISABLEOVERLAPPEDCONTENT: i32 = 4160;
pub const SPI_GETDOCKMOVING: i32 = 144;
pub const SPI_GETDRAGFROMMAXIMIZE: i32 = 140;
pub const SPI_GETDRAGFULLWINDOWS: i32 = 38;
pub const SPI_GETDROPSHADOW: i32 = 4132;
pub const SPI_GETFASTTASKSWITCH: i32 = 35;
pub const SPI_GETFILTERKEYS: i32 = 50;
pub const SPI_GETFLATMENU: i32 = 4130;
pub const SPI_GETFOCUSBORDERHEIGHT: i32 = 8208;
pub const SPI_GETFOCUSBORDERWIDTH: i32 = 8206;
pub const SPI_GETFONTSMOOTHING: i32 = 74;
pub const SPI_GETFONTSMOOTHINGCONTRAST: i32 = 8204;
pub const SPI_GETFONTSMOOTHINGORIENTATION: i32 = 8210;
pub const SPI_GETFONTSMOOTHINGTYPE: i32 = 8202;
pub const SPI_GETFOREGROUNDFLASHCOUNT: i32 = 8196;
pub const SPI_GETFOREGROUNDLOCKTIMEOUT: i32 = 8192;
pub const SPI_GETGESTUREVISUALIZATION: i32 = 8218;
pub const SPI_GETGRADIENTCAPTIONS: i32 = 4104;
pub const SPI_GETGRIDGRANULARITY: i32 = 18;
pub const SPI_GETHANDEDNESS: i32 = 8228;
pub const SPI_GETHIGHCONTRAST: i32 = 66;
pub const SPI_GETHOTTRACKING: i32 = 4110;
pub const SPI_GETHUNGAPPTIMEOUT: i32 = 120;
pub const SPI_GETICONMETRICS: i32 = 45;
pub const SPI_GETICONTITLELOGFONT: i32 = 31;
pub const SPI_GETICONTITLEWRAP: i32 = 25;
pub const SPI_GETKEYBOARDCUES: i32 = 4106;
pub const SPI_GETKEYBOARDDELAY: i32 = 22;
pub const SPI_GETKEYBOARDPREF: i32 = 68;
pub const SPI_GETKEYBOARDSPEED: i32 = 10;
pub const SPI_GETLISTBOXSMOOTHSCROLLING: i32 = 4102;
pub const SPI_GETLOGICALDPIOVERRIDE: i32 = 158;
pub const SPI_GETLOWPOWERACTIVE: i32 = 83;
pub const SPI_GETLOWPOWERTIMEOUT: i32 = 79;
pub const SPI_GETMENUANIMATION: i32 = 4098;
pub const SPI_GETMENUDROPALIGNMENT: i32 = 27;
pub const SPI_GETMENUFADE: i32 = 4114;
pub const SPI_GETMENURECT: i32 = 162;
pub const SPI_GETMENUSHOWDELAY: i32 = 106;
pub const SPI_GETMENUUNDERLINES: i32 = 4106;
pub const SPI_GETMESSAGEDURATION: i32 = 8214;
pub const SPI_GETMINIMIZEDMETRICS: i32 = 43;
pub const SPI_GETMINIMUMHITRADIUS: i32 = 8212;
pub const SPI_GETMOUSE: i32 = 3;
pub const SPI_GETMOUSECLICKLOCK: i32 = 4126;
pub const SPI_GETMOUSECLICKLOCKTIME: i32 = 8200;
pub const SPI_GETMOUSEDOCKTHRESHOLD: i32 = 126;
pub const SPI_GETMOUSEDRAGOUTTHRESHOLD: i32 = 132;
pub const SPI_GETMOUSEHOVERHEIGHT: i32 = 100;
pub const SPI_GETMOUSEHOVERTIME: i32 = 102;
pub const SPI_GETMOUSEHOVERWIDTH: i32 = 98;
pub const SPI_GETMOUSEKEYS: i32 = 54;
pub const SPI_GETMOUSESIDEMOVETHRESHOLD: i32 = 136;
pub const SPI_GETMOUSESONAR: i32 = 4124;
pub const SPI_GETMOUSESPEED: i32 = 112;
pub const SPI_GETMOUSETRAILS: i32 = 94;
pub const SPI_GETMOUSEVANISH: i32 = 4128;
pub const SPI_GETMOUSEWHEELROUTING: i32 = 8220;
pub const SPI_GETNONCLIENTMETRICS: i32 = 41;
pub const SPI_GETPENARBITRATIONTYPE: i32 = 8224;
pub const SPI_GETPENDOCKTHRESHOLD: i32 = 128;
pub const SPI_GETPENDRAGOUTTHRESHOLD: i32 = 134;
pub const SPI_GETPENSIDEMOVETHRESHOLD: i32 = 138;
pub const SPI_GETPENVISUALIZATION: i32 = 8222;
pub const SPI_GETPOWEROFFACTIVE: i32 = 84;
pub const SPI_GETPOWEROFFTIMEOUT: i32 = 80;
pub const SPI_GETSCREENREADER: i32 = 70;
pub const SPI_GETSCREENSAVEACTIVE: i32 = 16;
pub const SPI_GETSCREENSAVERRUNNING: i32 = 114;
pub const SPI_GETSCREENSAVESECURE: i32 = 118;
pub const SPI_GETSCREENSAVETIMEOUT: i32 = 14;
pub const SPI_GETSELECTIONFADE: i32 = 4116;
pub const SPI_GETSERIALKEYS: i32 = 62;
pub const SPI_GETSHOWIMEUI: i32 = 110;
pub const SPI_GETSHOWSOUNDS: i32 = 56;
pub const SPI_GETSNAPSIZING: i32 = 142;
pub const SPI_GETSNAPTODEFBUTTON: i32 = 95;
pub const SPI_GETSOUNDSENTRY: i32 = 64;
pub const SPI_GETSPEECHRECOGNITION: i32 = 4170;
pub const SPI_GETSTICKYKEYS: i32 = 58;
pub const SPI_GETSYSTEMLANGUAGEBAR: i32 = 4176;
pub const SPI_GETTHREADLOCALINPUTSETTINGS: i32 = 4174;
pub const SPI_GETTOGGLEKEYS: i32 = 52;
pub const SPI_GETTOOLTIPANIMATION: i32 = 4118;
pub const SPI_GETTOOLTIPFADE: i32 = 4120;
pub const SPI_GETTOUCHPADPARAMETERS: i32 = 174;
pub const SPI_GETTOUCHPREDICTIONPARAMETERS: i32 = 156;
pub const SPI_GETUIEFFECTS: i32 = 4158;
pub const SPI_GETWAITTOKILLSERVICETIMEOUT: i32 = 124;
pub const SPI_GETWAITTOKILLTIMEOUT: i32 = 122;
pub const SPI_GETWHEELSCROLLCHARS: i32 = 108;
pub const SPI_GETWHEELSCROLLLINES: i32 = 104;
pub const SPI_GETWINARRANGING: i32 = 130;
pub const SPI_GETWINDOWSEXTENSION: i32 = 92;
pub const SPI_GETWORKAREA: i32 = 48;
pub const SPI_ICONHORIZONTALSPACING: i32 = 13;
pub const SPI_ICONVERTICALSPACING: i32 = 24;
pub const SPI_LANGDRIVER: i32 = 12;
pub const SPI_SCREENSAVERRUNNING: i32 = 97;
pub const SPI_SETACCESSTIMEOUT: i32 = 61;
pub const SPI_SETACTIVEWINDOWTRACKING: i32 = 4097;
pub const SPI_SETACTIVEWNDTRKTIMEOUT: i32 = 8195;
pub const SPI_SETACTIVEWNDTRKZORDER: i32 = 4109;
pub const SPI_SETANIMATION: i32 = 73;
pub const SPI_SETAUDIODESCRIPTION: i32 = 117;
pub const SPI_SETBEEP: i32 = 2;
pub const SPI_SETBLOCKSENDINPUTRESETS: i32 = 4135;
pub const SPI_SETBORDER: i32 = 6;
pub const SPI_SETCARETBROWSING: i32 = 4173;
pub const SPI_SETCARETTIMEOUT: i32 = 8227;
pub const SPI_SETCARETWIDTH: i32 = 8199;
pub const SPI_SETCLEARTYPE: i32 = 4169;
pub const SPI_SETCLIENTAREAANIMATION: i32 = 4163;
pub const SPI_SETCOMBOBOXANIMATION: i32 = 4101;
pub const SPI_SETCONTACTVISUALIZATION: i32 = 8217;
pub const SPI_SETCURSORS: i32 = 87;
pub const SPI_SETCURSORSHADOW: i32 = 4123;
pub const SPI_SETDEFAULTINPUTLANG: i32 = 90;
pub const SPI_SETDESKPATTERN: i32 = 21;
pub const SPI_SETDESKWALLPAPER: i32 = 20;
pub const SPI_SETDISABLEOVERLAPPEDCONTENT: i32 = 4161;
pub const SPI_SETDOCKMOVING: i32 = 145;
pub const SPI_SETDOUBLECLICKTIME: i32 = 32;
pub const SPI_SETDOUBLECLKHEIGHT: i32 = 30;
pub const SPI_SETDOUBLECLKWIDTH: i32 = 29;
pub const SPI_SETDRAGFROMMAXIMIZE: i32 = 141;
pub const SPI_SETDRAGFULLWINDOWS: i32 = 37;
pub const SPI_SETDRAGHEIGHT: i32 = 77;
pub const SPI_SETDRAGWIDTH: i32 = 76;
pub const SPI_SETDROPSHADOW: i32 = 4133;
pub const SPI_SETFASTTASKSWITCH: i32 = 36;
pub const SPI_SETFILTERKEYS: i32 = 51;
pub const SPI_SETFLATMENU: i32 = 4131;
pub const SPI_SETFOCUSBORDERHEIGHT: i32 = 8209;
pub const SPI_SETFOCUSBORDERWIDTH: i32 = 8207;
pub const SPI_SETFONTSMOOTHING: i32 = 75;
pub const SPI_SETFONTSMOOTHINGCONTRAST: i32 = 8205;
pub const SPI_SETFONTSMOOTHINGORIENTATION: i32 = 8211;
pub const SPI_SETFONTSMOOTHINGTYPE: i32 = 8203;
pub const SPI_SETFOREGROUNDFLASHCOUNT: i32 = 8197;
pub const SPI_SETFOREGROUNDLOCKTIMEOUT: i32 = 8193;
pub const SPI_SETGESTUREVISUALIZATION: i32 = 8219;
pub const SPI_SETGRADIENTCAPTIONS: i32 = 4105;
pub const SPI_SETGRIDGRANULARITY: i32 = 19;
pub const SPI_SETHANDEDNESS: i32 = 8229;
pub const SPI_SETHANDHELD: i32 = 78;
pub const SPI_SETHIGHCONTRAST: i32 = 67;
pub const SPI_SETHOTTRACKING: i32 = 4111;
pub const SPI_SETHUNGAPPTIMEOUT: i32 = 121;
pub const SPI_SETICONMETRICS: i32 = 46;
pub const SPI_SETICONS: i32 = 88;
pub const SPI_SETICONTITLELOGFONT: i32 = 34;
pub const SPI_SETICONTITLEWRAP: i32 = 26;
pub const SPI_SETKEYBOARDCUES: i32 = 4107;
pub const SPI_SETKEYBOARDDELAY: i32 = 23;
pub const SPI_SETKEYBOARDPREF: i32 = 69;
pub const SPI_SETKEYBOARDSPEED: i32 = 11;
pub const SPI_SETLANGTOGGLE: i32 = 91;
pub const SPI_SETLISTBOXSMOOTHSCROLLING: i32 = 4103;
pub const SPI_SETLOGICALDPIOVERRIDE: i32 = 159;
pub const SPI_SETLOWPOWERACTIVE: i32 = 85;
pub const SPI_SETLOWPOWERTIMEOUT: i32 = 81;
pub const SPI_SETMENUANIMATION: i32 = 4099;
pub const SPI_SETMENUDROPALIGNMENT: i32 = 28;
pub const SPI_SETMENUFADE: i32 = 4115;
pub const SPI_SETMENURECT: i32 = 163;
pub const SPI_SETMENUSHOWDELAY: i32 = 107;
pub const SPI_SETMENUUNDERLINES: i32 = 4107;
pub const SPI_SETMESSAGEDURATION: i32 = 8215;
pub const SPI_SETMINIMIZEDMETRICS: i32 = 44;
pub const SPI_SETMINIMUMHITRADIUS: i32 = 8213;
pub const SPI_SETMOUSE: i32 = 4;
pub const SPI_SETMOUSEBUTTONSWAP: i32 = 33;
pub const SPI_SETMOUSECLICKLOCK: i32 = 4127;
pub const SPI_SETMOUSECLICKLOCKTIME: i32 = 8201;
pub const SPI_SETMOUSEDOCKTHRESHOLD: i32 = 127;
pub const SPI_SETMOUSEDRAGOUTTHRESHOLD: i32 = 133;
pub const SPI_SETMOUSEHOVERHEIGHT: i32 = 101;
pub const SPI_SETMOUSEHOVERTIME: i32 = 103;
pub const SPI_SETMOUSEHOVERWIDTH: i32 = 99;
pub const SPI_SETMOUSEKEYS: i32 = 55;
pub const SPI_SETMOUSESIDEMOVETHRESHOLD: i32 = 137;
pub const SPI_SETMOUSESONAR: i32 = 4125;
pub const SPI_SETMOUSESPEED: i32 = 113;
pub const SPI_SETMOUSETRAILS: i32 = 93;
pub const SPI_SETMOUSEVANISH: i32 = 4129;
pub const SPI_SETMOUSEWHEELROUTING: i32 = 8221;
pub const SPI_SETNONCLIENTMETRICS: i32 = 42;
pub const SPI_SETPENARBITRATIONTYPE: i32 = 8225;
pub const SPI_SETPENDOCKTHRESHOLD: i32 = 129;
pub const SPI_SETPENDRAGOUTTHRESHOLD: i32 = 135;
pub const SPI_SETPENSIDEMOVETHRESHOLD: i32 = 139;
pub const SPI_SETPENVISUALIZATION: i32 = 8223;
pub const SPI_SETPENWINDOWS: i32 = 49;
pub const SPI_SETPOWEROFFACTIVE: i32 = 86;
pub const SPI_SETPOWEROFFTIMEOUT: i32 = 82;
pub const SPI_SETSCREENREADER: i32 = 71;
pub const SPI_SETSCREENSAVEACTIVE: i32 = 17;
pub const SPI_SETSCREENSAVERRUNNING: i32 = 97;
pub const SPI_SETSCREENSAVESECURE: i32 = 119;
pub const SPI_SETSCREENSAVETIMEOUT: i32 = 15;
pub const SPI_SETSELECTIONFADE: i32 = 4117;
pub const SPI_SETSERIALKEYS: i32 = 63;
pub const SPI_SETSHOWIMEUI: i32 = 111;
pub const SPI_SETSHOWSOUNDS: i32 = 57;
pub const SPI_SETSNAPSIZING: i32 = 143;
pub const SPI_SETSNAPTODEFBUTTON: i32 = 96;
pub const SPI_SETSOUNDSENTRY: i32 = 65;
pub const SPI_SETSPEECHRECOGNITION: i32 = 4171;
pub const SPI_SETSTICKYKEYS: i32 = 59;
pub const SPI_SETSYSTEMLANGUAGEBAR: i32 = 4177;
pub const SPI_SETTHREADLOCALINPUTSETTINGS: i32 = 4175;
pub const SPI_SETTOGGLEKEYS: i32 = 53;
pub const SPI_SETTOOLTIPANIMATION: i32 = 4119;
pub const SPI_SETTOOLTIPFADE: i32 = 4121;
pub const SPI_SETTOUCHPADPARAMETERS: i32 = 175;
pub const SPI_SETTOUCHPREDICTIONPARAMETERS: i32 = 157;
pub const SPI_SETUIEFFECTS: i32 = 4159;
pub const SPI_SETWAITTOKILLSERVICETIMEOUT: i32 = 125;
pub const SPI_SETWAITTOKILLTIMEOUT: i32 = 123;
pub const SPI_SETWHEELSCROLLCHARS: i32 = 109;
pub const SPI_SETWHEELSCROLLLINES: i32 = 105;
pub const SPI_SETWINARRANGING: i32 = 131;
pub const SPI_SETWORKAREA: i32 = 47;
pub const SSF_AVAILABLE: i32 = 2;
pub const SSF_INDICATOR: i32 = 4;
pub const SSF_SOUNDSENTRYON: i32 = 1;
pub const SSGF_DISPLAY: i32 = 3;
pub const SSGF_NONE: i32 = 0;
pub const SSTF_BORDER: i32 = 2;
pub const SSTF_CHARS: i32 = 1;
pub const SSTF_DISPLAY: i32 = 3;
pub const SSTF_NONE: i32 = 0;
pub const SSWF_CUSTOM: i32 = 4;
pub const SSWF_DISPLAY: i32 = 3;
pub const SSWF_NONE: i32 = 0;
pub const SSWF_TITLE: i32 = 1;
pub const SSWF_WINDOW: i32 = 2;
pub const SS_BITMAP: i32 = 14;
pub const SS_BLACKFRAME: i32 = 7;
pub const SS_BLACKRECT: i32 = 4;
pub const SS_CENTER: i32 = 1;
pub const SS_CENTERIMAGE: i32 = 512;
pub const SS_EDITCONTROL: i32 = 8192;
pub const SS_ELLIPSISMASK: i32 = 49152;
pub const SS_ENDELLIPSIS: i32 = 16384;
pub const SS_ENHMETAFILE: i32 = 15;
pub const SS_ETCHEDFRAME: i32 = 18;
pub const SS_ETCHEDHORZ: i32 = 16;
pub const SS_ETCHEDVERT: i32 = 17;
pub const SS_GRAYFRAME: i32 = 8;
pub const SS_GRAYRECT: i32 = 5;
pub const SS_ICON: i32 = 3;
pub const SS_LEFT: i32 = 0;
pub const SS_LEFTNOWORDWRAP: i32 = 12;
pub const SS_NOPREFIX: i32 = 128;
pub const SS_NOTIFY: i32 = 256;
pub const SS_OWNERDRAW: i32 = 13;
pub const SS_PATHELLIPSIS: i32 = 32768;
pub const SS_REALSIZECONTROL: i32 = 64;
pub const SS_REALSIZEIMAGE: i32 = 2048;
pub const SS_RIGHT: i32 = 2;
pub const SS_RIGHTJUST: i32 = 1024;
pub const SS_SIMPLE: i32 = 11;
pub const SS_SUNKEN: i32 = 4096;
pub const SS_TYPEMASK: i32 = 31;
pub const SS_USERITEM: i32 = 10;
pub const SS_WHITEFRAME: i32 = 9;
pub const SS_WHITERECT: i32 = 6;
pub const SS_WORDELLIPSIS: i32 = 49152;
pub const STATE_SYSTEM_ALERT_HIGH: i32 = 268435456;
pub const STATE_SYSTEM_ALERT_LOW: i32 = 67108864;
pub const STATE_SYSTEM_ALERT_MEDIUM: i32 = 134217728;
pub const STATE_SYSTEM_ANIMATED: i32 = 16384;
pub const STATE_SYSTEM_BUSY: i32 = 2048;
pub const STATE_SYSTEM_CHECKED: i32 = 16;
pub const STATE_SYSTEM_COLLAPSED: i32 = 1024;
pub const STATE_SYSTEM_DEFAULT: i32 = 256;
pub const STATE_SYSTEM_EXPANDED: i32 = 512;
pub const STATE_SYSTEM_EXTSELECTABLE: i32 = 33554432;
pub const STATE_SYSTEM_FLOATING: i32 = 4096;
pub const STATE_SYSTEM_FOCUSABLE: i32 = 1048576;
pub const STATE_SYSTEM_FOCUSED: i32 = 4;
pub const STATE_SYSTEM_HOTTRACKED: i32 = 128;
pub const STATE_SYSTEM_INDETERMINATE: i32 = 32;
pub const STATE_SYSTEM_INVISIBLE: i32 = 32768;
pub const STATE_SYSTEM_LINKED: i32 = 4194304;
pub const STATE_SYSTEM_MARQUEED: i32 = 8192;
pub const STATE_SYSTEM_MIXED: i32 = 32;
pub const STATE_SYSTEM_MOVEABLE: i32 = 262144;
pub const STATE_SYSTEM_MULTISELECTABLE: i32 = 16777216;
pub const STATE_SYSTEM_OFFSCREEN: i32 = 65536;
pub const STATE_SYSTEM_PRESSED: i32 = 8;
pub const STATE_SYSTEM_PROTECTED: i32 = 536870912;
pub const STATE_SYSTEM_READONLY: i32 = 64;
pub const STATE_SYSTEM_SELECTABLE: i32 = 2097152;
pub const STATE_SYSTEM_SELECTED: i32 = 2;
pub const STATE_SYSTEM_SELFVOICING: i32 = 524288;
pub const STATE_SYSTEM_SIZEABLE: i32 = 131072;
pub const STATE_SYSTEM_TRAVERSED: i32 = 8388608;
pub const STATE_SYSTEM_UNAVAILABLE: i32 = 1;
pub const STATE_SYSTEM_VALID: i32 = 1073741823;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STICKYKEYS {
    pub cbSize: u32,
    pub dwFlags: u32,
}
pub const STM_GETICON: i32 = 369;
pub const STM_GETIMAGE: i32 = 371;
pub const STM_MSGMAX: i32 = 372;
pub const STM_SETICON: i32 = 368;
pub const STM_SETIMAGE: i32 = 370;
pub const STN_CLICKED: i32 = 0;
pub const STN_DBLCLK: i32 = 1;
pub const STN_DISABLE: i32 = 3;
pub const STN_ENABLE: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STYLESTRUCT {
    pub styleOld: u32,
    pub styleNew: u32,
}
pub const SWP_ASYNCWINDOWPOS: i32 = 16384;
pub const SWP_DEFERERASE: i32 = 8192;
pub const SWP_DRAWFRAME: i32 = 32;
pub const SWP_FRAMECHANGED: i32 = 32;
pub const SWP_HIDEWINDOW: i32 = 128;
pub const SWP_NOACTIVATE: i32 = 16;
pub const SWP_NOCOPYBITS: i32 = 256;
pub const SWP_NOMOVE: i32 = 2;
pub const SWP_NONE: i32 = 0;
pub const SWP_NOOWNERZORDER: i32 = 512;
pub const SWP_NOREDRAW: i32 = 8;
pub const SWP_NOREPOSITION: i32 = 512;
pub const SWP_NOSENDCHANGING: i32 = 1024;
pub const SWP_NOSIZE: i32 = 1;
pub const SWP_NOZORDER: i32 = 4;
pub const SWP_SHOWWINDOW: i32 = 64;
pub const SW_ERASE: i32 = 4;
pub const SW_FORCEMINIMIZE: i32 = 11;
pub const SW_HIDE: i32 = 0;
pub const SW_INVALIDATE: i32 = 2;
pub const SW_MAX: i32 = 11;
pub const SW_MAXIMIZE: i32 = 3;
pub const SW_MINIMIZE: i32 = 6;
pub const SW_NORMAL: i32 = 1;
pub const SW_OTHERUNZOOM: i32 = 4;
pub const SW_OTHERZOOM: i32 = 2;
pub const SW_PARENTCLOSING: i32 = 1;
pub const SW_PARENTOPENING: i32 = 3;
pub const SW_RESTORE: i32 = 9;
pub const SW_SCROLLCHILDREN: i32 = 1;
pub const SW_SHOW: i32 = 5;
pub const SW_SHOWDEFAULT: i32 = 10;
pub const SW_SHOWMAXIMIZED: i32 = 3;
pub const SW_SHOWMINIMIZED: i32 = 2;
pub const SW_SHOWMINNOACTIVE: i32 = 7;
pub const SW_SHOWNA: i32 = 8;
pub const SW_SHOWNOACTIVATE: i32 = 4;
pub const SW_SHOWNORMAL: i32 = 1;
pub const SW_SMOOTHSCROLL: i32 = 16;
pub const TDF_REGISTER: TOOLTIP_DISMISS_FLAGS = 1;
pub const TDF_UNREGISTER: TOOLTIP_DISMISS_FLAGS = 2;
#[cfg(feature = "windef")]
pub type TIMERPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: u32, param2: usize, param3: u32)>;
pub const TIMERV_COALESCING_MAX: i32 = 2147483637;
pub const TIMERV_COALESCING_MIN: i32 = 1;
pub const TIMERV_DEFAULT_COALESCING: i32 = 0;
pub const TIMERV_NO_COALESCING: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const TKF_AVAILABLE: i32 = 2;
pub const TKF_CONFIRMHOTKEY: i32 = 8;
pub const TKF_HOTKEYACTIVE: i32 = 4;
pub const TKF_HOTKEYSOUND: i32 = 16;
pub const TKF_INDICATOR: i32 = 32;
pub const TKF_TOGGLEKEYSON: i32 = 1;
pub const TME_CANCEL: u32 = 2147483648;
pub const TME_HOVER: i32 = 1;
pub const TME_LEAVE: i32 = 2;
pub const TME_NONCLIENT: i32 = 16;
pub const TME_QUERY: i32 = 1073741824;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOGGLEKEYS {
    pub cbSize: u32,
    pub dwFlags: u32,
}
pub type TOOLTIP_DISMISS_FLAGS = i32;
pub const TOUCHEVENTF_DOWN: i32 = 2;
pub const TOUCHEVENTF_INRANGE: i32 = 8;
pub const TOUCHEVENTF_MOVE: i32 = 1;
pub const TOUCHEVENTF_NOCOALESCE: i32 = 32;
pub const TOUCHEVENTF_PALM: i32 = 128;
pub const TOUCHEVENTF_PEN: i32 = 64;
pub const TOUCHEVENTF_PRIMARY: i32 = 16;
pub const TOUCHEVENTF_UP: i32 = 4;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const TOUCHINPUTMASKF_CONTACTAREA: i32 = 4;
pub const TOUCHINPUTMASKF_EXTRAINFO: i32 = 2;
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOUCHPAD_PARAMETERS_V1 {
    pub versionNumber: u32,
    pub maxSupportedContacts: u32,
    pub legacyTouchpadFeatures: LEGACY_TOUCHPAD_FEATURES,
    pub _bitfield1: windows_core::BOOL,
    pub _bitfield2: windows_core::BOOL,
    pub sensitivityLevel: TOUCHPAD_SENSITIVITY_LEVEL,
    pub cursorSpeed: u32,
    pub feedbackIntensity: u32,
    pub clickForceSensitivity: u32,
    pub rightClickZoneWidth: u32,
    pub rightClickZoneHeight: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOUCHPAD_PARAMETERS_V2 {
    pub Base: TOUCHPAD_PARAMETERS_V1,
    pub _bitfield: windows_core::BOOL,
}
pub const TOUCHPAD_PARAMETERS_VERSION_1: i32 = 1;
pub const TOUCHPAD_PARAMETERS_VERSION_2: i32 = 2;
pub type TOUCHPAD_SENSITIVITY_LEVEL = i32;
pub const TOUCHPAD_SENSITIVITY_LEVEL_HIGH_SENSITIVITY: TOUCHPAD_SENSITIVITY_LEVEL = 1;
pub const TOUCHPAD_SENSITIVITY_LEVEL_LEAST_SENSITIVE: TOUCHPAD_SENSITIVITY_LEVEL = 4;
pub const TOUCHPAD_SENSITIVITY_LEVEL_LOW_SENSITIVITY: TOUCHPAD_SENSITIVITY_LEVEL = 3;
pub const TOUCHPAD_SENSITIVITY_LEVEL_MEDIUM_SENSITIVITY: TOUCHPAD_SENSITIVITY_LEVEL = 2;
pub const TOUCHPAD_SENSITIVITY_LEVEL_MOST_SENSITIVE: TOUCHPAD_SENSITIVITY_LEVEL = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOUCHPREDICTIONPARAMETERS {
    pub cbSize: u32,
    pub dwLatency: u32,
    pub dwSampleTime: u32,
    pub bUseHWTimeStamp: u32,
}
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_LATENCY: i32 = 8;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_DELTA: f32 = 0.001;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_EXPO_SMOOTH_ALPHA: f32 = 0.99;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_LEARNING_RATE: f32 = 0.001;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_MAX: f32 = 0.999;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_RLS_LAMBDA_MIN: f32 = 0.9;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_SAMPLETIME: i32 = 8;
pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_USE_HW_TIMESTAMP: i32 = 1;
pub const TOUCH_FEEDBACK_DEFAULT: i32 = 1;
pub const TOUCH_FEEDBACK_INDIRECT: i32 = 2;
pub const TOUCH_FEEDBACK_NONE: i32 = 3;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TOUCH_FLAGS(pub u32);
pub const TOUCH_FLAG_NONE: i32 = 0;
pub const TOUCH_HIT_TESTING_CLIENT: i32 = 1;
pub const TOUCH_HIT_TESTING_DEFAULT: i32 = 0;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOUCH_HIT_TESTING_INPUT {
    pub pointerId: u32,
    pub point: super::POINT,
    pub boundingBox: super::RECT,
    pub nonOccludedBoundingBox: super::RECT,
    pub orientation: u32,
}
pub const TOUCH_HIT_TESTING_NONE: i32 = 2;
pub const TOUCH_HIT_TESTING_PROXIMITY_CLOSEST: i32 = 0;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    pub score: u16,
    pub adjustedPoint: super::POINT,
}
pub const TOUCH_HIT_TESTING_PROXIMITY_FARTHEST: i32 = 4095;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TOUCH_MASK(pub u32);
pub const TOUCH_MASK_CONTACTAREA: i32 = 1;
pub const TOUCH_MASK_NONE: i32 = 0;
pub const TOUCH_MASK_ORIENTATION: i32 = 2;
pub const TOUCH_MASK_PRESSURE: i32 = 4;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TPMPARAMS {
    pub cbSize: u32,
    pub rcExclude: super::RECT,
}
pub const TPM_BOTTOMALIGN: i32 = 32;
pub const TPM_CENTERALIGN: i32 = 4;
pub const TPM_HORIZONTAL: i32 = 0;
pub const TPM_HORNEGANIMATION: i32 = 2048;
pub const TPM_HORPOSANIMATION: i32 = 1024;
pub const TPM_LAYOUTRTL: i32 = 32768;
pub const TPM_LEFTALIGN: i32 = 0;
pub const TPM_LEFTBUTTON: i32 = 0;
pub const TPM_NOANIMATION: i32 = 16384;
pub const TPM_NONOTIFY: i32 = 128;
pub const TPM_RECURSE: i32 = 1;
pub const TPM_RETURNCMD: i32 = 256;
pub const TPM_RIGHTALIGN: i32 = 8;
pub const TPM_RIGHTBUTTON: i32 = 2;
pub const TPM_TOPALIGN: i32 = 0;
pub const TPM_VCENTERALIGN: i32 = 16;
pub const TPM_VERNEGANIMATION: i32 = 8192;
pub const TPM_VERPOSANIMATION: i32 = 4096;
pub const TPM_VERTICAL: i32 = 64;
pub const TPM_WORKAREA: i32 = 65536;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACKMOUSEEVENT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTrack: super::HWND,
    pub dwHoverTime: u32,
}
pub const TWF_FINETOUCH: i32 = 1;
pub const TWF_WANTPALM: i32 = 2;
pub const UISF_ACTIVE: i32 = 4;
pub const UISF_HIDEACCEL: i32 = 2;
pub const UISF_HIDEFOCUS: i32 = 1;
pub const UIS_CLEAR: i32 = 2;
pub const UIS_INITIALIZE: i32 = 3;
pub const UIS_SET: i32 = 1;
pub const ULW_ALPHA: i32 = 2;
pub const ULW_COLORKEY: i32 = 1;
pub const ULW_EX_NORESIZE: i32 = 8;
pub const ULW_OPAQUE: i32 = 4;
pub const UNICODE_NOCHAR: i32 = 65535;
pub const UOI_FLAGS: i32 = 1;
pub const UOI_HEAPSIZE: i32 = 5;
pub const UOI_IO: i32 = 6;
pub const UOI_NAME: i32 = 2;
pub const UOI_TIMERPROC_EXCEPTION_SUPPRESSION: i32 = 7;
pub const UOI_TYPE: i32 = 3;
pub const UOI_USER_SID: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USEROBJECTFLAGS {
    pub fInherit: windows_core::BOOL,
    pub fReserved: windows_core::BOOL,
    pub dwFlags: u32,
}
pub const USER_DEFAULT_SCREEN_DPI: i32 = 96;
pub const USER_TIMER_MAXIMUM: i32 = 2147483647;
pub const USER_TIMER_MINIMUM: i32 = 10;
pub const VK_ACCEPT: i32 = 30;
pub const VK_ADD: i32 = 107;
pub const VK_APPS: i32 = 93;
pub const VK_ATTN: i32 = 246;
pub const VK_BACK: i32 = 8;
pub const VK_BROWSER_BACK: i32 = 166;
pub const VK_BROWSER_FAVORITES: i32 = 171;
pub const VK_BROWSER_FORWARD: i32 = 167;
pub const VK_BROWSER_HOME: i32 = 172;
pub const VK_BROWSER_REFRESH: i32 = 168;
pub const VK_BROWSER_SEARCH: i32 = 170;
pub const VK_BROWSER_STOP: i32 = 169;
pub const VK_CANCEL: i32 = 3;
pub const VK_CAPITAL: i32 = 20;
pub const VK_CLEAR: i32 = 12;
pub const VK_CONTROL: i32 = 17;
pub const VK_CONVERT: i32 = 28;
pub const VK_CRSEL: i32 = 247;
pub const VK_DECIMAL: i32 = 110;
pub const VK_DELETE: i32 = 46;
pub const VK_DIVIDE: i32 = 111;
pub const VK_DOWN: i32 = 40;
pub const VK_END: i32 = 35;
pub const VK_EREOF: i32 = 249;
pub const VK_ESCAPE: i32 = 27;
pub const VK_EXECUTE: i32 = 43;
pub const VK_EXSEL: i32 = 248;
pub const VK_F1: i32 = 112;
pub const VK_F10: i32 = 121;
pub const VK_F11: i32 = 122;
pub const VK_F12: i32 = 123;
pub const VK_F13: i32 = 124;
pub const VK_F14: i32 = 125;
pub const VK_F15: i32 = 126;
pub const VK_F16: i32 = 127;
pub const VK_F17: i32 = 128;
pub const VK_F18: i32 = 129;
pub const VK_F19: i32 = 130;
pub const VK_F2: i32 = 113;
pub const VK_F20: i32 = 131;
pub const VK_F21: i32 = 132;
pub const VK_F22: i32 = 133;
pub const VK_F23: i32 = 134;
pub const VK_F24: i32 = 135;
pub const VK_F3: i32 = 114;
pub const VK_F4: i32 = 115;
pub const VK_F5: i32 = 116;
pub const VK_F6: i32 = 117;
pub const VK_F7: i32 = 118;
pub const VK_F8: i32 = 119;
pub const VK_F9: i32 = 120;
pub const VK_FINAL: i32 = 24;
pub const VK_GAMEPAD_A: i32 = 195;
pub const VK_GAMEPAD_B: i32 = 196;
pub const VK_GAMEPAD_DPAD_DOWN: i32 = 204;
pub const VK_GAMEPAD_DPAD_LEFT: i32 = 205;
pub const VK_GAMEPAD_DPAD_RIGHT: i32 = 206;
pub const VK_GAMEPAD_DPAD_UP: i32 = 203;
pub const VK_GAMEPAD_LEFT_SHOULDER: i32 = 200;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: i32 = 209;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: i32 = 212;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: i32 = 214;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: i32 = 213;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP: i32 = 211;
pub const VK_GAMEPAD_LEFT_TRIGGER: i32 = 201;
pub const VK_GAMEPAD_MENU: i32 = 207;
pub const VK_GAMEPAD_RIGHT_SHOULDER: i32 = 199;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: i32 = 210;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: i32 = 216;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: i32 = 218;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: i32 = 217;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: i32 = 215;
pub const VK_GAMEPAD_RIGHT_TRIGGER: i32 = 202;
pub const VK_GAMEPAD_VIEW: i32 = 208;
pub const VK_GAMEPAD_X: i32 = 197;
pub const VK_GAMEPAD_Y: i32 = 198;
pub const VK_HANGEUL: i32 = 21;
pub const VK_HANGUL: i32 = 21;
pub const VK_HANJA: i32 = 25;
pub const VK_HELP: i32 = 47;
pub const VK_HOME: i32 = 36;
pub const VK_ICO_00: i32 = 228;
pub const VK_ICO_CLEAR: i32 = 230;
pub const VK_ICO_HELP: i32 = 227;
pub const VK_IME_OFF: i32 = 26;
pub const VK_IME_ON: i32 = 22;
pub const VK_INSERT: i32 = 45;
pub const VK_JUNJA: i32 = 23;
pub const VK_KANA: i32 = 21;
pub const VK_KANJI: i32 = 25;
pub const VK_LAUNCH_APP1: i32 = 182;
pub const VK_LAUNCH_APP2: i32 = 183;
pub const VK_LAUNCH_MAIL: i32 = 180;
pub const VK_LAUNCH_MEDIA_SELECT: i32 = 181;
pub const VK_LBUTTON: i32 = 1;
pub const VK_LCONTROL: i32 = 162;
pub const VK_LEFT: i32 = 37;
pub const VK_LMENU: i32 = 164;
pub const VK_LSHIFT: i32 = 160;
pub const VK_LWIN: i32 = 91;
pub const VK_MBUTTON: i32 = 4;
pub const VK_MEDIA_NEXT_TRACK: i32 = 176;
pub const VK_MEDIA_PLAY_PAUSE: i32 = 179;
pub const VK_MEDIA_PREV_TRACK: i32 = 177;
pub const VK_MEDIA_STOP: i32 = 178;
pub const VK_MENU: i32 = 18;
pub const VK_MODECHANGE: i32 = 31;
pub const VK_MULTIPLY: i32 = 106;
pub const VK_NAVIGATION_ACCEPT: i32 = 142;
pub const VK_NAVIGATION_CANCEL: i32 = 143;
pub const VK_NAVIGATION_DOWN: i32 = 139;
pub const VK_NAVIGATION_LEFT: i32 = 140;
pub const VK_NAVIGATION_MENU: i32 = 137;
pub const VK_NAVIGATION_RIGHT: i32 = 141;
pub const VK_NAVIGATION_UP: i32 = 138;
pub const VK_NAVIGATION_VIEW: i32 = 136;
pub const VK_NEXT: i32 = 34;
pub const VK_NONAME: i32 = 252;
pub const VK_NONCONVERT: i32 = 29;
pub const VK_NUMLOCK: i32 = 144;
pub const VK_NUMPAD0: i32 = 96;
pub const VK_NUMPAD1: i32 = 97;
pub const VK_NUMPAD2: i32 = 98;
pub const VK_NUMPAD3: i32 = 99;
pub const VK_NUMPAD4: i32 = 100;
pub const VK_NUMPAD5: i32 = 101;
pub const VK_NUMPAD6: i32 = 102;
pub const VK_NUMPAD7: i32 = 103;
pub const VK_NUMPAD8: i32 = 104;
pub const VK_NUMPAD9: i32 = 105;
pub const VK_OEM_1: i32 = 186;
pub const VK_OEM_102: i32 = 226;
pub const VK_OEM_2: i32 = 191;
pub const VK_OEM_3: i32 = 192;
pub const VK_OEM_4: i32 = 219;
pub const VK_OEM_5: i32 = 220;
pub const VK_OEM_6: i32 = 221;
pub const VK_OEM_7: i32 = 222;
pub const VK_OEM_8: i32 = 223;
pub const VK_OEM_ATTN: i32 = 240;
pub const VK_OEM_AUTO: i32 = 243;
pub const VK_OEM_AX: i32 = 225;
pub const VK_OEM_BACKTAB: i32 = 245;
pub const VK_OEM_CLEAR: i32 = 254;
pub const VK_OEM_COMMA: i32 = 188;
pub const VK_OEM_COPY: i32 = 242;
pub const VK_OEM_CUSEL: i32 = 239;
pub const VK_OEM_ENLW: i32 = 244;
pub const VK_OEM_FINISH: i32 = 241;
pub const VK_OEM_FJ_JISHO: i32 = 146;
pub const VK_OEM_FJ_LOYA: i32 = 149;
pub const VK_OEM_FJ_MASSHOU: i32 = 147;
pub const VK_OEM_FJ_ROYA: i32 = 150;
pub const VK_OEM_FJ_TOUROKU: i32 = 148;
pub const VK_OEM_JUMP: i32 = 234;
pub const VK_OEM_MINUS: i32 = 189;
pub const VK_OEM_NEC_EQUAL: i32 = 146;
pub const VK_OEM_PA1: i32 = 235;
pub const VK_OEM_PA2: i32 = 236;
pub const VK_OEM_PA3: i32 = 237;
pub const VK_OEM_PERIOD: i32 = 190;
pub const VK_OEM_PLUS: i32 = 187;
pub const VK_OEM_RESET: i32 = 233;
pub const VK_OEM_WSCTRL: i32 = 238;
pub const VK_PA1: i32 = 253;
pub const VK_PACKET: i32 = 231;
pub const VK_PAUSE: i32 = 19;
pub const VK_PLAY: i32 = 250;
pub const VK_PRINT: i32 = 42;
pub const VK_PRIOR: i32 = 33;
pub const VK_PROCESSKEY: i32 = 229;
pub const VK_RBUTTON: i32 = 2;
pub const VK_RCONTROL: i32 = 163;
pub const VK_RETURN: i32 = 13;
pub const VK_RIGHT: i32 = 39;
pub const VK_RMENU: i32 = 165;
pub const VK_RSHIFT: i32 = 161;
pub const VK_RWIN: i32 = 92;
pub const VK_SCROLL: i32 = 145;
pub const VK_SELECT: i32 = 41;
pub const VK_SEPARATOR: i32 = 108;
pub const VK_SHIFT: i32 = 16;
pub const VK_SLEEP: i32 = 95;
pub const VK_SNAPSHOT: i32 = 44;
pub const VK_SPACE: i32 = 32;
pub const VK_SUBTRACT: i32 = 109;
pub const VK_TAB: i32 = 9;
pub const VK_UP: i32 = 38;
pub const VK_VOLUME_DOWN: i32 = 174;
pub const VK_VOLUME_MUTE: i32 = 173;
pub const VK_VOLUME_UP: i32 = 175;
pub const VK_XBUTTON1: i32 = 5;
pub const VK_XBUTTON2: i32 = 6;
pub const VK_ZOOM: i32 = 251;
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
pub const WA_ACTIVE: i32 = 1;
pub const WA_CLICKACTIVE: i32 = 2;
pub const WA_INACTIVE: i32 = 0;
pub const WB_ISDELIMITER: i32 = 2;
pub const WB_LEFT: i32 = 0;
pub const WB_RIGHT: i32 = 1;
pub const WDA_EXCLUDEFROMCAPTURE: i32 = 17;
pub const WDA_MONITOR: i32 = 1;
pub const WDA_NONE: i32 = 0;
pub const WHEEL_DELTA: i32 = 120;
pub const WHEEL_PAGESCROLL: u32 = 4294967295;
pub const WH_CALLWNDPROC: i32 = 4;
pub const WH_CALLWNDPROCRET: i32 = 12;
pub const WH_CBT: i32 = 5;
pub const WH_DEBUG: i32 = 9;
pub const WH_FOREGROUNDIDLE: i32 = 11;
pub const WH_GETMESSAGE: i32 = 3;
pub const WH_JOURNALPLAYBACK: i32 = 1;
pub const WH_JOURNALRECORD: i32 = 0;
pub const WH_KEYBOARD: i32 = 2;
pub const WH_KEYBOARD_LL: i32 = 13;
pub const WH_MAX: i32 = 14;
pub const WH_MAXHOOK: i32 = 14;
pub const WH_MIN: i32 = -1;
pub const WH_MINHOOK: i32 = -1;
pub const WH_MOUSE: i32 = 7;
pub const WH_MOUSE_LL: i32 = 14;
pub const WH_MSGFILTER: i32 = -1;
pub const WH_SHELL: i32 = 10;
pub const WH_SYSMSGFILTER: i32 = 6;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINDOWPOS {
    pub hwnd: super::HWND,
    pub hwndInsertAfter: super::HWND,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub flags: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINDOW_ACTION {
    pub kinds: WINDOW_ACTION_KINDS,
    pub modifiers: WINDOW_ACTION_MODIFIERS,
    pub visible: windows_core::BOOL,
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
pub type WINDOW_ACTION_KINDS = u32;
pub type WINDOW_ACTION_MODIFIERS = u32;
pub type WINDOW_PLACEMENT_STATE = i32;
#[cfg(feature = "windef")]
pub type WINEVENTPROC = Option<unsafe extern "system" fn(hwineventhook: super::HWINEVENTHOOK, event: u32, hwnd: super::HWND, idobject: i32, idchild: i32, ideventthread: u32, dwmseventtime: u32)>;
pub const WINEVENT_INCONTEXT: i32 = 4;
pub const WINEVENT_OUTOFCONTEXT: i32 = 0;
pub const WINEVENT_SKIPOWNPROCESS: i32 = 2;
pub const WINEVENT_SKIPOWNTHREAD: i32 = 1;
#[cfg(feature = "minwindef")]
pub type WINSTAENUMPROC = WINSTAENUMPROCA;
#[cfg(feature = "minwindef")]
pub type WINSTAENUMPROCA = NAMEENUMPROCA;
#[cfg(feature = "minwindef")]
pub type WINSTAENUMPROCW = NAMEENUMPROCW;
pub const WINSTA_ACCESSCLIPBOARD: i32 = 4;
pub const WINSTA_ACCESSGLOBALATOMS: i32 = 32;
pub const WINSTA_ALL_ACCESS: i32 = 895;
pub const WINSTA_CREATEDESKTOP: i32 = 8;
pub const WINSTA_ENUMDESKTOPS: i32 = 1;
pub const WINSTA_ENUMERATE: i32 = 256;
pub const WINSTA_EXITWINDOWS: i32 = 64;
pub const WINSTA_READATTRIBUTES: i32 = 2;
pub const WINSTA_READSCREEN: i32 = 512;
pub const WINSTA_WRITEATTRIBUTES: i32 = 16;
pub const WMSZ_BOTTOM: i32 = 6;
pub const WMSZ_BOTTOMLEFT: i32 = 7;
pub const WMSZ_BOTTOMRIGHT: i32 = 8;
pub const WMSZ_LEFT: i32 = 1;
pub const WMSZ_RIGHT: i32 = 2;
pub const WMSZ_TOP: i32 = 3;
pub const WMSZ_TOPLEFT: i32 = 4;
pub const WMSZ_TOPRIGHT: i32 = 5;
pub const WM_ACTIVATE: i32 = 6;
pub const WM_ACTIVATEAPP: i32 = 28;
pub const WM_AFXFIRST: i32 = 864;
pub const WM_AFXLAST: i32 = 895;
pub const WM_APP: i32 = 32768;
pub const WM_APPCOMMAND: i32 = 793;
pub const WM_ASKCBFORMATNAME: i32 = 780;
pub const WM_CANCELJOURNAL: i32 = 75;
pub const WM_CANCELMODE: i32 = 31;
pub const WM_CAPTURECHANGED: i32 = 533;
pub const WM_CHANGECBCHAIN: i32 = 781;
pub const WM_CHANGEUISTATE: i32 = 295;
pub const WM_CHAR: i32 = 258;
pub const WM_CHARTOITEM: i32 = 47;
pub const WM_CHILDACTIVATE: i32 = 34;
pub const WM_CLEAR: i32 = 771;
pub const WM_CLIPBOARDUPDATE: i32 = 797;
pub const WM_CLOAKED_STATE_CHANGED: i32 = 839;
pub const WM_CLOSE: i32 = 16;
pub const WM_COMMAND: i32 = 273;
pub const WM_COMMNOTIFY: i32 = 68;
pub const WM_COMPACTING: i32 = 65;
pub const WM_COMPAREITEM: i32 = 57;
pub const WM_CONTEXTMENU: i32 = 123;
pub const WM_COPY: i32 = 769;
pub const WM_COPYDATA: i32 = 74;
pub const WM_CREATE: i32 = 1;
pub const WM_CTLCOLORBTN: i32 = 309;
pub const WM_CTLCOLORDLG: i32 = 310;
pub const WM_CTLCOLOREDIT: i32 = 307;
pub const WM_CTLCOLORLISTBOX: i32 = 308;
pub const WM_CTLCOLORMSGBOX: i32 = 306;
pub const WM_CTLCOLORSCROLLBAR: i32 = 311;
pub const WM_CTLCOLORSTATIC: i32 = 312;
pub const WM_CUT: i32 = 768;
pub const WM_DEADCHAR: i32 = 259;
pub const WM_DELETEITEM: i32 = 45;
pub const WM_DESTROY: i32 = 2;
pub const WM_DESTROYCLIPBOARD: i32 = 775;
pub const WM_DEVICECHANGE: i32 = 537;
pub const WM_DEVMODECHANGE: i32 = 27;
pub const WM_DISPLAYCHANGE: i32 = 126;
pub const WM_DPICHANGED: i32 = 736;
pub const WM_DPICHANGED_AFTERPARENT: i32 = 739;
pub const WM_DPICHANGED_BEFOREPARENT: i32 = 738;
pub const WM_DRAWCLIPBOARD: i32 = 776;
pub const WM_DRAWITEM: i32 = 43;
pub const WM_DROPFILES: i32 = 563;
pub const WM_DWMCOLORIZATIONCOLORCHANGED: i32 = 800;
pub const WM_DWMCOMPOSITIONCHANGED: i32 = 798;
pub const WM_DWMNCRENDERINGCHANGED: i32 = 799;
pub const WM_DWMSENDICONICLIVEPREVIEWBITMAP: i32 = 806;
pub const WM_DWMSENDICONICTHUMBNAIL: i32 = 803;
pub const WM_DWMWINDOWMAXIMIZEDCHANGE: i32 = 801;
pub const WM_ENABLE: i32 = 10;
pub const WM_ENDSESSION: i32 = 22;
pub const WM_ENTERIDLE: i32 = 289;
pub const WM_ENTERMENULOOP: i32 = 529;
pub const WM_ENTERSIZEMOVE: i32 = 561;
pub const WM_ERASEBKGND: i32 = 20;
pub const WM_EXITMENULOOP: i32 = 530;
pub const WM_EXITSIZEMOVE: i32 = 562;
pub const WM_FONTCHANGE: i32 = 29;
pub const WM_GESTURE: i32 = 281;
pub const WM_GESTURENOTIFY: i32 = 282;
pub const WM_GETDLGCODE: i32 = 135;
pub const WM_GETDPISCALEDSIZE: i32 = 740;
pub const WM_GETFONT: i32 = 49;
pub const WM_GETHOTKEY: i32 = 51;
pub const WM_GETICON: i32 = 127;
pub const WM_GETMINMAXINFO: i32 = 36;
pub const WM_GETOBJECT: i32 = 61;
pub const WM_GETTEXT: i32 = 13;
pub const WM_GETTEXTLENGTH: i32 = 14;
pub const WM_GETTITLEBARINFOEX: i32 = 831;
pub const WM_HANDHELDFIRST: i32 = 856;
pub const WM_HANDHELDLAST: i32 = 863;
pub const WM_HELP: i32 = 83;
pub const WM_HOTKEY: i32 = 786;
pub const WM_HSCROLL: i32 = 276;
pub const WM_HSCROLLCLIPBOARD: i32 = 782;
pub const WM_ICONERASEBKGND: i32 = 39;
pub const WM_IME_CHAR: i32 = 646;
pub const WM_IME_COMPOSITION: i32 = 271;
pub const WM_IME_COMPOSITIONFULL: i32 = 644;
pub const WM_IME_CONTROL: i32 = 643;
pub const WM_IME_ENDCOMPOSITION: i32 = 270;
pub const WM_IME_KEYDOWN: i32 = 656;
pub const WM_IME_KEYLAST: i32 = 271;
pub const WM_IME_KEYUP: i32 = 657;
pub const WM_IME_NOTIFY: i32 = 642;
pub const WM_IME_REQUEST: i32 = 648;
pub const WM_IME_SELECT: i32 = 645;
pub const WM_IME_SETCONTEXT: i32 = 641;
pub const WM_IME_STARTCOMPOSITION: i32 = 269;
pub const WM_INITDIALOG: i32 = 272;
pub const WM_INITMENU: i32 = 278;
pub const WM_INITMENUPOPUP: i32 = 279;
pub const WM_INPUT: i32 = 255;
pub const WM_INPUTLANGCHANGE: i32 = 81;
pub const WM_INPUTLANGCHANGEREQUEST: i32 = 80;
pub const WM_INPUT_DEVICE_CHANGE: i32 = 254;
pub const WM_INTERCEPTED_WINDOW_ACTION: i32 = 838;
pub const WM_KEYDOWN: i32 = 256;
pub const WM_KEYFIRST: i32 = 256;
pub const WM_KEYLAST: i32 = 265;
pub const WM_KEYUP: i32 = 257;
pub const WM_KILLFOCUS: i32 = 8;
pub const WM_LBUTTONDBLCLK: i32 = 515;
pub const WM_LBUTTONDOWN: i32 = 513;
pub const WM_LBUTTONUP: i32 = 514;
pub const WM_MBUTTONDBLCLK: i32 = 521;
pub const WM_MBUTTONDOWN: i32 = 519;
pub const WM_MBUTTONUP: i32 = 520;
pub const WM_MDIACTIVATE: i32 = 546;
pub const WM_MDICASCADE: i32 = 551;
pub const WM_MDICREATE: i32 = 544;
pub const WM_MDIDESTROY: i32 = 545;
pub const WM_MDIGETACTIVE: i32 = 553;
pub const WM_MDIICONARRANGE: i32 = 552;
pub const WM_MDIMAXIMIZE: i32 = 549;
pub const WM_MDINEXT: i32 = 548;
pub const WM_MDIREFRESHMENU: i32 = 564;
pub const WM_MDIRESTORE: i32 = 547;
pub const WM_MDISETMENU: i32 = 560;
pub const WM_MDITILE: i32 = 550;
pub const WM_MEASUREITEM: i32 = 44;
pub const WM_MENUCHAR: i32 = 288;
pub const WM_MENUCOMMAND: i32 = 294;
pub const WM_MENUDRAG: i32 = 291;
pub const WM_MENUGETOBJECT: i32 = 292;
pub const WM_MENURBUTTONUP: i32 = 290;
pub const WM_MENUSELECT: i32 = 287;
pub const WM_MOUSEACTIVATE: i32 = 33;
pub const WM_MOUSEFIRST: i32 = 512;
pub const WM_MOUSEHOVER: i32 = 673;
pub const WM_MOUSEHWHEEL: i32 = 526;
pub const WM_MOUSELAST: i32 = 526;
pub const WM_MOUSELEAVE: i32 = 675;
pub const WM_MOUSEMOVE: i32 = 512;
pub const WM_MOUSEWHEEL: i32 = 522;
pub const WM_MOVE: i32 = 3;
pub const WM_MOVING: i32 = 534;
pub const WM_NCACTIVATE: i32 = 134;
pub const WM_NCCALCSIZE: i32 = 131;
pub const WM_NCCREATE: i32 = 129;
pub const WM_NCDESTROY: i32 = 130;
pub const WM_NCHITTEST: i32 = 132;
pub const WM_NCLBUTTONDBLCLK: i32 = 163;
pub const WM_NCLBUTTONDOWN: i32 = 161;
pub const WM_NCLBUTTONUP: i32 = 162;
pub const WM_NCMBUTTONDBLCLK: i32 = 169;
pub const WM_NCMBUTTONDOWN: i32 = 167;
pub const WM_NCMBUTTONUP: i32 = 168;
pub const WM_NCMOUSEHOVER: i32 = 672;
pub const WM_NCMOUSELEAVE: i32 = 674;
pub const WM_NCMOUSEMOVE: i32 = 160;
pub const WM_NCPAINT: i32 = 133;
pub const WM_NCPOINTERDOWN: i32 = 578;
pub const WM_NCPOINTERUP: i32 = 579;
pub const WM_NCPOINTERUPDATE: i32 = 577;
pub const WM_NCRBUTTONDBLCLK: i32 = 166;
pub const WM_NCRBUTTONDOWN: i32 = 164;
pub const WM_NCRBUTTONUP: i32 = 165;
pub const WM_NCXBUTTONDBLCLK: i32 = 173;
pub const WM_NCXBUTTONDOWN: i32 = 171;
pub const WM_NCXBUTTONUP: i32 = 172;
pub const WM_NEXTDLGCTL: i32 = 40;
pub const WM_NEXTMENU: i32 = 531;
pub const WM_NOTIFY: i32 = 78;
pub const WM_NOTIFYFORMAT: i32 = 85;
pub const WM_NULL: i32 = 0;
pub const WM_PAINT: i32 = 15;
pub const WM_PAINTCLIPBOARD: i32 = 777;
pub const WM_PAINTICON: i32 = 38;
pub const WM_PALETTECHANGED: i32 = 785;
pub const WM_PALETTEISCHANGING: i32 = 784;
pub const WM_PARENTNOTIFY: i32 = 528;
pub const WM_PASTE: i32 = 770;
pub const WM_PENWINFIRST: i32 = 896;
pub const WM_PENWINLAST: i32 = 911;
pub const WM_POINTERACTIVATE: i32 = 587;
pub const WM_POINTERCAPTURECHANGED: i32 = 588;
pub const WM_POINTERDEVICECHANGE: i32 = 568;
pub const WM_POINTERDEVICEINRANGE: i32 = 569;
pub const WM_POINTERDEVICEOUTOFRANGE: i32 = 570;
pub const WM_POINTERDOWN: i32 = 582;
pub const WM_POINTERENTER: i32 = 585;
pub const WM_POINTERHWHEEL: i32 = 591;
pub const WM_POINTERLEAVE: i32 = 586;
pub const WM_POINTERROUTEDAWAY: i32 = 594;
pub const WM_POINTERROUTEDRELEASED: i32 = 595;
pub const WM_POINTERROUTEDTO: i32 = 593;
pub const WM_POINTERUP: i32 = 583;
pub const WM_POINTERUPDATE: i32 = 581;
pub const WM_POINTERWHEEL: i32 = 590;
pub const WM_POWER: i32 = 72;
pub const WM_POWERBROADCAST: i32 = 536;
pub const WM_PRINT: i32 = 791;
pub const WM_PRINTCLIENT: i32 = 792;
pub const WM_QUERYDRAGICON: i32 = 55;
pub const WM_QUERYENDSESSION: i32 = 17;
pub const WM_QUERYNEWPALETTE: i32 = 783;
pub const WM_QUERYOPEN: i32 = 19;
pub const WM_QUERYUISTATE: i32 = 297;
pub const WM_QUEUESYNC: i32 = 35;
pub const WM_QUIT: i32 = 18;
pub const WM_RBUTTONDBLCLK: i32 = 518;
pub const WM_RBUTTONDOWN: i32 = 516;
pub const WM_RBUTTONUP: i32 = 517;
pub const WM_RENDERALLFORMATS: i32 = 774;
pub const WM_RENDERFORMAT: i32 = 773;
pub const WM_SETCURSOR: i32 = 32;
pub const WM_SETFOCUS: i32 = 7;
pub const WM_SETFONT: i32 = 48;
pub const WM_SETHOTKEY: i32 = 50;
pub const WM_SETICON: i32 = 128;
pub const WM_SETREDRAW: i32 = 11;
pub const WM_SETTEXT: i32 = 12;
pub const WM_SETTINGCHANGE: i32 = 26;
pub const WM_SHOWWINDOW: i32 = 24;
pub const WM_SIZE: i32 = 5;
pub const WM_SIZECLIPBOARD: i32 = 779;
pub const WM_SIZING: i32 = 532;
pub const WM_SPOOLERSTATUS: i32 = 42;
pub const WM_STYLECHANGED: i32 = 125;
pub const WM_STYLECHANGING: i32 = 124;
pub const WM_SYNCPAINT: i32 = 136;
pub const WM_SYSCHAR: i32 = 262;
pub const WM_SYSCOLORCHANGE: i32 = 21;
pub const WM_SYSCOMMAND: i32 = 274;
pub const WM_SYSDEADCHAR: i32 = 263;
pub const WM_SYSKEYDOWN: i32 = 260;
pub const WM_SYSKEYUP: i32 = 261;
pub const WM_TABLET_FIRST: i32 = 704;
pub const WM_TABLET_LAST: i32 = 735;
pub const WM_TCARD: i32 = 82;
pub const WM_THEMECHANGED: i32 = 794;
pub const WM_TIMECHANGE: i32 = 30;
pub const WM_TIMER: i32 = 275;
pub const WM_TOOLTIPDISMISS: i32 = 837;
pub const WM_TOUCH: i32 = 576;
pub const WM_TOUCHHITTESTING: i32 = 589;
pub const WM_UNDO: i32 = 772;
pub const WM_UNICHAR: i32 = 265;
pub const WM_UNINITMENUPOPUP: i32 = 293;
pub const WM_UPDATEUISTATE: i32 = 296;
pub const WM_USER: i32 = 1024;
pub const WM_USERCHANGED: i32 = 84;
pub const WM_VKEYTOITEM: i32 = 46;
pub const WM_VSCROLL: i32 = 277;
pub const WM_VSCROLLCLIPBOARD: i32 = 778;
pub const WM_WINDOWPOSCHANGED: i32 = 71;
pub const WM_WINDOWPOSCHANGING: i32 = 70;
pub const WM_WININICHANGE: i32 = 26;
pub const WM_WTSSESSION_CHANGE: i32 = 689;
pub const WM_XBUTTONDBLCLK: i32 = 525;
pub const WM_XBUTTONDOWN: i32 = 523;
pub const WM_XBUTTONUP: i32 = 524;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type WNDCLASS = WNDCLASSA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct WNDCLASSA {
    pub style: u32,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::HINSTANCE,
    pub hIcon: super::HICON,
    pub hCursor: super::HCURSOR,
    pub hbrBackground: super::HBRUSH,
    pub lpszMenuName: windows_core::PCSTR,
    pub lpszClassName: windows_core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type WNDCLASSEX = WNDCLASSEXA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
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
    pub lpszMenuName: windows_core::PCSTR,
    pub lpszClassName: windows_core::PCSTR,
    pub hIconSm: super::HICON,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
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
    pub lpszMenuName: windows_core::PCWSTR,
    pub lpszClassName: windows_core::PCWSTR,
    pub hIconSm: super::HICON,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct WNDCLASSW {
    pub style: u32,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: super::HINSTANCE,
    pub hIcon: super::HICON,
    pub hCursor: super::HCURSOR,
    pub hbrBackground: super::HBRUSH,
    pub lpszMenuName: windows_core::PCWSTR,
    pub lpszClassName: windows_core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type WNDENUMPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: super::LPARAM) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type WNDPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: u32, param2: super::WPARAM, param3: super::LPARAM) -> super::LRESULT>;
pub const WPF_ASYNCWINDOWPLACEMENT: i32 = 4;
pub const WPF_RESTORETOMAXIMIZED: i32 = 2;
pub const WPF_SETMINPOSITION: i32 = 1;
pub const WPS_ARRANGED: WINDOW_PLACEMENT_STATE = 3;
pub const WPS_MAXIMIZED: WINDOW_PLACEMENT_STATE = 1;
pub const WPS_MINIMIZED: WINDOW_PLACEMENT_STATE = 2;
pub const WPS_NORMAL: WINDOW_PLACEMENT_STATE = 0;
pub const WSF_VISIBLE: i32 = 1;
pub const WS_ACTIVECAPTION: i32 = 1;
pub const WS_BORDER: i32 = 8388608;
pub const WS_CAPTION: i32 = 12582912;
pub const WS_CHILD: i32 = 1073741824;
pub const WS_CHILDWINDOW: i32 = 1073741824;
pub const WS_CLIPCHILDREN: i32 = 33554432;
pub const WS_CLIPSIBLINGS: i32 = 67108864;
pub const WS_DISABLED: i32 = 134217728;
pub const WS_DLGFRAME: i32 = 4194304;
pub const WS_EX_ACCEPTFILES: i32 = 16;
pub const WS_EX_APPWINDOW: i32 = 262144;
pub const WS_EX_CLIENTEDGE: i32 = 512;
pub const WS_EX_COMPOSITED: i32 = 33554432;
pub const WS_EX_CONTEXTHELP: i32 = 1024;
pub const WS_EX_CONTROLPARENT: i32 = 65536;
pub const WS_EX_DLGMODALFRAME: i32 = 1;
pub const WS_EX_LAYERED: i32 = 524288;
pub const WS_EX_LAYOUTRTL: i32 = 4194304;
pub const WS_EX_LEFT: i32 = 0;
pub const WS_EX_LEFTSCROLLBAR: i32 = 16384;
pub const WS_EX_LTRREADING: i32 = 0;
pub const WS_EX_MDICHILD: i32 = 64;
pub const WS_EX_NOACTIVATE: i32 = 134217728;
pub const WS_EX_NOINHERITLAYOUT: i32 = 1048576;
pub const WS_EX_NOPARENTNOTIFY: i32 = 4;
pub const WS_EX_NOREDIRECTIONBITMAP: i32 = 2097152;
pub const WS_EX_OVERLAPPEDWINDOW: i32 = 768;
pub const WS_EX_PALETTEWINDOW: i32 = 392;
pub const WS_EX_RIGHT: i32 = 4096;
pub const WS_EX_RIGHTSCROLLBAR: i32 = 0;
pub const WS_EX_RTLREADING: i32 = 8192;
pub const WS_EX_STATICEDGE: i32 = 131072;
pub const WS_EX_TOOLWINDOW: i32 = 128;
pub const WS_EX_TOPMOST: i32 = 8;
pub const WS_EX_TRANSPARENT: i32 = 32;
pub const WS_EX_WINDOWEDGE: i32 = 256;
pub const WS_GROUP: i32 = 131072;
pub const WS_HSCROLL: i32 = 1048576;
pub const WS_ICONIC: i32 = 536870912;
pub const WS_MAXIMIZE: i32 = 16777216;
pub const WS_MAXIMIZEBOX: i32 = 65536;
pub const WS_MINIMIZE: i32 = 536870912;
pub const WS_MINIMIZEBOX: i32 = 131072;
pub const WS_OVERLAPPED: i32 = 0;
pub const WS_OVERLAPPEDWINDOW: i32 = 13565952;
pub const WS_POPUP: u32 = 2147483648;
pub const WS_POPUPWINDOW: u32 = 2156396544;
pub const WS_SIZEBOX: i32 = 262144;
pub const WS_SYSMENU: i32 = 524288;
pub const WS_TABSTOP: i32 = 65536;
pub const WS_THICKFRAME: i32 = 262144;
pub const WS_TILED: i32 = 0;
pub const WS_TILEDWINDOW: i32 = 13565952;
pub const WS_VISIBLE: i32 = 268435456;
pub const WS_VSCROLL: i32 = 2097152;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTSSESSION_NOTIFICATION {
    pub cbSize: u32,
    pub dwSessionId: u32,
}
pub const WTS_CONSOLE_CONNECT: i32 = 1;
pub const WTS_CONSOLE_DISCONNECT: i32 = 2;
pub const WTS_REMOTE_CONNECT: i32 = 3;
pub const WTS_REMOTE_DISCONNECT: i32 = 4;
pub const WTS_SESSION_CREATE: i32 = 10;
pub const WTS_SESSION_DESKTOP_READY: i32 = 15;
pub const WTS_SESSION_LOCK: i32 = 7;
pub const WTS_SESSION_LOGOFF: i32 = 6;
pub const WTS_SESSION_LOGON: i32 = 5;
pub const WTS_SESSION_REMOTE_CONTROL: i32 = 9;
pub const WTS_SESSION_TERMINATE: i32 = 11;
pub const WTS_SESSION_UNLOCK: i32 = 8;
pub const WVR_ALIGNBOTTOM: i32 = 64;
pub const WVR_ALIGNLEFT: i32 = 32;
pub const WVR_ALIGNRIGHT: i32 = 128;
pub const WVR_ALIGNTOP: i32 = 16;
pub const WVR_HREDRAW: i32 = 256;
pub const WVR_REDRAW: i32 = 768;
pub const WVR_VALIDRECTS: i32 = 1024;
pub const WVR_VREDRAW: i32 = 512;
pub const XBUTTON1: i32 = 1;
pub const XBUTTON2: i32 = 2;
pub type tagPOINTER_INPUT_TYPE = i32;
