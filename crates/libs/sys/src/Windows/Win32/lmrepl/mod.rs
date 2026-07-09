windows_link::link!("netapi32.dll" "system" fn NetReplExportDirAdd(servername : windows_sys::core::PCWSTR, level : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetReplExportDirDel(servername : windows_sys::core::PCWSTR, dirname : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetReplExportDirEnum(servername : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetReplExportDirGetInfo(servername : windows_sys::core::PCWSTR, dirname : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetReplExportDirLock(servername : windows_sys::core::PCWSTR, dirname : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetReplExportDirSetInfo(servername : windows_sys::core::PCWSTR, dirname : windows_sys::core::PCWSTR, level : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetReplExportDirUnlock(servername : windows_sys::core::PCWSTR, dirname : windows_sys::core::PCWSTR, unlockforce : u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetReplGetInfo(servername : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetReplImportDirAdd(servername : windows_sys::core::PCWSTR, level : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetReplImportDirDel(servername : windows_sys::core::PCWSTR, dirname : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetReplImportDirEnum(servername : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetReplImportDirGetInfo(servername : windows_sys::core::PCWSTR, dirname : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetReplImportDirLock(servername : windows_sys::core::PCWSTR, dirname : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetReplImportDirUnlock(servername : windows_sys::core::PCWSTR, dirname : windows_sys::core::PCWSTR, unlockforce : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetReplSetInfo(servername : windows_sys::core::PCWSTR, level : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
pub type LPREPL_EDIR_INFO_0 = *mut REPL_EDIR_INFO_0;
pub type LPREPL_EDIR_INFO_1 = *mut REPL_EDIR_INFO_1;
pub type LPREPL_EDIR_INFO_1000 = *mut REPL_EDIR_INFO_1000;
pub type LPREPL_EDIR_INFO_1001 = *mut REPL_EDIR_INFO_1001;
pub type LPREPL_EDIR_INFO_2 = *mut REPL_EDIR_INFO_2;
pub type LPREPL_IDIR_INFO_0 = *mut REPL_IDIR_INFO_0;
pub type LPREPL_IDIR_INFO_1 = *mut REPL_IDIR_INFO_1;
pub type LPREPL_INFO_0 = *mut REPL_INFO_0;
pub type LPREPL_INFO_1000 = *mut REPL_INFO_1000;
pub type LPREPL_INFO_1001 = *mut REPL_INFO_1001;
pub type LPREPL_INFO_1002 = *mut REPL_INFO_1002;
pub type LPREPL_INFO_1003 = *mut REPL_INFO_1003;
pub type PREPL_EDIR_INFO_0 = *mut REPL_EDIR_INFO_0;
pub type PREPL_EDIR_INFO_1 = *mut REPL_EDIR_INFO_1;
pub type PREPL_EDIR_INFO_1000 = *mut REPL_EDIR_INFO_1000;
pub type PREPL_EDIR_INFO_1001 = *mut REPL_EDIR_INFO_1001;
pub type PREPL_EDIR_INFO_2 = *mut REPL_EDIR_INFO_2;
pub type PREPL_IDIR_INFO_0 = *mut REPL_IDIR_INFO_0;
pub type PREPL_IDIR_INFO_1 = *mut REPL_IDIR_INFO_1;
pub type PREPL_INFO_0 = *mut REPL_INFO_0;
pub type PREPL_INFO_1000 = *mut REPL_INFO_1000;
pub type PREPL_INFO_1001 = *mut REPL_INFO_1001;
pub type PREPL_INFO_1002 = *mut REPL_INFO_1002;
pub type PREPL_INFO_1003 = *mut REPL_INFO_1003;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPL_EDIR_INFO_0 {
    pub rped0_dirname: windows_sys::core::PWSTR,
}
impl Default for REPL_EDIR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPL_EDIR_INFO_1 {
    pub rped1_dirname: windows_sys::core::PWSTR,
    pub rped1_integrity: u32,
    pub rped1_extent: u32,
}
impl Default for REPL_EDIR_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REPL_EDIR_INFO_1000 {
    pub rped1000_integrity: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REPL_EDIR_INFO_1001 {
    pub rped1001_extent: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPL_EDIR_INFO_2 {
    pub rped2_dirname: windows_sys::core::PWSTR,
    pub rped2_integrity: u32,
    pub rped2_extent: u32,
    pub rped2_lockcount: u32,
    pub rped2_locktime: u32,
}
impl Default for REPL_EDIR_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REPL_EXPORT_EXTENT_INFOLEVEL: u32 = 1001;
pub const REPL_EXPORT_INTEGRITY_INFOLEVEL: u32 = 1000;
pub const REPL_EXTENT_FILE: u32 = 1;
pub const REPL_EXTENT_TREE: u32 = 2;
pub const REPL_GUARDTIME_INFOLEVEL: u32 = 1002;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPL_IDIR_INFO_0 {
    pub rpid0_dirname: windows_sys::core::PWSTR,
}
impl Default for REPL_IDIR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPL_IDIR_INFO_1 {
    pub rpid1_dirname: windows_sys::core::PWSTR,
    pub rpid1_state: u32,
    pub rpid1_mastername: windows_sys::core::PWSTR,
    pub rpid1_last_update_time: u32,
    pub rpid1_lockcount: u32,
    pub rpid1_locktime: u32,
}
impl Default for REPL_IDIR_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPL_INFO_0 {
    pub rp0_role: u32,
    pub rp0_exportpath: windows_sys::core::PWSTR,
    pub rp0_exportlist: windows_sys::core::PWSTR,
    pub rp0_importpath: windows_sys::core::PWSTR,
    pub rp0_importlist: windows_sys::core::PWSTR,
    pub rp0_logonusername: windows_sys::core::PWSTR,
    pub rp0_interval: u32,
    pub rp0_pulse: u32,
    pub rp0_guardtime: u32,
    pub rp0_random: u32,
}
impl Default for REPL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REPL_INFO_1000 {
    pub rp1000_interval: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REPL_INFO_1001 {
    pub rp1001_pulse: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REPL_INFO_1002 {
    pub rp1002_guardtime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
