#[cfg(feature = "minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetServiceControl(servername : windows_sys::core::PCWSTR, service : windows_sys::core::PCWSTR, opcode : u32, arg : u32, bufptr : *mut super::LPBYTE) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetServiceEnum(servername : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetServiceGetInfo(servername : windows_sys::core::PCWSTR, service : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::LPBYTE) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetServiceInstall(servername : windows_sys::core::PCWSTR, service : windows_sys::core::PCWSTR, argc : u32, argv : *const windows_sys::core::PCWSTR, bufptr : *mut super::LPBYTE) -> u32);
pub const LM20_SERVICE_ACTIVE: i32 = 0;
pub const LM20_SERVICE_CONTINUE_PENDING: i32 = 4;
pub const LM20_SERVICE_PAUSED: i32 = 12;
pub const LM20_SERVICE_PAUSE_PENDING: i32 = 8;
pub const LOWER_GET_HINT_MASK: i32 = 65280;
pub const LOWER_HINT_MASK: i32 = 255;
pub type LPSERVICE_INFO_0 = *mut SERVICE_INFO_0;
pub type LPSERVICE_INFO_1 = *mut SERVICE_INFO_1;
pub type LPSERVICE_INFO_2 = *mut SERVICE_INFO_2;
pub type PSERVICE_INFO_0 = *mut SERVICE_INFO_0;
pub type PSERVICE_INFO_1 = *mut SERVICE_INFO_1;
pub type PSERVICE_INFO_2 = *mut SERVICE_INFO_2;
pub const SERVICE2_BASE: i32 = 5600;
pub const SERVICE_BASE: i32 = 3050;
pub const SERVICE_CCP_CHKPT_NUM: i32 = 255;
pub const SERVICE_CCP_NO_HINT: i32 = 0;
pub const SERVICE_CCP_QUERY_HINT: i32 = 65536;
pub const SERVICE_CCP_WAIT_TIME: i32 = 65280;
pub const SERVICE_CTRL_CONTINUE: i32 = 2;
pub const SERVICE_CTRL_INTERROGATE: i32 = 0;
pub const SERVICE_CTRL_PAUSE: i32 = 1;
pub const SERVICE_CTRL_REDIR_COMM: i32 = 4;
pub const SERVICE_CTRL_REDIR_DISK: i32 = 1;
pub const SERVICE_CTRL_REDIR_PRINT: i32 = 2;
pub const SERVICE_CTRL_UNINSTALL: i32 = 3;
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
pub const SERVICE_INSTALLED: i32 = 3;
pub const SERVICE_INSTALL_PENDING: i32 = 1;
pub const SERVICE_INSTALL_STATE: i32 = 3;
pub const SERVICE_IP_CHKPT_NUM: i32 = 255;
pub const SERVICE_IP_NO_HINT: i32 = 0;
pub const SERVICE_IP_QUERY_HINT: i32 = 65536;
pub const SERVICE_IP_WAITTIME_SHIFT: i32 = 8;
pub const SERVICE_IP_WAIT_TIME: i32 = 65280;
pub const SERVICE_MAXTIME: i32 = 255;
pub const SERVICE_NOT_PAUSABLE: i32 = 0;
pub const SERVICE_NOT_UNINSTALLABLE: i32 = 0;
pub const SERVICE_NTIP_WAITTIME_SHIFT: i32 = 12;
pub const SERVICE_NT_MAXTIME: i32 = 65535;
pub const SERVICE_PAUSABLE: i32 = 32;
pub const SERVICE_PAUSE_STATE: i32 = 12;
pub const SERVICE_REDIR_COMM_PAUSED: i32 = 1024;
pub const SERVICE_REDIR_DISK_PAUSED: i32 = 256;
pub const SERVICE_REDIR_PAUSED: i32 = 1792;
pub const SERVICE_REDIR_PRINT_PAUSED: i32 = 512;
pub const SERVICE_RESRV_MASK: i32 = 131071;
pub const SERVICE_UIC_AMBIGPARM: i32 = 3058;
pub const SERVICE_UIC_BADPARMVAL: i32 = 3051;
pub const SERVICE_UIC_CONFIG: i32 = 3055;
pub const SERVICE_UIC_CONFLPARM: i32 = 3063;
pub const SERVICE_UIC_DUPPARM: i32 = 3059;
pub const SERVICE_UIC_EXEC: i32 = 3061;
pub const SERVICE_UIC_FILE: i32 = 3064;
pub const SERVICE_UIC_INTERNAL: i32 = 3057;
pub const SERVICE_UIC_KILL: i32 = 3060;
pub const SERVICE_UIC_MISSPARM: i32 = 3052;
pub const SERVICE_UIC_M_ADDPAK: i32 = 3090;
pub const SERVICE_UIC_M_ANNOUNCE: i32 = 3083;
pub const SERVICE_UIC_M_DATABASE_ERROR: i32 = 5602;
pub const SERVICE_UIC_M_DISK: i32 = 3071;
pub const SERVICE_UIC_M_ERRLOG: i32 = 3088;
pub const SERVICE_UIC_M_FILES: i32 = 3079;
pub const SERVICE_UIC_M_FILE_UW: i32 = 3089;
pub const SERVICE_UIC_M_LANGROUP: i32 = 3081;
pub const SERVICE_UIC_M_LANROOT: i32 = 3075;
pub const SERVICE_UIC_M_LAZY: i32 = 3091;
pub const SERVICE_UIC_M_LOGS: i32 = 3080;
pub const SERVICE_UIC_M_LSA_MACHINE_ACCT: i32 = 5601;
pub const SERVICE_UIC_M_MEMORY: i32 = 3070;
pub const SERVICE_UIC_M_MSGNAME: i32 = 3082;
pub const SERVICE_UIC_M_NETLOGON_AUTH: i32 = 3098;
pub const SERVICE_UIC_M_NETLOGON_DC_CFLCT: i32 = 3097;
pub const SERVICE_UIC_M_NETLOGON_MPATH: i32 = 5600;
pub const SERVICE_UIC_M_NETLOGON_NO_DC: i32 = 3096;
pub const SERVICE_UIC_M_NULL: i32 = 0;
pub const SERVICE_UIC_M_PROCESSES: i32 = 3073;
pub const SERVICE_UIC_M_REDIR: i32 = 3076;
pub const SERVICE_UIC_M_SECURITY: i32 = 3074;
pub const SERVICE_UIC_M_SEC_FILE_ERR: i32 = 3078;
pub const SERVICE_UIC_M_SERVER: i32 = 3077;
pub const SERVICE_UIC_M_SERVER_SEC_ERR: i32 = 3085;
pub const SERVICE_UIC_M_THREADS: i32 = 3072;
pub const SERVICE_UIC_M_UAS: i32 = 3084;
pub const SERVICE_UIC_M_UAS_INVALID_ROLE: i32 = 3095;
pub const SERVICE_UIC_M_UAS_MACHINE_ACCT: i32 = 3092;
pub const SERVICE_UIC_M_UAS_PROLOG: i32 = 3099;
pub const SERVICE_UIC_M_UAS_SERVERS_NMEMB: i32 = 3093;
pub const SERVICE_UIC_M_UAS_SERVERS_NOGRP: i32 = 3094;
pub const SERVICE_UIC_M_WKSTA: i32 = 3087;
pub const SERVICE_UIC_NORMAL: i32 = 0;
pub const SERVICE_UIC_RESOURCE: i32 = 3054;
pub const SERVICE_UIC_SUBSERV: i32 = 3062;
pub const SERVICE_UIC_SYSTEM: i32 = 3056;
pub const SERVICE_UIC_UNKPARM: i32 = 3053;
pub const SERVICE_UNINSTALLABLE: i32 = 16;
pub const SERVICE_UNINSTALLED: i32 = 0;
pub const SERVICE_UNINSTALL_PENDING: i32 = 2;
pub const UPPER_GET_HINT_MASK: i32 = 267386880;
pub const UPPER_HINT_MASK: i32 = 65280;
