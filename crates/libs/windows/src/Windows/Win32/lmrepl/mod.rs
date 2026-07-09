#[inline]
pub unsafe fn NetReplExportDirAdd<P0>(servername: P0, level: u32, buf: *mut u8, parm_err: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirAdd(servername : windows_core::PCWSTR, level : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
    unsafe { NetReplExportDirAdd(servername.param().abi(), level, buf as _, parm_err as _) }
}
#[inline]
pub unsafe fn NetReplExportDirDel<P0, P1>(servername: P0, dirname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirDel(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR) -> u32);
    unsafe { NetReplExportDirDel(servername.param().abi(), dirname.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn NetReplExportDirEnum<P0>(servername: P0, level: u32, bufptr: *mut super::minwindef::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
    unsafe { NetReplExportDirEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resumehandle as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn NetReplExportDirGetInfo<P0, P1>(servername: P0, dirname: P1, level: u32, bufptr: *mut super::minwindef::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirGetInfo(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { NetReplExportDirGetInfo(servername.param().abi(), dirname.param().abi(), level, bufptr as _) }
}
#[inline]
pub unsafe fn NetReplExportDirLock<P0, P1>(servername: P0, dirname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirLock(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR) -> u32);
    unsafe { NetReplExportDirLock(servername.param().abi(), dirname.param().abi()) }
}
#[inline]
pub unsafe fn NetReplExportDirSetInfo<P0, P1>(servername: P0, dirname: P1, level: u32, buf: *mut u8, parm_err: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirSetInfo(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, level : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
    unsafe { NetReplExportDirSetInfo(servername.param().abi(), dirname.param().abi(), level, buf as _, parm_err as _) }
}
#[inline]
pub unsafe fn NetReplExportDirUnlock<P0, P1>(servername: P0, dirname: P1, unlockforce: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplExportDirUnlock(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, unlockforce : u32) -> u32);
    unsafe { NetReplExportDirUnlock(servername.param().abi(), dirname.param().abi(), unlockforce) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn NetReplGetInfo<P0>(servername: P0, level: u32, bufptr: *mut super::minwindef::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplGetInfo(servername : windows_core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { NetReplGetInfo(servername.param().abi(), level, bufptr as _) }
}
#[inline]
pub unsafe fn NetReplImportDirAdd<P0>(servername: P0, level: u32, buf: *mut u8, parm_err: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplImportDirAdd(servername : windows_core::PCWSTR, level : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
    unsafe { NetReplImportDirAdd(servername.param().abi(), level, buf as _, parm_err as _) }
}
#[inline]
pub unsafe fn NetReplImportDirDel<P0, P1>(servername: P0, dirname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplImportDirDel(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR) -> u32);
    unsafe { NetReplImportDirDel(servername.param().abi(), dirname.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn NetReplImportDirEnum<P0>(servername: P0, level: u32, bufptr: *mut super::minwindef::LPBYTE, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplImportDirEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
    unsafe { NetReplImportDirEnum(servername.param().abi(), level, bufptr as _, prefmaxlen, entriesread as _, totalentries as _, resumehandle as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn NetReplImportDirGetInfo<P0, P1>(servername: P0, dirname: P1, level: u32, bufptr: *mut super::minwindef::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplImportDirGetInfo(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { NetReplImportDirGetInfo(servername.param().abi(), dirname.param().abi(), level, bufptr as _) }
}
#[inline]
pub unsafe fn NetReplImportDirLock<P0, P1>(servername: P0, dirname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplImportDirLock(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR) -> u32);
    unsafe { NetReplImportDirLock(servername.param().abi(), dirname.param().abi()) }
}
#[inline]
pub unsafe fn NetReplImportDirUnlock<P0, P1>(servername: P0, dirname: P1, unlockforce: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplImportDirUnlock(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, unlockforce : u32) -> u32);
    unsafe { NetReplImportDirUnlock(servername.param().abi(), dirname.param().abi(), unlockforce) }
}
#[inline]
pub unsafe fn NetReplSetInfo<P0>(servername: P0, level: u32, buf: *mut u8, parm_err: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetReplSetInfo(servername : windows_core::PCWSTR, level : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
    unsafe { NetReplSetInfo(servername.param().abi(), level, buf as _, parm_err as _) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREPL_EDIR_INFO_0(pub *mut REPL_EDIR_INFO_0);
impl LPREPL_EDIR_INFO_0 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREPL_EDIR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREPL_EDIR_INFO_1(pub *mut REPL_EDIR_INFO_1);
impl LPREPL_EDIR_INFO_1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREPL_EDIR_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREPL_EDIR_INFO_1000(pub *mut REPL_EDIR_INFO_1000);
impl LPREPL_EDIR_INFO_1000 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREPL_EDIR_INFO_1000 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREPL_EDIR_INFO_1001(pub *mut REPL_EDIR_INFO_1001);
impl LPREPL_EDIR_INFO_1001 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREPL_EDIR_INFO_1001 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREPL_EDIR_INFO_2(pub *mut REPL_EDIR_INFO_2);
impl LPREPL_EDIR_INFO_2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREPL_EDIR_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREPL_IDIR_INFO_0(pub *mut REPL_IDIR_INFO_0);
impl LPREPL_IDIR_INFO_0 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREPL_IDIR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREPL_IDIR_INFO_1(pub *mut REPL_IDIR_INFO_1);
impl LPREPL_IDIR_INFO_1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREPL_IDIR_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREPL_INFO_0(pub *mut REPL_INFO_0);
impl LPREPL_INFO_0 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREPL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREPL_INFO_1000(pub *mut REPL_INFO_1000);
impl LPREPL_INFO_1000 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREPL_INFO_1000 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREPL_INFO_1001(pub *mut REPL_INFO_1001);
impl LPREPL_INFO_1001 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREPL_INFO_1001 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREPL_INFO_1002(pub *mut REPL_INFO_1002);
impl LPREPL_INFO_1002 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREPL_INFO_1002 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREPL_INFO_1003(pub *mut REPL_INFO_1003);
impl LPREPL_INFO_1003 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREPL_INFO_1003 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPL_EDIR_INFO_0(pub *mut REPL_EDIR_INFO_0);
impl PREPL_EDIR_INFO_0 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPL_EDIR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPL_EDIR_INFO_1(pub *mut REPL_EDIR_INFO_1);
impl PREPL_EDIR_INFO_1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPL_EDIR_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPL_EDIR_INFO_1000(pub *mut REPL_EDIR_INFO_1000);
impl PREPL_EDIR_INFO_1000 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPL_EDIR_INFO_1000 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPL_EDIR_INFO_1001(pub *mut REPL_EDIR_INFO_1001);
impl PREPL_EDIR_INFO_1001 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPL_EDIR_INFO_1001 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPL_EDIR_INFO_2(pub *mut REPL_EDIR_INFO_2);
impl PREPL_EDIR_INFO_2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPL_EDIR_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPL_IDIR_INFO_0(pub *mut REPL_IDIR_INFO_0);
impl PREPL_IDIR_INFO_0 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPL_IDIR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPL_IDIR_INFO_1(pub *mut REPL_IDIR_INFO_1);
impl PREPL_IDIR_INFO_1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPL_IDIR_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPL_INFO_0(pub *mut REPL_INFO_0);
impl PREPL_INFO_0 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPL_INFO_1000(pub *mut REPL_INFO_1000);
impl PREPL_INFO_1000 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPL_INFO_1000 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPL_INFO_1001(pub *mut REPL_INFO_1001);
impl PREPL_INFO_1001 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPL_INFO_1001 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPL_INFO_1002(pub *mut REPL_INFO_1002);
impl PREPL_INFO_1002 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPL_INFO_1002 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPL_INFO_1003(pub *mut REPL_INFO_1003);
impl PREPL_INFO_1003 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPL_INFO_1003 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPL_EDIR_INFO_0 {
    pub rped0_dirname: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPL_EDIR_INFO_1 {
    pub rped1_dirname: windows_core::PWSTR,
    pub rped1_integrity: u32,
    pub rped1_extent: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPL_EDIR_INFO_1000 {
    pub rped1000_integrity: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPL_EDIR_INFO_1001 {
    pub rped1001_extent: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPL_EDIR_INFO_2 {
    pub rped2_dirname: windows_core::PWSTR,
    pub rped2_integrity: u32,
    pub rped2_extent: u32,
    pub rped2_lockcount: u32,
    pub rped2_locktime: u32,
}
pub const REPL_EXPORT_EXTENT_INFOLEVEL: u32 = 1001;
pub const REPL_EXPORT_INTEGRITY_INFOLEVEL: u32 = 1000;
pub const REPL_EXTENT_FILE: u32 = 1;
pub const REPL_EXTENT_TREE: u32 = 2;
pub const REPL_GUARDTIME_INFOLEVEL: u32 = 1002;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPL_IDIR_INFO_0 {
    pub rpid0_dirname: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPL_IDIR_INFO_1 {
    pub rpid1_dirname: windows_core::PWSTR,
    pub rpid1_state: u32,
    pub rpid1_mastername: windows_core::PWSTR,
    pub rpid1_last_update_time: u32,
    pub rpid1_lockcount: u32,
    pub rpid1_locktime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPL_INFO_0 {
    pub rp0_role: u32,
    pub rp0_exportpath: windows_core::PWSTR,
    pub rp0_exportlist: windows_core::PWSTR,
    pub rp0_importpath: windows_core::PWSTR,
    pub rp0_importlist: windows_core::PWSTR,
    pub rp0_logonusername: windows_core::PWSTR,
    pub rp0_interval: u32,
    pub rp0_pulse: u32,
    pub rp0_guardtime: u32,
    pub rp0_random: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPL_INFO_1000 {
    pub rp1000_interval: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPL_INFO_1001 {
    pub rp1001_pulse: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPL_INFO_1002 {
    pub rp1002_guardtime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPL_INFO_1003 {
    pub rp1003_random: u32,
}
pub const REPL_INTEGRITY_FILE: u32 = 1;
pub const REPL_INTEGRITY_TREE: u32 = 2;
pub const REPL_INTERVAL_INFOLEVEL: u32 = 1000;
pub const REPL_PULSE_INFOLEVEL: u32 = 1001;
pub const REPL_RANDOM_INFOLEVEL: u32 = 1003;
pub const REPL_ROLE_BOTH: u32 = 3;
pub const REPL_ROLE_EXPORT: u32 = 1;
pub const REPL_ROLE_IMPORT: u32 = 2;
pub const REPL_STATE_NEVER_REPLICATED: u32 = 3;
pub const REPL_STATE_NO_MASTER: u32 = 1;
pub const REPL_STATE_NO_SYNC: u32 = 2;
pub const REPL_STATE_OK: u32 = 0;
pub const REPL_UNLOCK_FORCE: u32 = 1;
pub const REPL_UNLOCK_NOFORCE: u32 = 0;
