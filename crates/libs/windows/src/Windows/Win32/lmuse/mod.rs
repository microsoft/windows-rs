#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn NetUseAdd(servername: Option<super::winnt::LPTSTR>, levelflags: u32, buf: *mut u8, parm_err: Option<*mut u32>) -> u32 {
    windows_core::link!("netapi32.dll" "system" fn NetUseAdd(servername : super::winnt::LPTSTR, levelflags : u32, buf : *mut u8, parm_err : *mut u32) -> u32);
    unsafe { NetUseAdd(servername.unwrap_or(core::mem::zeroed()) as _, levelflags, buf as _, parm_err.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetUseDel<P0, P1>(uncservername: P0, usename: P1, forcelevelflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUseDel(uncservername : windows_core::PCWSTR, usename : windows_core::PCWSTR, forcelevelflags : u32) -> u32);
    unsafe { NetUseDel(uncservername.param().abi(), usename.param().abi(), forcelevelflags) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn NetUseEnum<P0>(uncservername: P0, levelflags: u32, bufptr: *mut super::minwindef::LPBYTE, preferedmaximumsize: u32, entriesread: Option<*mut u32>, totalentries: *mut u32, resumehandle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUseEnum(uncservername : windows_core::PCWSTR, levelflags : u32, bufptr : *mut super::minwindef::LPBYTE, preferedmaximumsize : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
    unsafe { NetUseEnum(uncservername.param().abi(), levelflags, bufptr as _, preferedmaximumsize, entriesread.unwrap_or(core::mem::zeroed()) as _, totalentries as _, resumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn NetUseGetInfo<P0, P1>(uncservername: P0, usename: P1, levelflags: u32, bufptr: *mut super::minwindef::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUseGetInfo(uncservername : windows_core::PCWSTR, usename : windows_core::PCWSTR, levelflags : u32, bufptr : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { NetUseGetInfo(uncservername.param().abi(), usename.param().abi(), levelflags, bufptr as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPUSE_INFO_0(pub *mut USE_INFO_0);
impl LPUSE_INFO_0 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPUSE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPUSE_INFO_1(pub *mut USE_INFO_1);
impl LPUSE_INFO_1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPUSE_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPUSE_INFO_2(pub *mut USE_INFO_2);
impl LPUSE_INFO_2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPUSE_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPUSE_INFO_3(pub *mut USE_INFO_3);
impl LPUSE_INFO_3 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPUSE_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPUSE_INFO_4(pub *mut USE_INFO_4);
#[cfg(feature = "Win32_minwindef")]
impl LPUSE_INFO_4 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for LPUSE_INFO_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPUSE_INFO_5(pub *mut USE_INFO_5);
#[cfg(feature = "Win32_minwindef")]
impl LPUSE_INFO_5 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for LPUSE_INFO_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NoneFlag: TRANSPORT_INFO_FLAG = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBLOCK_NTLM_INFO(pub *mut BLOCK_NTLM_INFO);
impl PBLOCK_NTLM_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBLOCK_NTLM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSMB_COMPRESSION_INFO(pub *mut SMB_COMPRESSION_INFO);
impl PSMB_COMPRESSION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSMB_COMPRESSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSMB_TREE_CONNECT_PARAMETERS(pub *mut SMB_TREE_CONNECT_PARAMETERS);
impl PSMB_TREE_CONNECT_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSMB_TREE_CONNECT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSMB_USE_OPTION_COMPRESSION_PARAMETERS(pub *mut SMB_USE_OPTION_COMPRESSION_PARAMETERS);
impl PSMB_USE_OPTION_COMPRESSION_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSMB_USE_OPTION_COMPRESSION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSPORT_INFO(pub *mut TRANSPORT_INFO);
impl PTRANSPORT_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSPORT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSPORT_INFO_FLAG(pub *mut TRANSPORT_INFO_FLAG);
impl PTRANSPORT_INFO_FLAG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSPORT_INFO_FLAG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSPORT_TYPE(pub *mut TRANSPORT_TYPE);
impl PTRANSPORT_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSPORT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSE_INFO_0(pub *mut USE_INFO_0);
impl PUSE_INFO_0 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSE_INFO_1(pub *mut USE_INFO_1);
impl PUSE_INFO_1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSE_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSE_INFO_2(pub *mut USE_INFO_2);
impl PUSE_INFO_2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSE_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSE_INFO_3(pub *mut USE_INFO_3);
impl PUSE_INFO_3 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSE_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSE_INFO_4(pub *mut USE_INFO_4);
#[cfg(feature = "Win32_minwindef")]
impl PUSE_INFO_4 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PUSE_INFO_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSE_INFO_5(pub *mut USE_INFO_5);
#[cfg(feature = "Win32_minwindef")]
impl PUSE_INFO_5 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PUSE_INFO_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSE_OPTION_BLOCK_NTLM_PARAMETERS(pub *mut USE_OPTION_BLOCK_NTLM_PARAMETERS);
impl PUSE_OPTION_BLOCK_NTLM_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSE_OPTION_BLOCK_NTLM_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSE_OPTION_DEFERRED_CONNECTION_PARAMETERS(pub *mut USE_OPTION_DEFERRED_CONNECTION_PARAMETERS);
impl PUSE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSE_OPTION_GENERIC(pub *mut USE_OPTION_GENERIC);
impl PUSE_OPTION_GENERIC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSE_OPTION_GENERIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSE_OPTION_PROPERTIES(pub *mut USE_OPTION_PROPERTIES);
impl PUSE_OPTION_PROPERTIES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSE_OPTION_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSE_OPTION_TRANSPORT_PARAMETERS(pub *mut USE_OPTION_TRANSPORT_PARAMETERS);
impl PUSE_OPTION_TRANSPORT_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSE_OPTION_TRANSPORT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const QuicPortSetFlag: TRANSPORT_INFO_FLAG = 2;
pub const RdmaPortSetFlag: TRANSPORT_INFO_FLAG = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SMB_COMPRESSION_INFO {
    pub Switch: bool,
    pub Reserved1: u8,
    pub Reserved2: u16,
    pub Reserved3: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SMB_TREE_CONNECT_PARAMETERS {
    pub EABufferOffset: u32,
    pub EABufferLen: u32,
    pub CreateOptions: u32,
    pub TreeConnectAttributes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USE_INFO_0 {
    pub ui0_local: windows_core::PWSTR,
    pub ui0_remote: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USE_INFO_1 {
    pub ui1_local: windows_core::PWSTR,
    pub ui1_remote: windows_core::PWSTR,
    pub ui1_password: windows_core::PWSTR,
    pub ui1_status: u32,
    pub ui1_asg_type: u32,
    pub ui1_refcount: u32,
    pub ui1_usecount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USE_INFO_2 {
    pub ui2_local: windows_core::PWSTR,
    pub ui2_remote: windows_core::PWSTR,
    pub ui2_password: windows_core::PWSTR,
    pub ui2_status: u32,
    pub ui2_asg_type: u32,
    pub ui2_refcount: u32,
    pub ui2_usecount: u32,
    pub ui2_username: windows_core::PWSTR,
    pub ui2_domainname: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USE_INFO_3 {
    pub ui3_ui2: USE_INFO_2,
    pub ui3_flags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USE_INFO_4 {
    pub ui4_ui3: USE_INFO_3,
    pub ui4_auth_identity_length: u32,
    pub ui4_auth_identity: super::minwindef::PBYTE,
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USE_INFO_5 {
    pub ui4_ui3: USE_INFO_3,
    pub ui4_auth_identity_length: u32,
    pub ui4_auth_identity: super::minwindef::PBYTE,
    pub ui5_security_descriptor_length: u32,
    pub ui5_security_descriptor: super::minwindef::PBYTE,
    pub ui5_use_options_length: u32,
    pub ui5_use_options: super::minwindef::PBYTE,
}
pub const USE_IPC: u32 = 3;
pub const USE_LOCAL_PARMNUM: u32 = 1;
pub const USE_NETERR: u32 = 3;
pub const USE_OK: u32 = 0;
pub const USE_OPTIONS_PARMNUM: u32 = 10;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USE_OPTION_BLOCK_NTLM_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
pub const USE_OPTION_BLOCK_NTLM_PARAMS: u32 = 1315925058;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
pub const USE_OPTION_DEFERRED_CONNECTION_PARAMS: u32 = 1130784068;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USE_OPTION_GENERIC {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
