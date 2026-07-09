#[repr(C)]
#[derive(Clone, Copy)]
pub struct BG_AUTH_CREDENTIALS {
    pub Target: BG_AUTH_TARGET,
    pub Scheme: BG_AUTH_SCHEME,
    pub Credentials: BG_AUTH_CREDENTIALS_UNION,
}
impl Default for BG_AUTH_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union BG_AUTH_CREDENTIALS_UNION {
    pub Basic: BG_BASIC_CREDENTIALS,
}
impl Default for BG_AUTH_CREDENTIALS_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BG_AUTH_SCHEME = i32;
pub const BG_AUTH_SCHEME_BASIC: BG_AUTH_SCHEME = 1;
pub const BG_AUTH_SCHEME_DIGEST: BG_AUTH_SCHEME = 2;
pub const BG_AUTH_SCHEME_NEGOTIATE: BG_AUTH_SCHEME = 4;
pub const BG_AUTH_SCHEME_NTLM: BG_AUTH_SCHEME = 3;
pub const BG_AUTH_SCHEME_PASSPORT: BG_AUTH_SCHEME = 5;
pub type BG_AUTH_TARGET = i32;
pub const BG_AUTH_TARGET_PROXY: BG_AUTH_TARGET = 2;
pub const BG_AUTH_TARGET_SERVER: BG_AUTH_TARGET = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BG_BASIC_CREDENTIALS {
    pub UserName: windows_sys::core::PWSTR,
    pub Password: windows_sys::core::PWSTR,
}
impl Default for BG_BASIC_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BG_JOB_REPLY_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
}
pub const BackgroundCopyManager1_5: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf087771f_d74f_4c1a_bb8a_e16aca9124ea);
pub type PBG_AUTH_CREDENTIALS = *mut BG_AUTH_CREDENTIALS;
pub type PBG_BASIC_CREDENTIALS = *mut BG_BASIC_CREDENTIALS;
