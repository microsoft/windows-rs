pub const AccountDomainInformation: LSA_LOOKUP_DOMAIN_INFO_CLASS = 5;
pub const DnsDomainInformation: LSA_LOOKUP_DOMAIN_INFO_CLASS = 12;
pub const LOOKUP_TRANSLATE_NAMES: u32 = 2048;
pub const LOOKUP_VIEW_LOCAL_INFORMATION: u32 = 1;
pub type LSA_LOOKUP_DOMAIN_INFO_CLASS = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LSA_LOOKUP_HANDLE(pub *mut core::ffi::c_void);
impl LSA_LOOKUP_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LSA_LOOKUP_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_REFERENCED_DOMAIN_LIST {
    pub Entries: u32,
    pub Domains: PLSA_TRUST_INFORMATION,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: super::winnt::PCHAR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_TRANSLATED_NAME {
    pub Use: super::winnt::SID_NAME_USE,
    pub Name: LSA_UNICODE_STRING,
    pub DomainIndex: i32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_TRANSLATED_SID2 {
    pub Use: super::winnt::SID_NAME_USE,
    pub Sid: super::winnt::PSID,
    pub DomainIndex: i32,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_TRUST_INFORMATION {
    pub Name: LSA_UNICODE_STRING,
    pub Sid: super::winnt::PSID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LSA_UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: windows_core::PWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLSA_LOOKUP_DOMAIN_INFO_CLASS(pub *mut LSA_LOOKUP_DOMAIN_INFO_CLASS);
impl PLSA_LOOKUP_DOMAIN_INFO_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLSA_LOOKUP_DOMAIN_INFO_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLSA_LOOKUP_HANDLE(pub *mut *mut core::ffi::c_void);
impl PLSA_LOOKUP_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLSA_LOOKUP_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLSA_OBJECT_ATTRIBUTES(pub *mut LSA_OBJECT_ATTRIBUTES);
#[cfg(feature = "winnt")]
impl PLSA_OBJECT_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PLSA_OBJECT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLSA_REFERENCED_DOMAIN_LIST(pub *mut LSA_REFERENCED_DOMAIN_LIST);
#[cfg(feature = "winnt")]
impl PLSA_REFERENCED_DOMAIN_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PLSA_REFERENCED_DOMAIN_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLSA_STRING(pub *mut LSA_STRING);
#[cfg(feature = "winnt")]
impl PLSA_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PLSA_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLSA_TRANSLATED_NAME(pub *mut LSA_TRANSLATED_NAME);
#[cfg(feature = "winnt")]
impl PLSA_TRANSLATED_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PLSA_TRANSLATED_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLSA_TRANSLATED_SID2(pub *mut LSA_TRANSLATED_SID2);
#[cfg(feature = "winnt")]
impl PLSA_TRANSLATED_SID2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PLSA_TRANSLATED_SID2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLSA_TRUST_INFORMATION(pub *mut LSA_TRUST_INFORMATION);
#[cfg(feature = "winnt")]
impl PLSA_TRUST_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PLSA_TRUST_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLSA_UNICODE_STRING(pub *mut LSA_UNICODE_STRING);
impl PLSA_UNICODE_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLSA_UNICODE_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_ACCOUNT_DOMAIN_INFO {
    pub DomainName: LSA_UNICODE_STRING,
    pub DomainSid: super::winnt::PSID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICY_DNS_DOMAIN_INFO {
    pub Name: LSA_UNICODE_STRING,
    pub DnsDomainName: LSA_UNICODE_STRING,
    pub DnsForestName: LSA_UNICODE_STRING,
    pub DomainGuid: windows_core::GUID,
    pub Sid: super::winnt::PSID,
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOLICY_ACCOUNT_DOMAIN_INFO(pub *mut POLICY_ACCOUNT_DOMAIN_INFO);
#[cfg(feature = "winnt")]
impl PPOLICY_ACCOUNT_DOMAIN_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PPOLICY_ACCOUNT_DOMAIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOLICY_DNS_DOMAIN_INFO(pub *mut POLICY_DNS_DOMAIN_INFO);
#[cfg(feature = "winnt")]
impl PPOLICY_DNS_DOMAIN_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PPOLICY_DNS_DOMAIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
