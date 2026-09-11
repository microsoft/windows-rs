windows_core::link!("user32.dll" "system" fn AppendMenuW(hmenu : HMENU, uflags : u32, uidnewitem : usize, lpnewitem : windows_core::PCWSTR) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn CreatePopupMenu() -> HMENU);
windows_core::link!("user32.dll" "system" fn DestroyIcon(hicon : HICON) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn DestroyMenu(hmenu : HMENU) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn GetMonitorInfoW(hmonitor : HMONITOR, lpmi : *mut MONITORINFO) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn GetSystemMetrics(nindex : i32) -> i32);
windows_core::link!("user32.dll" "system" fn KillTimer(hwnd : HWND, uidevent : usize) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn LoadImageW(hinst : HINSTANCE, name : windows_core::PCWSTR, r#type : u32, cx : i32, cy : i32, fuload : u32) -> HANDLE);
windows_core::link!("user32.dll" "system" fn MonitorFromPoint(pt : POINT, dwflags : u32) -> HMONITOR);
windows_core::link!("user32.dll" "system" fn PhysicalToLogicalPointForPerMonitorDPI(hwnd : HWND, lppoint : *mut POINT) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn PostMessageW(hwnd : HWND, msg : u32, wparam : WPARAM, lparam : LPARAM) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn RegisterWindowMessageW(lpstring : windows_core::PCWSTR) -> u32);
windows_core::link!("user32.dll" "system" fn SetForegroundWindow(hwnd : HWND) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn SetThreadDpiAwarenessContext(dpicontext : DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS_CONTEXT);
windows_core::link!("user32.dll" "system" fn SetTimer(hwnd : HWND, nidevent : usize, uelapse : u32, lptimerfunc : TIMERPROC) -> usize);
windows_core::link!("shell32.dll" "system" fn Shell_NotifyIconGetRect(identifier : *const NOTIFYICONIDENTIFIER, iconlocation : *mut RECT) -> windows_core::HRESULT);
windows_core::link!("shell32.dll" "system" fn Shell_NotifyIconW(dwmessage : u32, lpdata : *const NOTIFYICONDATAW) -> windows_core::BOOL);
windows_core::link!("user32.dll" "system" fn TrackPopupMenu(hmenu : HMENU, uflags : u32, x : i32, y : i32, nreserved : i32, hwnd : HWND, prcrect : *const RECT) -> windows_core::BOOL);
pub type DPI_AWARENESS_CONTEXT = *mut core::ffi::c_void;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = -4 as _;
pub type HANDLE = *mut core::ffi::c_void;
pub type HICON = *mut core::ffi::c_void;
pub type HINSTANCE = *mut core::ffi::c_void;
pub type HMENU = *mut core::ffi::c_void;
pub type HMONITOR = *mut core::ffi::c_void;
pub type HWND = *mut core::ffi::c_void;
pub const IMAGE_ICON: i32 = 1;
pub type LPARAM = isize;
pub const LR_LOADFROMFILE: i32 = 16;
pub const MF_SEPARATOR: i32 = 2048;
pub const MF_STRING: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: RECT,
    pub rcWork: RECT,
    pub dwFlags: u32,
}
pub const MONITOR_DEFAULTTONEAREST: i32 = 2;
pub const NIF_ICON: i32 = 2;
pub const NIF_MESSAGE: i32 = 1;
pub const NIF_SHOWTIP: i32 = 128;
pub const NIF_TIP: i32 = 4;
pub const NIM_ADD: i32 = 0;
pub const NIM_DELETE: i32 = 2;
pub const NIM_MODIFY: i32 = 1;
pub const NIM_SETVERSION: i32 = 4;
pub const NIN_KEYSELECT: i32 = 1025;
pub const NIN_SELECT: i32 = 1024;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: HICON,
    pub szTip: [u16; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u16; 256],
    pub Anonymous: NOTIFYICONDATAW_0,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: u32,
    pub guidItem: windows_core::GUID,
    pub hBalloonIcon: HICON,
}
#[cfg(target_arch = "x86")]
impl Default for NOTIFYICONDATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union NOTIFYICONDATAW_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(target_arch = "x86")]
impl Default for NOTIFYICONDATAW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: HICON,
    pub szTip: [u16; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u16; 256],
    pub Anonymous: NOTIFYICONDATAW_0,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: u32,
    pub guidItem: windows_core::GUID,
    pub hBalloonIcon: HICON,
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for NOTIFYICONDATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub union NOTIFYICONDATAW_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for NOTIFYICONDATAW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct NOTIFYICONIDENTIFIER {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub guidItem: windows_core::GUID,
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NOTIFYICONIDENTIFIER {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub guidItem: windows_core::GUID,
}
pub const NOTIFYICON_VERSION_4: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
pub const SM_CXSMICON: i32 = 49;
pub const SM_CYSMICON: i32 = 50;
pub type TIMERPROC =
    Option<unsafe extern "system" fn(param0: HWND, param1: u32, param2: usize, param3: u32)>;
pub const TPM_BOTTOMALIGN: i32 = 32;
pub const TPM_LEFTALIGN: i32 = 0;
pub const TPM_RETURNCMD: i32 = 256;
pub const TPM_RIGHTALIGN: i32 = 8;
pub const TPM_RIGHTBUTTON: i32 = 2;
pub const TPM_TOPALIGN: i32 = 0;
pub const WM_CONTEXTMENU: i32 = 123;
pub const WM_NULL: i32 = 0;
pub const WM_TIMER: i32 = 275;
pub const WM_USER: i32 = 1024;
pub type WPARAM = usize;
