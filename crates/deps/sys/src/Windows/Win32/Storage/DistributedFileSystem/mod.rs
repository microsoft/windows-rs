#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsAdd();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsAddFtRoot();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsAddRootTarget();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsAddStdRoot();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsEnum();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsGetClientInfo();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetDfsGetFtContainerSecurity();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsGetInfo();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetDfsGetSecurity();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetDfsGetStdContainerSecurity();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsGetSupportedNamespaceVersion();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsMove();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsRemove();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsRemoveFtRoot();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsRemoveFtRootForced();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsRemoveRootTarget();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsRemoveStdRoot();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsSetClientInfo();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetDfsSetFtContainerSecurity();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetDfsSetInfo();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetDfsSetSecurity();
    #[doc = "*Required features: `Win32_Storage_DistributedFileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetDfsSetStdContainerSecurity();
}
