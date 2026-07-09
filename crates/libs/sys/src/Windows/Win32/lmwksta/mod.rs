#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetWkstaGetInfo(servername : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetWkstaSetInfo(servername : windows_sys::core::PCWSTR, level : u32, buffer : *const u8, parm_err : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("netapi32.dll" "system" fn NetWkstaTransportAdd(servername : super::winnt::LPTSTR, level : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetWkstaTransportDel(servername : windows_sys::core::PCWSTR, transportname : windows_sys::core::PCWSTR, ucond : u32) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("netapi32.dll" "system" fn NetWkstaTransportEnum(servername : super::winnt::LPTSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetWkstaUserEnum(servername : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetWkstaUserGetInfo(reserved : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetWkstaUserSetInfo(reserved : windows_sys::core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
pub type LPWKSTA_INFO_100 = *mut WKSTA_INFO_100;
pub type LPWKSTA_INFO_101 = *mut WKSTA_INFO_101;
pub type LPWKSTA_INFO_1010 = *mut WKSTA_INFO_1010;
pub type LPWKSTA_INFO_1011 = *mut WKSTA_INFO_1011;
pub type LPWKSTA_INFO_1012 = *mut WKSTA_INFO_1012;
pub type LPWKSTA_INFO_1013 = *mut WKSTA_INFO_1013;
pub type LPWKSTA_INFO_1018 = *mut WKSTA_INFO_1018;
pub type LPWKSTA_INFO_102 = *mut WKSTA_INFO_102;
pub type LPWKSTA_INFO_1023 = *mut WKSTA_INFO_1023;
pub type LPWKSTA_INFO_1027 = *mut WKSTA_INFO_1027;
pub type LPWKSTA_INFO_1028 = *mut WKSTA_INFO_1028;
pub type LPWKSTA_INFO_1032 = *mut WKSTA_INFO_1032;
pub type LPWKSTA_INFO_1033 = *mut WKSTA_INFO_1033;
pub type LPWKSTA_INFO_1041 = *mut WKSTA_INFO_1041;
pub type LPWKSTA_INFO_1042 = *mut WKSTA_INFO_1042;
pub type LPWKSTA_INFO_1043 = *mut WKSTA_INFO_1043;
pub type LPWKSTA_INFO_1044 = *mut WKSTA_INFO_1044;
pub type LPWKSTA_INFO_1045 = *mut WKSTA_INFO_1045;
pub type LPWKSTA_INFO_1046 = *mut WKSTA_INFO_1046;
pub type LPWKSTA_INFO_1047 = *mut WKSTA_INFO_1047;
pub type LPWKSTA_INFO_1048 = *mut WKSTA_INFO_1048;
pub type LPWKSTA_INFO_1049 = *mut WKSTA_INFO_1049;
pub type LPWKSTA_INFO_1050 = *mut WKSTA_INFO_1050;
pub type LPWKSTA_INFO_1051 = *mut WKSTA_INFO_1051;
pub type LPWKSTA_INFO_1052 = *mut WKSTA_INFO_1052;
pub type LPWKSTA_INFO_1053 = *mut WKSTA_INFO_1053;
pub type LPWKSTA_INFO_1054 = *mut WKSTA_INFO_1054;
pub type LPWKSTA_INFO_1055 = *mut WKSTA_INFO_1055;
pub type LPWKSTA_INFO_1056 = *mut WKSTA_INFO_1056;
pub type LPWKSTA_INFO_1057 = *mut WKSTA_INFO_1057;
pub type LPWKSTA_INFO_1058 = *mut WKSTA_INFO_1058;
pub type LPWKSTA_INFO_1059 = *mut WKSTA_INFO_1059;
pub type LPWKSTA_INFO_1060 = *mut WKSTA_INFO_1060;
pub type LPWKSTA_INFO_1061 = *mut WKSTA_INFO_1061;
pub type LPWKSTA_INFO_1062 = *mut WKSTA_INFO_1062;
pub type LPWKSTA_INFO_302 = *mut WKSTA_INFO_302;
pub type LPWKSTA_INFO_402 = *mut WKSTA_INFO_402;
pub type LPWKSTA_INFO_502 = *mut WKSTA_INFO_502;
pub type LPWKSTA_TRANSPORT_INFO_0 = *mut WKSTA_TRANSPORT_INFO_0;
pub type LPWKSTA_USER_INFO_0 = *mut WKSTA_USER_INFO_0;
pub type LPWKSTA_USER_INFO_1 = *mut WKSTA_USER_INFO_1;
pub type LPWKSTA_USER_INFO_1101 = *mut WKSTA_USER_INFO_1101;
pub type PWKSTA_INFO_100 = *mut WKSTA_INFO_100;
pub type PWKSTA_INFO_101 = *mut WKSTA_INFO_101;
pub type PWKSTA_INFO_1010 = *mut WKSTA_INFO_1010;
pub type PWKSTA_INFO_1011 = *mut WKSTA_INFO_1011;
pub type PWKSTA_INFO_1012 = *mut WKSTA_INFO_1012;
pub type PWKSTA_INFO_1013 = *mut WKSTA_INFO_1013;
pub type PWKSTA_INFO_1018 = *mut WKSTA_INFO_1018;
pub type PWKSTA_INFO_102 = *mut WKSTA_INFO_102;
pub type PWKSTA_INFO_1023 = *mut WKSTA_INFO_1023;
pub type PWKSTA_INFO_1027 = *mut WKSTA_INFO_1027;
pub type PWKSTA_INFO_1028 = *mut WKSTA_INFO_1028;
pub type PWKSTA_INFO_1032 = *mut WKSTA_INFO_1032;
pub type PWKSTA_INFO_1033 = *mut WKSTA_INFO_1033;
pub type PWKSTA_INFO_1041 = *mut WKSTA_INFO_1041;
pub type PWKSTA_INFO_1042 = *mut WKSTA_INFO_1042;
pub type PWKSTA_INFO_1043 = *mut WKSTA_INFO_1043;
pub type PWKSTA_INFO_1044 = *mut WKSTA_INFO_1044;
pub type PWKSTA_INFO_1045 = *mut WKSTA_INFO_1045;
pub type PWKSTA_INFO_1046 = *mut WKSTA_INFO_1046;
pub type PWKSTA_INFO_1047 = *mut WKSTA_INFO_1047;
pub type PWKSTA_INFO_1048 = *mut WKSTA_INFO_1048;
pub type PWKSTA_INFO_1049 = *mut WKSTA_INFO_1049;
pub type PWKSTA_INFO_1050 = *mut WKSTA_INFO_1050;
pub type PWKSTA_INFO_1051 = *mut WKSTA_INFO_1051;
pub type PWKSTA_INFO_1052 = *mut WKSTA_INFO_1052;
pub type PWKSTA_INFO_1053 = *mut WKSTA_INFO_1053;
pub type PWKSTA_INFO_1054 = *mut WKSTA_INFO_1054;
pub type PWKSTA_INFO_1055 = *mut WKSTA_INFO_1055;
pub type PWKSTA_INFO_1056 = *mut WKSTA_INFO_1056;
pub type PWKSTA_INFO_1057 = *mut WKSTA_INFO_1057;
pub type PWKSTA_INFO_1058 = *mut WKSTA_INFO_1058;
pub type PWKSTA_INFO_1059 = *mut WKSTA_INFO_1059;
pub type PWKSTA_INFO_1060 = *mut WKSTA_INFO_1060;
pub type PWKSTA_INFO_1061 = *mut WKSTA_INFO_1061;
pub type PWKSTA_INFO_1062 = *mut WKSTA_INFO_1062;
pub type PWKSTA_INFO_302 = *mut WKSTA_INFO_302;
pub type PWKSTA_INFO_402 = *mut WKSTA_INFO_402;
pub type PWKSTA_INFO_502 = *mut WKSTA_INFO_502;
pub type PWKSTA_TRANSPORT_INFO_0 = *mut WKSTA_TRANSPORT_INFO_0;
pub type PWKSTA_USER_INFO_0 = *mut WKSTA_USER_INFO_0;
pub type PWKSTA_USER_INFO_1 = *mut WKSTA_USER_INFO_1;
pub type PWKSTA_USER_INFO_1101 = *mut WKSTA_USER_INFO_1101;
pub const TRANSPORT_NAME_PARMNUM: u32 = 202;
pub const TRANSPORT_QUALITYOFSERVICE_PARMNUM: u32 = 201;
pub const WKSTA_BUFFERNAMEDPIPES_PARMNUM: u32 = 51;
pub const WKSTA_BUFFERREADONLYFILES_PARMNUM: u32 = 59;
pub const WKSTA_BUFFILESWITHDENYWRITE_PARMNUM: u32 = 58;
pub const WKSTA_CACHEFILETIMEOUT_PARMNUM: u32 = 47;
pub const WKSTA_CHARCOUNT_PARMNUM: u32 = 12;
pub const WKSTA_CHARTIME_PARMNUM: u32 = 11;
pub const WKSTA_CHARWAIT_PARMNUM: u32 = 10;
pub const WKSTA_COMPUTERNAME_PARMNUM: u32 = 1;
pub const WKSTA_DORMANTFILELIMIT_PARMNUM: u32 = 46;
pub const WKSTA_ERRLOGSZ_PARMNUM: u32 = 27;
pub const WKSTA_FORCECORECREATEMODE_PARMNUM: u32 = 60;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WKSTA_INFO_100 {
    pub wki100_platform_id: u32,
    pub wki100_computername: windows_sys::core::PWSTR,
    pub wki100_langroup: windows_sys::core::PWSTR,
    pub wki100_ver_major: u32,
    pub wki100_ver_minor: u32,
}
impl Default for WKSTA_INFO_100 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WKSTA_INFO_101 {
    pub wki101_platform_id: u32,
    pub wki101_computername: windows_sys::core::PWSTR,
    pub wki101_langroup: windows_sys::core::PWSTR,
    pub wki101_ver_major: u32,
    pub wki101_ver_minor: u32,
    pub wki101_lanroot: windows_sys::core::PWSTR,
}
impl Default for WKSTA_INFO_101 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1010 {
    pub wki1010_char_wait: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1011 {
    pub wki1011_collection_time: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1012 {
    pub wki1012_maximum_collection_count: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1013 {
    pub wki1013_keep_conn: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1018 {
    pub wki1018_sess_timeout: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WKSTA_INFO_102 {
    pub wki102_platform_id: u32,
    pub wki102_computername: windows_sys::core::PWSTR,
    pub wki102_langroup: windows_sys::core::PWSTR,
    pub wki102_ver_major: u32,
    pub wki102_ver_minor: u32,
    pub wki102_lanroot: windows_sys::core::PWSTR,
    pub wki102_logged_on_users: u32,
}
impl Default for WKSTA_INFO_102 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1023 {
    pub wki1023_siz_char_buf: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1027 {
    pub wki1027_errlog_sz: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1028 {
    pub wki1028_print_buf_time: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1032 {
    pub wki1032_wrk_heuristics: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1033 {
    pub wki1033_max_threads: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1041 {
    pub wki1041_lock_quota: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1042 {
    pub wki1042_lock_increment: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1043 {
    pub wki1043_lock_maximum: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1044 {
    pub wki1044_pipe_increment: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1045 {
    pub wki1045_pipe_maximum: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1046 {
    pub wki1046_dormant_file_limit: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1047 {
    pub wki1047_cache_file_timeout: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1048 {
    pub wki1048_use_opportunistic_locking: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1049 {
    pub wki1049_use_unlock_behind: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1050 {
    pub wki1050_use_close_behind: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1051 {
    pub wki1051_buf_named_pipes: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1052 {
    pub wki1052_use_lock_read_unlock: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1053 {
    pub wki1053_utilize_nt_caching: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1054 {
    pub wki1054_use_raw_read: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1055 {
    pub wki1055_use_raw_write: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1056 {
    pub wki1056_use_write_raw_data: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1057 {
    pub wki1057_use_encryption: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1058 {
    pub wki1058_buf_files_deny_write: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1059 {
    pub wki1059_buf_read_only_files: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1060 {
    pub wki1060_force_core_create_mode: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1061 {
    pub wki1061_use_512_byte_max_transfer: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_1062 {
    pub wki1062_read_ahead_throughput: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WKSTA_INFO_302 {
    pub wki302_char_wait: u32,
    pub wki302_collection_time: u32,
    pub wki302_maximum_collection_count: u32,
    pub wki302_keep_conn: u32,
    pub wki302_keep_search: u32,
    pub wki302_max_cmds: u32,
    pub wki302_num_work_buf: u32,
    pub wki302_siz_work_buf: u32,
    pub wki302_max_wrk_cache: u32,
    pub wki302_sess_timeout: u32,
    pub wki302_siz_error: u32,
    pub wki302_num_alerts: u32,
    pub wki302_num_services: u32,
    pub wki302_errlog_sz: u32,
    pub wki302_print_buf_time: u32,
    pub wki302_num_char_buf: u32,
    pub wki302_siz_char_buf: u32,
    pub wki302_wrk_heuristics: windows_sys::core::PWSTR,
    pub wki302_mailslots: u32,
    pub wki302_num_dgram_buf: u32,
}
impl Default for WKSTA_INFO_302 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WKSTA_INFO_402 {
    pub wki402_char_wait: u32,
    pub wki402_collection_time: u32,
    pub wki402_maximum_collection_count: u32,
    pub wki402_keep_conn: u32,
    pub wki402_keep_search: u32,
    pub wki402_max_cmds: u32,
    pub wki402_num_work_buf: u32,
    pub wki402_siz_work_buf: u32,
    pub wki402_max_wrk_cache: u32,
    pub wki402_sess_timeout: u32,
    pub wki402_siz_error: u32,
    pub wki402_num_alerts: u32,
    pub wki402_num_services: u32,
    pub wki402_errlog_sz: u32,
    pub wki402_print_buf_time: u32,
    pub wki402_num_char_buf: u32,
    pub wki402_siz_char_buf: u32,
    pub wki402_wrk_heuristics: windows_sys::core::PWSTR,
    pub wki402_mailslots: u32,
    pub wki402_num_dgram_buf: u32,
    pub wki402_max_threads: u32,
}
impl Default for WKSTA_INFO_402 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WKSTA_INFO_502 {
    pub wki502_char_wait: u32,
    pub wki502_collection_time: u32,
    pub wki502_maximum_collection_count: u32,
    pub wki502_keep_conn: u32,
    pub wki502_max_cmds: u32,
    pub wki502_sess_timeout: u32,
    pub wki502_siz_char_buf: u32,
    pub wki502_max_threads: u32,
    pub wki502_lock_quota: u32,
    pub wki502_lock_increment: u32,
    pub wki502_lock_maximum: u32,
    pub wki502_pipe_increment: u32,
    pub wki502_pipe_maximum: u32,
    pub wki502_cache_file_timeout: u32,
    pub wki502_dormant_file_limit: u32,
    pub wki502_read_ahead_throughput: u32,
    pub wki502_num_mailslot_buffers: u32,
    pub wki502_num_srv_announce_buffers: u32,
    pub wki502_max_illegal_datagram_events: u32,
    pub wki502_illegal_datagram_event_reset_frequency: u32,
    pub wki502_log_election_packets: windows_sys::core::BOOL,
    pub wki502_use_opportunistic_locking: windows_sys::core::BOOL,
    pub wki502_use_unlock_behind: windows_sys::core::BOOL,
    pub wki502_use_close_behind: windows_sys::core::BOOL,
    pub wki502_buf_named_pipes: windows_sys::core::BOOL,
    pub wki502_use_lock_read_unlock: windows_sys::core::BOOL,
    pub wki502_utilize_nt_caching: windows_sys::core::BOOL,
    pub wki502_use_raw_read: windows_sys::core::BOOL,
    pub wki502_use_raw_write: windows_sys::core::BOOL,
    pub wki502_use_write_raw_data: windows_sys::core::BOOL,
    pub wki502_use_encryption: windows_sys::core::BOOL,
    pub wki502_buf_files_deny_write: windows_sys::core::BOOL,
    pub wki502_buf_read_only_files: windows_sys::core::BOOL,
    pub wki502_force_core_create_mode: windows_sys::core::BOOL,
    pub wki502_use_512_byte_max_transfer: windows_sys::core::BOOL,
}
pub const WKSTA_KEEPCONN_PARMNUM: u32 = 13;
pub const WKSTA_KEEPSEARCH_PARMNUM: u32 = 14;
pub const WKSTA_LANGROUP_PARMNUM: u32 = 2;
pub const WKSTA_LANROOT_PARMNUM: u32 = 7;
pub const WKSTA_LOCKINCREMENT_PARMNUM: u32 = 42;
pub const WKSTA_LOCKMAXIMUM_PARMNUM: u32 = 43;
pub const WKSTA_LOCKQUOTA_PARMNUM: u32 = 41;
pub const WKSTA_LOGGED_ON_USERS_PARMNUM: u32 = 6;
pub const WKSTA_LOGON_DOMAIN_PARMNUM: u32 = 8;
pub const WKSTA_LOGON_SERVER_PARMNUM: u32 = 9;
pub const WKSTA_MAILSLOTS_PARMNUM: u32 = 30;
pub const WKSTA_MAXCMDS_PARMNUM: u32 = 15;
pub const WKSTA_MAXTHREADS_PARMNUM: u32 = 33;
pub const WKSTA_MAXWRKCACHE_PARMNUM: u32 = 17;
pub const WKSTA_NUMALERTS_PARMNUM: u32 = 20;
pub const WKSTA_NUMCHARBUF_PARMNUM: u32 = 22;
pub const WKSTA_NUMDGRAMBUF_PARMNUM: u32 = 31;
pub const WKSTA_NUMSERVICES_PARMNUM: u32 = 21;
pub const WKSTA_NUMWORKBUF_PARMNUM: u32 = 16;
pub const WKSTA_OTH_DOMAINS_PARMNUM: u32 = 101;
pub const WKSTA_PIPEINCREMENT_PARMNUM: u32 = 44;
pub const WKSTA_PIPEMAXIMUM_PARMNUM: u32 = 45;
pub const WKSTA_PLATFORM_ID_PARMNUM: u32 = 100;
pub const WKSTA_PRINTBUFTIME_PARMNUM: u32 = 28;
pub const WKSTA_READAHEADTHRUPUT_PARMNUM: u32 = 62;
pub const WKSTA_SESSTIMEOUT_PARMNUM: u32 = 18;
pub const WKSTA_SIZCHARBUF_PARMNUM: u32 = 23;
pub const WKSTA_SIZERROR_PARMNUM: u32 = 19;
pub const WKSTA_SIZWORKBUF_PARMNUM: u32 = 29;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WKSTA_TRANSPORT_INFO_0 {
    pub wkti0_quality_of_service: u32,
    pub wkti0_number_of_vcs: u32,
    pub wkti0_transport_name: windows_sys::core::PWSTR,
    pub wkti0_transport_address: windows_sys::core::PWSTR,
    pub wkti0_wan_ish: windows_sys::core::BOOL,
}
impl Default for WKSTA_TRANSPORT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WKSTA_USE512BYTESMAXTRANSFER_PARMNUM: u32 = 61;
pub const WKSTA_USECLOSEBEHIND_PARMNUM: u32 = 50;
pub const WKSTA_USEENCRYPTION_PARMNUM: u32 = 57;
pub const WKSTA_USELOCKANDREADANDUNLOCK_PARMNUM: u32 = 52;
pub const WKSTA_USEOPPORTUNISTICLOCKING_PARMNUM: u32 = 48;
pub const WKSTA_USERAWREAD_PARMNUM: u32 = 54;
pub const WKSTA_USERAWWRITE_PARMNUM: u32 = 55;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WKSTA_USER_INFO_0 {
    pub wkui0_username: windows_sys::core::PWSTR,
}
impl Default for WKSTA_USER_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WKSTA_USER_INFO_1 {
    pub wkui1_username: windows_sys::core::PWSTR,
    pub wkui1_logon_domain: windows_sys::core::PWSTR,
    pub wkui1_oth_domains: windows_sys::core::PWSTR,
    pub wkui1_logon_server: windows_sys::core::PWSTR,
}
impl Default for WKSTA_USER_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WKSTA_USER_INFO_1101 {
    pub wkui1101_oth_domains: windows_sys::core::PWSTR,
}
impl Default for WKSTA_USER_INFO_1101 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WKSTA_USEUNLOCKBEHIND_PARMNUM: u32 = 49;
pub const WKSTA_USEWRITERAWWITHDATA_PARMNUM: u32 = 56;
pub const WKSTA_UTILIZENTCACHING_PARMNUM: u32 = 53;
pub const WKSTA_VER_MAJOR_PARMNUM: u32 = 4;
pub const WKSTA_VER_MINOR_PARMNUM: u32 = 5;
pub const WKSTA_WRKHEURISTICS_PARMNUM: u32 = 32;
