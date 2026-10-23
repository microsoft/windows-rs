#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn DdeSetQualityOfService(hwndclient: super::HWND, pqosnew: *const super::SECURITY_QUALITY_OF_SERVICE, pqosprev: super::PSECURITY_QUALITY_OF_SERVICE) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DdeSetQualityOfService(hwndclient : super::HWND, pqosnew : *const super::SECURITY_QUALITY_OF_SERVICE, pqosprev : super::PSECURITY_QUALITY_OF_SERVICE) -> windows_core::BOOL);
    unsafe { DdeSetQualityOfService(hwndclient as _, pqosnew as _, pqosprev as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn FreeDDElParam(msg: super::UINT, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn FreeDDElParam(msg : super::UINT, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { FreeDDElParam(msg, lparam) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImpersonateDdeClientWindow(hwndclient: super::HWND, hwndserver: super::HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ImpersonateDdeClientWindow(hwndclient : super::HWND, hwndserver : super::HWND) -> windows_core::BOOL);
    unsafe { ImpersonateDdeClientWindow(hwndclient as _, hwndserver as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PackDDElParam(msg: super::UINT, uilo: super::UINT_PTR, uihi: super::UINT_PTR) -> super::LPARAM {
    windows_core::link!("user32.dll" "system" fn PackDDElParam(msg : super::UINT, uilo : super::UINT_PTR, uihi : super::UINT_PTR) -> super::LPARAM);
    unsafe { PackDDElParam(msg, uilo, uihi) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn ReuseDDElParam(lparam: super::LPARAM, msgin: super::UINT, msgout: super::UINT, uilo: super::UINT_PTR, uihi: super::UINT_PTR) -> super::LPARAM {
    windows_core::link!("user32.dll" "system" fn ReuseDDElParam(lparam : super::LPARAM, msgin : super::UINT, msgout : super::UINT, uilo : super::UINT_PTR, uihi : super::UINT_PTR) -> super::LPARAM);
    unsafe { ReuseDDElParam(lparam, msgin, msgout, uilo, uihi) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn UnpackDDElParam(msg: super::UINT, lparam: super::LPARAM, puilo: super::PUINT_PTR, puihi: super::PUINT_PTR) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnpackDDElParam(msg : super::UINT, lparam : super::LPARAM, puilo : super::PUINT_PTR, puihi : super::PUINT_PTR) -> windows_core::BOOL);
    unsafe { UnpackDDElParam(msg, lparam, puilo as _, puihi as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DDEACK {
    pub _bitfield: u16,
}
impl DDEACK {
    pub fn bAppReturnCode(&self) -> u16 {
        (self._bitfield) & 255
    }
    pub fn set_bAppReturnCode(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn reserved(&self) -> u16 {
        (self._bitfield >> 8) & 63
    }
    pub fn set_reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(63 << 8)) | ((value & 63) << 8);
    }
    pub fn fBusy(&self) -> u16 {
        (self._bitfield >> 14) & 1
    }
    pub fn set_fBusy(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value & 1) << 14);
    }
    pub fn fAck(&self) -> u16 {
        self._bitfield >> 15
    }
    pub fn set_fAck(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
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
        (self._bitfield) & 16383
    }
    pub fn set_reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !16383) | (value & 16383);
    }
    pub fn fDeferUpd(&self) -> u16 {
        (self._bitfield >> 14) & 1
    }
    pub fn set_fDeferUpd(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value & 1) << 14);
    }
    pub fn fAckReq(&self) -> u16 {
        self._bitfield >> 15
    }
    pub fn set_fAckReq(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDEDATA {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl DDEDATA {
    pub fn unused(&self) -> u16 {
        (self._bitfield) & 4095
    }
    pub fn set_unused(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !4095) | (value & 4095);
    }
    pub fn fResponse(&self) -> u16 {
        (self._bitfield >> 12) & 1
    }
    pub fn set_fResponse(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value & 1) << 12);
    }
    pub fn fRelease(&self) -> u16 {
        (self._bitfield >> 13) & 1
    }
    pub fn set_fRelease(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value & 1) << 13);
    }
    pub fn reserved(&self) -> u16 {
        (self._bitfield >> 14) & 1
    }
    pub fn set_reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value & 1) << 14);
    }
    pub fn fAckReq(&self) -> u16 {
        self._bitfield >> 15
    }
    pub fn set_fAckReq(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
}
#[cfg(feature = "shared")]
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
        (self._bitfield) & 8191
    }
    pub fn set_unused(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !8191) | (value & 8191);
    }
    pub fn fRelease(&self) -> u16 {
        (self._bitfield >> 13) & 1
    }
    pub fn set_fRelease(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value & 1) << 13);
    }
    pub fn fDeferUpd(&self) -> u16 {
        (self._bitfield >> 14) & 1
    }
    pub fn set_fDeferUpd(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value & 1) << 14);
    }
    pub fn fAckReq(&self) -> u16 {
        self._bitfield >> 15
    }
    pub fn set_fAckReq(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDEPOKE {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl DDEPOKE {
    pub fn unused(&self) -> u16 {
        (self._bitfield) & 8191
    }
    pub fn set_unused(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !8191) | (value & 8191);
    }
    pub fn fRelease(&self) -> u16 {
        (self._bitfield >> 13) & 1
    }
    pub fn set_fRelease(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value & 1) << 13);
    }
    pub fn fReserved(&self) -> u16 {
        self._bitfield >> 14
    }
    pub fn set_fReserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(3 << 14)) | ((value & 3) << 14);
    }
}
#[cfg(feature = "shared")]
impl Default for DDEPOKE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDEUP {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub rgb: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl DDEUP {
    pub fn unused(&self) -> u16 {
        (self._bitfield) & 4095
    }
    pub fn set_unused(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !4095) | (value & 4095);
    }
    pub fn fAck(&self) -> u16 {
        (self._bitfield >> 12) & 1
    }
    pub fn set_fAck(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value & 1) << 12);
    }
    pub fn fRelease(&self) -> u16 {
        (self._bitfield >> 13) & 1
    }
    pub fn set_fRelease(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value & 1) << 13);
    }
    pub fn fReserved(&self) -> u16 {
        (self._bitfield >> 14) & 1
    }
    pub fn set_fReserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value & 1) << 14);
    }
    pub fn fAckReq(&self) -> u16 {
        self._bitfield >> 15
    }
    pub fn set_fAckReq(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
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
