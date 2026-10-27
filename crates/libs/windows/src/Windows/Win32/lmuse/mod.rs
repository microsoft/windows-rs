#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NetUseAdd(servername: Option<super::LPTSTR>, levelflags: super::DWORD, buf: super::LPBYTE, parm_err: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("netapi32.dll" "system" fn NetUseAdd(servername : super::LPTSTR, levelflags : super::DWORD, buf : super::LPBYTE, parm_err : super::LPDWORD) -> super::DWORD);
    unsafe { NetUseAdd(servername.unwrap_or(core::mem::zeroed()) as _, levelflags, buf as _, parm_err.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetUseDel<P0, P1>(uncservername: P0, usename: P1, forcelevelflags: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUseDel(uncservername : windows_core::PCWSTR, usename : windows_core::PCWSTR, forcelevelflags : super::DWORD) -> super::DWORD);
    unsafe { NetUseDel(uncservername.param().abi(), usename.param().abi(), forcelevelflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetUseEnum<P0>(uncservername: P0, levelflags: super::DWORD, bufptr: Option<*mut super::LPBYTE>, preferedmaximumsize: super::DWORD, entriesread: Option<super::LPDWORD>, totalentries: super::LPDWORD, resumehandle: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUseEnum(uncservername : windows_core::PCWSTR, levelflags : super::DWORD, bufptr : *mut super::LPBYTE, preferedmaximumsize : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resumehandle : super::LPDWORD) -> super::DWORD);
    unsafe { NetUseEnum(uncservername.param().abi(), levelflags, bufptr.unwrap_or(core::mem::zeroed()) as _, preferedmaximumsize, entriesread.unwrap_or(core::mem::zeroed()) as _, totalentries as _, resumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetUseGetInfo<P0, P1>(uncservername: P0, usename: P1, levelflags: super::DWORD, bufptr: *mut super::LPBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUseGetInfo(uncservername : windows_core::PCWSTR, usename : windows_core::PCWSTR, levelflags : super::DWORD, bufptr : *mut super::LPBYTE) -> super::DWORD);
    unsafe { NetUseGetInfo(uncservername.param().abi(), usename.param().abi(), levelflags, bufptr as _) }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BLOCK_NTLM_INFO {
    pub BlockNTLM: super::BOOLEAN,
    pub Reserved1: super::BYTE,
    pub Reserved2: super::USHORT,
    pub Reserved3: super::ULONG,
}
pub const CREATE_BYPASS_CSC: i32 = 2;
pub const CREATE_CRED_RESET: i32 = 4;
pub const CREATE_GLOBAL_MAPPING: i32 = 256;
pub const CREATE_NO_CONNECT: i32 = 1;
pub const CREATE_PERSIST_MAPPING: i32 = 32;
pub const CREATE_REQUIRE_CONNECTION_INTEGRITY: i32 = 8;
pub const CREATE_REQUIRE_CONNECTION_PRIVACY: i32 = 16;
pub const CREATE_WRITE_THROUGH_SEMANTICS: i32 = 64;
pub type LPUSE_INFO_0 = *mut USE_INFO_0;
#[cfg(feature = "shared")]
pub type LPUSE_INFO_1 = *mut USE_INFO_1;
#[cfg(feature = "shared")]
pub type LPUSE_INFO_2 = *mut USE_INFO_2;
#[cfg(feature = "shared")]
pub type LPUSE_INFO_3 = *mut USE_INFO_3;
#[cfg(feature = "shared")]
pub type LPUSE_INFO_4 = *mut USE_INFO_4;
#[cfg(feature = "shared")]
pub type LPUSE_INFO_5 = *mut USE_INFO_5;
pub const NoneFlag: TRANSPORT_INFO_FLAG = 0;
#[cfg(feature = "shared")]
pub type PBLOCK_NTLM_INFO = *mut BLOCK_NTLM_INFO;
#[cfg(feature = "shared")]
pub type PSMB_COMPRESSION_INFO = *mut SMB_COMPRESSION_INFO;
#[cfg(feature = "shared")]
pub type PSMB_TREE_CONNECT_PARAMETERS = *mut SMB_TREE_CONNECT_PARAMETERS;
#[cfg(feature = "shared")]
pub type PSMB_USE_OPTION_COMPRESSION_PARAMETERS = *mut SMB_USE_OPTION_COMPRESSION_PARAMETERS;
#[cfg(feature = "shared")]
pub type PTRANSPORT_INFO = *mut TRANSPORT_INFO;
pub type PTRANSPORT_INFO_FLAG = *mut TRANSPORT_INFO_FLAG;
pub type PTRANSPORT_TYPE = *mut TRANSPORT_TYPE;
pub type PUSE_INFO_0 = *mut USE_INFO_0;
#[cfg(feature = "shared")]
pub type PUSE_INFO_1 = *mut USE_INFO_1;
#[cfg(feature = "shared")]
pub type PUSE_INFO_2 = *mut USE_INFO_2;
#[cfg(feature = "shared")]
pub type PUSE_INFO_3 = *mut USE_INFO_3;
#[cfg(feature = "shared")]
pub type PUSE_INFO_4 = *mut USE_INFO_4;
#[cfg(feature = "shared")]
pub type PUSE_INFO_5 = *mut USE_INFO_5;
#[cfg(feature = "shared")]
pub type PUSE_OPTION_BLOCK_NTLM_PARAMETERS = *mut USE_OPTION_BLOCK_NTLM_PARAMETERS;
#[cfg(feature = "shared")]
pub type PUSE_OPTION_DEFERRED_CONNECTION_PARAMETERS = *mut USE_OPTION_DEFERRED_CONNECTION_PARAMETERS;
#[cfg(feature = "shared")]
pub type PUSE_OPTION_GENERIC = *mut USE_OPTION_GENERIC;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSE_OPTION_PROPERTIES = *mut USE_OPTION_PROPERTIES;
#[cfg(feature = "shared")]
pub type PUSE_OPTION_TRANSPORT_PARAMETERS = *mut USE_OPTION_TRANSPORT_PARAMETERS;
pub const QuicPortSetFlag: TRANSPORT_INFO_FLAG = 2;
pub const RdmaPortSetFlag: TRANSPORT_INFO_FLAG = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SMB_COMPRESSION_INFO {
    pub Switch: super::BOOLEAN,
    pub Reserved1: super::BYTE,
    pub Reserved2: super::USHORT,
    pub Reserved3: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SMB_TREE_CONNECT_PARAMETERS {
    pub EABufferOffset: super::ULONG,
    pub EABufferLen: super::DWORD,
    pub CreateOptions: super::ULONG,
    pub TreeConnectAttributes: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    pub Tag: super::ULONG,
    pub Length: super::USHORT,
    pub Reserved: super::USHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSPORT_INFO {
    pub Type: TRANSPORT_TYPE,
    pub SkipCertificateCheck: super::BOOLEAN,
    pub TcpPort: super::USHORT,
    pub QuicPort: super::USHORT,
    pub RdmaPort: super::USHORT,
    pub Flags: super::ULONG,
}
pub type TRANSPORT_INFO_FLAG = i32;
pub type TRANSPORT_TYPE = i32;
pub const TcpPortSetFlag: TRANSPORT_INFO_FLAG = 1;
pub const USE_ASGTYPE_PARMNUM: i32 = 4;
pub const USE_AUTHIDENTITY_PARMNUM: i32 = 8;
pub const USE_CHARDEV: i32 = 2;
pub const USE_CONN: i32 = 4;
pub const USE_DEFAULT_CREDENTIALS: i32 = 4;
pub const USE_DISCONN: i32 = 2;
pub const USE_DISKDEV: i32 = 0;
pub const USE_DOMAINNAME_PARMNUM: i32 = 6;
pub const USE_FLAGS_PARMNUM: i32 = 7;
pub const USE_FLAG_GLOBAL_MAPPING: i32 = 65536;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USE_INFO_0 {
    pub ui0_local: windows_core::PWSTR,
    pub ui0_remote: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USE_INFO_1 {
    pub ui1_local: windows_core::PWSTR,
    pub ui1_remote: windows_core::PWSTR,
    pub ui1_password: windows_core::PWSTR,
    pub ui1_status: super::DWORD,
    pub ui1_asg_type: super::DWORD,
    pub ui1_refcount: super::DWORD,
    pub ui1_usecount: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USE_INFO_2 {
    pub ui2_local: windows_core::PWSTR,
    pub ui2_remote: windows_core::PWSTR,
    pub ui2_password: windows_core::PWSTR,
    pub ui2_status: super::DWORD,
    pub ui2_asg_type: super::DWORD,
    pub ui2_refcount: super::DWORD,
    pub ui2_usecount: super::DWORD,
    pub ui2_username: windows_core::PWSTR,
    pub ui2_domainname: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USE_INFO_3 {
    pub ui3_ui2: USE_INFO_2,
    pub ui3_flags: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USE_INFO_4 {
    pub ui4_ui3: USE_INFO_3,
    pub ui4_auth_identity_length: super::DWORD,
    pub ui4_auth_identity: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USE_INFO_5 {
    pub ui4_ui3: USE_INFO_3,
    pub ui4_auth_identity_length: super::DWORD,
    pub ui4_auth_identity: super::PBYTE,
    pub ui5_security_descriptor_length: super::DWORD,
    pub ui5_security_descriptor: super::PBYTE,
    pub ui5_use_options_length: super::DWORD,
    pub ui5_use_options: super::PBYTE,
}
pub const USE_IPC: i32 = 3;
pub const USE_LOCAL_PARMNUM: i32 = 1;
pub const USE_NETERR: i32 = 3;
pub const USE_OK: i32 = 0;
pub const USE_OPTIONS_PARMNUM: i32 = 10;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USE_OPTION_BLOCK_NTLM_PARAMETERS {
    pub Tag: super::ULONG,
    pub Length: super::USHORT,
    pub Reserved: super::USHORT,
}
pub const USE_OPTION_BLOCK_NTLM_PARAMS: i32 = 1315925058;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    pub Tag: super::ULONG,
    pub Length: super::USHORT,
    pub Reserved: super::USHORT,
}
pub const USE_OPTION_DEFERRED_CONNECTION_PARAMS: i32 = 1130784068;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USE_OPTION_GENERIC {
    pub Tag: super::ULONG,
    pub Length: super::USHORT,
    pub Reserved: super::USHORT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USE_OPTION_PROPERTIES {
    pub Tag: super::ULONG,
    pub pInfo: super::PVOID,
    pub Length: super::size_t,
}
pub const USE_OPTION_SMB_COMPRESSION_PARAMS: i32 = 1349349187;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USE_OPTION_TRANSPORT_PARAMETERS {
    pub Tag: super::ULONG,
    pub Length: super::USHORT,
    pub Reserved: super::USHORT,
}
pub const USE_OPTION_TRANSPORT_PARAMS: i32 = 1348563540;
pub const USE_PASSWORD_PARMNUM: i32 = 3;
pub const USE_PAUSED: i32 = 1;
pub const USE_RECONN: i32 = 5;
pub const USE_REMOTE_PARMNUM: i32 = 2;
pub const USE_SD_PARMNUM: i32 = 9;
pub const USE_SESSLOST: i32 = 2;
pub const USE_SPOOLDEV: i32 = 1;
pub const USE_USERNAME_PARMNUM: i32 = 5;
#[cfg(feature = "shared")]
pub const USE_WILDCARD: super::DWORD = 4294967295;
pub const UseTransportType_None: TRANSPORT_TYPE = 0;
pub const UseTransportType_Quic: TRANSPORT_TYPE = 2;
pub const UseTransportType_Wsk: TRANSPORT_TYPE = 1;
