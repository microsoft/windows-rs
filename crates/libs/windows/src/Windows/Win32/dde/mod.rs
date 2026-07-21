#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn DdeSetQualityOfService(hwndclient: super::HWND, pqosnew: *const super::SECURITY_QUALITY_OF_SERVICE, pqosprev: *mut super::SECURITY_QUALITY_OF_SERVICE) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DdeSetQualityOfService(hwndclient : super::HWND, pqosnew : *const super::SECURITY_QUALITY_OF_SERVICE, pqosprev : *mut super::SECURITY_QUALITY_OF_SERVICE) -> windows_core::BOOL);
    unsafe { DdeSetQualityOfService(hwndclient, pqosnew, pqosprev as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FreeDDElParam(msg: u32, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn FreeDDElParam(msg : u32, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { FreeDDElParam(msg, lparam) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImpersonateDdeClientWindow(hwndclient: super::HWND, hwndserver: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ImpersonateDdeClientWindow(hwndclient : super::HWND, hwndserver : super::HWND) -> windows_core::BOOL);
    unsafe { ImpersonateDdeClientWindow(hwndclient, hwndserver) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PackDDElParam(msg: u32, uilo: usize, uihi: usize) -> super::LPARAM {
    windows_core::link!("user32.dll" "system" fn PackDDElParam(msg : u32, uilo : usize, uihi : usize) -> super::LPARAM);
    unsafe { PackDDElParam(msg, uilo, uihi) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ReuseDDElParam(lparam: super::LPARAM, msgin: u32, msgout: u32, uilo: usize, uihi: usize) -> super::LPARAM {
    windows_core::link!("user32.dll" "system" fn ReuseDDElParam(lparam : super::LPARAM, msgin : u32, msgout : u32, uilo : usize, uihi : usize) -> super::LPARAM);
    unsafe { ReuseDDElParam(lparam, msgin, msgout, uilo, uihi) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn UnpackDDElParam(msg: u32, lparam: super::LPARAM, puilo: *mut u32, puihi: *mut u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnpackDDElParam(msg : u32, lparam : super::LPARAM, puilo : *mut u32, puihi : *mut u32) -> windows_core::BOOL);
    unsafe { UnpackDDElParam(msg, lparam, puilo as _, puihi as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn UnpackDDElParam(msg: u32, lparam: super::LPARAM, puilo: *mut u64, puihi: *mut u64) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnpackDDElParam(msg : u32, lparam : super::LPARAM, puilo : *mut u64, puihi : *mut u64) -> windows_core::BOOL);
    unsafe { UnpackDDElParam(msg, lparam, puilo as _, puihi as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DDEACK {
    pub _bitfield: u16,
}
impl DDEACK {
    pub fn bAppReturnCode(&self) -> u16 {
        (self._bitfield << 8) >> 8
    }
    pub fn set_bAppReturnCode(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn reserved(&self) -> u16 {
        (self._bitfield << 2) >> 10
    }
    pub fn set_reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(63 << 8)) | ((value & 63) << 8);
    }
    pub fn fBusy(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_fBusy(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u16) << 14);
    }
    pub fn fAck(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_fAck(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DDEADVISE {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
impl DDEADVISE {
    pub fn reserved(&self) -> u16 {
        (self._bitfield << 2) >> 2
    }
    pub fn set_reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !16383) | (value & 16383);
    }
    pub fn fDeferUpd(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_fDeferUpd(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u16) << 14);
    }
    pub fn fAckReq(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_fAckReq(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDEDATA {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [u8; 1],
}
impl DDEDATA {
    pub fn unused(&self) -> u16 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_unused(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !4095) | (value & 4095);
    }
    pub fn fResponse(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_fResponse(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u16) << 12);
    }
    pub fn fRelease(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_fRelease(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u16) << 13);
    }
    pub fn reserved(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_reserved(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u16) << 14);
    }
    pub fn fAckReq(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_fAckReq(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
impl Default for DDEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DDELN {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
impl DDELN {
    pub fn unused(&self) -> u16 {
        (self._bitfield << 3) >> 3
    }
    pub fn set_unused(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !8191) | (value & 8191);
    }
    pub fn fRelease(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_fRelease(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u16) << 13);
    }
    pub fn fDeferUpd(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_fDeferUpd(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u16) << 14);
    }
    pub fn fAckReq(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_fAckReq(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDEPOKE {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [u8; 1],
}
impl DDEPOKE {
    pub fn unused(&self) -> u16 {
        (self._bitfield << 3) >> 3
    }
    pub fn set_unused(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !8191) | (value & 8191);
    }
    pub fn fRelease(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_fRelease(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u16) << 13);
    }
    pub fn fReserved(&self) -> u16 {
        self._bitfield >> 14
    }
    pub fn set_fReserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(3 << 14)) | ((value & 3) << 14);
    }
}
impl Default for DDEPOKE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDEUP {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub rgb: [u8; 1],
}
impl DDEUP {
    pub fn unused(&self) -> u16 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_unused(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !4095) | (value & 4095);
    }
    pub fn fAck(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_fAck(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u16) << 12);
    }
    pub fn fRelease(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_fRelease(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u16) << 13);
    }
    pub fn fReserved(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_fReserved(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u16) << 14);
    }
    pub fn fAckReq(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_fAckReq(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
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
