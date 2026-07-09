#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetServiceControl(servername : windows_sys::core::PCWSTR, service : windows_sys::core::PCWSTR, opcode : u32, arg : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetServiceEnum(servername : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetServiceGetInfo(servername : windows_sys::core::PCWSTR, service : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetServiceInstall(servername : windows_sys::core::PCWSTR, service : windows_sys::core::PCWSTR, argc : u32, argv : *const windows_sys::core::PCWSTR, bufptr : *mut super::minwindef::LPBYTE) -> u32);
pub const LM20_SERVICE_ACTIVE: u32 = 0;
pub const LM20_SERVICE_CONTINUE_PENDING: u32 = 4;
pub const LM20_SERVICE_PAUSED: u32 = 12;
pub const LM20_SERVICE_PAUSE_PENDING: u32 = 8;
pub const LOWER_GET_HINT_MASK: u32 = 65280;
pub const LOWER_HINT_MASK: u32 = 255;
pub type LPSERVICE_INFO_0 = *mut SERVICE_INFO_0;
pub type LPSERVICE_INFO_1 = *mut SERVICE_INFO_1;
pub type LPSERVICE_INFO_2 = *mut SERVICE_INFO_2;
pub type PSERVICE_INFO_0 = *mut SERVICE_INFO_0;
pub type PSERVICE_INFO_1 = *mut SERVICE_INFO_1;
pub type PSERVICE_INFO_2 = *mut SERVICE_INFO_2;
pub const SERVICE2_BASE: u32 = 5600;
pub const SERVICE_BASE: u32 = 3050;
pub const SERVICE_CCP_CHKPT_NUM: u32 = 255;
pub const SERVICE_CCP_NO_HINT: u32 = 0;
pub const SERVICE_CCP_QUERY_HINT: u32 = 65536;
pub const SERVICE_CCP_WAIT_TIME: u32 = 65280;
pub const SERVICE_CTRL_CONTINUE: u32 = 2;
pub const SERVICE_CTRL_INTERROGATE: u32 = 0;
pub const SERVICE_CTRL_PAUSE: u32 = 1;
pub const SERVICE_CTRL_REDIR_COMM: u32 = 4;
pub const SERVICE_CTRL_REDIR_DISK: u32 = 1;
pub const SERVICE_CTRL_REDIR_PRINT: u32 = 2;
pub const SERVICE_CTRL_UNINSTALL: u32 = 3;
pub const SERVICE_DOS_ENCRYPTION: windows_sys::core::PCWSTR = windows_sys::core::w!("ENCRYPT");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_INFO_0 {
    pub svci0_name: windows_sys::core::PWSTR,
}
impl Default for SERVICE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_INFO_1 {
    pub svci1_name: windows_sys::core::PWSTR,
    pub svci1_status: u32,
    pub svci1_code: u32,
    pub svci1_pid: u32,
}
impl Default for SERVICE_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_INFO_2 {
    pub svci2_name: windows_sys::core::PWSTR,
    pub svci2_status: u32,
    pub svci2_code: u32,
    pub svci2_pid: u32,
    pub svci2_text: windows_sys::core::PWSTR,
    pub svci2_specific_error: u32,
    pub svci2_display_name: windows_sys::core::PWSTR,
}
impl Default for SERVICE_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVICE_INSTALLED: u32 = 3;
pub const SERVICE_INSTALL_PENDING: u32 = 1;
pub const SERVICE_INSTALL_STATE: u32 = 3;
pub const SERVICE_IP_CHKPT_NUM: u32 = 255;
pub const SERVICE_IP_NO_HINT: u32 = 0;
pub const SERVICE_IP_QUERY_HINT: u32 = 65536;
pub const SERVICE_IP_WAITTIME_SHIFT: u32 = 8;
pub const SERVICE_IP_WAIT_TIME: u32 = 65280;
pub const SERVICE_MAXTIME: u32 = 255;
pub const SERVICE_NOT_PAUSABLE: u32 = 0;
pub const SERVICE_NOT_UNINSTALLABLE: u32 = 0;
pub const SERVICE_NTIP_WAITTIME_SHIFT: u32 = 12;
pub const SERVICE_NT_MAXTIME: u32 = 65535;
pub const SERVICE_PAUSABLE: u32 = 32;
pub const SERVICE_PAUSE_STATE: u32 = 12;
pub const SERVICE_REDIR_COMM_PAUSED: u32 = 1024;
pub const SERVICE_REDIR_DISK_PAUSED: u32 = 256;
pub const SERVICE_REDIR_PAUSED: u32 = 1792;
pub const SERVICE_REDIR_PRINT_PAUSED: u32 = 512;
pub const SERVICE_RESRV_MASK: u32 = 131071;
pub const SERVICE_UIC_AMBIGPARM: u32 = 3058;
pub const SERVICE_UIC_BADPARMVAL: u32 = 3051;
pub const SERVICE_UIC_CONFIG: u32 = 3055;
pub const SERVICE_UIC_CONFLPARM: u32 = 3063;
pub const SERVICE_UIC_DUPPARM: u32 = 3059;
pub const SERVICE_UIC_EXEC: u32 = 3061;
pub const SERVICE_UIC_FILE: u32 = 3064;
pub const SERVICE_UIC_INTERNAL: u32 = 3057;
pub const SERVICE_UIC_KILL: u32 = 3060;
pub const SERVICE_UIC_MISSPARM: u32 = 3052;
pub const SERVICE_UIC_M_ADDPAK: u32 = 3090;
pub const SERVICE_UIC_M_ANNOUNCE: u32 = 3083;
pub const SERVICE_UIC_M_DATABASE_ERROR: u32 = 5602;
pub const SERVICE_UIC_M_DISK: u32 = 3071;
pub const SERVICE_UIC_M_ERRLOG: u32 = 3088;
pub const SERVICE_UIC_M_FILES: u32 = 3079;
pub const SERVICE_UIC_M_FILE_UW: u32 = 3089;
pub const SERVICE_UIC_M_LANGROUP: u32 = 3081;
pub const SERVICE_UIC_M_LANROOT: u32 = 3075;
pub const SERVICE_UIC_M_LAZY: u32 = 3091;
pub const SERVICE_UIC_M_LOGS: u32 = 3080;
pub const SERVICE_UIC_M_LSA_MACHINE_ACCT: u32 = 5601;
pub const SERVICE_UIC_M_MEMORY: u32 = 3070;
pub const SERVICE_UIC_M_MSGNAME: u32 = 3082;
pub const SERVICE_UIC_M_NETLOGON_AUTH: u32 = 3098;
pub const SERVICE_UIC_M_NETLOGON_DC_CFLCT: u32 = 3097;
pub const SERVICE_UIC_M_NETLOGON_MPATH: u32 = 5600;
pub const SERVICE_UIC_M_NETLOGON_NO_DC: u32 = 3096;
pub const SERVICE_UIC_M_NULL: u32 = 0;
pub const SERVICE_UIC_M_PROCESSES: u32 = 3073;
pub const SERVICE_UIC_M_REDIR: u32 = 3076;
pub const SERVICE_UIC_M_SECURITY: u32 = 3074;
pub const SERVICE_UIC_M_SEC_FILE_ERR: u32 = 3078;
pub const SERVICE_UIC_M_SERVER: u32 = 3077;
pub const SERVICE_UIC_M_SERVER_SEC_ERR: u32 = 3085;
pub const SERVICE_UIC_M_THREADS: u32 = 3072;
pub const SERVICE_UIC_M_UAS: u32 = 3084;
pub const SERVICE_UIC_M_UAS_INVALID_ROLE: u32 = 3095;
pub const SERVICE_UIC_M_UAS_MACHINE_ACCT: u32 = 3092;
pub const SERVICE_UIC_M_UAS_PROLOG: u32 = 3099;
pub const SERVICE_UIC_M_UAS_SERVERS_NMEMB: u32 = 3093;
pub const SERVICE_UIC_M_UAS_SERVERS_NOGRP: u32 = 3094;
pub const SERVICE_UIC_M_WKSTA: u32 = 3087;
pub const SERVICE_UIC_NORMAL: u32 = 0;
pub const SERVICE_UIC_RESOURCE: u32 = 3054;
pub const SERVICE_UIC_SUBSERV: u32 = 3062;
pub const SERVICE_UIC_SYSTEM: u32 = 3056;
pub const SERVICE_UIC_UNKPARM: u32 = 3053;
pub const SERVICE_UNINSTALLABLE: u32 = 16;
pub const SERVICE_UNINSTALLED: u32 = 0;
pub const SERVICE_UNINSTALL_PENDING: u32 = 2;
pub const UPPER_GET_HINT_MASK: u32 = 267386880;
pub const UPPER_HINT_MASK: u32 = 65280;
