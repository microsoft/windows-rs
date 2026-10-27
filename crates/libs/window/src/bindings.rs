windows_core::link!("user32.dll" "system" fn AdjustWindowRectExForDpi(lprect : LPRECT, dwstyle : DWORD, bmenu : windows_core::BOOL, dwexstyle : DWORD, dpi : UINT) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn CreateWindowExW(dwexstyle : DWORD, lpclassname : windows_core::PCWSTR, lpwindowname : windows_core::PCWSTR, dwstyle : DWORD, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : HWND, hmenu : HMENU, hinstance : HINSTANCE, lpparam : LPVOID) -> HWND);
windows_core::link!("user32.dll" "system" fn DefWindowProcW(hwnd : HWND, msg : UINT, wparam : WPARAM, lparam : LPARAM) -> LRESULT);
windows_core::link!("user32.dll" "system" fn DestroyWindow(hwnd : HWND) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn DispatchMessageW(lpmsg : *const MSG) -> LRESULT);
windows_core::link!("user32.dll" "system" fn GetClientRect(hwnd : HWND, lprect : LPRECT) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn GetDpiForSystem() -> UINT);
windows_core::link!("user32.dll" "system" fn GetMessageW(lpmsg : LPMSG, hwnd : HWND, wmsgfiltermin : UINT, wmsgfiltermax : UINT) -> windows_core::BOOL);
#[cfg(target_arch = "x86")]
windows_core::link!("user32.dll" "system" "GetWindowLongW" fn GetWindowLongPtrW(hwnd : HWND, nindex : i32) -> LONG);
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
windows_core::link!("user32.dll" "system" fn GetWindowLongPtrW(hwnd : HWND, nindex : i32) -> LONG_PTR);
windows_core::link!("user32.dll" "system" fn LoadCursorW(hinstance : HINSTANCE, lpcursorname : windows_core::PCWSTR) -> HCURSOR);
windows_core::link!("user32.dll" "system" fn PeekMessageW(lpmsg : LPMSG, hwnd : HWND, wmsgfiltermin : UINT, wmsgfiltermax : UINT, wremovemsg : UINT) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn PostQuitMessage(nexitcode : i32));
windows_core::link!("user32.dll" "system" fn RegisterClassW(lpwndclass : *const WNDCLASSW) -> ATOM);
windows_core::link!("user32.dll" "system" fn SendMessageW(hwnd : HWND, msg : UINT, wparam : WPARAM, lparam : LPARAM) -> LRESULT);
windows_core::link!("user32.dll" "system" fn SetProcessDpiAwarenessContext(value : DPI_AWARENESS_CONTEXT) -> windows_core::BOOL);
#[cfg(target_arch = "x86")]
windows_core::link!("user32.dll" "system" "SetWindowLongW" fn SetWindowLongPtrW(hwnd : HWND, nindex : i32, dwnewlong : LONG) -> LONG);
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
windows_core::link!("user32.dll" "system" fn SetWindowLongPtrW(hwnd : HWND, nindex : i32, dwnewlong : LONG_PTR) -> LONG_PTR);
windows_core::link!("user32.dll" "system" fn ShowWindow(hwnd : HWND, ncmdshow : i32) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn TranslateMessage(lpmsg : *const MSG) -> windows_core::BOOL);
pub type ATOM = u16;
pub const CS_HREDRAW: i32 = 2;
pub const CS_VREDRAW: i32 = 1;
pub const CW_USEDEFAULT: i32 = -2147483648;
pub type DPI_AWARENESS_CONTEXT = *mut DPI_AWARENESS_CONTEXT__;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = -4 as _;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DPI_AWARENESS_CONTEXT__ {
    pub unused: i32,
}
pub type DWORD = u32;
pub const GWLP_USERDATA: i32 = -21;
pub type HBRUSH = *mut HBRUSH__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HBRUSH__ {
    pub unused: i32,
}
pub type HCURSOR = *mut HICON__;
pub type HICON = *mut HICON__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HICON__ {
    pub unused: i32,
}
pub type HINSTANCE = *mut HINSTANCE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HINSTANCE__ {
    pub unused: i32,
}
pub type HMENU = *mut HMENU__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HMENU__ {
    pub unused: i32,
}
pub type HWND = *mut HWND__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HWND__ {
    pub unused: i32,
}
pub const IDC_ARROW: windows_core::PCSTR = windows_core::PCSTR(32512 as _);
pub type LONG = i32;
#[allow(dead_code)]
pub type LONG_PTR = isize;
pub type LPARAM = isize;
pub type LPMSG = *mut MSG;
pub type LPRECT = *mut RECT;
pub type LPVOID = *mut core::ffi::c_void;
pub type LRESULT = isize;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
}
pub const PM_REMOVE: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}
pub const SW_SHOWNORMAL: i32 = 1;
pub type UINT = u32;
pub const WM_CLOSE: i32 = 16;
pub const WM_DESTROY: i32 = 2;
pub const WM_MOVE: i32 = 3;
pub const WM_NCDESTROY: i32 = 130;
pub const WM_QUIT: i32 = 18;
pub const WM_SIZE: i32 = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WNDCLASSW {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: windows_core::PCWSTR,
    pub lpszClassName: windows_core::PCWSTR,
}
pub type WNDPROC = Option<
    unsafe extern "system" fn(
        param0: HWND,
        param1: UINT,
        param2: WPARAM,
        param3: LPARAM,
    ) -> LRESULT,
>;
pub type WPARAM = usize;
pub const WS_EX_NOREDIRECTIONBITMAP: i32 = 2097152;
pub const WS_OVERLAPPEDWINDOW: i32 = 13565952;
pub const WS_VISIBLE: i32 = 268435456;
