#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetReplExportDirAdd<P0>(servername: P0, level: super::DWORD, buf: super::LPBYTE, parm_err: super::LPDWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirAdd(servername : windows_core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
    unsafe { NetReplExportDirAdd(servername.param().abi(), level, buf as _, parm_err as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetReplExportDirDel<P0, P1>(servername: P0, dirname: P1) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirDel(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR) -> super::DWORD);
    unsafe { NetReplExportDirDel(servername.param().abi(), dirname.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetReplExportDirEnum<P0>(servername: P0, level: super::DWORD, bufptr: *mut super::LPBYTE, prefmaxlen: super::DWORD, entriesread: super::LPDWORD, totalentries: super::LPDWORD, resumehandle: super::LPDWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirEnum(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resumehandle : super::LPDWORD) -> super::DWORD);
    unsafe { NetReplExportDirEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resumehandle as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetReplExportDirGetInfo<P0, P1>(servername: P0, dirname: P1, level: super::DWORD, bufptr: *mut super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirGetInfo(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE) -> super::DWORD);
    unsafe { NetReplExportDirGetInfo(servername.param().abi(), dirname.param().abi(), level, bufptr as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetReplExportDirLock<P0, P1>(servername: P0, dirname: P1) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirLock(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR) -> super::DWORD);
    unsafe { NetReplExportDirLock(servername.param().abi(), dirname.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetReplExportDirSetInfo<P0, P1>(servername: P0, dirname: P1, level: super::DWORD, buf: super::LPBYTE, parm_err: super::LPDWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirSetInfo(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
    unsafe { NetReplExportDirSetInfo(servername.param().abi(), dirname.param().abi(), level, buf as _, parm_err as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetReplExportDirUnlock<P0, P1>(servername: P0, dirname: P1, unlockforce: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirUnlock(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, unlockforce : super::DWORD) -> super::DWORD);
    unsafe { NetReplExportDirUnlock(servername.param().abi(), dirname.param().abi(), unlockforce) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetReplGetInfo<P0>(servername: P0, level: super::DWORD, bufptr: *mut super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplGetInfo(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE) -> super::DWORD);
    unsafe { NetReplGetInfo(servername.param().abi(), level, bufptr as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetReplImportDirAdd<P0>(servername: P0, level: super::DWORD, buf: super::LPBYTE, parm_err: super::LPDWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplImportDirAdd(servername : windows_core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
    unsafe { NetReplImportDirAdd(servername.param().abi(), level, buf as _, parm_err as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetReplImportDirDel<P0, P1>(servername: P0, dirname: P1) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplImportDirDel(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR) -> super::DWORD);
    unsafe { NetReplImportDirDel(servername.param().abi(), dirname.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetReplImportDirEnum<P0>(servername: P0, level: super::DWORD, bufptr: *mut super::LPBYTE, prefmaxlen: super::DWORD, entriesread: super::LPDWORD, totalentries: super::LPDWORD, resumehandle: super::LPDWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplImportDirEnum(servername : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE, prefmaxlen : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resumehandle : super::LPDWORD) -> super::DWORD);
    unsafe { NetReplImportDirEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resumehandle as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetReplImportDirGetInfo<P0, P1>(servername: P0, dirname: P1, level: super::DWORD, bufptr: *mut super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplImportDirGetInfo(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, level : super::DWORD, bufptr : *mut super::LPBYTE) -> super::DWORD);
    unsafe { NetReplImportDirGetInfo(servername.param().abi(), dirname.param().abi(), level, bufptr as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetReplImportDirLock<P0, P1>(servername: P0, dirname: P1) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplImportDirLock(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR) -> super::DWORD);
    unsafe { NetReplImportDirLock(servername.param().abi(), dirname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetReplImportDirUnlock<P0, P1>(servername: P0, dirname: P1, unlockforce: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplImportDirUnlock(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, unlockforce : super::DWORD) -> super::DWORD);
    unsafe { NetReplImportDirUnlock(servername.param().abi(), dirname.param().abi(), unlockforce) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetReplSetInfo<P0>(servername: P0, level: super::DWORD, buf: super::LPBYTE, parm_err: super::LPDWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplSetInfo(servername : windows_core::PCWSTR, level : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
    unsafe { NetReplSetInfo(servername.param().abi(), level, buf as _, parm_err as _) }
}
pub type LPREPL_EDIR_INFO_0 = *mut REPL_EDIR_INFO_0;
#[cfg(feature = "shared")]
pub type LPREPL_EDIR_INFO_1 = *mut REPL_EDIR_INFO_1;
#[cfg(feature = "shared")]
pub type LPREPL_EDIR_INFO_1000 = *mut REPL_EDIR_INFO_1000;
#[cfg(feature = "shared")]
pub type LPREPL_EDIR_INFO_1001 = *mut REPL_EDIR_INFO_1001;
#[cfg(feature = "shared")]
pub type LPREPL_EDIR_INFO_2 = *mut REPL_EDIR_INFO_2;
pub type LPREPL_IDIR_INFO_0 = *mut REPL_IDIR_INFO_0;
#[cfg(feature = "shared")]
pub type LPREPL_IDIR_INFO_1 = *mut REPL_IDIR_INFO_1;
#[cfg(feature = "shared")]
pub type LPREPL_INFO_0 = *mut REPL_INFO_0;
#[cfg(feature = "shared")]
pub type LPREPL_INFO_1000 = *mut REPL_INFO_1000;
#[cfg(feature = "shared")]
pub type LPREPL_INFO_1001 = *mut REPL_INFO_1001;
#[cfg(feature = "shared")]
pub type LPREPL_INFO_1002 = *mut REPL_INFO_1002;
#[cfg(feature = "shared")]
pub type LPREPL_INFO_1003 = *mut REPL_INFO_1003;
pub type PREPL_EDIR_INFO_0 = *mut REPL_EDIR_INFO_0;
#[cfg(feature = "shared")]
pub type PREPL_EDIR_INFO_1 = *mut REPL_EDIR_INFO_1;
#[cfg(feature = "shared")]
pub type PREPL_EDIR_INFO_1000 = *mut REPL_EDIR_INFO_1000;
#[cfg(feature = "shared")]
pub type PREPL_EDIR_INFO_1001 = *mut REPL_EDIR_INFO_1001;
#[cfg(feature = "shared")]
pub type PREPL_EDIR_INFO_2 = *mut REPL_EDIR_INFO_2;
pub type PREPL_IDIR_INFO_0 = *mut REPL_IDIR_INFO_0;
#[cfg(feature = "shared")]
pub type PREPL_IDIR_INFO_1 = *mut REPL_IDIR_INFO_1;
#[cfg(feature = "shared")]
pub type PREPL_INFO_0 = *mut REPL_INFO_0;
#[cfg(feature = "shared")]
pub type PREPL_INFO_1000 = *mut REPL_INFO_1000;
#[cfg(feature = "shared")]
pub type PREPL_INFO_1001 = *mut REPL_INFO_1001;
#[cfg(feature = "shared")]
pub type PREPL_INFO_1002 = *mut REPL_INFO_1002;
#[cfg(feature = "shared")]
pub type PREPL_INFO_1003 = *mut REPL_INFO_1003;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPL_EDIR_INFO_0 {
    pub rped0_dirname: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPL_EDIR_INFO_1 {
    pub rped1_dirname: windows_core::PWSTR,
    pub rped1_integrity: super::DWORD,
    pub rped1_extent: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPL_EDIR_INFO_1000 {
    pub rped1000_integrity: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPL_EDIR_INFO_1001 {
    pub rped1001_extent: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPL_EDIR_INFO_2 {
    pub rped2_dirname: windows_core::PWSTR,
    pub rped2_integrity: super::DWORD,
    pub rped2_extent: super::DWORD,
    pub rped2_lockcount: super::DWORD,
    pub rped2_locktime: super::DWORD,
}
pub const REPL_EXPORT_EXTENT_INFOLEVEL: i32 = 1001;
pub const REPL_EXPORT_INTEGRITY_INFOLEVEL: i32 = 1000;
pub const REPL_EXTENT_FILE: i32 = 1;
pub const REPL_EXTENT_TREE: i32 = 2;
pub const REPL_GUARDTIME_INFOLEVEL: i32 = 1002;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPL_IDIR_INFO_0 {
    pub rpid0_dirname: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPL_IDIR_INFO_1 {
    pub rpid1_dirname: windows_core::PWSTR,
    pub rpid1_state: super::DWORD,
    pub rpid1_mastername: windows_core::PWSTR,
    pub rpid1_last_update_time: super::DWORD,
    pub rpid1_lockcount: super::DWORD,
    pub rpid1_locktime: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPL_INFO_0 {
    pub rp0_role: super::DWORD,
    pub rp0_exportpath: windows_core::PWSTR,
    pub rp0_exportlist: windows_core::PWSTR,
    pub rp0_importpath: windows_core::PWSTR,
    pub rp0_importlist: windows_core::PWSTR,
    pub rp0_logonusername: windows_core::PWSTR,
    pub rp0_interval: super::DWORD,
    pub rp0_pulse: super::DWORD,
    pub rp0_guardtime: super::DWORD,
    pub rp0_random: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPL_INFO_1000 {
    pub rp1000_interval: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPL_INFO_1001 {
    pub rp1001_pulse: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPL_INFO_1002 {
    pub rp1002_guardtime: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPL_INFO_1003 {
    pub rp1003_random: super::DWORD,
}
pub const REPL_INTEGRITY_FILE: i32 = 1;
pub const REPL_INTEGRITY_TREE: i32 = 2;
pub const REPL_INTERVAL_INFOLEVEL: i32 = 1000;
pub const REPL_PULSE_INFOLEVEL: i32 = 1001;
pub const REPL_RANDOM_INFOLEVEL: i32 = 1003;
pub const REPL_ROLE_BOTH: i32 = 3;
pub const REPL_ROLE_EXPORT: i32 = 1;
pub const REPL_ROLE_IMPORT: i32 = 2;
pub const REPL_STATE_NEVER_REPLICATED: i32 = 3;
pub const REPL_STATE_NO_MASTER: i32 = 1;
pub const REPL_STATE_NO_SYNC: i32 = 2;
pub const REPL_STATE_OK: i32 = 0;
pub const REPL_UNLOCK_FORCE: i32 = 1;
pub const REPL_UNLOCK_NOFORCE: i32 = 0;
