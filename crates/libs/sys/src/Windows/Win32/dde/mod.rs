#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("user32.dll" "system" fn DdeSetQualityOfService(hwndclient : super::HWND, pqosnew : *const super::SECURITY_QUALITY_OF_SERVICE, pqosprev : super::PSECURITY_QUALITY_OF_SERVICE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("user32.dll" "system" fn FreeDDElParam(msg : super::UINT, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("user32.dll" "system" fn ImpersonateDdeClientWindow(hwndclient : super::HWND, hwndserver : super::HWND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("user32.dll" "system" fn PackDDElParam(msg : super::UINT, uilo : super::UINT_PTR, uihi : super::UINT_PTR) -> super::LPARAM);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("user32.dll" "system" fn ReuseDDElParam(lparam : super::LPARAM, msgin : super::UINT, msgout : super::UINT, uilo : super::UINT_PTR, uihi : super::UINT_PTR) -> super::LPARAM);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared"))]
windows_link::link!("user32.dll" "system" fn UnpackDDElParam(msg : super::UINT, lparam : super::LPARAM, puilo : super::PUINT_PTR, puihi : super::PUINT_PTR) -> windows_sys::core::BOOL);
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDEDATA {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDEPOKE {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for DDEPOKE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDEUP {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub rgb: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for DDEUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WM_DDE_ACK: i32 = 996;
pub const WM_DDE_ADVISE: i32 = 994;
pub const WM_DDE_DATA: i32 = 997;
pub const WM_DDE_EXECUTE: i32 = 1000;
pub const WM_DDE_FIRST: i32 = 992;
pub const WM_DDE_INITIATE: i32 = 992;
pub const WM_DDE_LAST: i32 = 1000;
pub const WM_DDE_POKE: i32 = 999;
pub const WM_DDE_REQUEST: i32 = 998;
pub const WM_DDE_TERMINATE: i32 = 993;
pub const WM_DDE_UNADVISE: i32 = 995;
