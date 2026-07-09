#[inline]
pub unsafe fn NetMessageBufferSend<P0, P1, P2>(servername: P0, msgname: P1, fromname: P2, buf: *const u8, buflen: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetMessageBufferSend(servername : windows_core::PCWSTR, msgname : windows_core::PCWSTR, fromname : windows_core::PCWSTR, buf : *const u8, buflen : u32) -> u32);
    unsafe { NetMessageBufferSend(servername.param().abi(), msgname.param().abi(), fromname.param().abi(), buf, buflen) }
}
#[inline]
pub unsafe fn NetMessageNameAdd<P0, P1>(servername: P0, msgname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetMessageNameAdd(servername : windows_core::PCWSTR, msgname : windows_core::PCWSTR) -> u32);
    unsafe { NetMessageNameAdd(servername.param().abi(), msgname.param().abi()) }
}
#[inline]
pub unsafe fn NetMessageNameDel<P0, P1>(servername: P0, msgname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetMessageNameDel(servername : windows_core::PCWSTR, msgname : windows_core::PCWSTR) -> u32);
    unsafe { NetMessageNameDel(servername.param().abi(), msgname.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetMessageNameEnum<P0>(servername: P0, level: u32, bufptr: *const super::minwindef::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetMessageNameEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *const super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    unsafe { NetMessageNameEnum(servername.param().abi(), level, bufptr, prefmaxlen, entriesread as _, totalentries as _, resume_handle as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetMessageNameGetInfo<P0, P1>(servername: P0, msgname: P1, level: u32, bufptr: *const super::minwindef::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetMessageNameGetInfo(servername : windows_core::PCWSTR, msgname : windows_core::PCWSTR, level : u32, bufptr : *const super::minwindef::LPBYTE) -> u32);
    unsafe { NetMessageNameGetInfo(servername.param().abi(), msgname.param().abi(), level, bufptr) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMSG_INFO_0(pub *mut MSG_INFO_0);
impl LPMSG_INFO_0 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMSG_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMSG_INFO_1(pub *mut MSG_INFO_1);
impl LPMSG_INFO_1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMSG_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSGNAME_FORWARDED_FROM: u32 = 16;
pub const MSGNAME_FORWARDED_TO: u32 = 4;
pub const MSGNAME_NOT_FORWARDED: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSG_INFO_0 {
    pub msgi0_name: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSG_INFO_1 {
    pub msgi1_name: windows_core::PWSTR,
    pub msgi1_forward_flag: u32,
    pub msgi1_forward: windows_core::PWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMSG_INFO_0(pub *mut MSG_INFO_0);
impl PMSG_INFO_0 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMSG_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMSG_INFO_1(pub *mut MSG_INFO_1);
impl PMSG_INFO_1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMSG_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
