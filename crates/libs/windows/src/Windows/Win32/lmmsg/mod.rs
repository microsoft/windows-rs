#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetMessageBufferSend<P0, P1, P2>(servername: P0, msgname: P1, fromname: P2, buf: super::LPBYTE, buflen: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetMessageBufferSend(servername : windows_core::PCWSTR, msgname : windows_core::PCWSTR, fromname : windows_core::PCWSTR, buf : super::LPBYTE, buflen : super::DWORD) -> super::DWORD);
    unsafe { NetMessageBufferSend(servername.param().abi(), msgname.param().abi(), fromname.param().abi(), buf, buflen) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetMessageNameAdd<P0, P1>(servername: P0, msgname: P1) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetMessageNameAdd(servername : windows_core::PCWSTR, msgname : windows_core::PCWSTR) -> super::DWORD);
    unsafe { NetMessageNameAdd(servername.param().abi(), msgname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetMessageNameDel<P0, P1>(servername: P0, msgname: P1) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetMessageNameDel(servername : windows_core::PCWSTR, msgname : windows_core::PCWSTR) -> super::DWORD);
    unsafe { NetMessageNameDel(servername.param().abi(), msgname.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetMessageNameEnum<P0>(servername: P0, level: super::DWORD, bufptr: *const super::LPBYTE, prefmaxlen: super::DWORD, entriesread: super::LPDWORD, totalentries: super::LPDWORD, resume_handle: super::LPDWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetMessageNameEnum(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : *const super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resume_handle : super::LPDWORD) -> super::DWORD);
    unsafe { NetMessageNameEnum(servername.param().abi(), level, bufptr, prefmaxlen, entriesread as _, totalentries as _, resume_handle as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetMessageNameGetInfo<P0, P1>(servername: P0, msgname: P1, level: super::DWORD, bufptr: *const super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetMessageNameGetInfo(servername : windows_core::PCWSTR, msgname : windows_core::PCWSTR, level : super::DWORD, bufptr : *const super::LPBYTE) -> super::DWORD);
    unsafe { NetMessageNameGetInfo(servername.param().abi(), msgname.param().abi(), level, bufptr) }
}
pub type LPMSG_INFO_0 = *mut MSG_INFO_0;
#[cfg(feature = "shared")]
pub type LPMSG_INFO_1 = *mut MSG_INFO_1;
pub const MSGNAME_FORWARDED_FROM: i32 = 16;
pub const MSGNAME_FORWARDED_TO: i32 = 4;
pub const MSGNAME_NOT_FORWARDED: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSG_INFO_0 {
    pub msgi0_name: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSG_INFO_1 {
    pub msgi1_name: windows_core::PWSTR,
    pub msgi1_forward_flag: super::DWORD,
    pub msgi1_forward: windows_core::PWSTR,
}
pub type PMSG_INFO_0 = *mut MSG_INFO_0;
#[cfg(feature = "shared")]
pub type PMSG_INFO_1 = *mut MSG_INFO_1;
