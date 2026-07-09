#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn DdeSetQualityOfService(hwndclient: super::windef::HWND, pqosnew: *const super::winnt::SECURITY_QUALITY_OF_SERVICE, pqosprev: *mut super::winnt::SECURITY_QUALITY_OF_SERVICE) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DdeSetQualityOfService(hwndclient : super::windef::HWND, pqosnew : *const super::winnt::SECURITY_QUALITY_OF_SERVICE, pqosprev : *mut super::winnt::SECURITY_QUALITY_OF_SERVICE) -> windows_core::BOOL);
    unsafe { DdeSetQualityOfService(hwndclient, pqosnew, pqosprev as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FreeDDElParam(msg: u32, lparam: super::minwindef::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn FreeDDElParam(msg : u32, lparam : super::minwindef::LPARAM) -> windows_core::BOOL);
    unsafe { FreeDDElParam(msg, lparam) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImpersonateDdeClientWindow(hwndclient: super::windef::HWND, hwndserver: super::windef::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ImpersonateDdeClientWindow(hwndclient : super::windef::HWND, hwndserver : super::windef::HWND) -> windows_core::BOOL);
    unsafe { ImpersonateDdeClientWindow(hwndclient, hwndserver) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PackDDElParam(msg: u32, uilo: usize, uihi: usize) -> super::minwindef::LPARAM {
    windows_core::link!("user32.dll" "system" fn PackDDElParam(msg : u32, uilo : usize, uihi : usize) -> super::minwindef::LPARAM);
    unsafe { PackDDElParam(msg, uilo, uihi) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ReuseDDElParam(lparam: super::minwindef::LPARAM, msgin: u32, msgout: u32, uilo: usize, uihi: usize) -> super::minwindef::LPARAM {
    windows_core::link!("user32.dll" "system" fn ReuseDDElParam(lparam : super::minwindef::LPARAM, msgin : u32, msgout : u32, uilo : usize, uihi : usize) -> super::minwindef::LPARAM);
    unsafe { ReuseDDElParam(lparam, msgin, msgout, uilo, uihi) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn UnpackDDElParam(msg: u32, lparam: super::minwindef::LPARAM, puilo: *mut u32, puihi: *mut u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnpackDDElParam(msg : u32, lparam : super::minwindef::LPARAM, puilo : *mut u32, puihi : *mut u32) -> windows_core::BOOL);
    unsafe { UnpackDDElParam(msg, lparam, puilo as _, puihi as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn UnpackDDElParam(msg: u32, lparam: super::minwindef::LPARAM, puilo: *mut u64, puihi: *mut u64) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnpackDDElParam(msg : u32, lparam : super::minwindef::LPARAM, puilo : *mut u64, puihi : *mut u64) -> windows_core::BOOL);
    unsafe { UnpackDDElParam(msg, lparam, puilo as _, puihi as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DDEACK {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DDEADVISE {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DDELN {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
