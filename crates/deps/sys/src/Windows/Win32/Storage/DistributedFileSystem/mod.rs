#![allow(non_snake_case, non_camel_case_types)]
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
