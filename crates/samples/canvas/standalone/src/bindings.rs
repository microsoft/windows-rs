windows_core::link!("user32.dll" "system" fn CreateWindowExA(dwexstyle : WINDOW_EX_STYLE, lpclassname : windows_core::PCSTR, lpwindowname : windows_core::PCSTR, dwstyle : WINDOW_STYLE, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : HWND, hmenu : HMENU, hinstance : HINSTANCE, lpparam : *const core::ffi::c_void) -> HWND);
windows_core::link!("user32.dll" "system" fn DefWindowProcA(hwnd : HWND, msg : u32, wparam : WPARAM, lparam : LPARAM) -> LRESULT);
windows_core::link!("user32.dll" "system" fn DispatchMessageA(lpmsg : *const MSG) -> LRESULT);
windows_core::link!("user32.dll" "system" fn PeekMessageA(lpmsg : *mut MSG, hwnd : HWND, wmsgfiltermin : u32, wmsgfiltermax : u32, wremovemsg : PEEK_MESSAGE_REMOVE_TYPE) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn PostQuitMessage(nexitcode : i32));
windows_core::link!("user32.dll" "system" fn RegisterClassA(lpwndclass : *const WNDCLASSA) -> u16);
windows_core::link!("user32.dll" "system" fn TranslateMessage(lpmsg : *const MSG) -> windows_core::BOOL);
pub const CS_HREDRAW: WNDCLASS_STYLES = 2u32;
pub const CS_VREDRAW: WNDCLASS_STYLES = 1u32;
pub const CW_USEDEFAULT: i32 = -2147483648i32;
pub type HBRUSH = *mut core::ffi::c_void;
pub type HCURSOR = *mut core::ffi::c_void;
pub type HICON = *mut core::ffi::c_void;
pub type HINSTANCE = *mut core::ffi::c_void;
pub type HMENU = *mut core::ffi::c_void;
pub type HWND = *mut core::ffi::c_void;
pub type LPARAM = isize;
pub type LRESULT = isize;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}
pub type PEEK_MESSAGE_REMOVE_TYPE = u32;
pub const PM_REMOVE: PEEK_MESSAGE_REMOVE_TYPE = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
pub type WINDOW_EX_STYLE = u32;
pub type WINDOW_STYLE = u32;
pub const WM_DESTROY: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WNDCLASSA {
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: windows_core::PCSTR,
    pub lpszClassName: windows_core::PCSTR,
}
pub type WNDCLASS_STYLES = u32;
pub type WNDPROC = Option<
    unsafe extern "system" fn(param0: HWND, param1: u32, param2: WPARAM, param3: LPARAM) -> LRESULT,
>;
pub type WPARAM = usize;
pub const WS_OVERLAPPEDWINDOW: WINDOW_STYLE = 13565952u32;
pub const WS_VISIBLE: WINDOW_STYLE = 268435456u32;
