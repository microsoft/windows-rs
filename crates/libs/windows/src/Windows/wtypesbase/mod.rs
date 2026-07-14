pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048;
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1;
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4;
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32;
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16;
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8;
pub const APPIDREGFLAGS_RESERVED1: u32 = 64;
pub const APPIDREGFLAGS_RESERVED10: u32 = 32768;
pub const APPIDREGFLAGS_RESERVED2: u32 = 128;
pub const APPIDREGFLAGS_RESERVED3: u32 = 256;
pub const APPIDREGFLAGS_RESERVED4: u32 = 512;
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024;
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096;
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192;
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384;
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl Default for BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "rpc")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BYTE_BLOB {
    pub clSize: u32,
    pub abData: [super::rpc::byte; 1],
}
#[cfg(feature = "rpc")]
impl Default for BYTE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "rpc")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BYTE_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut super::rpc::byte,
}
#[cfg(feature = "rpc")]
impl Default for BYTE_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLSCTX = u32;
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = 262144;
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = 524288;
pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = 8388608;
pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = 33554432;
pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = 262144;
pub const CLSCTX_ALLOW_LOWER_TRUST_REGISTRATION: CLSCTX = 67108864;
pub const CLSCTX_APPCONTAINER: CLSCTX = 4194304;
pub const CLSCTX_DISABLE_AAA: CLSCTX = 32768;
pub const CLSCTX_DO_NOT_ELEVATE_SERVER: CLSCTX = 268435456;
pub const CLSCTX_ENABLE_AAA: CLSCTX = 65536;
pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = 1048576;
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = 8192;
pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = 131072;
pub const CLSCTX_INPROC_HANDLER: CLSCTX = 2;
pub const CLSCTX_INPROC_HANDLER16: CLSCTX = 32;
pub const CLSCTX_INPROC_SERVER: CLSCTX = 1;
pub const CLSCTX_INPROC_SERVER16: CLSCTX = 8;
pub const CLSCTX_LOCAL_SERVER: CLSCTX = 4;
pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = 1024;
pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = 4096;
pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = 16384;
pub const CLSCTX_PS_DLL: CLSCTX = 2147483648;
pub const CLSCTX_REMOTE_SERVER: CLSCTX = 16;
pub const CLSCTX_RESERVED1: CLSCTX = 64;
pub const CLSCTX_RESERVED2: CLSCTX = 128;
pub const CLSCTX_RESERVED3: CLSCTX = 256;
pub const CLSCTX_RESERVED4: CLSCTX = 512;
pub const CLSCTX_RESERVED5: CLSCTX = 2048;
pub const CLSCTX_RESERVED6: CLSCTX = 16777216;
pub const CLSCTX_SERVER_MUST_BE_EQUAL_OR_GREATER_PRIVILEGE: CLSCTX = 134217728;
pub const CLSCTX_VALID_MASK: i32 = -1612712929;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COAUTHIDENTITY {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl Default for COAUTHIDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COAUTHINFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pwszServerPrincName: windows_core::PWSTR,
    pub dwAuthnLevel: u32,
    pub dwImpersonationLevel: u32,
    pub pAuthIdentityData: *mut COAUTHIDENTITY,
    pub dwCapabilities: u32,
}
impl Default for COAUTHINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2;
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1;
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32;
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16;
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8;
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWORD_BLOB {
    pub clSize: u32,
    pub alData: [u32; 1],
}
impl Default for DWORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWORD_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u32,
}
impl Default for DWORD_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "rpc")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLAGGED_BYTE_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub abData: [super::rpc::byte; 1],
}
#[cfg(feature = "rpc")]
impl Default for FLAGGED_BYTE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl Default for FLAGGED_WORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HYPER_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut i64,
}
impl Default for HYPER_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPBLOB = *mut BLOB;
pub type MEMCTX = i32;
pub const MEMCTX_MACSYSTEM: MEMCTX = 3;
pub const MEMCTX_SAME: MEMCTX = -2;
pub const MEMCTX_SHARED: MEMCTX = 2;
pub const MEMCTX_TASK: MEMCTX = 1;
pub const MEMCTX_UNKNOWN: MEMCTX = -1;
pub type MSHCTX = i32;
pub const MSHCTX_CONTAINER: MSHCTX = 5;
pub const MSHCTX_CROSSCTX: MSHCTX = 4;
pub const MSHCTX_DIFFERENTMACHINE: MSHCTX = 2;
pub const MSHCTX_INPROC: MSHCTX = 3;
pub const MSHCTX_LOCAL: MSHCTX = 0;
pub const MSHCTX_NOSHAREDMEM: MSHCTX = 1;
pub type MSHLFLAGS = i32;
pub const MSHLFLAGS_NOPING: MSHLFLAGS = 4;
pub const MSHLFLAGS_NORMAL: MSHLFLAGS = 0;
pub const MSHLFLAGS_RESERVED1: MSHLFLAGS = 8;
pub const MSHLFLAGS_RESERVED2: MSHLFLAGS = 16;
pub const MSHLFLAGS_RESERVED3: MSHLFLAGS = 32;
pub const MSHLFLAGS_RESERVED4: MSHLFLAGS = 64;
pub const MSHLFLAGS_TABLESTRONG: MSHLFLAGS = 1;
pub const MSHLFLAGS_TABLEWEAK: MSHLFLAGS = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OLECHAR(pub u16);
pub type PSCODE = *mut SCODE;
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SCODE(pub i32);
#[cfg(feature = "rpc")]
pub type UP_BYTE_BLOB = *mut BYTE_BLOB;
pub type UP_DWORD_BLOB = *mut DWORD_BLOB;
#[cfg(feature = "rpc")]
pub type UP_FLAGGED_BYTE_BLOB = *mut FLAGGED_BYTE_BLOB;
pub type UP_FLAGGED_WORD_BLOB = *mut FLAGGED_WORD_BLOB;
pub type UP_WORD_BLOB = *mut WORD_BLOB;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WORD_BLOB {
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl Default for WORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WORD_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u16,
}
impl Default for WORD_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
