#[cfg(feature = "Win32_winnt")]
windows_link::link!("netapi32.dll" "system" fn NetUseAdd(servername : super::winnt::LPTSTR, levelflags : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetUseDel(uncservername : windows_sys::core::PCWSTR, usename : windows_sys::core::PCWSTR, forcelevelflags : u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetUseEnum(uncservername : windows_sys::core::PCWSTR, levelflags : u32, bufptr : *mut super::minwindef::LPBYTE, preferedmaximumsize : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetUseGetInfo(uncservername : windows_sys::core::PCWSTR, usename : windows_sys::core::PCWSTR, levelflags : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BLOCK_NTLM_INFO {
    pub BlockNTLM: bool,
    pub Reserved1: u8,
    pub Reserved2: u16,
    pub Reserved3: u32,
}
pub const CREATE_BYPASS_CSC: u32 = 2;
pub const CREATE_CRED_RESET: u32 = 4;
pub const CREATE_GLOBAL_MAPPING: u32 = 256;
pub const CREATE_NO_CONNECT: u32 = 1;
pub const CREATE_PERSIST_MAPPING: u32 = 32;
pub const CREATE_REQUIRE_CONNECTION_INTEGRITY: u32 = 8;
pub const CREATE_REQUIRE_CONNECTION_PRIVACY: u32 = 16;
pub const CREATE_WRITE_THROUGH_SEMANTICS: u32 = 64;
pub type LPUSE_INFO_0 = *mut USE_INFO_0;
pub type LPUSE_INFO_1 = *mut USE_INFO_1;
pub type LPUSE_INFO_2 = *mut USE_INFO_2;
pub type LPUSE_INFO_3 = *mut USE_INFO_3;
#[cfg(feature = "Win32_minwindef")]
pub type LPUSE_INFO_4 = *mut USE_INFO_4;
#[cfg(feature = "Win32_minwindef")]
pub type LPUSE_INFO_5 = *mut USE_INFO_5;
pub const NoneFlag: TRANSPORT_INFO_FLAG = 0;
pub type PBLOCK_NTLM_INFO = *mut BLOCK_NTLM_INFO;
pub type PSMB_COMPRESSION_INFO = *mut SMB_COMPRESSION_INFO;
pub type PSMB_TREE_CONNECT_PARAMETERS = *mut SMB_TREE_CONNECT_PARAMETERS;
pub type PSMB_USE_OPTION_COMPRESSION_PARAMETERS = *mut SMB_USE_OPTION_COMPRESSION_PARAMETERS;
pub type PTRANSPORT_INFO = *mut TRANSPORT_INFO;
pub type PTRANSPORT_INFO_FLAG = *mut TRANSPORT_INFO_FLAG;
pub type PTRANSPORT_TYPE = *mut TRANSPORT_TYPE;
pub type PUSE_INFO_0 = *mut USE_INFO_0;
pub type PUSE_INFO_1 = *mut USE_INFO_1;
pub type PUSE_INFO_2 = *mut USE_INFO_2;
pub type PUSE_INFO_3 = *mut USE_INFO_3;
#[cfg(feature = "Win32_minwindef")]
pub type PUSE_INFO_4 = *mut USE_INFO_4;
#[cfg(feature = "Win32_minwindef")]
pub type PUSE_INFO_5 = *mut USE_INFO_5;
pub type PUSE_OPTION_BLOCK_NTLM_PARAMETERS = *mut USE_OPTION_BLOCK_NTLM_PARAMETERS;
pub type PUSE_OPTION_DEFERRED_CONNECTION_PARAMETERS = *mut USE_OPTION_DEFERRED_CONNECTION_PARAMETERS;
pub type PUSE_OPTION_GENERIC = *mut USE_OPTION_GENERIC;
pub type PUSE_OPTION_PROPERTIES = *mut USE_OPTION_PROPERTIES;
pub type PUSE_OPTION_TRANSPORT_PARAMETERS = *mut USE_OPTION_TRANSPORT_PARAMETERS;
pub const QuicPortSetFlag: TRANSPORT_INFO_FLAG = 2;
pub const RdmaPortSetFlag: TRANSPORT_INFO_FLAG = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SMB_COMPRESSION_INFO {
    pub Switch: bool,
    pub Reserved1: u8,
    pub Reserved2: u16,
    pub Reserved3: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SMB_TREE_CONNECT_PARAMETERS {
    pub EABufferOffset: u32,
    pub EABufferLen: u32,
    pub CreateOptions: u32,
    pub TreeConnectAttributes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRANSPORT_INFO {
    pub Type: TRANSPORT_TYPE,
    pub SkipCertificateCheck: bool,
    pub TcpPort: u16,
    pub QuicPort: u16,
    pub RdmaPort: u16,
    pub Flags: u32,
}
pub type TRANSPORT_INFO_FLAG = i32;
pub type TRANSPORT_TYPE = i32;
pub const TcpPortSetFlag: TRANSPORT_INFO_FLAG = 1;
pub const USE_ASGTYPE_PARMNUM: u32 = 4;
pub const USE_AUTHIDENTITY_PARMNUM: u32 = 8;
pub const USE_CHARDEV: u32 = 2;
pub const USE_CONN: u32 = 4;
pub const USE_DEFAULT_CREDENTIALS: u32 = 4;
pub const USE_DISCONN: u32 = 2;
pub const USE_DISKDEV: u32 = 0;
pub const USE_DOMAINNAME_PARMNUM: u32 = 6;
pub const USE_FLAGS_PARMNUM: u32 = 7;
pub const USE_FLAG_GLOBAL_MAPPING: u32 = 65536;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct USE_INFO_0 {
    pub ui0_local: windows_sys::core::PWSTR,
    pub ui0_remote: windows_sys::core::PWSTR,
}
impl Default for USE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct USE_INFO_1 {
    pub ui1_local: windows_sys::core::PWSTR,
    pub ui1_remote: windows_sys::core::PWSTR,
    pub ui1_password: windows_sys::core::PWSTR,
    pub ui1_status: u32,
    pub ui1_asg_type: u32,
    pub ui1_refcount: u32,
    pub ui1_usecount: u32,
}
impl Default for USE_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct USE_INFO_2 {
    pub ui2_local: windows_sys::core::PWSTR,
    pub ui2_remote: windows_sys::core::PWSTR,
    pub ui2_password: windows_sys::core::PWSTR,
    pub ui2_status: u32,
    pub ui2_asg_type: u32,
    pub ui2_refcount: u32,
    pub ui2_usecount: u32,
    pub ui2_username: windows_sys::core::PWSTR,
    pub ui2_domainname: windows_sys::core::PWSTR,
}
impl Default for USE_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USE_INFO_3 {
    pub ui3_ui2: USE_INFO_2,
    pub ui3_flags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct USE_INFO_4 {
    pub ui4_ui3: USE_INFO_3,
    pub ui4_auth_identity_length: u32,
    pub ui4_auth_identity: super::minwindef::PBYTE,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for USE_INFO_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct USE_INFO_5 {
    pub ui4_ui3: USE_INFO_3,
    pub ui4_auth_identity_length: u32,
    pub ui4_auth_identity: super::minwindef::PBYTE,
    pub ui5_security_descriptor_length: u32,
    pub ui5_security_descriptor: super::minwindef::PBYTE,
    pub ui5_use_options_length: u32,
    pub ui5_use_options: super::minwindef::PBYTE,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for USE_INFO_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USE_IPC: u32 = 3;
pub const USE_LOCAL_PARMNUM: u32 = 1;
pub const USE_NETERR: u32 = 3;
pub const USE_OK: u32 = 0;
pub const USE_OPTIONS_PARMNUM: u32 = 10;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USE_OPTION_BLOCK_NTLM_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
pub const USE_OPTION_BLOCK_NTLM_PARAMS: u32 = 1315925058;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
pub const USE_OPTION_DEFERRED_CONNECTION_PARAMS: u32 = 1130784068;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USE_OPTION_GENERIC {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct USE_OPTION_PROPERTIES {
    pub Tag: u32,
    pub pInfo: *mut core::ffi::c_void,
    pub Length: usize,
}
impl Default for USE_OPTION_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USE_OPTION_SMB_COMPRESSION_PARAMS: u32 = 1349349187;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USE_OPTION_TRANSPORT_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
pub const USE_OPTION_TRANSPORT_PARAMS: u32 = 1348563540;
pub const USE_PASSWORD_PARMNUM: u32 = 3;
pub const USE_PAUSED: u32 = 1;
pub const USE_RECONN: u32 = 5;
pub const USE_REMOTE_PARMNUM: u32 = 2;
pub const USE_SD_PARMNUM: u32 = 9;
pub const USE_SESSLOST: u32 = 2;
pub const USE_SPOOLDEV: u32 = 1;
pub const USE_USERNAME_PARMNUM: u32 = 5;
pub const USE_WILDCARD: i32 = -1;
pub const UseTransportType_None: TRANSPORT_TYPE = 0;
pub const UseTransportType_Quic: TRANSPORT_TYPE = 2;
pub const UseTransportType_Wsk: TRANSPORT_TYPE = 1;
