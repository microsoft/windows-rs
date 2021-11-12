#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_ADD_VOLUME: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_FORCE_REMOVE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_MOVE_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_PROPERTY_FLAG_ABDE: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_PROPERTY_FLAG_CLUSTER_ENABLED: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_PROPERTY_FLAG_INSITE_REFERRALS: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_PROPERTY_FLAG_ROOT_SCALABILITY: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_PROPERTY_FLAG_SITE_COSTING: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_PROPERTY_FLAG_TARGET_FAILBACK: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_RESTORE_VOLUME: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_SITE_PRIMARY: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_STORAGE_FLAVOR_UNUSED2: u32 = 768u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_STORAGE_STATES: u32 = 15u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_STORAGE_STATE_ACTIVE: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_STORAGE_STATE_OFFLINE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_STORAGE_STATE_ONLINE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_VOLUME_FLAVORS: u32 = 768u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_VOLUME_FLAVOR_AD_BLOB: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_VOLUME_FLAVOR_STANDALONE: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_VOLUME_FLAVOR_UNUSED1: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_VOLUME_STATES: u32 = 15u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_VOLUME_STATE_FORCE_SYNC: u32 = 64u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_VOLUME_STATE_INCONSISTENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_VOLUME_STATE_OFFLINE: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_VOLUME_STATE_OK: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_VOLUME_STATE_ONLINE: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_VOLUME_STATE_RESYNCHRONIZE: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const DFS_VOLUME_STATE_STANDBY: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const FSCTL_DFS_BASE: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const FSCTL_DFS_GET_PKT_ENTRY_STATE: u32 = 401340u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const NET_DFS_SETDC_FLAGS: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const NET_DFS_SETDC_INITPKT: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_DistributedFileSystem`*"]
pub const NET_DFS_SETDC_TIMEOUT: u32 = 1u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsAdd(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, comment: super::super::Foundation::PWSTR, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsAddFtRoot(servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, ftdfsname: super::super::Foundation::PWSTR, comment: super::super::Foundation::PWSTR, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsAddRootTarget(pdfspath: super::super::Foundation::PWSTR, ptargetpath: super::super::Foundation::PWSTR, majorversion: u32, pcomment: super::super::Foundation::PWSTR, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsAddStdRoot(servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, comment: super::super::Foundation::PWSTR, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsEnum(dfsname: super::super::Foundation::PWSTR, level: u32, prefmaxlen: u32, buffer: *mut *mut u8, entriesread: *mut u32, resumehandle: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsGetClientInfo(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, level: u32, buffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetDfsGetFtContainerSecurity(domainname: super::super::Foundation::PWSTR, securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsGetInfo(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, level: u32, buffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetDfsGetSecurity(dfsentrypath: super::super::Foundation::PWSTR, securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetDfsGetStdContainerSecurity(machinename: super::super::Foundation::PWSTR, securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsGetSupportedNamespaceVersion(origin: DFS_NAMESPACE_VERSION_ORIGIN, pname: super::super::Foundation::PWSTR, ppversioninfo: *mut *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsMove(olddfsentrypath: super::super::Foundation::PWSTR, newdfsentrypath: super::super::Foundation::PWSTR, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsRemove(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsRemoveFtRoot(servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, ftdfsname: super::super::Foundation::PWSTR, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsRemoveFtRootForced(domainname: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, ftdfsname: super::super::Foundation::PWSTR, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsRemoveRootTarget(pdfspath: super::super::Foundation::PWSTR, ptargetpath: super::super::Foundation::PWSTR, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsRemoveStdRoot(servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsSetClientInfo(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, level: u32, buffer: *const u8) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetDfsSetFtContainerSecurity(domainname: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsSetInfo(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, level: u32, buffer: *const u8) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetDfsSetSecurity(dfsentrypath: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetDfsSetStdContainerSecurity(machinename: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
}
