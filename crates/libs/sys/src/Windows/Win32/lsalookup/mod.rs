pub const AccountDomainInformation: LSA_LOOKUP_DOMAIN_INFO_CLASS = 5;
pub const DnsDomainInformation: LSA_LOOKUP_DOMAIN_INFO_CLASS = 12;
pub const LOOKUP_TRANSLATE_NAMES: i32 = 2048;
pub const LOOKUP_VIEW_LOCAL_INFORMATION: i32 = 1;
pub type LSA_LOOKUP_DOMAIN_INFO_CLASS = i32;
pub type LSA_LOOKUP_HANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct LSA_OBJECT_ATTRIBUTES {
    pub Length: super::ULONG,
    pub RootDirectory: super::HANDLE,
    pub ObjectName: PLSA_UNICODE_STRING,
    pub Attributes: super::ULONG,
    pub SecurityDescriptor: super::PVOID,
    pub SecurityQualityOfService: super::PVOID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct LSA_REFERENCED_DOMAIN_LIST {
    pub Entries: super::ULONG,
    pub Domains: PLSA_TRUST_INFORMATION,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type LSA_STRING = super::STRING;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct LSA_TRANSLATED_NAME {
    pub Use: super::SID_NAME_USE,
    pub Name: LSA_UNICODE_STRING,
    pub DomainIndex: super::LONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct LSA_TRANSLATED_SID2 {
    pub Use: super::SID_NAME_USE,
    pub Sid: super::PSID,
    pub DomainIndex: super::LONG,
    pub Flags: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct LSA_TRUST_INFORMATION {
    pub Name: LSA_UNICODE_STRING,
    pub Sid: super::PSID,
}
#[cfg(all(feature = "shared", feature = "winternl"))]
pub type LSA_UNICODE_STRING = super::UNICODE_STRING;
pub type PLSA_LOOKUP_DOMAIN_INFO_CLASS = *mut LSA_LOOKUP_DOMAIN_INFO_CLASS;
pub type PLSA_LOOKUP_HANDLE = *mut *mut core::ffi::c_void;
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type PLSA_OBJECT_ATTRIBUTES = *mut LSA_OBJECT_ATTRIBUTES;
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type PLSA_REFERENCED_DOMAIN_LIST = *mut LSA_REFERENCED_DOMAIN_LIST;
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type PLSA_STRING = *mut super::STRING;
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type PLSA_TRANSLATED_NAME = *mut LSA_TRANSLATED_NAME;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PLSA_TRANSLATED_SID2 = *mut LSA_TRANSLATED_SID2;
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type PLSA_TRUST_INFORMATION = *mut LSA_TRUST_INFORMATION;
#[cfg(all(feature = "shared", feature = "winternl"))]
pub type PLSA_UNICODE_STRING = *mut super::UNICODE_STRING;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct POLICY_ACCOUNT_DOMAIN_INFO {
    pub DomainName: LSA_UNICODE_STRING,
    pub DomainSid: super::PSID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct POLICY_DNS_DOMAIN_INFO {
    pub Name: LSA_UNICODE_STRING,
    pub DnsDomainName: LSA_UNICODE_STRING,
    pub DnsForestName: LSA_UNICODE_STRING,
    pub DomainGuid: windows_sys::core::GUID,
    pub Sid: super::PSID,
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type PPOLICY_ACCOUNT_DOMAIN_INFO = *mut POLICY_ACCOUNT_DOMAIN_INFO;
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type PPOLICY_DNS_DOMAIN_INFO = *mut POLICY_DNS_DOMAIN_INFO;
