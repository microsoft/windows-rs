#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("user32.dll" "system" fn DdeSetQualityOfService(hwndclient : super::windef::HWND, pqosnew : *const super::winnt::SECURITY_QUALITY_OF_SERVICE, pqosprev : *mut super::winnt::SECURITY_QUALITY_OF_SERVICE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("user32.dll" "system" fn FreeDDElParam(msg : u32, lparam : super::minwindef::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windef")]
windows_link::link!("user32.dll" "system" fn ImpersonateDdeClientWindow(hwndclient : super::windef::HWND, hwndserver : super::windef::HWND) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("user32.dll" "system" fn PackDDElParam(msg : u32, uilo : usize, uihi : usize) -> super::minwindef::LPARAM);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("user32.dll" "system" fn ReuseDDElParam(lparam : super::minwindef::LPARAM, msgin : u32, msgout : u32, uilo : usize, uihi : usize) -> super::minwindef::LPARAM);
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("user32.dll" "system" fn UnpackDDElParam(msg : u32, lparam : super::minwindef::LPARAM, puilo : *mut u32, puihi : *mut u32) -> windows_sys::core::BOOL);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("user32.dll" "system" fn UnpackDDElParam(msg : u32, lparam : super::minwindef::LPARAM, puilo : *mut u64, puihi : *mut u64) -> windows_sys::core::BOOL);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DDEACK {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DDEADVISE {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDEDATA {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [u8; 1],
}
impl Default for DDEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DDELN {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDEPOKE {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [u8; 1],
}
impl Default for DDEPOKE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDEUP {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub rgb: [u8; 1],
}
impl Default for DDEUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WM_DDE_ACK: u32 = 996;
pub const WM_DDE_ADVISE: u32 = 994;
pub const WM_DDE_DATA: u32 = 997;
pub const WM_DDE_EXECUTE: u32 = 1000;
pub const WM_DDE_FIRST: u32 = 992;
pub const WM_DDE_INITIATE: u32 = 992;
pub const WM_DDE_LAST: u32 = 1000;
pub const WM_DDE_POKE: u32 = 999;
pub const WM_DDE_REQUEST: u32 = 998;
pub const WM_DDE_TERMINATE: u32 = 993;
pub const WM_DDE_UNADVISE: u32 = 995;
