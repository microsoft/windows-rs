#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetConnectionEnum(servername : windows_sys::core::PCWSTR, qualifier : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetFileClose(servername : windows_sys::core::PCWSTR, fileid : u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetFileEnum(servername : windows_sys::core::PCWSTR, basepath : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut usize) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetFileGetInfo(servername : windows_sys::core::PCWSTR, fileid : u32, level : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetServerAliasAdd(servername : windows_sys::core::PCWSTR, level : u32, buf : *const u8) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetServerAliasDel(servername : windows_sys::core::PCWSTR, level : u32, buf : *mut u8) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetServerAliasEnum(servername : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetSessionDel(servername : windows_sys::core::PCWSTR, uncclientname : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetSessionEnum(servername : windows_sys::core::PCWSTR, uncclientname : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetSessionGetInfo(servername : windows_sys::core::PCWSTR, uncclientname : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetShareAdd(servername : windows_sys::core::PCWSTR, level : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetShareCheck(servername : windows_sys::core::PCWSTR, device : windows_sys::core::PCWSTR, r#type : *mut u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetShareDel(servername : windows_sys::core::PCWSTR, netname : windows_sys::core::PCWSTR, reserved : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetShareDelEx(servername : windows_sys::core::PCWSTR, level : u32, buf : *mut u8) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetShareDelSticky(servername : windows_sys::core::PCWSTR, netname : windows_sys::core::PCWSTR, reserved : u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetShareEnum(servername : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetShareEnumSticky(servername : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetShareGetInfo(servername : windows_sys::core::PCWSTR, netname : windows_sys::core::PCWSTR, level : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetShareSetInfo(servername : windows_sys::core::PCWSTR, netname : windows_sys::core::PCWSTR, level : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONNECTION_INFO_0 {
    pub coni0_id: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONNECTION_INFO_1 {
    pub coni1_id: u32,
    pub coni1_type: u32,
    pub coni1_num_opens: u32,
    pub coni1_num_users: u32,
    pub coni1_time: u32,
    pub coni1_username: windows_sys::core::PWSTR,
    pub coni1_netname: windows_sys::core::PWSTR,
}
impl Default for CONNECTION_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CSC_CACHE_AUTO_REINT: u32 = 16;
pub const CSC_CACHE_MANUAL_REINT: u32 = 0;
pub const CSC_CACHE_NONE: u32 = 48;
pub const CSC_CACHE_VDO: u32 = 32;
pub const CSC_MASK: u32 = 48;
pub const CSC_MASK_EXT: u32 = 8240;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_INFO_2 {
    pub fi2_id: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_INFO_3 {
    pub fi3_id: u32,
    pub fi3_permissions: u32,
    pub fi3_num_locks: u32,
    pub fi3_pathname: windows_sys::core::PWSTR,
    pub fi3_username: windows_sys::core::PWSTR,
}
impl Default for FILE_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPCONNECTION_INFO_0 = *mut CONNECTION_INFO_0;
pub type LPCONNECTION_INFO_1 = *mut CONNECTION_INFO_1;
pub type LPFILE_INFO_2 = *mut FILE_INFO_2;
pub type LPFILE_INFO_3 = *mut FILE_INFO_3;
pub type LPSERVER_ALIAS_INFO_0 = *mut SERVER_ALIAS_INFO_0;
pub type LPSESSION_INFO_0 = *mut SESSION_INFO_0;
pub type LPSESSION_INFO_1 = *mut SESSION_INFO_1;
pub type LPSESSION_INFO_10 = *mut SESSION_INFO_10;
pub type LPSESSION_INFO_2 = *mut SESSION_INFO_2;
pub type LPSESSION_INFO_502 = *mut SESSION_INFO_502;
pub type LPSHARE_INFO_0 = *mut SHARE_INFO_0;
pub type LPSHARE_INFO_1 = *mut SHARE_INFO_1;
pub type LPSHARE_INFO_1004 = *mut SHARE_INFO_1004;
pub type LPSHARE_INFO_1005 = *mut SHARE_INFO_1005;
pub type LPSHARE_INFO_1006 = *mut SHARE_INFO_1006;
#[cfg(feature = "Win32_winnt")]
pub type LPSHARE_INFO_1501 = *mut SHARE_INFO_1501;
pub type LPSHARE_INFO_1503 = *mut SHARE_INFO_1503;
pub type LPSHARE_INFO_2 = *mut SHARE_INFO_2;
pub type LPSHARE_INFO_501 = *mut SHARE_INFO_501;
#[cfg(feature = "Win32_winnt")]
pub type LPSHARE_INFO_502 = *mut SHARE_INFO_502;
#[cfg(feature = "Win32_winnt")]
pub type LPSHARE_INFO_503 = *mut SHARE_INFO_503;
pub type PCONNECTION_INFO_0 = *mut CONNECTION_INFO_0;
pub type PCONNECTION_INFO_1 = *mut CONNECTION_INFO_1;
pub const PERM_FILE_CREATE: u32 = 4;
pub const PERM_FILE_READ: u32 = 1;
pub const PERM_FILE_WRITE: u32 = 2;
pub type PFILE_INFO_2 = *mut FILE_INFO_2;
pub type PFILE_INFO_3 = *mut FILE_INFO_3;
pub type PSERVER_ALIAS_INFO_0 = *mut SERVER_ALIAS_INFO_0;
pub type PSESSION_INFO_0 = *mut SESSION_INFO_0;
pub type PSESSION_INFO_1 = *mut SESSION_INFO_1;
pub type PSESSION_INFO_10 = *mut SESSION_INFO_10;
pub type PSESSION_INFO_2 = *mut SESSION_INFO_2;
pub type PSESSION_INFO_502 = *mut SESSION_INFO_502;
pub type PSHARE_INFO_0 = *mut SHARE_INFO_0;
pub type PSHARE_INFO_1 = *mut SHARE_INFO_1;
pub type PSHARE_INFO_1004 = *mut SHARE_INFO_1004;
pub type PSHARE_INFO_1005 = *mut SHARE_INFO_1005;
pub type PSHARE_INFO_1006 = *mut SHARE_INFO_1006;
#[cfg(feature = "Win32_winnt")]
pub type PSHARE_INFO_1501 = *mut SHARE_INFO_1501;
pub type PSHARE_INFO_1503 = *mut SHARE_INFO_1503;
pub type PSHARE_INFO_2 = *mut SHARE_INFO_2;
pub type PSHARE_INFO_501 = *mut SHARE_INFO_501;
#[cfg(feature = "Win32_winnt")]
pub type PSHARE_INFO_502 = *mut SHARE_INFO_502;
#[cfg(feature = "Win32_winnt")]
pub type PSHARE_INFO_503 = *mut SHARE_INFO_503;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVER_ALIAS_INFO_0 {
    pub srvai0_alias: windows_sys::core::PWSTR,
    pub srvai0_target: windows_sys::core::PWSTR,
    pub srvai0_default: bool,
    pub srvai0_reserved: u32,
}
impl Default for SERVER_ALIAS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SESI1_NUM_ELEMENTS: u32 = 8;
pub const SESI2_NUM_ELEMENTS: u32 = 9;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SESSION_INFO_0 {
    pub sesi0_cname: windows_sys::core::PWSTR,
}
impl Default for SESSION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SESSION_INFO_1 {
    pub sesi1_cname: windows_sys::core::PWSTR,
    pub sesi1_username: windows_sys::core::PWSTR,
    pub sesi1_num_opens: u32,
    pub sesi1_time: u32,
    pub sesi1_idle_time: u32,
    pub sesi1_user_flags: u32,
}
impl Default for SESSION_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SESSION_INFO_10 {
    pub sesi10_cname: windows_sys::core::PWSTR,
    pub sesi10_username: windows_sys::core::PWSTR,
    pub sesi10_time: u32,
    pub sesi10_idle_time: u32,
}
impl Default for SESSION_INFO_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SESSION_INFO_2 {
    pub sesi2_cname: windows_sys::core::PWSTR,
    pub sesi2_username: windows_sys::core::PWSTR,
    pub sesi2_num_opens: u32,
    pub sesi2_time: u32,
    pub sesi2_idle_time: u32,
    pub sesi2_user_flags: u32,
    pub sesi2_cltype_name: windows_sys::core::PWSTR,
}
impl Default for SESSION_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SESSION_INFO_502 {
    pub sesi502_cname: windows_sys::core::PWSTR,
    pub sesi502_username: windows_sys::core::PWSTR,
    pub sesi502_num_opens: u32,
    pub sesi502_time: u32,
    pub sesi502_idle_time: u32,
    pub sesi502_user_flags: u32,
    pub sesi502_cltype_name: windows_sys::core::PWSTR,
    pub sesi502_transport: windows_sys::core::PWSTR,
}
impl Default for SESSION_INFO_502 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SESS_GUEST: u32 = 1;
pub const SESS_NOENCRYPTION: u32 = 2;
pub const SHARE_CURRENT_USES_PARMNUM: u32 = 7;
pub const SHARE_FILE_SD_INFOLEVEL: u32 = 1501;
pub const SHARE_FILE_SD_PARMNUM: u32 = 501;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHARE_INFO_0 {
    pub shi0_netname: windows_sys::core::PWSTR,
}
impl Default for SHARE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHARE_INFO_1 {
    pub shi1_netname: windows_sys::core::PWSTR,
    pub shi1_type: u32,
    pub shi1_remark: windows_sys::core::PWSTR,
}
impl Default for SHARE_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHARE_INFO_1004 {
    pub shi1004_remark: windows_sys::core::PWSTR,
}
impl Default for SHARE_INFO_1004 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SHARE_INFO_1005 {
    pub shi1005_flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SHARE_INFO_1006 {
    pub shi1006_max_uses: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct SHARE_INFO_1501 {
    pub shi1501_reserved: u32,
    pub shi1501_security_descriptor: super::winnt::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for SHARE_INFO_1501 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SHARE_INFO_1503 {
    pub shi1503_sharefilter: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHARE_INFO_2 {
    pub shi2_netname: windows_sys::core::PWSTR,
    pub shi2_type: u32,
    pub shi2_remark: windows_sys::core::PWSTR,
    pub shi2_permissions: u32,
    pub shi2_max_uses: u32,
    pub shi2_current_uses: u32,
    pub shi2_path: windows_sys::core::PWSTR,
    pub shi2_passwd: windows_sys::core::PWSTR,
}
impl Default for SHARE_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHARE_INFO_501 {
    pub shi501_netname: windows_sys::core::PWSTR,
    pub shi501_type: u32,
    pub shi501_remark: windows_sys::core::PWSTR,
    pub shi501_flags: u32,
}
impl Default for SHARE_INFO_501 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct SHARE_INFO_502 {
    pub shi502_netname: windows_sys::core::PWSTR,
    pub shi502_type: u32,
    pub shi502_remark: windows_sys::core::PWSTR,
    pub shi502_permissions: u32,
    pub shi502_max_uses: u32,
    pub shi502_current_uses: u32,
    pub shi502_path: windows_sys::core::PWSTR,
    pub shi502_passwd: windows_sys::core::PWSTR,
    pub shi502_reserved: u32,
    pub shi502_security_descriptor: super::winnt::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for SHARE_INFO_502 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct SHARE_INFO_503 {
    pub shi503_netname: windows_sys::core::PWSTR,
    pub shi503_type: u32,
    pub shi503_remark: windows_sys::core::PWSTR,
    pub shi503_permissions: u32,
    pub shi503_max_uses: u32,
    pub shi503_current_uses: u32,
    pub shi503_path: windows_sys::core::PWSTR,
    pub shi503_passwd: windows_sys::core::PWSTR,
    pub shi503_servername: windows_sys::core::PWSTR,
    pub shi503_reserved: u32,
    pub shi503_security_descriptor: super::winnt::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for SHARE_INFO_503 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SHARE_MAX_USES_INFOLEVEL: u32 = 1006;
pub const SHARE_MAX_USES_PARMNUM: u32 = 6;
pub const SHARE_NETNAME_PARMNUM: u32 = 1;
pub const SHARE_PASSWD_PARMNUM: u32 = 9;
pub const SHARE_PATH_PARMNUM: u32 = 8;
pub const SHARE_PERMISSIONS_PARMNUM: u32 = 5;
pub const SHARE_QOS_POLICY_PARMNUM: u32 = 504;
pub const SHARE_REMARK_INFOLEVEL: u32 = 1004;
pub const SHARE_REMARK_PARMNUM: u32 = 4;
pub const SHARE_SERVER_PARMNUM: u32 = 503;
pub const SHARE_TYPE_PARMNUM: u32 = 3;
pub const SHI1005_FLAGS_ACCESS_BASED_DIRECTORY_ENUM: u32 = 2048;
pub const SHI1005_FLAGS_ALLOW_NAMESPACE_CACHING: u32 = 1024;
pub const SHI1005_FLAGS_CLUSTER_MANAGED: u32 = 524288;
pub const SHI1005_FLAGS_COMPRESS_DATA: u32 = 1048576;
pub const SHI1005_FLAGS_DFS: u32 = 1;
pub const SHI1005_FLAGS_DFS_ROOT: u32 = 2;
pub const SHI1005_FLAGS_DISABLE_CLIENT_BUFFERING: u32 = 131072;
pub const SHI1005_FLAGS_DISABLE_CLIENT_METADATA_CACHING: u32 = 8388608;
pub const SHI1005_FLAGS_DISABLE_DIRECTORY_HANDLE_LEASING: u32 = 4194304;
pub const SHI1005_FLAGS_ENABLE_CA: u32 = 16384;
pub const SHI1005_FLAGS_ENABLE_HASH: u32 = 8192;
pub const SHI1005_FLAGS_ENCRYPT_DATA: u32 = 32768;
pub const SHI1005_FLAGS_FORCE_LEVELII_OPLOCK: u32 = 4096;
pub const SHI1005_FLAGS_FORCE_SHARED_DELETE: u32 = 512;
pub const SHI1005_FLAGS_IDENTITY_REMOTING: u32 = 262144;
pub const SHI1005_FLAGS_ISOLATED_TRANSPORT: u32 = 2097152;
pub const SHI1005_FLAGS_RESERVED: u32 = 65536;
pub const SHI1005_FLAGS_RESTRICT_EXCLUSIVE_OPENS: u32 = 256;
pub const SHI1005_VALID_FLAGS_SET: u32 = 16777008;
pub const SHI1_NUM_ELEMENTS: u32 = 4;
pub const SHI2_NUM_ELEMENTS: u32 = 10;
pub const SHI_USES_UNLIMITED: u32 = 4294967295;
pub const STYPE_DEVICE: u32 = 2;
pub const STYPE_DISKTREE: u32 = 0;
pub const STYPE_IPC: u32 = 3;
pub const STYPE_MASK: u32 = 255;
pub const STYPE_PRINTQ: u32 = 1;
pub const STYPE_RESERVED1: u32 = 16777216;
pub const STYPE_RESERVED2: u32 = 33554432;
pub const STYPE_RESERVED3: u32 = 67108864;
pub const STYPE_RESERVED4: u32 = 134217728;
pub const STYPE_RESERVED5: u32 = 1048576;
pub const STYPE_RESERVED_ALL: u32 = 1073741568;
pub const STYPE_SPECIAL: u32 = 2147483648;
pub const STYPE_TEMPORARY: u32 = 1073741824;
