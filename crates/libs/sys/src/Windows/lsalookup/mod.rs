pub const AccountDomainInformation: LSA_LOOKUP_DOMAIN_INFO_CLASS = 5;
pub const DnsDomainInformation: LSA_LOOKUP_DOMAIN_INFO_CLASS = 12;
pub const LOOKUP_TRANSLATE_NAMES: u32 = 2048;
pub const LOOKUP_VIEW_LOCAL_INFORMATION: u32 = 1;
pub type LSA_LOOKUP_DOMAIN_INFO_CLASS = i32;
pub type LSA_LOOKUP_HANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct LSA_OBJECT_ATTRIBUTES {
    pub Length: u32,
    pub RootDirectory: super::winnt::HANDLE,
    pub ObjectName: PLSA_UNICODE_STRING,
    pub Attributes: u32,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub SecurityQualityOfService: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for LSA_OBJECT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct LSA_REFERENCED_DOMAIN_LIST {
    pub Entries: u32,
    pub Domains: PLSA_TRUST_INFORMATION,
}
#[cfg(feature = "winnt")]
impl Default for LSA_REFERENCED_DOMAIN_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct LSA_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: super::winnt::PCHAR,
}
#[cfg(feature = "winnt")]
impl Default for LSA_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct LSA_TRANSLATED_NAME {
    pub Use: super::winnt::SID_NAME_USE,
    pub Name: LSA_UNICODE_STRING,
    pub DomainIndex: i32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct LSA_TRANSLATED_SID2 {
    pub Use: super::winnt::SID_NAME_USE,
    pub Sid: super::winnt::PSID,
    pub DomainIndex: i32,
    pub Flags: u32,
}
#[cfg(feature = "winnt")]
impl Default for LSA_TRANSLATED_SID2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct LSA_TRUST_INFORMATION {
    pub Name: LSA_UNICODE_STRING,
    pub Sid: super::winnt::PSID,
}
#[cfg(feature = "winnt")]
impl Default for LSA_TRUST_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LSA_UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: windows_sys::core::PWSTR,
}
impl Default for LSA_UNICODE_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PLSA_LOOKUP_DOMAIN_INFO_CLASS = *mut LSA_LOOKUP_DOMAIN_INFO_CLASS;
pub type PLSA_LOOKUP_HANDLE = *mut *mut core::ffi::c_void;
#[cfg(feature = "winnt")]
pub type PLSA_OBJECT_ATTRIBUTES = *mut LSA_OBJECT_ATTRIBUTES;
#[cfg(feature = "winnt")]
pub type PLSA_REFERENCED_DOMAIN_LIST = *mut LSA_REFERENCED_DOMAIN_LIST;
#[cfg(feature = "winnt")]
pub type PLSA_STRING = *mut LSA_STRING;
#[cfg(feature = "winnt")]
pub type PLSA_TRANSLATED_NAME = *mut LSA_TRANSLATED_NAME;
#[cfg(feature = "winnt")]
pub type PLSA_TRANSLATED_SID2 = *mut LSA_TRANSLATED_SID2;
#[cfg(feature = "winnt")]
pub type PLSA_TRUST_INFORMATION = *mut LSA_TRUST_INFORMATION;
pub type PLSA_UNICODE_STRING = *mut LSA_UNICODE_STRING;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct POLICY_ACCOUNT_DOMAIN_INFO {
    pub DomainName: LSA_UNICODE_STRING,
    pub DomainSid: super::winnt::PSID,
}
#[cfg(feature = "winnt")]
impl Default for POLICY_ACCOUNT_DOMAIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct POLICY_DNS_DOMAIN_INFO {
    pub Name: LSA_UNICODE_STRING,
    pub DnsDomainName: LSA_UNICODE_STRING,
    pub DnsForestName: LSA_UNICODE_STRING,
    pub DomainGuid: windows_sys::core::GUID,
    pub Sid: super::winnt::PSID,
}
#[cfg(feature = "winnt")]
impl Default for POLICY_DNS_DOMAIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type PPOLICY_ACCOUNT_DOMAIN_INFO = *mut POLICY_ACCOUNT_DOMAIN_INFO;
#[cfg(feature = "winnt")]
pub type PPOLICY_DNS_DOMAIN_INFO = *mut POLICY_DNS_DOMAIN_INFO;
