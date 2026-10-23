#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetConnectionEnum<P0, P1>(servername: P0, qualifier: P1, level: super::DWORD, bufptr: *mut super::LPBYTE, prefmaxlen: super::DWORD, entriesread: super::LPDWORD, totalentries: super::LPDWORD, resume_handle: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetConnectionEnum(servername : windows_core::PCWSTR, qualifier : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resume_handle : super::LPDWORD) -> super::DWORD);
    unsafe { NetConnectionEnum(servername.param().abi(), qualifier.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetFileClose<P0>(servername: P0, fileid: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetFileClose(servername : windows_core::PCWSTR, fileid : super::DWORD) -> super::DWORD);
    unsafe { NetFileClose(servername.param().abi(), fileid) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetFileEnum<P0, P1, P2>(servername: P0, basepath: P1, username: P2, level: super::DWORD, bufptr: *mut super::LPBYTE, prefmaxlen: super::DWORD, entriesread: super::LPDWORD, totalentries: super::LPDWORD, resume_handle: Option<super::PDWORD_PTR>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetFileEnum(servername : windows_core::PCWSTR, basepath : windows_core::PCWSTR, username : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resume_handle : super::PDWORD_PTR) -> super::DWORD);
    unsafe { NetFileEnum(servername.param().abi(), basepath.param().abi(), username.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetFileGetInfo<P0>(servername: P0, fileid: super::DWORD, level: super::DWORD, bufptr: *mut super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetFileGetInfo(servername : windows_core::PCWSTR, fileid : super::DWORD, level : super::DWORD, bufptr : *mut super::LPBYTE) -> super::DWORD);
    unsafe { NetFileGetInfo(servername.param().abi(), fileid, level, bufptr as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetServerAliasAdd<P0>(servername: P0, level: super::DWORD, buf: super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerAliasAdd(servername : windows_core::PCWSTR, level : super::DWORD, buf : super::LPBYTE) -> super::DWORD);
    unsafe { NetServerAliasAdd(servername.param().abi(), level, buf) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetServerAliasDel<P0>(servername: P0, level: super::DWORD, buf: super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerAliasDel(servername : windows_core::PCWSTR, level : super::DWORD, buf : super::LPBYTE) -> super::DWORD);
    unsafe { NetServerAliasDel(servername.param().abi(), level, buf as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetServerAliasEnum<P0>(servername: P0, level: super::DWORD, bufptr: *mut super::LPBYTE, prefmaxlen: super::DWORD, entriesread: super::LPDWORD, totalentries: super::LPDWORD, resumehandle: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetServerAliasEnum(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resumehandle : super::LPDWORD) -> super::DWORD);
    unsafe { NetServerAliasEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetSessionDel<P0, P1, P2>(servername: P0, uncclientname: P1, username: P2) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetSessionDel(servername : windows_core::PCWSTR, uncclientname : windows_core::PCWSTR, username : windows_core::PCWSTR) -> super::DWORD);
    unsafe { NetSessionDel(servername.param().abi(), uncclientname.param().abi(), username.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetSessionEnum<P0, P1, P2>(servername: P0, uncclientname: P1, username: P2, level: super::DWORD, bufptr: *mut super::LPBYTE, prefmaxlen: super::DWORD, entriesread: super::LPDWORD, totalentries: super::LPDWORD, resume_handle: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetSessionEnum(servername : windows_core::PCWSTR, uncclientname : windows_core::PCWSTR, username : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resume_handle : super::LPDWORD) -> super::DWORD);
    unsafe { NetSessionEnum(servername.param().abi(), uncclientname.param().abi(), username.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetSessionGetInfo<P0, P1, P2>(servername: P0, uncclientname: P1, username: P2, level: super::DWORD, bufptr: *mut super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetSessionGetInfo(servername : windows_core::PCWSTR, uncclientname : windows_core::PCWSTR, username : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE) -> super::DWORD);
    unsafe { NetSessionGetInfo(servername.param().abi(), uncclientname.param().abi(), username.param().abi(), level, bufptr as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetShareAdd<P0>(servername: P0, level: super::DWORD, buf: super::LPBYTE, parm_err: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetShareAdd(servername : windows_core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
    unsafe { NetShareAdd(servername.param().abi(), level, buf as _, parm_err.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetShareCheck<P0, P1>(servername: P0, device: P1, r#type: super::LPDWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetShareCheck(servername : windows_core::PCWSTR, device : windows_core::PCWSTR, r#type : super::LPDWORD) -> super::DWORD);
    unsafe { NetShareCheck(servername.param().abi(), device.param().abi(), r#type as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetShareDel<P0, P1>(servername: P0, netname: P1, reserved: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetShareDel(servername : windows_core::PCWSTR, netname : windows_core::PCWSTR, reserved : super::DWORD) -> super::DWORD);
    unsafe { NetShareDel(servername.param().abi(), netname.param().abi(), reserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetShareDelEx<P0>(servername: P0, level: super::DWORD, buf: super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetShareDelEx(servername : windows_core::PCWSTR, level : super::DWORD, buf : super::LPBYTE) -> super::DWORD);
    unsafe { NetShareDelEx(servername.param().abi(), level, buf as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetShareDelSticky<P0, P1>(servername: P0, netname: P1, reserved: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetShareDelSticky(servername : windows_core::PCWSTR, netname : windows_core::PCWSTR, reserved : super::DWORD) -> super::DWORD);
    unsafe { NetShareDelSticky(servername.param().abi(), netname.param().abi(), reserved) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetShareEnum<P0>(servername: P0, level: super::DWORD, bufptr: *mut super::LPBYTE, prefmaxlen: super::DWORD, entriesread: super::LPDWORD, totalentries: super::LPDWORD, resume_handle: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetShareEnum(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resume_handle : super::LPDWORD) -> super::DWORD);
    unsafe { NetShareEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetShareEnumSticky<P0>(servername: P0, level: super::DWORD, bufptr: *mut super::LPBYTE, prefmaxlen: super::DWORD, entriesread: super::LPDWORD, totalentries: super::LPDWORD, resume_handle: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetShareEnumSticky(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resume_handle : super::LPDWORD) -> super::DWORD);
    unsafe { NetShareEnumSticky(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resume_handle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetShareGetInfo<P0, P1>(servername: P0, netname: P1, level: super::DWORD, bufptr: *mut super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetShareGetInfo(servername : windows_core::PCWSTR, netname : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE) -> super::DWORD);
    unsafe { NetShareGetInfo(servername.param().abi(), netname.param().abi(), level, bufptr as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetShareSetInfo<P0, P1>(servername: P0, netname: P1, level: super::DWORD, buf: super::LPBYTE, parm_err: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetShareSetInfo(servername : windows_core::PCWSTR, netname : windows_core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
    unsafe { NetShareSetInfo(servername.param().abi(), netname.param().abi(), level, buf as _, parm_err.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONNECTION_INFO_0 {
    pub coni0_id: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONNECTION_INFO_1 {
    pub coni1_id: super::DWORD,
    pub coni1_type: super::DWORD,
    pub coni1_num_opens: super::DWORD,
    pub coni1_num_users: super::DWORD,
    pub coni1_time: super::DWORD,
    pub coni1_username: windows_core::PWSTR,
    pub coni1_netname: windows_core::PWSTR,
}
pub const CSC_CACHE_AUTO_REINT: i32 = 16;
pub const CSC_CACHE_MANUAL_REINT: i32 = 0;
pub const CSC_CACHE_NONE: i32 = 48;
pub const CSC_CACHE_VDO: i32 = 32;
pub const CSC_MASK: i32 = 48;
pub const CSC_MASK_EXT: i32 = 8240;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_INFO_2 {
    pub fi2_id: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_INFO_3 {
    pub fi3_id: super::DWORD,
    pub fi3_permissions: super::DWORD,
    pub fi3_num_locks: super::DWORD,
    pub fi3_pathname: windows_core::PWSTR,
    pub fi3_username: windows_core::PWSTR,
}
#[cfg(feature = "shared")]
pub type LPCONNECTION_INFO_0 = *mut CONNECTION_INFO_0;
#[cfg(feature = "shared")]
pub type LPCONNECTION_INFO_1 = *mut CONNECTION_INFO_1;
#[cfg(feature = "shared")]
pub type LPFILE_INFO_2 = *mut FILE_INFO_2;
#[cfg(feature = "shared")]
pub type LPFILE_INFO_3 = *mut FILE_INFO_3;
#[cfg(feature = "shared")]
pub type LPSERVER_ALIAS_INFO_0 = *mut SERVER_ALIAS_INFO_0;
pub type LPSESSION_INFO_0 = *mut SESSION_INFO_0;
#[cfg(feature = "shared")]
pub type LPSESSION_INFO_1 = *mut SESSION_INFO_1;
#[cfg(feature = "shared")]
pub type LPSESSION_INFO_10 = *mut SESSION_INFO_10;
#[cfg(feature = "shared")]
pub type LPSESSION_INFO_2 = *mut SESSION_INFO_2;
#[cfg(feature = "shared")]
pub type LPSESSION_INFO_502 = *mut SESSION_INFO_502;
pub type LPSHARE_INFO_0 = *mut SHARE_INFO_0;
#[cfg(feature = "shared")]
pub type LPSHARE_INFO_1 = *mut SHARE_INFO_1;
pub type LPSHARE_INFO_1004 = *mut SHARE_INFO_1004;
#[cfg(feature = "shared")]
pub type LPSHARE_INFO_1005 = *mut SHARE_INFO_1005;
#[cfg(feature = "shared")]
pub type LPSHARE_INFO_1006 = *mut SHARE_INFO_1006;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSHARE_INFO_1501 = *mut SHARE_INFO_1501;
pub type LPSHARE_INFO_1503 = *mut SHARE_INFO_1503;
#[cfg(feature = "shared")]
pub type LPSHARE_INFO_2 = *mut SHARE_INFO_2;
#[cfg(feature = "shared")]
pub type LPSHARE_INFO_501 = *mut SHARE_INFO_501;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSHARE_INFO_502 = *mut SHARE_INFO_502;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSHARE_INFO_503 = *mut SHARE_INFO_503;
#[cfg(feature = "shared")]
pub type PCONNECTION_INFO_0 = *mut CONNECTION_INFO_0;
#[cfg(feature = "shared")]
pub type PCONNECTION_INFO_1 = *mut CONNECTION_INFO_1;
pub const PERM_FILE_CREATE: i32 = 4;
pub const PERM_FILE_READ: i32 = 1;
pub const PERM_FILE_WRITE: i32 = 2;
#[cfg(feature = "shared")]
pub type PFILE_INFO_2 = *mut FILE_INFO_2;
#[cfg(feature = "shared")]
pub type PFILE_INFO_3 = *mut FILE_INFO_3;
#[cfg(feature = "shared")]
pub type PSERVER_ALIAS_INFO_0 = *mut SERVER_ALIAS_INFO_0;
pub type PSESSION_INFO_0 = *mut SESSION_INFO_0;
#[cfg(feature = "shared")]
pub type PSESSION_INFO_1 = *mut SESSION_INFO_1;
#[cfg(feature = "shared")]
pub type PSESSION_INFO_10 = *mut SESSION_INFO_10;
#[cfg(feature = "shared")]
pub type PSESSION_INFO_2 = *mut SESSION_INFO_2;
#[cfg(feature = "shared")]
pub type PSESSION_INFO_502 = *mut SESSION_INFO_502;
pub type PSHARE_INFO_0 = *mut SHARE_INFO_0;
#[cfg(feature = "shared")]
pub type PSHARE_INFO_1 = *mut SHARE_INFO_1;
pub type PSHARE_INFO_1004 = *mut SHARE_INFO_1004;
#[cfg(feature = "shared")]
pub type PSHARE_INFO_1005 = *mut SHARE_INFO_1005;
#[cfg(feature = "shared")]
pub type PSHARE_INFO_1006 = *mut SHARE_INFO_1006;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSHARE_INFO_1501 = *mut SHARE_INFO_1501;
pub type PSHARE_INFO_1503 = *mut SHARE_INFO_1503;
#[cfg(feature = "shared")]
pub type PSHARE_INFO_2 = *mut SHARE_INFO_2;
#[cfg(feature = "shared")]
pub type PSHARE_INFO_501 = *mut SHARE_INFO_501;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSHARE_INFO_502 = *mut SHARE_INFO_502;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSHARE_INFO_503 = *mut SHARE_INFO_503;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVER_ALIAS_INFO_0 {
    pub srvai0_alias: windows_core::PWSTR,
    pub srvai0_target: windows_core::PWSTR,
    pub srvai0_default: super::BOOLEAN,
    pub srvai0_reserved: super::ULONG,
}
pub const SESI1_NUM_ELEMENTS: i32 = 8;
pub const SESI2_NUM_ELEMENTS: i32 = 9;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SESSION_INFO_0 {
    pub sesi0_cname: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SESSION_INFO_1 {
    pub sesi1_cname: windows_core::PWSTR,
    pub sesi1_username: windows_core::PWSTR,
    pub sesi1_num_opens: super::DWORD,
    pub sesi1_time: super::DWORD,
    pub sesi1_idle_time: super::DWORD,
    pub sesi1_user_flags: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SESSION_INFO_10 {
    pub sesi10_cname: windows_core::PWSTR,
    pub sesi10_username: windows_core::PWSTR,
    pub sesi10_time: super::DWORD,
    pub sesi10_idle_time: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SESSION_INFO_2 {
    pub sesi2_cname: windows_core::PWSTR,
    pub sesi2_username: windows_core::PWSTR,
    pub sesi2_num_opens: super::DWORD,
    pub sesi2_time: super::DWORD,
    pub sesi2_idle_time: super::DWORD,
    pub sesi2_user_flags: super::DWORD,
    pub sesi2_cltype_name: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SESSION_INFO_502 {
    pub sesi502_cname: windows_core::PWSTR,
    pub sesi502_username: windows_core::PWSTR,
    pub sesi502_num_opens: super::DWORD,
    pub sesi502_time: super::DWORD,
    pub sesi502_idle_time: super::DWORD,
    pub sesi502_user_flags: super::DWORD,
    pub sesi502_cltype_name: windows_core::PWSTR,
    pub sesi502_transport: windows_core::PWSTR,
}
pub const SESS_GUEST: i32 = 1;
pub const SESS_NOENCRYPTION: i32 = 2;
pub const SHARE_CURRENT_USES_PARMNUM: i32 = 7;
pub const SHARE_FILE_SD_INFOLEVEL: i32 = 1501;
pub const SHARE_FILE_SD_PARMNUM: i32 = 501;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHARE_INFO_0 {
    pub shi0_netname: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHARE_INFO_1 {
    pub shi1_netname: windows_core::PWSTR,
    pub shi1_type: super::DWORD,
    pub shi1_remark: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHARE_INFO_1004 {
    pub shi1004_remark: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHARE_INFO_1005 {
    pub shi1005_flags: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHARE_INFO_1006 {
    pub shi1006_max_uses: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHARE_INFO_1501 {
    pub shi1501_reserved: super::DWORD,
    pub shi1501_security_descriptor: super::PSECURITY_DESCRIPTOR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHARE_INFO_1503 {
    pub shi1503_sharefilter: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHARE_INFO_2 {
    pub shi2_netname: windows_core::PWSTR,
    pub shi2_type: super::DWORD,
    pub shi2_remark: windows_core::PWSTR,
    pub shi2_permissions: super::DWORD,
    pub shi2_max_uses: super::DWORD,
    pub shi2_current_uses: super::DWORD,
    pub shi2_path: windows_core::PWSTR,
    pub shi2_passwd: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHARE_INFO_501 {
    pub shi501_netname: windows_core::PWSTR,
    pub shi501_type: super::DWORD,
    pub shi501_remark: windows_core::PWSTR,
    pub shi501_flags: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHARE_INFO_502 {
    pub shi502_netname: windows_core::PWSTR,
    pub shi502_type: super::DWORD,
    pub shi502_remark: windows_core::PWSTR,
    pub shi502_permissions: super::DWORD,
    pub shi502_max_uses: super::DWORD,
    pub shi502_current_uses: super::DWORD,
    pub shi502_path: windows_core::PWSTR,
    pub shi502_passwd: windows_core::PWSTR,
    pub shi502_reserved: super::DWORD,
    pub shi502_security_descriptor: super::PSECURITY_DESCRIPTOR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHARE_INFO_503 {
    pub shi503_netname: windows_core::PWSTR,
    pub shi503_type: super::DWORD,
    pub shi503_remark: windows_core::PWSTR,
    pub shi503_permissions: super::DWORD,
    pub shi503_max_uses: super::DWORD,
    pub shi503_current_uses: super::DWORD,
    pub shi503_path: windows_core::PWSTR,
    pub shi503_passwd: windows_core::PWSTR,
    pub shi503_servername: windows_core::PWSTR,
    pub shi503_reserved: super::DWORD,
    pub shi503_security_descriptor: super::PSECURITY_DESCRIPTOR,
}
pub const SHARE_MAX_USES_INFOLEVEL: i32 = 1006;
pub const SHARE_MAX_USES_PARMNUM: i32 = 6;
pub const SHARE_NETNAME_PARMNUM: i32 = 1;
pub const SHARE_PASSWD_PARMNUM: i32 = 9;
pub const SHARE_PATH_PARMNUM: i32 = 8;
pub const SHARE_PERMISSIONS_PARMNUM: i32 = 5;
pub const SHARE_QOS_POLICY_PARMNUM: i32 = 504;
pub const SHARE_REMARK_INFOLEVEL: i32 = 1004;
pub const SHARE_REMARK_PARMNUM: i32 = 4;
pub const SHARE_SERVER_PARMNUM: i32 = 503;
pub const SHARE_TYPE_PARMNUM: i32 = 3;
pub const SHI1005_FLAGS_ACCESS_BASED_DIRECTORY_ENUM: i32 = 2048;
pub const SHI1005_FLAGS_ALLOW_NAMESPACE_CACHING: i32 = 1024;
pub const SHI1005_FLAGS_CLUSTER_MANAGED: i32 = 524288;
pub const SHI1005_FLAGS_COMPRESS_DATA: i32 = 1048576;
pub const SHI1005_FLAGS_DFS: i32 = 1;
pub const SHI1005_FLAGS_DFS_ROOT: i32 = 2;
pub const SHI1005_FLAGS_DISABLE_CLIENT_BUFFERING: i32 = 131072;
pub const SHI1005_FLAGS_DISABLE_CLIENT_METADATA_CACHING: i32 = 8388608;
pub const SHI1005_FLAGS_DISABLE_DIRECTORY_HANDLE_LEASING: i32 = 4194304;
pub const SHI1005_FLAGS_ENABLE_CA: i32 = 16384;
pub const SHI1005_FLAGS_ENABLE_HASH: i32 = 8192;
pub const SHI1005_FLAGS_ENCRYPT_DATA: i32 = 32768;
pub const SHI1005_FLAGS_FORCE_LEVELII_OPLOCK: i32 = 4096;
pub const SHI1005_FLAGS_FORCE_SHARED_DELETE: i32 = 512;
pub const SHI1005_FLAGS_IDENTITY_REMOTING: i32 = 262144;
pub const SHI1005_FLAGS_ISOLATED_TRANSPORT: i32 = 2097152;
pub const SHI1005_FLAGS_RESERVED: i32 = 65536;
pub const SHI1005_FLAGS_RESTRICT_EXCLUSIVE_OPENS: i32 = 256;
pub const SHI1005_VALID_FLAGS_SET: i32 = 16777008;
pub const SHI1_NUM_ELEMENTS: i32 = 4;
pub const SHI2_NUM_ELEMENTS: i32 = 10;
#[cfg(feature = "shared")]
pub const SHI_USES_UNLIMITED: super::DWORD = 4294967295;
pub const STYPE_DEVICE: i32 = 2;
pub const STYPE_DISKTREE: i32 = 0;
pub const STYPE_IPC: i32 = 3;
pub const STYPE_MASK: i32 = 255;
pub const STYPE_PRINTQ: i32 = 1;
pub const STYPE_RESERVED1: i32 = 16777216;
pub const STYPE_RESERVED2: i32 = 33554432;
pub const STYPE_RESERVED3: i32 = 67108864;
pub const STYPE_RESERVED4: i32 = 134217728;
pub const STYPE_RESERVED5: i32 = 1048576;
pub const STYPE_RESERVED_ALL: i32 = 1073741568;
pub const STYPE_SPECIAL: u32 = 2147483648;
pub const STYPE_TEMPORARY: i32 = 1073741824;
