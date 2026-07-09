#[inline]
pub unsafe fn NetServerComputerNameAdd<P0, P1, P2>(servername: P0, emulateddomainname: P1, emulatedservername: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerComputerNameAdd(servername : windows_core::PCWSTR, emulateddomainname : windows_core::PCWSTR, emulatedservername : windows_core::PCWSTR) -> u32);
    unsafe { NetServerComputerNameAdd(servername.param().abi(), emulateddomainname.param().abi(), emulatedservername.param().abi()) }
}
#[inline]
pub unsafe fn NetServerComputerNameDel<P0, P1>(servername: P0, emulatedservername: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerComputerNameDel(servername : windows_core::PCWSTR, emulatedservername : windows_core::PCWSTR) -> u32);
    unsafe { NetServerComputerNameDel(servername.param().abi(), emulatedservername.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetServerDiskEnum<P0>(servername: P0, level: u32, bufptr: *mut super::minwindef::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerDiskEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    unsafe { NetServerDiskEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetServerEnum<P0, P7>(servername: P0, level: u32, bufptr: *mut super::minwindef::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, servertype: u32, domain: P7, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, servertype : u32, domain : windows_core::PCWSTR, resume_handle : *mut u32) -> u32);
    unsafe { NetServerEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, servertype, domain.param().abi(), resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetServerGetInfo<P0>(servername: P0, level: u32, bufptr: *mut super::minwindef::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerGetInfo(servername : windows_core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { NetServerGetInfo(servername.param().abi(), level, bufptr as _) }
}
#[inline]
pub unsafe fn NetServerSetInfo<P0>(servername: P0, level: u32, buf: *mut u8, parmerror: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerSetInfo(servername : windows_core::PCWSTR, level : u32, buf : *mut u8, parmerror : *mut u32) -> u32);
    unsafe { NetServerSetInfo(servername.param().abi(), level, buf as _, parmerror.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetServerTransportAdd<P0>(servername: P0, level: u32, bufptr: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerTransportAdd(servername : windows_core::PCWSTR, level : u32, bufptr : *const u8) -> u32);
    unsafe { NetServerTransportAdd(servername.param().abi(), level, bufptr) }
}
#[inline]
pub unsafe fn NetServerTransportAddEx<P0>(servername: P0, level: u32, bufptr: *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerTransportAddEx(servername : windows_core::PCWSTR, level : u32, bufptr : *mut u8) -> u32);
    unsafe { NetServerTransportAddEx(servername.param().abi(), level, bufptr as _) }
}
#[inline]
pub unsafe fn NetServerTransportDel<P0>(servername: P0, level: u32, bufptr: *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerTransportDel(servername : windows_core::PCWSTR, level : u32, bufptr : *mut u8) -> u32);
    unsafe { NetServerTransportDel(servername.param().abi(), level, bufptr as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetServerTransportEnum<P0>(servername: P0, level: u32, bufptr: *mut super::minwindef::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerTransportEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    unsafe { NetServerTransportEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winsvc")]
#[inline]
pub unsafe fn SetServiceBits(hservicestatus: super::winsvc::SERVICE_STATUS_HANDLE, dwservicebits: u32, bsetbitson: bool, bupdateimmediately: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetServiceBits(hservicestatus : super::winsvc::SERVICE_STATUS_HANDLE, dwservicebits : u32, bsetbitson : windows_core::BOOL, bupdateimmediately : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetServiceBits(hservicestatus, dwservicebits, bsetbitson.into(), bupdateimmediately.into()) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_100(pub *mut SERVER_INFO_100);
impl LPSERVER_INFO_100 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_100 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1005(pub *mut SERVER_INFO_1005);
impl LPSERVER_INFO_1005 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1005 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_101(pub *mut SERVER_INFO_101);
impl LPSERVER_INFO_101 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_101 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1010(pub *mut SERVER_INFO_1010);
impl LPSERVER_INFO_1010 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1010 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1016(pub *mut SERVER_INFO_1016);
impl LPSERVER_INFO_1016 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1016 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1017(pub *mut SERVER_INFO_1017);
impl LPSERVER_INFO_1017 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1017 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1018(pub *mut SERVER_INFO_1018);
impl LPSERVER_INFO_1018 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1018 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_102(pub *mut SERVER_INFO_102);
impl LPSERVER_INFO_102 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_102 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_103(pub *mut SERVER_INFO_103);
impl LPSERVER_INFO_103 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_103 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1107(pub *mut SERVER_INFO_1107);
impl LPSERVER_INFO_1107 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1107 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1501(pub *mut SERVER_INFO_1501);
impl LPSERVER_INFO_1501 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1501 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1502(pub *mut SERVER_INFO_1502);
impl LPSERVER_INFO_1502 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1502 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1503(pub *mut SERVER_INFO_1503);
impl LPSERVER_INFO_1503 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1503 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1506(pub *mut SERVER_INFO_1506);
impl LPSERVER_INFO_1506 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1506 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1509(pub *mut SERVER_INFO_1509);
impl LPSERVER_INFO_1509 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1509 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1510(pub *mut SERVER_INFO_1510);
impl LPSERVER_INFO_1510 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1510 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1511(pub *mut SERVER_INFO_1511);
impl LPSERVER_INFO_1511 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1511 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1512(pub *mut SERVER_INFO_1512);
impl LPSERVER_INFO_1512 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1512 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1513(pub *mut SERVER_INFO_1513);
impl LPSERVER_INFO_1513 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1513 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1514(pub *mut SERVER_INFO_1514);
impl LPSERVER_INFO_1514 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1514 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1515(pub *mut SERVER_INFO_1515);
impl LPSERVER_INFO_1515 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1515 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1516(pub *mut SERVER_INFO_1516);
impl LPSERVER_INFO_1516 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1516 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1518(pub *mut SERVER_INFO_1518);
impl LPSERVER_INFO_1518 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1518 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1520(pub *mut SERVER_INFO_1520);
impl LPSERVER_INFO_1520 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1520 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1521(pub *mut SERVER_INFO_1521);
impl LPSERVER_INFO_1521 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1521 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1522(pub *mut SERVER_INFO_1522);
impl LPSERVER_INFO_1522 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1522 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1523(pub *mut SERVER_INFO_1523);
impl LPSERVER_INFO_1523 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1523 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1524(pub *mut SERVER_INFO_1524);
impl LPSERVER_INFO_1524 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1524 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1525(pub *mut SERVER_INFO_1525);
impl LPSERVER_INFO_1525 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1525 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1528(pub *mut SERVER_INFO_1528);
impl LPSERVER_INFO_1528 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1528 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1529(pub *mut SERVER_INFO_1529);
impl LPSERVER_INFO_1529 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1529 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1530(pub *mut SERVER_INFO_1530);
impl LPSERVER_INFO_1530 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1530 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1533(pub *mut SERVER_INFO_1533);
impl LPSERVER_INFO_1533 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1533 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1534(pub *mut SERVER_INFO_1534);
impl LPSERVER_INFO_1534 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1534 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1535(pub *mut SERVER_INFO_1535);
impl LPSERVER_INFO_1535 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1535 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1536(pub *mut SERVER_INFO_1536);
impl LPSERVER_INFO_1536 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1536 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1537(pub *mut SERVER_INFO_1537);
impl LPSERVER_INFO_1537 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1537 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1538(pub *mut SERVER_INFO_1538);
impl LPSERVER_INFO_1538 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1538 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1539(pub *mut SERVER_INFO_1539);
impl LPSERVER_INFO_1539 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1539 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1540(pub *mut SERVER_INFO_1540);
impl LPSERVER_INFO_1540 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1540 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1541(pub *mut SERVER_INFO_1541);
impl LPSERVER_INFO_1541 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1541 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1542(pub *mut SERVER_INFO_1542);
impl LPSERVER_INFO_1542 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1542 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1543(pub *mut SERVER_INFO_1543);
impl LPSERVER_INFO_1543 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1543 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1544(pub *mut SERVER_INFO_1544);
impl LPSERVER_INFO_1544 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1544 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1545(pub *mut SERVER_INFO_1545);
impl LPSERVER_INFO_1545 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1545 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1546(pub *mut SERVER_INFO_1546);
impl LPSERVER_INFO_1546 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1546 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1547(pub *mut SERVER_INFO_1547);
impl LPSERVER_INFO_1547 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1547 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1548(pub *mut SERVER_INFO_1548);
impl LPSERVER_INFO_1548 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1548 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1549(pub *mut SERVER_INFO_1549);
impl LPSERVER_INFO_1549 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1549 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1550(pub *mut SERVER_INFO_1550);
impl LPSERVER_INFO_1550 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1550 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1552(pub *mut SERVER_INFO_1552);
impl LPSERVER_INFO_1552 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1552 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1553(pub *mut SERVER_INFO_1553);
impl LPSERVER_INFO_1553 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1553 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1554(pub *mut SERVER_INFO_1554);
impl LPSERVER_INFO_1554 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1554 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1555(pub *mut SERVER_INFO_1555);
impl LPSERVER_INFO_1555 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1555 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1556(pub *mut SERVER_INFO_1556);
impl LPSERVER_INFO_1556 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1556 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1557(pub *mut SERVER_INFO_1557);
impl LPSERVER_INFO_1557 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1557 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1560(pub *mut SERVER_INFO_1560);
impl LPSERVER_INFO_1560 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1560 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1561(pub *mut SERVER_INFO_1561);
impl LPSERVER_INFO_1561 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1561 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1562(pub *mut SERVER_INFO_1562);
impl LPSERVER_INFO_1562 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1562 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1563(pub *mut SERVER_INFO_1563);
impl LPSERVER_INFO_1563 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1563 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1564(pub *mut SERVER_INFO_1564);
impl LPSERVER_INFO_1564 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1564 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1565(pub *mut SERVER_INFO_1565);
impl LPSERVER_INFO_1565 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1565 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1566(pub *mut SERVER_INFO_1566);
impl LPSERVER_INFO_1566 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1566 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1567(pub *mut SERVER_INFO_1567);
impl LPSERVER_INFO_1567 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1567 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1568(pub *mut SERVER_INFO_1568);
impl LPSERVER_INFO_1568 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1568 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1569(pub *mut SERVER_INFO_1569);
impl LPSERVER_INFO_1569 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1569 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1570(pub *mut SERVER_INFO_1570);
impl LPSERVER_INFO_1570 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1570 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1571(pub *mut SERVER_INFO_1571);
impl LPSERVER_INFO_1571 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1571 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1572(pub *mut SERVER_INFO_1572);
impl LPSERVER_INFO_1572 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1572 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1573(pub *mut SERVER_INFO_1573);
impl LPSERVER_INFO_1573 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1573 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1574(pub *mut SERVER_INFO_1574);
impl LPSERVER_INFO_1574 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1574 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1575(pub *mut SERVER_INFO_1575);
impl LPSERVER_INFO_1575 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1575 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1576(pub *mut SERVER_INFO_1576);
impl LPSERVER_INFO_1576 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1576 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1577(pub *mut SERVER_INFO_1577);
impl LPSERVER_INFO_1577 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1577 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1578(pub *mut SERVER_INFO_1578);
impl LPSERVER_INFO_1578 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1578 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1579(pub *mut SERVER_INFO_1579);
impl LPSERVER_INFO_1579 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1579 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1580(pub *mut SERVER_INFO_1580);
impl LPSERVER_INFO_1580 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1580 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1581(pub *mut SERVER_INFO_1581);
impl LPSERVER_INFO_1581 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1581 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1582(pub *mut SERVER_INFO_1582);
impl LPSERVER_INFO_1582 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1582 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1583(pub *mut SERVER_INFO_1583);
impl LPSERVER_INFO_1583 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1583 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1584(pub *mut SERVER_INFO_1584);
impl LPSERVER_INFO_1584 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1584 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1585(pub *mut SERVER_INFO_1585);
impl LPSERVER_INFO_1585 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1585 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1586(pub *mut SERVER_INFO_1586);
impl LPSERVER_INFO_1586 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1586 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1587(pub *mut SERVER_INFO_1587);
impl LPSERVER_INFO_1587 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1587 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1588(pub *mut SERVER_INFO_1588);
impl LPSERVER_INFO_1588 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1588 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1590(pub *mut SERVER_INFO_1590);
impl LPSERVER_INFO_1590 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1590 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1591(pub *mut SERVER_INFO_1591);
impl LPSERVER_INFO_1591 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1591 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1592(pub *mut SERVER_INFO_1592);
impl LPSERVER_INFO_1592 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1592 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1593(pub *mut SERVER_INFO_1593);
impl LPSERVER_INFO_1593 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1593 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1594(pub *mut SERVER_INFO_1594);
impl LPSERVER_INFO_1594 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1594 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1595(pub *mut SERVER_INFO_1595);
impl LPSERVER_INFO_1595 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1595 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1596(pub *mut SERVER_INFO_1596);
impl LPSERVER_INFO_1596 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1596 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1597(pub *mut SERVER_INFO_1597);
impl LPSERVER_INFO_1597 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1597 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1598(pub *mut SERVER_INFO_1598);
impl LPSERVER_INFO_1598 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1598 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1599(pub *mut SERVER_INFO_1599);
impl LPSERVER_INFO_1599 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1599 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1600(pub *mut SERVER_INFO_1600);
impl LPSERVER_INFO_1600 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1600 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1601(pub *mut SERVER_INFO_1601);
impl LPSERVER_INFO_1601 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1601 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_1602(pub *mut SERVER_INFO_1602);
impl LPSERVER_INFO_1602 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_1602 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_402(pub *mut SERVER_INFO_402);
impl LPSERVER_INFO_402 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_402 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_403(pub *mut SERVER_INFO_403);
impl LPSERVER_INFO_403 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_403 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_502(pub *mut SERVER_INFO_502);
impl LPSERVER_INFO_502 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_502 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_503(pub *mut SERVER_INFO_503);
impl LPSERVER_INFO_503 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_503 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_598(pub *mut SERVER_INFO_598);
impl LPSERVER_INFO_598 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_598 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_INFO_599(pub *mut SERVER_INFO_599);
impl LPSERVER_INFO_599 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSERVER_INFO_599 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_TRANSPORT_INFO_0(pub *mut SERVER_TRANSPORT_INFO_0);
#[cfg(feature = "minwindef")]
impl LPSERVER_TRANSPORT_INFO_0 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPSERVER_TRANSPORT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_TRANSPORT_INFO_1(pub *mut SERVER_TRANSPORT_INFO_1);
#[cfg(feature = "minwindef")]
impl LPSERVER_TRANSPORT_INFO_1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPSERVER_TRANSPORT_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_TRANSPORT_INFO_2(pub *mut SERVER_TRANSPORT_INFO_2);
#[cfg(feature = "minwindef")]
impl LPSERVER_TRANSPORT_INFO_2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPSERVER_TRANSPORT_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSERVER_TRANSPORT_INFO_3(pub *mut SERVER_TRANSPORT_INFO_3);
#[cfg(feature = "minwindef")]
impl LPSERVER_TRANSPORT_INFO_3 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPSERVER_TRANSPORT_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAJOR_VERSION_MASK: u32 = 15;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_100(pub *mut SERVER_INFO_100);
impl PSERVER_INFO_100 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_100 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1005(pub *mut SERVER_INFO_1005);
impl PSERVER_INFO_1005 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1005 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_101(pub *mut SERVER_INFO_101);
impl PSERVER_INFO_101 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_101 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1010(pub *mut SERVER_INFO_1010);
impl PSERVER_INFO_1010 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1010 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1016(pub *mut SERVER_INFO_1016);
impl PSERVER_INFO_1016 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1016 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1017(pub *mut SERVER_INFO_1017);
impl PSERVER_INFO_1017 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1017 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1018(pub *mut SERVER_INFO_1018);
impl PSERVER_INFO_1018 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1018 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_102(pub *mut SERVER_INFO_102);
impl PSERVER_INFO_102 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_102 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_103(pub *mut SERVER_INFO_103);
impl PSERVER_INFO_103 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_103 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1107(pub *mut SERVER_INFO_1107);
impl PSERVER_INFO_1107 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1107 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1501(pub *mut SERVER_INFO_1501);
impl PSERVER_INFO_1501 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1501 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1502(pub *mut SERVER_INFO_1502);
impl PSERVER_INFO_1502 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1502 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1503(pub *mut SERVER_INFO_1503);
impl PSERVER_INFO_1503 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1503 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1506(pub *mut SERVER_INFO_1506);
impl PSERVER_INFO_1506 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1506 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1509(pub *mut SERVER_INFO_1509);
impl PSERVER_INFO_1509 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1509 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1510(pub *mut SERVER_INFO_1510);
impl PSERVER_INFO_1510 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1510 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1511(pub *mut SERVER_INFO_1511);
impl PSERVER_INFO_1511 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1511 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1512(pub *mut SERVER_INFO_1512);
impl PSERVER_INFO_1512 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1512 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1513(pub *mut SERVER_INFO_1513);
impl PSERVER_INFO_1513 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1513 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1514(pub *mut SERVER_INFO_1514);
impl PSERVER_INFO_1514 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1514 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1515(pub *mut SERVER_INFO_1515);
impl PSERVER_INFO_1515 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1515 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1516(pub *mut SERVER_INFO_1516);
impl PSERVER_INFO_1516 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1516 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1518(pub *mut SERVER_INFO_1518);
impl PSERVER_INFO_1518 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1518 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1520(pub *mut SERVER_INFO_1520);
impl PSERVER_INFO_1520 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1520 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1521(pub *mut SERVER_INFO_1521);
impl PSERVER_INFO_1521 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1521 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1522(pub *mut SERVER_INFO_1522);
impl PSERVER_INFO_1522 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1522 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1523(pub *mut SERVER_INFO_1523);
impl PSERVER_INFO_1523 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1523 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1524(pub *mut SERVER_INFO_1524);
impl PSERVER_INFO_1524 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1524 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1525(pub *mut SERVER_INFO_1525);
impl PSERVER_INFO_1525 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1525 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1528(pub *mut SERVER_INFO_1528);
impl PSERVER_INFO_1528 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1528 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1529(pub *mut SERVER_INFO_1529);
impl PSERVER_INFO_1529 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1529 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1530(pub *mut SERVER_INFO_1530);
impl PSERVER_INFO_1530 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1530 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1533(pub *mut SERVER_INFO_1533);
impl PSERVER_INFO_1533 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1533 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1534(pub *mut SERVER_INFO_1534);
impl PSERVER_INFO_1534 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1534 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1535(pub *mut SERVER_INFO_1535);
impl PSERVER_INFO_1535 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1535 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1536(pub *mut SERVER_INFO_1536);
impl PSERVER_INFO_1536 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1536 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1537(pub *mut SERVER_INFO_1537);
impl PSERVER_INFO_1537 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1537 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1538(pub *mut SERVER_INFO_1538);
impl PSERVER_INFO_1538 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1538 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1539(pub *mut SERVER_INFO_1539);
impl PSERVER_INFO_1539 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1539 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1540(pub *mut SERVER_INFO_1540);
impl PSERVER_INFO_1540 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1540 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1541(pub *mut SERVER_INFO_1541);
impl PSERVER_INFO_1541 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1541 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1542(pub *mut SERVER_INFO_1542);
impl PSERVER_INFO_1542 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1542 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1543(pub *mut SERVER_INFO_1543);
impl PSERVER_INFO_1543 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1543 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1544(pub *mut SERVER_INFO_1544);
impl PSERVER_INFO_1544 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1544 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1545(pub *mut SERVER_INFO_1545);
impl PSERVER_INFO_1545 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1545 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1546(pub *mut SERVER_INFO_1546);
impl PSERVER_INFO_1546 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1546 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1547(pub *mut SERVER_INFO_1547);
impl PSERVER_INFO_1547 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1547 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1548(pub *mut SERVER_INFO_1548);
impl PSERVER_INFO_1548 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1548 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1549(pub *mut SERVER_INFO_1549);
impl PSERVER_INFO_1549 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1549 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1550(pub *mut SERVER_INFO_1550);
impl PSERVER_INFO_1550 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1550 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1552(pub *mut SERVER_INFO_1552);
impl PSERVER_INFO_1552 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1552 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1553(pub *mut SERVER_INFO_1553);
impl PSERVER_INFO_1553 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1553 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1554(pub *mut SERVER_INFO_1554);
impl PSERVER_INFO_1554 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1554 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1555(pub *mut SERVER_INFO_1555);
impl PSERVER_INFO_1555 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1555 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1556(pub *mut SERVER_INFO_1556);
impl PSERVER_INFO_1556 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1556 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1557(pub *mut SERVER_INFO_1557);
impl PSERVER_INFO_1557 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1557 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1560(pub *mut SERVER_INFO_1560);
impl PSERVER_INFO_1560 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1560 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1561(pub *mut SERVER_INFO_1561);
impl PSERVER_INFO_1561 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1561 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1562(pub *mut SERVER_INFO_1562);
impl PSERVER_INFO_1562 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1562 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1563(pub *mut SERVER_INFO_1563);
impl PSERVER_INFO_1563 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1563 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1564(pub *mut SERVER_INFO_1564);
impl PSERVER_INFO_1564 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1564 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1565(pub *mut SERVER_INFO_1565);
impl PSERVER_INFO_1565 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1565 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1566(pub *mut SERVER_INFO_1566);
impl PSERVER_INFO_1566 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1566 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1567(pub *mut SERVER_INFO_1567);
impl PSERVER_INFO_1567 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1567 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1568(pub *mut SERVER_INFO_1568);
impl PSERVER_INFO_1568 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1568 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1569(pub *mut SERVER_INFO_1569);
impl PSERVER_INFO_1569 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1569 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1570(pub *mut SERVER_INFO_1570);
impl PSERVER_INFO_1570 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1570 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1571(pub *mut SERVER_INFO_1571);
impl PSERVER_INFO_1571 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1571 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1572(pub *mut SERVER_INFO_1572);
impl PSERVER_INFO_1572 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1572 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1573(pub *mut SERVER_INFO_1573);
impl PSERVER_INFO_1573 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1573 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1574(pub *mut SERVER_INFO_1574);
impl PSERVER_INFO_1574 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1574 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1575(pub *mut SERVER_INFO_1575);
impl PSERVER_INFO_1575 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1575 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1576(pub *mut SERVER_INFO_1576);
impl PSERVER_INFO_1576 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1576 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1577(pub *mut SERVER_INFO_1577);
impl PSERVER_INFO_1577 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1577 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1578(pub *mut SERVER_INFO_1578);
impl PSERVER_INFO_1578 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1578 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1579(pub *mut SERVER_INFO_1579);
impl PSERVER_INFO_1579 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1579 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1580(pub *mut SERVER_INFO_1580);
impl PSERVER_INFO_1580 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1580 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1581(pub *mut SERVER_INFO_1581);
impl PSERVER_INFO_1581 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1581 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1582(pub *mut SERVER_INFO_1582);
impl PSERVER_INFO_1582 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1582 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1583(pub *mut SERVER_INFO_1583);
impl PSERVER_INFO_1583 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1583 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1584(pub *mut SERVER_INFO_1584);
impl PSERVER_INFO_1584 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1584 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1585(pub *mut SERVER_INFO_1585);
impl PSERVER_INFO_1585 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1585 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1586(pub *mut SERVER_INFO_1586);
impl PSERVER_INFO_1586 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1586 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1587(pub *mut SERVER_INFO_1587);
impl PSERVER_INFO_1587 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1587 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1588(pub *mut SERVER_INFO_1588);
impl PSERVER_INFO_1588 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1588 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1590(pub *mut SERVER_INFO_1590);
impl PSERVER_INFO_1590 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1590 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1591(pub *mut SERVER_INFO_1591);
impl PSERVER_INFO_1591 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1591 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1592(pub *mut SERVER_INFO_1592);
impl PSERVER_INFO_1592 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1592 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1593(pub *mut SERVER_INFO_1593);
impl PSERVER_INFO_1593 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1593 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1594(pub *mut SERVER_INFO_1594);
impl PSERVER_INFO_1594 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1594 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1595(pub *mut SERVER_INFO_1595);
impl PSERVER_INFO_1595 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1595 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1596(pub *mut SERVER_INFO_1596);
impl PSERVER_INFO_1596 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1596 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1597(pub *mut SERVER_INFO_1597);
impl PSERVER_INFO_1597 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1597 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1598(pub *mut SERVER_INFO_1598);
impl PSERVER_INFO_1598 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1598 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1599(pub *mut SERVER_INFO_1599);
impl PSERVER_INFO_1599 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1599 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1600(pub *mut SERVER_INFO_1600);
impl PSERVER_INFO_1600 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1600 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1601(pub *mut SERVER_INFO_1601);
impl PSERVER_INFO_1601 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1601 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_1602(pub *mut SERVER_INFO_1602);
impl PSERVER_INFO_1602 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_1602 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_402(pub *mut SERVER_INFO_402);
impl PSERVER_INFO_402 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_402 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_403(pub *mut SERVER_INFO_403);
impl PSERVER_INFO_403 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_403 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_502(pub *mut SERVER_INFO_502);
impl PSERVER_INFO_502 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_502 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_503(pub *mut SERVER_INFO_503);
impl PSERVER_INFO_503 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_503 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_598(pub *mut SERVER_INFO_598);
impl PSERVER_INFO_598 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_598 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_INFO_599(pub *mut SERVER_INFO_599);
impl PSERVER_INFO_599 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVER_INFO_599 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_TRANSPORT_INFO_0(pub *mut SERVER_TRANSPORT_INFO_0);
#[cfg(feature = "minwindef")]
impl PSERVER_TRANSPORT_INFO_0 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PSERVER_TRANSPORT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_TRANSPORT_INFO_1(pub *mut SERVER_TRANSPORT_INFO_1);
#[cfg(feature = "minwindef")]
impl PSERVER_TRANSPORT_INFO_1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PSERVER_TRANSPORT_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_TRANSPORT_INFO_2(pub *mut SERVER_TRANSPORT_INFO_2);
#[cfg(feature = "minwindef")]
impl PSERVER_TRANSPORT_INFO_2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PSERVER_TRANSPORT_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVER_TRANSPORT_INFO_3(pub *mut SERVER_TRANSPORT_INFO_3);
#[cfg(feature = "minwindef")]
impl PSERVER_TRANSPORT_INFO_3 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PSERVER_TRANSPORT_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_100 {
    pub sv100_platform_id: u32,
    pub sv100_name: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1005 {
    pub sv1005_comment: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_101 {
    pub sv101_platform_id: u32,
    pub sv101_name: windows_core::PWSTR,
    pub sv101_version_major: u32,
    pub sv101_version_minor: u32,
    pub sv101_type: u32,
    pub sv101_comment: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1010 {
    pub sv1010_disc: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1016 {
    pub sv1016_hidden: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1017 {
    pub sv1017_announce: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1018 {
    pub sv1018_anndelta: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_102 {
    pub sv102_platform_id: u32,
    pub sv102_name: windows_core::PWSTR,
    pub sv102_version_major: u32,
    pub sv102_version_minor: u32,
    pub sv102_type: u32,
    pub sv102_comment: windows_core::PWSTR,
    pub sv102_users: u32,
    pub sv102_disc: i32,
    pub sv102_hidden: windows_core::BOOL,
    pub sv102_announce: u32,
    pub sv102_anndelta: u32,
    pub sv102_licenses: u32,
    pub sv102_userpath: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_103 {
    pub sv103_platform_id: u32,
    pub sv103_name: windows_core::PWSTR,
    pub sv103_version_major: u32,
    pub sv103_version_minor: u32,
    pub sv103_type: u32,
    pub sv103_comment: windows_core::PWSTR,
    pub sv103_users: u32,
    pub sv103_disc: i32,
    pub sv103_hidden: windows_core::BOOL,
    pub sv103_announce: u32,
    pub sv103_anndelta: u32,
    pub sv103_licenses: u32,
    pub sv103_userpath: windows_core::PWSTR,
    pub sv103_capabilities: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1107 {
    pub sv1107_users: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1501 {
    pub sv1501_sessopens: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1502 {
    pub sv1502_sessvcs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1503 {
    pub sv1503_opensearch: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1506 {
    pub sv1506_maxworkitems: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1509 {
    pub sv1509_maxrawbuflen: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1510 {
    pub sv1510_sessusers: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1511 {
    pub sv1511_sessconns: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1512 {
    pub sv1512_maxnonpagedmemoryusage: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1513 {
    pub sv1513_maxpagedmemoryusage: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1514 {
    pub sv1514_enablesoftcompat: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1515 {
    pub sv1515_enableforcedlogoff: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1516 {
    pub sv1516_timesource: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1518 {
    pub sv1518_lmannounce: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1520 {
    pub sv1520_maxcopyreadlen: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1521 {
    pub sv1521_maxcopywritelen: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1522 {
    pub sv1522_minkeepsearch: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1523 {
    pub sv1523_maxkeepsearch: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1524 {
    pub sv1524_minkeepcomplsearch: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1525 {
    pub sv1525_maxkeepcomplsearch: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1528 {
    pub sv1528_scavtimeout: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1529 {
    pub sv1529_minrcvqueue: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1530 {
    pub sv1530_minfreeworkitems: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1533 {
    pub sv1533_maxmpxct: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1534 {
    pub sv1534_oplockbreakwait: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1535 {
    pub sv1535_oplockbreakresponsewait: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1536 {
    pub sv1536_enableoplocks: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1537 {
    pub sv1537_enableoplockforceclose: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1538 {
    pub sv1538_enablefcbopens: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1539 {
    pub sv1539_enableraw: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1540 {
    pub sv1540_enablesharednetdrives: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1541 {
    pub sv1541_minfreeconnections: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1542 {
    pub sv1542_maxfreeconnections: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1543 {
    pub sv1543_initsesstable: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1544 {
    pub sv1544_initconntable: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1545 {
    pub sv1545_initfiletable: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1546 {
    pub sv1546_initsearchtable: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1547 {
    pub sv1547_alertschedule: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1548 {
    pub sv1548_errorthreshold: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1549 {
    pub sv1549_networkerrorthreshold: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1550 {
    pub sv1550_diskspacethreshold: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1552 {
    pub sv1552_maxlinkdelay: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1553 {
    pub sv1553_minlinkthroughput: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1554 {
    pub sv1554_linkinfovalidtime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1555 {
    pub sv1555_scavqosinfoupdatetime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1556 {
    pub sv1556_maxworkitemidletime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1557 {
    pub sv1557_maxrawworkitems: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1560 {
    pub sv1560_producttype: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1561 {
    pub sv1561_serversize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1562 {
    pub sv1562_connectionlessautodisc: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1563 {
    pub sv1563_sharingviolationretries: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1564 {
    pub sv1564_sharingviolationdelay: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1565 {
    pub sv1565_maxglobalopensearch: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1566 {
    pub sv1566_removeduplicatesearches: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1567 {
    pub sv1567_lockviolationretries: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1568 {
    pub sv1568_lockviolationoffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1569 {
    pub sv1569_lockviolationdelay: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1570 {
    pub sv1570_mdlreadswitchover: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1571 {
    pub sv1571_cachedopenlimit: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1572 {
    pub sv1572_criticalthreads: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1573 {
    pub sv1573_restrictnullsessaccess: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1574 {
    pub sv1574_enablewfw311directipx: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1575 {
    pub sv1575_otherqueueaffinity: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1576 {
    pub sv1576_queuesamplesecs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1577 {
    pub sv1577_balancecount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1578 {
    pub sv1578_preferredaffinity: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1579 {
    pub sv1579_maxfreerfcbs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1580 {
    pub sv1580_maxfreemfcbs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1581 {
    pub sv1581_maxfreemlcbs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1582 {
    pub sv1582_maxfreepagedpoolchunks: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1583 {
    pub sv1583_minpagedpoolchunksize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1584 {
    pub sv1584_maxpagedpoolchunksize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1585 {
    pub sv1585_sendsfrompreferredprocessor: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1586 {
    pub sv1586_maxthreadsperqueue: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1587 {
    pub sv1587_cacheddirectorylimit: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1588 {
    pub sv1588_maxcopylength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1590 {
    pub sv1590_enablecompression: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1591 {
    pub sv1591_autosharewks: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1592 {
    pub sv1592_autosharewks: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1593 {
    pub sv1593_enablesecuritysignature: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1594 {
    pub sv1594_requiresecuritysignature: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1595 {
    pub sv1595_minclientbuffersize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1596 {
    pub sv1596_ConnectionNoSessionsTimeout: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1597 {
    pub sv1597_IdleThreadTimeOut: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1598 {
    pub sv1598_enableW9xsecuritysignature: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1599 {
    pub sv1598_enforcekerberosreauthentication: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1600 {
    pub sv1598_disabledos: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1601 {
    pub sv1598_lowdiskspaceminimum: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_1602 {
    pub sv_1598_disablestrictnamechecking: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_402 {
    pub sv402_ulist_mtime: u32,
    pub sv402_glist_mtime: u32,
    pub sv402_alist_mtime: u32,
    pub sv402_alerts: windows_core::PWSTR,
    pub sv402_security: u32,
    pub sv402_numadmin: u32,
    pub sv402_lanmask: u32,
    pub sv402_guestacct: windows_core::PWSTR,
    pub sv402_chdevs: u32,
    pub sv402_chdevq: u32,
    pub sv402_chdevjobs: u32,
    pub sv402_connections: u32,
    pub sv402_shares: u32,
    pub sv402_openfiles: u32,
    pub sv402_sessopens: u32,
    pub sv402_sessvcs: u32,
    pub sv402_sessreqs: u32,
    pub sv402_opensearch: u32,
    pub sv402_activelocks: u32,
    pub sv402_numreqbuf: u32,
    pub sv402_sizreqbuf: u32,
    pub sv402_numbigbuf: u32,
    pub sv402_numfiletasks: u32,
    pub sv402_alertsched: u32,
    pub sv402_erroralert: u32,
    pub sv402_logonalert: u32,
    pub sv402_accessalert: u32,
    pub sv402_diskalert: u32,
    pub sv402_netioalert: u32,
    pub sv402_maxauditsz: u32,
    pub sv402_srvheuristics: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_403 {
    pub sv403_ulist_mtime: u32,
    pub sv403_glist_mtime: u32,
    pub sv403_alist_mtime: u32,
    pub sv403_alerts: windows_core::PWSTR,
    pub sv403_security: u32,
    pub sv403_numadmin: u32,
    pub sv403_lanmask: u32,
    pub sv403_guestacct: windows_core::PWSTR,
    pub sv403_chdevs: u32,
    pub sv403_chdevq: u32,
    pub sv403_chdevjobs: u32,
    pub sv403_connections: u32,
    pub sv403_shares: u32,
    pub sv403_openfiles: u32,
    pub sv403_sessopens: u32,
    pub sv403_sessvcs: u32,
    pub sv403_sessreqs: u32,
    pub sv403_opensearch: u32,
    pub sv403_activelocks: u32,
    pub sv403_numreqbuf: u32,
    pub sv403_sizreqbuf: u32,
    pub sv403_numbigbuf: u32,
    pub sv403_numfiletasks: u32,
    pub sv403_alertsched: u32,
    pub sv403_erroralert: u32,
    pub sv403_logonalert: u32,
    pub sv403_accessalert: u32,
    pub sv403_diskalert: u32,
    pub sv403_netioalert: u32,
    pub sv403_maxauditsz: u32,
    pub sv403_srvheuristics: windows_core::PWSTR,
    pub sv403_auditedevents: u32,
    pub sv403_autoprofile: u32,
    pub sv403_autopath: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_502 {
    pub sv502_sessopens: u32,
    pub sv502_sessvcs: u32,
    pub sv502_opensearch: u32,
    pub sv502_sizreqbuf: u32,
    pub sv502_initworkitems: u32,
    pub sv502_maxworkitems: u32,
    pub sv502_rawworkitems: u32,
    pub sv502_irpstacksize: u32,
    pub sv502_maxrawbuflen: u32,
    pub sv502_sessusers: u32,
    pub sv502_sessconns: u32,
    pub sv502_maxpagedmemoryusage: u32,
    pub sv502_maxnonpagedmemoryusage: u32,
    pub sv502_enablesoftcompat: windows_core::BOOL,
    pub sv502_enableforcedlogoff: windows_core::BOOL,
    pub sv502_timesource: windows_core::BOOL,
    pub sv502_acceptdownlevelapis: windows_core::BOOL,
    pub sv502_lmannounce: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_503 {
    pub sv503_sessopens: u32,
    pub sv503_sessvcs: u32,
    pub sv503_opensearch: u32,
    pub sv503_sizreqbuf: u32,
    pub sv503_initworkitems: u32,
    pub sv503_maxworkitems: u32,
    pub sv503_rawworkitems: u32,
    pub sv503_irpstacksize: u32,
    pub sv503_maxrawbuflen: u32,
    pub sv503_sessusers: u32,
    pub sv503_sessconns: u32,
    pub sv503_maxpagedmemoryusage: u32,
    pub sv503_maxnonpagedmemoryusage: u32,
    pub sv503_enablesoftcompat: windows_core::BOOL,
    pub sv503_enableforcedlogoff: windows_core::BOOL,
    pub sv503_timesource: windows_core::BOOL,
    pub sv503_acceptdownlevelapis: windows_core::BOOL,
    pub sv503_lmannounce: windows_core::BOOL,
    pub sv503_domain: windows_core::PWSTR,
    pub sv503_maxcopyreadlen: u32,
    pub sv503_maxcopywritelen: u32,
    pub sv503_minkeepsearch: u32,
    pub sv503_maxkeepsearch: u32,
    pub sv503_minkeepcomplsearch: u32,
    pub sv503_maxkeepcomplsearch: u32,
    pub sv503_threadcountadd: u32,
    pub sv503_numblockthreads: u32,
    pub sv503_scavtimeout: u32,
    pub sv503_minrcvqueue: u32,
    pub sv503_minfreeworkitems: u32,
    pub sv503_xactmemsize: u32,
    pub sv503_threadpriority: u32,
    pub sv503_maxmpxct: u32,
    pub sv503_oplockbreakwait: u32,
    pub sv503_oplockbreakresponsewait: u32,
    pub sv503_enableoplocks: windows_core::BOOL,
    pub sv503_enableoplockforceclose: windows_core::BOOL,
    pub sv503_enablefcbopens: windows_core::BOOL,
    pub sv503_enableraw: windows_core::BOOL,
    pub sv503_enablesharednetdrives: windows_core::BOOL,
    pub sv503_minfreeconnections: u32,
    pub sv503_maxfreeconnections: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_598 {
    pub sv598_maxrawworkitems: u32,
    pub sv598_maxthreadsperqueue: u32,
    pub sv598_producttype: u32,
    pub sv598_serversize: u32,
    pub sv598_connectionlessautodisc: u32,
    pub sv598_sharingviolationretries: u32,
    pub sv598_sharingviolationdelay: u32,
    pub sv598_maxglobalopensearch: u32,
    pub sv598_removeduplicatesearches: u32,
    pub sv598_lockviolationoffset: u32,
    pub sv598_lockviolationdelay: u32,
    pub sv598_mdlreadswitchover: u32,
    pub sv598_cachedopenlimit: u32,
    pub sv598_otherqueueaffinity: u32,
    pub sv598_restrictnullsessaccess: windows_core::BOOL,
    pub sv598_enablewfw311directipx: windows_core::BOOL,
    pub sv598_queuesamplesecs: u32,
    pub sv598_balancecount: u32,
    pub sv598_preferredaffinity: u32,
    pub sv598_maxfreerfcbs: u32,
    pub sv598_maxfreemfcbs: u32,
    pub sv598_maxfreelfcbs: u32,
    pub sv598_maxfreepagedpoolchunks: u32,
    pub sv598_minpagedpoolchunksize: u32,
    pub sv598_maxpagedpoolchunksize: u32,
    pub sv598_sendsfrompreferredprocessor: windows_core::BOOL,
    pub sv598_cacheddirectorylimit: u32,
    pub sv598_maxcopylength: u32,
    pub sv598_enablecompression: windows_core::BOOL,
    pub sv598_autosharewks: windows_core::BOOL,
    pub sv598_autoshareserver: windows_core::BOOL,
    pub sv598_enablesecuritysignature: windows_core::BOOL,
    pub sv598_requiresecuritysignature: windows_core::BOOL,
    pub sv598_minclientbuffersize: u32,
    pub sv598_serverguid: windows_core::GUID,
    pub sv598_ConnectionNoSessionsTimeout: u32,
    pub sv598_IdleThreadTimeOut: u32,
    pub sv598_enableW9xsecuritysignature: windows_core::BOOL,
    pub sv598_enforcekerberosreauthentication: windows_core::BOOL,
    pub sv598_disabledos: windows_core::BOOL,
    pub sv598_lowdiskspaceminimum: u32,
    pub sv598_disablestrictnamechecking: windows_core::BOOL,
    pub sv598_enableauthenticateusersharing: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_INFO_599 {
    pub sv599_sessopens: u32,
    pub sv599_sessvcs: u32,
    pub sv599_opensearch: u32,
    pub sv599_sizreqbuf: u32,
    pub sv599_initworkitems: u32,
    pub sv599_maxworkitems: u32,
    pub sv599_rawworkitems: u32,
    pub sv599_irpstacksize: u32,
    pub sv599_maxrawbuflen: u32,
    pub sv599_sessusers: u32,
    pub sv599_sessconns: u32,
    pub sv599_maxpagedmemoryusage: u32,
    pub sv599_maxnonpagedmemoryusage: u32,
    pub sv599_enablesoftcompat: windows_core::BOOL,
    pub sv599_enableforcedlogoff: windows_core::BOOL,
    pub sv599_timesource: windows_core::BOOL,
    pub sv599_acceptdownlevelapis: windows_core::BOOL,
    pub sv599_lmannounce: windows_core::BOOL,
    pub sv599_domain: windows_core::PWSTR,
    pub sv599_maxcopyreadlen: u32,
    pub sv599_maxcopywritelen: u32,
    pub sv599_minkeepsearch: u32,
    pub sv599_maxkeepsearch: u32,
    pub sv599_minkeepcomplsearch: u32,
    pub sv599_maxkeepcomplsearch: u32,
    pub sv599_threadcountadd: u32,
    pub sv599_numblockthreads: u32,
    pub sv599_scavtimeout: u32,
    pub sv599_minrcvqueue: u32,
    pub sv599_minfreeworkitems: u32,
    pub sv599_xactmemsize: u32,
    pub sv599_threadpriority: u32,
    pub sv599_maxmpxct: u32,
    pub sv599_oplockbreakwait: u32,
    pub sv599_oplockbreakresponsewait: u32,
    pub sv599_enableoplocks: windows_core::BOOL,
    pub sv599_enableoplockforceclose: windows_core::BOOL,
    pub sv599_enablefcbopens: windows_core::BOOL,
    pub sv599_enableraw: windows_core::BOOL,
    pub sv599_enablesharednetdrives: windows_core::BOOL,
    pub sv599_minfreeconnections: u32,
    pub sv599_maxfreeconnections: u32,
    pub sv599_initsesstable: u32,
    pub sv599_initconntable: u32,
    pub sv599_initfiletable: u32,
    pub sv599_initsearchtable: u32,
    pub sv599_alertschedule: u32,
    pub sv599_errorthreshold: u32,
    pub sv599_networkerrorthreshold: u32,
    pub sv599_diskspacethreshold: u32,
    pub sv599_reserved: u32,
    pub sv599_maxlinkdelay: u32,
    pub sv599_minlinkthroughput: u32,
    pub sv599_linkinfovalidtime: u32,
    pub sv599_scavqosinfoupdatetime: u32,
    pub sv599_maxworkitemidletime: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_TRANSPORT_INFO_0 {
    pub svti0_numberofvcs: u32,
    pub svti0_transportname: windows_core::PWSTR,
    pub svti0_transportaddress: super::minwindef::LPBYTE,
    pub svti0_transportaddresslength: u32,
    pub svti0_networkaddress: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_TRANSPORT_INFO_1 {
    pub svti1_numberofvcs: u32,
    pub svti1_transportname: windows_core::PWSTR,
    pub svti1_transportaddress: super::minwindef::LPBYTE,
    pub svti1_transportaddresslength: u32,
    pub svti1_networkaddress: windows_core::PWSTR,
    pub svti1_domain: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVER_TRANSPORT_INFO_2 {
    pub svti2_numberofvcs: u32,
    pub svti2_transportname: windows_core::PWSTR,
    pub svti2_transportaddress: super::minwindef::LPBYTE,
    pub svti2_transportaddresslength: u32,
    pub svti2_networkaddress: windows_core::PWSTR,
    pub svti2_domain: windows_core::PWSTR,
    pub svti2_flags: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SERVER_TRANSPORT_INFO_3 {
    pub svti3_numberofvcs: u32,
    pub svti3_transportname: windows_core::PWSTR,
    pub svti3_transportaddress: super::minwindef::LPBYTE,
    pub svti3_transportaddresslength: u32,
    pub svti3_networkaddress: windows_core::PWSTR,
    pub svti3_domain: windows_core::PWSTR,
    pub svti3_flags: u32,
    pub svti3_passwordlength: u32,
    pub svti3_password: [u8; 256],
}
#[cfg(feature = "minwindef")]
impl Default for SERVER_TRANSPORT_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SRV_HASH_GENERATION_ACTIVE: u32 = 2;
pub const SRV_SUPPORT_HASH_GENERATION: u32 = 1;
pub const SVI1_NUM_ELEMENTS: u32 = 5;
pub const SVI2_NUM_ELEMENTS: u32 = 40;
pub const SVI3_NUM_ELEMENTS: u32 = 44;
pub const SVTI2_CLUSTER_DNN_NAME: u32 = 16;
pub const SVTI2_CLUSTER_NAME: u32 = 8;
pub const SVTI2_REMAP_PIPE_NAMES: u32 = 2;
pub const SVTI2_RESERVED1: u32 = 4096;
pub const SVTI2_RESERVED2: u32 = 8192;
pub const SVTI2_RESERVED3: u32 = 16384;
pub const SVTI2_SCOPED_NAME: u32 = 4;
pub const SVTI2_UNICODE_TRANSPORT_ADDRESS: u32 = 32;
pub const SVTI2_VALID_FLAGS: u32 = 62;
pub const SV_ACCEPTDOWNLEVELAPIS_PARMNUM: u32 = 517;
pub const SV_ACCESSALERT_PARMNUM: u32 = 40;
pub const SV_ACTIVELOCKS_PARMNUM: u32 = 419;
pub const SV_ALERTSCHEDULE_INFOLEVEL: u32 = 1547;
pub const SV_ALERTSCHEDULE_PARMNUM: u32 = 547;
pub const SV_ALERTSCHED_PARMNUM: u32 = 37;
pub const SV_ALERTS_PARMNUM: u32 = 11;
pub const SV_ALIST_MTIME_PARMNUM: u32 = 403;
pub const SV_ANNDELTA_INFOLEVEL: u32 = 1018;
pub const SV_ANNDELTA_PARMNUM: u32 = 18;
pub const SV_ANNOUNCE_INFOLEVEL: u32 = 1017;
pub const SV_ANNOUNCE_PARMNUM: u32 = 17;
pub const SV_AUTOSHARESERVER_INFOLEVEL: u32 = 1592;
pub const SV_AUTOSHARESERVER_PARMNUM: u32 = 592;
pub const SV_AUTOSHAREWKS_INFOLEVEL: u32 = 1591;
pub const SV_AUTOSHAREWKS_PARMNUM: u32 = 591;
pub const SV_BALANCECOUNT_INFOLEVEL: u32 = 1577;
pub const SV_BALANCECOUNT_PARMNUM: u32 = 577;
pub const SV_CACHEDDIRECTORYLIMIT_INFOLEVEL: u32 = 1587;
pub const SV_CACHEDDIRECTORYLIMIT_PARMNUM: u32 = 587;
pub const SV_CACHEDOPENLIMIT_INFOLOEVEL: u32 = 1571;
pub const SV_CACHEDOPENLIMIT_PARMNUM: u32 = 571;
pub const SV_CHDEVJOBS_PARMNUM: u32 = 411;
pub const SV_CHDEVQ_PARMNUM: u32 = 410;
pub const SV_COMMENT_INFOLEVEL: u32 = 1005;
pub const SV_COMMENT_PARMNUM: u32 = 5;
pub const SV_CONNECTIONLESSAUTODISC_INFOLOEVEL: u32 = 1562;
pub const SV_CONNECTIONLESSAUTODISC_PARMNUM: u32 = 562;
pub const SV_CONNECTIONNOSESSIONSTIMEOUT_INFOLEVEL: u32 = 1596;
pub const SV_CONNECTIONNOSESSIONSTIMEOUT_PARMNUM: u32 = 596;
pub const SV_CONNECTIONS_PARMNUM: u32 = 412;
pub const SV_CRITICALTHREADS_INFOLOEVEL: u32 = 1572;
pub const SV_CRITICALTHREADS_PARMNUM: u32 = 572;
pub const SV_DISABLEDOS_INFOLEVEL: u32 = 1600;
pub const SV_DISABLEDOS_PARMNUM: u32 = 600;
pub const SV_DISABLESTRICTNAMECHECKING_INFOLEVEL: u32 = 1602;
pub const SV_DISABLESTRICTNAMECHECKING_PARMNUM: u32 = 602;
pub const SV_DISC_INFOLEVEL: u32 = 1010;
pub const SV_DISC_PARMNUM: u32 = 10;
pub const SV_DISKALERT_PARMNUM: u32 = 41;
pub const SV_DISKSPACETHRESHOLD_INFOLEVEL: u32 = 1550;
pub const SV_DISKSPACETHRESHOLD_PARMNUM: u32 = 550;
pub const SV_DOMAIN_PARMNUM: u32 = 519;
pub const SV_ENABLEAUTHENTICATEUSERSHARING_INFOLEVEL: u32 = 1603;
pub const SV_ENABLEAUTHENTICATEUSERSHARING_PARMNUM: u32 = 603;
pub const SV_ENABLECOMPRESSION_INFOLEVEL: u32 = 1590;
pub const SV_ENABLECOMPRESSION_PARMNUM: u32 = 590;
pub const SV_ENABLEFCBOPENS_INFOLEVEL: u32 = 1538;
pub const SV_ENABLEFCBOPENS_PARMNUM: u32 = 538;
pub const SV_ENABLEFORCEDLOGOFF_INFOLEVEL: u32 = 1515;
pub const SV_ENABLEFORCEDLOGOFF_PARMNUM: u32 = 515;
pub const SV_ENABLEOPLOCKFORCECLOSE_INFOLEVEL: u32 = 1537;
pub const SV_ENABLEOPLOCKFORCECLOSE_PARMNUM: u32 = 537;
pub const SV_ENABLEOPLOCKS_INFOLEVEL: u32 = 1536;
pub const SV_ENABLEOPLOCKS_PARMNUM: u32 = 536;
pub const SV_ENABLERAW_INFOLEVEL: u32 = 1539;
pub const SV_ENABLERAW_PARMNUM: u32 = 539;
pub const SV_ENABLESECURITYSIGNATURE_INFOLEVEL: u32 = 1593;
pub const SV_ENABLESECURITYSIGNATURE_PARMNUM: u32 = 593;
pub const SV_ENABLESHAREDNETDRIVES_INFOLEVEL: u32 = 1540;
pub const SV_ENABLESHAREDNETDRIVES_PARMNUM: u32 = 540;
pub const SV_ENABLESOFTCOMPAT_INFOLEVEL: u32 = 1514;
pub const SV_ENABLESOFTCOMPAT_PARMNUM: u32 = 514;
pub const SV_ENABLEW9XSECURITYSIGNATURE_INFOLEVEL: u32 = 1598;
pub const SV_ENABLEW9XSECURITYSIGNATURE_PARMNUM: u32 = 598;
pub const SV_ENABLEWFW311DIRECTIPX_INFOLOEVEL: u32 = 1574;
pub const SV_ENABLEWFW311DIRECTIPX_PARMNUM: u32 = 574;
pub const SV_ENFORCEKERBEROSREAUTHENTICATION_INFOLEVEL: u32 = 1599;
pub const SV_ENFORCEKERBEROSREAUTHENTICATION_PARMNUM: u32 = 599;
pub const SV_ERRORALERT_PARMNUM: u32 = 38;
pub const SV_ERRORTHRESHOLD_INFOLEVEL: u32 = 1548;
pub const SV_ERRORTHRESHOLD_PARMNUM: u32 = 548;
pub const SV_GLIST_MTIME_PARMNUM: u32 = 402;
pub const SV_GUESTACC_PARMNUM: u32 = 408;
pub const SV_HIDDEN: u32 = 1;
pub const SV_HIDDEN_INFOLEVEL: u32 = 1016;
pub const SV_HIDDEN_PARMNUM: u32 = 16;
pub const SV_IDLETHREADTIMEOUT_INFOLEVEL: u32 = 1597;
pub const SV_IDLETHREADTIMEOUT_PARMNUM: u32 = 597;
pub const SV_INITCONNTABLE_INFOLEVEL: u32 = 1544;
pub const SV_INITCONNTABLE_PARMNUM: u32 = 544;
pub const SV_INITFILETABLE_INFOLEVEL: u32 = 1545;
pub const SV_INITFILETABLE_PARMNUM: u32 = 545;
pub const SV_INITSEARCHTABLE_INFOLEVEL: u32 = 1546;
pub const SV_INITSEARCHTABLE_PARMNUM: u32 = 546;
pub const SV_INITSESSTABLE_INFOLEVEL: u32 = 1543;
pub const SV_INITSESSTABLE_PARMNUM: u32 = 543;
pub const SV_INITWORKITEMS_PARMNUM: u32 = 505;
pub const SV_IRPSTACKSIZE_PARMNUM: u32 = 508;
pub const SV_LANMASK_PARMNUM: u32 = 407;
pub const SV_LINKINFOVALIDTIME_INFOLEVEL: u32 = 1554;
pub const SV_LINKINFOVALIDTIME_PARMNUM: u32 = 554;
pub const SV_LMANNOUNCE_INFOLEVEL: u32 = 1518;
pub const SV_LMANNOUNCE_PARMNUM: u32 = 518;
pub const SV_LOCKVIOLATIONDELAY_INFOLOEVEL: u32 = 1569;
pub const SV_LOCKVIOLATIONDELAY_PARMNUM: u32 = 569;
pub const SV_LOCKVIOLATIONOFFSET_INFOLOEVEL: u32 = 1568;
pub const SV_LOCKVIOLATIONOFFSET_PARMNUM: u32 = 568;
pub const SV_LOCKVIOLATIONRETRIES_INFOLOEVEL: u32 = 1567;
pub const SV_LOCKVIOLATIONRETRIES_PARMNUM: u32 = 567;
pub const SV_LOGONALERT_PARMNUM: u32 = 39;
pub const SV_LOWDISKSPACEMINIMUM_INFOLEVEL: u32 = 1601;
pub const SV_LOWDISKSPACEMINIMUM_PARMNUM: u32 = 601;
pub const SV_MAXAUDITSZ_PARMNUM: u32 = 43;
pub const SV_MAXCOPYLENGTH_INFOLEVEL: u32 = 1588;
pub const SV_MAXCOPYLENGTH_PARMNUM: u32 = 588;
pub const SV_MAXCOPYREADLEN_INFOLEVEL: u32 = 1520;
pub const SV_MAXCOPYREADLEN_PARMNUM: u32 = 520;
pub const SV_MAXCOPYWRITELEN_INFOLEVEL: u32 = 1521;
pub const SV_MAXCOPYWRITELEN_PARMNUM: u32 = 521;
pub const SV_MAXFREECONNECTIONS_INFOLEVEL: u32 = 1542;
pub const SV_MAXFREECONNECTIONS_PARMNUM: u32 = 542;
pub const SV_MAXFREELFCBS_INFOLEVEL: u32 = 1581;
pub const SV_MAXFREELFCBS_PARMNUM: u32 = 581;
pub const SV_MAXFREEMFCBS_INFOLEVEL: u32 = 1580;
pub const SV_MAXFREEMFCBS_PARMNUM: u32 = 580;
pub const SV_MAXFREEPAGEDPOOLCHUNKS_INFOLEVEL: u32 = 1582;
pub const SV_MAXFREEPAGEDPOOLCHUNKS_PARMNUM: u32 = 582;
pub const SV_MAXFREERFCBS_INFOLEVEL: u32 = 1579;
pub const SV_MAXFREERFCBS_PARMNUM: u32 = 579;
pub const SV_MAXGLOBALOPENSEARCH_INFOLOEVEL: u32 = 1565;
pub const SV_MAXGLOBALOPENSEARCH_PARMNUM: u32 = 565;
pub const SV_MAXKEEPCOMPLSEARCH_INFOLEVEL: u32 = 1525;
pub const SV_MAXKEEPCOMPLSEARCH_PARMNUM: u32 = 525;
pub const SV_MAXKEEPSEARCH_INFOLEVEL: u32 = 1523;
pub const SV_MAXKEEPSEARCH_PARMNUM: u32 = 523;
pub const SV_MAXLINKDELAY_INFOLEVEL: u32 = 1552;
pub const SV_MAXLINKDELAY_PARMNUM: u32 = 552;
pub const SV_MAXMPXCT_INFOLEVEL: u32 = 1533;
pub const SV_MAXMPXCT_PARMNUM: u32 = 533;
pub const SV_MAXNONPAGEDMEMORYUSAGE_INFOLEVEL: u32 = 1512;
pub const SV_MAXNONPAGEDMEMORYUSAGE_PARMNUM: u32 = 512;
pub const SV_MAXPAGEDMEMORYUSAGE_INFOLEVEL: u32 = 1513;
pub const SV_MAXPAGEDMEMORYUSAGE_PARMNUM: u32 = 513;
pub const SV_MAXPAGEDPOOLCHUNKSIZE_INFOLEVEL: u32 = 1584;
pub const SV_MAXPAGEDPOOLCHUNKSIZE_PARMNUM: u32 = 584;
pub const SV_MAXRAWBUFLEN_INFOLEVEL: u32 = 1509;
pub const SV_MAXRAWBUFLEN_PARMNUM: u32 = 509;
pub const SV_MAXRAWWORKITEMS_INFOLOEVEL: u32 = 1557;
pub const SV_MAXRAWWORKITEMS_PARMNUM: u32 = 557;
pub const SV_MAXTHREADSPERQUEUE_INFOLEVEL: u32 = 1586;
pub const SV_MAXTHREADSPERQUEUE_PARMNUM: u32 = 586;
pub const SV_MAXWORKITEMIDLETIME_INFOLEVEL: u32 = 1556;
pub const SV_MAXWORKITEMIDLETIME_PARMNUM: u32 = 556;
pub const SV_MAXWORKITEMS_INFOLEVEL: u32 = 1506;
pub const SV_MAXWORKITEMS_PARMNUM: u32 = 506;
pub const SV_MAX_CMD_LEN: u32 = 256;
pub const SV_MAX_SRV_HEUR_LEN: u32 = 32;
pub const SV_MDLREADSWITCHOVER_INFOLOEVEL: u32 = 1570;
pub const SV_MDLREADSWITCHOVER_PARMNUM: u32 = 570;
pub const SV_MINCLIENTBUFFERSIZE_INFOLEVEL: u32 = 1595;
pub const SV_MINCLIENTBUFFERSIZE_PARMNUM: u32 = 595;
pub const SV_MINFREECONNECTIONS_INFOLEVEL: u32 = 1541;
pub const SV_MINFREECONNECTIONS_PARMNUM: u32 = 541;
pub const SV_MINFREEWORKITEMS_INFOLEVEL: u32 = 1530;
pub const SV_MINFREEWORKITEMS_PARMNUM: u32 = 530;
pub const SV_MINKEEPCOMPLSEARCH_INFOLEVEL: u32 = 1524;
pub const SV_MINKEEPCOMPLSEARCH_PARMNUM: u32 = 524;
pub const SV_MINKEEPSEARCH_INFOLEVEL: u32 = 1522;
pub const SV_MINKEEPSEARCH_PARMNUM: u32 = 522;
pub const SV_MINLINKTHROUGHPUT_INFOLEVEL: u32 = 1553;
pub const SV_MINLINKTHROUGHPUT_PARMNUM: u32 = 553;
pub const SV_MINPAGEDPOOLCHUNKSIZE_INFOLEVEL: u32 = 1583;
pub const SV_MINPAGEDPOOLCHUNKSIZE_PARMNUM: u32 = 583;
pub const SV_MINRCVQUEUE_INFOLEVEL: u32 = 1529;
pub const SV_MINRCVQUEUE_PARMNUM: u32 = 529;
pub const SV_NAME_PARMNUM: u32 = 102;
pub const SV_NETIOALERT_PARMNUM: u32 = 42;
pub const SV_NETWORKERRORTHRESHOLD_INFOLEVEL: u32 = 1549;
pub const SV_NETWORKERRORTHRESHOLD_PARMNUM: u32 = 549;
pub const SV_NODISC: i32 = -1;
pub const SV_NUMADMIN_PARMNUM: u32 = 406;
pub const SV_NUMBIGBUF_PARMNUM: u32 = 422;
pub const SV_NUMBLOCKTHREADS_PARMNUM: u32 = 527;
pub const SV_NUMFILETASKS_PARMNUM: u32 = 423;
pub const SV_NUMREQBUF_PARMNUM: u32 = 420;
pub const SV_OPENFILES_PARMNUM: u32 = 414;
pub const SV_OPENSEARCH_INFOLEVEL: u32 = 1503;
pub const SV_OPENSEARCH_PARMNUM: u32 = 503;
pub const SV_OPLOCKBREAKRESPONSEWAIT_INFOLEVEL: u32 = 1535;
pub const SV_OPLOCKBREAKRESPONSEWAIT_PARMNUM: u32 = 535;
pub const SV_OPLOCKBREAKWAIT_INFOLEVEL: u32 = 1534;
pub const SV_OPLOCKBREAKWAIT_PARMNUM: u32 = 534;
pub const SV_OTHERQUEUEAFFINITY_INFOLEVEL: u32 = 1575;
pub const SV_OTHERQUEUEAFFINITY_PARMNUM: u32 = 575;
pub const SV_PLATFORM_ID_NT: u32 = 500;
pub const SV_PLATFORM_ID_OS2: u32 = 400;
pub const SV_PLATFORM_ID_PARMNUM: u32 = 101;
pub const SV_PREFERREDAFFINITY_INFOLEVEL: u32 = 1578;
pub const SV_PREFERREDAFFINITY_PARMNUM: u32 = 578;
pub const SV_PRODUCTTYPE_INFOLOEVEL: u32 = 1560;
pub const SV_PRODUCTTYPE_PARMNUM: u32 = 560;
pub const SV_QUEUESAMPLESECS_INFOLEVEL: u32 = 1576;
pub const SV_QUEUESAMPLESECS_PARMNUM: u32 = 576;
pub const SV_RAWWORKITEMS_PARMNUM: u32 = 507;
pub const SV_REMOVEDUPLICATESEARCHES_INFOLOEVEL: u32 = 1566;
pub const SV_REMOVEDUPLICATESEARCHES_PARMNUM: u32 = 566;
pub const SV_REQUIRESECURITYSIGNATURE_INFOLEVEL: u32 = 1594;
pub const SV_REQUIRESECURITYSIGNATURE_PARMNUM: u32 = 594;
pub const SV_RESTRICTNULLSESSACCESS_INFOLOEVEL: u32 = 1573;
pub const SV_RESTRICTNULLSESSACCESS_PARMNUM: u32 = 573;
pub const SV_SCAVQOSINFOUPDATETIME_INFOLEVEL: u32 = 1555;
pub const SV_SCAVQOSINFOUPDATETIME_PARMNUM: u32 = 555;
pub const SV_SCAVTIMEOUT_INFOLEVEL: u32 = 1528;
pub const SV_SCAVTIMEOUT_PARMNUM: u32 = 528;
pub const SV_SECURITY_PARMNUM: u32 = 405;
pub const SV_SENDSFROMPREFERREDPROCESSOR_INFOLEVEL: u32 = 1585;
pub const SV_SENDSFROMPREFERREDPROCESSOR_PARMNUM: u32 = 585;
pub const SV_SERVERSIZE_INFOLOEVEL: u32 = 1561;
pub const SV_SERVERSIZE_PARMNUM: u32 = 561;
pub const SV_SESSCONNS_INFOLEVEL: u32 = 1511;
pub const SV_SESSCONNS_PARMNUM: u32 = 511;
pub const SV_SESSOPENS_INFOLEVEL: u32 = 1501;
pub const SV_SESSOPENS_PARMNUM: u32 = 501;
pub const SV_SESSREQS_PARMNUM: u32 = 417;
pub const SV_SESSUSERS_INFOLEVEL: u32 = 1510;
pub const SV_SESSUSERS_PARMNUM: u32 = 510;
pub const SV_SESSVCS_INFOLEVEL: u32 = 1502;
pub const SV_SESSVCS_PARMNUM: u32 = 502;
pub const SV_SHARESECURITY: u32 = 0;
pub const SV_SHARES_PARMNUM: u32 = 413;
pub const SV_SHARINGVIOLATIONDELAY_INFOLOEVEL: u32 = 1564;
pub const SV_SHARINGVIOLATIONDELAY_PARMNUM: u32 = 564;
pub const SV_SHARINGVIOLATIONRETRIES_INFOLOEVEL: u32 = 1563;
pub const SV_SHARINGVIOLATIONRETRIES_PARMNUM: u32 = 563;
pub const SV_SIZREQBUF_PARMNUM: u32 = 504;
pub const SV_SRVHEURISTICS_PARMNUM: u32 = 431;
pub const SV_THREADCOUNTADD_PARMNUM: u32 = 526;
pub const SV_THREADPRIORITY_PARMNUM: u32 = 532;
pub const SV_TIMESOURCE_INFOLEVEL: u32 = 1516;
pub const SV_TIMESOURCE_PARMNUM: u32 = 516;
pub const SV_TYPE_AFP: u32 = 64;
pub const SV_TYPE_ALL: u32 = 4294967295;
pub const SV_TYPE_ALTERNATE_XPORT: u32 = 536870912;
pub const SV_TYPE_BACKUP_BROWSER: u32 = 131072;
pub const SV_TYPE_CLUSTER_NT: u32 = 16777216;
pub const SV_TYPE_CLUSTER_VS_NT: u32 = 67108864;
pub const SV_TYPE_DCE: u32 = 268435456;
pub const SV_TYPE_DFS: u32 = 8388608;
pub const SV_TYPE_DIALIN_SERVER: u32 = 1024;
pub const SV_TYPE_DOMAIN_BAKCTRL: u32 = 16;
pub const SV_TYPE_DOMAIN_CTRL: u32 = 8;
pub const SV_TYPE_DOMAIN_ENUM: u32 = 2147483648;
pub const SV_TYPE_DOMAIN_MASTER: u32 = 524288;
pub const SV_TYPE_DOMAIN_MEMBER: u32 = 256;
pub const SV_TYPE_LOCAL_LIST_ONLY: u32 = 1073741824;
pub const SV_TYPE_MASTER_BROWSER: u32 = 262144;
pub const SV_TYPE_NOVELL: u32 = 128;
pub const SV_TYPE_NT: u32 = 4096;
pub const SV_TYPE_PARMNUM: u32 = 105;
pub const SV_TYPE_POTENTIAL_BROWSER: u32 = 65536;
pub const SV_TYPE_PRINTQ_SERVER: u32 = 512;
pub const SV_TYPE_SERVER: u32 = 2;
pub const SV_TYPE_SERVER_MFPN: u32 = 16384;
pub const SV_TYPE_SERVER_NT: u32 = 32768;
pub const SV_TYPE_SERVER_OSF: u32 = 1048576;
pub const SV_TYPE_SERVER_UNIX: u32 = 2048;
pub const SV_TYPE_SERVER_VMS: u32 = 2097152;
pub const SV_TYPE_SQLSERVER: u32 = 4;
pub const SV_TYPE_TERMINALSERVER: u32 = 33554432;
pub const SV_TYPE_TIME_SOURCE: u32 = 32;
pub const SV_TYPE_WFW: u32 = 8192;
pub const SV_TYPE_WINDOWS: u32 = 4194304;
pub const SV_TYPE_WORKSTATION: u32 = 1;
pub const SV_TYPE_XENIX_SERVER: u32 = 2048;
pub const SV_ULIST_MTIME_PARMNUM: u32 = 401;
pub const SV_USERPATH_PARMNUM: u32 = 112;
pub const SV_USERSECURITY: u32 = 1;
pub const SV_USERS_INFOLEVEL: u32 = 1107;
pub const SV_USERS_PARMNUM: u32 = 107;
pub const SV_USERS_PER_LICENSE: u32 = 5;
pub const SV_VERSION_MAJOR_PARMNUM: u32 = 103;
pub const SV_VERSION_MINOR_PARMNUM: u32 = 104;
pub const SV_VISIBLE: u32 = 0;
pub const SV_XACTMEMSIZE_PARMNUM: u32 = 531;
pub const SW_AUTOPROF_LOAD_MASK: u32 = 1;
pub const SW_AUTOPROF_SAVE_MASK: u32 = 2;
